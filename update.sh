#!/bin/sh

# Generate client
openapi-generator generate \
  -i api.yaml \
  -g rust \
  -o . \
  -p packageName=euvd

# Modify source code (of default_api)
echo "Modify source code ..."
python3 modify.py

# Add dev Dependencies
cat <<EOF >> Cargo.toml

[dev-dependencies]
tokio = { version = "1", features = ["full", "macros"] }
reqwest = { version = "^0.12", default-features = false, features = ["json", "multipart", "rustls-tls"] }
EOF