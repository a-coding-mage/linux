// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2019 Xilinx, Inc.
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

/* Constant Definitions */
const IXR_FPGA_DONE_MASK: u32 = 1u32 << 3;

/**
 * struct zynqmp_fpga_priv - Private data structure
 * @dev:        Device data structure
 * @flags:      flags which is used to identify the bitfile type
 */
#[repr(C)]
pub struct zynqmp_fpga_priv {
    pub dev: *mut device,
    pub flags: u32,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fpga_manager {
    pub priv_: *mut zynqmp_fpga_priv,
}

#[repr(C)]
pub struct fpga_image_info {
    pub flags: u32,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct device_attribute {
    _private: [u8; 0],
}

#[repr(C)]
pub struct attribute {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fpga_manager_ops {
    pub state: Option<unsafe extern "C" fn(*mut fpga_manager) -> fpga_mgr_states>,
    pub write_init: Option<unsafe extern "C" fn(*mut fpga_manager, *mut fpga_image_info, *const i8, usize) -> i32>,
    pub write: Option<unsafe extern "C" fn(*mut fpga_manager, *const i8, usize) -> i32>,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const u8,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum fpga_mgr_states {
    FPGA_MGR_STATE_UNKNOWN,
    FPGA_MGR_STATE_OPERATING,
}

type dma_addr_t = usize;
type ssize_t = isize;

const GFP_KERNEL: u32 = 0;
const FPGA_MGR_PARTIAL_RECONFIG: u32 = 1;
const XILINX_ZYNQMP_PM_FPGA_PARTIAL: u32 = 1;

extern "C" {
    fn dma_alloc_coherent(dev: *mut device, size: usize, dma_handle: *mut dma_addr_t, flags: u32) -> *mut i8;
    fn dma_free_coherent(dev: *mut device, size: usize, cpu_addr: *mut i8, dma_handle: dma_addr_t);
    fn memcpy(dest: *mut i8, src: *const i8, n: usize) -> *mut i8;
    fn wmb();
    fn zynqmp_pm_fpga_load(dma_addr: dma_addr_t, size: usize, flags: u32) -> i32;
    fn zynqmp_pm_fpga_get_status(status: *mut u32) -> i32;
    fn zynqmp_pm_fpga_get_config_status(status: *mut u32) -> i32;
    fn sysfs_emit(buf: *mut i8, fmt: *const i8, ...) -> ssize_t;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut zynqmp_fpga_priv;
    fn devm_fpga_mgr_register(dev: *mut device, name: *const i8, ops: *const fpga_manager_ops, priv_: *mut zynqmp_fpga_priv) -> *mut fpga_manager;
    fn ptr_err_or_zero(ptr: *mut fpga_manager) -> i32;
}

unsafe extern "C" fn zynqmp_fpga_ops_write_init(
    mgr: *mut fpga_manager,
    info: *mut fpga_image_info,
    _buf: *const i8,
    _size: usize,
) -> i32 {
    let priv_ = (*mgr).priv_;
    (*priv_).flags = (*info).flags;
    0
}

unsafe extern "C" fn zynqmp_fpga_ops_write(
    mgr: *mut fpga_manager,
    buf: *const i8,
    size: usize,
) -> i32 {
    let priv_ = (*mgr).priv_;
    let mut dma_addr: dma_addr_t = 0;
    let mut eemi_flags: u32 = 0;
    let kbuf = dma_alloc_coherent((*priv_).dev, size, &mut dma_addr, GFP_KERNEL);
    if kbuf.is_null() {
        return -12;
    }
    memcpy(kbuf, buf, size);
    wmb(); /* ensure all writes are done before initiate FW call */
    if (*priv_).flags & FPGA_MGR_PARTIAL_RECONFIG != 0 {
        eemi_flags |= XILINX_ZYNQMP_PM_FPGA_PARTIAL;
    }
    let ret = zynqmp_pm_fpga_load(dma_addr, size, eemi_flags);
    dma_free_coherent((*priv_).dev, size, kbuf, dma_addr);
    ret
}

unsafe extern "C" fn zynqmp_fpga_ops_state(mgr: *mut fpga_manager) -> fpga_mgr_states {
    let _ = mgr;
    let mut status: u32 = 0;
    zynqmp_pm_fpga_get_status(&mut status);
    if status & IXR_FPGA_DONE_MASK != 0 {
        return fpga_mgr_states::FPGA_MGR_STATE_OPERATING;
    }
    fpga_mgr_states::FPGA_MGR_STATE_UNKNOWN
}

unsafe extern "C" fn status_show(
    dev: *mut device,
    attr: *mut device_attribute,
    buf: *mut i8,
) -> ssize_t {
    let _ = (dev, attr);
    let mut status: u32 = 0;
    let ret = zynqmp_pm_fpga_get_config_status(&mut status);
    if ret != 0 {
        return ret as ssize_t;
    }
    sysfs_emit(buf, b"0x%x\n\0".as_ptr() as *const i8, status)
}

// static DEVICE_ATTR_RO(status);
static mut zynqmp_fpga_attrs: [*mut attribute; 2] = [core::ptr::null_mut(), core::ptr::null_mut()];
// ATTRIBUTE_GROUPS(zynqmp_fpga);

static zynqmp_fpga_ops: fpga_manager_ops = fpga_manager_ops {
    state: Some(zynqmp_fpga_ops_state),
    write_init: Some(zynqmp_fpga_ops_write_init),
    write: Some(zynqmp_fpga_ops_write),
};

unsafe extern "C" fn zynqmp_fpga_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let priv_ = devm_kzalloc(dev, core::mem::size_of::<zynqmp_fpga_priv>(), GFP_KERNEL);
    if priv_.is_null() {
        return -12;
    }
    (*priv_).dev = dev;
    let mgr = devm_fpga_mgr_register(
        dev,
        b"Xilinx ZynqMP FPGA Manager\0".as_ptr() as *const i8,
        &zynqmp_fpga_ops,
        priv_,
    );
    ptr_err_or_zero(mgr)
}

#[cfg(CONFIG_OF)]
static zynqmp_fpga_of_match: [of_device_id; 2] = [
    of_device_id { compatible: b"xlnx,zynqmp-pcap-fpga\0".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];
// MODULE_DEVICE_TABLE(of, zynqmp_fpga_of_match);

static mut zynqmp_fpga_driver: platform_driver = platform_driver {
    probe: Some(zynqmp_fpga_probe),
};

// module_platform_driver(zynqmp_fpga_driver);
// MODULE_AUTHOR("Nava kishore Manne <navam@xilinx.com>");
// MODULE_DESCRIPTION("Xilinx ZynqMp FPGA Manager");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
