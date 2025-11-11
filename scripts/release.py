#!/usr/bin/env python

# -----------------------------------------------------------------------------

# Copyright (c) 2025 Zensical and contributors

# SPDX-License-Identifier: MIT
# Third-party contributions licensed under DCO

# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to
# deal in the Software without restriction, including without limitation the
# rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
# sell copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:

# The above copyright notice and this permission notice shall be included in
# all copies or substantial portions of the Software.

# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
# FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
# IN THE SOFTWARE.

import json, os, re, sys  # noqa: E401

from enum import IntEnum

# ----------------------------------------------------------------------------
# Classes
# ----------------------------------------------------------------------------


class Bump(IntEnum):
    """
    Bump suggestion.
    """

    PATCH = 0
    MINOR = 1
    MAJOR = 2


# ----------------------------------------------------------------------------
# Functions
# ----------------------------------------------------------------------------


def resolve():
    """
    Return versions and dependents for workspace members.
    """
    versions: dict[str, str] = {}
    dependents: dict[str, list[str]] = {}

    # Retrieve cargo metadata
    with os.popen("cargo metadata --format-version 1 --no-deps") as p:
        meta = json.loads(p.read())

    # Create dependency mapping of all packages
    for package in meta["packages"]:
        versions[package["name"]] = package["version"]

        # Create inverse dependency mapping
        for dependency in package["dependencies"]:
            if dependency.get("path"):
                packages = dependents.setdefault(dependency["name"], [])
                packages.append(package["name"])

    # Return versions and dependents
    return versions, dependents


def bump(name: str) -> Bump | None:
    """
    Return bump suggestion for given crate.
    """
    args = f'--include-path "crates/{name}/**" --unreleased --context'
    with os.popen(f"git cliff -c .gitcliff.toml {args}") as p:
        [meta] = json.loads(p.read())

    # Only bump if there are commits
    commits = meta.get("commits", [])
    if not commits:
        return None

    # Determine bump suggestion
    value = Bump.PATCH
    for commit in commits:
        # Check for breaking changes
        if commit.get("breaking"):
            return Bump.MAJOR

        # Check for features
        if commit.get("group").endswith("Features"):
            if value < Bump.MINOR:
                value = Bump.MINOR

    # Return bump suggestion
    return value


def version(current: str, level: Bump) -> str:
    """
    Compute new version given current version and bump suggestion.
    """
    match = re.match(r"^(\d+)\.(\d+)\.(\d+)$", current)
    if not match:
        raise ValueError(f"Invalid version: {current}")

    # 0.0.x => only bump patch
    major, minor, patch = map(int, match.groups())
    if major == 0 and minor == 0:
        patch += 1

    # 0.x.y => only bump patch or minor
    elif major == 0:
        if level >= Bump.MINOR:
            minor += 1
            patch = 0
        else:
            patch += 1

    # 1.x.y => use suggested bump as-is
    return f"{major}.{minor}.{patch}"


# ----------------------------------------------------------------------------
# Program
# ----------------------------------------------------------------------------


def main():
    """
    Bump all changed packages and their dependents.
    """
    versions, dependents = resolve()

    # Detect uncommitted changes
    with os.popen("git status --porcelain") as p:
        output = p.read().strip()
        if output:
            print("Uncommitted changes detected, aborting release.")
            return sys.exit(1)

    # Determine bumps
    levels: dict[str, Bump] = {}
    for name in versions:
        level = bump(name)
        if level is None:
            continue

        # Set version for directly affected package
        value = version(versions[name], level)
        os.system(f"cargo set-version {value} --package {name}")

        # Now, propagate to dependents
        for dependent in dependents.get(name, []):
            if dependent in levels and level is not None:
                levels[dependent] = max(levels[dependent], level)
            elif level is not None:
                levels[dependent] = level

    # Set versions for direct and transitive dependents
    for name, level in levels.items():
        value = version(versions[name], level)
        os.system(f"cargo set-version {value} --package {name}")


# ----------------------------------------------------------------------------

if __name__ == "__main__":
    main()
