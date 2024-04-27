# Use the official Rust image as the build environment
FROM rust:1.77 as builder

# Set the working directory
WORKDIR /app

# Copy the source code into the container
ADD . ./

# Build the application
RUN apt-get update && apt-get install -y python3-venv python3-pip && rm -rf /var/lib/apt/lists/*
RUN pip3 install torch==2.1.0+cpu --index-url https://download.pytorch.org/whl/cpu --break-system-packages
ENV LIBTORCH_USE_PYTORCH=1

RUN cargo clean && cargo build --release

# Create a new lightweight container for the application
FROM debian:latest

# Set the working directory
WORKDIR /app

# Make sure torch is set up correctly in the container
RUN apt-get update && apt-get install -y libfontconfig1 wget python3 python3-pip && \
  pip3 install torch==2.1.0+cpu --index-url https://download.pytorch.org/whl/cpu --break-system-packages && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/*

# Make sure the model is available for usage
RUN mkdir /app/model
COPY ./model /app/model

# Make sure the model is available for usage
RUN mkdir /app/model
COPY ./model /app/model

# Copy the compiled binary from the builder stage into the lightweight container
COPY --from=builder /app/target/release/llm_mlops .

ENV LD_LIBRARY_PATH=/usr/local/lib/python3.11/dist-packages/torch/lib:$LD_LIBRARY_PATH

# Command to run the application
CMD ["./llm_mlops"]
