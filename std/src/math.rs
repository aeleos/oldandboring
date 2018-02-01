// double pow(double x, double pow) {
// 	double ret = x;
// 	for (int i = 0; i < pow; i++) {
// 		ret *= x;
// 	}
// 	return ret;
// }
pub use core::f32::consts::PI as PI32;
pub use core::f64::consts::PI as PI64;
// pub const PI32DIV2 =
pub const FACTORIAL_2: u32 = 2u32;
pub const FACTORIAL_3: u32 = 6u32;
pub const FACTORIAL_4: u32 = 24u32;
pub const FACTORIAL_5: u32 = 120u32;
pub const FACTORIAL_6: u32 = 720u32;
pub const FACTORIAL_7: u32 = 5040u32;
pub const FACTORIAL_8: u32 = 40320u32;
pub const FACTORIAL_9: u32 = 362880u32;

pub fn radiansf32(degrees: f32) -> f32 {
    degrees * PI32 / 180.0
}

pub fn radiansf64(degrees: f64) -> f64 {
    degrees * PI64 / 180.0
}

pub fn degreesf32(radians: f32) -> f32 {
    radians * PI32 / 180.0
}

pub fn degreesf64(radians: f64) -> f64 {
    radians * PI64 / 180.0
}

// // xdeg = fmod(xdeg, 360);
// // if (xdeg > 180) xdeg = -(xdeg - 180);
// // else if (xdeg < -180) xdeg = xdeg + 360;
// // // Now -180 <= xdeg <= 180
// //
// // if (xdeg > 90) xdeg = 180-xdeg;
// // else if (xdeg < -90) xdeg = -180 - xdeg;
//
// pub fn nsinf32(degrees: f32) -> f32 {
//     let mut norm_degrees = degrees;
//
//     if degrees > 360.0 || degrees < -360.0 {
//         let rounded = degrees as i32;
//         let remaining = degrees - rounded as f32;
//         let remainder = rounded % 360;
//         norm_degrees = remainder as f32 + remaining;
//     }
//
//     if norm_degrees > 180.0 {
//         norm_degrees = -(norm_degrees - 180.0);
//     } else if norm_degrees < -180.0 {
//         norm_degrees += 360.0;
//     }
//
//     if norm_degrees > 90.0 {
//         norm_degrees = 180.0 - norm_degrees;
//     } else if degrees < -90.0 {
//         norm_degrees = -180.0 - norm_degrees;
//     }
//
//     return degrees;
// }

pub fn normalizef32(mut degrees: f32) -> f32 {
    if degrees > 360.0 || degrees < -360.0 {
        let rounded = degrees as i32;
        let remaining = degrees - rounded as f32;
        let remainder = rounded % 360;
        degrees = remainder as f32 + remaining;
    }

    if degrees > 180.0 {
        degrees = -(degrees - 180.0);
    } else if degrees < -180.0 {
        degrees += 360.0;
    }

    // if degrees > 90.0 {
    //     degrees = 180.0 - degrees;
    // } else if degrees < -90.0 {
    //     degrees = -180.0 - degrees;
    // }
    return degrees;
}

pub fn normalizef64(mut degrees: f64) -> f64 {
    if degrees > 360.0 || degrees < -360.0 {
        let rounded = degrees as i64;
        let remaining = degrees - rounded as f64;
        let remainder = rounded % 360;
        degrees = remainder as f64 + remaining;
    }

    degrees %= 360.0;
    if degrees > 180.0 {
        degrees = -(degrees - 180.0);
    } else if degrees < -180.0 {
        degrees += 360.0;
    }

    if degrees > 90.0 {
        degrees = 180.0 - degrees;
    } else if degrees < -90.0 {
        degrees = -180.0 - degrees;
    }
    return degrees;
}

pub fn powf64(x: f64, pow: f64) -> f64 {
    let mut ret = x;
    for _ in 0..pow as u64 {
        ret *= x;
    }
    ret
}

pub fn powf32(x: f32, pow: f32) -> f32 {
    let mut ret = x;
    for _ in 0..pow as u32 {
        ret *= x;
    }
    ret
}

// unsigned long factorial(unsigned long x) {
// 	if (x == 0) return 1;
// 	return (x * factorial(x - 1));
// }

pub fn factorialf32(x: u32) -> u32 {
    if x == 0 {
        return 1;
    }
    (x * factorialf32(x - 1))
}

pub fn factorialf64(x: u64) -> u64 {
    if x == 0 {
        return 1;
    }
    (x * factorialf64(x - 1))
}

// double sin(double x) {
// 	//approximate taylor series for sin
// 	double ret = x;
// 	ret -= (pow(x, 3)/factorial(3));
// 	ret += (pow(x, 5)/factorial(5));
// 	ret -= (pow(x, 7)/factorial(7));
// 	return ret;
// }

pub fn sinf64(x: f64) -> f64 {
    let mut ret: f64 = x;
    let mut x_temp = x * x * x;
    ret -= x_temp / FACTORIAL_3 as f64;
    x_temp = x_temp * x * x;
    ret += x_temp / FACTORIAL_5 as f64;
    x_temp = x_temp * x * x;
    ret -= x_temp / FACTORIAL_7 as f64;
    ret
}

pub fn sinf32(x: f32) -> f32 {
    let mut ret: f32 = x;
    let mut x_temp = x * x * x;
    ret -= x_temp / FACTORIAL_3 as f32;
    x_temp = x_temp * x * x;
    ret += x_temp / FACTORIAL_5 as f32;
    x_temp = x_temp * x * x;
    ret -= x_temp / FACTORIAL_7 as f32;
    ret
}

// double cos(double x) {
// 	//approximate taylor series for cos
// 	double ret = 1;
// 	ret -= (pow(x, 2)/factorial(2));
// 	ret += (pow(x, 4)/factorial(4));
// 	ret -= (pow(x, 6)/factorial(6));
// 	return ret;
// }

pub fn cosf64(x: f64) -> f64 {
    let mut ret: f64 = 1.0;
    let mut x_temp = x * x;
    ret -= x_temp / FACTORIAL_2 as f64;
    x_temp = x_temp * x * x;
    ret += x_temp / FACTORIAL_4 as f64;
    x_temp = x_temp * x * x;
    ret -= x_temp / FACTORIAL_6 as f64;
    ret
}

pub fn cosf32(x: f32) -> f32 {
    let mut ret: f32 = 1.0;
    let mut x_temp = x * x;
    ret -= x_temp / FACTORIAL_2 as f32;
    x_temp = x_temp * x * x;
    ret += x_temp / FACTORIAL_4 as f32;
    x_temp = x_temp * x * x;
    ret -= x_temp / FACTORIAL_6 as f32;
    ret
}
