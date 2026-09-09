/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied externally: linux/clk-provider.h

#[repr(C)]
pub struct krait_mux_clk {
    pub parent_map: *mut u32,
    pub offset: u32,
    pub mask: u32,
    pub shift: u32,
    pub en_mask: u32,
    pub lpl: bool,
    pub safe_sel: u8,
    pub old_index: u8,
    pub reparent: bool,
    pub disable_sec_src_gating: bool,

    pub hw: clk_hw,
    pub clk_nb: notifier_block,
}

#[inline]
pub unsafe fn to_krait_mux_clk(_hw: *mut clk_hw) -> *mut krait_mux_clk {
    (_hw as *mut u8).sub(std::mem::offset_of!(krait_mux_clk, hw)) as *mut krait_mux_clk
}

unsafe extern "C" {
    pub static krait_mux_clk_ops: clk_ops;
}

#[repr(C)]
pub struct krait_div2_clk {
    pub offset: u32,
    pub width: u8,
    pub shift: u32,
    pub lpl: bool,

    pub hw: clk_hw,
}

#[inline]
pub unsafe fn to_krait_div2_clk(_hw: *mut clk_hw) -> *mut krait_div2_clk {
    (_hw as *mut u8).sub(std::mem::offset_of!(krait_div2_clk, hw)) as *mut krait_div2_clk
}

unsafe extern "C" {
    pub static krait_div2_clk_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
