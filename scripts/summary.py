#!/usr/bin/env python3

import json
import os
import sys
import prettytable

def main():
    root_dir = os.path.normpath(os.path.join(os.path.dirname(__file__), '..'))
    criterion_dir = os.path.join(root_dir, 'target', 'criterion')
    if not os.path.isdir(criterion_dir):
        sys.exit("'{}' directory doesn't exist, run `cargo bench` first.".format(criterion_dir))

    libs = ['glam', 'cgmath', 'nalgebra']
    benches = {}
    for bench_dir in os.listdir(criterion_dir):
        if bench_dir == 'report':
            continue
        bench_path = os.path.join(criterion_dir, bench_dir)
        for lib_name in libs:
            new_path = os.path.join(bench_path, lib_name, 'new')
            benchmark_path = os.path.join(new_path, 'benchmark.json')
            estimates_path = os.path.join(new_path, 'estimates.json')
            with open(benchmark_path) as f:
                benchmarks = json.load(f)
                bench_name = benchmarks['group_id']
            with open(estimates_path) as f:
                estimates = json.load(f)
                slope_point = estimates['Slope']['point_estimate']
                benches.setdefault(bench_name, {}).setdefault(lib_name, slope_point)
   
    pt = prettytable.PrettyTable(['benchmark'] + [f'  {x:}  ' for x in libs])
    for bench_name in benches:
        bench = benches[bench_name]
        values = [bench[x] for x in libs]
        max_value = max(values)
        min_value = min(values)
        if max_value >= 1000:
            value_strs = [f'__{x/1000:3.4} us__' if x == min_value else f'  {x/1000:3.4} us  ' for x in values]
        else:
            value_strs = [f'__{x:3.4f} ns__' if x == min_value else f'  {x:3.4f} ns  ' for x in values]
        pt.add_row([bench_name] + value_strs)
    pt.sortby = 'benchmark'
    pt.align = 'r'
    pt.align['benchmark'] = 'l'
    pt.hrules = prettytable.HEADER
    pt.junction_char = '|'
    print(pt)
        

if __name__ == '__main__':
    main()
