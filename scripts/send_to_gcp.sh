#!/bin/bash
set -ex

cd "$(dirname "$0")/.."
gcloud compute scp target/release/ajisen mainali_subrat@instance-1:/home/mainali_subrat/ajisen_env --zone us-central1-a
