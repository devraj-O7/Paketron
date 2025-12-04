# CLI Contracts

## Commands

### `install`
Installs one or more packages.

**Usage**: `paketron install <PACKAGE_NAME>... [FLAGS]`

**Arguments**:
- `<PACKAGE_NAME>`: Name of the package(s) to install.

**Flags**:
- `--version <VERSION>`: Install a specific version.
- `--force`: Force re-installation.
- `--yes` (`-y`): Skip confirmation prompts.

**Output**:
- Success: `Successfully installed <PACKAGE> v<VERSION>`
- Error: `Package <PACKAGE> not found` or `Checksum mismatch`

### `uninstall`
Removes a package.

**Usage**: `paketron uninstall <PACKAGE_NAME>...`

**Output**:
- Success: `Successfully removed <PACKAGE>`

### `search`
Searches for packages in the remote registry.

**Usage**: `paketron search <QUERY>`

**Output**:
- List of matching packages with descriptions.

### `list`
Lists installed packages.

**Usage**: `paketron list`

**Output**:
- Table of installed packages and versions.
