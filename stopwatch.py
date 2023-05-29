import os, subprocess
import re
import time

"""
The following programs crashed:
./p022-names-scores/Cargo.toml
./p042-coded-triangle-numbers/Cargo.toml
./p054-poker-hands/Cargo.toml
./p059-xor-decryption/Cargo.toml
./p067-maximum-path-sum-ii/Cargo.toml
./p079-passcode-derivation/Cargo.toml
=> Those are all programs that need to load files, they normally work, but --manifest-path seems to mess stuff up

With --release:

The following programs did not get sub60:
./p070-totient-permutation/Cargo.toml took 66.4913735 seconds.

Without --release:
    
The following programs did not get sub60:
./p039-integer-right-triangles/Cargo.toml took >60.0155485 seconds.
./p060-prime-pair-sets/Cargo.toml took >60.0110544 seconds.
./p068-magic-5-gon-ring/Cargo.toml took >60.0053095 seconds.
./p070-totient-permutation/Cargo.toml took >60.014967 seconds.
./p078-coin-partitions/Cargo.toml took >60.0050478 seconds.
"""
    
def RUN_CMD(filename, mode="run", release=False):
    if release:
        return ["cargo", mode, "--quiet", "--manifest-path", f"{filename}", "--release"]
    else:
        return ["cargo", mode, "--quiet", "--manifest-path", f"{filename}"]


def call_cmd(cmd):
    return subprocess.call(cmd, timeout=60)

if __name__ == "__main__":
    paths = [folder for folder in os.listdir() if not folder.startswith(".") and folder.startswith("p")]
    # print(paths)

    ids = [int(re.findall('\d+', p)[0]) for p in paths]
    # print(ids)

    crashed = []
    not_sub = []

    release = True

    for id, path in zip(ids, paths):
        if id > 80:
            break
        project_path = "./" + path + "/Cargo.toml"
        start_time = time.perf_counter_ns()
        exit_code = call_cmd(RUN_CMD(project_path, mode="build", release=release))
        compile_time = (time.perf_counter_ns() - start_time) / 1e9
        print(f"Building {project_path} took {compile_time}s.")

    for id, path in zip(ids, paths):
        if id > 80:
            break
        project_path = "./" + path + "/Cargo.toml"
        start_time = time.perf_counter_ns()
        try:
            exit_code = call_cmd(RUN_CMD(project_path, release=release))
        except subprocess.TimeoutExpired:
            print("THIS PROGRAM NEEDED LONGER THAN 60 SECONDS TO FINISH!")
            compile_time = (time.perf_counter_ns() - start_time) / 1e9
            not_sub.append((project_path, compile_time))
            continue
        compile_time = (time.perf_counter_ns() - start_time) / 1e9
        print(f"Running {project_path} took {compile_time}s.")
        if exit_code != 0:
            print("THIS PROGRAM CRASHED!")
            crashed.append(project_path)
    
    print()
    print("The following programs crashed:")
    for path in crashed:
        print(path)
    print()
    print("The following programs did not get sub60:")
    for (path, time) in not_sub:
        print(f"{path} took >{time} seconds.")