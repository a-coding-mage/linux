/* SPDX-License-Identifier: GPL-2.0+ */
/* Copyright 2023 NXP. */

//! Rust translation of `fsl-edma-trace.h`.
//!
//! The Linux tracepoint declaration machinery used by the original header is
//! represented here by its C-layout event payloads and trace event names.

#[allow(non_camel_case_types)]
pub type u32_t = u32;

#[allow(non_camel_case_types)]
pub type u16_t = u16;

#[allow(non_camel_case_types)]
pub type u64_t = u64;

/// Opaque declaration supplied by the eDMA implementation.
#[repr(C)]
pub struct fsl_edma_engine {
    _private: [u8; 0],
}

/// Opaque declaration supplied by the eDMA implementation.
#[repr(C)]
pub struct fsl_edma_chan {
    _private: [u8; 0],
}

/// Payload corresponding to the `edma_log_io` trace event class.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edma_log_io_entry {
    pub edma: *mut fsl_edma_engine,
    pub addr: *mut core::ffi::c_void,
    pub value: u32,
    pub membase: *mut core::ffi::c_void,
}

/// Event names instantiated from `edma_log_io`.
pub const EDMA_READL: &str = "edma_readl";
pub const EDMA_WRITEL: &str = "edma_writel";
pub const EDMA_READW: &str = "edma_readw";
pub const EDMA_WRITEW: &str = "edma_writew";
pub const EDMA_READB: &str = "edma_readb";
pub const EDMA_WRITEB: &str = "edma_writeb";

/// Payload corresponding to the `edma_log_tcd` trace event class.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edma_log_tcd_entry {
    pub saddr: u64,
    pub soff: u16,
    pub attr: u16,
    pub nbytes: u32,
    pub slast: u64,
    pub daddr: u64,
    pub doff: u16,
    pub citer: u16,
    pub dlast_sga: u64,
    pub csr: u16,
    pub biter: u16,
}

pub const EDMA_FILL_TCD: &str = "edma_fill_tcd";

/// Equivalent to the `edma_log_io` tracepoint's formatted output.
pub unsafe fn edma_log_io_print(entry: &edma_log_io_entry) -> ([u8; 64], usize) {
    let offset = (entry.addr as usize).wrapping_sub(entry.membase as usize) as u32;
    let text = format!("offset {:08x}: value {:08x}", offset, entry.value);
    let bytes = text.as_bytes();
    let mut out = [0u8; 64];
    let len = core::cmp::min(bytes.len(), out.len());
    out[..len].copy_from_slice(&bytes[..len]);
    (out, len)
}

/// Equivalent to the `edma_log_tcd` tracepoint's formatted output.
pub fn edma_log_tcd_print(entry: &edma_log_tcd_entry) -> String {
    format!(
        "\n==== TCD =====\n  saddr:  0x{:016x}\n  soff:               0x{:04x}\n  attr:               0x{:04x}\n  nbytes:         0x{:08x}\n  slast:  0x{:016x}\n  daddr:  0x{:016x}\n  doff:               0x{:04x}\n  citer:              0x{:04x}\n  dlast:  0x{:016x}\n  csr:                0x{:04x}\n  biter:              0x{:04x}\n",
        entry.saddr, entry.soff, entry.attr, entry.nbytes, entry.slast,
        entry.daddr, entry.doff, entry.citer, entry.dlast_sga, entry.csr,
        entry.biter
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
