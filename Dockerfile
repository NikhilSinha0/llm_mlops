# Use the official Rust image as the build environment
FROM rust:1.77-slim-buster as builder

# Set the working directory
WORKDIR /app

# Copy the source code into the container
ADD . ./

# Build the application
RUN apt-get update -y && \
  apt-get install -y pkg-config make g++ libssl-dev
RUN cargo clean && cargo build --release

# Create a new lightweight container for the application
FROM debian:buster-slim

# Set the working directory
WORKDIR /app

# Make sure openSSL is set up correctly in the container
RUN apt-get update -y && \
  apt-get install -y pkg-config make g++ libssl-dev
# Make sure the model is available for usage
RUN mkdir /app/model
COPY ./model /app/model

# Copy the compiled binary from the builder stage into the lightweight container
COPY --from=builder /app/target/release/ns380-transformer-lambda .

# Command to run the application
CMD ["./ns380-transformer-lambda"]
