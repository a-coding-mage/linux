/* SPDX-License-Identifier: GPL-2.0-or-later */

pub const MXC_CPU_MX1: u32 = 1;
pub const MXC_CPU_MX21: u32 = 21;
pub const MXC_CPU_MX25: u32 = 25;
pub const MXC_CPU_MX27: u32 = 27;
pub const MXC_CPU_MX31: u32 = 31;
pub const MXC_CPU_MX35: u32 = 35;
pub const MXC_CPU_MX50: u32 = 50;
pub const MXC_CPU_MX51: u32 = 51;
pub const MXC_CPU_MX53: u32 = 53;
pub const MXC_CPU_IMX6SL: u32 = 0x60;
pub const MXC_CPU_IMX6DL: u32 = 0x61;
pub const MXC_CPU_IMX6SX: u32 = 0x62;
pub const MXC_CPU_IMX6Q: u32 = 0x63;
pub const MXC_CPU_IMX6UL: u32 = 0x64;
pub const MXC_CPU_IMX6ULL: u32 = 0x65;
/* virtual cpu id for i.mx6ulz */
pub const MXC_CPU_IMX6ULZ: u32 = 0x6b;
pub const MXC_CPU_IMX6SLL: u32 = 0x67;
pub const MXC_CPU_IMX7D: u32 = 0x72;
pub const MXC_CPU_IMX7ULP: u32 = 0xff;

pub const MXC_CPU_VFx10: u32 = 0x010;
pub const MXC_CPU_VF500: u32 = 0x500;
pub const MXC_CPU_VF510: u32 = MXC_CPU_VF500 | MXC_CPU_VFx10;
pub const MXC_CPU_VF600: u32 = 0x600;
pub const MXC_CPU_VF610: u32 = MXC_CPU_VF600 | MXC_CPU_VFx10;

unsafe extern "C" {
    pub static mut __mxc_cpu_type: u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
