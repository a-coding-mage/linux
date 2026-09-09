/* SPDX-License-Identifier: GPL-2.0 */
/* Header for the Gemini SATA bridge */

#[repr(C)]
pub struct sata_gemini {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum gemini_muxmode {
    GEMINI_MUXMODE_0 = 0,
    GEMINI_MUXMODE_1,
    GEMINI_MUXMODE_2,
    GEMINI_MUXMODE_3,
}

unsafe extern "C" {
    pub fn gemini_sata_bridge_get() -> *mut sata_gemini;
    pub fn gemini_sata_bridge_enabled(sg: *mut sata_gemini, is_ata1: bool) -> bool;
    pub fn gemini_sata_get_muxmode(sg: *mut sata_gemini) -> gemini_muxmode;
    pub fn gemini_sata_start_bridge(sg: *mut sata_gemini, bridge: u32) -> i32;
    pub fn gemini_sata_stop_bridge(sg: *mut sata_gemini, bridge: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
