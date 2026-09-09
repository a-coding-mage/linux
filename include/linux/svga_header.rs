/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the Linux PCI and VGA headers are referenced here. */

pub const VGA_REGSET_END_VAL: u8 = 0xFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vga_regset {
    pub regnum: u8,
    pub lowbit: u8,
    pub highbit: u8,
}

pub const VGA_REGSET_END: vga_regset = vga_regset {
    regnum: VGA_REGSET_END_VAL,
    lowbit: 0,
    highbit: 0,
};

pub const SVGA_FORMAT_END_VAL: u16 = 0xFFFF;

#[repr(C)]
pub struct svga_fb_format {
    pub bits_per_pixel: u32,
    pub red: fb_bitfield,
    pub green: fb_bitfield,
    pub blue: fb_bitfield,
    pub transp: fb_bitfield,
    pub nonstd: u32,
    pub r#type: u32,
    pub type_aux: u32,
    pub visual: u32,
    pub xpanstep: u32,
    pub xresstep: u32,
}

pub const SVGA_FORMAT_END: svga_fb_format = svga_fb_format {
    bits_per_pixel: SVGA_FORMAT_END_VAL as u32,
    red: fb_bitfield { offset: 0, length: 0, msb_right: 0 },
    green: fb_bitfield { offset: 0, length: 0, msb_right: 0 },
    blue: fb_bitfield { offset: 0, length: 0, msb_right: 0 },
    transp: fb_bitfield { offset: 0, length: 0, msb_right: 0 },
    nonstd: 0,
    r#type: 0,
    type_aux: 0,
    visual: 0,
    xpanstep: 0,
    xresstep: 0,
};

#[repr(C)]
pub struct svga_timing_regs {
    pub h_total_regs: *const vga_regset,
    pub h_display_regs: *const vga_regset,
    pub h_blank_start_regs: *const vga_regset,
    pub h_blank_end_regs: *const vga_regset,
    pub h_sync_start_regs: *const vga_regset,
    pub h_sync_end_regs: *const vga_regset,
    pub v_total_regs: *const vga_regset,
    pub v_display_regs: *const vga_regset,
    pub v_blank_start_regs: *const vga_regset,
    pub v_blank_end_regs: *const vga_regset,
    pub v_sync_start_regs: *const vga_regset,
    pub v_sync_end_regs: *const vga_regset,
}

#[repr(C)]
pub struct svga_pll {
    pub m_min: u16,
    pub m_max: u16,
    pub n_min: u16,
    pub n_max: u16,
    pub r_min: u16,
    pub r_max: u16,
    pub f_vco_min: u32,
    pub f_vco_max: u32,
    pub f_base: u32,
}

#[inline]
pub unsafe fn svga_wattr(regbase: *mut core::ffi::c_void, index: u8, data: u8) {
    vga_r(regbase, VGA_IS1_RC);
    vga_w(regbase, VGA_ATT_IW, index);
    vga_w(regbase, VGA_ATT_W, data);
}

#[inline]
pub unsafe fn svga_wseq_mask(
    regbase: *mut core::ffi::c_void,
    index: u8,
    data: u8,
    mask: u8,
) {
    vga_wseq(regbase, index, (data & mask) | (vga_rseq(regbase, index) & !mask));
}

#[inline]
pub unsafe fn svga_wcrt_mask(
    regbase: *mut core::ffi::c_void,
    index: u8,
    data: u8,
    mask: u8,
) {
    vga_wcrt(regbase, index, (data & mask) | (vga_rcrt(regbase, index) & !mask));
}

#[inline]
pub unsafe fn svga_primary_device(dev: *mut pci_dev) -> i32 {
    let mut flags: u16 = 0;
    pci_read_config_word(dev, PCI_COMMAND, &mut flags);
    (flags & PCI_COMMAND_IO) as i32
}

extern "C" {
    pub fn svga_wcrt_multi(regbase: *mut core::ffi::c_void, regset: *const vga_regset, value: u32);
    pub fn svga_wseq_multi(regbase: *mut core::ffi::c_void, regset: *const vga_regset, value: u32);
    pub fn svga_set_default_gfx_regs(regbase: *mut core::ffi::c_void);
    pub fn svga_set_default_atc_regs(regbase: *mut core::ffi::c_void);
    pub fn svga_set_default_seq_regs(regbase: *mut core::ffi::c_void);
    pub fn svga_set_default_crt_regs(regbase: *mut core::ffi::c_void);
    pub fn svga_set_textmode_vga_regs(regbase: *mut core::ffi::c_void);
    pub fn svga_settile(info: *mut fb_info, map: *mut fb_tilemap);
    pub fn svga_tilecopy(info: *mut fb_info, area: *mut fb_tilearea);
    pub fn svga_tilefill(info: *mut fb_info, rect: *mut fb_tilerect);
    pub fn svga_tileblit(info: *mut fb_info, blit: *mut fb_tileblit);
    pub fn svga_tilecursor(regbase: *mut core::ffi::c_void, info: *mut fb_info, cursor: *mut fb_tilecursor);
    pub fn svga_get_tilemax(info: *mut fb_info) -> i32;
    pub fn svga_get_caps(info: *mut fb_info, caps: *mut fb_blit_caps, var: *mut fb_var_screeninfo);
    pub fn svga_compute_pll(pll: *const svga_pll, f_wanted: u32, m: *mut u16, n: *mut u16, r: *mut u16, node: i32) -> i32;
    pub fn svga_check_timings(tm: *const svga_timing_regs, var: *mut fb_var_screeninfo, node: i32) -> i32;
    pub fn svga_set_timings(regbase: *mut core::ffi::c_void, tm: *const svga_timing_regs, var: *mut fb_var_screeninfo, hmul: u32, hdiv: u32, vmul: u32, vdiv: u32, hborder: u32, node: i32);
    pub fn svga_match_format(frm: *const svga_fb_format, var: *mut fb_var_screeninfo, fix: *mut fb_fix_screeninfo) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
