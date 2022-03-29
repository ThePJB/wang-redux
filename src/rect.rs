use crate::kmath::*;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect{x,y,w,h}
    }
    pub fn child(&self, x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect::new(
            self.x + x*self.w,
            self.y + y*self.h,
            self.w * w,
            self.h * h,
        )
    }
    pub fn grid_child(&self, x: i32, y: i32, w: i32, h: i32) -> Rect {
        let r_w = self.w / w as f32;
        let r_h = self.h / h as f32;

        Rect::new(
            self.x + r_w * x as f32,
            self.y + r_h * y as f32,
            r_w,
            r_h,
        )
    }
    pub fn fit_center_square(&self) -> Rect {
        let s = self.w.min(self.h);
        Rect::new_centered(self.w / 2.0, self.h / 2.0, s, s)
    }
    pub fn centroid(&self) -> Vec2 {
        Vec2::new(self.x + self.w/2.0, self.y + self.h/2.0)
    }
    pub fn new_centered(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect::new(x-w/2.0, y-h/2.0, w, h)
    }
    pub fn translate(&self, v: Vec2) -> Rect {
        return Rect::new(self.x + v.x, self.y + v.y, self.w, self.h);
    }
    pub fn dilate(&self, d: f32) -> Rect {
        return Rect::new(self.x - d, self.y - d, self.w + 2.0*d, self.h + 2.0*d);
    }
    pub fn left(self) -> f32 {
        self.x
    }
    pub fn right(self) -> f32 {
        self.x + self.w
    }
    pub fn top(self) -> f32 {
        self.y
    }
    pub fn bot(self) -> f32 {
        self.y + self.h
    }
}