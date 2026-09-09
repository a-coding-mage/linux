/* SPDX-License-Identifier: GPL-2.0 */

/* Kernel controls MTRR and/or PAT MSRs. */
extern "C" {
    pub static mut memory_caching_control: u32;
}

pub const CACHE_MTRR: u32 = 0x01;
pub const CACHE_PAT: u32 = 0x02;

extern "C" {
    pub fn cache_disable();
    pub fn cache_enable();
    pub fn set_cache_aps_delayed_init(val: bool);
    pub fn get_cache_aps_delayed_init() -> bool;
    pub fn cache_bp_init();
    pub fn cache_bp_restore();
    pub fn cache_aps_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
