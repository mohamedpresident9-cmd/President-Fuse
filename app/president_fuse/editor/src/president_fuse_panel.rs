use fyrox::core::pool::Handle;
use fyrox::gui::{
    button::{Button, ButtonBuilder, ButtonMessage},
    dock::DockingManagerMessage,
    dropdown_list::{DropdownList, DropdownListBuilder, DropdownListMessage},
    file_browser::{
        FileSelector, FileSelectorBuilder, FileSelectorMessage, FileSelectorMode, FileType,
        PathFilter,
    },
    message::UiMessage,
    stack_panel::StackPanelBuilder,
    text::{Text, TextBuilder, TextMessage},
    widget::WidgetBuilder,
    window::{Window, WindowAlignment, WindowBuilder, WindowMessage, WindowTitle},
    Orientation,
};
use fyroxed_base::{plugin::EditorPlugin, Editor};
use president_fuse::product::{AssetKind, ProjectManifest};
use std::path::PathBuf;

#[derive(Clone, Copy)]
enum DialogPurpose {
    ProjectFolder,
    AssetFile,
    AnimationFile,
}

#[derive(Default)]
pub struct PresidentFusePanel {
    window: Handle<Window>,
    project_button: Handle<Button>,
    asset_kind_dropdown: Handle<DropdownList>,
    asset_button: Handle<Button>,
    animation_button: Handle<Button>,
    websites_button: Handle<Button>,
    status: Handle<Text>,
    active_dialog: Option<(Handle<FileSelector>, DialogPurpose)>,
    project_dir: Option<PathBuf>,
    manifest: Option<ProjectManifest>,
    selected_asset_kind: AssetKind,
}

impl PresidentFusePanel {
    fn set_status(&self, editor: &mut Editor, message: impl Into<String>) {
        editor
            .engine
            .user_interfaces
            .first_mut()
            .send(self.status, TextMessage::Text(message.into()));
    }

    fn open_selector(&mut self, editor: &mut Editor, purpose: DialogPurpose) {
        let ui = editor.engine.user_interfaces.first_mut();
        let mut context = ui.build_ctx();
        let dialog = match purpose {
            DialogPurpose::ProjectFolder => FileSelectorBuilder::new(
                WindowBuilder::new(WidgetBuilder::new().with_width(640.0).with_height(440.0))
                    .open(false)
                    .can_minimize(false),
            )
            .with_path(std::env::current_dir().unwrap_or_default())
            .with_filter(PathFilter::folder())
            .with_mode(FileSelectorMode::Open)
            .build(&mut context),
            DialogPurpose::AssetFile | DialogPurpose::AnimationFile => {
                let mut filter = PathFilter::new()
                    .with_file_type(FileType::new_extension("glb").with_description("glTF binary"))
                    .with_file_type(FileType::new_extension("gltf").with_description("glTF scene"))
                    .with_file_type(
                        FileType::new_extension("fbx").with_description("FBX scene/animation"),
                    );
                if matches!(purpose, DialogPurpose::AssetFile) {
                    filter = filter.with_file_type(
                        FileType::new_extension("obj").with_description("Wavefront OBJ"),
                    );
                }
                FileSelectorBuilder::new(
                    WindowBuilder::new(WidgetBuilder::new().with_width(720.0).with_height(480.0))
                        .open(false)
                        .can_minimize(false),
                )
                .with_path(std::env::current_dir().unwrap_or_default())
                .with_filter(filter)
                .with_mode(FileSelectorMode::Open)
                .build(&mut context)
            }
        };
        drop(context);
        ui.send(
            dialog,
            WindowMessage::Open {
                alignment: WindowAlignment::Center,
                modal: true,
                focus_content: true,
            },
        );
        self.active_dialog = Some((dialog, purpose));
    }

    fn on_path_selected(&mut self, editor: &mut Editor, purpose: DialogPurpose, path: PathBuf) {
        match purpose {
            DialogPurpose::ProjectFolder => self.open_project(editor, path),
            DialogPurpose::AssetFile | DialogPurpose::AnimationFile => {
                let Some(project_dir) = self.project_dir.as_deref() else {
                    self.set_status(editor, "Choose a project folder before importing files.");
                    return;
                };
                let Some(manifest) = self.manifest.as_mut() else {
                    self.set_status(editor, "The project manifest is not loaded.");
                    return;
                };
                let kind = match purpose {
                    DialogPurpose::AssetFile => self.selected_asset_kind,
                    DialogPurpose::AnimationFile => AssetKind::Animation,
                    DialogPurpose::ProjectFolder => unreachable!(),
                };
                match manifest.import_asset(project_dir, &path, kind) {
                    Ok(asset) => match manifest.save(project_dir) {
                        Ok(()) => {
                            let detail = if matches!(purpose, DialogPurpose::AnimationFile) {
                                "Animation file copied into the project library. Clip parsing and rig retargeting are not wired yet."
                            } else {
                                "Asset copied into the project library. Scene placement is the next workflow step."
                            };
                            self.set_status(editor, format!("{} — {detail}", asset.name));
                        }
                        Err(error) => self.set_status(
                            editor,
                            format!("Could not save project manifest: {error}"),
                        ),
                    },
                    Err(error) => {
                        self.set_status(editor, format!("Could not import file: {error}"))
                    }
                }
            }
        }
    }

    fn open_project(&mut self, editor: &mut Editor, directory: PathBuf) {
        let loaded = if directory.join(".pfuse").join("project.toml").is_file() {
            ProjectManifest::load(&directory)
        } else {
            let name = directory
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("President Fuse Project");
            ProjectManifest::create(&directory, name)
        };

        match loaded {
            Ok(manifest) => {
                let name = manifest.name.clone();
                self.project_dir = Some(directory);
                self.manifest = Some(manifest);
                self.set_status(editor, format!("Project ready: {name}"));
            }
            Err(error) => {
                self.set_status(editor, format!("Could not open or create project: {error}"))
            }
        }
    }

    fn open_animation_website(&mut self, editor: &mut Editor) {
        let Some(manifest) = self.manifest.as_ref() else {
            self.set_status(
                editor,
                "Choose a project first; animation sites are stored in its manifest.",
            );
            return;
        };
        let Some(site) = manifest.animation_sites.first() else {
            self.set_status(editor, "No animation sites configured. Add one to .pfuse/project.toml, then reload the project.");
            return;
        };
        if !(site.url.starts_with("https://") || site.url.starts_with("http://")) {
            self.set_status(
                editor,
                "The saved site URL must start with http:// or https://.",
            );
            return;
        }
        match open::that(&site.url) {
            Ok(()) => self.set_status(editor, format!("Opened animation site: {}", site.label)),
            Err(error) => self.set_status(editor, format!("Could not open site: {error}")),
        }
    }
}

impl EditorPlugin for PresidentFusePanel {
    fn on_start(&mut self, editor: &mut Editor) {
        let ui = editor.engine.user_interfaces.first_mut();
        let mut context = ui.build_ctx();

        self.project_button = ButtonBuilder::new(
            WidgetBuilder::new()
                .with_width(290.0)
                .with_height(30.0)
                .with_margin(fyrox::gui::Thickness::uniform(3.0)),
        )
        .with_text("Open or Create Project")
        .build(&mut context);
        let asset_kind_names = [
            "Character",
            "Clothing",
            "Animal",
            "Vehicle",
            "Weapon",
            "Building",
            "Other",
        ];
        let asset_kind_items = asset_kind_names
            .iter()
            .map(|name| {
                TextBuilder::new(WidgetBuilder::new())
                    .with_text(*name)
                    .build(&mut context)
                    .to_base()
            })
            .collect();
        self.asset_kind_dropdown = DropdownListBuilder::new(
            WidgetBuilder::new()
                .with_width(290.0)
                .with_height(28.0)
                .with_margin(fyrox::gui::Thickness::uniform(3.0)),
        )
        .with_items(asset_kind_items)
        .with_selected(0)
        .build(&mut context);
        self.asset_button = ButtonBuilder::new(
            WidgetBuilder::new()
                .with_width(290.0)
                .with_height(30.0)
                .with_margin(fyrox::gui::Thickness::uniform(3.0)),
        )
        .with_text("Add 3D Asset to Library")
        .build(&mut context);
        self.animation_button = ButtonBuilder::new(
            WidgetBuilder::new()
                .with_width(290.0)
                .with_height(30.0)
                .with_margin(fyrox::gui::Thickness::uniform(3.0)),
        )
        .with_text("Animate: Import from Computer")
        .build(&mut context);
        self.websites_button = ButtonBuilder::new(
            WidgetBuilder::new()
                .with_width(290.0)
                .with_height(30.0)
                .with_margin(fyrox::gui::Thickness::uniform(3.0)),
        )
        .with_text("Animate: Open Animation Site")
        .build(&mut context);
        self.status = TextBuilder::new(
            WidgetBuilder::new()
                .with_width(290.0)
                .with_margin(fyrox::gui::Thickness::uniform(5.0)),
        )
        .with_text("Choose or create a President Fuse project to begin.")
        .build(&mut context);

        let content = StackPanelBuilder::new(
            WidgetBuilder::new()
                .with_child(self.project_button)
                .with_child(self.asset_kind_dropdown)
                .with_child(self.asset_button)
                .with_child(self.animation_button)
                .with_child(self.websites_button)
                .with_child(self.status),
        )
        .with_orientation(Orientation::Vertical)
        .build(&mut context);

        self.window = WindowBuilder::new(WidgetBuilder::new().with_width(330.0).with_height(260.0))
            .with_title(WindowTitle::text("President Fuse"))
            .with_content(content)
            .open(false)
            .build(&mut context);

        drop(context);
        ui.send(
            self.window,
            WindowMessage::Open {
                alignment: WindowAlignment::Center,
                modal: false,
                focus_content: false,
            },
        );
        ui.send(
            editor.docking_manager,
            DockingManagerMessage::AddFloatingWindow(self.window),
        );
    }

    fn on_ui_message(&mut self, message: &mut UiMessage, editor: &mut Editor) {
        if let Some(FileSelectorMessage::Commit(path)) = message.data() {
            if let Some((dialog, purpose)) = self.active_dialog {
                if message.destination() == dialog {
                    self.active_dialog = None;
                    self.on_path_selected(editor, purpose, path.clone());
                    return;
                }
            }
        }

        if message.destination() == self.asset_kind_dropdown {
            if let Some(DropdownListMessage::Selection(Some(index))) = message.data() {
                self.selected_asset_kind = match index {
                    0 => AssetKind::Character,
                    1 => AssetKind::Clothing,
                    2 => AssetKind::Animal,
                    3 => AssetKind::Vehicle,
                    4 => AssetKind::Weapon,
                    5 => AssetKind::Building,
                    _ => AssetKind::Other,
                };
            }
        }

        if let Some(ButtonMessage::Click) = message.data() {
            if message.destination() == self.project_button {
                self.open_selector(editor, DialogPurpose::ProjectFolder);
            } else if message.destination() == self.asset_button {
                self.open_selector(editor, DialogPurpose::AssetFile);
            } else if message.destination() == self.animation_button {
                self.open_selector(editor, DialogPurpose::AnimationFile);
            } else if message.destination() == self.websites_button {
                self.open_animation_website(editor);
            }
        }
    }
}
