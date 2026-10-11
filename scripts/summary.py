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


def parse_bench(json_dir, benches):
    benchmark_path = os.path.join(json_dir, 'benchmark.json')
    estimates_path = os.path.join(json_dir, 'estimates.json')
    try:
        with open(benchmark_path) as f:
            benchmark = json.load(f)
        with open(estimates_path) as f:
            estimates = json.load(f)
    except FileNotFoundError:
        return

    bench_name = benchmark['group_id']
    lib_name = benchmark['function_id']
    input_size = benchmark['value_str']

    # Criterion's estimate is per timed iteration, i.e. per batch of `size`
    # elements. Report per element so rows are comparable across sizes and
    # across scalar and wide types.
    size = 1
    if input_size is not None:
        try:
            size = int(input_size)
        except ValueError:
            pass
        bench_name = '{} x{}'.format(bench_name, input_size)

    slope = estimates['slope']
    point = slope['point_estimate'] / size
    interval = slope.get('confidence_interval') or {}
    lower = interval.get('lower_bound')
    upper = interval.get('upper_bound')
    lower = lower / size if lower is not None else None
    upper = upper / size if upper is not None else None

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

    if args.wide or args.scalar or args.all:
        if args.wide:
            libs = WIDE
        if args.scalar:
            libs = SCALAR
        if args.all:
            libs = CHOICES
    else:
        libs = list(dict.fromkeys(args.libs))

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
            # Prefer parameterized (per-size) results. Fall back to a single
            # `new` directory for a benchmark that has no size.
            input_sizes = []
            for entry in os.listdir(lib_path):
                try:
                    input_sizes.append(int(entry))
                except ValueError:
                    pass
            if input_sizes:
                for input_size in sorted(input_sizes):
                    parse_bench(os.path.join(lib_path, str(input_size), 'new'), benches)
            else:
                parse_bench(os.path.join(lib_path, 'new'), benches)

    pt = prettytable.PrettyTable(['benchmark'] + [f'  {x:}  ' for x in libs])
    pt.title = 'ns per element'
    for bench_name in benches:
        if args.wide:
            if WIDE_PREFIX not in bench_name:
                continue
        else:
            if WIDE_PREFIX in bench_name:
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

        # strip prefix off the bench name for display
        if bench_name.startswith(SCALAR_PREFIX):
            bench_name = bench_name[len(SCALAR_PREFIX):]
        elif bench_name.startswith(WIDE_PREFIX):
            bench_name = bench_name[len(WIDE_PREFIX):]

        pt.add_row([bench_name] + value_strs)
    pt.sortby = 'benchmark'
    pt.align = 'r'
    pt.align['benchmark'] = 'l'
    pt.hrules = prettytable.HEADER
    pt.junction_char = '|'
    print(pt)


if __name__ == '__main__':
    main()
