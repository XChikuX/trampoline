# Use a smaller base image for the build stage
FROM alpine:latest

# Set working directory
WORKDIR /app

COPY check_if_email_exists .

# Set the entrypoint
ENTRYPOINT [ "./check_if_email_exists"]