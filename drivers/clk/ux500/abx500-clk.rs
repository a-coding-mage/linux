// SPDX-License-Identifier: GPL-2.0-only
/*
 * abx500 clock implementation for ux500 platform.
 *
 * Copyright (C) 2012 ST-Ericsson SA
 * Author: Ulf Hansson <ulf.hansson@linaro.org>
 */

// Linux headers supplying the declarations used below are external dependencies.

const AB8500_NUM_CLKS: usize = 6;

extern "C" {
    type clk;
    type device;
    type device_node;
    type platform_device;
    type ab8500;
    type clk_onecell_data;
    type of_device_id;
    type platform_driver;

    fn ab8500_sysctrl_set(reg: u8, value: u8) -> i32;
    fn clk_reg_sysctrl_gate(
        dev: *mut device,
        name: *const u8,
        parent_name: *const u8,
        reg: u8,
        bit_idx: u8,
        mask: u8,
        flags: u32,
        clk_flags: u32,
    ) -> *mut clk;
    fn clk_reg_sysctrl_gate_fixed_rate(
        dev: *mut device,
        name: *const u8,
        parent_name: *const u8,
        reg: u8,
        bit_idx: u8,
        mask: u8,
        fixed_rate: u32,
        fixed_accuracy: u32,
        flags: u32,
    ) -> *mut clk;
    fn clk_reg_sysctrl_set_parent(
        dev: *mut device,
        name: *const u8,
        parents: *const *const u8,
        num_parents: u8,
        reg_sel: *const u16,
        reg_mask: *const u8,
        reg_bits: *const u8,
        flags: u32,
    ) -> *mut clk;
    fn dev_get_drvdata(dev: *mut device) -> *mut ab8500;
    fn is_ab8500(parent: *mut ab8500) -> bool;
    fn is_ab8505(parent: *mut ab8500) -> bool;
    fn of_clk_add_provider(
        np: *mut device_node,
        get: Option<unsafe extern "C" fn()>,
        data: *mut clk_onecell_data,
    ) -> i32;
    fn of_clk_src_onecell_get();
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn dev_info(dev: *mut device, fmt: *const u8, ...);
    fn dev_err(dev: *mut device, fmt: *const u8, ...);
}

// Constants supplied by the AB8500 and clock binding headers.
extern "C" {
    static AB8500_SWATCTRL: u8;
    static AB8500_SWATCTRL_SWATENABLE: u8;
    static AB8500_SYSULPCLKCTRL1: u8;
    static AB8500_SYSULPCLKCTRL1_SYSCLKBUF2REQ: u8;
    static AB8500_SYSULPCLKCTRL1_SYSCLKBUF3REQ: u8;
    static AB8500_SYSULPCLKCTRL1_SYSCLKBUF4REQ: u8;
    static AB8500_SYSULPCLKCTRL1_ULPCLKREQ: u8;
    static AB8500_SYSULPCLKCTRL1_AUDIOCLKENA: u8;
    static AB8500_SYSULPCLKCTRL1_SYSULPCLKINTSEL_MASK: u8;
    static AB8500_SYSULPCLKCTRL1_SYSULPCLKINTSEL_SHIFT: u8;
    static AB8500_SYSCLK_BUF2: usize;
    static AB8500_SYSCLK_BUF3: usize;
    static AB8500_SYSCLK_BUF4: usize;
    static AB8500_SYSCLK_ULP: usize;
    static AB8500_SYSCLK_INT: usize;
    static AB8500_SYSCLK_AUDIO: usize;
}

static mut ab8500_clks: [*mut clk; AB8500_NUM_CLKS] = [core::ptr::null_mut(); AB8500_NUM_CLKS];
static mut ab8500_clk_data: clk_onecell_data = unsafe { core::mem::zeroed() };

/* Clock definitions for ab8500 */
unsafe fn ab8500_reg_clks(dev: *mut device) -> i32 {
    let mut ret: i32;
    let mut clk: *mut clk;
    let np: *mut device_node = (*(dev as *mut DeviceRepr)).of_node;
    let intclk_parents: [*const u8; 2] = [b"ab8500_sysclk\0".as_ptr(), b"ulpclk\0".as_ptr()];
    let intclk_reg_sel: [u16; 2] = [0, AB8500_SYSULPCLKCTRL1 as u16];
    let intclk_reg_mask: [u8; 2] = [0, AB8500_SYSULPCLKCTRL1_SYSULPCLKINTSEL_MASK];
    let intclk_reg_bits: [u8; 2] = [0, 1u8 << AB8500_SYSULPCLKCTRL1_SYSULPCLKINTSEL_SHIFT];

    /* Enable SWAT */
    ret = ab8500_sysctrl_set(AB8500_SWATCTRL, AB8500_SWATCTRL_SWATENABLE);
    if ret != 0 { return ret; }

    /* ab8500_sysclk2 */
    clk = clk_reg_sysctrl_gate(dev, b"ab8500_sysclk2\0".as_ptr(), b"ab8500_sysclk\0".as_ptr(), AB8500_SYSULPCLKCTRL1, AB8500_SYSULPCLKCTRL1_SYSCLKBUF2REQ, AB8500_SYSULPCLKCTRL1_SYSCLKBUF2REQ, 0, 0);
    ab8500_clks[AB8500_SYSCLK_BUF2] = clk;
    /* ab8500_sysclk3 */
    clk = clk_reg_sysctrl_gate(dev, b"ab8500_sysclk3\0".as_ptr(), b"ab8500_sysclk\0".as_ptr(), AB8500_SYSULPCLKCTRL1, AB8500_SYSULPCLKCTRL1_SYSCLKBUF3REQ, AB8500_SYSULPCLKCTRL1_SYSCLKBUF3REQ, 0, 0);
    ab8500_clks[AB8500_SYSCLK_BUF3] = clk;
    /* ab8500_sysclk4 */
    clk = clk_reg_sysctrl_gate(dev, b"ab8500_sysclk4\0".as_ptr(), b"ab8500_sysclk\0".as_ptr(), AB8500_SYSULPCLKCTRL1, AB8500_SYSULPCLKCTRL1_SYSCLKBUF4REQ, AB8500_SYSULPCLKCTRL1_SYSCLKBUF4REQ, 0, 0);
    ab8500_clks[AB8500_SYSCLK_BUF4] = clk;
    /* ab_ulpclk */
    clk = clk_reg_sysctrl_gate_fixed_rate(dev, b"ulpclk\0".as_ptr(), core::ptr::null(), AB8500_SYSULPCLKCTRL1, AB8500_SYSULPCLKCTRL1_ULPCLKREQ, AB8500_SYSULPCLKCTRL1_ULPCLKREQ, 38400000, 9000, 0);
    ab8500_clks[AB8500_SYSCLK_ULP] = clk;
    /* ab8500_intclk */
    clk = clk_reg_sysctrl_set_parent(dev, b"intclk\0".as_ptr(), intclk_parents.as_ptr(), 2, intclk_reg_sel.as_ptr(), intclk_reg_mask.as_ptr(), intclk_reg_bits.as_ptr(), 0);
    ab8500_clks[AB8500_SYSCLK_INT] = clk;
    /* ab8500_audioclk */
    clk = clk_reg_sysctrl_gate(dev, b"audioclk\0".as_ptr(), b"intclk\0".as_ptr(), AB8500_SYSULPCLKCTRL1, AB8500_SYSULPCLKCTRL1_AUDIOCLKENA, AB8500_SYSULPCLKCTRL1_AUDIOCLKENA, 0, 0);
    ab8500_clks[AB8500_SYSCLK_AUDIO] = clk;

    (*(&mut ab8500_clk_data)).clks = ab8500_clks.as_mut_ptr();
    (*(&mut ab8500_clk_data)).clk_num = AB8500_NUM_CLKS;
    of_clk_add_provider(np, Some(core::mem::transmute(of_clk_src_onecell_get as unsafe extern "C" fn())), &mut ab8500_clk_data);
    dev_info(dev, b"registered clocks for ab850x\n\0".as_ptr());
    0
}

#[repr(C)] struct DeviceRepr { of_node: *mut device_node }

unsafe fn abx500_clk_probe(pdev: *mut platform_device) -> i32 {
    let pdev_repr = &*(pdev as *mut PlatformDeviceRepr);
    let dev = pdev_repr.dev;
    let parent = dev_get_drvdata(pdev_repr.parent);
    let ret: i32;
    if is_ab8500(parent) || is_ab8505(parent) {
        ret = ab8500_reg_clks(dev);
    } else {
        dev_err(dev, b"non supported plf id\n\0".as_ptr());
        return -19; // -ENODEV
    }
    ret
}

#[repr(C)]
struct OfDeviceId { compatible: *const u8 }

#[repr(C)]
struct PlatformDeviceRepr { dev: *mut device, parent: *mut device }

static abx500_clk_match: [OfDeviceId; 2] = [
    OfDeviceId { compatible: b"stericsson,ab8500-clk\0".as_ptr() },
    OfDeviceId { compatible: core::ptr::null() },
];

// The complete platform_driver layout is supplied by the Linux platform-driver dependency.
static mut abx500_clk_driver: platform_driver = unsafe { core::mem::zeroed() };

unsafe fn abx500_clk_init() -> i32 {
    platform_driver_register(&mut abx500_clk_driver)
}

// arch_initcall(abx500_clk_init);
// MODULE_AUTHOR("Ulf Hansson <ulf.hansson@linaro.org");
// MODULE_DESCRIPTION("ABX500 clk driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
