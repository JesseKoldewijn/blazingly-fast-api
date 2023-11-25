
# Use the official Rust image as the base image
FROM rust:latest

# Create a new directory for our application
WORKDIR /usr/src/myapp

# Set the environment variables\
ARG PORT=8000
ENV PORT=${PORT}

# Copy source code to the container
COPY . ./

# Build the dependencies
RUN cargo build --release

# Copy the source code to the container
COPY src ./src

# Build the application
RUN cargo build --release

# Remove the source code
RUN rm -rf ./src

# Expose the port that the server will listen on
EXPOSE ${PORT}

# Start the server
CMD ["./target/release/jkinsight-api"]
