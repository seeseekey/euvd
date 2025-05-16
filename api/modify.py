#!/usr/bin/env python3

from pathlib import Path

# Path to file
file_path = Path("../src/apis/default_api.rs")

# List of replacement tuples
replacements = [
    (
        'return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::Vulnerability&gt;`"))),',
        'serde_json::from_str(&content).map_err(Error::from),'
    ),
    (
        'return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Vulnerabilities`"))),',
        'serde_json::from_str(&content).map_err(Error::from),'
    ),
    (
        'return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::VulnerabilityWithRelations`"))),',
        'serde_json::from_str(&content).map_err(Error::from),'
    ),
    (
        'return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::VulnerabilityWithComponents`"))),',
        'serde_json::from_str(&content).map_err(Error::from),'
    ),
    (
        'return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Advisory`"))),',
        'serde_json::from_str(&content).map_err(Error::from),'
    )
]

# Read file
text = file_path.read_text(encoding="utf-8")

# Replace
for old, new in replacements:
    text = text.replace(old, new)

# Save file
file_path.write_text(text, encoding="utf-8")