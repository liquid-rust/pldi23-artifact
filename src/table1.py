#!/usr/bin/env python3

import argparse
from math import floor
import sys
import os
import subprocess
import time
from count_lines import count_file, count_files

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
        # os.path.join("lib", "vecwrapper.rs"): False,
        # os.path.join("lib", "matwrapper.rs"): False,
        # # Benchmarks
        # "bsearch.rs": True,
        # "dotprod.rs": True,
        # "fft.rs": True,
        # "heapsort.rs": True,
        # "simplex.rs": True,
        # "kmeans.rs": True,
        # "kmp.rs": True,
    },
    "flux": {
        # Libraries
        # os.path.join("lib", "rvec.rs"): False,
        # os.path.join("lib", "rmat.rs"): True,
        # # Benchmarks
        "bsearch.rs": True,
        # "dotprod.rs": True,
        # "fft.rs": True,
        # "heapsort.rs": True,
        # "simplex.rs": True,
        # "kmeans.rs": True,
        # "kmp.rs": True,
    }
}


def benchmark_path(*segments):
    return os.path.join(os.path.dirname(__file__), "benchmarks", *segments)


def overhead(counts):
    if counts['annot'] > 0:
        return f"{(floor((counts['annot']) / counts['loc'] * 100))}%"
    else:
        return "-"


def annot(counts):
    if counts['annot'] > 0:
        return str(counts['annot'])
    else:
        return "-"


def get_table_for_tool(tool, args):
    if args.repeat > 0:
        print(f'\nVerifying benchmarks with {tool}')

    table = []
    for (benchmark, verify) in BENCHMARKS[tool].items():
        counts = count_file(benchmark_path(tool, benchmark))
        if verify and args.repeat > 0:
            cmd = [*CMD[tool], benchmark_path(tool, benchmark)]
            time = str(timeit(benchmark, cmd, args.repeat))
        else:
            time = "-"
        row = [
            benchmark,
            str(counts["loc"]),
            str(counts["spec"]),
            annot(counts),
            overhead(counts), time
        ]
        table.append(row)
    table.append(check_wave(tool, args))

    return table


def check_wave(tool, args):
    cwd = os.getcwd()

    os.chdir(benchmark_path(tool, "wave"))
    # Touch file to avoid clear cargo cache
    before = lambda: subprocess.run(["touch", "src/lib.rs"])
    if args.repeat > 0 and tool == "flux":
        cmd = ["cargo-flux", "check"]
        # Check once to avoid including dependencies in the total time
        init = lambda: subprocess.run(cmd, stderr=subprocess.DEVNULL)
        time = str(timeit("wave", cmd, args.repeat, init, before))
    elif args.repeat > 0 and tool == "prusti":
        cmd = ["cargo-prusti", "--features", "verify"]
        # Check once to avoid including dependencies in the total time
        init = lambda: subprocess.run(cmd, stderr=subprocess.DEVNULL)
        time = str(timeit("wave", cmd, args.repeat, init, before))
    else:
        time = "-"
    counts = count_files("src/**/*.rs")

    row = [
        "wave",
        str(counts["loc"]),
        str(counts["spec"]),
        annot(counts),
        overhead(counts), time
    ]
    os.chdir(cwd)

    return row


def mean(data):
    n = len(data)
    total = sum(data)
    return total / n


def timeit(name, cmd, repeat=5, init=None, before=None):
    times = []
    print(" ", name, end="", flush=True)
    if init:
        init()
    for _ in range(repeat):
        if before:
            before()
        t1 = time.monotonic()
        subprocess.run(cmd,
                       stderr=subprocess.DEVNULL,
                       stdout=subprocess.DEVNULL)
        t2 = time.monotonic()
        times.append(t2 - t1)
    print(" ✓")
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
    parser.add_argument("--repeat", type=int, default=5)
    args = parser.parse_args()

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
