use crate::*;
use rfd::FileDialog;

#[derive(Clone)]
pub struct LoadFile {
    label: &'static str,
    path: Arc<Mutex<String>>,
}
impl LoadFile {
    pub fn new(label: &'static str) -> Self {
        Self { label, path: Arc::new(Mutex::new(String::new())) }
    }
    pub fn path(&mut self) -> String {
        let label = self.label.clone();
        let path = (*self.path.lock()).clone();
        *self = Self::new(label);
        path
    }
}
impl Widget for LoadFile {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.label(self.label);
        ui.horizontal(|ui| {
            if ui.button("choose").clicked() {
                let p = FileDialog::new().pick_file();
                if p.is_some() {
                    (*self.path.lock()) = p.unwrap().to_str().unwrap().to_string();
                }
            }
            ui.label(Path::new((*self.path.lock()).as_str()).file_name().unwrap_or_default().to_str().unwrap());
        }).response
    }
}

#[derive(Clone)]
pub struct LoadFiles {
    label: &'static str,
    paths: Arc<Mutex<Vec<String>>>,
}
impl LoadFiles {
    pub fn new(label: &'static str) -> Self {
        Self { label, paths: Arc::new(Mutex::new(Vec::new())) }
    }
    pub fn paths(&mut self) -> Vec<String> {
        let label = self.label.clone();
        let paths = (*self.paths.lock()).clone();
        *self = Self::new(label);
        paths
    }
}
impl Widget for LoadFiles {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.label(self.label);
        ui.horizontal(|ui| {
            if ui.button("choose").clicked() {
                let p = FileDialog::new().pick_files();
                if p.is_some() {
                    (*self.paths.lock()).extend(p.unwrap().iter().map(|x| x.to_str().unwrap().to_string()).collect::<Vec<String>>());
                }
            }
            ui.label(format!("{} files", (*self.paths.lock()).len()));
        }).response
    }
}

#[derive(Clone)]
pub struct SaveFile {
    label: &'static str,
    path: Arc<Mutex<String>>,
}
impl SaveFile {
    pub fn new(label: &'static str) -> Self {
        Self { label, path: Arc::new(Mutex::new(String::new())) }
    }
    pub fn path(&mut self) -> String {
        let label = self.label.clone();
        let path = (*self.path.lock()).clone();
        *self = Self::new(label);
        path
    }
}
impl Widget for SaveFile {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.label(self.label);
        ui.horizontal(|ui| {
            if ui.button("choose").clicked() {
                let p = FileDialog::new().save_file();
                if p.is_some() {
                    (*self.path.lock()) = p.unwrap().to_str().unwrap().to_string();
                }
            }
            ui.label(Path::new((*self.path.lock()).as_str()).file_name().unwrap_or_default().to_str().unwrap());
        }).response
    }
}
