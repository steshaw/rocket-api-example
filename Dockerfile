from rustlang/rust:nightly-slim as build

workdir /app
copy . .
run cargo build --release

from debian:10-slim
workdir /app
copy --from=build /app/target/release/rocket-eg-1 /app
copy --from=build /app/Rocket.toml /app/
cmd ["/app/rocket-eg-1"]
