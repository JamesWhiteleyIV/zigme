# Use the official Rust image as the base image
FROM rust:1.76-bullseye AS builder

# Set the working directory inside the Docker container
WORKDIR /app

# Copy source files into container
COPY . .

# Build the application
RUN cargo build --release

# Use a slim image for the final image
FROM debian:bullseye-slim AS runner

# Set the working directory inside the Docker container
WORKDIR /app

RUN apt-get update -y \
     && apt-get install -y --no-install-recommends openssl ca-certificates curl \
     && apt-get autoremove -y \
     && apt-get clean -y \
     && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage to the final image
COPY --from=builder /app/target/release/zigme-api ./

EXPOSE 3020

# Run the Axum web application when the Docker container starts
CMD ["./zigme-api"]
