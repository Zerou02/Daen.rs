use crate::constants::HEIGHT;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::point::Point;

pub fn getTime() -> u128 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("asdad")
        .as_millis();
}

pub fn rotatePoint(point: &Point, rad: f64, pivot: &Point) -> Point {
    let c = rad.cos();
    let s = rad.sin();
    let x = point.x - pivot.x;
    let y = point.y - pivot.y;
    let tmpPoint = Point {
        x: (x * c + y * s),
        y: (-x * s + y * c),
    };
    return Point {
        x: tmpPoint.x + pivot.x,
        y: tmpPoint.y + pivot.y,
    };
}

pub fn screenToCartesianY(y: f64) -> f64 {
    let baseY = HEIGHT as f64;
    return (baseY - y);
}
