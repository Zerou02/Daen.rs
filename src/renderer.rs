use std::cmp::Ordering;

use pixels::Pixels;

use crate::{point::Point, utils::rotatePoint};

pub type rgbColour = [u8; 4];
pub struct Renderer {
    height: i32,
    width: i32,
    pub pixelsObj: Pixels,
}

impl Renderer {
    pub fn new(pixelsObj: Pixels, height: i32, width: i32) -> Renderer {
        return Renderer {
            height,
            width,
            pixelsObj,
        };
    }

    pub fn clearBuf(&mut self, colour: u32) {
        let aR = ((colour & 0xff000000) >> 24) as u8;
        let aG = ((colour & 0x00ff0000) >> 16) as u8;
        let aB = ((colour & 0x0000ff00) >> 8) as u8;
        let aA = (colour & 0x000000ff) as u8;
        let arr = [aR, aG, aB, aA];
        for (i, pixel) in self.pixelsObj.frame_mut().chunks_exact_mut(4).enumerate() {
            pixel.copy_from_slice(&arr)
        }
    }

    pub fn getIndexByPosF(self: &Self, x: f64, y: f64) -> i32 {
        return self.getIndexByPos(x as i32, y as i32);
    }

    pub fn getIndexByPos(self: &Self, x: i32, y: i32) -> i32 {
        return ((y * self.width + x) * 4);
    }

    pub fn fillPixelRotated(&mut self, p: &Point, pivot: &Point, rad: f64, colour: rgbColour) {
        let p2 = rotatePoint(p, rad, pivot);
        self.fillPixelF(p2.x, p2.y, colour);
    }
    pub fn fillPixelF(self: &mut Self, x: f64, y: f64, colour: rgbColour) {
        self.fillPixel(x.round() as i32, y.round() as i32, colour)
    }

    pub fn fillPixel(self: &mut Self, x: i32, y: i32, colour: rgbColour) {
        if (x >= self.width || y >= self.height || x < 0 || y < 0) {
            return;
        }
        let index = self.getIndexByPos(x, y) as usize;
        let buf = self.pixelsObj.frame_mut();

        let aR = (colour[0] as f64 / 255.0);
        let aG = (colour[1] as f64 / 255.0);
        let aB = (colour[2] as f64 / 255.0);
        let aA = (colour[3] as f64 / 255.0);
        let bR = (buf[index] as f64 / 255.0);
        let bG = (buf[index + 1] as f64 / 255.0);
        let bB = (buf[index + 2] as f64 / 255.0);
        let bA = (buf[index + 3] as f64 / 255.0);

        let cA = aA + (1.0 - aA) * bA;

        let cR = ((1.0 / cA) * (aA * aR + (1.0 - aA) * bA * bR));
        let cG = ((1.0 / cA) * (aA * aG + (1.0 - aA) * bA * bG));
        let cB = ((1.0 / cA) * (aA * aB + (1.0 - aA) * bA * bB));
        buf[index] = (cR * 255.0) as u8;
        buf[index + 1] = (cG * 255.0) as u8;
        buf[index + 2] = (cB * 255.0) as u8;
        buf[index + 3] = (cA * 255.0) as u8;
    }

    pub fn fillSquare(self: &mut Self, leftUpMostPoint: &Point, dim: &Point, colour: rgbColour) {
        let startIndex = self.getIndexByPosF(leftUpMostPoint.x, leftUpMostPoint.y);
        for yi in (leftUpMostPoint.y as i32..(leftUpMostPoint.y + dim.y) as i32) {
            for xi in (leftUpMostPoint.x as i32..(leftUpMostPoint.x + dim.x) as i32) {
                self.fillPixel(xi, yi, colour)
            }
        }
    }

    pub fn fillSquareFill(
        &mut self,
        leftUpMostPoint: &Point,
        dim: &Point,
        gradient: Vec<rgbColour>,
    ) {
    }
    pub fn getDistanceBetweenPoints(self: &Self, x1: i32, y1: i32, x2: i32, y2: i32) -> f64 {
        let cSquared = (x2 as f64 - x1 as f64) * (x2 as f64 - x1 as f64)
            + (y2 as f64 - y1 as f64) * (y2 as f64 - y1 as f64);
        return (cSquared as f64).sqrt();
    }

    pub fn fillCircle(self: &mut Self, centre: &Point, radius: i32, colour: rgbColour) {
        let x = centre.x as i32;
        let y = centre.y as i32;
        let startIndex = self.getIndexByPos(x - radius, y - radius);
        // prevent overflow
        for yi in ((y - radius).max(0)..(y + radius)) {
            for xi in ((x - radius).max(0)..(x + radius)) {
                if (self.getDistanceBetweenPoints(x, y, xi, yi) < radius as f64) {
                    self.fillPixel(xi, yi, colour)
                }
            }
        }
    }

    pub fn drawLine(self: &mut Self, p1: &Point, p2: &Point, colour: rgbColour) {
        let mut x0 = p1.x as i32;
        let mut y0 = p1.y as i32;
        let x1 = p2.x as i32;
        let y1 = p2.y as i32;
        //bresenham-line:
        let dx = (x1 - x0).abs() as i32;
        let sx = if (x0 < x1) { 1 } else { -1 };
        let dy = -(y1 - y0).abs() as i32;
        let sy = if (y0 < y1) { 1 } else { -1 };
        let mut err = dx + dy;
        loop {
            self.fillPixel(x0, y0, colour);
            if (x0 == x1 && y0 == y1) {
                break;
            };
            let e2 = 2 * err;
            if (e2 > dy) {
                err += dy;
                x0 += sx;
            }
            if (e2 < dx) {
                err += dx;
                y0 += sy;
            }
        }
    }

    pub fn fillEllipsis(
        self: &mut Self,
        point1: &Point,
        point2: &Point,
        distance: i32,
        colour: rgbColour,
    ) {
        let minX = point1.x.min(point2.x) as i32;
        let minY = point1.y.min(point2.y) as i32;
        let maxX = point1.x.max(point2.x) as i32;
        let maxY = point1.y.max(point2.y) as i32;

        let epsilon = 0.8;
        for yi in ((minY - distance).max(0)..=(maxY + distance).min(self.height)) {
            for xi in ((minX - distance).max(0)..=(maxX + distance).min(self.width)) {
                let point = Point {
                    x: xi as f64,
                    y: yi as f64,
                };
                if ((point1.distanceTo(&point) + point2.distanceTo(&point)) < distance as f64) {
                    self.fillPixel(xi, yi, colour)
                }
            }
        }
    }

    pub fn drawCircle(self: &mut Self, point: &Point, distance: i32, colour: rgbColour) {
        self.drawEllipsis(point, point, distance, colour);
    }

    pub fn drawEllipsis(
        self: &mut Self,
        point1: &Point,
        point2: &Point,
        distance: i32,
        colour: rgbColour,
    ) {
        let minX = point1.x.min(point2.x) as i32;
        let minY = if (minX == point1.x as i32) {
            point1.y
        } else {
            point2.y
        };
        let maxX = point1.x.max(point2.x) as i32;
        let maxY = if (maxX == point1.x as i32) {
            point1.y
        } else {
            point2.y
        };

        let mut leftmostPoint: Point;

        let mut resultVec: Vec<f64> = vec![];
        //findLeftmostPoint
        for x in (0..=minX) {
            let point = Point {
                x: x as f64,
                y: minY as f64,
            };
            resultVec.push(
                (((point.distanceTo(point1) + point.distanceTo(point2)) - distance as f64).abs()),
            );
        }
        let mut index = 9999;
        let mut min = 99999.0;
        for (i, x) in resultVec.iter().enumerate() {
            if (*x < min) {
                min = *x;
                index = i;
            }
        }
        leftmostPoint = Point {
            x: index as f64,
            y: minY,
        };

        let originalPoint = Point {
            x: leftmostPoint.x,
            y: leftmostPoint.y,
        };
        let mut lastEntry = Point {
            x: leftmostPoint.x,
            y: leftmostPoint.y,
        };
        let directionMap = [
            [-1, 0],
            [-1, -1],
            [0, -1],
            [1, -1],
            [1, 0],
            [1, 1],
            [0, 1],
            [-1, 1],
        ];
        let mut goneFullEllipsis = false;
        while !goneFullEllipsis {
            let mut distVec: Vec<f64> = Vec::with_capacity(3);
            //checkAllIndices
            for x in directionMap {
                let p = Point {
                    x: leftmostPoint.x + x[0] as f64,
                    y: leftmostPoint.y + x[1] as f64,
                };
                distVec
                    .push(((p.distanceTo(point1) + p.distanceTo(point2)) - distance as f64).abs());
            }
            //eval
            let mut index = 9999;
            let mut min = 99999.0;
            for (i, x) in distVec.iter().enumerate() {
                if (*x < min
                    && !(directionMap[i][0] as f64 + leftmostPoint.x == lastEntry.x
                        && directionMap[i][1] as f64 + leftmostPoint.y == lastEntry.y))
                {
                    min = *x;
                    index = i;
                }
            }
            lastEntry = Point {
                x: leftmostPoint.x,
                y: leftmostPoint.y,
            };

            leftmostPoint = Point {
                x: leftmostPoint.x + directionMap[index][0] as f64,
                y: leftmostPoint.y + directionMap[index][1] as f64,
            };
            self.fillPixelF(leftmostPoint.x, leftmostPoint.y, colour);
            if (leftmostPoint.x as i32 == originalPoint.x as i32
                && leftmostPoint.y as i32 == originalPoint.y as i32)
            {
                goneFullEllipsis = true;
            }
        }
    }
}
