# Data Model: Paketron Registry

## Entities

### PackageManifest
Represents a single version of a package.

```toml
[package]
name = "7zip"
version = "23.01"
description = "7-Zip is a file archiver with a high compression ratio."
license = "LGPL-2.1"
homepage = "https://www.7-zip.org/"

[installer]
url = "https://www.7-zip.org/a/7z2301-x64.exe"
sha256 = "a1b2c3d4e5f6..."
type = "exe" # or "msi", "zip"
args = ["/S"] # Silent install arguments
```

### LocalRegistry
Tracks installed packages on the user's system.
Location: `%ProgramData%\Paketron\registry.json`

```json
{
  "packages": [
    {
      "name": "7zip",
      "version": "23.01",
      "installed_at": "2025-12-04T10:00:00Z",
      "files": [
        "C:\\Program Files\\7-Zip\\7z.exe",
        "..."
      ]
    }
  ]
}
```
