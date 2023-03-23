#!/usr/bin/env python3

'''
Run terragrunt only in directories where files were change

TODO:
- recognize files that are under last terragrunt.hcl or .terraform.lock.hcl
'''

__author__ = "gitbluf"


import os
import sys
import subprocess
import typing

from os import path
from subprocess import PIPE, Popen
from collections import OrderedDict


def _get_changed_files() -> str:
    try:
        output = Popen(
            ["git", "diff", "HEAD", "HEAD~", "--name-only"], stdout=PIPE
        ).communicate()[0]
        return output.decode("utf-8")
    except subprocess.CalledProcessError as sbcpe:
        raise sbcpe


def run_terragrunt(cmd: str, path: str) -> None:
    currentDir = os.getcwd()
    try:
        os.chdir(path)
        subprocess.call(
            ["terragrunt", "run-all", cmd, "--terragrunt-non-interactive"],
            stdout=sys.stdout,
            stderr=sys.stderr,
        )
    except subprocess.CalledProcessError as sbcpe:
        raise sbcpe
    finally:
        os.chdir(currentDir)


def get_paths() -> list:
    changed = _get_changed_files().split("\n")
    configurations = []
    for change in changed:
        splited = change.split("/")
        if len(splited) > 2:
            configurations.append([s for s in splited if not s.endswith(".hcl")])
    return [
        "/".join(path)
        for i, path in enumerate(configurations)
        if i == configurations.index(path)
    ]


# ENTRY
if __name__ == "__main__":
    for change in get_paths():
        run_terragrunt(cmd=sys.argv[1], path=change)

