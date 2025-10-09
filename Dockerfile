# Use an official Rust runtime as a parent image
FROM rust:1.56.0 as builder

# Set the working directory in the container
WORKDIR /usr/src/plume

# Copy the current directory contents into the container at /usr/src/plume
COPY . .

# Build the project
RUN cargo build --release

# Use a smaller base image for production
FROM debian:latest

# Set the working directory in the container
WORKDIR /usr/src/plume

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/plume/target/release/plume /usr/src/plume/plume

# Expose the port that the API will listen on
EXPOSE 8080

# Run the API when the container starts
CMD ["/usr/src/plume/plume"]