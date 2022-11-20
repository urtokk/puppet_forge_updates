# puppet_forge_updates

Little helper to check for updates of your puppet modules.
It takes as an argument the path to your Puppetfile.
Its output is a list of modules that have updates available, and the new version.
The versions are colored according to the severity of the update.

Example:

![example-output](docs/images/example-output.png)

## Installation

### Cargo

  ```bash
  git clone https://github.com/urtokk/puppet_forge_updates.git
  cd puppet_forge_updates
  cargo install --path .
  ```
