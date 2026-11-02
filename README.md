# President Fuse

**President Fuse** is a proposed Rust-based 3D asset authoring application for Windows and Linux. The goal is to create and edit game-ready characters, clothing, animals, rigs, vehicles, weapons, and buildings in a 3D workspace.

## Product direction

- A 3D viewport with editable scenes and assets.
- Character and animal creation, clothing, and rigging workflows.
- Rigging support for characters, vehicles, and weapons where useful.
- Game-engine-friendly export, initially through glTF/GLB.
- An **Animate** action offering local animation import or opening user-configured animation websites.
- A Rust codebase targeting Windows and Linux.

This repository contains a generated Fyrox 3D editor/game-plugin starter under `app/president_fuse`, initial product specifications, a project panel, and a native project-manifest API. Users can create/open projects and copy 3D or animation files into the project's asset library. The custom 3D authoring workflows are still at an early foundation stage; this is not yet the finished asset editor.

## Documents

- [Product requirements](docs/PRODUCT_REQUIREMENTS.md)
- [Architecture proposal](docs/ARCHITECTURE.md)
- [Roadmap](docs/ROADMAP.md)
- [Rust application starter](app/president_fuse/README.md)

## Build check

The `editor` project passed `cargo check --locked` on Windows. A GitHub Actions workflow is included to check the editor project on both Windows and Linux.

## Initial technology direction

Fyrox is a Rust 2D/3D engine with an editor. Its project template supports an editor plugin alongside the game and executor projects, which is the preferred starting point for an integrated product. The standalone editor is documented as unsupported, so this project should not depend on it. See [Fyrox](https://github.com/FyroxEngine/Fyrox), its [template](https://github.com/FyroxEngine/Fyrox/tree/master/template), and [standalone editor notice](https://github.com/FyroxEngine/Fyrox/tree/master/editor-standalone).

Use an editable native project format for authoring data and glTF/GLB for interchange and runtime export. glTF supports scene nodes, PBR materials, skinning joints and weights, and animation channels; it is not a complete substitute for preserving application-specific authoring state. See the [glTF 2.0 specification](https://github.com/KhronosGroup/glTF/blob/main/specification/2.0/Specification.adoc).

## GitHub storage

Keep source code and compact project metadata in Git. Use Git LFS or release assets for large models, textures, and sample content; avoid placing large binary assets directly in ordinary Git history. See [Git LFS](https://docs.github.com/en/repositories/working-with-files/managing-large-files/about-git-large-file-storage).

## Repository status

Source repository: [mohamedpresident9-cmd/President-Fuse](https://github.com/mohamedpresident9-cmd/President-Fuse). The repository is public by request. Local Android signing keys and build caches are excluded from source control.
