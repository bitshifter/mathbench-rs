#!/usr/bin/env python3

import argparse
import json
import os
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
        interval = slope.get('confidence_interval') or {}
        lower = interval.get('lower_bound')
        upper = interval.get('upper_bound')
        lower = lower / size if lower is not None else None
        upper = upper / size if upper is not None else None
    except (KeyError, TypeError, ValueError, OSError) as e:
        fail("unexpected data in '{}': {}".format(json_dir, e))

    benches.setdefault(bench_name, {}).setdefault(
        lib_name, (point, lower, upper))


def overlaps_best(entry, best):
    """True when `entry`'s confidence interval overlaps the row minimum's.

    Falls back to the point estimate when a confidence interval is missing.
    """
    point, lower, upper = entry
    best_point, best_lower, best_upper = best
    lower = lower if lower is not None else point
    upper = upper if upper is not None else point
    best_lower = best_lower if best_lower is not None else best_point
    best_upper = best_upper if best_upper is not None else best_point
    return lower <= best_upper and upper >= best_lower


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('-w', '--wide', action='store_true', help='include all wide libraries')
    parser.add_argument('-s', '--scalar', action='store_true', help='include all scalar libraries')
    parser.add_argument('-a', '--all', action='store_true', help='include all libraries')
    parser.add_argument('libs', nargs='*', action=DefaultListAction,
                        default=DEFAULT,
                        help='choose from {0}'.format(CHOICES))
    args = parser.parse_args()

    if args.all or (args.wide and args.scalar):
        libs = CHOICES
    elif args.wide:
        libs = WIDE
    elif args.scalar:
        libs = SCALAR
    else:
        libs = list(dict.fromkeys(args.libs))

    # `-a` (or `-w -s` together) includes both row kinds; otherwise restrict to
    # the requested kind, defaulting to scalar-only.
    if args.wide or args.scalar or args.all:
        show_wide = args.wide or args.all
        show_scalar = args.scalar or args.all
    else:
        show_wide, show_scalar = False, True

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
        # Highlight every library whose confidence interval overlaps the
        # fastest point estimate: those are statistically indistinguishable
        # from the best and should not be presented as a winner.
        if len(libs) == 1:
            best = None
        else:
            best = min((bench[x] for x in libs if x in bench),
                       key=lambda entry: entry[0])

        value_strs = []
        for x in libs:
            entry = bench.get(x)
            if entry is None:
                value_strs.append(fmt_value(None))
                continue
            value = fmt_value(entry[0])
            if best is not None and overlaps_best(entry, best):
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
