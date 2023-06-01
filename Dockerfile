# Tells docker to use the latest Rust official image
FROM rust:latest
# Copy current working directory into the container
COPY ./ ./
# Create the release build
RUN cargo build --package octopus_web --release
# Expose the port the app is running on
EXPOSE 8080

# Run the application with server logging!
ENV RUST_LOG="trace"
CMD ["./target/release/octopus_web"]