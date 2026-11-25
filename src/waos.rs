//! WaOS: an app's own folders, which WaOS gives every app at the same paths. `/data` is what it
//! keeps, `/cache` what it can lose, and `/tmp` what goes when the app ends.
//!
//! An app has no home folder, and none of the user's own (documents, downloads, pictures): it
//! reaches a folder of the user's only once the user gives it one, through a file dialog.

use std::path::PathBuf;

fn data() -> Option<PathBuf> {
    Some(PathBuf::from("/data"))
}

pub fn home_dir()         -> Option<PathBuf> { None }

pub fn cache_dir()        -> Option<PathBuf> { Some(PathBuf::from("/cache")) }
pub fn config_dir()       -> Option<PathBuf> { data() }
pub fn config_local_dir() -> Option<PathBuf> { data() }
pub fn data_dir()         -> Option<PathBuf> { data() }
pub fn data_local_dir()   -> Option<PathBuf> { data() }
pub fn preference_dir()   -> Option<PathBuf> { data() }
pub fn runtime_dir()      -> Option<PathBuf> { Some(PathBuf::from("/tmp")) }
pub fn executable_dir()   -> Option<PathBuf> { None }
pub fn state_dir()        -> Option<PathBuf> { data() }

pub fn audio_dir()        -> Option<PathBuf> { None }
pub fn desktop_dir()      -> Option<PathBuf> { None }
pub fn document_dir()     -> Option<PathBuf> { None }
pub fn download_dir()     -> Option<PathBuf> { None }
pub fn font_dir()         -> Option<PathBuf> { None }
pub fn picture_dir()      -> Option<PathBuf> { None }
pub fn public_dir()       -> Option<PathBuf> { None }
pub fn template_dir()     -> Option<PathBuf> { None }
pub fn video_dir()        -> Option<PathBuf> { None }
