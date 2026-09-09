// SPDX-License-Identifier: GPL-2.0
/*
 * EMMA Mobile EV2 common clock framework support
 *
 * Copyright (C) 2013 Takashi Yoshii <takashi.yoshii.ze@renesas.com>
 * Copyright (C) 2012 Magnus Damm
 */

// Dependencies supplied by the Linux kernel and other translation units.
use core::ffi::{c_char, c_int, c_ulong, c_void};

// EMEV2 SMU registers
const USIAU0_RSTCTRL: usize = 0x094;
const USIBU1_RSTCTRL: usize = 0x0ac;
const USIBU2_RSTCTRL: usize = 0x0b0;
const USIBU3_RSTCTRL: usize = 0x0b4;
const IIC0_RSTCTRL: usize = 0x0dc;
const IIC1_RSTCTRL: usize = 0x0e0;
const STI_RSTCTRL: usize = 0x124;
const STI_CLKSEL: usize = 0x688;

const PAGE_SIZE: usize = 4096;

#[repr(C)]
pub struct Spinlock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const c_char,
}

extern "C" {
    static mut lock: Spinlock;
    fn writel_relaxed(value: c_ulong, addr: *mut c_void);
    fn of_find_matching_node(from: *mut DeviceNode, matches: *const OfDeviceId) -> *mut DeviceNode;
    fn of_iomap(np: *mut DeviceNode, index: c_int) -> *mut c_void;
    fn of_node_put(np: *mut DeviceNode);
    fn of_clk_get_parent_name(np: *mut DeviceNode, index: c_int) -> *const c_char;
    fn of_property_read_u32_array(
        np: *mut DeviceNode,
        propname: *const c_char,
        out_values: *mut u32,
        sz: usize,
    ) -> c_int;
    fn clk_register_divider(
        dev: *mut c_void,
        name: *const c_char,
        parent_name: *const c_char,
        flags: c_ulong,
        reg: *mut c_void,
        shift: u32,
        width: u8,
        clk_divider_flags: u8,
        lock: *mut Spinlock,
    ) -> *mut Clk;
    fn clk_register_gate(
        dev: *mut c_void,
        name: *const c_char,
        parent_name: *const c_char,
        flags: c_ulong,
        reg: *mut c_void,
        bit_idx: u8,
        clk_gate_flags: u8,
        lock: *mut Spinlock,
    ) -> *mut Clk;
    fn of_clk_add_provider(
        np: *mut DeviceNode,
        get: *const c_void,
        clk: *mut Clk,
    ) -> c_int;
    fn of_clk_src_simple_get(_: *mut DeviceNode, _: *const c_void) -> *mut Clk;
    fn pr_debug(fmt: *const c_char, ...);
}

// not pretty, but hey
static mut smu_base: *mut c_void = core::ptr::null_mut();

unsafe fn emev2_smu_write(value: c_ulong, offs: c_int) {
    if smu_base.is_null() || offs as usize >= PAGE_SIZE {
        panic!("BUG_ON");
    }
    writel_relaxed(value, (smu_base as *mut u8).add(offs as usize) as *mut c_void);
}

static smu_id: [OfDeviceId; 2] = [
    OfDeviceId { compatible: b"renesas,emev2-smu\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: core::ptr::null() },
];

unsafe fn emev2_smu_init() {
    let np: *mut DeviceNode;

    np = of_find_matching_node(core::ptr::null_mut(), smu_id.as_ptr());
    if np.is_null() {
        panic!("BUG_ON");
    }
    smu_base = of_iomap(np, 0);
    if smu_base.is_null() {
        panic!("BUG_ON");
    }
    of_node_put(np);

    /* setup STI timer to run on 32.768 kHz and deassert reset */
    emev2_smu_write(0, STI_CLKSEL as c_int);
    emev2_smu_write(1, STI_RSTCTRL as c_int);

    /* deassert reset for UART0->UART3 */
    emev2_smu_write(2, USIAU0_RSTCTRL as c_int);
    emev2_smu_write(2, USIBU1_RSTCTRL as c_int);
    emev2_smu_write(2, USIBU2_RSTCTRL as c_int);
    emev2_smu_write(2, USIBU3_RSTCTRL as c_int);

    /* deassert reset for IIC0->IIC1 */
    emev2_smu_write(1, IIC0_RSTCTRL as c_int);
    emev2_smu_write(1, IIC1_RSTCTRL as c_int);
}

unsafe fn emev2_smu_clkdiv_init(np: *mut DeviceNode) {
    let mut reg = [0u32; 2];
    let clk: *mut Clk;
    let parent_name = of_clk_get_parent_name(np, 0);
    if of_property_read_u32_array(np, b"reg\0".as_ptr() as *const c_char, reg.as_mut_ptr(), 2) != 0 {
        return;
    }
    if smu_base.is_null() {
        emev2_smu_init();
    }
    clk = clk_register_divider(
        core::ptr::null_mut(), core::ptr::null(), parent_name, 0,
        (smu_base as *mut u8).add(reg[0] as usize) as *mut c_void,
        reg[1], 8, 0, &raw mut lock,
    );
    of_clk_add_provider(np, of_clk_src_simple_get as *const c_void, clk);
    pr_debug(b"## %s %pOFn %p\0".as_ptr() as *const c_char);
}

// CLK_OF_DECLARE(emev2_smu_clkdiv, "renesas,emev2-smu-clkdiv", emev2_smu_clkdiv_init);

unsafe fn emev2_smu_gclk_init(np: *mut DeviceNode) {
    let mut reg = [0u32; 2];
    let clk: *mut Clk;
    let parent_name = of_clk_get_parent_name(np, 0);
    if of_property_read_u32_array(np, b"reg\0".as_ptr() as *const c_char, reg.as_mut_ptr(), 2) != 0 {
        return;
    }
    if smu_base.is_null() {
        emev2_smu_init();
    }
    clk = clk_register_gate(
        core::ptr::null_mut(), core::ptr::null(), parent_name, 0,
        (smu_base as *mut u8).add(reg[0] as usize) as *mut c_void,
        reg[1] as u8, 0, &raw mut lock,
    );
    of_clk_add_provider(np, of_clk_src_simple_get as *const c_void, clk);
    pr_debug(b"## %s %pOFn %p\0".as_ptr() as *const c_char);
}

// CLK_OF_DECLARE(emev2_smu_gclk, "renesas,emev2-smu-gclk", emev2_smu_gclk_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
