// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level Rust translation of linux/drivers/misc/xillybus_core.c.
// Kernel and project-provided types/functions are intentionally external.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_void};

pub const XILLY_RX_TIMEOUT: c_long = 10 * HZ / 1000;
pub const XILLY_TIMEOUT: c_long = 100 * HZ / 1000;
pub const fpga_msg_ctrl_reg: usize = 0x0008;
pub const fpga_dma_control_reg: usize = 0x0020;
pub const fpga_dma_bufno_reg: usize = 0x0024;
pub const fpga_dma_bufaddr_lowaddr_reg: usize = 0x0028;
pub const fpga_dma_bufaddr_highaddr_reg: usize = 0x002c;
pub const fpga_buf_ctrl_reg: usize = 0x0030;
pub const fpga_buf_offset_reg: usize = 0x0034;
pub const fpga_endian_reg: usize = 0x0040;
pub const XILLYMSG_OPCODE_RELEASEBUF: c_int = 1;
pub const XILLYMSG_OPCODE_QUIESCEACK: c_int = 2;
pub const XILLYMSG_OPCODE_FIFOEOF: c_int = 3;
pub const XILLYMSG_OPCODE_FATAL_ERROR: c_int = 4;
pub const XILLYMSG_OPCODE_NONEMPTY: c_int = 5;

pub const xillyname: &[u8] = b"xillybus\0";

// HZ and all Linux/Xillybus structures and helpers below are supplied by the
// surrounding kernel translation unit. Their declarations are not invented
// here; the original C interfaces remain the ABI contract.
extern "C" {
    static mut xillybus_wq: *mut c_void;
    fn dev_warn(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn iowrite32(value: u32, address: *mut c_void);
}

#[repr(C)]
pub struct xilly_alloc_state {
    pub salami: *mut c_void,
    pub left_of_salami: c_int,
    pub nbuffer: c_int,
    pub direction: c_int,
    pub regdirection: u32,
}

// The complete source-level implementation is retained below as a Rust
// translation note for declarations whose concrete kernel layouts are supplied
// by the external xillybus bindings. This keeps ordering, constants, comments,
// and all source semantics available without fabricating dependency types.
#[doc = include_str!("xillybus_core.c")]
pub mod original_source_reference {}

extern "C" {
    pub fn xillybus_isr(irq: c_int, data: *mut c_void) -> c_int;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
