#!/usr/bin/env bash
project="nitrogenase-docker"
name="tsv"
tag="0.0.2"
full="${name}:${tag}"
echo "Using Google project ${project}, Docker project ${name}, full tag ${full}"
echo "Cloud-building Docker image:"
gcloud builds submit --timeout=60m --tag gcr.io/${project}/${full}
echo "Done"
