import os, subprocess
import re
import time

from datetime import datetime

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
"""

# Default values, please don't change
flags = {
    "START_ID": 0,
    "LAST_ID": 100,
    "TIMEOUT": 1,
    "RELEASE": True,
    "SLOWEST": 10,
    "UPDATE_README": False
}

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
        if id < flags["START_ID"]:
            continue
        if id > flags["LAST_ID"]:
            break
        project_path = "./" + path + "/Cargo.toml"
        os.chdir(path)
        start_time = time.perf_counter_ns()
        call_cmd(RUN_CMD(mode="clean", release=flags["RELEASE"]))
        compile_time = (time.perf_counter_ns() - start_time) / 1e9
        compile_time, _ = translate(f"{compile_time * 1000}ms")
        print(f"Cleaning {project_path} took {compile_time}.")
        os.chdir("..")
    print()

def measure_build(paths, ids):
    for id, path in zip(ids, paths):
        if id < flags["START_ID"]:
            continue
        if id > flags["LAST_ID"]:
            break
        os.chdir(path)
        project_path = "./" + path + "/Cargo.toml"
        start_time = time.perf_counter_ns()
        call_cmd(RUN_CMD(mode="build", release=flags["RELEASE"]))
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
        if id < flags["START_ID"]:
            continue
        if id > flags["LAST_ID"]:
            break
        timeout = False
        os.chdir(path)
        project_path = "./" + path + "/Cargo.toml"
        try:
            success, time_as_list = get_output(release=flags["RELEASE"], timeout=flags["TIMEOUT"])
            time_as_list = parse(time_as_list)
            time_as_list, time_in_ms = translate(time_as_list)
        except subprocess.TimeoutExpired:
            tmo = flags["TIMEOUT"]
            print(f"THIS PROGRAM NEEDED LONGER THAN {tmo} SECONDS TO FINISH!")
            timeout = True
            time_in_ms = tmo * 1_000_000
            time_as_list = f">{tmo}s"
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

def parse_input(type, msg):
    while True:
        inp = input()
        if inp == "":
            return None
        try:
            inp = type(inp)
            if inp < 0:
                print(msg)
                continue
            break
        except:
            print(msg)
            pass
    return inp

def ask_for_cli():
    assert len(flags) == 6, "Please ask for all flags in ask_for_cli()"
    print("Welcome to the setup of this script!")
    print("Leave a line empty [Press ENTER] to keep the default value.")

    print(f"Select Start ID (Default: " + str(flags["START_ID"]) + ")")
    sid = parse_input(int, "Please enter a number >= 0")
    if sid == None:
        sid = flags["START_ID"]
    flags["START_ID"] = sid

    print(f"Select End ID (Default: " + str(flags["LAST_ID"]) + ")")
    while True:
        eid = parse_input(int, "Please enter a number >= 0")
        if eid == None:
            eid = flags["LAST_ID"]
            break
        if eid < sid:
            print("End ID must be greater than Start ID!")
        else:
            break
    flags["LAST_ID"] = eid

    print(f"Select Timeout in seconds (Default: " + str(flags["TIMEOUT"]) + ")")
    to = parse_input(int, "Please enter a number >= 1")
    if to == None:
        to = flags["TIMEOUT"]
    flags["TIMEOUT"] = to

    print("Benchmark in Release Mode? (Default: " + str(flags["RELEASE"]) + ")")
    while True:
        rel = parse_input(int, "Please enter 0 (False) or 1 (True)")
        if rel == None or rel == 0 or rel == 1:
            break
        else:
            print("Please enter 0 (False) or 1 (True)")
    if rel == None:
        rel = flags["RELEASE"]
    flags["RELEASE"] = bool(rel)

    print("How many programs should be printed? (Default: " + str(flags["SLOWEST"]) + " slowest programs)")
    slo = parse_input(int, "Please enter a number >= 0")
    if slo == None:
        slo = flags["SLOWEST"]
    flags["SLOWEST"] = slo

    print("Update README? (Default: " + str(flags["UPDATE_README"]) + ")")
    while True:
        rdme = parse_input(int, "Please enter 0 (False) or 1 (True)")
        if rdme == None or rdme == 0 or rdme == 1:
            break
        else:
            print("Please enter 0 (False) or 1 (True)")
    if rdme == None:
        rdme = flags["UPDATE_README"]
    flags["UPDATE_README"] = bool(rdme)
    
    print("Successfully set all flags!")

if __name__ == "__main__":
    ask_for_cli()
    
    paths = [folder for folder in os.listdir() if folder.startswith("p")]

    ids = [int(re.findall('\d+', p)[0]) for p in paths]

    # measure_clean(paths, ids)
    # exit()
    measure_build(paths, ids)
    all_projects, crashed, not_sub = measure_exec(paths, ids)
    
    all_projects.sort(key=itemgetter(0), reverse=True)

    print("All programs sorted by run time:")
    for (_, t, path) in all_projects:
        print(f"{t:>8}: {path}")
    print()

    lid = flags["LAST_ID"]
    tmo = flags["TIMEOUT"]
    print("The following programs crashed:")
    for path in crashed:
        print(path)
    print(f"That's {len(crashed)}/{lid} programs or {(len(crashed) * 100 / lid):.2f}%.")
    print()

    print(f"The following programs did not get sub {tmo}sec:")
    for path in not_sub:
        print(path)
    print(f"That's {len(not_sub)}/{lid} programs or {(len(not_sub) * 100 / lid):.2f}%.")
    print()

    slo = flags["SLOWEST"]
    print(f"The {slo} slowest programs are:")
    for (_, t, path) in all_projects[:slo]:
        print(f"{t:>8}: {path}")
    print()

    if flags["UPDATE_README"]:
        prev_readme = "".join(open("README.md", "r").readlines())
        os.remove("README.md")
        with open("README.md", "x", encoding="utf-8") as readme:
            if "\n## Benchmarks\n" in prev_readme:
                prev_readme = prev_readme.split("\n## Benchmarks\n")[0]
            readme.write(prev_readme)
            readme.write("\n## Benchmarks\n")
            readme.write(f"Currently the {slo} slowest programs are:\n")
            readme.write("| Time | Project |\n")
            readme.write("| :---: | --- |\n")
            overtime = False
            for (_, t, path) in all_projects[:slo]:
                if ">" in t:
                    overtime = True
                readme.write(f"|{t:>8}|{path}|\n")
            if overtime:
                readme.write(f"\nA `>` indicates that the program did not finish within my set timeout of {TIMEOUT}s.\n")
        print("Updated README.md with the current benchmarks.")