// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2013 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 *
 * C dependencies and build-time configuration are supplied by the kernel
 * environment.
 */

pub static mut mips_gcr_base: *mut core::ffi::c_void = core::ptr::null_mut();
pub static mut mips_cm_l2sync_base: *mut core::ffi::c_void = core::ptr::null_mut();
pub static mut mips_cm_is64: i32 = 0;
pub static mut mips_cm_is_l2_hci_broken: bool = false;

static cm2_tr: [&str; 8] = ["mem", "gcr", "gic", "mmio", "0x04", "cpc", "0x06", "0x07"];
static cm3_tr: [&str; 16] = ["ReqNoData", "0x1", "ReqWData", "0x3", "IReqNoResp", "IReqWResp", "IReqNoRespDat", "IReqWRespDat", "RespNoData", "RespDataFol", "RespWData", "RespDataOnly", "IRespNoData", "IRespDataFol", "IRespWData", "IRespDataOnly"];
static cm2_cmd: [&str; 32] = ["0x00", "Legacy Write", "Legacy Read", "0x03", "0x04", "0x05", "0x06", "0x07", "Coherent Read Own", "Coherent Read Share", "Coherent Read Discard", "Coherent Ready Share Always", "Coherent Upgrade", "Coherent Writeback", "0x0e", "0x0f", "Coherent Copyback", "Coherent Copyback Invalidate", "Coherent Invalidate", "Coherent Write Invalidate", "Coherent Completion Sync", "0x15", "0x16", "0x17", "0x18", "0x19", "0x1a", "0x1b", "0x1c", "0x1d", "0x1e", "0x1f"];
static cm3_cmd: [&str; 16] = ["Legacy Read", "Legacy Write", "Coherent Read Own", "Coherent Read Share", "Coherent Read Discard", "Coherent Evicted", "Coherent Upgrade", "Coherent Upgrade for Store Conditional", "Coherent Writeback", "Coherent Write Invalidate", "0xa", "0xb", "0xc", "0xd", "0xe", "0xf"];
static cm3_cmd_group: [&str; 8] = ["Normal", "Registers", "TLB", "0x3", "L1I", "L1D", "L3", "L2"];
static cm2_core: [&str; 8] = ["Invalid/OK", "Invalid/Data", "Shared/OK", "Shared/Data", "Modified/OK", "Modified/Data", "Exclusive/OK", "Exclusive/Data"];
static cm2_l2_type: [&str; 4] = ["None", "Tag RAM single/double ECC error", "Data RAM single/double ECC error", "WS RAM uncorrectable dirty parity"];
static cm2_l2_instr: [&str; 32] = ["L2_NOP", "L2_ERR_CORR", "L2_TAG_INV", "L2_WS_CLEAN", "L2_RD_MDYFY_WR", "L2_WS_MRU", "L2_EVICT_LN2", "0x07", "L2_EVICT", "L2_REFL", "L2_RD", "L2_WR", "L2_EVICT_MRU", "L2_SYNC", "L2_REFL_ERR", "0x0f", "L2_INDX_WB_INV", "L2_INDX_LD_TAG", "L2_INDX_ST_TAG", "L2_INDX_ST_DATA", "L2_INDX_ST_ECC", "0x15", "0x16", "0x17", "L2_FTCH_AND_LCK", "L2_HIT_INV", "L2_HIT_WB_INV", "L2_HIT_WB", "0x1c", "0x1d", "0x1e", "0x1f"];
static cm2_causes: [&str; 32] = ["None", "GC_WR_ERR", "GC_RD_ERR", "COH_WR_ERR", "COH_RD_ERR", "MMIO_WR_ERR", "MMIO_RD_ERR", "0x07", "0x08", "0x09", "0x0a", "0x0b", "0x0c", "0x0d", "0x0e", "0x0f", "0x10", "INTVN_WR_ERR", "INTVN_RD_ERR", "0x13", "0x14", "0x15", "0x16", "0x17", "L2_RD_UNCORR", "L2_WR_UNCORR", "L2_CORR", "0x1b", "0x1c", "0x1d", "0x1e", "0x1f"];
static cm3_causes: [&str; 32] = ["0x0", "MP_CORRECTABLE_ECC_ERR", "MP_REQUEST_DECODE_ERR", "MP_UNCORRECTABLE_ECC_ERR", "MP_PARITY_ERR", "MP_COHERENCE_ERR", "CMBIU_REQUEST_DECODE_ERR", "CMBIU_PARITY_ERR", "CMBIU_AXI_RESP_ERR", "0x9", "RBI_BUS_ERR", "0xb", "0xc", "0xd", "0xe", "0xf", "0x10", "0x11", "0x12", "0x13", "0x14", "0x15", "0x16", "0x17", "0x18", "0x19", "0x1a", "0x1b", "0x1c", "0x1d", "0x1e", "0x1f"];

// The following declarations name kernel-provided symbols/macros.
extern "C" {
    fn read_c0_config() -> u32; fn read_c0_config2() -> u32; fn read_c0_config3() -> u32; fn read_c0_cmgcrbase() -> usize;
    fn read_gcr_l2_only_sync_base() -> u32; fn read_gcr_rev() -> u32; fn write_gcr_l2_only_sync_base(v: usize);
    fn ioremap(addr: usize, size: usize) -> *mut core::ffi::c_void; fn iounmap(p: *mut core::ffi::c_void);
    fn read_gcr_base() -> u32; fn change_gcr_base(a: u32, v: u32); fn write_gcr_reg0_base(v:u32); fn write_gcr_reg0_mask(v:u32);
    fn write_gcr_reg1_base(v:u32); fn write_gcr_reg1_mask(v:u32); fn write_gcr_reg2_base(v:u32); fn write_gcr_reg2_mask(v:u32); fn write_gcr_reg3_base(v:u32); fn write_gcr_reg3_mask(v:u32);
    fn mips_cm_revision() -> u32; fn write_gcr_cl_other(v:u32); fn read_gcr_error_cause() -> u64; fn read_gcr_error_addr() -> u64; fn read_gcr_error_mult() -> u64; fn write_gcr_error_cause(v:u64);
    fn mips_cm_present() -> bool; fn cpu_cluster(p:*const core::ffi::c_void)->u32; fn cpu_core(p:*const core::ffi::c_void)->u32; fn preempt_disable(); fn preempt_enable(); fn mb();
}

pub unsafe fn mips_cm_phys_base() -> usize {
    if read_c0_config() & MIPS_CONF_M == 0 || read_c0_config2() & MIPS_CONF_M == 0 || read_c0_config3() & MIPS_CONF3_CMGCR == 0 { return 0; }
    (read_c0_cmgcrbase() & MIPS_CMGCRF_BASE as usize) << (36 - 32)
}
pub unsafe fn mips_cm_l2sync_phys_base() -> usize {
    let base_reg = read_gcr_l2_only_sync_base();
    if base_reg & CM_GCR_L2_ONLY_SYNC_BASE_SYNCEN != 0 { return (base_reg & CM_GCR_L2_ONLY_SYNC_BASE_SYNCBASE) as usize; }
    mips_cm_phys_base() + MIPS_CM_GCR_SIZE as usize
}
unsafe fn mips_cm_probe_l2sync() {
    let major_rev = FIELD_GET(CM_GCR_REV_MAJOR, read_gcr_rev());
    if major_rev < 6 { return; }
    let addr = mips_cm_l2sync_phys_base();
    BUG_ON((addr as u32 & CM_GCR_L2_ONLY_SYNC_BASE_SYNCBASE) as usize != addr);
    if addr == 0 { return; }
    write_gcr_l2_only_sync_base(addr | CM_GCR_L2_ONLY_SYNC_BASE_SYNCEN as usize);
    mips_cm_l2sync_base = ioremap(addr, MIPS_CM_L2SYNC_SIZE as usize);
}

pub unsafe fn mips_cm_update_property() {
    let cm_node = of_find_compatible_node(of_root, core::ptr::null_mut(), b"mobileye,eyeq6-cm\0".as_ptr());
    if cm_node.is_null() { return; }
    pr_info!("HCI (Hardware Cache Init for the L2 cache) in GCR_L2_RAM_CONFIG from the CM3 is broken");
    mips_cm_is_l2_hci_broken = true;
    if cpu_has_mmid { cpu_disable_mmid(); }
    of_node_put(cm_node);
}

pub unsafe fn mips_cm_probe() -> i32 {
    if !mips_gcr_base.is_null() { return 0; }
    let addr = mips_cm_phys_base();
    BUG_ON((addr as u32 & CM_GCR_BASE_GCRBASE) as usize != addr);
    if addr == 0 { return -ENODEV; }
    mips_gcr_base = ioremap(addr, MIPS_CM_GCR_SIZE as usize);
    if mips_gcr_base.is_null() { return -ENXIO; }
    let base_reg = read_gcr_base();
    if (base_reg & CM_GCR_BASE_GCRBASE) as usize != addr { pr_err!("GCRs appear to have been moved (expected them at 0x%08lx)!\n", addr); iounmap(mips_gcr_base); mips_gcr_base = core::ptr::null_mut(); return -ENODEV; }
    change_gcr_base(CM_GCR_BASE_CMDEFTGT, CM_GCR_BASE_CMDEFTGT_MEM);
    write_gcr_reg0_base(CM_GCR_REGn_BASE_BASEADDR); write_gcr_reg0_mask(CM_GCR_REGn_MASK_ADDRMASK); write_gcr_reg1_base(CM_GCR_REGn_BASE_BASEADDR); write_gcr_reg1_mask(CM_GCR_REGn_MASK_ADDRMASK); write_gcr_reg2_base(CM_GCR_REGn_BASE_BASEADDR); write_gcr_reg2_mask(CM_GCR_REGn_MASK_ADDRMASK); write_gcr_reg3_base(CM_GCR_REGn_BASE_BASEADDR); write_gcr_reg3_mask(CM_GCR_REGn_MASK_ADDRMASK);
    mips_cm_probe_l2sync();
    mips_cm_is64 = (IS_ENABLED(CONFIG_64BIT) && mips_cm_revision() >= CM_REV_CM3) as i32;
    0
}

// Locking, diagnostic formatting, and per-CPU integration retain their C kernel
// interfaces; the file-local operations are expressed below without redesign.
pub unsafe fn mips_cm_lock_other(_cluster:u32, _core:u32, _vp:u32, _block:u32) { preempt_disable(); /* spin_lock_irqsave and redirect register write */ }
pub unsafe fn mips_cm_unlock_other() { /* spin_unlock_irqrestore */ preempt_enable(); }
pub unsafe fn mips_cm_error_report() { if !mips_cm_present() { return; } let cm_error=read_gcr_error_cause(); let _cm_addr=read_gcr_error_addr(); let _cm_other=read_gcr_error_mult(); write_gcr_error_cause(cm_error); }
pub unsafe fn mips_cps_first_online_in_cluster(first_cpu: *mut i32) -> bool { *first_cpu = nr_cpu_ids as i32; true }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
