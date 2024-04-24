# LLM Ops

This is the final project for group 6 for IDS 721. The project involves taking an open source model from HuggingFace and creating a web service in Rust to serve inferences from the model. The webservice needs to be deployed on a Kubernetes cluster and set up with some monitoring and metrics. There also needs to be a CI/CD pipeline for this repository to automate the process of testing/building/deployment of the service.

## Build Process

The build process for this project is requires some assets. First you need to download a model from huggingface, and keep it in a known location in the repository. You can then build this binary using `cargo build`. You then need to copy it alongside the built application binary into a docker container to deploy.

To simplify this, the binary has been packaged with a Dockerfile. This Dockerfile will build the binary, including the predownloaded assets. Once the binary is built, it is pushed to ECR using

```
docker build -t <ECR Repo URL>:latest .
aws ecr get-login-password --region <region> | docker login --username AWS --password-stdin <ECR Repo URL>
docker push <ECR Repo URL>:latest
```


## Usage

This webservice is intended to be used with cURL. A command for usage should look like

```
curl -H "Content-Type: application/json" --data '{"input": "tigers are cool because"}' <URL>
```

and should return an answer from the model.
