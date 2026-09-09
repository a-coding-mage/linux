/* SPDX-License-Identifier: GPL-2.0-or-later */

// TRACE_SYSTEM: fsi_master_aspeed
// The Linux tracepoint declarations included by the C header are represented
// below as C-layout event-entry types and their externally supplied emitters.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FsiMasterAspeedOpbRead {
    pub addr: u32,
    pub size: usize,
    pub result: u32,
    pub status: u32,
    pub irq_status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FsiMasterAspeedOpbWrite {
    pub addr: u32,
    pub val: u32,
    pub size: usize,
    pub status: u32,
    pub irq_status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FsiMasterAspeedOpbError {
    pub mresp0: u32,
    pub mstap0: u32,
    pub mesrb0: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FsiMasterAspeedCfamReset {
    pub start: bool,
}

extern "C" {
    pub fn trace_fsi_master_aspeed_opb_read(
        addr: u32,
        size: usize,
        result: u32,
        status: u32,
        irq_status: u32,
    );

    pub fn trace_fsi_master_aspeed_opb_write(
        addr: u32,
        val: u32,
        size: usize,
        status: u32,
        irq_status: u32,
    );

    pub fn trace_fsi_master_aspeed_opb_error(
        mresp0: u32,
        mstap0: u32,
        mesrb0: u32,
    );

    pub fn trace_fsi_master_aspeed_cfam_reset(start: bool);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
