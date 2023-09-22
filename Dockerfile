# Use a smaller base image for the build stage
FROM scratch


COPY trampoline /usr/local/bin/


# Set the entrypoint
ENTRYPOINT [ "/usr/local/bin/trampoline", "--http-host", "0.0.0.0"]
