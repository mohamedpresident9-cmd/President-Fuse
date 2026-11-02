# Product requirements

## Vision

Build **President Fuse**, a Rust desktop tool for authoring and adapting 3D assets for game engines. Users should be able to inspect and edit assets in a 3D workspace and export portable results.

## Users and outcomes

- Game developers need editable assets that can move into their chosen engine.
- Artists need a straightforward workspace for arranging, previewing, and modifying assets.
- Technical artists need predictable rig, animation, and export behavior.
- Teams working with legacy software need to inspect existing behavior and data flows before building compatible replacements.

## Asset scope

1. Characters, clothing, and character rigs.
2. Animals and animal rigs.
3. Vehicles and suitable rig controls.
4. Weapons, including optional rigging/attachment points.
5. Buildings and other static or modular environment assets.

## Core workflows

### 3D editing

- Create or open a project and scene.
- Import an asset, select objects, inspect hierarchy, and edit transforms.
- Preview materials, meshes, skeletons, and animation.
- Save editable project state and export a portable asset.

### Animate action

When the user presses **Animate**, offer two explicit choices:

1. **Import from this computer**: choose a supported animation file and map it to a compatible rig.
2. **Open animation sites**: show a configurable list of user-approved websites in the system browser. The app does not silently download content; the user downloads content and imports it through the first option.

Supported formats and retargeting behavior must be confirmed in implementation. The first delivery should support a narrow, documented animation interchange path rather than promise universal retargeting.

## MVP

- Rust desktop shell for Windows and Linux.
- Project creation/opening and a usable 3D viewport.
- Scene hierarchy and basic object selection/transforms.
- Import and export of GLB/glTF assets.
- Save/reopen application-specific editable project data.
- Animate menu with local import and configurable website shortcuts.
- Clear errors for unsupported files and incompatible skeletons.

Implemented foundation so far: project folder selection, create/load manifest, choose an asset category, copy supported GLB/glTF/FBX/OBJ assets into the project library, choose a GLB/glTF/FBX animation file for library storage, and open the first user-configured HTTP(S) animation-site shortcut. Advanced modeling, scene placement of imported assets, animation decoding/preview/retargeting, procedural asset generation, character/animal authoring, full rig editing, and asset marketplaces remain later milestones.

## Compatibility and quality

- Keep the native project format versioned and migrateable.
- Preserve stable identifiers and hierarchy where possible during import/export.
- Validate exported files with independent glTF tooling during implementation.
- Document supported operating systems and file-format limitations.
- Keep large optional assets outside normal source history using Git LFS or release downloads.
