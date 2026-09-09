// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2017 HiSilicon Limited, All Rights Reserved.
 * Author: Gabriele Paoloni <gabriele.paoloni@huawei.com>
 * Author: Zhichang Yuan <yuanzhichang@hisilicon.com>
 */

// Dependency supplied externally: linux/fwnode.h

#[repr(C)]
#[derive(Copy, Clone)]
pub enum LogicPioFlag {
    LOGIC_PIO_INDIRECT,
    LOGIC_PIO_CPU_MMIO,
}

// Dependency-supplied C types.
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

pub type resource_size_t = usize;

#[repr(C)]
pub struct logic_pio_hwaddr {
    pub list: list_head,
    pub fwnode: *const fwnode_handle,
    pub hw_start: resource_size_t,
    pub io_start: resource_size_t,
    pub size: resource_size_t, // range size populated
    pub flags: usize,
    pub hostdata: *mut core::ffi::c_void,
    pub ops: *const logic_pio_host_ops,
}

#[repr(C)]
pub struct logic_pio_host_ops {
    pub in_: Option<unsafe extern "C" fn(
        hostdata: *mut core::ffi::c_void,
        addr: usize,
        dwidth: usize,
    ) -> u32>,
    pub out: Option<unsafe extern "C" fn(
        hostdata: *mut core::ffi::c_void,
        addr: usize,
        val: u32,
        dwidth: usize,
    )>,
    pub ins: Option<unsafe extern "C" fn(
        hostdata: *mut core::ffi::c_void,
        addr: usize,
        buffer: *mut core::ffi::c_void,
        dwidth: usize,
        count: u32,
    ) -> u32>,
    pub outs: Option<unsafe extern "C" fn(
        hostdata: *mut core::ffi::c_void,
        addr: usize,
        buffer: *const core::ffi::c_void,
        dwidth: usize,
        count: u32,
    )>,
}

// The following declarations and aliases are present only when CONFIG_INDIRECT_PIO
// is enabled in the C build configuration.
#[cfg(feature = "CONFIG_INDIRECT_PIO")]
extern "C" {
    pub fn logic_inb(addr: usize) -> u8;
    pub fn logic_inw(addr: usize) -> u16;
    pub fn logic_inl(addr: usize) -> u32;
    pub fn logic_outb(value: u8, addr: usize);
    pub fn logic_outw(value: u16, addr: usize);
    pub fn logic_outl(value: u32, addr: usize);
    pub fn logic_insb(addr: usize, buffer: *mut core::ffi::c_void, count: u32);
    pub fn logic_insl(addr: usize, buffer: *mut core::ffi::c_void, count: u32);
    pub fn logic_insw(addr: usize, buffer: *mut core::ffi::c_void, count: u32);
    pub fn logic_outsb(addr: usize, buffer: *const core::ffi::c_void, count: u32);
    pub fn logic_outsw(addr: usize, buffer: *const core::ffi::c_void, count: u32);
    pub fn logic_outsl(addr: usize, buffer: *const core::ffi::c_void, count: u32);
}

// C preprocessor aliases (inb/inw/inl/outb/outw/outl/insb/insw/insl/
// outsb/outsw/outsl) map to the corresponding logic_* functions when enabled.

// We reserve 0x4000 bytes for Indirect IO as so far this library is only
// used by the HiSilicon LPC Host. If needed, we can reserve a wider IO
// area by redefining the macro below.
#[cfg(feature = "CONFIG_INDIRECT_PIO")]
pub const PIO_INDIRECT_SIZE: usize = 0x4000;
#[cfg(not(feature = "CONFIG_INDIRECT_PIO"))]
pub const PIO_INDIRECT_SIZE: usize = 0;

// IO_SPACE_LIMIT is supplied by the target environment.
pub const MMIO_UPPER_LIMIT: usize = IO_SPACE_LIMIT - PIO_INDIRECT_SIZE;

extern "C" {
    pub fn find_io_range_by_fwnode(
        fwnode: *const fwnode_handle,
    ) -> *mut logic_pio_hwaddr;
    pub fn logic_pio_trans_hwaddr(
        fwnode: *const fwnode_handle,
        hw_addr: resource_size_t,
        size: resource_size_t,
    ) -> usize;
    pub fn logic_pio_register_range(newrange: *mut logic_pio_hwaddr) -> i32;
    pub fn logic_pio_unregister_range(range: *mut logic_pio_hwaddr);
    pub fn logic_pio_to_hwaddr(pio: usize) -> resource_size_t;
    pub fn logic_pio_trans_cpuaddr(hw_addr: resource_size_t) -> usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
