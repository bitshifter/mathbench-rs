#!/usr/bin/env python3

import json
import os
from prettytable import PrettyTable

def main():
    root_dir = os.path.normpath(os.path.join(os.path.dirname(__file__), '..'))
    criterion_dir = os.path.join(root_dir, 'target', 'criterion')
    if not os.path.isdir(criterion_dir):
        sys.exit("'{}' directory doesn't exist, run `cargo bench` first.")

    libs = ['glam', 'cgmath', 'nalgebra']
    benches = {}
    for bench_name in os.listdir(criterion_dir):
        if bench_name == 'report':
            continue
        bench_dir = os.path.join(criterion_dir, bench_name)
        for lib_name in libs:
            estimates_path = os.path.join(bench_dir, lib_name, 'new', 'estimates.json')
            with open(estimates_path) as f:
                estimates = json.load(f)
                slope_point = estimates['Slope']['point_estimate']
                benches.setdefault(bench_name, {}).setdefault(lib_name, slope_point)
   
    pt = PrettyTable(['benchmark'] + libs)
    for bench_name in benches:
        bench = benches[bench_name]
        values = [bench[x] for x in libs]
        max_value = max(values)
        min_value = min(values)
        if max_value >= 1000:
            value_strs = [f'__{x/1000:5.5} us__' if x == min_value else f'  {x/1000:5.5} us  ' for x in values]
        else:
            value_strs = [f'__{x:5.5f} ns__' if x == min_value else f'  {x:5.5f} ns  ' for x in values]
        pt.add_row([bench_name] + value_strs)
    pt.sortby = 'benchmark'
    pt.align = 'r'
    pt.align['benchmark'] = 'l'
    print(pt)
        

if __name__ == '__main__':
    main()
