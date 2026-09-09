// SPDX-License-Identifier: GPL-2.0-or-later
/*
    NetWinder Floating Point Emulator
    (c) Rebel.COM, 1998,1999

    Direct questions, comments to Scott Bambrough <scottb@netwinder.org>

*/

// Dependencies supplied by the surrounding translation unit/build:
// fpa11.h, softfloat.h, fpopcode.h, fpsr.h, fpmodule.h, and fpmodule.inl.

#[cfg(CONFIG_FPE_NWFPE_XP)]
pub const floatx80Constant: [floatx80; 8] = [
    floatx80 { high: 0x0000, low: 0x0000000000000000u64 }, // extended 0.0
    floatx80 { high: 0x3fff, low: 0x8000000000000000u64 }, // extended 1.0
    floatx80 { high: 0x4000, low: 0x8000000000000000u64 }, // extended 2.0
    floatx80 { high: 0x4000, low: 0xc000000000000000u64 }, // extended 3.0
    floatx80 { high: 0x4001, low: 0x8000000000000000u64 }, // extended 4.0
    floatx80 { high: 0x4001, low: 0xa000000000000000u64 }, // extended 5.0
    floatx80 { high: 0x3ffe, low: 0x8000000000000000u64 }, // extended 0.5
    floatx80 { high: 0x4002, low: 0xa000000000000000u64 }, // extended 10.0
];

pub const float64Constant: [float64; 8] = [
    0x0000000000000000u64, // double 0.0
    0x3ff0000000000000u64, // double 1.0
    0x4000000000000000u64, // double 2.0
    0x4008000000000000u64, // double 3.0
    0x4010000000000000u64, // double 4.0
    0x4014000000000000u64, // double 5.0
    0x3fe0000000000000u64, // double 0.5
    0x4024000000000000u64, // double 10.0
];

pub const float32Constant: [float32; 8] = [
    0x00000000u32, // single 0.0
    0x3f800000u32, // single 1.0
    0x40000000u32, // single 2.0
    0x40400000u32, // single 3.0
    0x40800000u32, // single 4.0
    0x40a00000u32, // single 5.0
    0x3f000000u32, // single 0.5
    0x41200000u32, // single 10.0
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
