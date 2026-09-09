/* SPDX-License-Identifier: GPL-2.0 */

/**
 * struct aemif_cs_timings: structure to hold CS timing configuration
 * values are expressed in number of clock cycles - 1
 * @ta: minimum turn around time
 * @rhold: read hold width
 * @rstrobe: read strobe width
 * @rsetup: read setup width
 * @whold: write hold width
 * @wstrobe: write strobe width
 * @wsetup: write setup width
 */
#[repr(C)]
pub struct aemif_cs_timings {
    pub ta: u32,
    pub rhold: u32,
    pub rstrobe: u32,
    pub rsetup: u32,
    pub whold: u32,
    pub wstrobe: u32,
    pub wsetup: u32,
}

#[repr(C)]
pub struct aemif_device {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn aemif_set_cs_timings(
        aemif: *mut aemif_device,
        cs: u8,
        timings: *mut aemif_cs_timings,
    ) -> i32;
    pub fn aemif_check_cs_timings(timings: *mut aemif_cs_timings) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
