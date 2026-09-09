/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2016 Cavium, Inc.
 */

// C dependencies: asm/byteorder.h, linux/delay.h, linux/pci.h, cpt_hw_types.h

/* Device ID */
pub const CPT_81XX_PCI_PF_DEVICE_ID: u32 = 0xa040;
pub const CPT_81XX_PCI_VF_DEVICE_ID: u32 = 0xa041;

/* flags to indicate the features supported */
pub const CPT_FLAG_SRIOV_ENABLED: u64 = 1 << 1;
pub const CPT_FLAG_VF_DRIVER: u64 = 1 << 2;
pub const CPT_FLAG_DEVICE_READY: u64 = 1 << 3;

macro_rules! cpt_sriov_enabled { ($cpt:expr) => { (($cpt).flags & CPT_FLAG_SRIOV_ENABLED) }; }
macro_rules! cpt_vf_driver { ($cpt:expr) => { (($cpt).flags & CPT_FLAG_VF_DRIVER) }; }
macro_rules! cpt_device_ready { ($cpt:expr) => { (($cpt).flags & CPT_FLAG_DEVICE_READY) }; }

pub const CPT_MBOX_MSG_TYPE_ACK: u32 = 1;
pub const CPT_MBOX_MSG_TYPE_NACK: u32 = 2;
pub const CPT_MBOX_MSG_TIMEOUT: u32 = 2000;
pub const VF_STATE_DOWN: u32 = 0;
pub const VF_STATE_UP: u32 = 1;

macro_rules! cpt_pf_reg1 { ($name:ident, $base:expr) => { macro_rules! $name { ($a:expr) => { (($base as u64) + (($a as u64) << 36)) }; } }; }
cpt_pf_reg1!(CPTX_PF_CONSTANTS, 0x0);
cpt_pf_reg1!(CPTX_PF_RESET, 0x100);
cpt_pf_reg1!(CPTX_PF_DIAG, 0x120);
cpt_pf_reg1!(CPTX_PF_BIST_STATUS, 0x160);
cpt_pf_reg1!(CPTX_PF_ECC0_CTL, 0x200);
cpt_pf_reg1!(CPTX_PF_ECC0_FLIP, 0x210);
cpt_pf_reg1!(CPTX_PF_ECC0_INT, 0x220);
cpt_pf_reg1!(CPTX_PF_ECC0_INT_W1S, 0x230);
cpt_pf_reg1!(CPTX_PF_ECC0_ENA_W1S, 0x240);
cpt_pf_reg1!(CPTX_PF_ECC0_ENA_W1C, 0x250);

macro_rules! cpt_pf_reg2 { ($name:ident, $base:expr) => { macro_rules! $name { ($a:expr, $b:expr) => { (($base as u64) + (($a as u64) << 36) + (($b as u64) << 3)) }; } }; }
cpt_pf_reg2!(CPTX_PF_MBOX_INTX, 0x400);
cpt_pf_reg2!(CPTX_PF_MBOX_INT_W1SX, 0x420);
cpt_pf_reg2!(CPTX_PF_MBOX_ENA_W1CX, 0x440);
cpt_pf_reg2!(CPTX_PF_MBOX_ENA_W1SX, 0x460);
cpt_pf_reg2!(CPTX_PF_GX_EN, 0x600);
cpt_pf_reg2!(CPTX_PF_EXE_DBG_CNTX, 0x4001100);
cpt_pf_reg2!(CPTX_PF_EXE_EPCI_INBX_CNT, 0x4001200);
cpt_pf_reg2!(CPTX_PF_EXE_EPCI_OUTBX_CNT, 0x4001240);
cpt_pf_reg2!(CPTX_PF_ENGX_UCODE_BASE, 0x4002000);

macro_rules! cpt_pf_reg1_rest { ($name:ident, $base:expr) => { macro_rules! $name { ($a:expr) => { (($base as u64) + (($a as u64) << 36)) }; } }; }
cpt_pf_reg1_rest!(CPTX_PF_EXEC_INT_W1S, 0x520);
cpt_pf_reg1_rest!(CPTX_PF_EXEC_ENA_W1C, 0x540);
cpt_pf_reg1_rest!(CPTX_PF_EXEC_ENA_W1S, 0x560);
cpt_pf_reg1_rest!(CPTX_PF_EXEC_INFO, 0x700);
cpt_pf_reg1_rest!(CPTX_PF_EXEC_BUSY, 0x800);
cpt_pf_reg1_rest!(CPTX_PF_EXEC_INFO0, 0x900);
cpt_pf_reg1_rest!(CPTX_PF_EXEC_INFO1, 0x910);
cpt_pf_reg1_rest!(CPTX_PF_INST_REQ_PC, 0x10000);
cpt_pf_reg1_rest!(CPTX_PF_INST_LATENCY_PC, 0x10020);
cpt_pf_reg1_rest!(CPTX_PF_RD_REQ_PC, 0x10040);
cpt_pf_reg1_rest!(CPTX_PF_RD_LATENCY_PC, 0x10060);
cpt_pf_reg1_rest!(CPTX_PF_RD_UC_PC, 0x10080);
cpt_pf_reg1_rest!(CPTX_PF_ACTIVE_CYCLES_PC, 0x10100);
cpt_pf_reg1_rest!(CPTX_PF_EXE_CTL, 0x4000000);
cpt_pf_reg1_rest!(CPTX_PF_EXE_STATUS, 0x4000008);
cpt_pf_reg1_rest!(CPTX_PF_EXE_CLK, 0x4000010);
cpt_pf_reg1_rest!(CPTX_PF_EXE_DBG_CTL, 0x4000018);
cpt_pf_reg1_rest!(CPTX_PF_EXE_DBG_DATA, 0x4000020);
cpt_pf_reg1_rest!(CPTX_PF_EXE_BIST_STATUS, 0x4000028);
cpt_pf_reg1_rest!(CPTX_PF_EXE_REQ_TIMER, 0x4000030);
cpt_pf_reg1_rest!(CPTX_PF_EXE_MEM_CTL, 0x4000038);
cpt_pf_reg1_rest!(CPTX_PF_EXE_PERF_CTL, 0x4001000);
cpt_pf_reg1_rest!(CPTX_PF_EXE_PERF_EVENT_CNT, 0x4001180);

macro_rules! cpt_pf_qx { ($name:ident, $base:expr) => { macro_rules! $name { ($a:expr, $b:expr) => { (($base as u64) + (($a as u64) << 36) + (($b as u64) << 20)) }; } }; }
cpt_pf_qx!(CPTX_PF_QX_CTL, 0x8000000);
cpt_pf_qx!(CPTX_PF_QX_GMCTL, 0x8000020);
cpt_pf_qx!(CPTX_PF_QX_CTL2, 0x8000100);

macro_rules! CPTX_PF_VFX_MBOXX { ($a:expr, $b:expr, $c:expr) => { 0x8001000u64 + (($a as u64) << 36) + (($b as u64) << 20) + (($c as u64) << 8) }; }
macro_rules! CPTX_PF_EXEC_INT { ($a:expr) => { 0x500u64 + 0x1000000000u64 * (($a as u64) & 0x1) }; }

macro_rules! cpt_vf_reg { ($name:ident, $base:expr) => { macro_rules! $name { ($a:expr, $b:expr) => { (($base as u64) + (($a as u64) << 36) + (($b as u64) << 20)) }; } }; }
cpt_vf_reg!(CPTX_VQX_CTL, 0x100); cpt_vf_reg!(CPTX_VQX_SADDR, 0x200);
cpt_vf_reg!(CPTX_VQX_DONE_WAIT, 0x400); cpt_vf_reg!(CPTX_VQX_INPROG, 0x410);
cpt_vf_reg!(CPTX_VQX_DONE, 0x420); cpt_vf_reg!(CPTX_VQX_DONE_ACK, 0x440);
cpt_vf_reg!(CPTX_VQX_DONE_INT_W1S, 0x460); cpt_vf_reg!(CPTX_VQX_DONE_INT_W1C, 0x468);
cpt_vf_reg!(CPTX_VQX_DONE_ENA_W1S, 0x470); cpt_vf_reg!(CPTX_VQX_DONE_ENA_W1C, 0x478);
cpt_vf_reg!(CPTX_VQX_MISC_INT, 0x500); cpt_vf_reg!(CPTX_VQX_MISC_INT_W1S, 0x508);
cpt_vf_reg!(CPTX_VQX_MISC_ENA_W1S, 0x510); cpt_vf_reg!(CPTX_VQX_MISC_ENA_W1C, 0x518);
cpt_vf_reg!(CPTX_VQX_DOORBELL, 0x600);
macro_rules! CPTX_VFX_PF_MBOXX { ($a:expr, $b:expr, $c:expr) => { 0x1000u64 + (($a as u64) << 36) + (($b as u64) << 20) + (($c as u64) << 3) }; }

#[repr(i32)]
pub enum vftype { AE_TYPES = 1, SE_TYPES = 2, BAD_CPT_TYPES }

#[repr(i32)]
pub enum cpt_mbox_opcode { CPT_MSG_VF_UP = 1, CPT_MSG_VF_DOWN, CPT_MSG_READY, CPT_MSG_QLEN, CPT_MSG_QBIND_GRP, CPT_MSG_VQ_PRIORITY }

#[repr(C)]
pub struct cpt_mbox { pub msg: u64, pub data: u64 }

extern "C" {
    fn writeq(val: u64, addr: *mut u8);
    fn readq(addr: *const u8) -> u64;
}

pub unsafe fn cpt_write_csr64(hw_addr: *mut u8, offset: u64, val: u64) {
    writeq(val, hw_addr.add(offset as usize));
}

pub unsafe fn cpt_read_csr64(hw_addr: *mut u8, offset: u64) -> u64 {
    readq(hw_addr.add(offset as usize))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
