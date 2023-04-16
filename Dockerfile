# Stage 1: Build
FROM rust:1.68 as builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the source code into the container
COPY . .

# Build the release version of the microservice
RUN cargo build --release

# Stage 2: Serve
FROM debian:buster-slim

# Install required libraries
RUN apt-get update && \
    apt-get install -y libssl1.1 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Create a new user to run the application
RUN useradd -ms /bin/bash appuser

# Set the working directory
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/microservice ./microservice

# Set the ownership and permissions for the binary
RUN chown appuser:appuser ./microservice && \
    chmod 755 ./microservice

# Switch to the appuser
USER appuser

# Expose the port the app runs on
EXPOSE 8080

# Start the application
CMD ["./microservice"]
