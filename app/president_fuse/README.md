# President Fuse Rust starter

This is a Fyrox 1.0.1 project-template scaffold with the President Fuse panel registered as an editor plugin. It gives the product a native Rust 3D editor foundation for Windows and Linux.

## Run

Install the Rust toolchain and the platform dependencies required by Fyrox, then from this directory run:

```powershell
cargo run --package editor --release
```

The generated project also includes executor and export-cli packages. The editor panel lets a user select a project folder, choose an asset category, create a cube, import GLB/glTF/FBX models into an open 3D scene, or choose an animation file from the computer.

Tagged releases build a Windows NSIS installer and Linux Debian/AppImage packages. The packaged editor locates its bundled starter scene relative to the installed application.

## Project metadata and domain model

The `game` crate contains product-level types and a versioned TOML project manifest. `ProjectManifest::create`, `load`, and `save` maintain `.pfuse/project.toml`; `import_asset` copies selected files into `.pfuse/assets` and registers them.

Animation file selection currently copies the file into the project library; clip decoding, preview, and rig retargeting are not implemented yet. Character/animal rig authoring and skin weighting are also not available yet. Website shortcuts are stored in the project manifest, and the panel opens the first configured HTTP(S) shortcut.

Add website shortcuts to `.pfuse/project.toml` like this, then reopen the project in the panel:

```toml
[[animation_sites]]
label = "My animation source"
url = "https://example.org/animations"
```
