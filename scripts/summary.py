#!/usr/bin/env python3

import argparse
import json
import os
import sys
import prettytable


DEFAULT = ['glam', 'cgmath', 'nalgebra', 'euclid']
OPTIONAL = ['vek']
CHOICES = DEFAULT + OPTIONAL

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


def fmt_bench(x, max_value, min_value):
    if x is not None:
        if max_value >= 1000:
            return f'__{x/1000:3.4} us__' if x == min_value else f'  {x/1000:3.4} us  '
        else:
            return f'__{x:3.4f} ns__' if x == min_value else f'  {x:3.4f} ns  '
    return '   N/A      '


def main():
    default_libs = DEFAULT
    parser = argparse.ArgumentParser()
    parser.add_argument('libs', nargs='*', action=DefaultListAction,
                        default=default_libs,
                        help='choose from {0}'.format(CHOICES))
    args = parser.parse_args()

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
            new_path = os.path.join(bench_path, lib_name, 'new')
            benchmark_path = os.path.join(new_path, 'benchmark.json')
            estimates_path = os.path.join(new_path, 'estimates.json')
            try:
                with open(benchmark_path) as f:
                    benchmarks = json.load(f)
                    bench_name = benchmarks['group_id']
                with open(estimates_path) as f:
                    estimates = json.load(f)
                    slope_point = estimates['Slope']['point_estimate']
                    benches.setdefault(bench_name, {}).setdefault(lib_name, slope_point)
            except FileNotFoundError:
                pass

    pt = prettytable.PrettyTable(['benchmark'] + [f'  {x:}  ' for x in libs])
    for bench_name in benches:
        bench = benches[bench_name]
        values = [bench[x] for x in libs if x in bench]
        max_value = max(values)
        min_value = min(values)
        # hack so nothing is highlighted if there's only one lib to display
        if len(libs) == 1:
            min_value = max_value + 1
        value_strs = [fmt_bench(bench.get(x, None), max_value, min_value) for x in libs]
        pt.add_row([bench_name] + value_strs)
    pt.sortby = 'benchmark'
    pt.align = 'r'
    pt.align['benchmark'] = 'l'
    pt.hrules = prettytable.HEADER
    pt.junction_char = '|'
    print(pt)


if __name__ == '__main__':
    main()
