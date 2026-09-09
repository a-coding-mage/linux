// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
// Do not edit directly, auto-generated from:
//     Documentation/netlink/specs/dpll.yaml
// YNL-GEN kernel source
// To regenerate run: tools/net/ynl/ynl-regen.sh

// Dependencies are supplied by the surrounding kernel translation unit.

extern "C" {
    fn dpll_lock_doit() -> i32;
    fn dpll_unlock_doit() -> i32;
    fn dpll_pre_doit() -> i32;
    fn dpll_post_doit() -> i32;
    fn dpll_pin_pre_doit() -> i32;
    fn dpll_pin_post_doit() -> i32;
    fn dpll_nl_device_id_get_doit() -> i32;
    fn dpll_nl_device_get_doit() -> i32;
    fn dpll_nl_device_get_dumpit() -> i32;
    fn dpll_nl_device_set_doit() -> i32;
    fn dpll_nl_pin_id_get_doit() -> i32;
    fn dpll_nl_pin_get_doit() -> i32;
    fn dpll_nl_pin_get_dumpit() -> i32;
    fn dpll_nl_pin_set_doit() -> i32;
}

#[repr(C)]
pub struct nla_policy {
    pub type_: u16,
    pub validation_type: u8,
    pub len: u8,
}

#[repr(C)]
pub struct genl_split_ops {
    pub cmd: u8,
    pub pre_doit: Option<unsafe extern "C" fn() -> i32>,
    pub doit: Option<unsafe extern "C" fn() -> i32>,
    pub post_doit: Option<unsafe extern "C" fn() -> i32>,
    pub dumpit: Option<unsafe extern "C" fn() -> i32>,
    pub policy: *const nla_policy,
    pub maxattr: u16,
    pub flags: u32,
}

#[repr(C)]
pub struct genl_multicast_group {
    pub name: *const core::ffi::c_char,
}

#[repr(C)]
pub struct genl_family {
    pub name: *const core::ffi::c_char,
    pub version: u8,
    pub netnsok: bool,
    pub parallel_ops: bool,
    pub module: *mut core::ffi::c_void,
    pub split_ops: *const genl_split_ops,
    pub n_split_ops: usize,
    pub mcgrps: *const genl_multicast_group,
    pub n_mcgrps: usize,
}

const NLA_U32: u16 = 3;
const NLA_U64: u16 = 5;
const NLA_S32: u16 = 4;
const NLA_S64: u16 = 6;
const NLA_SINT: u16 = 14;
const NLA_NUL_STRING: u16 = 10;

const fn policy(t: u16) -> nla_policy { nla_policy { type_: t, validation_type: 0, len: 0 } }

pub static dpll_pin_parent_device_nl_policy: [nla_policy; (DPLL_A_PIN_OPERSTATE + 1) as usize] = [policy(0); (DPLL_A_PIN_OPERSTATE + 1) as usize];
pub static dpll_pin_parent_pin_nl_policy: [nla_policy; (DPLL_A_PIN_STATE + 1) as usize] = [policy(0); (DPLL_A_PIN_STATE + 1) as usize];
pub static dpll_reference_sync_nl_policy: [nla_policy; (DPLL_A_PIN_STATE + 1) as usize] = [policy(0); (DPLL_A_PIN_STATE + 1) as usize];

static dpll_device_id_get_nl_policy: [nla_policy; (DPLL_A_TYPE + 1) as usize] = [policy(0); (DPLL_A_TYPE + 1) as usize];
static dpll_device_get_nl_policy: [nla_policy; (DPLL_A_ID + 1) as usize] = [policy(0); (DPLL_A_ID + 1) as usize];
static dpll_device_set_nl_policy: [nla_policy; (DPLL_A_FREQUENCY_MONITOR + 1) as usize] = [policy(0); (DPLL_A_FREQUENCY_MONITOR + 1) as usize];
static dpll_pin_id_get_nl_policy: [nla_policy; (DPLL_A_PIN_TYPE + 1) as usize] = [policy(0); (DPLL_A_PIN_TYPE + 1) as usize];
static dpll_pin_get_do_nl_policy: [nla_policy; (DPLL_A_PIN_ID + 1) as usize] = [policy(0); (DPLL_A_PIN_ID + 1) as usize];
static dpll_pin_get_dump_nl_policy: [nla_policy; (DPLL_A_PIN_ID + 1) as usize] = [policy(0); (DPLL_A_PIN_ID + 1) as usize];
static dpll_pin_set_nl_policy: [nla_policy; (DPLL_A_PIN_REFERENCE_SYNC + 1) as usize] = [policy(0); (DPLL_A_PIN_REFERENCE_SYNC + 1) as usize];

// The designated initializers below retain the C operation-table ordering.
static dpll_nl_ops: [genl_split_ops; 8] = [
    genl_split_ops { cmd: DPLL_CMD_DEVICE_ID_GET, pre_doit: Some(dpll_lock_doit), doit: Some(dpll_nl_device_id_get_doit), post_doit: Some(dpll_unlock_doit), dumpit: None, policy: dpll_device_id_get_nl_policy.as_ptr(), maxattr: DPLL_A_TYPE, flags: GENL_ADMIN_PERM | GENL_CMD_CAP_DO },
    genl_split_ops { cmd: DPLL_CMD_DEVICE_GET, pre_doit: Some(dpll_pre_doit), doit: Some(dpll_nl_device_get_doit), post_doit: Some(dpll_post_doit), dumpit: None, policy: dpll_device_get_nl_policy.as_ptr(), maxattr: DPLL_A_ID, flags: GENL_ADMIN_PERM | GENL_CMD_CAP_DO },
    genl_split_ops { cmd: DPLL_CMD_DEVICE_GET, pre_doit: None, doit: None, post_doit: None, dumpit: Some(dpll_nl_device_get_dumpit), policy: core::ptr::null(), maxattr: 0, flags: GENL_ADMIN_PERM | GENL_CMD_CAP_DUMP },
    genl_split_ops { cmd: DPLL_CMD_DEVICE_SET, pre_doit: Some(dpll_pre_doit), doit: Some(dpll_nl_device_set_doit), post_doit: Some(dpll_post_doit), dumpit: None, policy: dpll_device_set_nl_policy.as_ptr(), maxattr: DPLL_A_FREQUENCY_MONITOR, flags: GENL_ADMIN_PERM | GENL_CMD_CAP_DO },
    genl_split_ops { cmd: DPLL_CMD_PIN_ID_GET, pre_doit: Some(dpll_lock_doit), doit: Some(dpll_nl_pin_id_get_doit), post_doit: Some(dpll_unlock_doit), dumpit: None, policy: dpll_pin_id_get_nl_policy.as_ptr(), maxattr: DPLL_A_PIN_TYPE, flags: GENL_ADMIN_PERM | GENL_CMD_CAP_DO },
    genl_split_ops { cmd: DPLL_CMD_PIN_GET, pre_doit: Some(dpll_pin_pre_doit), doit: Some(dpll_nl_pin_get_doit), post_doit: Some(dpll_pin_post_doit), dumpit: None, policy: dpll_pin_get_do_nl_policy.as_ptr(), maxattr: DPLL_A_PIN_ID, flags: GENL_ADMIN_PERM | GENL_CMD_CAP_DO },
    genl_split_ops { cmd: DPLL_CMD_PIN_GET, pre_doit: None, doit: None, post_doit: None, dumpit: Some(dpll_nl_pin_get_dumpit), policy: dpll_pin_get_dump_nl_policy.as_ptr(), maxattr: DPLL_A_PIN_ID, flags: GENL_ADMIN_PERM | GENL_CMD_CAP_DUMP },
    genl_split_ops { cmd: DPLL_CMD_PIN_SET, pre_doit: Some(dpll_pin_pre_doit), doit: Some(dpll_nl_pin_set_doit), post_doit: Some(dpll_pin_post_doit), dumpit: None, policy: dpll_pin_set_nl_policy.as_ptr(), maxattr: DPLL_A_PIN_REFERENCE_SYNC, flags: GENL_ADMIN_PERM | GENL_CMD_CAP_DO },
];

static dpll_nl_mcgrps: [genl_multicast_group; 1] = [genl_multicast_group { name: b"monitor\0".as_ptr() as *const _ }];

#[no_mangle]
pub static mut dpll_nl_family: genl_family = genl_family {
    name: DPLL_FAMILY_NAME.as_ptr() as *const _,
    version: DPLL_FAMILY_VERSION,
    netnsok: true,
    parallel_ops: true,
    module: THIS_MODULE,
    split_ops: dpll_nl_ops.as_ptr(),
    n_split_ops: dpll_nl_ops.len(),
    mcgrps: dpll_nl_mcgrps.as_ptr(),
    n_mcgrps: dpll_nl_mcgrps.len(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
