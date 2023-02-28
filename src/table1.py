#!/usr/bin/env python3

import argparse
from math import floor
import sys
import os
import subprocess
import time
from count_lines import count_file

CMD = {
    "flux": ["rustc-flux", "--crate-type=lib"],
    "prusti": [
        "prusti-rustc", "-Pcheck_overflows=false", "-Coverflow-checks=off",
        "--crate-type=lib"
    ]
}

BENCHMARKS = {
    "prusti": {
        # Libraries
        os.path.join("lib", "vecwrapper.rs"): False,
        os.path.join("lib", "matwrapper.rs"): False,
        # Benchmarks
        "bsearch.rs": True,
        "dotprod.rs": True,
        "fft.rs": True,
        "heapsort.rs": True,
        "simplex.rs": True,
        "kmeans.rs": True,
        "kmp.rs": True,
    },
    "flux": {
        # Libraries
        os.path.join("lib", "rvec.rs"): False,
        os.path.join("lib", "rmat.rs"): True,
        # Benchmarks
        "bsearch.rs": True,
        "dotprod.rs": True,
        "fft.rs": True,
        "heapsort.rs": True,
        "simplex.rs": True,
        "kmeans.rs": True,
        "kmp.rs": True,
    }
}


def get_table_for_tool(tool, args):
    if not args.no_verify:
        print(f'\nVerifying benchmarks with {tool}')

    table = []
    for (benchmark, verify) in BENCHMARKS[tool].items():
        counts = count_file(os.path.join("benchmarks", tool, benchmark))
        overhead = floor((counts['annot']) / counts['loc'] * 100)
        row = [
            benchmark,
            str(counts["loc"]),
            str(counts["spec"]),
            str(counts["annot"]), f"{overhead}%"
        ]
        if verify and not args.no_verify:
            cmd = [*CMD[tool], os.path.join("benchmarks", tool, benchmark)]
            time = str(timeit(benchmark, cmd, args.repeat))
        else:
            time = "-"
        row.append(time)
        table.append(row)
    return table


def mean(data):
    n = len(data)
    total = sum(data)
    return total / n


def timeit(name, cmd, repeat=5):
    times = []
    print(" ", name, end="", flush=True)
    for _ in range(repeat):
        print(".", end="", flush=True)
        t1 = time.monotonic()
        subprocess.run(cmd, stderr=subprocess.DEVNULL)
        t2 = time.monotonic()
        times.append(t2 - t1)
    print("")
    return round(mean(times), 2)


def print_table(table):
    widths = [18, 5, 5, 6, 5, 8]
    headers = ["Benchmark", "LOC", "Spec", "Annot", "(% LOC)", "Time (s)"]
    print("  ".join(f"{header:>{w}}" for (header, w) in zip(headers, widths)))
    print("-" * 59)
    for row in table:
        print("  ".join(f"{cell:>{w}}" for (cell, w) in zip(row, widths)))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--no-verify", action='store_true')
    parser.add_argument("--repeat", type=int, default=5)
    args = parser.parse_args()

    if args.repeat < 1:
        sys.exit("Error: --repeat must be at least 1")

    flux = get_table_for_tool("flux", args)
    prusti = get_table_for_tool("prusti", args)

    print()
    print("FLUX")
    print("====")
    print_table(flux)

    print()
    print()

    print("PRUSTI")
    print("======")
    print_table(prusti)
