extern crate egui;

pub(crate) use egui::{
    Widget, Window, TopBottomPanel, RichText,
    Color32,
    Ui, Context, mutex::Mutex, Response,
};
pub(crate) use std::{sync::Arc, path::Path};

mod info_bar;
pub use info_bar::InfoBar;

mod file;
pub use file::{LoadFile, LoadFiles, SaveFile};
