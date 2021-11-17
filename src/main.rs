/*
 * kitkat
 *
 * Copyright 2021 - Manos Pitsidianakis
 *
 * This file is part of kitkat.
 *
 * kitkat is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * kitkat is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with kitkat. If not, see <http://www.gnu.org/licenses/>.
 */

use minifb::{Key, Window, WindowOptions};
use std::f64;
use std::f64::consts::{FRAC_PI_2, PI};
use std::time::{Duration, Instant, SystemTime};

mod image;
pub use image::*;
mod draw;
pub use draw::*;
mod hands;

pub const fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
pub(crate) const AZURE_BLUE: u32 = from_u8_rgb(0, 127, 255);
pub(crate) const RED: u32 = from_u8_rgb(157, 37, 10);
pub(crate) const WHITE: u32 = from_u8_rgb(255, 255, 255);
pub(crate) const BLACK: u32 = 0;

#[derive(Clone, Copy)]
struct Bitmap<'bits> {
    bits: &'bits [u8],
    width: usize,
    height: usize,
    x_offset: usize,
    y_offset: usize,
}

#[inline(always)]
pub fn pixel_width_to_bits_width(i: usize) -> usize {
    i.wrapping_div(8) + if i.wrapping_rem(8) > 0 { 1 } else { 0 }
}

pub fn bits_to_bytes(bits: &[u8], width: usize) -> Vec<u32> {
    let mut ret = Vec::with_capacity(bits.len() * 8);
    let mut current_row_count = 0;
    for byte in bits {
        for n in 0..8 {
            if byte.rotate_right(n) & 0x01 > 0 {
                ret.push(BLACK);
            } else {
                ret.push(WHITE);
            }
            current_row_count += 1;
            if current_row_count == width {
                current_row_count = 0;
                break;
            }
        }
    }
    ret
}

impl<'bits> Bitmap<'bits> {
    /*
    fn draw(&self, buffer: &mut Vec<u32>, fg: u32, bg: Option<u32>) {
        let row_width = pixel_width_to_bits_width(self.width);
        //std::dbg!(row_width);
        debug_assert_eq!(row_width * self.height, self.bits.len());
        let mut bits = self.bits.iter();
        let mut i = self.y_offset * (self.width);
        for _ in 0..self.height {
            let mut c = 0;
            'byte_row: for byte in bits.by_ref().take(row_width) {
                for n in 0..8 {
                    if self.x_offset + c == self.width || i + self.x_offset + c >= buffer.len() {
                        break 'byte_row;
                    }
                    if byte.rotate_right(n) & 0x01 > 0 {
                        buffer[i + self.x_offset + c] = fg;
                    } else if let Some(bg) = bg {
                        buffer[i + self.x_offset + c] = bg;
                    };
                    c += 1;
                }
            }
            i += self.width + self.x_offset;
        }
        //debug_assert_eq!(i, self.width * self.height);
    }
    */
}

include!("catback.rs");
const CATBACK: Bitmap<'static> = Bitmap {
    bits: CAT_BITS,
    width: CAT_WIDTH,
    height: CAT_HEIGHT,
    x_offset: 0,
    y_offset: 0,
};

include!("cattie.rs");
const CATTIE: Bitmap<'static> = Bitmap {
    bits: CATTIE_BITS,
    width: CATTIE_WIDTH,
    height: CATTIE_HEIGHT,
    x_offset: 0,
    y_offset: 0,
};

include!("catwhite.rs");
const CATWHITE: Bitmap<'static> = Bitmap {
    bits: CATWHITE_BITS,
    width: CATWHITE_WIDTH,
    height: CATWHITE_HEIGHT,
    x_offset: 0,
    y_offset: 0,
};

include!("eyes.rs");
const EYES: Bitmap<'static> = Bitmap {
    bits: EYES_BITS,
    width: EYES_WIDTH,
    height: EYES_HEIGHT,
    x_offset: CAT_WIDTH / 2,
    y_offset: 30,
};

include!("tail.rs");
const TAIL: Bitmap<'static> = Bitmap {
    bits: TAIL_BITS,
    width: TAIL_WIDTH,
    height: TAIL_HEIGHT,
    x_offset: 0,
    y_offset: TAIL_OFFSET_Y,
};

const NUM_TAILS: usize = 10;

const N_TAIL_PTS: usize = 7;
const CENTER_TAIL: [(i32, i32); N_TAIL_PTS] = [
    /*  "Center" tail points definition */
    (0, 0),
    (0, 76),
    (3, 82),
    (10, 84),
    (18, 82),
    (21, 76),
    (21, 70),
];

fn create_eye_pixmap(t: f64) -> Image {
    macro_rules! tr {
        ($cond:expr ,? $then:expr ,: $else:expr) => {
            if $cond {
                $then
            } else {
                $else
            }
        };
    }
    let mut ret = Image {
        bytes: vec![WHITE; 30 * 60],
        width: 60,
        height: 30,
        x_offset: 47,
        y_offset: 30,
    };

    //ret.draw_outline();

    const A: f64 = 0.7;
    let omega: f64 = 1.0;
    let phi: f64 = 3.0 * FRAC_PI_2;
    let mut u: f64;
    let w: f64 = FRAC_PI_2;
    /*  Sphere parameters    */
    /*  Radius               */
    let r: f64 = 1.0;
    /*  Center of sphere     */
    let x0: f64 = 0.0;
    let y0: f64 = 0.0;
    let z0: f64 = 2.0;

    let angle: f64 = A * f64::sin(omega * t + phi) + w;
    let mut points: Vec<(i32, i32)> = Vec::with_capacity(100);

    let mut i = 0;
    u = -1.0 * FRAC_PI_2;
    while u < FRAC_PI_2 {
        let x = x0 + r * f64::cos(u) * f64::cos(angle + PI / 7.0);
        let z = z0 + r * f64::cos(u) * f64::sin(angle + PI / 7.0);
        let y = y0 + r * f64::sin(u);

        let a = ((tr!(z == 0.0 ,? x ,: x / z) * 23.0) + 12.0) as i32;
        let b = ((tr!(z == 0.0 ,? y ,: y / z) * 23.0) + 11.0) as i32;
        points.push((a, b));
        u += 0.25;
        i += 1;
    }

    u = FRAC_PI_2;
    while u > -1.0 * FRAC_PI_2 {
        let x = x0 + r * f64::cos(u) * f64::cos(angle - PI / 7.0);
        let z = z0 + r * f64::cos(u) * f64::sin(angle - PI / 7.0);
        let y = y0 + r * f64::sin(u);

        let a = ((tr!(z == 0.0 ,? x ,: x / z) * 23.0) + 12.0) as i32;
        let b = ((tr!(z == 0.0 ,? y ,: y / z) * 23.0) + 11.0) as i32;
        points.push((a, b));
        u -= 0.25;
        i += 1;
    }

    let (_xf, _yf) = points[0];
    for window in points.as_slice().windows(2) {
        let point_a = window[0];
        let point_b = window[1];
        ret.plot_line_width(point_a, point_b, 1.);
    }
    //ret.flood_fill(xf, yf+1);
    for j in 0..i {
        points[j].0 += 31;
    }
    for window in points.as_slice().windows(2) {
        let point_a = window[0];
        let point_b = window[1];
        ret.plot_line_width(point_a, point_b, 1.);
    }

    ret
}
fn create_eye_pixmap2(t: f64) -> Vec<u8> {
    let mut ret = EYES.bits.to_vec();
    let mut buf = Buffer {
        vec: &mut ret,
        row_width: EYES.width,
        height: EYES.height,
    };
    let mut points: Vec<Vec<bool>> = vec![vec![false; EYES.width + 1]; EYES.height + 1];
    let top_point: (i32, i32) = ((EYES.width / 2) as i32, 0);
    let bottom_point: (i32, i32) = ((EYES.width / 2) as i32, EYES.height as i32);

    let center_point = (((3 * EYES.width) / 2) as i32, (EYES.height / 2) as i32);
    //println!("center_point: {:?}", center_point);

    let plot_point = move |points: &mut Vec<Vec<bool>>, point| {
        //eprintln!("plot_point: {:?}", point);
        let (x, y) = point;
        if y as usize >= points.len() || x as usize >= points[y as usize].len() {
            return;
        }
        points[y as usize][x as usize] = true;
    };

    //plot(&mut buf, top_point);
    plot_point(&mut points, top_point);
    plot_point(&mut points, bottom_point);

    let W64 = EYES.width as f64 - 3. * t;
    let H64 = EYES.height as f64;
    let H64_2 = H64 / 2.0;

    let mut cos_theta_i; //= f64::cos(FRAC_PI_2 + std::dbg!(f64::acos((W64/r))));
    let mut sin_theta_i; //= f64::sin(FRAC_PI_2 + f64::acos((W64/r)));

    let r: f64 = f64::sqrt(W64.powi(2) + (H64_2).powi(2));
    //std::dbg!(H64_2);
    //std::dbg!(r);
    //std::dbg!(H64_2 / r);
    //let mut theta_i = -1.*(FRAC_PI_2 + f64::acos(H64_2/r));
    //   println!("theta_i: {:?}", theta_i);

    for y_k in top_point.1..=bottom_point.1 {
        sin_theta_i = (y_k - center_point.1) as f64 / r;
        cos_theta_i = f64::sqrt(1. - sin_theta_i.powi(2));

        if t >= 1.0 {
            /* right */
            let x_k = center_point.0 + center_point.0 / 2
                - ((center_point.0 + (r * cos_theta_i) as i32)
                    - center_point.0
                    - EYES.width as i32);
            plot_point(&mut points, (x_k, y_k));
            plot(&mut buf, (x_k, y_k));

            /* left */
            /* increase left r */
            let _offset = t * (EYES.width as f64);
            let r = r + t;
            let x_k =
                (center_point.0 + (r * cos_theta_i) as i32) - center_point.0 - EYES.width as i32;
            plot_point(&mut points, (x_k, y_k));
            plot(&mut buf, (x_k, y_k));
        } else {
            /* left */
            let x_k =
                (center_point.0 + (r * cos_theta_i) as i32) - center_point.0 - EYES.width as i32;
            plot_point(&mut points, (x_k, y_k));
            plot(&mut buf, (x_k, y_k));
        }
    }
    //eprintln!("got points:");
    //for row in points {
    //    for p in row {
    //        print!("{}", if p { "❖" } else { "." });
    //    }
    //    println!("");
    //}
    //for b in buf.iter_mut() {
    //    *b = 0b10101010;
    //}

    ret
}

fn create_tail_pixmap(t: f64) -> Vec<u8> {
    /*  Pendulum parameters */
    let mut sin_theta: f64;
    let mut cos_theta: f64;
    const A: f64 = 0.4;
    let omega: f64 = 1.0;
    let phi: f64 = 3.0 * FRAC_PI_2;
    let mut angle: f64;

    //    static XPoint tailOffset = { 74, -15 };
    const TAIL_OFFSET: (i32, i32) = (72, 0);

    let mut off_center_tail: Vec<(i32, i32)> = vec![(0, 0); N_TAIL_PTS]; /* off center tail    */
    let mut new_tail: Vec<(i32, i32)> = vec![(0, 0); N_TAIL_PTS]; /*  Tail at time "t"  */

    {
        /*
         *  Create an "off-center" tail to deal with the fact that
         *  the tail has a hook to it.  A real pendulum so shaped would
         *  hang a bit to the left (as you look at the cat).
         */
        angle = -0.08;
        sin_theta = f64::sin(angle);
        cos_theta = f64::cos(angle);

        for i in 0..N_TAIL_PTS {
            off_center_tail[i].0 = ((CENTER_TAIL[i].0 as f64) * cos_theta
                + ((CENTER_TAIL[i].1 as f64) * sin_theta))
                as i32;
            off_center_tail[i].1 = ((-1.0 * (CENTER_TAIL[i].0 as f64)) * sin_theta
                + ((CENTER_TAIL[i].1 as f64) * cos_theta))
                as i32;
        }
    }

    /*
     *  Compute pendulum function.
     */
    angle = A * f64::sin(omega * t + phi);
    sin_theta = f64::sin(angle);
    cos_theta = f64::cos(angle);

    let mut ret = TAIL.bits.to_vec();
    let mut buf = Buffer {
        vec: &mut ret,
        row_width: TAIL.width,
        height: TAIL.height,
    };
    /*
     *  Rotate the center tail about its origin by "angle" degrees.
     */
    for i in 0..N_TAIL_PTS {
        new_tail[i].0 = ((off_center_tail[i].0 as f64) * cos_theta
            + ((off_center_tail[i].1 as f64) * sin_theta)) as i32;
        new_tail[i].1 = ((off_center_tail[i].0 as f64 * -1.0) * sin_theta
            + ((off_center_tail[i].1 as f64) * cos_theta)) as i32;

        new_tail[i].0 += TAIL_OFFSET.0;
        new_tail[i].1 += TAIL_OFFSET.1;
    }

    const WIDTH: f64 = 15.0;
    const WIDTH2: f64 = WIDTH / 2.0;
    for window in new_tail.as_slice().windows(2) {
        let point_a = window[0];
        let point_b = window[1];
        plot_line_with_width(&mut buf, point_a, point_b, WIDTH as _);
    }

    let mut last_point = *new_tail.last().unwrap();
    last_point.1 += 1;
    for b in 0..=((0.8 * WIDTH2) as i32) {
        plot_ellipse(
            &mut buf,
            last_point,
            (WIDTH2 as i32, b),
            [false, false, true, true],
            1.0,
        );
    }

    ret
}

/*
macro_rules! tr {
    ($cond:expr ,? $then:expr ,: $else:expr) => {
        if $cond {
            $then
        } else {
            $else
        }
    };
}

fn create_eye_pixmap(t: f64) -> Vec<u8> {
    const A: f64 = 0.7;
    let omega: f64 = 1.0;
    let phi: f64 = 3.0 * FRAC_PI_2;
    let mut u: f64;
    let mut w: f64 = FRAC_PI_2;
    /*  Sphere parameters    */
    /*  Radius               */
    let mut r: f64 = 1.0;
    /*  Center of sphere     */
    let mut x0: f64 = 0.0;
    let mut y0: f64 = 0.0;
    let mut z0: f64 = 2.0;

    let mut angle: f64 = A * f64::sin(omega * t + phi) + w;
    let mut points: Vec<(i32, i32)> = Vec::with_capacity(100);

    let mut i = 0;
    u = -1.0 * FRAC_PI_2;
    while u < FRAC_PI_2 {
        let x = x0 + r * f64::cos(u) * f64::cos(angle + PI / 7.0);
        let z = z0 + r * f64::cos(u) * f64::sin(angle + PI / 7.0);
        let y = y0 + r * f64::sin(u);

        let a = ((tr!(z == 0.0 ,? x ,: x / z) * 23.0) + 12.0) as i32;
        let b = ((tr!(z == 0.0 ,? y ,: y / z) * 23.0) + 11.0) as i32;
        points.push((a, b));
        u += 0.25;
        i += 1;
    }

    u = FRAC_PI_2;
    while u > -1.0 * FRAC_PI_2 {
        let x = x0 + r * f64::cos(u) * f64::cos(angle - PI / 7.0);
        let z = z0 + r * f64::cos(u) * f64::sin(angle - PI / 7.0);
        let y = y0 + r * f64::sin(u);

        let a = ((tr!(z == 0.0 ,? x ,: x / z) * 23.0) + 12.0) as i32;
        let b = ((tr!(z == 0.0 ,? y ,: y / z) * 23.0) + 11.0) as i32;
        points.push((a, b));
        u -= 0.25;
        i += 1;
    }

    let mut ret = EYES.bits.to_vec();
    let mut buf = Buffer {
        vec: &mut ret,
        row_width: EYES.width,
        height: EYES.height,
    };
    for window in points.as_slice().windows(2) {
        let point_a = window[0];
        let point_b = window[1];
        plot_line_with_width(&mut buf, point_a, point_b, 1.);
    }
    for j in 0 .. i {
        points[j].0 -= 31;
    }
    for window in points.as_slice().windows(2) {
        let point_a = window[0];
        let point_b = window[1];
        plot_line_with_width(&mut buf, point_a, point_b, 1.);
    }
*/
/*
     *
        /*
         *  Create pixmap for drawing eye (and stippling on update)
         */
        XFillPolygon(dpy, eyeBitmap, bitmapGC, pts, i, Nonconvex, CoordModeOrigin);

        for (j = 0; j < i; j++) {
            pts[j].x += 31;
        }
        XFillPolygon(dpy, eyeBitmap, bitmapGC, pts, i, Nonconvex, CoordModeOrigin);

        XFreeGC(dpy, bitmapGC);

        return (eyeBitmap);
    }
    let mut ret = EYES.bits.to_vec();
    ret
}
    */

fn main() {
    let mut buffer: Vec<u32> = vec![WHITE; CAT_WIDTH * CAT_HEIGHT];
    let mut tails_frames: Vec<Image> = Vec::with_capacity(NUM_TAILS);
    let mut eyes_frames: Vec<Image> = Vec::with_capacity(NUM_TAILS);

    for i in 0..NUM_TAILS {
        tails_frames.push(Image {
            bytes: bits_to_bytes(
                &create_tail_pixmap(i as f64 * PI / (NUM_TAILS as f64)),
                TAIL.width,
            ),
            width: TAIL.width,
            height: TAIL.height,
            x_offset: TAIL.x_offset,
            y_offset: TAIL.y_offset,
        });
        eyes_frames.push(create_eye_pixmap(i as f64 * PI / (NUM_TAILS as f64)));
    }

    let mut window = Window::new(
        "Test - ESC to exit",
        CAT_WIDTH,
        CAT_HEIGHT,
        WindowOptions {
            title: true,
            //borderless: true,
            //resize: false,
            //transparency: true,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let catwhite = Image::from(CATWHITE);
    catwhite.draw(&mut buffer, WHITE, Some(WHITE));
    let catback = Image::from(CATBACK);
    catback.draw(&mut buffer, BLACK, None);
    let cattie = Image::from(CATTIE);
    cattie.draw(&mut buffer, AZURE_BLUE, None);
    let tail = Image::from(TAIL);
    tail.draw(&mut buffer, BLACK, None);
    let eyes = Image::from(EYES);
    eyes.draw(&mut buffer, BLACK, None);

    //CATTIE.draw(&mut buffer, AZURE_BLUE, None);
    //TAIL.draw(&mut buffer, black, None);
    //EYES.draw(&mut buffer, black, None);
    /*
     */

    let _blank_face = Image {
        bytes: vec![WHITE; hands::FACE_WIDTH * hands::FACE_HEIGHT],
        width: hands::FACE_WIDTH,
        height: hands::FACE_HEIGHT,
        x_offset: hands::FACE_OFFSET_X,
        y_offset: hands::FACE_OFFSET_Y,
    };
    let mut hour_hand = Image {
        bytes: vec![WHITE; hands::FACE_WIDTH * hands::FACE_HEIGHT],
        width: hands::FACE_WIDTH,
        height: hands::FACE_HEIGHT,
        x_offset: hands::FACE_OFFSET_X,
        y_offset: hands::FACE_OFFSET_Y,
    };
    let mut minute_hand = Image {
        bytes: vec![WHITE; hands::FACE_WIDTH * hands::FACE_HEIGHT],
        width: hands::FACE_WIDTH,
        height: hands::FACE_HEIGHT,
        x_offset: hands::FACE_OFFSET_X,
        y_offset: hands::FACE_OFFSET_Y,
    };
    let mut second_hand = Image {
        bytes: vec![WHITE; hands::FACE_WIDTH * hands::FACE_HEIGHT],
        width: hands::FACE_WIDTH,
        height: hands::FACE_HEIGHT,
        x_offset: hands::FACE_OFFSET_X,
        y_offset: hands::FACE_OFFSET_Y,
    };
    let mut i: usize = 0;
    let mut prev_i = i;
    let mut up = true;
    let mut system_now_second;
    let mut now_second = Instant::now();
    let mut seconds;
    let _now = SystemTime::now();
    let hour: u8 = 16;
    let mut minutes: u8 = 0;
    let mut passed_seconds = 60;
    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        let cur_tail = &tails_frames[i];
        tail.draw(&mut buffer, BLACK, Some(WHITE));
        cur_tail.draw(&mut buffer, BLACK, None);
        let cur_eyes = &eyes_frames[i];
        eyes_frames[prev_i].draw(&mut buffer, WHITE, None);
        prev_i = i;
        cur_eyes.draw(&mut buffer, BLACK, None);

        let new_now_second = Instant::now();

        if new_now_second - now_second >= Duration::from_secs(1) {
            passed_seconds += 1;
            now_second = new_now_second;
            system_now_second = SystemTime::now();
            seconds = system_now_second
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            //blank_face.draw(&mut buffer, WHITE, Some(WHITE));
            second_hand.draw(&mut buffer, WHITE, None);
            second_hand.clear();
            hands::draw_second(&mut second_hand, 23, 1, -5, (seconds as f64) / 60.0);
            second_hand.draw(&mut buffer, BLACK, None);
        }
        if passed_seconds >= 60 {
            passed_seconds = 0;
            minutes += 1;
            if minutes == 60 {
                minutes = 0;
            }
            minute_hand.draw(&mut buffer, WHITE, None);
            minute_hand.clear();
            hands::draw_second(&mut minute_hand, 29, 3, -5, (minutes as f64) / 60.0);
        }
        minute_hand.draw(&mut buffer, BLACK, None);
        //blank_face.draw(&mut buffer, WHITE, Some(WHITE));
        hour_hand.draw(&mut buffer, WHITE, None);
        hour_hand.clear();
        hands::draw_second(&mut hour_hand, 18, 4, -5, (hour as f64) / 12.0);
        hour_hand.draw(&mut buffer, BLACK, None);
        //}

        if up {
            if i + 1 == tails_frames.len() {
                up = false;
            } else {
                i = (i + 1).wrapping_rem(tails_frames.len());
            }
        } else {
            if i == 0 {
                up = true;
            } else {
                i -= 1;
            }
        }
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, CAT_WIDTH, CAT_HEIGHT)
            .unwrap();

        let millis = std::time::Duration::from_millis(100);

        std::thread::sleep(millis);
    }
}
