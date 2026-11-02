//! Editor with your game connected to it as a plugin.
mod president_fuse_panel;

use fyroxed_base::{fyrox::core::log::Log, fyrox::event_loop::EventLoop, Editor, StartupData};
use president_fuse_panel::PresidentFusePanel;
use std::path::PathBuf;

fn application_resources() -> PathBuf {
    let executable_directory = std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(ToOwned::to_owned));

    if let Some(directory) = executable_directory {
        let candidates = [
            directory.join("data"),
            directory.join("president-fuse").join("data"),
            directory
                .parent()
                .unwrap_or(&directory)
                .join("lib")
                .join("president-fuse")
                .join("data"),
        ];
        if let Some(resources) = candidates
            .into_iter()
            .find(|path| path.join("scene.rgs").is_file())
        {
            return resources;
        }
    }

    std::env::current_dir().unwrap_or_default()
}

fn main() {
    Log::set_file_name("president_fuse.log");

    let event_loop = EventLoop::new().unwrap();
    let resources = application_resources();
    let mut editor = Editor::new(Some(StartupData {
        working_directory: resources.clone(),
        scenes: vec![resources.join("scene.rgs")],
        named_objects: false,
    }));
    editor.add_editor_plugin(PresidentFusePanel::default());

    // Dynamic linking with hot reloading.
    #[cfg(feature = "dylib")]
    {
        #[cfg(target_os = "windows")]
        let file_name = "game_dylib.dll";
        #[cfg(target_os = "linux")]
        let file_name = "libgame_dylib.so";
        #[cfg(target_os = "macos")]
        let file_name = "libgame_dylib.dylib";
        editor.add_dynamic_plugin(file_name, true, true).unwrap();
    }

    // Static linking.
    #[cfg(not(feature = "dylib"))]
    {
        use president_fuse::Game;
        editor.add_game_plugin(Game::default());
    }

    editor.run(event_loop)
}
