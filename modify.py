from pathlib import Path

# Pfad zur Datei
file_path = Path("src/apis/default_api.rs")

# Liste von (suche, ersetze)-Tupeln
replacements = [
    (
        'return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::Vulnerability&gt;`"))),',
        'serde_json::from_str(&content).map_err(Error::from),'
    ),
    (
        'return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Vulnerabilities`"))),',
        'serde_json::from_str(&content).map_err(Error::from),'
    )
]

# Read file
lines = file_path.read_text(encoding="utf-8").splitlines()

# Replace lines
new_lines = []

for line in lines:

    for old, new in replacements:

        if old in line:
            line = line.replace(old, new)

    new_lines.append(line)

# Save file
file_path.write_text("\n".join(new_lines) + "\n", encoding="utf-8")