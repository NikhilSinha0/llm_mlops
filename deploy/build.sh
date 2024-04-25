#!/bin/bash

# This script gets the model defined at the wget link and puts it at llm_mlops/model/pythia.bin
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
current_path=$(pwd)
cd $parent_path/.. # Run this from the root of the llm_ops main directory
trap "cd $current_path" EXIT # Make sure we go back to the original calling directory when we are done
ECR_URL="None"
REGION="us-west-2"
NO_DEPLOY=false
PUBLIC=false
SKIP_BUILD=false
while getopts r:u:nsp flag
do
    case "${flag}" in
        r) REGION=${OPTARG};;
        u) ECR_URL=${OPTARG};;
        p) PUBLIC=true;;
        s) SKIP_BUILD=true;;
        n) NO_DEPLOY=true;;
        \?) echo "Invalid option -$OPTARG" >&2
        exit 1
    esac
done
if [ "$ECR_URL" == "None" ]; then
    echo -e "Please specify an ECR Repo URL using -u"
    exit 1
fi
if ! $SKIP_BUILD; then
    docker build -t $ECR_URL:latest .
fi
if $NO_DEPLOY; then
    echo -e "Not deploying as NO_DEPLOY was set to true"
    exit 0
fi
if $PUBLIC; then
    TMP=$(aws ecr-public get-login-password --region $REGION)
else
    TMP=$(aws ecr get-login-password --region $REGION)
fi
docker login --username AWS --password $TMP $ECR_URL
docker push $ECR_URL:latest