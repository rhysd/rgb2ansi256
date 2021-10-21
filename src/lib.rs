// Ported from https://github.com/mina86/ansi_colours/blob/eef25edf851c0d78e1b68b713a238c5aaf562fbd/src/ansi256.c

#![allow(clippy::many_single_char_names)]

const ANSI256_FROM_GREY: [u8; 256] = [
    16, 16, 16, 16, 16, 232, 232, 232, 232, 232, 232, 232, 232, 232, 233, 233, 233, 233, 233, 233,
    233, 233, 233, 233, 234, 234, 234, 234, 234, 234, 234, 234, 234, 234, 235, 235, 235, 235, 235,
    235, 235, 235, 235, 235, 236, 236, 236, 236, 236, 236, 236, 236, 236, 236, 237, 237, 237, 237,
    237, 237, 237, 237, 237, 237, 238, 238, 238, 238, 238, 238, 238, 238, 238, 238, 239, 239, 239,
    239, 239, 239, 239, 239, 239, 239, 240, 240, 240, 240, 240, 240, 240, 240, 59, 59, 59, 59, 59,
    241, 241, 241, 241, 241, 241, 241, 242, 242, 242, 242, 242, 242, 242, 242, 242, 242, 243, 243,
    243, 243, 243, 243, 243, 243, 243, 244, 244, 244, 244, 244, 244, 244, 244, 244, 102, 102, 102,
    102, 102, 245, 245, 245, 245, 245, 245, 246, 246, 246, 246, 246, 246, 246, 246, 246, 246, 247,
    247, 247, 247, 247, 247, 247, 247, 247, 247, 248, 248, 248, 248, 248, 248, 248, 248, 248, 145,
    145, 145, 145, 145, 249, 249, 249, 249, 249, 249, 250, 250, 250, 250, 250, 250, 250, 250, 250,
    250, 251, 251, 251, 251, 251, 251, 251, 251, 251, 251, 252, 252, 252, 252, 252, 252, 252, 252,
    252, 188, 188, 188, 188, 188, 253, 253, 253, 253, 253, 253, 254, 254, 254, 254, 254, 254, 254,
    254, 254, 254, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 231, 231,
    231, 231, 231, 231, 231, 231, 231,
];

pub const fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> u8 {
    let rgb = ((r as u32) << 16) + ((g as u32) << 8) + b as u32;
    if r == g && g == b {
        return ANSI256_FROM_GREY[(rgb as usize) & 0xff];
    }

    let grey_index: u8 = ANSI256_FROM_GREY[luminance(rgb) as usize];
    let grey_distance: u32 = distance(rgb, rgb_from_ansi256(grey_index));

    let cube: u32 = cube_index_red(r) + cube_index_green(g) + cube_index_blue(b);
    if distance(rgb, cube) < grey_distance {
        (cube >> 24) as u8
    } else {
        grey_index
    }
}

const fn luminance(rgb: u32) -> u8 {
    let v: u32 =
        3567664u32 * (rgb >> 16 & 0xff) + 11998547u32 * (rgb >> 8 & 0xff) + 1211005 * (rgb & 0xff);
    ((v + (1 << 23)) >> 24) as u8
}

const fn rgb_from_ansi256(index: u8) -> u32 {
    const SYSTEM_COLOURS: [u32; 16] = [
        0x000000, 0xcd0000, 0x00cd00, 0xcdcd00, 0x0000ee, 0xcd00cd, 0x00cdcd, 0xe5e5e5, 0x7f7f7f,
        0xff0000, 0x00ff00, 0xffff00, 0x5c5cff, 0xff00ff, 0x00ffff, 0xffffff,
    ];

    if index < 16 {
        SYSTEM_COLOURS[index as usize]
    } else if index < 232 {
        let index = index - 16;
        (cube_value(index / 36) << 16) | (cube_value(index / 6 % 6) << 8) | (cube_value(index % 6))
    } else {
        let index = (index - 232) * 10 + 8;
        (index as u32) * 0x010101
    }
}

const fn cube_value(idx: u8) -> u32 {
    [0, 95, 135, 175, 215, 255][idx as usize]
}

const fn distance(x: u32, y: u32) -> u32 {
    const fn r(c: u32) -> i32 {
        ((c >> 16) & 0xff) as i32
    }
    const fn g(c: u32) -> i32 {
        ((c >> 8) & 0xff) as i32
    }
    const fn b(c: u32) -> i32 {
        (c & 0xff) as i32
    }

    let r_sum: i32 = r(x) + r(y);
    let r = r(x) - r(y);
    let g = g(x) - g(y);
    let b = b(x) - b(y);

    ((1024 + r_sum) * r * r + 2048 * g * g + (1534 - r_sum) * b * b) as u32
}

#[rustfmt::skip]
macro_rules! cube_thresholds {
    ($v:ident, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $idx:ident) => {
        if      $v < $a { $idx(0,   0) }
        else if $v < $b { $idx(1,  95) }
        else if $v < $c { $idx(2, 135) }
        else if $v < $d { $idx(3, 175) }
        else if $v < $e { $idx(4, 215) }
        else            { $idx(5, 255) }
    }
}

const fn cube_index_red(v: u8) -> u32 {
    const fn idx(i: u32, v: u32) -> u32 {
        ((i * 36 + 16) << 24) | (v << 16)
    }
    cube_thresholds!(v, 38, 115, 155, 196, 235, idx)
}

const fn cube_index_green(v: u8) -> u32 {
    const fn idx(i: u32, v: u32) -> u32 {
        ((i * 6) << 24) | (v << 8)
    }
    cube_thresholds!(v, 36, 116, 154, 195, 235, idx)
}

const fn cube_index_blue(v: u8) -> u32 {
    const fn idx(i: u32, v: u32) -> u32 {
        (i << 24) | v
    }
    cube_thresholds!(v, 35, 115, 155, 195, 235, idx)
}

#[cfg(test)]
mod tests {
    use super::rgb_to_ansi256;

    const MEDIUM_SPRING_GREEN: u8 = rgb_to_ansi256(0, 255, 175);

    #[test]
    fn validate_const_eval() {
        assert_eq!(MEDIUM_SPRING_GREEN, 49);
    }

    #[test]
    fn validate_with_ansi_colours() {
        for r in 0..=255 {
            for g in 0..=255 {
                for b in 0..=255 {
                    let got = rgb_to_ansi256(r, g, b);
                    let expected = ansi_colours::ansi256_from_rgb((r, g, b));
                    assert_eq!(expected, got);
                }
            }
        }
    }
}
