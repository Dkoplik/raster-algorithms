use std::ops::{Index, IndexMut};

use egui::{Color32, ColorImage, Pos2};

use std::collections::VecDeque;

/// Вариант связности, нужен для заливки.
pub enum Connectivity {
    /// 4-х связная заливка
    FOUR,
    /// 8-ми связная заливка
    EIGHT,
}

// =============== Реализация холста ===============

#[derive(Default)]
pub struct Canvas {
    pixels: Vec<Color32>,
    width: usize,
    height: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![Color32::WHITE; width * height],
            width,
            height,
        }
    }

    #[inline]
    /// Проверить границы полотна.
    fn check_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
}

// =============== Доступ к отдельным пикселям холста ===============

impl Index<(usize, usize)> for Canvas {
    type Output = Color32;

    // index = (x, y)
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        self.check_bounds(x, y);
        &self.pixels[y * self.width + x]
    }
}

impl IndexMut<(usize, usize)> for Canvas {
    // index = (x, y)
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index;
        self.check_bounds(x, y);
        &mut self.pixels[y * self.width + x]
    }
}

// =============== Растровые алгоритмы над холстом ===============

// Задание 1 (всякие заливки)
impl Canvas {
    // Сюда можно приватные вспомогательные методы, если нужно

    /// Вспомогательная функция для нахождения границ линии
    fn find_line_bounds(&self, x: usize, y: usize, old_color: Color32) -> (usize, usize) {
        let mut left = x;
        let mut right = x;

        while self.check_bounds(left, y) && self[(left, y)] == old_color {
            left -= 1;
        }
        left += 1;

        while self.check_bounds(right, y) && self[(right, y)] == old_color {
            right += 1;
        }
        right -= 1;

        (left, right)
    }

    /// Вспомогательная функция для проверки и добавления точки в стек
    fn check_and_push(
        &self,
        x: usize,
        y: usize,
        old_color: Color32,
        stack: &mut VecDeque<(usize, usize)>,
    ) {
        if self.check_bounds(x, y) && self[(x, y)] == old_color {
            stack.push_back((x, y));
        }
    }

    /// Рекурсивная заливка изображения.
    /// pos - позиция, в которой применяется заливка;
    /// color - цвет заливки;
    /// connectivity - тип заливки (4-х или 8-ми связная);
    pub fn fill_with_color(&mut self, pos: Pos2, color: Color32, connectivity: Connectivity) {
        let start_x = pos.x as usize;
        let start_y = pos.y as usize;

        if !self.check_bounds(start_x, start_y) {
            return;
        }

        let old_color = self[(start_x, start_y)];
        if old_color == color {
            return;
        }

        let mut stack = VecDeque::new();
        stack.push_back((start_x, start_y));

        while let Some((x, y)) = stack.pop_front() {
            if !self.check_bounds(x, y) || self[(x, y)] != old_color {
                continue;
            }

            let (left, right) = self.find_line_bounds(x, y, old_color);

            for i in left..=right {
                if self.check_bounds(i, y) {
                    self[(i, y)] = color;
                }
            }

            match connectivity {
                Connectivity::FOUR => {
                    for i in left..=right {
                        if y > 0 {
                            self.check_and_push(i, y - 1, old_color, &mut stack);
                        }
                        self.check_and_push(i, y + 1, old_color, &mut stack);
                    }
                }
                Connectivity::EIGHT => {
                    for i in left..=right {
                        if y > 0 {
                            self.check_and_push(i, y - 1, old_color, &mut stack);
                        }
                        self.check_and_push(i, y + 1, old_color, &mut stack);
                    }

                    if y > 0 {
                        if left > 0 {
                            self.check_and_push(left - 1, y - 1, old_color, &mut stack);
                        }
                        self.check_and_push(right + 1, y - 1, old_color, &mut stack);
                    }
                    if left > 0 {
                        self.check_and_push(left - 1, y + 1, old_color, &mut stack);
                    }
                    self.check_and_push(right + 1, y + 1, old_color, &mut stack);
                }
            }
        }
    }

    /// Рекурсивная заливка изображения.
    /// pos - позиция, в которой применяется заливка;
    /// img - изображение для заливки;
    /// connectivity - тип заливки (4-х или 8-ми связная);
    pub fn fill_with_img(&mut self, pos: Pos2, img: &ColorImage, connectivity: Connectivity) {
        let start_x = pos.x as usize;
        let start_y = pos.y as usize;

        if !self.check_bounds(start_x, start_y) {
            return;
        }

        let old_color = self[(start_x, start_y)];
        let img_width = img.width();
        let img_height = img.height();

        let mut stack = VecDeque::new();
        stack.push_back((start_x, start_y));

        while let Some((x, y)) = stack.pop_front() {
            if !self.check_bounds(x, y) || self[(x, y)] != old_color {
                continue;
            }

            let (left, right) = self.find_line_bounds(x, y, old_color);

            for i in left..=right {
                if self.check_bounds(i, y) {
                    let img_x = (i - start_x).rem_euclid(img_width);
                    let img_y = (y - start_y).rem_euclid(img_height);

                    if img_x < img_width && img_y < img_height {
                        self[(i, y)] = img[(img_x, img_y)];
                    }
                }
            }

            match connectivity {
                Connectivity::FOUR => {
                    for i in left..=right {
                        if y > 0 {
                            self.check_and_push(i, y - 1, old_color, &mut stack);
                        }
                        self.check_and_push(i, y + 1, old_color, &mut stack);
                    }
                }
                Connectivity::EIGHT => {
                    for i in left..=right {
                        if y > 0 {
                            self.check_and_push(i, y - 1, old_color, &mut stack);
                        }
                        self.check_and_push(i, y + 1, old_color, &mut stack);
                    }

                    if y > 0 {
                        if left > 0 {
                            self.check_and_push(left - 1, y - 1, old_color, &mut stack);
                        }
                        self.check_and_push(right + 1, y - 1, old_color, &mut stack);
                    }
                    if left > 0 {
                        self.check_and_push(left - 1, y + 1, old_color, &mut stack);
                    }
                    self.check_and_push(right + 1, y + 1, old_color, &mut stack);
                }
            }
        }
    }
}

// Задание 2 (линии)
impl Canvas {
    /// Рисование линии алгоритмом Брезенхема.
    /// pos1 - первая точка линии;
    /// pos2 - вторая точка линии;
    /// color - цвет линии;
    pub fn draw_sharp_line(&mut self, pos1: Pos2, pos2: Pos2, color: Color32) {
        let mut x0 = pos1.x as i32;
        let mut y0 = pos1.y as i32;
        let mut x1 = pos2.x as i32;
        let mut y1 = pos2.y as i32;

        let steep = (y1 - y0).abs() > (x1 - x0).abs();

        if steep {
            std::mem::swap(&mut x0, &mut y0);
            std::mem::swap(&mut x1, &mut y1);
        }

        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }

        let deltax = x1 - x0;
        let deltay = (y1 - y0).abs();

        let mut error = 0;
        let deltaerr = deltay + 1;
        let mut y = y0;
        let diry = if y1 > y0 { 1 } else { -1 };

        for x in x0..=x1 {
            if steep {
                self[(y as usize, x as usize)] = color;
            } else {
                self[(x as usize, y as usize)] = color;
            }

            error += deltaerr;
            if error >= (deltax + 1) {
                y += diry;
                error -= deltax + 1;
            }
        }
    }

    /// Рисование линии алгоритмом Ву.
    /// pos1 - первая точка линии;
    /// pos2 - вторая точка линии;
    /// color - цвет линии;
    pub fn draw_smooth_line_simple(&mut self, pos1: Pos2, pos2: Pos2, color: Color32) {
        let mut x1 = pos1.x;
        let mut y1 = pos1.y;
        let mut x2 = pos2.x;
        let mut y2 = pos2.y;

        let steep = (y2 - y1).abs() > (x2 - x1).abs();
        if steep {
            std::mem::swap(&mut x1, &mut y1);
            std::mem::swap(&mut x2, &mut y2);
        }
        if x1 > x2 {
            std::mem::swap(&mut x1, &mut x2);
            std::mem::swap(&mut y1, &mut y2);
        }

        let dx = x2 - x1;
        let dy = y2 - y1;
        let gradient = dy / dx;

        let mut intery = y1 + gradient;

        for x in (x1 as i32)..=(x2 as i32) {
            let y_floor = intery as i32;
            let intensity1 = 1.0 - (intery - y_floor as f32);
            let intensity2 = intery - y_floor as f32;

            if steep {
                self.set_pixel(y_floor, x, color, intensity1);
                self.set_pixel(y_floor + 1, x, color, intensity2);
            } else {
                self.set_pixel(x, y_floor, color, intensity1);
                self.set_pixel(x, y_floor + 1, color, intensity2);
            }

            intery += gradient;
        }
    }

    fn set_pixel(&mut self, x: i32, y: i32, color: Color32, intensity: f32) {
        if x >= 0 && y >= 0 {
            let r = (color.r() as f32 * intensity) as u8;
            let g = (color.g() as f32 * intensity) as u8;
            let b = (color.b() as f32 * intensity) as u8;
            self[(x as usize, y as usize)] = Color32::from_rgb(r, g, b);
        }
    }
}

// Задание 3 (растеризация треугольника с градиентом)
impl Canvas {
    // Сюда можно приватные вспомогательные методы, если нужно

    /// Рисование линии алгоритмом Ву.
    /// pos[1..3] - 3 точки треугольника;
    /// color[1..3] - цвета соответствующих точек;
    pub fn draw_gradient_triangle(
        &mut self,
        pos1: Pos2,
        pos2: Pos2,
        pos3: Pos2,
        color1: Color32,
        color2: Color32,
        color3: Color32,
    ) {
        // TODO
        // для операций над холстом использовать эти методы:
        // self[(x, y)]; - выдаёт egui::Color32
        // self[(x, y)] = color; - устанавлиает цвет пикселя
    }
}
