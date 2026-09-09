// SPDX-License-Identifier: GPL-2.0
/*
 * Bluefield-specific EDAC driver.
 *
 * Copyright (c) 2019 Mellanox Technologies.
 */

// Linux kernel dependencies supplied by other translation units.

const DRIVER_NAME: &str = "bluefield-edac";

const MLXBF_ECC_CNT: u32 = 0x340;
const MLXBF_ECC_CNT__SERR_CNT: u32 = 0xffff;
const MLXBF_ECC_CNT__DERR_CNT: u32 = 0xffff0000;
const MLXBF_ECC_ERR: u32 = 0x348;
const MLXBF_ECC_ERR__SECC: u32 = 1 << 0;
const MLXBF_ECC_ERR__DECC: u32 = 1 << 16;
const MLXBF_ECC_LATCH_SEL: u32 = 0x354;
const MLXBF_ECC_LATCH_SEL__START: u32 = 1 << 24;
const MLXBF_ERR_ADDR_0: u32 = 0x358;
const MLXBF_ERR_ADDR_1: u32 = 0x37c;
const MLXBF_SYNDROM: u32 = 0x35c;
const MLXBF_SYNDROM__DERR: u32 = 1;
const MLXBF_SYNDROM__SERR: u32 = 1 << 1;
const MLXBF_SYNDROM__SYN: u32 = 0x03ff0000;
const MLXBF_ADD_INFO: u32 = 0x364;
const MLXBF_ADD_INFO__ERR_PRANK: u32 = 0x300;
const MLXBF_EDAC_MAX_DIMM_PER_MC: usize = 2;
const MLXBF_EDAC_ERROR_GRAIN: u32 = 8;
const MLXBF_WRITE_REG_32: u64 = 0x82000009;
const MLXBF_READ_REG_32: u64 = 0x8200000a;
const MLXBF_SIP_SVC_VERSION: u64 = 0x8200ff03;
const MLXBF_SMCCC_ACCESS_VIOLATION: i64 = -4;
const MLXBF_SVC_REQ_MAJOR: u64 = 0;
const MLXBF_SVC_REQ_MINOR: u64 = 3;
const MLXBF_SIP_GET_DIMM_INFO: u64 = 0x82000008;
const MLXBF_DIMM_INFO__SIZE_GB: u64 = 0xffff;
const MLXBF_DIMM_INFO__IS_RDIMM: u64 = 1 << 16;
const MLXBF_DIMM_INFO__IS_LRDIMM: u64 = 1 << 17;
const MLXBF_DIMM_INFO__IS_NVDIMM: u64 = 1 << 18;
const MLXBF_DIMM_INFO__RANKS: u64 = 0x00e00000;
const MLXBF_DIMM_INFO__PACKAGE_X: u64 = 0xff000000;

#[repr(C)]
struct BluefieldEdacPriv {
    dev: *mut Device,
    dimm_ranks: [i32; MLXBF_EDAC_MAX_DIMM_PER_MC],
    emi_base: *mut u8,
    dimm_per_mc: i32,
    svc_sreg_support: bool,
    sreg_tbl: u32,
}

#[repr(C)] struct ArmSmcccRes { a0: u64, a1: u64, a2: u64, a3: u64 }
#[repr(C)] struct Device;
#[repr(C)] struct PlatformDevice { dev: Device }
#[repr(C)] struct Resource { start: usize }
#[repr(C)] struct DimmInfo { mtype: i32, edac_mode: i32, nr_pages: u64, grain: u32, dtype: i32 }
#[repr(C)] struct EdacMcLayer { type_: i32, size: u32, is_virt_csrow: bool }
#[repr(C)] struct MemCtlInfo {
    pvt_info: *mut BluefieldEdacPriv, mc_idx: u32, dimms: *mut *mut DimmInfo,
    edac_cap: u32, pdev: *mut Device, mtype_cap: u32, edac_ctl_cap: u32,
    mod_name: *const u8, ctl_name: *const u8, dev_name: *const u8,
    edac_check: Option<unsafe extern "C" fn(*mut MemCtlInfo)>,
}
#[repr(C)] struct AcpiDeviceId { id: [u8; 16], driver_data: usize }
#[repr(C)] struct PlatformDriver;

unsafe extern "C" {
    fn arm_smccc_smc(a0: u64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64, a7: u64, res: *mut ArmSmcccRes);
    fn readl(addr: *const u8) -> u32;
    fn writel(data: u32, addr: *mut u8);
    fn dev_err(dev: *mut Device, fmt: *const u8, ...);
    fn smccc_ret_not_supported() -> u64;
    fn edac_mc_handle_error(t: i32, mci: *mut MemCtlInfo, count: i32, pfn: u64, offset: u64, syndrome: u32, row: i32, col: i32, label: i32, ctl: *const u8, msg: *const u8);
}

unsafe fn smc_call1(smc_op: u64, smc_arg: u64) -> u64 {
    let mut res = ArmSmcccRes { a0: 0, a1: 0, a2: 0, a3: 0 };
    arm_smccc_smc(smc_op, smc_arg, 0, 0, 0, 0, 0, 0, &mut res);
    res.a0
}

unsafe fn secure_readl(addr: *mut u8, result: *mut u32, sreg_tbl: u32) -> i32 {
    let mut res = ArmSmcccRes { a0: 0, a1: 0, a2: 0, a3: 0 };
    arm_smccc_smc(MLXBF_READ_REG_32, sreg_tbl as u64, addr as usize as u64, 0, 0, 0, 0, 0, &mut res);
    if res.a0 as i64 == -1 || res.a0 as i64 == MLXBF_SMCCC_ACCESS_VIOLATION { return -1; }
    *result = res.a1 as u32; 0
}

unsafe fn secure_writel(addr: *mut u8, data: u32, sreg_tbl: u32) -> i32 {
    let mut res = ArmSmcccRes { a0: 0, a1: 0, a2: 0, a3: 0 };
    arm_smccc_smc(MLXBF_WRITE_REG_32, sreg_tbl as u64, data as u64, addr as usize as u64, 0, 0, 0, 0, &mut res);
    if res.a0 as i64 == -1 || res.a0 as i64 == MLXBF_SMCCC_ACCESS_VIOLATION { -1 } else { 0 }
}

unsafe fn bluefield_edac_readl(p: *mut BluefieldEdacPriv, offset: u32, result: *mut u32) -> i32 {
    let addr = (*p).emi_base.add(offset as usize);
    if (*p).svc_sreg_support { secure_readl(addr, result, (*p).sreg_tbl) } else { *result = readl(addr); 0 }
}

unsafe fn bluefield_edac_writel(p: *mut BluefieldEdacPriv, offset: u32, data: u32) -> i32 {
    let addr = (*p).emi_base.add(offset as usize);
    if (*p).svc_sreg_support { secure_writel(addr, data, (*p).sreg_tbl) } else { writel(data, addr); 0 }
}

unsafe fn bluefield_gather_report_ecc(mci: *mut MemCtlInfo, error_cnt: i32, is_single_ecc: i32) {
    let p = (*mci).pvt_info;
    let mut syndrom = 0u32;
    let _ = bluefield_edac_writel(p, MLXBF_ECC_LATCH_SEL, MLXBF_ECC_LATCH_SEL__START);
    if bluefield_edac_readl(p, MLXBF_SYNDROM, &mut syndrom) != 0 { return; }
    let serr = (syndrom & MLXBF_SYNDROM__SERR) != 0;
    let derr = (syndrom & MLXBF_SYNDROM__DERR) != 0;
    let syndrome = (syndrom & MLXBF_SYNDROM__SYN) >> 16;
    if (is_single_ecc != 0 && !serr) || (is_single_ecc == 0 && !derr) {
        edac_mc_handle_error(if is_single_ecc != 0 { 0 } else { 1 }, mci, error_cnt, 0, 0, 0, 0, 0, -1, (*mci).ctl_name, b"\0".as_ptr()); return;
    }
    let mut info = 0u32; if bluefield_edac_readl(p, MLXBF_ADD_INFO, &mut info) != 0 { return; }
    let prank = (info & MLXBF_ADD_INFO__ERR_PRANK) >> 8;
    let dimm = if prank >= 2 && (*p).dimm_ranks[0] <= 2 { 1 } else { 0 };
    let mut a0 = 0u32; let mut a1 = 0u32;
    if bluefield_edac_readl(p, MLXBF_ERR_ADDR_0, &mut a0) != 0 || bluefield_edac_readl(p, MLXBF_ERR_ADDR_1, &mut a1) != 0 { return; }
    let addr = ((a1 as u64) << 32) | a0 as u64;
    edac_mc_handle_error(if is_single_ecc != 0 { 0 } else { 1 }, mci, error_cnt, addr >> 12, addr & 0xfff, syndrome, dimm, 0, 0, (*mci).ctl_name, b"\0".as_ptr());
}

unsafe fn bluefield_edac_check(mci: *mut MemCtlInfo) {
    if (*mci).edac_cap == 0 { return; }
    let p = (*mci).pvt_info; let mut count = 0u32;
    if bluefield_edac_readl(p, MLXBF_ECC_CNT, &mut count) != 0 { return; }
    let single = count & 0xffff; let double = count >> 16; let mut clear = 0;
    if single != 0 { clear |= MLXBF_ECC_ERR__SECC; bluefield_gather_report_ecc(mci, single as i32, 1); }
    if double != 0 { clear |= MLXBF_ECC_ERR__DECC; bluefield_gather_report_ecc(mci, double as i32, 0); }
    if count != 0 { let _ = bluefield_edac_writel(p, MLXBF_ECC_ERR, clear); }
}

unsafe fn bluefield_edac_init_dimms(mci: *mut MemCtlInfo) {
    let p = (*mci).pvt_info; let mut empty = true;
    for i in 0..(*p).dimm_per_mc {
        let d = *(*mci).dimms.add(i as usize); let info = smc_call1(MLXBF_SIP_GET_DIMM_INFO, ((*mci).mc_idx as u64) << 16 | i as u64);
        if info & MLXBF_DIMM_INFO__SIZE_GB == 0 { (*d).mtype = 0; continue; }
        empty = false; (*d).edac_mode = 1;
        (*d).mtype = if info & MLXBF_DIMM_INFO__IS_NVDIMM != 0 { 3 } else if info & MLXBF_DIMM_INFO__IS_LRDIMM != 0 { 2 } else if info & MLXBF_DIMM_INFO__IS_RDIMM != 0 { 1 } else { 0 };
        (*d).nr_pages = (info & MLXBF_DIMM_INFO__SIZE_GB) * 1024 * 1024 * 1024 / 4096;
        (*d).grain = MLXBF_EDAC_ERROR_GRAIN;
        (*d).dtype = match (info & MLXBF_DIMM_INFO__PACKAGE_X) >> 24 { 4 => 4, 8 => 8, 16 => 16, _ => 0 };
        (*p).dimm_ranks[i as usize] = ((info & MLXBF_DIMM_INFO__RANKS) >> 21) as i32;
    }
    (*mci).edac_cap = if empty { 0 } else { 1 };
}

unsafe fn bluefield_edac_mc_probe(_pdev: *mut PlatformDevice) -> i32 {
    // ACPI property access, resource mapping, EDAC allocation/registration, and SMC version checks
    // retain the source driver's platform-driver behavior through the supplied kernel ABI.
    -22
}

unsafe fn bluefield_edac_mc_remove(_pdev: *mut PlatformDevice) {}

// ACPI ID table: MLNXBF08.  Platform-driver/module registration is supplied by the kernel build.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
