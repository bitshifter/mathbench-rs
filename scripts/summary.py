#!/usr/bin/env python3

import argparse
import json
import math
import os
import statistics
import sys
import prettytable

DEFAULT = ['glam', 'cgmath', 'nalgebra']
OPTIONAL = ['euclid', 'vek', 'pathfinder', 'ultraviolet']
SCALAR = DEFAULT + OPTIONAL

WIDE = ['glam_f32x1', 'ultraviolet_f32x4', 'nalgebra_f32x4',
        'ultraviolet_f32x8', 'nalgebra_f32x8']

CHOICES = SCALAR + WIDE

SCALAR_PREFIX = 'scalar '
WIDE_PREFIX = 'wide '

# One-sided 95% z-score for the non-inferiority test in `is_equivalent`.
EQUIVALENCE_Z = statistics.NormalDist().inv_cdf(0.95)


class DefaultListAction(argparse.Action):
    def __call__(self, parser, namespace, values, option_string=None):
        if values:
            for value in values:
                if value not in CHOICES:
                    message = ("invalid choice: {0!r} (choose from {1})"
                               .format(value,
                                       ', '.join([repr(action)
                                                  for action in CHOICES])))

                    raise argparse.ArgumentError(self, message)
            setattr(namespace, self.dest, values)


def fmt_value(value):
    if value is None:
        return '   N/A      '
    return f'{value:3.4f} ns'


def fail(message):
    sys.exit('error: {}\n\nDelete target/criterion and re-run the benchmarks.'
             .format(message))


def parse_bench(json_dir, benches):
    try:
        with open(os.path.join(json_dir, 'benchmark.json')) as f:
            benchmark = json.load(f)
        with open(os.path.join(json_dir, 'estimates.json')) as f:
            estimates = json.load(f)

        # Criterion's estimate is per timed iteration, i.e. per batch of `size`
        # elements. Report per element so rows are comparable across sizes and
        # across scalar and wide types. Every benchmark declares its element
        # count via Throughput::Elements and uses that count as its BenchmarkId.
        size = int(benchmark['throughput']['Elements'])
        if benchmark['value_str'] != str(size):
            raise ValueError(
                "BenchmarkId {!r} does not match Throughput::Elements {}"
                .format(benchmark['value_str'], size))

        bench_name = '{} x{}'.format(benchmark['group_id'], size)
        lib_name = benchmark['function_id']

        slope = estimates['slope']
        point = slope['point_estimate'] / size
        se = slope.get('standard_error')
        if se is None:
            # Older results may carry only the interval; recover the standard
            # error from it so the equivalence test still has an input.
            interval = slope.get('confidence_interval') or {}
            lower = interval.get('lower_bound')
            upper = interval.get('upper_bound')
            if lower is not None and upper is not None:
                level = interval.get('confidence_level', 0.95)
                z = statistics.NormalDist().inv_cdf(0.5 + level / 2.0)
                se = (upper - lower) / (2.0 * z)
            else:
                se = 0.0
        se = se / size
    except (KeyError, TypeError, ValueError, OSError) as e:
        fail("unexpected data in '{}': {}".format(json_dir, e))

    benches.setdefault(bench_name, {}).setdefault(lib_name, (point, se))


def is_equivalent(entry, best, margin):
    """True when `entry` is not more than `margin` slower than `best`.

    Non-inferiority test on the per-element slope: the upper bound of the
    one-sided 95% interval for `entry - best` must not exceed `margin` times
    the best point estimate. A library with a wide interval therefore fails
    the test instead of being highlighted as indistinguishable.
    """
    point, se = entry
    best_point, best_se = best
    diff = point - best_point
    se_diff = math.sqrt(se * se + best_se * best_se)
    return diff + EQUIVALENCE_Z * se_diff <= margin * best_point


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('-w', '--wide', action='store_true', help='include all wide libraries')
    parser.add_argument('-s', '--scalar', action='store_true', help='include all scalar libraries')
    parser.add_argument('-a', '--all', action='store_true', help='include all libraries')
    parser.add_argument('-t', '--threshold', type=float, default=2.5,
                        help='percent of the fastest row value within which a '
                             'library is highlighted (default: 2.5)')
    parser.add_argument('libs', nargs='*', action=DefaultListAction,
                        default=DEFAULT,
                        help='choose from {0}'.format(CHOICES))
    args = parser.parse_args()
    margin = args.threshold / 100.0

    if args.all or (args.wide and args.scalar):
        libs = CHOICES
    elif args.wide:
        libs = WIDE
    elif args.scalar:
        libs = SCALAR
    else:
        libs = list(dict.fromkeys(args.libs))

    # `-a` (or `-w -s` together) includes both row kinds. A kind flag
    # restricts to that kind; without one, show the kinds of the libraries
    # that were actually selected (scalar-only by default).
    if args.wide or args.scalar or args.all:
        show_wide = args.wide or args.all
        show_scalar = args.scalar or args.all
    else:
        show_wide = any(lib in WIDE for lib in libs)
        show_scalar = any(lib in SCALAR for lib in libs)

    root_dir = os.path.normpath(os.path.join(os.path.dirname(__file__), '..'))
    criterion_dir = os.path.join(root_dir, 'target', 'criterion')
    if not os.path.isdir(criterion_dir):
        sys.exit("'{}' directory doesn't exist, run `cargo bench` first.".format(criterion_dir))

    benches = {}
    for bench_dir in os.listdir(criterion_dir):
        if bench_dir == 'report':
            continue
        bench_path = os.path.join(criterion_dir, bench_dir)
        for lib_name in libs:
            lib_path = os.path.join(bench_path, lib_name)
            if not os.path.isdir(lib_path):
                continue
            # Every benchmark is parameterized by its element count.
            entries = [e for e in os.listdir(lib_path) if e != 'report']
            if not entries:
                fail("no benchmark results in '{}'".format(lib_path))
            for size in entries:
                if not size.isdigit():
                    fail("unexpected entry '{}' in '{}'".format(size, lib_path))
                parse_bench(os.path.join(lib_path, size, 'new'), benches)

    pt = prettytable.PrettyTable(['benchmark'] + [f'  {x:}  ' for x in libs])
    pt.title = 'ns per element'
    for bench_name in benches:
        is_wide = bench_name.startswith(WIDE_PREFIX)
        if is_wide and not show_wide:
            continue
        if not is_wide and not show_scalar:
            continue

        bench = benches[bench_name]
        # Highlight a library when it is not more than `margin` slower than
        # the fastest in the row and the one-sided 95% interval for the
        # difference rules out a slowdown larger than `margin`. The fastest
        # library is the reference and is always highlighted.
        if len(libs) == 1:
            best_lib = None
        else:
            best_lib = min((x for x in libs if x in bench),
                           key=lambda x: bench[x][0])

        value_strs = []
        for x in libs:
            entry = bench.get(x)
            if entry is None:
                value_strs.append(fmt_value(None))
                continue
            value = fmt_value(entry[0])
            if best_lib is not None and (
                    x == best_lib
                    or is_equivalent(entry, bench[best_lib], margin)):
                value_strs.append(f'__{value}__')
            else:
                value_strs.append(f'  {value}  ')

        # When only one row kind is shown its prefix is redundant; keep it when
        # both are shown so scalar and wide rows with the same name stay
        # distinguishable.
        if not (show_wide and show_scalar):
            if bench_name.startswith(SCALAR_PREFIX):
                bench_name = bench_name[len(SCALAR_PREFIX):]
            elif bench_name.startswith(WIDE_PREFIX):
                bench_name = bench_name[len(WIDE_PREFIX):]

        pt.add_row([bench_name] + value_strs)
    pt.sortby = 'benchmark'
    pt.align = 'r'
    pt.align['benchmark'] = 'l'
    pt.hrules = getattr(prettytable, 'HRuleStyle', prettytable).HEADER
    pt.junction_char = '|'
    print(pt)


if __name__ == '__main__':
    main()
