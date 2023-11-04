use crate::pages::types::PageType;
use crate::types::screen_grabber::ScreenGrabber;
use egui::panel::Side;

pub fn launcher_page(app: &mut ScreenGrabber, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    let window_size = ctx.screen_rect().width();
    egui::containers::CentralPanel::default().show(ctx, |_ui| {
        // egui::TopBottomPanel::new(TopBottomSide::Top, "header").show(ctx, |ui|{
        //    ui.with_layout(Layout::right_to_left(Align::Max), |ui|{
        //       //TODO: add top left button for settings
        //    });
        // });
        egui::SidePanel::new(Side::Left, "image_panel")
            .resizable(false)
            .exact_width(window_size * 0.7)
            .show(ctx, |ui| ui.label("image/empty rectangle"));
        egui::CentralPanel::default().show(ctx, |ui| {
            let max = ui.max_rect();
            ui.allocate_ui_at_rect(max, |ui| {
                ui.label("Launcher page");
                if ui.button("Capture").clicked() {
                    app.set_page(PageType::Capture)
                }
                if ui.button("Settings").clicked() {
                    app.set_page(PageType::Settings)
                }
            });
        });
    });
}
