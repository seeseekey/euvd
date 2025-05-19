#!/bin/sh

# Delete docs and src folder
rm -rf ../docs ../src

# Generate client
openapi-generator generate \
  -i api.yaml \
  -g rust \
  -o ../ \
  -p packageName=euvd

# Modify source code (of default_api)
echo "Modify source code ..."
python3 modify.py

# Cargo.toml edits
# Add homepage and repository
sed -i '' '/^edition = "2021"/a\
homepage = "https://seeseekey.net"\
repository = "https://github.com/seeseekey/euvd.git"
' ../Cargo.toml

# Add dev Dependencies
cat <<EOF >> ../Cargo.toml

[dev-dependencies]
tokio = { version = "1", features = ["full", "macros"] }
reqwest = { version = "^0.12", default-features = false, features = ["json", "multipart", "rustls-tls"] }
EOF