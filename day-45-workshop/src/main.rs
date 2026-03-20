pub struct RectIter {
    points: Vec<(f64, f64)>,
    idx: usize,
}

impl Iterator for RectIter {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.points.len() {
            return None;
        }
        let point = self.points[self.idx];
        self.idx += 1;
        return Some(point);
    }
}

impl IntoIterator for RectIter {
    type Item = (f64, f64);
    type Iterator = RectIter;

    fn into_iter(self) -> Self::IntoIter {
        return RectIter {
            points: vec![
                (self.x, self.y),
                (self.x, self.y + self.height),
                (self.x + self.width, self.y),
                (self.x + self.width, self.y + self.height),
            ],
            idx: 0,
        };
    }
}

fn main() {
    println!("Hello, world!");
}
