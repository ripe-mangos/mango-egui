use egui::Hyperlink;

use crate::*;

#[macro_export]
macro_rules! version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

#[derive(Clone)]
pub struct InfoBarCtx {
    show_about: Arc<Mutex<bool>>,
    show_licenses: Arc<Mutex<bool>>,
}
impl Default for InfoBarCtx {
    fn default() -> Self {
        Self {
            show_about: Arc::new(Mutex::new(false)),
            show_licenses: Arc::new(Mutex::new(false)), 
        }
    }
}

#[derive(Clone)]
pub struct InfoBar {
    pub source_url: &'static str,
    pub bug_tracker_url: &'static str,
    /// Version of the application
    ///
    /// **TIP:** Use [version] to automatically get the crate's version
    pub version: &'static str,

    pub ctx: InfoBarCtx,
}
impl Default for InfoBar {
    fn default() -> Self {
        Self {
            source_url: "",
            bug_tracker_url: "",
            version: "",
            ctx: InfoBarCtx::default()
        }
    }
}
impl Widget for InfoBar {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        about(&mut *self.ctx.show_about.lock(), &ui.ctx());
        licenses(&mut *self.ctx.show_licenses.lock(), &ui.ctx()); 
        
        TopBottomPanel::bottom("info").show(ui.ctx(), |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("♥").color(Color32::RED));
                ui.label(format!("v{}", self.version));
                if ui.link("about").clicked() {
                    (*self.ctx.show_about.lock()) = true;
                }
                ui.hyperlink_to("sauce", self.source_url).on_hover_text("source code");
                ui.hyperlink_to("bugs", self.bug_tracker_url).on_hover_text("bug tracker");
                if ui.link("licenses").on_hover_text("licensing info for open source libraries").clicked() {
                    (*self.ctx.show_licenses.lock()) = true;
                }
            });
        }).response
    }
}

fn about(show: &mut bool, ctx: &Context) {
    if *show {
        Window::new("about").open(show).show(ctx, |ui| {
            ui.heading("Suzuharu");
            ui.label("Advanced desktop subtitle suite");
        });
    }
}

fn licenses(show: &mut bool, ctx: &Context) {
    if *show {
        Window::new("licenses").open(show).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Suzuharu");
                ui.weak("(GPL)");
            });
            ui.label("(c) 2022 Lincoln Williams");
        });
    }
}
