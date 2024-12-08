# gpui Examples

This repository contains examples of how to use gpui, a library designed to simplify handling graphical interfaces using the GPU. These examples are designed to run on Nix-compatible systems and are configured with a `flake.nix` file for easy building and execution.

## Project Structure

- **`assets/`**: Contains resources such as images and SVG graphics used by the examples.
  - `image/`: Images like PNG, GIF, and SVG.
  - `svg/`: Specific SVG graphics and their handling.
- **`examples/`**: Examples demonstrating gpui usage, each representing a specific functionality or use case.
- **`src/`**: Main application source code.
- **`flake.nix`**: Nix Flake configuration file that automates the building and execution of examples.
- **`rust-toolchain.toml`**: Specifies the Rust version used in the project.

## Available Examples

The examples are located in the `examples/` folder and can be executed easily. Some of the available examples include:

- `animation`: GPU animation handling.
- `gif_viewer`: GIF image viewer.
- `hello_world`: Basic introductory example.
- `input`: User input handling.
- `opacity`: Opacity manipulation example.
- `window`: Window creation and management.

For a complete list, check the `examples/` folder.

## Requirements

- **Nix**: This project uses Nix for dependency and environment management.
- **GPU-compatible hardware**: To run examples that utilize graphical acceleration.

## Usage Instructions

### Building and Running

1. **Build examples**:

   ```bash
   nix build .#packages
   ```

   This will generate binaries for all examples in the `./result` directory.

2. **Run an example**:
   ```bash
   nix run .#animation
   ```
   Replace `animation` with the name of the example you want to run.

### Development Environment

You can enter the development environment configured with all necessary dependencies:

```bash
nix develop
```

This sets up the environment with the required libraries and the appropriate Rust compiler.

## How the Flake Works

The `flake.nix` file:

1. Automatically detects all examples in the `examples/` folder.
2. Defines packages and applications for each example.
3. Sets up a development environment with necessary libraries such as `wayland`, `vulkan-loader`, and more.
4. Allows building and running each example independently with simple Nix commands.

## Contributions

This project is a starting point for experimenting with gpui. If you wish to contribute or add more examples, you can do so by following the existing structure.
