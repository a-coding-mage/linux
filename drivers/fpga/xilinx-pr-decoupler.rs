// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2017, National Instruments Corp.
 * Copyright (c) 2017, Xilinx Inc
 *
 * FPGA Bridge Driver for the Xilinx LogiCORE Partial Reconfiguration
 * Decoupler IP Core.
 */

// Linux kernel dependencies supplied by the surrounding build.

const CTRL_CMD_DECOUPLE: u32 = 1 << 0;
const CTRL_CMD_COUPLE: u32 = 0;
const CTRL_OFFSET: u32 = 0;

#[repr(C)]
pub struct xlnx_config_data {
    pub name: *const core::ffi::c_char,
}

#[repr(C)]
pub struct xlnx_pr_decoupler_data {
    pub ipconfig: *const xlnx_config_data,
    pub io_base: *mut core::ffi::c_void,
    pub clk: *mut clk,
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fpga_bridge {
    pub priv_: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fpga_bridge_ops {
    pub enable_set: Option<unsafe extern "C" fn(*mut fpga_bridge, bool) -> i32>,
    pub enable_show: Option<unsafe extern "C" fn(*mut fpga_bridge) -> i32>,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const core::ffi::c_char,
    pub data: *const core::ffi::c_void,
}

unsafe extern "C" {
    fn clk_enable(clk: *mut clk) -> i32;
    fn clk_disable(clk: *mut clk);
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn fpga_bridge_register(
        dev: *mut core::ffi::c_void,
        name: *const core::ffi::c_char,
        ops: *const fpga_bridge_ops,
        priv_: *mut xlnx_pr_decoupler_data,
    ) -> *mut fpga_bridge;
    fn fpga_bridge_unregister(bridge: *mut fpga_bridge);
    fn device_get_match_data(dev: *mut core::ffi::c_void) -> *const xlnx_config_data;
    fn devm_kzalloc(dev: *mut core::ffi::c_void, size: usize, flags: u32) -> *mut xlnx_pr_decoupler_data;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: u32) -> *mut core::ffi::c_void;
    fn devm_clk_get_prepared(dev: *mut core::ffi::c_void, name: *const core::ffi::c_char) -> *mut clk;
    fn dev_err_probe(dev: *mut core::ffi::c_void, err: i32, fmt: *const core::ffi::c_char) -> i32;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut fpga_bridge);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut fpga_bridge;
    fn dev_err(dev: *mut core::ffi::c_void, fmt: *const core::ffi::c_char, ...);
}

#[inline]
unsafe fn xlnx_pr_decoupler_write(d: *mut xlnx_pr_decoupler_data, offset: u32, val: u32) {
    writel(val, (*d).io_base.add(offset as usize));
}

#[inline]
unsafe fn xlnx_pr_decouple_read(d: *const xlnx_pr_decoupler_data, offset: u32) -> u32 {
    readl((*d).io_base.add(offset as usize))
}

unsafe extern "C" fn xlnx_pr_decoupler_enable_set(bridge: *mut fpga_bridge, enable: bool) -> i32 {
    let priv_ = (*bridge).priv_ as *mut xlnx_pr_decoupler_data;
    let err = clk_enable((*priv_).clk);
    if err != 0 {
        return err;
    }
    if enable {
        xlnx_pr_decoupler_write(priv_, CTRL_OFFSET, CTRL_CMD_COUPLE);
    } else {
        xlnx_pr_decoupler_write(priv_, CTRL_OFFSET, CTRL_CMD_DECOUPLE);
    }
    clk_disable((*priv_).clk);
    0
}

unsafe extern "C" fn xlnx_pr_decoupler_enable_show(bridge: *mut fpga_bridge) -> i32 {
    let priv_ = (*bridge).priv_ as *const xlnx_pr_decoupler_data;
    let err = clk_enable((*priv_).clk);
    if err != 0 {
        return err;
    }
    let status = xlnx_pr_decouple_read(priv_, CTRL_OFFSET);
    clk_disable((*priv_).clk);
    if status == 0 { 1 } else { 0 }
}

static xlnx_pr_decoupler_br_ops: fpga_bridge_ops = fpga_bridge_ops {
    enable_set: Some(xlnx_pr_decoupler_enable_set),
    enable_show: Some(xlnx_pr_decoupler_enable_show),
};

static decoupler_config: xlnx_config_data = xlnx_config_data {
    name: b"Xilinx PR Decoupler\0".as_ptr() as *const _,
};

static shutdown_config: xlnx_config_data = xlnx_config_data {
    name: b"Xilinx DFX AXI Shutdown Manager\0".as_ptr() as *const _,
};

static xlnx_pr_decoupler_of_match: [of_device_id; 5] = [
    of_device_id { compatible: b"xlnx,pr-decoupler-1.00\0".as_ptr() as *const _, data: &decoupler_config as *const _ as *const _ },
    of_device_id { compatible: b"xlnx,pr-decoupler\0".as_ptr() as *const _, data: &decoupler_config as *const _ as *const _ },
    of_device_id { compatible: b"xlnx,dfx-axi-shutdown-manager-1.00\0".as_ptr() as *const _, data: &shutdown_config as *const _ as *const _ },
    of_device_id { compatible: b"xlnx,dfx-axi-shutdown-manager\0".as_ptr() as *const _, data: &shutdown_config as *const _ as *const _ },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

unsafe extern "C" fn xlnx_pr_decoupler_probe(pdev: *mut platform_device) -> i32 {
    let priv_ = devm_kzalloc(pdev as *mut _, core::mem::size_of::<xlnx_pr_decoupler_data>(), 0);
    if priv_.is_null() { return -12; }
    (*priv_).ipconfig = device_get_match_data(pdev as *mut _);
    (*priv_).io_base = devm_platform_ioremap_resource(pdev, 0);
    if (*priv_).io_base as isize == -1 { return (*priv_).io_base as isize as i32; }
    (*priv_).clk = devm_clk_get_prepared(pdev as *mut _, b"aclk\0".as_ptr() as *const _);
    if (*priv_).clk as isize == -1 {
        return dev_err_probe(pdev as *mut _, (*priv_).clk as isize as i32, b"input clock not found\n\0".as_ptr() as *const _);
    }
    let br = fpga_bridge_register(pdev as *mut _, (*(*priv_).ipconfig).name, &xlnx_pr_decoupler_br_ops, priv_);
    if br as isize == -1 {
        let err = br as isize as i32;
        dev_err(pdev as *mut _, b"unable to register %s\0".as_ptr() as *const _, (*(*priv_).ipconfig).name);
        return err;
    }
    platform_set_drvdata(pdev, br);
    0
}

unsafe extern "C" fn xlnx_pr_decoupler_remove(pdev: *mut platform_device) {
    fpga_bridge_unregister(platform_get_drvdata(pdev));
}

// Equivalent of module_platform_driver(xlnx_pr_decoupler_driver) and module metadata.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
