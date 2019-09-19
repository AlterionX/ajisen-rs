#!/bin/bash
set -ex

cd "$(dirname "$0")"
./rebuild_all.sh
cd ..
target/release/ajisen
