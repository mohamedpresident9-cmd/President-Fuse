# Roadmap

## Stage 0 — Product and technical foundation

- Confirm product scope, licensing, repository, and target Linux distributions.
- Prototype the selected Rust engine/editor integration and project save/load.
- Define the native project schema and initial GLB/glTF support boundaries.

**Progress:** Fyrox template generated; project panel, asset category selection, versioned TOML manifest, create/load/save, local asset copying/catalog, and configured animation-site launching implemented. Editor passes Windows `cargo check --locked`.

**Exit:** Windows and Linux CI checks pass and the application opens a project in the editor.

## Stage 1 — Editing MVP

- Project browser, 3D viewport, hierarchy, selection, and basic transforms.
- Undo/redo for supported edits and reliable save/reopen behavior.
- Import/export GLB/glTF and report unsupported content clearly.

**Exit:** a user can import a scene, make basic edits, save it, reopen it, and export a usable asset.

## Stage 2 — Animation workflow

- Add the **Animate** choice: local file import or configured browser shortcuts.
- Preview supported clips and check whether the selected rig is compatible.
- Document format and retargeting limitations.

**Exit:** supported animation files can be imported and previewed, with understandable failure messages for incompatible rigs.

## Stage 3 — Asset authoring and library

- Asset categories, previews, metadata, reusable project library, and modular building workflow.
- Improve materials, lighting, and viewport presentation.
- Add project templates and sample assets with clear licenses.

## Stage 4 — Rigs and specialized assets

- Character and animal skeleton/rig tools.
- Clothing attachment and skinning workflow.
- Vehicle controls and weapon attachment points where applicable.
- Export validation against selected game-engine targets.

## Stage 5 — Advanced authoring

- Expand modeling/procedural tools based on user research.
- Consider collaborative asset workflows and optional assisted generation.
- Preserve offline-first editing and transparent asset provenance.
