#!/bin/bash
docker run -p 8000:8000 \
  -e ROCKET_ENV=stage \
  -e ROCKET_PORT=8000 \
  -e PIPEDREAM_CAT_WRITE_ENABLED=true \
  -v "$(pwd)/files:/files" \
  rust-musl-pipedream
