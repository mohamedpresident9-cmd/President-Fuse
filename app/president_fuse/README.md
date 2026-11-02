# President Fuse Rust starter

This is a Fyrox 1.0.1 project-template scaffold with the President Fuse panel registered as an editor plugin. It gives the product a native Rust 3D editor foundation for Windows and Linux.

## Run

Install the Rust toolchain and the platform dependencies required by Fyrox, then from this directory run:

```powershell
cargo run --package editor --release
```

The generated project also includes executor and export-cli packages. The editor panel currently lets a user select a project folder, choose an asset category, add supported 3D files to its library, or choose an animation file from the computer.

## Project metadata and domain model

The `game` crate contains product-level types and a versioned TOML project manifest. `ProjectManifest::create`, `load`, and `save` maintain `.pfuse/project.toml`; `import_asset` copies selected files into `.pfuse/assets` and registers them.

Animation file selection currently copies the file into the project library; clip decoding, preview, and rig retargeting are not implemented yet. Website shortcuts are stored in the project manifest, and the panel opens the first configured HTTP(S) shortcut.

Add website shortcuts to `.pfuse/project.toml` like this, then reopen the project in the panel:

```toml
[[animation_sites]]
label = "My animation source"
url = "https://example.org/animations"
```
