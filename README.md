## General
____________

### Author
* Josh McIntyre

### Website
* jmcintyre.net

### Overview
* HashHelper is a simple file hash verification tool

## Development
________________

### Git Workflow
* master for releases (merge development)
* development for bugfixes and new features

### Building
* make build
Build the application - wraps `cargo build`
* make clean
Clean the build directory

### Features
* Calculate the cryptographic hash of a file (SHA-256)
* Display the hash on the command line in hexadecimal format
* Includes 2 slightly different sample programs (to show how a file can be maliciously changed)

### Requirements
* Requires Rust development tools

### Platforms
* Windows
* MacOSX
* Linux

## Usage
____________

### Command Line Usage
* `./hashhelper <filename>` to show the SHA-256 hash of the provided file
