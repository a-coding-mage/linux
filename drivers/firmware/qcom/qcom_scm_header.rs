/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2010-2015,2019 The Linux Foundation. All rights reserved. */

// Forward declarations supplied by other translation units.
pub enum device {}
pub enum qcom_tzmem_pool {}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum qcom_scm_convention {
    SMC_CONVENTION_UNKNOWN,
    SMC_CONVENTION_LEGACY,
    SMC_CONVENTION_ARM_32,
    SMC_CONVENTION_ARM_64,
}

unsafe extern "C" {
    pub static mut qcom_scm_convention: qcom_scm_convention;
}

pub const MAX_QCOM_SCM_ARGS: usize = 10;
pub const MAX_QCOM_SCM_RETS: usize = 3;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum qcom_scm_arg_types {
    QCOM_SCM_VAL,
    QCOM_SCM_RO,
    QCOM_SCM_RW,
    QCOM_SCM_BUFVAL,
}

#[macro_export]
macro_rules! QCOM_SCM_ARGS {
    ($num:expr $(, $arg:expr)*) => {
        $crate::qcom_scm_args_impl!($num $(, $arg)*)
    };
}

#[macro_export]
macro_rules! qcom_scm_args_impl {
    ($num:expr) => { (($num as u32) & 0xf) };
    ($num:expr, $a:expr) => { (($a as u32 & 0x3) << 4) | (($num as u32) & 0xf) };
    ($num:expr, $a:expr, $b:expr) => { (($a as u32 & 0x3) << 4) | (($b as u32 & 0x3) << 6) | (($num as u32) & 0xf) };
    ($num:expr, $a:expr, $b:expr, $c:expr) => { (($a as u32 & 0x3) << 4) | (($b as u32 & 0x3) << 6) | (($c as u32 & 0x3) << 8) | (($num as u32) & 0xf) };
    ($num:expr, $a:expr, $b:expr, $c:expr, $d:expr) => { (($a as u32 & 0x3) << 4) | (($b as u32 & 0x3) << 6) | (($c as u32 & 0x3) << 8) | (($d as u32 & 0x3) << 10) | (($num as u32) & 0xf) };
    ($num:expr, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => { $crate::qcom_scm_args_impl!($num, $a, $b, $c, $d) | (($e as u32 & 0x3) << 12) };
    ($num:expr, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr) => { $crate::qcom_scm_args_impl!($num, $a, $b, $c, $d, $e) | (($f as u32 & 0x3) << 14) };
    ($num:expr, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr) => { $crate::qcom_scm_args_impl!($num, $a, $b, $c, $d, $e, $f) | (($g as u32 & 0x3) << 16) };
    ($num:expr, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr) => { $crate::qcom_scm_args_impl!($num, $a, $b, $c, $d, $e, $f, $g) | (($h as u32 & 0x3) << 18) };
    ($num:expr, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr) => { $crate::qcom_scm_args_impl!($num, $a, $b, $c, $d, $e, $f, $g, $h) | (($i as u32 & 0x3) << 20) };
    ($num:expr, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $j:expr) => { $crate::qcom_scm_args_impl!($num, $a, $b, $c, $d, $e, $f, $g, $h, $i) | (($j as u32 & 0x3) << 22) };
}

#[repr(C)]
pub struct qcom_scm_desc {
    pub svc: u32,
    pub cmd: u32,
    pub arginfo: u32,
    pub args: [u64; MAX_QCOM_SCM_ARGS],
    pub owner: u32,
}

#[repr(C)]
pub struct qcom_scm_res {
    pub result: [u64; MAX_QCOM_SCM_RETS],
}

unsafe extern "C" {
    pub fn qcom_scm_wait_for_wq_completion(dev: *mut device, wq_ctx: u32) -> i32;
    pub fn scm_get_wq_ctx(wq_ctx: *mut u32, flags: *mut u32, more_pending: *mut u32) -> i32;
    pub fn __scm_smc_call(
        dev: *mut device,
        desc: *const qcom_scm_desc,
        qcom_convention: qcom_scm_convention,
        res: *mut qcom_scm_res,
        atomic: bool,
    ) -> i32;
    pub fn scm_legacy_call_atomic(dev: *mut device, desc: *const qcom_scm_desc, res: *mut qcom_scm_res) -> i32;
    pub fn scm_legacy_call(dev: *mut device, desc: *const qcom_scm_desc, res: *mut qcom_scm_res) -> i32;
    pub fn qcom_scm_get_tzmem_pool() -> *mut qcom_tzmem_pool;
    pub fn qcom_scm_shm_bridge_enable(scm_dev: *mut device) -> i32;
}

#[macro_export]
macro_rules! SCM_SMC_FNID { ($s:expr, $c:expr) => { ((($s as u32 & 0xff) << 8) | ($c as u32 & 0xff)) }; }
#[macro_export]
macro_rules! SCM_LEGACY_FNID { ($s:expr, $c:expr) => { (($s as u32 << 10) | ($c as u32 & 0x3ff)) }; }

#[macro_export]
macro_rules! scm_smc_call {
    ($dev:expr, $desc:expr, $res:expr, $atomic:expr) => {
        unsafe { $crate::__scm_smc_call($dev, $desc, $crate::qcom_scm_convention, $res, $atomic) }
    };
}

macro_rules! c_u32 { ($name:ident, $value:expr) => { pub const $name: u32 = $value; }; }
c_u32!(QCOM_SCM_SVC_BOOT, 0x01); c_u32!(QCOM_SCM_BOOT_SET_ADDR, 0x01); c_u32!(QCOM_SCM_BOOT_TERMINATE_PC, 0x02);
c_u32!(QCOM_SCM_BOOT_SDI_CONFIG, 0x09); c_u32!(QCOM_SCM_BOOT_SET_DLOAD_MODE, 0x10); c_u32!(QCOM_SCM_BOOT_SET_ADDR_MC, 0x11);
c_u32!(QCOM_SCM_BOOT_SET_REMOTE_STATE, 0x0a); c_u32!(QCOM_SCM_FLUSH_FLAG_MASK, 0x3); c_u32!(QCOM_SCM_BOOT_MAX_CPUS, 4);
c_u32!(QCOM_SCM_BOOT_MC_FLAG_AARCH64, 1 << 0); c_u32!(QCOM_SCM_BOOT_MC_FLAG_COLDBOOT, 1 << 1); c_u32!(QCOM_SCM_BOOT_MC_FLAG_WARMBOOT, 1 << 2);
c_u32!(QCOM_SCM_SVC_PIL, 0x02); c_u32!(QCOM_SCM_PIL_PAS_INIT_IMAGE, 0x01); c_u32!(QCOM_SCM_PIL_PAS_MEM_SETUP, 0x02);
c_u32!(QCOM_SCM_PIL_PAS_AUTH_AND_RESET, 0x05); c_u32!(QCOM_SCM_PIL_PAS_SHUTDOWN, 0x06); c_u32!(QCOM_SCM_PIL_PAS_IS_SUPPORTED, 0x07);
c_u32!(QCOM_SCM_PIL_PAS_MSS_RESET, 0x0a); c_u32!(QCOM_SCM_PIL_PAS_GET_RSCTABLE, 0x21);
c_u32!(QCOM_SCM_SVC_IO, 0x05); c_u32!(QCOM_SCM_IO_READ, 0x01); c_u32!(QCOM_SCM_IO_WRITE, 0x02);
c_u32!(QCOM_SCM_SVC_INFO, 0x06); c_u32!(QCOM_SCM_INFO_IS_CALL_AVAIL, 0x01);
c_u32!(QCOM_SCM_SVC_MP, 0x0c); c_u32!(QCOM_SCM_MP_RESTORE_SEC_CFG, 0x02); c_u32!(QCOM_SCM_MP_IOMMU_SECURE_PTBL_SIZE, 0x03); c_u32!(QCOM_SCM_MP_IOMMU_SECURE_PTBL_INIT, 0x04); c_u32!(QCOM_SCM_MP_IOMMU_SET_CP_POOL_SIZE, 0x05); c_u32!(QCOM_SCM_MP_VIDEO_VAR, 0x08); c_u32!(QCOM_SCM_MP_ASSIGN, 0x16); c_u32!(QCOM_SCM_MP_CP_SMMU_APERTURE_ID, 0x1b); c_u32!(QCOM_SCM_MP_SHM_BRIDGE_ENABLE, 0x1c); c_u32!(QCOM_SCM_MP_SHM_BRIDGE_DELETE, 0x1d); c_u32!(QCOM_SCM_MP_SHM_BRIDGE_CREATE, 0x1e);
c_u32!(QCOM_SCM_SVC_OCMEM, 0x0f); c_u32!(QCOM_SCM_OCMEM_LOCK_CMD, 0x01); c_u32!(QCOM_SCM_OCMEM_UNLOCK_CMD, 0x02);
c_u32!(QCOM_SCM_SVC_ES, 0x10); c_u32!(QCOM_SCM_ES_INVALIDATE_ICE_KEY, 0x03); c_u32!(QCOM_SCM_ES_CONFIG_SET_ICE_KEY, 0x04); c_u32!(QCOM_SCM_ES_DERIVE_SW_SECRET, 0x07); c_u32!(QCOM_SCM_ES_GENERATE_ICE_KEY, 0x08); c_u32!(QCOM_SCM_ES_PREPARE_ICE_KEY, 0x09); c_u32!(QCOM_SCM_ES_IMPORT_ICE_KEY, 0x0a);
c_u32!(QCOM_SCM_SVC_HDCP, 0x11); c_u32!(QCOM_SCM_HDCP_INVOKE, 0x01); c_u32!(QCOM_SCM_SVC_LMH, 0x13); c_u32!(QCOM_SCM_LMH_LIMIT_PROFILE_CHANGE, 0x01); c_u32!(QCOM_SCM_LMH_LIMIT_DCVSH, 0x10);
c_u32!(QCOM_SCM_SVC_SMMU_PROGRAM, 0x15); c_u32!(QCOM_SCM_SMMU_PT_FORMAT, 0x01); c_u32!(QCOM_SCM_SMMU_CONFIG_ERRATA1, 0x03); c_u32!(QCOM_SCM_SMMU_CONFIG_ERRATA1_CLIENT_ALL, 0x02);
c_u32!(QCOM_SCM_SVC_WAITQ, 0x24); c_u32!(QCOM_SCM_WAITQ_RESUME, 0x02); c_u32!(QCOM_SCM_WAITQ_GET_WQ_CTX, 0x03); c_u32!(QCOM_SCM_WAITQ_GET_INFO, 0x04);
c_u32!(QCOM_SCM_SVC_GPU, 0x28); c_u32!(QCOM_SCM_SVC_GPU_INIT_REGS, 0x01); c_u32!(QCOM_SCM_SVC_SMCINVOKE, 0x06); c_u32!(QCOM_SCM_SMCINVOKE_INVOKE_LEGACY, 0x00); c_u32!(QCOM_SCM_SMCINVOKE_CB_RSP, 0x01); c_u32!(QCOM_SCM_SMCINVOKE_INVOKE, 0x02);

pub const QCOM_SCM_V2_EBUSY: i32 = -12; pub const QCOM_SCM_ENOMEM: i32 = -5; pub const QCOM_SCM_EOPNOTSUPP: i32 = -4; pub const QCOM_SCM_EINVAL_ADDR: i32 = -3; pub const QCOM_SCM_EINVAL_ARG: i32 = -2; pub const QCOM_SCM_ERROR: i32 = -1; pub const QCOM_SCM_INTERRUPTED: i32 = 1; pub const QCOM_SCM_WAITQ_SLEEP: i32 = 2;

#[inline]
pub const fn qcom_scm_remap_error(err: i32) -> i32 {
    match err {
        QCOM_SCM_ERROR => -EIO,
        QCOM_SCM_EINVAL_ADDR | QCOM_SCM_EINVAL_ARG => -EINVAL,
        QCOM_SCM_EOPNOTSUPP => -EOPNOTSUPP,
        QCOM_SCM_ENOMEM => -ENOMEM,
        QCOM_SCM_V2_EBUSY => -EBUSY,
        _ => -EINVAL,
    }
}

// Kernel errno constants are supplied by the target environment.
unsafe extern "C" {
    static EIO: i32; static EINVAL: i32; static EOPNOTSUPP: i32; static ENOMEM: i32; static EBUSY: i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
