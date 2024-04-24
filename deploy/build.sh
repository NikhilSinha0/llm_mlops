#!/bin/bash

# This script gets the model defined at the wget link and puts it at llm_mlops/model/pythia.bin
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
current_path=$(pwd)
cd $parent_path/.. # Run this from the root of the llm_ops main directory
trap "cd $current_path" EXIT # Make sure we go back to the original calling directory when we are done
ECR_URL="None"
REGION="us-west-2"
while getopts u:a:f: flag
do
    case "${flag}" in
        u) ECR_URL=${OPTARG};;
        r) REGION=${OPTARG};;
    esac
done
docker build -t $ECR_URL:latest .
aws ecr get-login-password --region <region> | docker login --username AWS --password-stdin $ECR_URL
docker push $ECR_URL:latest