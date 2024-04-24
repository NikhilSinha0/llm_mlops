# LLM Ops

This is the final project for group 6 for IDS 721. The project involves taking an open source model from HuggingFace and creating a web service in Rust to serve inferences from the model. The webservice needs to be deployed on a Kubernetes cluster and set up with some monitoring and metrics. There also needs to be a CI/CD pipeline for this repository to automate the process of testing/building/deployment of the service.

## Build Process

The build process for this project is requires some assets. First you need to download a model from huggingface, and keep it in a known location in the repository.

This process is simplified as you can simply call the `model/get_model.sh` script to download the correct model and set it up with the correct name and location without direct user input.

You can then build this binary using `cargo build`. You then need to copy it alongside the built application binary into a docker container to deploy.

To simplify this, the binary has been packaged with a Dockerfile. This Dockerfile will build the binary, including the predownloaded assets. Once the binary is built, it is pushed to ECR using

```
export ECR_URL=<ECR Repo URL>
docker build -t $ECR_URL:latest .
aws ecr get-login-password --region <region> | docker login --username AWS --password-stdin $ECR_URL
docker push $ECR_URL:latest
```

A simpler way to do this is to simply call the
```
deploy/build.sh
```
script instead, which will run these instructions for you.

Then you can run the container locally to test using a command like
```
docker run -it --rm -p 8080:8080 $ECR_URL:latest
```

and hit the endpoint with a command like
```
curl -X POST -H "Content-Type: application/json" --data '{"input": "tigers are cool because"}' http://localhost:8080/message
```
run from a new terminal (since the original terminal will be running the service).


## Usage

This webservice is intended to be used with cURL. A command for usage should look like

```
curl -H "Content-Type: application/json" --data '{"input": "tigers are cool because"}' <URL>
```

and should return an answer from the model.
