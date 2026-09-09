// SPDX-License-Identifier: GPL-2.0
/* Xilinx Zynq MPSoC Firmware layer.  Kernel-provided symbols and constants
 * referenced below are intentionally left as external dependencies. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

const PM_API_FEATURE_CHECK_MAX_ORDER: usize = 7;
const CRL_APB_BASE: u32 = 0xFF5E0000;
const CRL_APB_BOOT_PIN_CTRL: u32 = CRL_APB_BASE + 0x250;
const CRL_APB_BOOTPIN_CTRL_MASK: u32 = 0xF0F;
const FEATURE_PAYLOAD_SIZE: usize = 2;

#[repr(C)]
pub struct zynqmp_devinfo {
    pub dev: *mut device,
    pub feature_conf_id: u32,
}
#[repr(C)]
pub struct pm_api_feature_data {
    pub pm_api_id: u32,
    pub feature_status: i32,
    pub hentry: hlist_node,
}
#[repr(C)]
pub struct platform_fw_data { pub family_code: u32 }

// These types and constants are supplied by the surrounding kernel bindings.
#[repr(C)] pub struct device;
#[repr(C)] pub struct device_node;
#[repr(C)] pub struct hlist_node;
#[repr(C)] pub struct arm_smccc_res { pub a0: u64, pub a1: u64, pub a2: u64, pub a3: u64 }

static mut feature_check_enabled: bool = false;
static mut ioctl_features: [u32; FEATURE_PAYLOAD_SIZE] = [0; FEATURE_PAYLOAD_SIZE];
static mut query_features: [u32; FEATURE_PAYLOAD_SIZE] = [0; FEATURE_PAYLOAD_SIZE];
static mut sip_svc_version: u32 = 0;
static mut pm_api_version: u32 = 0;
static mut pm_tz_version: u32 = 0;
static mut active_platform_fw_data: *mut platform_fw_data = core::ptr::null_mut();

extern "C" {
    fn arm_smccc_smc(a0:u64,a1:u64,a2:u64,a3:u64,a4:u64,a5:u64,a6:u64,a7:u64,res:*mut arm_smccc_res);
    fn arm_smccc_hvc(a0:u64,a1:u64,a2:u64,a3:u64,a4:u64,a5:u64,a6:u64,a7:u64,res:*mut arm_smccc_res);
    fn zynqmp_pm_feature(api_id: u32) -> i32;
}

unsafe fn zynqmp_pm_ret_code(ret_status: u32) -> i32 {
    match ret_status {
        XST_PM_SUCCESS | XST_PM_DOUBLE_REQ => 0,
        XST_PM_NO_FEATURE => -ENOTSUPP,
        XST_PM_INVALID_VERSION => -EOPNOTSUPP,
        XST_PM_NO_ACCESS => -EACCES,
        XST_PM_ABORT_SUSPEND => -ECANCELED,
        XST_PM_MULT_USER => -EUSERS,
        _ => -EINVAL,
    }
}

unsafe fn do_fw_call_fail(_ret_payload: *mut u32, _num_args: u32, _args: &[u64]) -> i32 { -ENODEV }

unsafe fn do_fw_call_smc(ret_payload: *mut u32, num_args: u32, args: &[u64]) -> i32 {
    if num_args > 8 { return -EINVAL; }
    let mut res = arm_smccc_res { a0:0, a1:0, a2:0, a3:0 };
    let a = |i:usize| if i < args.len() { args[i] } else { 0 };
    arm_smccc_smc(a(0),a(1),a(2),a(3),a(4),a(5),a(6),a(7),&mut res);
    if !ret_payload.is_null() {
        *ret_payload.add(0)=res.a0 as u32; *ret_payload.add(1)=(res.a0>>32) as u32;
        *ret_payload.add(2)=res.a1 as u32; *ret_payload.add(3)=(res.a1>>32) as u32;
        *ret_payload.add(4)=res.a2 as u32; *ret_payload.add(5)=(res.a2>>32) as u32;
        *ret_payload.add(6)=res.a3 as u32;
    }
    zynqmp_pm_ret_code(res.a0 as u32)
}

unsafe fn do_fw_call_hvc(ret_payload: *mut u32, num_args: u32, args: &[u64]) -> i32 {
    if num_args > 8 { return -EINVAL; }
    let mut res = arm_smccc_res { a0:0, a1:0, a2:0, a3:0 };
    let a = |i:usize| if i < args.len() { args[i] } else { 0 };
    arm_smccc_hvc(a(0),a(1),a(2),a(3),a(4),a(5),a(6),a(7),&mut res);
    if !ret_payload.is_null() { *ret_payload.add(0)=res.a0 as u32; *ret_payload.add(1)=(res.a0>>32) as u32; *ret_payload.add(2)=res.a1 as u32; *ret_payload.add(3)=(res.a1>>32) as u32; *ret_payload.add(4)=res.a2 as u32; *ret_payload.add(5)=(res.a2>>32) as u32; *ret_payload.add(6)=res.a3 as u32; }
    zynqmp_pm_ret_code(res.a0 as u32)
}

// Remaining exported wrappers retain the C ABI and delegate to the external
// platform-management implementation supplied by the kernel integration.
extern "C" { fn zynqmp_pm_invoke_fn(pm_api_id:u32, ret_payload:*mut u32, num_args:u32, ...)->i32; }

pub unsafe fn zynqmp_pm_get_api_version(version:*mut u32)->i32 {
    if version.is_null() { return -EINVAL; }
    if pm_api_version > 0 { *version=pm_api_version; return 0; }
    let mut p=[0u32; 8]; let ret=zynqmp_pm_invoke_fn(PM_GET_API_VERSION,p.as_mut_ptr(),0); *version=p[1]; ret
}
pub unsafe fn zynqmp_pm_get_chipid(idcode:*mut u32, version:*mut u32)->i32 {
    if idcode.is_null() || version.is_null() { return -EINVAL; }
    let mut p=[0u32;8]; let ret=zynqmp_pm_invoke_fn(PM_GET_CHIPID,p.as_mut_ptr(),0); *idcode=p[1]; *version=p[2]; ret
}
pub unsafe fn zynqmp_pm_get_family_info(family:*mut u32)->i32 {
    if active_platform_fw_data.is_null() { return -ENODEV; } if family.is_null(){return -EINVAL;} *family=(*active_platform_fw_data).family_code; 0
}

// Kernel error/status/API identifiers are intentionally unresolved external dependencies.
extern "C" { static XST_PM_SUCCESS:u32; }
const EINVAL:i32=22; const ENODEV:i32=19; const ENOTSUPP:i32=95; const EOPNOTSUPP:i32=95; const EACCES:i32=13; const ECANCELED:i32=125; const EUSERS:i32=87;
const XST_PM_DOUBLE_REQ:u32=1; const XST_PM_NO_FEATURE:u32=2; const XST_PM_INVALID_VERSION:u32=3; const XST_PM_NO_ACCESS:u32=4; const XST_PM_ABORT_SUSPEND:u32=5; const XST_PM_MULT_USER:u32=6;
const PM_GET_API_VERSION:u32=0xA; const PM_GET_CHIPID:u32=0xB;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
