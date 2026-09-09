// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright (C) 2010 Thomas Langer <thomas.langer@lantiq.com>
 * Copyright (C) 2010 John Crispin <john@phrozen.org>
 */

// Dependencies supplied by the Linux/MIPS and Lantiq headers are intentionally
// referenced here rather than implemented in this translation unit.

#[repr(C)]
pub struct ClkLookup {
    pub dev_id: *const core::ffi::c_char,
    pub con_id: *const core::ffi::c_char,
}

#[repr(C)]
pub struct Clk {
    pub rate: libc::c_ulong,
    pub get_rate: Option<unsafe extern "C" fn() -> libc::c_ulong>,
    pub rates: *mut libc::c_ulong,
    pub cl: ClkLookup,
    pub enable: Option<unsafe extern "C" fn(*mut Clk) -> libc::c_int>,
    pub disable: Option<unsafe extern "C" fn(*mut Clk)>,
    pub activate: Option<unsafe extern "C" fn(*mut Clk) -> libc::c_int>,
    pub deactivate: Option<unsafe extern "C" fn(*mut Clk)>,
}

extern "C" {
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn pr_err(fmt: *const core::ffi::c_char, ...) -> libc::c_int;
    fn pr_info(fmt: *const core::ffi::c_char, ...) -> libc::c_int;
    fn clk_put(clk: *mut Clk);
    fn ltq_soc_init();
    fn read_c0_count() -> u32;
    fn write_c0_compare(value: u32);
    static mut mips_hpt_frequency: libc::c_ulong;
}

// lantiq socs have 3 static clocks
static mut cpu_clk_generic: [Clk; 4] = unsafe { core::mem::zeroed() };

pub unsafe extern "C" fn clkdev_add_static(
    cpu: libc::c_ulong,
    fpi: libc::c_ulong,
    io: libc::c_ulong,
    ppe: libc::c_ulong,
) {
    cpu_clk_generic[0].rate = cpu;
    cpu_clk_generic[1].rate = fpi;
    cpu_clk_generic[2].rate = io;
    cpu_clk_generic[3].rate = ppe;
}

pub unsafe extern "C" fn clk_get_cpu() -> *mut Clk {
    &mut cpu_clk_generic[0]
}

pub unsafe extern "C" fn clk_get_fpi() -> *mut Clk {
    &mut cpu_clk_generic[1]
}

pub unsafe extern "C" fn clk_get_io() -> *mut Clk {
    &mut cpu_clk_generic[2]
}

pub unsafe extern "C" fn clk_get_ppe() -> *mut Clk {
    &mut cpu_clk_generic[3]
}

#[inline]
unsafe fn clk_good(clk: *mut Clk) -> bool {
    !clk.is_null() && !IS_ERR(clk.cast())
}

pub unsafe extern "C" fn clk_get_rate(clk: *mut Clk) -> libc::c_ulong {
    if !clk_good(clk) { return 0; }
    if (*clk).rate != 0 { return (*clk).rate; }
    if let Some(get_rate) = (*clk).get_rate { return get_rate(); }
    0
}

pub unsafe extern "C" fn clk_set_rate(clk: *mut Clk, rate: libc::c_ulong) -> libc::c_int {
    if !clk_good(clk) { return 0; }
    if !(*clk).rates.is_null() && *(*clk).rates != 0 {
        let mut r = (*clk).rates;
        while *r != 0 && *r != rate { r = r.add(1); }
        if *r == 0 {
            pr_err(b"clk %s.%s: trying to set invalid rate %ld\0".as_ptr().cast(),
                (*clk).cl.dev_id, (*clk).cl.con_id, rate);
            return -1;
        }
    }
    (*clk).rate = rate;
    0
}

pub unsafe extern "C" fn clk_round_rate(clk: *mut Clk, rate: libc::c_ulong) -> libc::c_long {
    if !clk_good(clk) { return 0; }
    if !(*clk).rates.is_null() && *(*clk).rates != 0 {
        let mut r = (*clk).rates;
        while *r != 0 && *r != rate { r = r.add(1); }
        if *r == 0 { return (*clk).rate as libc::c_long; }
    }
    rate as libc::c_long
}

pub unsafe extern "C" fn clk_enable(clk: *mut Clk) -> libc::c_int {
    if !clk_good(clk) { return -1; }
    if let Some(enable) = (*clk).enable { return enable(clk); }
    -1
}

pub unsafe extern "C" fn clk_disable(clk: *mut Clk) {
    if !clk_good(clk) { return; }
    if let Some(disable) = (*clk).disable { disable(clk); }
}

pub unsafe extern "C" fn clk_activate(clk: *mut Clk) -> libc::c_int {
    if !clk_good(clk) { return -1; }
    if let Some(activate) = (*clk).activate { return activate(clk); }
    -1
}

pub unsafe extern "C" fn clk_deactivate(clk: *mut Clk) {
    if !clk_good(clk) { return; }
    if let Some(deactivate) = (*clk).deactivate { deactivate(clk); }
}

pub unsafe extern "C" fn clk_get_parent(_clk: *mut Clk) -> *mut Clk { core::ptr::null_mut() }

pub unsafe extern "C" fn clk_set_parent(_clk: *mut Clk, _parent: *mut Clk) -> libc::c_int { 0 }

#[inline]
unsafe fn get_counter_resolution() -> u32 {
    let res: u32;
    core::arch::asm!(
        ".set push\n.set mips32r2\nrdhwr {0}, $3\n.set pop",
        out(reg) res,
        options(nostack, preserves_flags)
    );
    res
}

pub unsafe extern "C" fn plat_time_init() {
    ltq_soc_init();
    let clk = clk_get_cpu();
    mips_hpt_frequency = clk_get_rate(clk) / get_counter_resolution() as libc::c_ulong;
    write_c0_compare(read_c0_count());
    pr_info(b"CPU Clock: %ldMHz\n\0".as_ptr().cast(), clk_get_rate(clk) / 1_000_000);
    clk_put(clk);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
