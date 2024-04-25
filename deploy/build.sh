#!/bin/bash

# This script gets the model defined at the wget link and puts it at llm_mlops/model/pythia.bin
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
current_path=$(pwd)
cd $parent_path/.. # Run this from the root of the llm_ops main directory
trap "cd $current_path" EXIT # Make sure we go back to the original calling directory when we are done
ECR_URL="None"
REGION="us-west-2"
NO_DEPLOY=false
while getopts r:u:n flag
do
    case "${flag}" in
        r) REGION=${OPTARG};;
        u) ECR_URL=${OPTARG};;
        n) NO_DEPLOY=true;;
        \?) echo "Invalid option -$OPTARG" >&2
        exit 1
    esac
done
if [ "$ECR_URL" == "None" ]; then
    echo -e "Please specify an ECR Repo URL using -u"
    exit 1
fi
docker build -t $ECR_URL:latest .
if $NO_DEPLOY; then
    echo -e "Not deploying as NO_DEPLOY was set to true"
    exit 0
fi
TMP=$(aws ecr get-login-password --region $REGION)
docker login --username AWS --password $TMP $ECR_URL
docker push $ECR_URL:latest