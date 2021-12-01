use eframe::egui;
use std::f32::consts::TAU;

pub struct BezierDemo {
    animate: bool,
}

impl Default for BezierDemo {
    fn default() -> Self {
        Self { animate: false }
    }
}

impl BezierDemo {
    fn options_ui(&mut self, ui: &mut egui::Ui) {
        let Self { animate } = self;

        ui.horizontal(|ui| {
            ui.checkbox(animate, "animate");
        });
    }

    fn peace_sign(&self, ui: &mut egui::Ui, ctx: &egui::CtxRef) {
        let size = egui::Vec2::splat(256.0);
        let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());
        let rect = response.rect;
        let c = match ctx.input().pointer.hover_pos() {
            Some(pos) => pos,
            None => rect.center(),
        };

        let r = rect.width() / 3.0;
        let color = egui::Color32::from_gray(255);
        let stroke = egui::Stroke::new(5.0, color);
        painter.circle_stroke(c, r, stroke);
        painter.line_segment([c - egui::vec2(0.0, r), c + egui::vec2(0.0, r)], stroke);
        painter.line_segment([c, c + r * egui::Vec2::angled(TAU * 1.0 / 8.0)], stroke);
        painter.line_segment([c, c + r * egui::Vec2::angled(TAU * 3.0 / 8.0)], stroke);
    }

    pub fn ui(&mut self, ctx: &egui::CtxRef) {
        let Self { animate } = self;
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("Bezier Curve Applets");
            ui.hyperlink("https://github.com/chipfranzen/bezier");
            ui.add(egui::github_link_file!(
                "https://github.com/chipfranzen/bezier/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);

            self.options_ui(ui);
            if self.animate {
                ui.ctx().request_repaint();
            };
            self.peace_sign(ui, ctx);
        });
    }
}
