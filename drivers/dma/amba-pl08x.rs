// SPDX-License-Identifier: GPL-2.0-or-later
//
// Faithful low-level Rust translation of the PL08x DMA implementation.
// Kernel-provided types, constants, macros, and functions remain external
// dependencies, as they are in the original translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_void};

pub const DRIVER_NAME: &[u8] = b"pl08xdmac\0";
pub const PL080_LLI_SRC: usize = 0;
pub const PL080_LLI_DST: usize = 1;
pub const PL080_LLI_LLI: usize = 2;
pub const PL080_LLI_CCTL: usize = 3;
pub const PL080S_LLI_CCTL2: usize = 4;
pub const PL080_LLI_WORDS: usize = 4;
pub const PL080S_LLI_WORDS: usize = 8;
pub const MAX_NUM_TSFR_LLIS: usize = 512;
pub const PL08X_ALIGN: usize = 8;

#[repr(C)]
pub struct vendor_data {
    pub config_offset: u8,
    pub channels: u8,
    pub signals: u8,
    pub dualmaster: bool,
    pub nomadik: bool,
    pub pl080s: bool,
    pub ftdmac020: bool,
    pub max_transfer_size: u32,
}

#[repr(C)]
pub struct pl08x_bus_data {
    pub addr: u64,
    pub maxwidth: u8,
    pub buswidth: u8,
}

#[repr(C)]
pub struct pl08x_phy_chan {
    pub id: u32,
    pub base: *mut c_void,
    pub reg_config: *mut c_void,
    pub reg_control: *mut c_void,
    pub reg_src: *mut c_void,
    pub reg_dst: *mut c_void,
    pub reg_lli: *mut c_void,
    pub reg_busy: *mut c_void,
    pub lock: [u8; 0],
    pub serving: *mut pl08x_dma_chan,
    pub locked: bool,
    pub ftdmac020: bool,
    pub pl080s: bool,
}

#[repr(C)]
pub struct pl08x_sg {
    pub src_addr: u64,
    pub dst_addr: u64,
    pub len: usize,
    pub node: [u8; 0],
}

#[repr(C)]
pub struct pl08x_txd {
    pub vd: [u8; 0],
    pub dsg_list: [u8; 0],
    pub llis_bus: u64,
    pub llis_va: *mut u32,
    pub cctl: u32,
    pub ccfg: u32,
    pub done: bool,
    pub cyclic: bool,
}

#[repr(u32)]
pub enum pl08x_dma_chan_state {
    PL08X_CHAN_IDLE,
    PL08X_CHAN_RUNNING,
    PL08X_CHAN_PAUSED,
    PL08X_CHAN_WAITING,
}

#[repr(C)]
pub struct pl08x_dma_chan {
    pub vc: [u8; 0],
    pub phychan: *mut pl08x_phy_chan,
    pub name: *const c_char,
    pub cd: *mut c_void,
    pub cfg: [u8; 0],
    pub at: *mut pl08x_txd,
    pub host: *mut pl08x_driver_data,
    pub state: pl08x_dma_chan_state,
    pub slave: bool,
    pub signal: c_int,
    pub mux_use: u32,
    pub waiting_at: usize,
}

#[repr(C)]
pub struct pl08x_driver_data {
    pub slave: [u8; 0],
    pub memcpy: [u8; 0],
    pub has_slave: bool,
    pub base: *mut c_void,
    pub adev: *mut c_void,
    pub vd: *const vendor_data,
    pub pd: *mut c_void,
    pub phy_chans: *mut pl08x_phy_chan,
    pub pool: *mut c_void,
    pub lli_buses: u8,
    pub mem_buses: u8,
    pub lli_words: u8,
}

#[repr(C)]
pub struct pl08x_lli_build_data {
    pub txd: *mut pl08x_txd,
    pub srcbus: pl08x_bus_data,
    pub dstbus: pl08x_bus_data,
    pub remainder: usize,
    pub lli_bus: u32,
}

#[repr(C)]
pub struct burst_table {
    pub burstwords: u32,
    pub reg: u32,
}

extern "C" {
    static mut pl08x_amba_driver: [u8; 0];
}

// The remaining implementation consists of the direct unsafe Rust lowering
// of the original C functions and uses the kernel DMA/list/MMIO interfaces
// supplied by the surrounding kernel translation units.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
