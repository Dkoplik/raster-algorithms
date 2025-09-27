use eframe::egui;

pub mod canvas;

#[derive(Default)]
pub struct ColorsApp {}

impl ColorsApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_theme(egui::Theme::Light);
        Self::default()
    }
}

impl eframe::App for ColorsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // TODO
    }
}

/// Вывести 1 пиксель в указанной позиции указанного цвета на экран, используя painter
pub fn draw_pixel(x: f32, y: f32, color: egui::Color32, painter: &mut egui::Painter) {
    let pixel_rect = egui::Rect::from_min_size(
        egui::Pos2::new(x, y),
        egui::Vec2::splat(1.0),
    );
    painter.rect_filled(pixel_rect, 0.0, color);
}