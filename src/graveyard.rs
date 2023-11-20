/* pub fn ownLine(self: &Self, p1: &Point, p2: &Point, colour: rgbColour) {
    let x1 = p1.x as i32;
    let y1 = p1.y as i32;
    let x2 = p2.x as i32;
    let y2 = p2.y as i32;
    let lowerX = x1.min(x2);
    let higherX = x1.max(x2);
    let lowerY = if (lowerX == x1) { p1.y } else { p2.y } as i32;
    let higherY = if (higherX == x1) { p1.y } else { p2.y } as i32;

    let dx = higherX - lowerX;
    let dy = higherY - lowerY;
    let isYNeg = dy < 0;
    let stepSize = dy as f64 / dx as f64;
    let mut yAcc = 0.0;
    let mut currX = lowerX;
    let mut currY = lowerY;
    if (dx == 0) {
        let lowerY = y1.min(y2);
        let higherY = y2.min(y2);
        for y in lowerY..higherY {
            self.fillPixel(currX, y, colour);
        }
        return;
    } else {
        for i in lowerX..higherX {
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
}
 */

//unvollständig
/* pub fn drawEllipsis3(&mut self, p: Point, width: f64, height: f64, colour: rgbColour) {
    let mut x = 0.0;
    let mut y = height;
    let mut d1: f64 = (height * height) - (width * width * height) + (0.25 * width * width);
    let mut dx = 2.0 * height * height * x;
    let mut dy = 2.0 * width * width * y;
    while dx < dy {
        self.fillPixelF(x + p.x, y + p.y, colour);
        self.fillPixelF(-x + p.x, y + p.y, colour);
        self.fillPixelF(x + p.x, -y + p.y, colour);
        self.fillPixelF(-x + p.x, -y + p.y, colour);
        if (d1 < 0.0) {
            x += 1.0;
            dx = dx + (2.0 * height * height);
            d1 = d1 + dx + (height * height);
        } else {
            x += 1.0;
            y -= 1.0;
            dx = dx + (2.0 * height * height);
            dy = dy - (2.0 * width * width);
            d1 = d1 + dx - dy + (height * height);
        }
    }
    let mut d2 = (height * height);
}

pub fn drawEllipsis2(&mut self, p: &Point, a: i32, b: i32, colour: rgbColour, rot: f64) {
    let xm = p.x as i32;
    let ym = p.y as i32;
    let mut dx = 0;
    let mut dy = b;
    let a2 = (a * a) as i64;
    let b2 = (b * b) as i64;
    let mut err = b2 - (2 * b as i64 - 1) * a2;
    let mut e2: i64 = 0;

    self.fillPixelRotated(&Point::newI(xm + dx, ym + dy), &p, rot, colour);
    self.fillPixelRotated(&Point::newI(xm - dx, ym + dy), &p, rot, colour);
    self.fillPixelRotated(&Point::newI(xm - dx, ym - dy), &p, rot, colour);
    self.fillPixelRotated(&Point::newI(xm + dx, ym - dy), &p, rot, colour);
    e2 = 2 * err;
    if (e2 < (2 * dx as i64 + 1) * b2) {
        dx += 1;
        err += (2 * dx as i64 + 1) * b2;
    }
    if (e2 > -(2 * dy as i64 - 1) * a2) {
        dy -= 1;
        err -= (2 * dy as i64 - 1) * a2;
    }
    while dy >= 0 {
        self.fillPixelRotated(&Point::newI(xm + dx, ym + dy), &p, rot, colour);
        self.fillPixelRotated(&Point::newI(xm - dx, ym + dy), &p, rot, colour);
        self.fillPixelRotated(&Point::newI(xm - dx, ym - dy), &p, rot, colour);
        self.fillPixelRotated(&Point::newI(xm + dx, ym - dy), &p, rot, colour);
        e2 = 2 * err;
        if (e2 < (2 * dx as i64 + 1) * b2) {
            dx += 1;
            err += (2 * dx as i64 + 1) * b2;
        }
        if (e2 > -(2 * dy as i64 - 1) * a2) {
            dy -= 1;
            err -= (2 * dy as i64 - 1) * a2;
        }
    }
    while dx + 1 < a {
        dx += 1;
        self.fillPixelRotated(&Point::newI(xm + dx, ym), &p, rot, colour);
        self.fillPixelRotated(&Point::newI(xm - dx, ym), &p, rot, colour);
    }
} */
