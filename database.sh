#!/usr/bin/env sh
set -eux

skopeo copy --insecure-policy \
    docker-archive:$(nix build .#postgres/docker --no-link --print-out-paths) \
    docker-daemon:postgres-lockpad:latest

docker run --rm \
    --name postgres-lockpad \
    -p 5444:5432 \
    -e POSTGRES_DB=postgres \
    -e POSTGRES_USER=postgres \
    -e POSTGRES_PASSWORD=password \
    -d postgres-lockpad:latest
