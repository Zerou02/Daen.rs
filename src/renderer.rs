use std::cmp::Ordering;

use pixels::Pixels;

use crate::point::Point;

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

    pub fn getIndexByPos(self: &Self, x: i32, y: i32) -> i32 {
        return ((y * self.width + x) * 4);
    }

    pub fn fillPixel(self: &mut Self, x: i32, y: i32, colour: u32) {
        if (x >= self.width || y >= self.height) {
            return;
        }
        let index = self.getIndexByPos(x, y) as usize;
        let buf = self.pixelsObj.frame_mut();
        if (index >= buf.len()) {
            return;
        }

        let aR = (((colour & 0xff000000) >> 24) as f64 / 255.0);
        let aG = (((colour & 0x00ff0000) >> 16) as f64 / 255.0);
        let aB = (((colour & 0x0000ff00) >> 8) as f64 / 255.0);
        let aA = ((colour & 0x000000ff) as f64 / 255.0);
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

    pub fn fillSquare(self: &mut Self, leftUpMostPoint: &Point, dim: &Point, colour: u32) {
        let startIndex = self.getIndexByPos(leftUpMostPoint.x, leftUpMostPoint.y);
        for yi in (leftUpMostPoint.y..(leftUpMostPoint.y + dim.y)) {
            for xi in (leftUpMostPoint.x..(leftUpMostPoint.x + dim.x)) {
                self.fillPixel(xi, yi, colour)
            }
        }
    }

    pub fn getDistanceBetweenPoints(self: &Self, x1: i32, y1: i32, x2: i32, y2: i32) -> f64 {
        let cSquared = (x2 as f64 - x1 as f64) * (x2 as f64 - x1 as f64)
            + (y2 as f64 - y1 as f64) * (y2 as f64 - y1 as f64);
        return (cSquared as f64).sqrt();
    }

    pub fn fillCircle(self: &mut Self, x: i32, y: i32, radius: i32, colour: u32) {
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

    pub fn drawLine(self: &mut Self, x1: i32, y1: i32, x2: i32, y2: i32, colour: u32) {
        let lowerX = x1.min(x2);
        let higherX = x1.max(x2);

        let dx = higherX - lowerX;
        let dy = if lowerX == x1 { y2 - y1 } else { y1 - y2 };
        let isYNeg = dy < 0;
        let stepSize = dy as f64 / dx as f64;
        let mut yAcc = 0.0;
        let mut currX = lowerX;
        let mut currY = if lowerX == x1 { y1 } else { y2 };
        if (dx == 0) {
            let lowerY = y1.min(y2);
            let higherY = y2.min(y2);
            for y in lowerY..=higherY {
                self.fillPixel(currX, y, colour);
            }
            return;
        }
        for i in lowerX..=higherX {
            self.fillPixel(currX, currY, colour);
            currX += 1;
            yAcc += stepSize;
            if (yAcc <= 1.0 && isYNeg) {
                while yAcc <= 1.0 {
                    yAcc += 1.0;
                    currY -= 1;
                    self.fillPixel(currX, currY, colour)
                }
            } else if (yAcc >= 1.0 && !isYNeg) {
                while yAcc >= 1.0 {
                    yAcc -= 1.0;
                    currY += 1;
                    self.fillPixel(currX, currY, colour);
                }
            }
        }
    }

    pub fn fillEllipsis(
        self: &mut Self,
        point1: &Point,
        point2: &Point,
        distance: i32,
        colour: u32,
    ) {
        let minX = point1.x.min(point2.x);
        let minY = point1.y.min(point2.y);
        let maxX = point1.x.max(point2.x);
        let maxY = point1.y.max(point2.y);

        let epsilon = 0.8;
        for yi in ((minY - distance).max(0)..=(maxY + distance).min(self.height)) {
            for xi in ((minX - distance).max(0)..=(maxX + distance).min(self.width)) {
                let point = Point { x: xi, y: yi };
                if ((point1.distanceTo(&point) + point2.distanceTo(&point)) < distance as f64) {
                    self.fillPixel(xi, yi, colour)
                }
            }
        }
    }

    pub fn drawCircle(self: &mut Self, point: &Point, distance: i32, colour: u32) {
        self.drawEllipsis(point, point, distance, colour);
    }

    pub fn drawEllipsis(
        self: &mut Self,
        point1: &Point,
        point2: &Point,
        distance: i32,
        colour: u32,
    ) {
        let minX = point1.x.min(point2.x);
        let minY = point1.y.min(point2.y);
        let maxX = point1.x.max(point2.x);
        let maxY = point1.y.max(point2.y);

        let mut leftmostPoint = Point { x: 0, y: 0 };

        //findLeftmostPoint
        for x in (0..=minX) {
            let point = Point { x, y: minY };
            if ((point1.distanceTo(&point) + point2.distanceTo(&point)) == distance as f64) {
                leftmostPoint = Point { x, y: minY };
            }
        }

        let originalPoint = Point {
            x: leftmostPoint.x,
            y: leftmostPoint.y,
        };
        let mut lastEntry = Point {
            x: leftmostPoint.x,
            y: leftmostPoint.y,
        };
        //0-7: Directions, beginnend bei UP
        let directionMap = [
            [[0, -1], [1, -1], [0, 0]],
            [[0, -1], [1, -1], [1, 0]],
            [[1, 0], [1, 1], [0, 0]],
            [[1, 0], [1, 1], [0, 1]],
            [[0, 1], [-1, 0], [0, 0]],
            [[0, 1], [-1, 0], [-1, -1]],
            [[-1, 0], [-1, -1], [0, 0]],
            [[-1, 0], [-1, -1], [0, -1]],
        ];
        let betterMap = [
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
            for x in betterMap {
                let p = Point {
                    x: leftmostPoint.x + x[0],
                    y: leftmostPoint.y + x[1],
                };
                distVec
                    .push(((p.distanceTo(point1) + p.distanceTo(point2)) - distance as f64).abs());
            }
            //eval
            let mut index = 9999;
            let mut min = 99999.0;
            for (i, x) in distVec.iter().enumerate() {
                if (*x < min
                    && !(betterMap[i][0] + leftmostPoint.x == lastEntry.x
                        && betterMap[i][1] + leftmostPoint.y == lastEntry.y))
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
                x: leftmostPoint.x + betterMap[index][0],
                y: leftmostPoint.y + betterMap[index][1],
            };
            self.fillPixel(leftmostPoint.x, leftmostPoint.y, colour);
            if (leftmostPoint.x == originalPoint.x && leftmostPoint.y == originalPoint.y) {
                goneFullEllipsis = true;
            }
        }
    }
}
