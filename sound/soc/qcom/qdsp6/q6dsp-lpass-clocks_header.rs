/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct q6dsp_clk_init {
    pub clk_id: core::ffi::c_int,
    pub q6dsp_clk_id: core::ffi::c_int,
    pub name: *mut core::ffi::c_char,
    pub rate: core::ffi::c_int,
}

macro_rules! Q6DSP_VOTE_CLK {
    ($id:expr, $blkid:expr, $n:expr) => {
        q6dsp_clk_init {
            clk_id: $id,
            q6dsp_clk_id: $blkid,
            name: $n,
            rate: 0,
        }
    };
}

#[repr(C)]
pub struct q6dsp_clk_desc {
    pub clks: *const q6dsp_clk_init,
    pub num_clks: usize,
    pub lpass_set_clk: Option<
        unsafe extern "C" fn(
            dev: *mut device,
            clk_id: core::ffi::c_int,
            attr: core::ffi::c_int,
            root_clk: core::ffi::c_int,
            freq: core::ffi::c_uint,
        ) -> core::ffi::c_int,
    >,
    pub lpass_vote_clk: Option<
        unsafe extern "C" fn(
            dev: *mut device,
            hid: u32,
            n: *const core::ffi::c_char,
            h: *mut u32,
        ) -> core::ffi::c_int,
    >,
    pub lpass_unvote_clk: Option<
        unsafe extern "C" fn(
            dev: *mut device,
            hid: u32,
            h: u32,
        ) -> core::ffi::c_int,
    >,
}

unsafe extern "C" {
    pub fn q6dsp_clock_dev_probe(pdev: *mut platform_device) -> core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
