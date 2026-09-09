// SPDX-License-Identifier: MIT
/*
 * clock framework for AMD FCH controller block
 *
 * Copyright 2018 Advanced Micro Devices, Inc.
 */

// External Linux kernel declarations supplied by the surrounding translation.

/* Clock Driving Strength 2 register */
const CLKDRVSTR2: usize = 0x28;
/* Clock Control 1 register */
const MISCCLKCNTL1: usize = 0x40;
/* Auxiliary clock1 enable bit */
const OSCCLKENB: u8 = 2;
/* 25Mhz auxiliary output clock freq bit */
const OSCOUT1CLK25MHZ: u8 = 16;

const ST_CLK_48M: usize = 0;
const ST_CLK_25M: usize = 1;
const ST_CLK_MUX: usize = 2;
const ST_CLK_GATE: usize = 3;
const ST_MAX_CLKS: usize = 4;

const CLK_48M_FIXED: usize = 0;
const CLK_GATE_FIXED: usize = 1;
const CLK_MAX_FIXED: usize = 2;

/* List of supported CPU ids for clk mux with 25Mhz clk support */
const AMD_CPU_ID_ST: u16 = 0x1576;

static CLK_OSCOUT1_PARENTS: [&'static str; 2] = ["clk48MHz", "clk25MHz"];
static mut HWS: [*mut clk_hw; ST_MAX_CLKS] = [core::ptr::null_mut(); ST_MAX_CLKS];

// The following types and functions are provided by the Linux kernel bindings.
#[repr(C)]
pub struct clk_hw {
    pub clk: *mut clk,
}
pub struct clk;
pub struct platform_device;
pub struct pci_dev;
pub struct fch_clk_data {
    pub base: *mut u8,
    pub name: *const core::ffi::c_char,
}

extern "C" {
    fn dev_get_platdata(dev: *mut core::ffi::c_void) -> *mut fch_clk_data;
    fn pci_get_domain_bus_and_slot(domain: u32, bus: u32, devfn: u32) -> *mut pci_dev;
    fn pci_match_id(ids: *const pci_device_id, dev: *mut pci_dev) -> bool;
    fn pci_dev_put(dev: *mut pci_dev);
    fn clk_hw_register_fixed_rate(
        dev: *mut core::ffi::c_void,
        name: *const core::ffi::c_char,
        parent_name: *const core::ffi::c_char,
        flags: u32,
        rate: u64,
    ) -> *mut clk_hw;
    fn clk_hw_register_mux(
        dev: *mut core::ffi::c_void,
        name: *const core::ffi::c_char,
        parent_names: *const *const core::ffi::c_char,
        num_parents: u32,
        flags: u32,
        reg: *mut u8,
        shift: u8,
        width: u8,
        clk_gate_flags: u8,
        lock: *mut core::ffi::c_void,
    ) -> *mut clk_hw;
    fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> i32;
    fn clk_hw_register_gate(
        dev: *mut core::ffi::c_void,
        name: *const core::ffi::c_char,
        parent_name: *const core::ffi::c_char,
        flags: u32,
        reg: *mut u8,
        bit_idx: u8,
        clk_gate_flags: u8,
        lock: *mut core::ffi::c_void,
    ) -> *mut clk_hw;
    fn devm_clk_hw_register_clkdev(
        dev: *mut core::ffi::c_void,
        hw: *mut clk_hw,
        con_id: *const core::ffi::c_char,
        dev_id: *const core::ffi::c_char,
    ) -> i32;
    fn clk_hw_unregister(hw: *mut clk_hw);
}

#[repr(C)]
pub struct pci_device_id {
    pub vendor: u32,
    pub device: u32,
}

static FCH_PCI_IDS: [pci_device_id; 2] = [
    pci_device_id { vendor: 0x1022, device: AMD_CPU_ID_ST as u32 },
    pci_device_id { vendor: 0, device: 0 },
];

unsafe fn fch_clk_probe(pdev: *mut platform_device) -> i32 {
    let fch_data: *mut fch_clk_data;
    let rdev: *mut pci_dev;

    fch_data = dev_get_platdata(pdev as *mut core::ffi::c_void);
    if fch_data.is_null() || (*fch_data).base.is_null() {
        return -22;
    }

    rdev = pci_get_domain_bus_and_slot(0, 0, 0);
    if rdev.is_null() {
        return -19;
    }

    if pci_match_id(FCH_PCI_IDS.as_ptr(), rdev) {
        HWS[ST_CLK_48M] = clk_hw_register_fixed_rate(core::ptr::null_mut(), b"clk48MHz\0".as_ptr() as _, core::ptr::null(), 0, 48000000);
        HWS[ST_CLK_25M] = clk_hw_register_fixed_rate(core::ptr::null_mut(), b"clk25MHz\0".as_ptr() as _, core::ptr::null(), 0, 25000000);
        HWS[ST_CLK_MUX] = clk_hw_register_mux(core::ptr::null_mut(), b"oscout1_mux\0".as_ptr() as _, core::ptr::null(), 2, 0, (*fch_data).base.add(CLKDRVSTR2), OSCOUT1CLK25MHZ, 3, 0, core::ptr::null_mut());
        clk_set_parent((*HWS[ST_CLK_MUX]).clk, (*HWS[ST_CLK_48M]).clk);
        HWS[ST_CLK_GATE] = clk_hw_register_gate(core::ptr::null_mut(), b"oscout1\0".as_ptr() as _, b"oscout1_mux\0".as_ptr() as _, 0, (*fch_data).base.add(MISCCLKCNTL1), OSCCLKENB, 1, core::ptr::null_mut());
        devm_clk_hw_register_clkdev(pdev as *mut _, HWS[ST_CLK_GATE], (*fch_data).name, core::ptr::null());
    } else {
        HWS[CLK_48M_FIXED] = clk_hw_register_fixed_rate(core::ptr::null_mut(), b"clk48MHz\0".as_ptr() as _, core::ptr::null(), 0, 48000000);
        HWS[CLK_GATE_FIXED] = clk_hw_register_gate(core::ptr::null_mut(), b"oscout1\0".as_ptr() as _, b"clk48MHz\0".as_ptr() as _, 0, (*fch_data).base.add(MISCCLKCNTL1), OSCCLKENB, 0, core::ptr::null_mut());
        devm_clk_hw_register_clkdev(pdev as *mut _, HWS[CLK_GATE_FIXED], (*fch_data).name, core::ptr::null());
    }
    pci_dev_put(rdev);
    0
}

unsafe fn fch_clk_remove(_pdev: *mut platform_device) {
    let rdev = pci_get_domain_bus_and_slot(0, 0, 0);
    if rdev.is_null() { return; }
    let clks = if pci_match_id(FCH_PCI_IDS.as_ptr(), rdev) { CLK_MAX_FIXED } else { ST_MAX_CLKS };
    for i in 0..clks { clk_hw_unregister(HWS[i]); }
    pci_dev_put(rdev);
}

// Equivalent platform-driver registration is supplied by the kernel build system.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
