# Use a smaller base image for the build stage
FROM alpine:latest

# Set working directory
WORKDIR /app

COPY trampoline .

# Set the entrypoint
ENTRYPOINT [ "./trampoline"]