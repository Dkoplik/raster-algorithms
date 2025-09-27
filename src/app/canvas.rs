use std::ops::{Index, IndexMut};

use egui::{Color32, ColorImage, Pos2};

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
    fn check_bounds(&self, x: usize, y: usize) {
        if x >= self.width {
            panic!("x = {} is greater than image width of {}", x, self.width);
        }
        if y >= self.height {
            panic!("y = {} is greater than image height of {}", y, self.height);
        }
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

    /// Рекурсивная заливка изображения.
    /// pos - позиция, в которой применяется заливка;
    /// color - цвет заливки;
    /// connectivity - тип заливки (4-х или 8-ми связная);
    pub fn fill_with_color(&mut self, pos: Pos2, color: Color32, connectivity: Connectivity) {
        // TODO
        // для операций над холстом использовать эти методы:
        // self[(x, y)]; - выдаёт egui::Color32
        // self[(x, y)] = color; - устанавлиает цвет пикселя
    }

    /// Рекурсивная заливка изображения.
    /// pos - позиция, в которой применяется заливка;
    /// img - изображение для заливки;
    /// connectivity - тип заливки (4-х или 8-ми связная);
    pub fn fill_with_img(&mut self, pos: Pos2, img: &ColorImage, connectivity: Connectivity) {
        // TODO
        // для операций над холстом использовать эти методы:
        // self[(x, y)]; - выдаёт egui::Color32
        // self[(x, y)] = color; - устанавлиает цвет пикселя
        //
        // для получения цветов из картинки, использовать
        // img[(x, y)]
    }
}

// Задание 2 (линии)
impl Canvas {
    // Сюда можно приватные вспомогательные методы, если нужно

    /// Рисование линии алгоритмом Брезенхема.
    /// pos1 - первая точка линии;
    /// pos2 - вторая точка линии;
    /// color - цвет линии;
    pub fn draw_sharp_line(&mut self, pos1: Pos2, pos2: Pos2, color: Color32) {
        // TODO
        // для операций над холстом использовать эти методы:
        // self[(x, y)]; - выдаёт egui::Color32
        // self[(x, y)] = color; - устанавлиает цвет пикселя
    }

    /// Рисование линии алгоритмом Ву.
    /// pos1 - первая точка линии;
    /// pos2 - вторая точка линии;
    /// color - цвет линии;
    pub fn draw_smooth_line(&mut self, pos1: Pos2, pos2: Pos2, color: Color32) {
        // TODO
        // для операций над холстом использовать эти методы:
        // self[(x, y)]; - выдаёт egui::Color32
        // self[(x, y)] = color; - устанавлиает цвет пикселя
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
