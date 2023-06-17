import os, subprocess
import re
import time

from operator import itemgetter

"""
Small helper script I use for measuring execution times of my Project Euler implementations.
It goes into each directory, runs `cargo run --quiet [--release]`, and gets the output from running that command.
It then gets the time (obtained using std::time::Instant in my projects), and adds the tuple (time_in_µs, time_str, path) to the list.
It does that for every project with an ID between START_ID and LAST_ID (both inclusive).
It then sorts the created list by time_in_µs, and prints that list to the console.
It also takes note of any problem that crashed (for example if Rust needed to load a file, but did not find it),
 and all problems that did not finish within the specified TIMEOUT (so that I can optimize them).
It also prints the n slowest programs with their execution time.
The 5 lines below this comment block can be set to benchmark specific projects.
I'll eventually change the script to work with CLI input, such as `python stopwatch.py 50 80` for running IDs 50 to 80.
"""

START_ID = 5
LAST_ID = 11
TIMEOUT = 1
RELEASE = True
SLOWEST = 10

def get_output(release=False, timeout=999999999):
    text = ""
    try:
        text = subprocess.check_output(RUN_CMD(mode="run", release=release), timeout=timeout)
    except subprocess.CalledProcessError as e:
        return False,
    except FileNotFoundError:
        return False,
    formatted_text = text.decode("utf-8")
    lines = formatted_text.split("\n")
    return True, lines

def RUN_CMD(mode="run", release=False):
    if release:
        return ["cargo", mode, "--quiet", "--release"]
    else:
        return ["cargo", mode, "--quiet"]

def call_cmd(cmd, timeout=999999999):
    return subprocess.call(cmd, timeout=timeout)

def measure_clean(paths, ids):
    for id, path in zip(ids, paths):
        if id < START_ID:
            continue
        if id > LAST_ID:
            break
        project_path = "./" + path + "/Cargo.toml"
        os.chdir(path)
        start_time = time.perf_counter_ns()
        call_cmd(RUN_CMD(mode="clean", release=RELEASE))
        compile_time = (time.perf_counter_ns() - start_time) / 1e9
        compile_time, _ = translate(f"{compile_time * 1000}ms")
        print(f"Cleaning {project_path} took {compile_time}.")
        os.chdir("..")
    print()

def measure_build(paths, ids):
    for id, path in zip(ids, paths):
        if id < START_ID:
            continue
        if id > LAST_ID:
            break
        os.chdir(path)
        project_path = "./" + path + "/Cargo.toml"
        start_time = time.perf_counter_ns()
        call_cmd(RUN_CMD(mode="build", release=RELEASE))
        compile_time = (time.perf_counter_ns() - start_time) / 1e9
        compile_time, _ = translate(f"{compile_time * 1000}ms")
        print(f"Building {project_path} took {compile_time}.")
        os.chdir("..")
    print()
    
def measure_exec(paths, ids):
    all_projects = []
    crashed = []
    not_sub = []
    for id, path in zip(ids, paths):
        if id < START_ID:
            continue
        if id > LAST_ID:
            break
        timeout = False
        os.chdir(path)
        project_path = "./" + path + "/Cargo.toml"
        try:
            success, time_as_list = get_output(release=RELEASE, timeout=TIMEOUT)
            time_as_list = parse(time_as_list)
            time_as_list, time_in_ms = translate(time_as_list)
        except subprocess.TimeoutExpired:
            print(f"THIS PROGRAM NEEDED LONGER THAN {TIMEOUT} SECONDS TO FINISH!")
            timeout = True
            time_in_ms = TIMEOUT * 1_000_000
            time_as_list = f">{TIMEOUT}s"
        print(f"Running {project_path} took {time_as_list}.")
        all_projects.append((time_in_ms, time_as_list, project_path))
        if timeout:
            not_sub.append(project_path)
        if not timeout and not success:
            print("THIS PROGRAM CRASHED!")
            crashed.append(project_path)
        os.chdir("..")
    print()
    return all_projects, crashed, not_sub

def parse(time_as_list):
    return time_as_list[-2]

def translate(time):
    # translate time in any unit to time in microseconds
    unit = time[-2:] if time[-2:].isalpha() else time[-1]
    dur = float(time[:-len(unit)])
    old_dur = round(dur, 2)
    match unit:
        case "s" :
            dur *= 1_000_000
        case "ms":
            dur *= 1_000
        case "µs": pass
        case "ns":
            dur /= 1_000
        case _:
            print(f"ERROR: Illegal unit found: `{time}`!")
            exit(1)
    return f"{old_dur}{unit}", dur

if __name__ == "__main__":
    paths = [folder for folder in os.listdir() if folder.startswith("p")]
    # print(paths)

    ids = [int(re.findall('\d+', p)[0]) for p in paths]
    # paths = ["./" + path + "/Cargo.toml" for path in paths]
    # print(ids)

    # measure_clean(paths, ids)
    # exit()
    measure_build(paths, ids)
    all_projects, crashed, not_sub = measure_exec(paths, ids)
    
    all_projects.sort(key=itemgetter(0), reverse=True)

    print("All programs sorted by run time:")
    for (_, t, path) in all_projects:
        print(f"{t:>8}: {path}")
    print()
    print("The following programs crashed:")
    for path in crashed:
        print(path)
    print(f"That's {len(crashed)}/{LAST_ID} programs or {(len(crashed) * 100 / LAST_ID):.2f}%.")
    print()
    print(f"The following programs did not get sub {TIMEOUT}sec:")
    for path in not_sub:
        print(path)
    print(f"That's {len(not_sub)}/{LAST_ID} programs or {(len(not_sub) * 100 / LAST_ID):.2f}%.")
    print()
    print(f"The {SLOWEST} slowest programs are:")
    for (_, t, path) in all_projects[:SLOWEST]:
        print(f"{t:>8}: {path}")
        