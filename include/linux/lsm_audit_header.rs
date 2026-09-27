/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Common LSM logging functions
 * Heavily borrowed from selinux/avc.h
 *
 * Author : Etienne BASSET  <etienne.basset@ensta.org>
 *
 * All credits to : Stephen Smalley
 * All BUGS to : Etienne BASSET  <etienne.basset@ensta.org>
 */

/* C dependencies supplied by other headers are intentionally not implemented here. */

#[repr(C)]
pub struct lsm_network_audit {
    pub netif: ::kernel::ffi::c_int,
    pub sk: *const sock,
    pub family: u16,
    pub dport: __be16,
    pub sport: __be16,
    pub fam: lsm_network_audit_fam,
}

#[repr(C)]
pub union lsm_network_audit_fam {
    pub v4: lsm_network_audit_v4,
    pub v6: lsm_network_audit_v6,
}

#[repr(C)]
pub struct lsm_network_audit_v4 {
    pub daddr: __be32,
    pub saddr: __be32,
}

#[repr(C)]
pub struct lsm_network_audit_v6 {
    pub daddr: in6_addr,
    pub saddr: in6_addr,
}

#[repr(C)]
pub struct lsm_ioctlop_audit {
    pub path: path,
    pub cmd: u16,
}

#[repr(C)]
pub struct lsm_ibpkey_audit {
    pub subnet_prefix: u64,
    pub pkey: u16,
}

#[repr(C)]
pub struct lsm_ibendport_audit {
    pub dev_name: *const ::kernel::ffi::c_char,
    pub port: u8,
}

/* Auxiliary data to use in generating the audit record. */
#[repr(C)]
pub struct common_audit_data {
    pub type_: ::kernel::ffi::c_char,
    pub u: common_audit_data_u,
    /* this union contains LSM specific data */
    pub lsm_data: common_audit_data_lsm_data,
}

pub const LSM_AUDIT_DATA_PATH: ::kernel::ffi::c_int = 1;
pub const LSM_AUDIT_DATA_NET: ::kernel::ffi::c_int = 2;
pub const LSM_AUDIT_DATA_CAP: ::kernel::ffi::c_int = 3;
pub const LSM_AUDIT_DATA_IPC: ::kernel::ffi::c_int = 4;
pub const LSM_AUDIT_DATA_TASK: ::kernel::ffi::c_int = 5;
pub const LSM_AUDIT_DATA_KEY: ::kernel::ffi::c_int = 6;
pub const LSM_AUDIT_DATA_NONE: ::kernel::ffi::c_int = 7;
pub const LSM_AUDIT_DATA_KMOD: ::kernel::ffi::c_int = 8;
pub const LSM_AUDIT_DATA_INODE: ::kernel::ffi::c_int = 9;
pub const LSM_AUDIT_DATA_DENTRY: ::kernel::ffi::c_int = 10;
pub const LSM_AUDIT_DATA_IOCTL_OP: ::kernel::ffi::c_int = 11;
pub const LSM_AUDIT_DATA_FILE: ::kernel::ffi::c_int = 12;
pub const LSM_AUDIT_DATA_IBPKEY: ::kernel::ffi::c_int = 13;
pub const LSM_AUDIT_DATA_IBENDPORT: ::kernel::ffi::c_int = 14;
pub const LSM_AUDIT_DATA_LOCKDOWN: ::kernel::ffi::c_int = 15;
pub const LSM_AUDIT_DATA_NOTIFICATION: ::kernel::ffi::c_int = 16;
pub const LSM_AUDIT_DATA_ANONINODE: ::kernel::ffi::c_int = 17;
pub const LSM_AUDIT_DATA_NLMSGTYPE: ::kernel::ffi::c_int = 18;

#[repr(C)]
pub union common_audit_data_u {
    pub path: path,
    pub dentry: *mut dentry,
    pub inode: *mut inode,
    pub net: *mut lsm_network_audit,
    pub cap: ::kernel::ffi::c_int,
    pub ipc_id: ::kernel::ffi::c_int,
    pub tsk: *mut task_struct,
    /* CONFIG_KEYS conditional member */
    pub key_struct: common_audit_data_key_struct,
    pub kmod_name: *mut ::kernel::ffi::c_char,
    pub op: *mut lsm_ioctlop_audit,
    pub file: *const file,
    pub ibpkey: *mut lsm_ibpkey_audit,
    pub ibendport: *mut lsm_ibendport_audit,
    pub reason: ::kernel::ffi::c_int,
    pub anonclass: *const ::kernel::ffi::c_char,
    pub nlmsg_type: u16,
}

#[repr(C)]
pub struct common_audit_data_key_struct {
    pub key: key_serial_t,
    pub key_desc: *mut ::kernel::ffi::c_char,
}

#[repr(C)]
pub union common_audit_data_lsm_data {
    /* CONFIG_SECURITY_SMACK, CONFIG_SECURITY_SELINUX, and CONFIG_SECURITY_APPARMOR conditional members */
    pub smack_audit_data: *mut smack_audit_data,
    pub selinux_audit_data: *mut selinux_audit_data,
    pub apparmor_audit_data: *mut apparmor_audit_data,
}

/* C aliases: v4info expands to fam.v4, and v6info expands to fam.v6. */

#[cfg(CONFIG_AUDIT)]
extern "C" {
    pub fn ipv4_skb_to_auditdata(skb: *mut sk_buff, ad: *mut common_audit_data, proto: *mut u8) -> ::kernel::ffi::c_int;
    /* IS_ENABLED(CONFIG_IPV6) conditional declaration. */
    pub fn ipv6_skb_to_auditdata(skb: *mut sk_buff, ad: *mut common_audit_data, proto: *mut u8) -> ::kernel::ffi::c_int;
    pub fn common_lsm_audit(
        a: *mut common_audit_data,
        pre_audit: Option<unsafe extern "C" fn(*mut audit_buffer, *mut ::kernel::ffi::c_void)>,
        post_audit: Option<unsafe extern "C" fn(*mut audit_buffer, *mut ::kernel::ffi::c_void)>,
    );
    pub fn audit_log_lsm_data(ab: *mut audit_buffer, a: *const common_audit_data);
}

#[cfg(not(CONFIG_AUDIT))]
pub unsafe fn common_lsm_audit(
    _a: *mut common_audit_data,
    _pre_audit: Option<unsafe extern "C" fn(*mut audit_buffer, *mut ::kernel::ffi::c_void)>,
    _post_audit: Option<unsafe extern "C" fn(*mut audit_buffer, *mut ::kernel::ffi::c_void)>,
) {
}

#[cfg(not(CONFIG_AUDIT))]
pub unsafe fn audit_log_lsm_data(_ab: *mut audit_buffer, _a: *const common_audit_data) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
