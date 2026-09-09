// SPDX-License-Identifier: GPL-2.0
/*
 *  Shared Memory Communications over RDMA (SMC-R) and RoCE
 *
 *  Generic netlink support functions to interact with SMC module
 *
 *  Copyright IBM Corp. 2020
 *
 *  Author(s): Guvenc Gulce <guvenc@linux.ibm.com>
 */

// Linux kernel headers and local SMC headers supplied by the surrounding build.

use core::ffi::c_void;

extern "C" {
    fn genl_register_family(family: *mut genl_family) -> i32;
    fn genl_unregister_family(family: *mut genl_family);

    fn smc_nl_get_sys_info(_: *mut c_void, _: *mut c_void, _: *mut c_void) -> i32;
    fn smcr_nl_get_lgr(_: *mut c_void, _: *mut c_void, _: *mut c_void) -> i32;
    fn smcr_nl_get_link(_: *mut c_void, _: *mut c_void, _: *mut c_void) -> i32;
    fn smcd_nl_get_lgr(_: *mut c_void, _: *mut c_void, _: *mut c_void) -> i32;
    fn smcd_nl_get_device(_: *mut c_void, _: *mut c_void, _: *mut c_void) -> i32;
    fn smcr_nl_get_device(_: *mut c_void, _: *mut c_void, _: *mut c_void) -> i32;
    fn smc_nl_get_stats(_: *mut c_void, _: *mut c_void, _: *mut c_void) -> i32;
    fn smc_nl_get_fback_stats(_: *mut c_void, _: *mut c_void, _: *mut c_void) -> i32;
    fn smc_nl_dump_ueid(_: *mut c_void, _: *mut c_void, _: *mut c_void) -> i32;
    fn smc_nl_add_ueid(_: *mut c_void, _: *mut c_void, _: *mut c_void) -> i32;
    fn smc_nl_remove_ueid(_: *mut c_void, _: *mut c_void, _: *mut c_void) -> i32;
    fn smc_nl_flush_ueid(_: *mut c_void, _: *mut c_void, _: *mut c_void) -> i32;
    fn smc_nl_dump_seid(_: *mut c_void, _: *mut c_void, _: *mut c_void) -> i32;
    fn smc_nl_enable_seid(_: *mut c_void, _: *mut c_void, _: *mut c_void) -> i32;
    fn smc_nl_disable_seid(_: *mut c_void, _: *mut c_void, _: *mut c_void) -> i32;
    fn smc_nl_dump_hs_limitation(_: *mut c_void, _: *mut c_void, _: *mut c_void) -> i32;
    fn smc_nl_enable_hs_limitation(_: *mut c_void, _: *mut c_void, _: *mut c_void) -> i32;
    fn smc_nl_disable_hs_limitation(_: *mut c_void, _: *mut c_void, _: *mut c_void) -> i32;
}

#[repr(C)]
pub struct nla_policy { pub type_: u16, pub len: u16 }

#[repr(C)]
pub struct genl_ops {
    pub cmd: u8,
    pub flags: u8,
    pub dumpit: Option<unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> i32>,
    pub doit: Option<unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> i32>,
    pub policy: *const nla_policy,
}

#[repr(C)]
pub struct genl_family {
    pub hdrsize: u16,
    pub name: *const u8,
    pub version: u8,
    pub maxattr: u8,
    pub policy: *const nla_policy,
    pub netnsok: bool,
    pub module: *mut c_void,
    pub ops: *const genl_ops,
    pub n_ops: usize,
    pub resv_start_op: u8,
}

pub const SMC_CMD_MAX_ATTR: usize = 1;

#[no_mangle]
pub static smc_gen_ueid_policy: [nla_policy; SMC_NLA_EID_TABLE_MAX as usize + 1] = [
    nla_policy { type_: NLA_UNSPEC as u16, len: 0 },
    nla_policy { type_: NLA_STRING as u16, len: SMC_MAX_EID_LEN as u16 },
];

static smc_gen_nl_policy: [nla_policy; 2] = [
    nla_policy { type_: 0, len: 0 },
    nla_policy { type_: NLA_REJECT as u16, len: 0 },
];

static smc_gen_nl_ops: [genl_ops; 18] = [
    genl_ops { cmd: SMC_NETLINK_GET_SYS_INFO, flags: 0, dumpit: Some(smc_nl_get_sys_info), doit: None, policy: core::ptr::null() },
    genl_ops { cmd: SMC_NETLINK_GET_LGR_SMCR, flags: 0, dumpit: Some(smcr_nl_get_lgr), doit: None, policy: core::ptr::null() },
    genl_ops { cmd: SMC_NETLINK_GET_LINK_SMCR, flags: 0, dumpit: Some(smcr_nl_get_link), doit: None, policy: core::ptr::null() },
    genl_ops { cmd: SMC_NETLINK_GET_LGR_SMCD, flags: 0, dumpit: Some(smcd_nl_get_lgr), doit: None, policy: core::ptr::null() },
    genl_ops { cmd: SMC_NETLINK_GET_DEV_SMCD, flags: 0, dumpit: Some(smcd_nl_get_device), doit: None, policy: core::ptr::null() },
    genl_ops { cmd: SMC_NETLINK_GET_DEV_SMCR, flags: 0, dumpit: Some(smcr_nl_get_device), doit: None, policy: core::ptr::null() },
    genl_ops { cmd: SMC_NETLINK_GET_STATS, flags: 0, dumpit: Some(smc_nl_get_stats), doit: None, policy: core::ptr::null() },
    genl_ops { cmd: SMC_NETLINK_GET_FBACK_STATS, flags: 0, dumpit: Some(smc_nl_get_fback_stats), doit: None, policy: core::ptr::null() },
    genl_ops { cmd: SMC_NETLINK_DUMP_UEID, flags: 0, dumpit: Some(smc_nl_dump_ueid), doit: None, policy: core::ptr::null() },
    genl_ops { cmd: SMC_NETLINK_ADD_UEID, flags: GENL_ADMIN_PERM, dumpit: None, doit: Some(smc_nl_add_ueid), policy: smc_gen_ueid_policy.as_ptr() },
    genl_ops { cmd: SMC_NETLINK_REMOVE_UEID, flags: GENL_ADMIN_PERM, dumpit: None, doit: Some(smc_nl_remove_ueid), policy: smc_gen_ueid_policy.as_ptr() },
    genl_ops { cmd: SMC_NETLINK_FLUSH_UEID, flags: GENL_ADMIN_PERM, dumpit: None, doit: Some(smc_nl_flush_ueid), policy: core::ptr::null() },
    genl_ops { cmd: SMC_NETLINK_DUMP_SEID, flags: 0, dumpit: Some(smc_nl_dump_seid), doit: None, policy: core::ptr::null() },
    genl_ops { cmd: SMC_NETLINK_ENABLE_SEID, flags: GENL_ADMIN_PERM, dumpit: None, doit: Some(smc_nl_enable_seid), policy: core::ptr::null() },
    genl_ops { cmd: SMC_NETLINK_DISABLE_SEID, flags: GENL_ADMIN_PERM, dumpit: None, doit: Some(smc_nl_disable_seid), policy: core::ptr::null() },
    genl_ops { cmd: SMC_NETLINK_DUMP_HS_LIMITATION, flags: 0, dumpit: Some(smc_nl_dump_hs_limitation), doit: None, policy: core::ptr::null() },
    genl_ops { cmd: SMC_NETLINK_ENABLE_HS_LIMITATION, flags: GENL_ADMIN_PERM, dumpit: None, doit: Some(smc_nl_enable_hs_limitation), policy: core::ptr::null() },
    genl_ops { cmd: SMC_NETLINK_DISABLE_HS_LIMITATION, flags: GENL_ADMIN_PERM, dumpit: None, doit: Some(smc_nl_disable_hs_limitation), policy: core::ptr::null() },
];

#[no_mangle]
pub static mut smc_gen_nl_family: genl_family = genl_family {
    hdrsize: 0,
    name: SMC_GENL_FAMILY_NAME.as_ptr(),
    version: SMC_GENL_FAMILY_VERSION,
    maxattr: SMC_CMD_MAX_ATTR as u8,
    policy: smc_gen_nl_policy.as_ptr(),
    netnsok: true,
    module: THIS_MODULE,
    ops: smc_gen_nl_ops.as_ptr(),
    n_ops: smc_gen_nl_ops.len(),
    resv_start_op: SMC_NETLINK_DISABLE_HS_LIMITATION + 1,
};

pub unsafe extern "C" fn smc_nl_init() -> i32 {
    genl_register_family(&mut smc_gen_nl_family)
}

pub unsafe extern "C" fn smc_nl_exit() {
    genl_unregister_family(&mut smc_gen_nl_family);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
