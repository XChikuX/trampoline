# Use a smaller base image for the build stage
FROM alpine:latest

# Set working directory
WORKDIR /app

COPY ./target/release/trampoline .

# Set the entrypoint
ENTRYPOINT [ "./trampoline"]