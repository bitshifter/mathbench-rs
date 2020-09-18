#!/usr/bin/env python3

import argparse
import json
import os
import sys
import prettytable

DEFAULT = ['glam', 'cgmath', 'nalgebra']
OPTIONAL = ['euclid', 'vek', 'pathfinder', 'static-math', 'ultraviolet']
SCALAR = DEFAULT + OPTIONAL

WIDE = ['glam', 'ultraviolet_f32x4', 'nalgebra_f32x4', 'ultraviolet_f32x8', 'nalgebra_f32x8']

CHOICES = SCALAR + WIDE

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


def fmt_bench(x, max_value, min_value, threshold):
    if x is not None:
        if max_value >= 1000:
            return f'__{x/1000:3.4} us__' if x/min_value <= threshold else f'  {x/1000:3.4} us  '
        else:
            return f'__{x:3.4f} ns__' if x/min_value <= threshold else f'  {x:3.4f} ns  '
    return '   N/A      '


def parse_bench(json_dir, benches):
    benchmark_path = os.path.join(json_dir, 'benchmark.json')
    estimates_path = os.path.join(json_dir, 'estimates.json')
    try:
        with open(benchmark_path) as f:
            benchmarks = json.load(f)
            bench_name = benchmarks['group_id']
            input_size = benchmarks['value_str']
            try:
                if input_size is not None:
                    bench_name = '{} x{}'.format(bench_name, input_size)
            except ValueError:
                pass
            lib_name = benchmarks['function_id']
        with open(estimates_path) as f:
            estimates = json.load(f)
            slope_point = estimates['slope']['point_estimate']
            benches.setdefault(bench_name, {}).setdefault(lib_name, slope_point)
    except FileNotFoundError:
        pass


def main():
    default_libs = DEFAULT
    parser = argparse.ArgumentParser()
    parser.add_argument('-w', '--wide', action='store_true', help='include all wide libraries')
    parser.add_argument('-s', '--scalar', action='store_true', help='include all scalar libraries')
    parser.add_argument('-a', '--all', action='store_true', help='include all libraries')
    parser.add_argument('-t', '--threshold', type=float, default=2.5, help='percent of minimum value to highlight')
    parser.add_argument('libs', nargs='*', action=DefaultListAction,
                        default=default_libs,
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

    threshold = 1.0 + args.threshold / 100.0

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
            input_sizes = []
            new_path = os.path.join(lib_path, 'new')
            if not os.path.isdir(new_path):
                # check for input sizes
                for input_dir in os.listdir(lib_path):
                    try:
                        input_sizes.append(int(input_dir))
                    except ValueError:
                        pass

            if input_sizes:
                for input_size in input_sizes:
                    new_path = os.path.join(lib_path, str(input_size), 'new')
                    parse_bench(new_path, benches)
            else:
                parse_bench(new_path, benches)

    pt = prettytable.PrettyTable(['benchmark'] + [f'  {x:}  ' for x in libs])
    for bench_name in benches:
        if args.wide:
            if "wide" not in bench_name:
                continue
        else:
            if "wide" in bench_name:
                continue

        bench = benches[bench_name]
        values = [bench[x] for x in libs if x in bench]
        max_value = max(values)
        min_value = min(values)
        # hack so nothing is highlighted if there's only one lib to display
        if len(libs) == 1:
            min_value = max_value + 1
        value_strs = [fmt_bench(bench.get(x, None), max_value, min_value, threshold) for x in libs]
        pt.add_row([bench_name] + value_strs)
    pt.sortby = 'benchmark'
    pt.align = 'r'
    pt.align['benchmark'] = 'l'
    pt.hrules = prettytable.HEADER
    pt.junction_char = '|'
    print(pt)


if __name__ == '__main__':
    main()
