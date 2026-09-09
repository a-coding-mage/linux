/* SPDX-License-Identifier: GPL-2.0 */
// C header guard __SH_KEYSC_H__ omitted from executable Rust.

pub const SH_KEYSC_MAXKEYS: usize = 64;

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ShKeyscMode {
    SH_KEYSC_MODE_1 = 0,
    SH_KEYSC_MODE_2 = 1,
    SH_KEYSC_MODE_3 = 2,
    SH_KEYSC_MODE_4 = 3,
    SH_KEYSC_MODE_5 = 4,
    SH_KEYSC_MODE_6 = 5,
}

#[repr(C)]
pub struct sh_keysc_info {
    pub mode: ShKeyscMode,
    pub scan_timing: i32, /* 0 -> 7, see KYCR1, SCN[2:0] */
    pub delay: i32,
    pub kycr2_delay: i32,
    pub keycodes: [i32; SH_KEYSC_MAXKEYS], /* KEYIN * KEYOUT */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
