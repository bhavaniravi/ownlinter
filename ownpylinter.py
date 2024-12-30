# Python Script
import os
import json
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
import sys
from itertools import chain


def load_config():
    try:
        with open("linter.json", "r") as config_file:
            return json.load(config_file)
    except FileNotFoundError:
        print("Config file 'linter.json' not found.")
        exit(1)
    except json.JSONDecodeError:
        print("Error parsing 'linter.json'. Make sure it's valid JSON.")
        exit(1)


def find_py_files(path):
    for root, _, files in os.walk(path):
        for file in files:
            if file.endswith(".py"):
                yield os.path.join(root, file)


def scan_file(file_path, line_length):
    errors = []
    try:
        with open(file_path, "r") as file:
            for line_no, line in enumerate(file, start=1):
                if len(line) > line_length:
                    errors.append((line_no, line.strip()))
    except Exception as e:
        # print(f"Error reading {file_path}: {e}")
        pass
    return file_path, errors


def main():
    if len(sys.argv) < 2:
        print("Usage: python ownpylinter.py <path>")
        exit(1)
    path = Path(sys.argv[1])
    config = load_config()
    line_length = config.get("line_length", 80)

    py_files = find_py_files(path)

    with ThreadPoolExecutor() as executor:
        results = executor.map(lambda f: scan_file(f, line_length), py_files)

    results = list(results)
    files_scanned = len(results)
    error_files = {file: errors for file, errors in results if errors}

    print("\nScan Report:")
    print(f"Files Scanned: {files_scanned}")
    print(f"Error Files: {len(error_files)}")
    print("\n\n")
    # for file, errors in error_files.items():
    #     print(f"\nFile: {file}")
    #     for line_no, line in errors:
    #         print(f"  Line {line_no}: {line}")


if __name__ == "__main__":
    main()
