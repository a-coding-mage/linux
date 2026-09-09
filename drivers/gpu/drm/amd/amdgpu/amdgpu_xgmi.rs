/*
 * Copyright 2018 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// External Linux/AMDGPU declarations, register definitions, and macros are
// supplied by the surrounding kernel translation unit.

const SMN_PCS_XGMI3X16_PCS_ERROR_STATUS: u32 = 0x11a0020c;
const SMN_PCS_XGMI3X16_PCS_ERROR_NONCORRECTABLE_MASK: u32 = 0x11a00218;
const SMN_PCS_GOPX1_PCS_ERROR_STATUS: u32 = 0x12200210;
const SMN_PCS_GOPX1_PCS_ERROR_NONCORRECTABLE_MASK: u32 = 0x12200218;
const XGMI_STATE_DISABLE: u32 = 0xD1;
const XGMI_STATE_LS0: u32 = 0x81;
const AMDGPU_MAX_XGMI_DEVICE_PER_HIVE: usize = 4;

static mut XGMI_MUTEX: Mutex = Mutex::new();
static mut XGMI_HIVE_LIST: ListHead = ListHead::new();

static XGMI_PCS_ERR_STATUS_REG_VG20: [i32; 2] = [
    smnXGMI0_PCS_GOPX16_PCS_ERROR_STATUS,
    smnXGMI0_PCS_GOPX16_PCS_ERROR_STATUS + 0x100000,
];
static WAFL_PCS_ERR_STATUS_REG_VG20: [i32; 2] = [
    smnPCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS,
    smnPCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS + 0x100000,
];
static XGMI_PCS_ERR_STATUS_REG_ARCT: [i32; 6] = [
    smnXGMI0_PCS_GOPX16_PCS_ERROR_STATUS,
    smnXGMI0_PCS_GOPX16_PCS_ERROR_STATUS + 0x100000,
    smnXGMI0_PCS_GOPX16_PCS_ERROR_STATUS + 0x500000,
    smnXGMI0_PCS_GOPX16_PCS_ERROR_STATUS + 0x600000,
    smnXGMI0_PCS_GOPX16_PCS_ERROR_STATUS + 0x700000,
    smnXGMI0_PCS_GOPX16_PCS_ERROR_STATUS + 0x800000,
];
static WAFL_PCS_ERR_STATUS_REG_ARCT: [i32; 2] = [
    smnPCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS,
    smnPCS_GOPX1_0_PCS_GOPX1_PCS_ERROR_STATUS + 0x100000,
];
static XGMI3X16_PCS_ERR_STATUS_REG_ALDEBARAN: [i32; 8] = [
    SMN_PCS_XGMI3X16_PCS_ERROR_STATUS, SMN_PCS_XGMI3X16_PCS_ERROR_STATUS + 0x100000,
    SMN_PCS_XGMI3X16_PCS_ERROR_STATUS + 0x200000, SMN_PCS_XGMI3X16_PCS_ERROR_STATUS + 0x300000,
    SMN_PCS_XGMI3X16_PCS_ERROR_STATUS + 0x400000, SMN_PCS_XGMI3X16_PCS_ERROR_STATUS + 0x500000,
    SMN_PCS_XGMI3X16_PCS_ERROR_STATUS + 0x600000, SMN_PCS_XGMI3X16_PCS_ERROR_STATUS + 0x700000,
];
static XGMI3X16_PCS_ERR_NONCORRECTABLE_MASK_REG_ALDEBARAN: [i32; 8] = [
    SMN_PCS_XGMI3X16_PCS_ERROR_NONCORRECTABLE_MASK, SMN_PCS_XGMI3X16_PCS_ERROR_NONCORRECTABLE_MASK + 0x100000,
    SMN_PCS_XGMI3X16_PCS_ERROR_NONCORRECTABLE_MASK + 0x200000, SMN_PCS_XGMI3X16_PCS_ERROR_NONCORRECTABLE_MASK + 0x300000,
    SMN_PCS_XGMI3X16_PCS_ERROR_NONCORRECTABLE_MASK + 0x400000, SMN_PCS_XGMI3X16_PCS_ERROR_NONCORRECTABLE_MASK + 0x500000,
    SMN_PCS_XGMI3X16_PCS_ERROR_NONCORRECTABLE_MASK + 0x600000, SMN_PCS_XGMI3X16_PCS_ERROR_NONCORRECTABLE_MASK + 0x700000,
];
static WAFL_PCS_ERR_STATUS_REG_ALDEBARAN: [i32; 2] = [
    SMN_PCS_GOPX1_PCS_ERROR_STATUS, SMN_PCS_GOPX1_PCS_ERROR_STATUS + 0x100000,
];
static WAFL_PCS_ERR_NONCORRECTABLE_MASK_REG_ALDEBARAN: [i32; 2] = [
    SMN_PCS_GOPX1_PCS_ERROR_NONCORRECTABLE_MASK,
    SMN_PCS_GOPX1_PCS_ERROR_NONCORRECTABLE_MASK + 0x100000,
];

// The field tables are represented using the externally defined C-compatible
// amdgpu_pcs_ras_field type; SOC15_REG_FIELD values remain external constants.
static XGMI_PCS_RAS_FIELDS: [amdgpu_pcs_ras_field; 18] = [
    pcs!("XGMI PCS DataLossErr", XGMI0_PCS_GOPX16_PCS_ERROR_STATUS, DataLossErr),
    pcs!("XGMI PCS TrainingErr", XGMI0_PCS_GOPX16_PCS_ERROR_STATUS, TrainingErr),
    pcs!("XGMI PCS CRCErr", XGMI0_PCS_GOPX16_PCS_ERROR_STATUS, CRCErr),
    pcs!("XGMI PCS BERExceededErr", XGMI0_PCS_GOPX16_PCS_ERROR_STATUS, BERExceededErr),
    pcs!("XGMI PCS TxMetaDataErr", XGMI0_PCS_GOPX16_PCS_ERROR_STATUS, TxMetaDataErr),
    pcs!("XGMI PCS ReplayBufParityErr", XGMI0_PCS_GOPX16_PCS_ERROR_STATUS, ReplayBufParityErr),
    pcs!("XGMI PCS DataParityErr", XGMI0_PCS_GOPX16_PCS_ERROR_STATUS, DataParityErr),
    pcs!("XGMI PCS ReplayFifoOverflowErr", XGMI0_PCS_GOPX16_PCS_ERROR_STATUS, ReplayFifoOverflowErr),
    pcs!("XGMI PCS ReplayFifoUnderflowErr", XGMI0_PCS_GOPX16_PCS_ERROR_STATUS, ReplayFifoUnderflowErr),
    pcs!("XGMI PCS ElasticFifoOverflowErr", XGMI0_PCS_GOPX16_PCS_ERROR_STATUS, ElasticFifoOverflowErr),
    pcs!("XGMI PCS DeskewErr", XGMI0_PCS_GOPX16_PCS_ERROR_STATUS, DeskewErr),
    pcs!("XGMI PCS DataStartupLimitErr", XGMI0_PCS_GOPX16_PCS_ERROR_STATUS, DataStartupLimitErr),
    pcs!("XGMI PCS FCInitTimeoutErr", XGMI0_PCS_GOPX16_PCS_ERROR_STATUS, FCInitTimeoutErr),
    pcs!("XGMI PCS RecoveryTimeoutErr", XGMI0_PCS_GOPX16_PCS_ERROR_STATUS, RecoveryTimeoutErr),
    pcs!("XGMI PCS ReadySerialTimeoutErr", XGMI0_PCS_GOPX16_PCS_ERROR_STATUS, ReadySerialTimeoutErr),
    pcs!("XGMI PCS ReadySerialAttemptErr", XGMI0_PCS_GOPX16_PCS_ERROR_STATUS, ReadySerialAttemptErr),
    pcs!("XGMI PCS RecoveryAttemptErr", XGMI0_PCS_GOPX16_PCS_ERROR_STATUS, RecoveryAttemptErr),
    pcs!("XGMI PCS RecoveryRelockAttemptErr", XGMI0_PCS_GOPX16_PCS_ERROR_STATUS, RecoveryRelockAttemptErr),
];

pub unsafe fn amdgpu_xgmi_get_ext_link(adev: *mut amdgpu_device, link_num: i32) -> i32 {
    let map = [0, 3, 1, 2, 7, 6, 4, 5];
    if (*adev).gmc.xgmi.num_physical_nodes <= 1 { return -EINVAL; }
    match amdgpu_ip_version(adev, XGMI_HWIP, 0) {
        IP_VERSION(6, 4, 0) | IP_VERSION(6, 4, 1) => {
            if link_num >= 0 && (link_num as usize) < map.len() { return map[link_num as usize]; }
        }
        _ => return -EINVAL,
    }
    -EINVAL
}

unsafe fn xgmi_v6_4_get_link_status(adev: *mut amdgpu_device, global_link_num: i32) -> u32 {
    let a = [0x11a00070u64, 0x11b00070];
    let b = [0x12100070u64, 0x11b00070];
    let (regs, n) = match amdgpu_ip_version(adev, XGMI_HWIP, 0) {
        IP_VERSION(6, 4, 0) => (&a, a.len()), IP_VERSION(6, 4, 1) => (&b, b.len()), _ => return U32_MAX,
    };
    let i = global_link_num as usize / n;
    if (*adev).aid_mask & BIT(i) == 0 { return U32_MAX; }
    let addr = regs[global_link_num as usize % n] + amdgpu_reg_get_smn_base64(adev, XGMI_HWIP, i);
    RREG32_PCIE_EXT(adev, addr)
}

pub unsafe fn amdgpu_get_xgmi_link_status(adev: *mut amdgpu_device, global_link_num: i32) -> i32 {
    if amdgpu_sriov_vf(adev) { return AMDGPU_XGMI_LINK_NA; }
    if (*adev).gmc.xgmi.num_physical_nodes <= 1 { return -EINVAL; }
    let v = match amdgpu_ip_version(adev, XGMI_HWIP, 0) {
        IP_VERSION(6, 4, 0) | IP_VERSION(6, 4, 1) => xgmi_v6_4_get_link_status(adev, global_link_num),
        _ => return -EOPNOTSUPP,
    };
    if v & 0xff == XGMI_STATE_DISABLE { return -ENOLINK; }
    if v & 0xff == XGMI_STATE_LS0 { AMDGPU_XGMI_LINK_ACTIVE } else { AMDGPU_XGMI_LINK_INACTIVE }
}

// The remaining sysfs, hive, topology, bandwidth, reset, and RAS routines
// retain the C implementation's control flow and call the surrounding kernel
// ABI.  Their declarations are kept here as external entry points.
extern "C" {
    pub fn amdgpu_xgmi_show_attrs(kobj: *mut kobject, attr: *mut attribute, buf: *mut c_char) -> ssize_t;
    pub fn amdgpu_get_xgmi_hive(adev: *mut amdgpu_device) -> *mut amdgpu_hive_info;
    pub fn amdgpu_put_xgmi_hive(hive: *mut amdgpu_hive_info);
    pub fn amdgpu_xgmi_set_pstate(adev: *mut amdgpu_device, pstate: i32) -> i32;
    pub fn amdgpu_xgmi_update_topology(hive: *mut amdgpu_hive_info, adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_xgmi_get_hops_count(adev: *mut amdgpu_device, peer: *mut amdgpu_device) -> i32;
    pub fn amdgpu_xgmi_get_bandwidth(adev: *mut amdgpu_device, peer: *mut amdgpu_device, mode: amdgpu_xgmi_bw_mode, unit: amdgpu_xgmi_bw_unit, min: *mut u32, max: *mut u32) -> i32;
    pub fn amdgpu_xgmi_get_is_sharing_enabled(adev: *mut amdgpu_device, peer: *mut amdgpu_device) -> bool;
    pub fn amdgpu_xgmi_add_device(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_xgmi_remove_device(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_xgmi_get_relative_phy_addr(adev: *mut amdgpu_device, addr: u64) -> u64;
    pub fn amdgpu_xgmi_ras_sw_init(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_xgmi_reset_on_init(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_xgmi_request_nps_change(adev: *mut amdgpu_device, hive: *mut amdgpu_hive_info, req_nps_mode: i32) -> i32;
    pub fn amdgpu_xgmi_same_hive(adev: *mut amdgpu_device, bo_adev: *mut amdgpu_device) -> bool;
    pub fn amdgpu_xgmi_early_init(adev: *mut amdgpu_device);
    pub fn amgpu_xgmi_set_max_speed_width(adev: *mut amdgpu_device, max_speed: u16, max_width: u8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
