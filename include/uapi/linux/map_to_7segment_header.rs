/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* Translated from map_to_7segment.h. */

pub const BIT_SEG7_A: u32 = 0;
pub const BIT_SEG7_B: u32 = 1;
pub const BIT_SEG7_C: u32 = 2;
pub const BIT_SEG7_D: u32 = 3;
pub const BIT_SEG7_E: u32 = 4;
pub const BIT_SEG7_F: u32 = 5;
pub const BIT_SEG7_G: u32 = 6;
pub const BIT_SEG7_RESERVED: u32 = 7;

#[repr(C)]
pub struct seg7_conversion_map {
    pub table: [u8; 128],
}

#[inline]
pub fn map_to_seg7(map: *mut seg7_conversion_map, c: i32) -> i32 {
    if c >= 0 && (c as usize) < core::mem::size_of_val(unsafe { &(*map).table }) {
        unsafe { (*map).table[c as usize] as i32 }
    } else {
        -22 /* -EINVAL */
    }
}

#[macro_export]
macro_rules! SEG7_CONVERSION_MAP {
    ($name:ident, $map:expr) => {
        pub static mut $name: $crate::seg7_conversion_map =
            $crate::seg7_conversion_map { table: $map };
    };
}

pub const MAP_TO_SEG7_SYSFS_FILE: &str = "map_seg7";

pub const fn _SEG7(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8) -> u8 {
    (a << BIT_SEG7_A) | (b << BIT_SEG7_B) | (c << BIT_SEG7_C) |
        (d << BIT_SEG7_D) | (e << BIT_SEG7_E) | (f << BIT_SEG7_F) |
        (g << BIT_SEG7_G)
}

const Z: u8 = 0;
const fn ascii_alphanum() -> [u8; 128] {
    let mut t = [0u8; 128];
    let mut i = 33usize;
    while i <= 126 { t[i] = match i {
        33 => _SEG7(0,0,0,0,1,1,0), 34 => _SEG7(0,1,0,0,0,1,0), 35 => _SEG7(0,1,1,0,1,1,0),
        36 => _SEG7(1,0,1,1,0,1,1), 37 => _SEG7(0,0,1,0,0,1,0), 38 => _SEG7(1,0,1,1,1,1,1),
        39 => _SEG7(0,0,0,0,0,1,0), 40 => _SEG7(1,0,0,1,1,1,0), 41 => _SEG7(1,1,1,1,0,0,0),
        42 => _SEG7(0,1,1,0,1,1,1), 43 => _SEG7(0,1,1,0,0,0,1), 44|46 => _SEG7(0,0,0,0,1,0,0),
        45 => _SEG7(0,0,0,0,0,0,1), 47 => _SEG7(0,1,0,0,1,0,1),
        48 => _SEG7(1,1,1,1,1,1,0), 49 => _SEG7(0,1,1,0,0,0,0), 50 => _SEG7(1,1,0,1,1,0,1),
        51 => _SEG7(1,1,1,1,0,0,1), 52 => _SEG7(0,1,1,0,0,1,1), 53 => _SEG7(1,0,1,1,0,1,1),
        54 => _SEG7(1,0,1,1,1,1,1), 55 => _SEG7(1,1,1,0,0,0,0), 56 => _SEG7(1,1,1,1,1,1,1),
        57 => _SEG7(1,1,1,1,0,1,1), 58|59|61 => _SEG7(0,0,0,1,0,0,1), 60 => _SEG7(1,0,0,0,0,1,1),
        62 => _SEG7(1,1,0,0,0,0,1), 63 => _SEG7(1,1,1,0,0,1,0), 64 => _SEG7(1,1,0,1,1,1,1),
        65 => _SEG7(1,1,1,0,1,1,1), 66 => _SEG7(1,1,1,1,1,1,1), 67 => _SEG7(1,0,0,1,1,1,0),
        68 => _SEG7(1,1,1,1,1,1,0), 69 => _SEG7(1,0,0,1,1,1,1), 70 => _SEG7(1,0,0,0,1,1,1),
        71 => _SEG7(1,1,1,1,0,1,1), 72 => _SEG7(0,1,1,0,1,1,1), 73|75 => _SEG7(0,1,1,0,1,1,1),
        74 => _SEG7(0,1,1,1,0,0,0), 76 => _SEG7(0,0,0,1,1,1,0), 77|78 => _SEG7(1,1,1,0,1,1,0),
        79 => _SEG7(1,1,1,1,1,1,0), 80 => _SEG7(1,1,0,0,1,1,1), 81 => _SEG7(1,1,1,1,1,1,0),
        82 => _SEG7(1,1,1,0,1,1,1), 83 => _SEG7(1,0,1,1,0,1,1), 84 => _SEG7(0,0,0,1,1,1,1),
        85|86 => _SEG7(0,1,1,1,1,1,0), 87 => _SEG7(0,1,1,1,1,1,1), 88 => _SEG7(0,1,1,0,1,1,1),
        89 => _SEG7(0,1,1,0,0,1,1), 90 => _SEG7(1,1,0,1,1,0,1),
        91 => _SEG7(1,0,0,1,1,1,0), 92 => _SEG7(0,0,1,0,0,1,1), 93 => _SEG7(1,1,1,1,0,0,0),
        94 => _SEG7(1,1,0,0,0,1,0), 95 => _SEG7(0,0,0,1,0,0,0), 96 => _SEG7(0,1,0,0,0,0,0),
        97 => _SEG7(1,1,1,0,1,1,1), 98 => _SEG7(0,0,1,1,1,1,1), 99 => _SEG7(0,0,0,1,1,0,1),
        100 => _SEG7(0,1,1,1,1,0,1), 101 => _SEG7(1,0,0,1,1,1,1), 102 => _SEG7(1,0,0,0,1,1,1),
        103 => _SEG7(1,1,1,1,0,1,1), 104 => _SEG7(0,0,1,0,1,1,1), 105 => _SEG7(0,0,1,0,0,0,0),
        106 => _SEG7(0,0,1,1,0,0,0), 107 => _SEG7(0,0,1,0,1,1,1), 108 => _SEG7(0,0,0,1,1,1,0),
        109 => _SEG7(1,1,1,0,1,1,0), 110 => _SEG7(0,0,1,0,1,0,1), 111 => _SEG7(0,0,1,1,1,0,1),
        112 => _SEG7(1,1,0,0,1,1,1), 113 => _SEG7(1,1,1,0,0,1,1), 114 => _SEG7(0,0,0,0,1,0,1),
        115 => _SEG7(1,0,1,1,0,1,1), 116 => _SEG7(0,0,0,1,1,1,1), 117|118 => _SEG7(0,0,1,1,1,0,0),
        119 => _SEG7(0,1,1,1,1,1,1), 120 => _SEG7(0,1,1,0,1,1,1), 121 => _SEG7(0,1,1,1,0,1,1),
        122 => _SEG7(1,1,0,1,1,0,1), 123 => _SEG7(1,0,0,1,1,1,0), 124 => _SEG7(0,0,0,0,1,1,0),
        125 => _SEG7(1,1,1,1,0,0,0), 126 => _SEG7(1,0,0,0,0,0,0), _ => Z
    }; i += 1; }
    t
}

pub const MAP_ASCII7SEG_ALPHANUM: [u8; 128] = ascii_alphanum();
const fn ascii_alphanum_lc() -> [u8; 128] {
    let mut t = ascii_alphanum();
    let mut i = 65usize;
    while i <= 90 { t[i] = t[i + 32]; i += 1; }
    t
}
pub const MAP_ASCII7SEG_ALPHANUM_LC: [u8; 128] = ascii_alphanum_lc();

#[macro_export]
macro_rules! SEG7_DEFAULT_MAP {
    ($name:ident) => {
        $crate::SEG7_CONVERSION_MAP!($name, $crate::MAP_ASCII7SEG_ALPHANUM);
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
