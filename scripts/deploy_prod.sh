#!/bin/bash
set -ex

cd "$(dirname "$0")"
./rebuild_all.sh
gcloud compute ssh mainali_subrat@instance-1 --zone us-central1-a --command="sudo systemctl stop ajisen"
./send_to_gcp.sh
gcloud compute ssh mainali_subrat@instance-1 --zone us-central1-a --command="sudo systemctl start ajisen"
