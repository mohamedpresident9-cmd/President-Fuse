# Architecture proposal

## Application foundation

Use Rust and evaluate Fyrox as the initial engine/editor foundation. Start from its project template and integrate President Fuse functionality as an editor plugin; do not base the product on the standalone editor, which the project marks unsupported. Keep application-specific features behind modules so the engine can be replaced if prototyping reveals a mismatch.

## Major modules

- **App shell:** project selection, preferences, menus, platform integration.
- **Project and scene:** versioned native project metadata, scene hierarchy, stable asset references.
- **Viewport/editor:** selection, transforms, inspection, undoable editing actions.
- **Asset library:** searchable metadata, previews, import status, categories.
- **Character and rig tools:** skeleton inspection, rig authoring, clothing/skin bindings, later retargeting.
- **Animation:** local import, clip preview, rig compatibility checks, configurable website shortcuts.
- **Import/export:** format adapters; glTF/GLB as the initial interchange and game-runtime target.

## Project and asset storage

Keep application-specific editability in a versioned native project directory, provisionally `.pfuse/`, with scene and asset metadata. Treat GLB/glTF as interchange/export formats; round-tripping may not retain every application-only control or authoring detail. Large source assets can be referenced rather than duplicated when practical.

The initial implementation stores the project name, format version, asset category, source path, rigged flag, and configured animation-site shortcuts in `.pfuse/project.toml`. Asset paths under the project root are stored relatively; external source paths remain absolute references.

Use Git for code and compact text metadata. Put large binary models and textures in Git LFS or GitHub release assets, subject to the selected plan's limits. The exact repository and licensing policy remain to be selected.

## Cross-platform build

Maintain one Rust workspace and build/test Windows and Linux targets in CI. Isolate platform-specific file dialogs, browser launching, and packaging behind small interfaces. Package signed/reproducible releases only after the build pipeline and distribution requirements are defined.

## Reference analysis: original Fuse executable

Earlier REA inspection of `Fuse.exe` found a Qt concurrent task path that schedules runnable work through the global `QThreadPool` at priority 0. The upload path examined reads a selected file with `QIODevice::readAll()`, builds one multipart request with a `file_data` field, and sends one `QNetworkAccessManager::post` request while observing progress/completion signals. The available evidence did not show application-level multipart chunking or `Content-Range` requests for that path. Therefore, do not assume chunked upload is a compatibility requirement without additional runtime or endpoint evidence.

This is a behavioral reference for compatibility research, not a design mandate for President Fuse. Evidence was retained in the REA/Ghidra snapshot at `work/fuse-rea-ghidra-snapshot.json` in the separate REA checkout; consult it before making more detailed claims.

## References

- [Fyrox engine](https://github.com/FyroxEngine/Fyrox)
- [Fyrox project template](https://github.com/FyroxEngine/Fyrox/tree/master/template)
- [Fyrox standalone editor status](https://github.com/FyroxEngine/Fyrox/tree/master/editor-standalone)
- [glTF 2.0 specification](https://github.com/KhronosGroup/glTF/blob/main/specification/2.0/Specification.adoc)
- [Git LFS overview](https://docs.github.com/en/repositories/working-with-files/managing-large-files/about-git-large-file-storage)
