pub mod canvas;
use canvas::Canvas;

#[derive(Default)]
enum Instrument {
    #[default]
    Pencil,
    Bucket,
    ImageBucket,
    Border,
    SharpLine,
    SmoothLine,
    Triangle,
}

impl Instrument {
    fn get_name(&self) -> String {
        match self {
            Self::Pencil => String::from("карандаш"),
            Self::Bucket => String::from("заливка"),
            Self::ImageBucket => String::from("заливка картинкой"),
            Self::Border => String::from("выделение границ"),
            Self::SharpLine => String::from("линия"),
            Self::SmoothLine => String::from("размытая линия"),
            Self::Triangle => String::from("треугольник"),
        }
    }
}

#[derive(Default)]
pub struct ColorsApp {
    // обработка холста
    canvas: Canvas,
    needs_redraw: bool,
    texture_handle: Option<egui::TextureHandle>,

    // рисование на холсте
    cur_color: egui::Color32,
    points: Vec<egui::Pos2>,
    colors: Vec<egui::Color32>,
    cur_instrument: Instrument,
    loaded_image: Option<egui::ColorImage>,

    // создание нового холста
    show_new_canvas_popup: bool,
    new_canvas_width: usize,
    new_canvas_height: usize,

    // отоображение холста
    display_canvas_width: f32,
    display_canvas_height: f32,
}

// =============== Инициализация приложения ===============

impl ColorsApp {
    // Размеры холста при запуске приложения
    const INIT_CANVAS_WIDTH: usize = 160;
    const INIT_CANVAS_HEIGHT: usize = 90;

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_theme(egui::Theme::Light);
        let mut tmp = Self::default();
        tmp.canvas = Canvas::new(ColorsApp::INIT_CANVAS_WIDTH, ColorsApp::INIT_CANVAS_HEIGHT);
        tmp.needs_redraw = true;
        tmp.show_new_canvas_popup = false;
        tmp.cur_color = egui::Color32::BLACK;
        tmp
    }
}

// =============== Функции рисования ===============

impl ColorsApp {
    /// Обрабатывает рисование карандашом на холсте
    fn handle_pencil(&mut self, canvas_rect: egui::Rect, response: &egui::Response) {
        if response.dragged()
            && let Some(pointer_pos) = response.hover_pos()
        {
            if let Some(pos) = self.coord_screen_to_canvas(pointer_pos, canvas_rect) {
                self.canvas_mut(&response.ctx)[(pos.x as usize, pos.y as usize)] = self.cur_color;

                #[cfg(debug_assertions)]
                println!("нарисован пиксель {:#?} в {:#?}", self.cur_color, pos);
            }
        }
    }

    /// Обрабатывает заливку цветом
    fn handle_bucket(&mut self, canvas_rect: egui::Rect, response: &egui::Response) {
        if response.clicked()
            && let Some(pointer_pos) = response.hover_pos()
        {
            if let Some(pos) = self.coord_screen_to_canvas(pointer_pos, canvas_rect) {
                let color = self.cur_color;
                self.canvas_mut(&response.ctx).fill_with_color(
                    pos,
                    color,
                    canvas::Connectivity::EIGHT,
                );

                #[cfg(debug_assertions)]
                println!("заливка {:#?} в {:#?}", self.cur_color, pos);
            }
        }
    }

    /// Обрабатывает заливку картинкой
    fn handle_image_bucket(&mut self, canvas_rect: egui::Rect, response: &egui::Response) {
        if response.clicked()
            && let Some(pointer_pos) = response.hover_pos()
        {
            if let Some(pos) = self.coord_screen_to_canvas(pointer_pos, canvas_rect)
                && let Some(img) = self.loaded_image.clone()
            {
                self.canvas_mut(&response.ctx).fill_with_img(
                    pos,
                    &img,
                    canvas::Connectivity::EIGHT,
                );

                #[cfg(debug_assertions)]
                println!("заливка картинкой в {:#?}", pos);
            }
        }
    }

    /// Обрабатывает выделение границ
    fn handle_border(&mut self, canvas_rect: egui::Rect, response: &egui::Response) {
        if response.clicked()
            && let Some(pointer_pos) = response.hover_pos()
        {
            if let Some(pos) = self.coord_screen_to_canvas(pointer_pos, canvas_rect) {
                let boudary = self.canvas_mut(&response.ctx).trace_boundary(pos);
                let color = self.cur_color;
                self.canvas_mut(&response.ctx).draw_boundary(&boudary, color);

                #[cfg(debug_assertions)]
                println!("выделение границы в {:#?}", pos);
            }
        }
    }

    /// Обрабатывает рисование линии
    fn handle_sharp_line(&mut self, canvas_rect: egui::Rect, response: &egui::Response) {
        if response.clicked()
            && let Some(pointer_pos) = response.hover_pos()
        {
            if let Some(pos) = self.coord_screen_to_canvas(pointer_pos, canvas_rect) {
                if self.points.len() < 1 {
                    self.points.push(pos);
                    println!("поставлена точка линии в {:#?}", pos);
                    return;
                }
                let prev_pos = self.points.pop().unwrap();
                let color = self.cur_color;
                self.canvas_mut(&response.ctx)
                    .draw_sharp_line(prev_pos, pos, color);

                #[cfg(debug_assertions)]
                println!("нарисована линия цвета {:#?}", self.cur_color);
            }
        }
    }

    /// Обрабатывает рисование размытой линии
    fn handle_smooth_line(&mut self, canvas_rect: egui::Rect, response: &egui::Response) {
        if response.clicked()
            && let Some(pointer_pos) = response.hover_pos()
        {
            if let Some(pos) = self.coord_screen_to_canvas(pointer_pos, canvas_rect) {
                if self.points.len() < 1 {
                    self.points.push(pos);

                    #[cfg(debug_assertions)]
                    println!("поставлена точка линии в {:#?}", pos);
                    return;
                }
                let prev_pos = self.points.pop().unwrap();
                let color = self.cur_color;
                self.canvas_mut(&response.ctx)
                    .draw_smooth_line_simple(prev_pos, pos, color);

                #[cfg(debug_assertions)]
                println!("нарисована линия цвета {:#?}", self.cur_color);
            }
        }
    }

    /// Обрабатывает рисование градиентного треугольника
    fn handle_triangle(&mut self, canvas_rect: egui::Rect, response: &egui::Response) {
        if response.clicked()
            && let Some(pointer_pos) = response.hover_pos()
        {
            if let Some(pos) = self.coord_screen_to_canvas(pointer_pos, canvas_rect) {
                if self.points.len() < 2 {
                    self.points.push(pos);
                    self.colors.push(self.cur_color);

                    #[cfg(debug_assertions)]
                    println!(
                        "поставлена точка треугольника {:#?} в {:#?}",
                        self.cur_color, pos
                    );
                    return;
                }
                let pos1 = self.points.pop().unwrap();
                let color1 = self.colors.pop().unwrap();

                let pos2 = self.points.pop().unwrap();
                let color2 = self.colors.pop().unwrap();

                let color = self.cur_color;
                self.canvas_mut(&response.ctx)
                    .draw_gradient_triangle(pos1, pos2, pos, color1, color2, color);

                #[cfg(debug_assertions)]
                println!("нарисован треугольник");
            }
        }
    }
}

// =============== Обработка UI ===============

impl ColorsApp {
    /// Выделяет место под текущий холст и выводит его на весь текущий размер экрана.
    fn allocate_canvas(&self, ui: &mut egui::Ui) -> (egui::Response, egui::Painter) {
        let available_size = ui.available_size();
        let canvas_size = self.canvas.size();

        // Это было только для целочисленного увеличения
        // let width_ratio = (available_size.x / canvas_size[0] as f32).floor().max(1.0);
        // let height_ratio = (available_size.y / canvas_size[1] as f32).floor().max(1.0);
        // let canvas_to_display_ratio = width_ratio.min(height_ratio);

        // let display_width = canvas_size[0] as f32 * canvas_to_display_ratio;
        // let display_height = canvas_size[1] as f32 * canvas_to_display_ratio;

        let canvas_aspect_ratio = canvas_size[0] as f32 / canvas_size[1] as f32;

        let display_width = available_size.x.min(available_size.y * canvas_aspect_ratio);
        let display_height = display_width / canvas_aspect_ratio;

        let (canvas_response, painter) = ui.allocate_painter(
            egui::Vec2::new(display_width, display_height),
            egui::Sense::click_and_drag(),
        );
        return (canvas_response, painter);
    }

    /// Отображает PopUp с созданием холста нового размера.
    fn show_popup(&mut self, ctx: &egui::Context) {
        let popup_id = egui::Id::new("new_canvas_popup");
        let screen_rect = ctx.input(|i| i.screen_rect());

        egui::Popup::new(
            popup_id,
            ctx.clone(),
            egui::PopupAnchor::ParentRect(screen_rect),
            egui::LayerId::background(),
        )
        .align(egui::RectAlign::TOP_START)
        .frame(egui::Frame::window(&ctx.style()).inner_margin(10.0))
        .show(|ui| {
            ui.vertical(|ui| {
                ui.heading("New Canvas Size");

                ui.horizontal(|ui| {
                    ui.label("Width:");
                    ui.add(egui::DragValue::new(&mut self.new_canvas_width));
                });

                ui.horizontal(|ui| {
                    ui.label("Height:");
                    ui.add(egui::DragValue::new(&mut self.new_canvas_height));
                });

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        self.show_new_canvas_popup = false;
                    }

                    if ui.button("Create").clicked() {
                        self.canvas = Canvas::new(self.new_canvas_width, self.new_canvas_height);
                        self.needs_redraw = true;

                        self.show_new_canvas_popup = false;
                    }
                });
            });
        });
    }
}

// =============== Главный цикл UI ===============

impl eframe::App for ColorsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_texture(ctx);

        if self.show_new_canvas_popup {
            self.show_popup(ctx);
        }

        // --------------- Верхняя панель ---------------
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    // Создать новый холст
                    if ui.button("New").clicked() {
                        self.show_new_canvas_popup = true;
                        self.new_canvas_width = ColorsApp::INIT_CANVAS_WIDTH;
                        self.new_canvas_height = ColorsApp::INIT_CANVAS_HEIGHT;
                        ui.close();
                    }

                    // Загрузить картинку (для заливки картинкой)
                    if ui.button("Load Image").clicked() {
                        self.load_image(ctx);
                    }

                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });

        // --------------- Левая панель ---------------
        egui::SidePanel::left("left_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    egui::color_picker::color_picker_color32(
                        ui,
                        &mut self.cur_color,
                        egui::color_picker::Alpha::Opaque,
                    );

                    ui.separator();

                    ui.label("Инструменты:");

                    if ui.button("Pencil").clicked() {
                        self.switch_instrument(Instrument::Pencil);
                    }
                    if ui.button("Bucket").clicked() {
                        self.switch_instrument(Instrument::Bucket);
                    }
                    if ui.button("Image Bucket").clicked() {
                        self.switch_instrument(Instrument::ImageBucket);
                    }
                    if ui.button("Border").clicked() {
                        self.switch_instrument(Instrument::Border);
                    }
                    if ui.button("Sharp Line").clicked() {
                        self.switch_instrument(Instrument::SharpLine);
                    }
                    if ui.button("Smooth Line").clicked() {
                        self.switch_instrument(Instrument::SmoothLine);
                    }
                    if ui.button("Triangle").clicked() {
                        self.switch_instrument(Instrument::Triangle);
                    }
                });
            });

        // --------------- Нижняя панель ---------------
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("инструмент: {}", self.cur_instrument.get_name()));

                ui.separator();

                ui.label(format!(
                    "размер холста: {} x {}",
                    self.canvas.size()[0],
                    self.canvas.size()[1]
                ));

                ui.separator();

                ui.label(format!(
                    "отображаемый размер: {:.1} x {:.1}",
                    self.display_canvas_width, self.display_canvas_height
                ));
            });
        });

        // --------------- Центральная область ---------------
        egui::CentralPanel::default().show(ctx, |ui| {
            // Выделить область под холст
            let (canvas_response, painter) = self.allocate_canvas(ui);
            let canvas_rect = canvas_response.rect;

            self.display_canvas_width = canvas_rect.width();
            self.display_canvas_height = canvas_rect.height();

            // Обработать рисование
            match self.cur_instrument {
                Instrument::Pencil => self.handle_pencil(canvas_rect, &canvas_response),
                Instrument::Bucket => self.handle_bucket(canvas_rect, &canvas_response),
                Instrument::ImageBucket => self.handle_image_bucket(canvas_rect, &canvas_response),
                Instrument::Border => self.handle_border(canvas_rect, &canvas_response),
                Instrument::SharpLine => self.handle_sharp_line(canvas_rect, &canvas_response),
                Instrument::SmoothLine => self.handle_smooth_line(canvas_rect, &canvas_response),
                Instrument::Triangle => self.handle_triangle(canvas_rect, &canvas_response),
            };

            // Вывести текущий холст на экран
            if let Some(texture) = &self.texture_handle {
                painter.image(
                    texture.id(),
                    canvas_rect,
                    egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
                    egui::Color32::WHITE,
                );
            }
        });
    }
}

// =============== Вспомогательные функции ===============

impl ColorsApp {
    /// Обновить текущую GPU текстуру для отображения.
    fn update_texture(&mut self, ctx: &egui::Context) {
        if !self.needs_redraw {
            return;
        }

        self.texture_handle = Some(ctx.load_texture(
            "canvas",
            self.canvas.to_color_image(),
            egui::TextureOptions::NEAREST, // Linear слишком размытый для отображения мелких пикселей
        ));
        self.needs_redraw = false;
    }

    /// Получить изменяемый холст и пометить его на перерисовку.
    fn canvas_mut(&mut self, ctx: &egui::Context) -> &mut Canvas {
        self.needs_redraw = true;
        ctx.request_repaint(); // Холст изменён, надо заново его нарисовать
        &mut self.canvas
    }

    /// Преобразует координаты экрана в координаты холста
    fn coord_screen_to_canvas(
        &self,
        screen_pos: egui::Pos2,
        canvas_rect: egui::Rect,
    ) -> Option<egui::Pos2> {
        if canvas_rect.contains(screen_pos) {
            let relative_x = (screen_pos.x - canvas_rect.left()) / canvas_rect.width();
            let relative_y = (screen_pos.y - canvas_rect.top()) / canvas_rect.height();

            let canvas_size = self.canvas.size();
            let pixel_x = relative_x * canvas_size[0] as f32;
            let pixel_y = relative_y * canvas_size[1] as f32;
            return Some(egui::Pos2 {
                x: pixel_x,
                y: pixel_y,
            });
        }
        None
    }

    /// Сменить инструмент (рисование)
    fn switch_instrument(&mut self, new_instrument: Instrument) {
        self.cur_instrument = new_instrument;
        self.points.clear();
        self.colors.clear();
    }

    /// Загрузить файл с картинкой из файловой системы
    fn load_image(&mut self, ctx: &egui::Context) {
        let path = rfd::FileDialog::new()
            .add_filter("Images", &["png", "jpg", "jpeg", "bmp", "tga", "tiff"])
            .pick_file();

        if let Some(path) = path {
            if let Ok(img) = image::open(&path) {
                let image_size = [img.width() as usize, img.height() as usize];
                let image_buf = img.to_rgb8().into_raw();
                self.loaded_image = Some(egui::ColorImage::from_rgb(image_size, &image_buf));
            }
        }
    }
}
