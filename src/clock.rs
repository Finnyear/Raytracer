use crate::point::Point;
pub struct Clock {
    pub clock: [[i32; 512]; 1024],
    pub center: Point,
    pub radius: i32,
}
pub fn abs(x : i32) -> i32{
    if x < 0 { -x } else {x}
}
pub fn absf(x : f64) -> f64{
    if x < 0.0 { -x } else {x}
}
pub fn sgn(x : f64) -> i32{
    if x == 0.0 {0}
    else{
        if x > 0.0{1}
        else{-1}
    }
}
pub fn max<T: std::cmp::PartialOrd>(x : T, y : T) -> T{
    if(x >= y){x} else{y}
}
pub fn min<T: std::cmp::PartialOrd>(x : T, y : T) -> T{
    if(x <= y){x} else{y}
}
pub fn check_dis(p1 : &Point, p2 : &Point, r : &i32) -> bool {
    let mn: i32;
    let mx: i32;
    if p1.x == p2.x{
        if p1.y == p2.y{
            mn = 0;
            mx = 2;
        }
        else{
            mn = (abs(p1.y - p2.y) * 2 - 1) * (abs(p1.y - p2.y) * 2 - 1);
            mx = (abs(p1.y - p2.y) * 2 + 1) * (abs(p1.y - p2.y) * 2 + 1) + 1;
        }
    }
    else{
        if p1.y == p2.y{
            mn = (abs(p1.x - p2.x) * 2 - 1) * (abs(p1.x - p2.x) * 2 - 1);
            mx = (abs(p1.x - p2.x) * 2 + 1) * (abs(p1.x - p2.x) * 2 + 1) + 1;
        }
        else{
            mn = (abs(p1.x - p2.x) * 2 - 1) * (abs(p1.x - p2.x) * 2 - 1) + 
                    (abs(p1.y - p2.y) * 2 - 1) * (abs(p1.y - p2.y) * 2 - 1);
            mx = (abs(p1.x - p2.x) * 2 + 1) * (abs(p1.x - p2.x) * 2 + 1) + 
                    (abs(p1.y - p2.y) * 2 + 1) * (abs(p1.y - p2.y) * 2 + 1);
        }
    }
    mn <= r * r && mx >= r * r
}
impl Clock {
    pub fn new() -> Self {
        Self {
            clock: [[0; 512]; 1024],
            center: Point::new(512, 256),
            radius: 400,
        }
    }
    pub fn draw_clock_plate(&mut self) {
        for x in (0..1024) {
            for y in (0..512) {
                if check_dis(&Point :: new(x, y), &self.center, &self.radius) == true{
                    self.clock[x as usize][y as usize] = 1;
                } 
            }
        }
    }
    pub fn draw_line(&mut self, alpha: f64, len: f64){
        let mut alpha = 90.0f64 - alpha;
        if alpha < 0.0 {alpha = alpha + 360.0;}
        alpha = alpha / 360.0 * 2.0 * std::f64::consts::PI;
        let si = alpha.sin();
        let co = alpha.cos();
        println!("{}, {}, {}", alpha, si, co);
        for x in (-512 as i32..511) {
            if abs(sgn(co) - sgn(x as f64)) <= 1{
            //if(sgn(x as f64) == sgn(co))
            for y in (-256 as i32..255) {
                if abs(sgn(si) - sgn(y as f64)) <= 1{
                let mut np = 0;
                let mut nn = 0;
                let x1 = (x as f64) * 2.0 - 1.0;
                let x2 = (x as f64) * 2.0 + 1.0;
                let y1 = (y as f64) * 2.0 - 1.0;
                let y2 = (y as f64) * 2.0 + 1.0;
                if min(absf(x1), absf(x2)).powf(2.0) + min(absf(y1), absf(y2)).powf(2.0) > len.powf(2.0) {
                        // println!("{} + {} > {}", min(absf(x1), absf(x2)).powf(2.0), min(absf(y1), absf(y2)).powf(2.0), len.powf(2.0));
                        continue;
                }
                if x1 * si - y1 * co >= 0.0 {np = np + 1;}
                if x1 * si - y1 * co <= 0.0 {nn = nn + 1;}
                if x1 * si - y2 * co >= 0.0 {np = np + 1;}
                if x1 * si - y2 * co <= 0.0 {nn = nn + 1;}
                if x2 * si - y1 * co >= 0.0 {np = np + 1;}
                if x2 * si - y1 * co <= 0.0 {nn = nn + 1;}
                if x2 * si - y2 * co >= 0.0 {np = np + 1;}
                if x2 * si - y2 * co <= 0.0 {nn = nn + 1;}
                if np > 0 && nn > 0 {self.clock[(x + 512)as usize][(-y + 255)as usize] = 1;}
                }
            }
            }
        }
    }
    pub fn draw_clock_needle(&mut self, h: i32, m: i32, s: i32) {
        let alpha: f64 = (s as f64) * 6.0f64;
        self.draw_line(alpha, 360.0);
        let alpha = alpha / 60.0f64 + (m as f64) * 6.0f64;
        self.draw_line(alpha, 300.0);
        let alpha = alpha / 60.0f64 + (h as f64) * 30.0f64;
        self.draw_line(alpha, 200.0);
    }
}
