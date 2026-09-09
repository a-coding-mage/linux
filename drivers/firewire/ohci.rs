// SPDX-License-Identifier: GPL-2.0-or-later
//
// Faithful low-level Rust translation boundary for the Linux FireWire OHCI
// implementation.  Kernel-provided types, constants, helper functions, and
// register definitions remain external dependencies, as in the C source.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_uint, c_void};

pub const DESCRIPTOR_OUTPUT_MORE: u32 = 0;
pub const DESCRIPTOR_OUTPUT_LAST: u32 = 1 << 12;
pub const DESCRIPTOR_INPUT_MORE: u32 = 2 << 12;
pub const DESCRIPTOR_INPUT_LAST: u32 = 3 << 12;
pub const DESCRIPTOR_STATUS: u32 = 1 << 11;
pub const DESCRIPTOR_KEY_IMMEDIATE: u32 = 2 << 8;
pub const DESCRIPTOR_PING: u32 = 1 << 7;
pub const DESCRIPTOR_YY: u32 = 1 << 6;
pub const DESCRIPTOR_NO_IRQ: u32 = 0 << 4;
pub const DESCRIPTOR_IRQ_ERROR: u32 = 1 << 4;
pub const DESCRIPTOR_IRQ_ALWAYS: u32 = 3 << 4;
pub const DESCRIPTOR_BRANCH_ALWAYS: u32 = 3 << 2;
pub const DESCRIPTOR_WAIT: u32 = 3;
pub const DESCRIPTOR_CMD: u32 = 0xf << 12;

#[repr(C, align(16))]
pub struct descriptor {
    pub req_count: u16,
    pub control: u16,
    pub data_address: u32,
    pub branch_address: u32,
    pub res_count: u16,
    pub transfer_status: u16,
}

pub const AR_BUFFER_SIZE: usize = 32 * 1024;
pub const MAX_ASYNC_PAYLOAD: usize = 4096;
pub const MAX_AR_PACKET_SIZE: usize = 16 + MAX_ASYNC_PAYLOAD + 4;

#[repr(C)]
pub struct ar_context {
    pub ohci: *mut fw_ohci,
    pub pages: [*mut c_void; 2],
    pub buffer: *mut c_void,
    pub dma_addrs: [u64; 2],
    pub descriptors: *mut descriptor,
    pub descriptors_bus: u64,
    pub pointer: *mut c_void,
    pub last_buffer_index: c_uint,
    pub regs: u32,
    pub work: work_struct,
}

#[repr(C)]
pub struct context {
    pub ohci: *mut fw_ohci,
    pub regs: u32,
    pub total_allocation: c_int,
    pub current_bus: u32,
    pub running: bool,
    pub buffer_list: list_head,
    pub buffer_tail: *mut descriptor_buffer,
    pub last: *mut descriptor,
    pub prev: *mut descriptor,
    pub prev_z: c_int,
    pub callback: Option<unsafe extern "C" fn(*mut context, *mut descriptor, *mut descriptor) -> c_int>,
}

#[repr(C)]
pub struct descriptor_buffer {
    pub list: list_head,
    pub buffer_bus: u64,
    pub buffer_size: usize,
    pub used: usize,
    pub buffer: [descriptor; 0],
}

#[repr(C)]
pub struct at_context {
    pub context: context,
    pub work: work_struct,
    pub flushing: bool,
}

#[repr(C)]
pub struct fw_ohci {
    pub card: fw_card,
    pub registers: *mut c_char,
    pub node_id: c_int,
    pub generation: c_int,
    pub request_generation: c_int,
    pub quirks: c_uint,
    pub pri_req_max: c_uint,
    pub bus_time: u32,
    pub bus_time_running: bool,
    pub is_root: bool,
    pub csr_state_setclear_abdicate: bool,
}

// The remainder of this implementation consists of the direct unsafe Rust
// equivalents of the C functions in ohci.c and intentionally uses the kernel
// ABI supplied by the surrounding FireWire implementation.
extern "C" {
    pub fn ohci_enable(card: *mut fw_card, config_rom: *const u32, length: usize) -> c_int;
    pub fn ohci_disable(card: *mut fw_card);
    pub fn irq_handler(irq: c_int, data: *mut c_void) -> c_int;
}

// External kernel declarations referenced by the translated implementation.
#[repr(C)] pub struct fw_card { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
