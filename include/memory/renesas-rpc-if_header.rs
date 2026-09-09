/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Renesas RPC-IF core driver
 *
 * Copyright (C) 2018~2019 Renesas Solutions Corp.
 * Copyright (C) 2019 Macronix International Co., Ltd.
 * Copyright (C) 2019-2020 Cogent Embedded, Inc.
 */

// The Linux kernel headers included by the C header provide these types and
// the device declaration. They are expected to be supplied by the consumer.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum rpcif_data_dir {
    RPCIF_NO_DATA,
    RPCIF_DATA_IN,
    RPCIF_DATA_OUT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpcif_op_cmd {
    pub buswidth: u8,
    pub opcode: u8,
    pub ddr: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpcif_op_addr {
    pub nbytes: u8,
    pub buswidth: u8,
    pub ddr: bool,
    pub val: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpcif_op_dummy {
    pub ncycles: u8,
    pub buswidth: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpcif_op_option {
    pub nbytes: u8,
    pub buswidth: u8,
    pub ddr: bool,
    pub val: u32,
}

#[repr(C)]
pub union rpcif_op_data_buf {
    pub in_: *mut core::ffi::c_void,
    pub out: *const core::ffi::c_void,
}

#[repr(C)]
pub struct rpcif_op_data {
    pub buswidth: u8,
    pub nbytes: core::ffi::c_uint,
    pub dir: rpcif_data_dir,
    pub ddr: bool,
    pub buf: rpcif_op_data_buf,
}

#[repr(C)]
pub struct rpcif_op {
    pub cmd: rpcif_op_cmd,
    pub ocmd: rpcif_op_cmd,
    pub addr: rpcif_op_addr,
    pub dummy: rpcif_op_dummy,
    pub option: rpcif_op_option,
    pub data: rpcif_op_data,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum rpcif_type {
    RPCIF_RCAR_GEN3,
    RPCIF_RCAR_GEN4,
    RPCIF_RZ_G2L,
    XSPI_RZ_G3E,
}

// Supplied by the Linux kernel dependency.
pub struct device;

#[repr(C)]
pub struct rpcif {
    pub dev: *mut device,
    pub dirmap: *mut core::ffi::c_void,
    pub size: usize,
    pub xspi: bool,
}

extern "C" {
    pub fn rpcif_sw_init(rpc: *mut rpcif, dev: *mut device) -> core::ffi::c_int;
    pub fn rpcif_hw_init(dev: *mut device, hyperflash: bool) -> core::ffi::c_int;
    pub fn rpcif_prepare(
        dev: *mut device,
        op: *const rpcif_op,
        offs: *mut u64,
        len: *mut usize,
    );
    pub fn rpcif_manual_xfer(dev: *mut device) -> core::ffi::c_int;
    pub fn rpcif_dirmap_read(
        dev: *mut device,
        offs: u64,
        len: usize,
        buf: *mut core::ffi::c_void,
    ) -> isize;
    pub fn xspi_dirmap_write(
        dev: *mut device,
        offs: u64,
        len: usize,
        buf: *const core::ffi::c_void,
    ) -> isize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
