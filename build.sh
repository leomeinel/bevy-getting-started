#!/usr/bin/env bash
###
# File: build.sh
# Author: Leopold Johannes Meinel (leo@meinel.dev)
# -----
# Copyright (c) 2025 Leopold Johannes Meinel & contributors
# SPDX ID: Apache-2.0
# URL: https://www.apache.org/licenses/LICENSE-2.0
###

# Fail on error
set -e

# Build debug build if any cli argument is given, otherwise build release build
if [[ -z "${1}" ]]; then
    cargo build --release
else
    cargo build
fi
