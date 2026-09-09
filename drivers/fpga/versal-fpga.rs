// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019-2021 Xilinx, Inc.
 */

use core::ffi::{c_char, c_int, c_void};

// Dependencies supplied by the Linux kernel and other translation units.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct fpga_manager {
    pub dev: *mut device,
}
#[repr(C)]
pub struct fpga_image_info {
    _private: [u8; 0],
}
#[repr(C)]
pub struct fpga_manager_ops {
    pub write_init: Option<unsafe extern "C" fn(*mut fpga_manager, *mut fpga_image_info, *const c_char, usize) -> c_int>,
    pub write: Option<unsafe extern "C" fn(*mut fpga_manager, *const c_char, usize) -> c_int>,
}
#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}
#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}
#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: platform_driver_driver,
}

type DmaAddr = u64;

const GFP_KERNEL: u32 = 0x0000_00d0;
const PDI_SRC_DDR: u32 = 0;

extern "C" {
    fn dma_alloc_coherent(dev: *mut device, size: usize, dma_handle: *mut DmaAddr, flags: u32) -> *mut c_void;
    fn dma_free_coherent(dev: *mut device, size: usize, cpu_addr: *mut c_void, dma_handle: DmaAddr);
    fn memcpy(dest: *mut c_void, src: *const c_void, count: usize) -> *mut c_void;
    fn zynqmp_pm_load_pdi(src: u32, address: DmaAddr) -> c_int;
    fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn devm_fpga_mgr_register(dev: *mut device, name: *const c_char, ops: *const fpga_manager_ops, priv_data: *mut c_void) -> *mut fpga_manager;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn ptr_err_or_zero<T>(ptr: *mut T) -> c_int;
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
}

unsafe extern "C" fn versal_fpga_ops_write_init(
    _mgr: *mut fpga_manager,
    _info: *mut fpga_image_info,
    _buf: *const c_char,
    _size: usize,
) -> c_int {
    0
}

unsafe extern "C" fn versal_fpga_ops_write(
    mgr: *mut fpga_manager,
    buf: *const c_char,
    size: usize,
) -> c_int {
    let mut dma_addr: DmaAddr = 0;
    let kbuf: *mut c_char;
    let ret: c_int;

    kbuf = dma_alloc_coherent((*mgr).dev, size, &mut dma_addr, GFP_KERNEL) as *mut c_char;
    if kbuf.is_null() {
        return -12; // -ENOMEM
    }

    memcpy(kbuf as *mut c_void, buf as *const c_void, size);
    ret = zynqmp_pm_load_pdi(PDI_SRC_DDR, dma_addr);
    dma_free_coherent((*mgr).dev, size, kbuf as *mut c_void, dma_addr);

    ret
}

static VERSAL_FPGA_OPS: fpga_manager_ops = fpga_manager_ops {
    write_init: Some(versal_fpga_ops_write_init),
    write: Some(versal_fpga_ops_write),
};

unsafe extern "C" fn versal_fpga_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let mgr: *mut fpga_manager;
    let ret: c_int;

    ret = dma_set_mask_and_coherent(&mut (*pdev).dev, (1u64 << 44) - 1);
    if ret < 0 {
        dev_err(dev, b"no usable DMA configuration\0".as_ptr() as *const c_char);
        return ret;
    }

    mgr = devm_fpga_mgr_register(
        dev,
        b"Xilinx Versal FPGA Manager\0".as_ptr() as *const c_char,
        &VERSAL_FPGA_OPS,
        core::ptr::null_mut(),
    );
    ptr_err_or_zero(mgr)
}

static VERSAL_FPGA_OF_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: b"xlnx,versal-fpga\0".as_ptr() as *const c_char },
    of_device_id { compatible: core::ptr::null() },
];

static mut VERSAL_FPGA_DRIVER: platform_driver = platform_driver {
    probe: Some(versal_fpga_probe),
    driver: platform_driver_driver {
        name: b"versal_fpga_manager\0".as_ptr() as *const c_char,
        of_match_table: VERSAL_FPGA_OF_MATCH.as_ptr(),
    },
};

// MODULE_DEVICE_TABLE(of, versal_fpga_of_match);
// module_platform_driver(versal_fpga_driver);
// MODULE_AUTHOR("Nava kishore Manne <nava.manne@xilinx.com>");
// MODULE_AUTHOR("Appana Durga Kedareswara rao <appanad.durga.rao@xilinx.com>");
// MODULE_DESCRIPTION("Xilinx Versal FPGA Manager");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
