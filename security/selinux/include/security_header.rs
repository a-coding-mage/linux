/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Security server interface.
 *
 * Author : Stephen Smalley, <stephen.smalley.work@gmail.com>
 *
 */

use core::ffi::{c_char, c_int, c_void};

pub const SECSID_NULL: u32 = 0x00000000; /* unspecified SID */
pub const SECSID_WILD: u32 = 0xffffffff; /* wildcard SID */
pub const SECCLASS_NULL: u16 = 0x0000; /* no class */

/* Identify specific policy version changes */
pub const POLICYDB_VERSION_BASE: c_int = 15;
pub const POLICYDB_VERSION_BOOL: c_int = 16;
pub const POLICYDB_VERSION_IPV6: c_int = 17;
pub const POLICYDB_VERSION_NLCLASS: c_int = 18;
pub const POLICYDB_VERSION_VALIDATETRANS: c_int = 19;
pub const POLICYDB_VERSION_MLS: c_int = 19;
pub const POLICYDB_VERSION_AVTAB: c_int = 20;
pub const POLICYDB_VERSION_RANGETRANS: c_int = 21;
pub const POLICYDB_VERSION_POLCAP: c_int = 22;
pub const POLICYDB_VERSION_PERMISSIVE: c_int = 23;
pub const POLICYDB_VERSION_BOUNDARY: c_int = 24;
pub const POLICYDB_VERSION_FILENAME_TRANS: c_int = 25;
pub const POLICYDB_VERSION_ROLETRANS: c_int = 26;
pub const POLICYDB_VERSION_NEW_OBJECT_DEFAULTS: c_int = 27;
pub const POLICYDB_VERSION_DEFAULT_TYPE: c_int = 28;
pub const POLICYDB_VERSION_CONSTRAINT_NAMES: c_int = 29;
pub const POLICYDB_VERSION_XPERMS_IOCTL: c_int = 30;
pub const POLICYDB_VERSION_INFINIBAND: c_int = 31;
pub const POLICYDB_VERSION_GLBLUB: c_int = 32;
pub const POLICYDB_VERSION_COMP_FTRANS: c_int = 33; /* compressed filename transitions */
pub const POLICYDB_VERSION_COND_XPERMS: c_int = 34; /* extended permissions in conditional policies */
pub const POLICYDB_VERSION_NEVERAUDIT: c_int = 35; /* neveraudit types */

/* Range of policy versions we understand*/
pub const POLICYDB_VERSION_MIN: c_int = POLICYDB_VERSION_BASE;
pub const POLICYDB_VERSION_MAX: c_int = POLICYDB_VERSION_NEVERAUDIT;

/* Mask for just the mount related flags */
pub const SE_MNTMASK: c_int = 0x0f;
/* Super block security struct flags for mount options */
/* BE CAREFUL, these need to be the low order bits for selinux_get_mnt_opts */
pub const CONTEXT_MNT: c_int = 0x01;
pub const FSCONTEXT_MNT: c_int = 0x02;
pub const ROOTCONTEXT_MNT: c_int = 0x04;
pub const DEFCONTEXT_MNT: c_int = 0x08;
pub const SBLABEL_MNT: c_int = 0x10;
/* Non-mount related flags */
pub const SE_SBINITIALIZED: c_int = 0x0100;
pub const SE_SBPROC: c_int = 0x0200;
pub const SE_SBGENFS: c_int = 0x0400;
pub const SE_SBGENFS_XATTR: c_int = 0x0800;
pub const SE_SBNATIVE: c_int = 0x1000;

pub const CONTEXT_STR: &[u8] = b"context\0";
pub const FSCONTEXT_STR: &[u8] = b"fscontext\0";
pub const ROOTCONTEXT_STR: &[u8] = b"rootcontext\0";
pub const DEFCONTEXT_STR: &[u8] = b"defcontext\0";
pub const SECLABEL_STR: &[u8] = b"seclabel\0";

pub enum netlbl_lsm_secattr {}

extern "C" {
    pub static mut selinux_enabled_boot: c_int;
}

/*
 * type_datum properties
 * available at the kernel policy version >= POLICYDB_VERSION_BOUNDARY
 */
pub const TYPEDATUM_PROPERTY_PRIMARY: c_int = 0x0001;
pub const TYPEDATUM_PROPERTY_ATTRIBUTE: c_int = 0x0002;

/* limitation of boundary depth  */
pub const POLICYDB_BOUNDS_MAXDEPTH: c_int = 4;

pub enum selinux_policy {}
pub enum selinux_policy_convert_data {}
pub enum page {}
pub enum mutex {}
pub enum qstr {}
pub enum super_block {}
pub enum path {}

extern "C" {
    pub static __POLICYDB_CAP_MAX: usize;
    pub static POLICYDB_CAP_NETPEER: usize;
    pub static POLICYDB_CAP_OPENPERM: usize;
    pub static POLICYDB_CAP_EXTSOCKCLASS: usize;
    pub static POLICYDB_CAP_ALWAYSNETWORK: usize;
    pub static POLICYDB_CAP_CGROUPSECLABEL: usize;
    pub static POLICYDB_CAP_NNP_NOSUID_TRANSITION: usize;
    pub static POLICYDB_CAP_GENFS_SECLABEL_SYMLINKS: usize;
    pub static POLICYDB_CAP_IOCTL_SKIP_CLOEXEC: usize;
    pub static POLICYDB_CAP_USERSPACE_INITIAL_CONTEXT: usize;
    pub static POLICYDB_CAP_NETLINK_XPERM: usize;
    pub static POLICYDB_CAP_FUNCTIONFS_SECLABEL: usize;
    pub static POLICYDB_CAP_MEMFD_CLASS: usize;
    pub static POLICYDB_CAP_BPF_TOKEN_PERMS: usize;
    pub static EIDRM: c_int;
    pub static ENOENT: c_int;
}

/* __randomize_layout in C requests randomized layout where supported. */
#[repr(C)]
pub struct selinux_state {
    #[cfg(CONFIG_SECURITY_SELINUX_DEVELOP)]
    pub enforcing: bool,
    pub initialized: bool,
    pub policycap: [bool; __POLICYDB_CAP_MAX],

    pub status_page: *mut page,
    pub status_lock: mutex,

    /* __rcu */
    pub policy: *mut selinux_policy,
    pub policy_mutex: mutex,
}

extern "C" {
    pub fn selinux_avc_init();
    pub static mut selinux_state: selinux_state;
}

#[inline]
pub unsafe fn selinux_initialized() -> bool {
    /* do a synchronized load to avoid race conditions */
    core::ptr::read_volatile(core::ptr::addr_of!(selinux_state.initialized))
}

#[inline]
pub unsafe fn selinux_mark_initialized() {
    /* do a synchronized write to avoid race conditions */
    core::ptr::write_volatile(core::ptr::addr_of_mut!(selinux_state.initialized), true);
}

#[cfg(CONFIG_SECURITY_SELINUX_DEVELOP)]
#[inline]
pub unsafe fn enforcing_enabled() -> bool {
    core::ptr::read_volatile(core::ptr::addr_of!(selinux_state.enforcing))
}

#[cfg(CONFIG_SECURITY_SELINUX_DEVELOP)]
#[inline]
pub unsafe fn enforcing_set(value: bool) {
    core::ptr::write_volatile(core::ptr::addr_of_mut!(selinux_state.enforcing), value);
}

#[cfg(not(CONFIG_SECURITY_SELINUX_DEVELOP))]
#[inline]
pub fn enforcing_enabled() -> bool {
    true
}

#[cfg(not(CONFIG_SECURITY_SELINUX_DEVELOP))]
#[inline]
pub fn enforcing_set(_value: bool) {}

#[inline]
pub fn checkreqprot_get() -> bool {
    /* non-zero/true checkreqprot values are no longer supported */
    false
}

#[inline]
unsafe fn read_policycap(cap: usize) -> bool {
    core::ptr::read_volatile(core::ptr::addr_of!(selinux_state.policycap[cap]))
}

#[inline]
pub unsafe fn selinux_policycap_netpeer() -> bool {
    read_policycap(POLICYDB_CAP_NETPEER)
}

#[inline]
pub unsafe fn selinux_policycap_openperm() -> bool {
    read_policycap(POLICYDB_CAP_OPENPERM)
}

#[inline]
pub unsafe fn selinux_policycap_extsockclass() -> bool {
    read_policycap(POLICYDB_CAP_EXTSOCKCLASS)
}

#[inline]
pub unsafe fn selinux_policycap_alwaysnetwork() -> bool {
    read_policycap(POLICYDB_CAP_ALWAYSNETWORK)
}

#[inline]
pub unsafe fn selinux_policycap_cgroupseclabel() -> bool {
    read_policycap(POLICYDB_CAP_CGROUPSECLABEL)
}

#[inline]
pub unsafe fn selinux_policycap_nnp_nosuid_transition() -> bool {
    read_policycap(POLICYDB_CAP_NNP_NOSUID_TRANSITION)
}

#[inline]
pub unsafe fn selinux_policycap_genfs_seclabel_symlinks() -> bool {
    read_policycap(POLICYDB_CAP_GENFS_SECLABEL_SYMLINKS)
}

#[inline]
pub unsafe fn selinux_policycap_ioctl_skip_cloexec() -> bool {
    read_policycap(POLICYDB_CAP_IOCTL_SKIP_CLOEXEC)
}

#[inline]
pub unsafe fn selinux_policycap_userspace_initial_context() -> bool {
    read_policycap(POLICYDB_CAP_USERSPACE_INITIAL_CONTEXT)
}

#[inline]
pub unsafe fn selinux_policycap_netlink_xperm() -> bool {
    read_policycap(POLICYDB_CAP_NETLINK_XPERM)
}

#[inline]
pub unsafe fn selinux_policycap_functionfs_seclabel() -> bool {
    read_policycap(POLICYDB_CAP_FUNCTIONFS_SECLABEL)
}

#[inline]
pub unsafe fn selinux_policycap_memfd_class() -> bool {
    read_policycap(POLICYDB_CAP_MEMFD_CLASS)
}

#[inline]
pub unsafe fn selinux_policycap_bpf_token_perms() -> bool {
    read_policycap(POLICYDB_CAP_BPF_TOKEN_PERMS)
}

#[repr(C)]
pub struct selinux_load_state {
    pub policy: *mut selinux_policy,
    pub convert_data: *mut selinux_policy_convert_data,
}

extern "C" {
    pub fn security_mls_enabled() -> c_int;
    pub fn security_load_policy(data: *mut c_void, len: usize, load_state: *mut selinux_load_state) -> c_int;
    pub fn selinux_policy_commit(load_state: *mut selinux_load_state);
    pub fn selinux_policy_cancel(load_state: *mut selinux_load_state);
    pub fn security_read_policy(data: *mut *mut c_void, len: *mut usize) -> c_int;
    pub fn security_read_state_kernel(data: *mut *mut c_void, len: *mut usize) -> c_int;
    pub fn security_policycap_supported(req_cap: u32) -> c_int;
}

/* Maximum supported number of permissions per class */
pub const SEL_VEC_MAX: c_int = 32;

#[repr(C)]
pub struct av_decision {
    pub allowed: u32,
    pub auditallow: u32,
    pub auditdeny: u32,
    pub seqno: u32,
    pub flags: u32,
}

pub const XPERMS_ALLOWED: c_int = 1;
pub const XPERMS_AUDITALLOW: c_int = 2;
pub const XPERMS_DONTAUDIT: c_int = 4;

#[inline]
pub unsafe fn security_xperm_set(perms: *mut u32, x: u32) {
    let slot = perms.add((x >> 5) as usize);
    *slot |= 1u32 << (x & 0x1f);
}

#[inline]
pub unsafe fn security_xperm_test(perms: *const u32, x: u32) -> u32 {
    1 & (*perms.add((x >> 5) as usize) >> (x & 0x1f))
}

#[repr(C)]
pub struct extended_perms_data {
    pub p: [u32; 8],
}

#[repr(C)]
pub struct extended_perms_decision {
    pub used: u8,
    pub driver: u8,
    pub base_perm: u8,
    pub allowed: *mut extended_perms_data,
    pub auditallow: *mut extended_perms_data,
    pub dontaudit: *mut extended_perms_data,
}

#[repr(C)]
pub struct extended_perms {
    pub len: u16, /* length associated decision chain */
    pub base_perms: u8, /* which base permissions are covered */
    pub drivers: extended_perms_data, /* flag drivers that are used */
}

/* definitions of av_decision.flags */
pub const AVD_FLAGS_PERMISSIVE: c_int = 0x0001;
pub const AVD_FLAGS_NEVERAUDIT: c_int = 0x0002;

extern "C" {
    pub fn security_compute_av(
        ssid: u32,
        tsid: u32,
        tclass: u16,
        avd: *mut av_decision,
        xperms: *mut extended_perms,
    );
    pub fn security_compute_xperms_decision(
        ssid: u32,
        tsid: u32,
        tclass: u16,
        driver: u8,
        base_perm: u8,
        xpermd: *mut extended_perms_decision,
    );
    pub fn security_compute_av_user(ssid: u32, tsid: u32, tclass: u16, avd: *mut av_decision);
    pub fn security_transition_sid(
        ssid: u32,
        tsid: u32,
        tclass: u16,
        qstr: *const qstr,
        out_sid: *mut u32,
    ) -> c_int;
    pub fn security_transition_sid_user(
        ssid: u32,
        tsid: u32,
        tclass: u16,
        objname: *const c_char,
        out_sid: *mut u32,
    ) -> c_int;
    pub fn security_member_sid(ssid: u32, tsid: u32, tclass: u16, out_sid: *mut u32) -> c_int;
    pub fn security_change_sid(ssid: u32, tsid: u32, tclass: u16, out_sid: *mut u32) -> c_int;
    pub fn security_sid_to_context(sid: u32, scontext: *mut *mut c_char, scontext_len: *mut u32) -> c_int;
    pub fn security_sid_to_context_force(sid: u32, scontext: *mut *mut c_char, scontext_len: *mut u32) -> c_int;
    pub fn security_sid_to_context_inval(sid: u32, scontext: *mut *mut c_char, scontext_len: *mut u32) -> c_int;
    pub fn security_context_to_sid(
        scontext: *const c_char,
        scontext_len: u32,
        out_sid: *mut u32,
        gfp: gfp_t,
    ) -> c_int;
    pub fn security_context_str_to_sid(scontext: *const c_char, out_sid: *mut u32, gfp: gfp_t) -> c_int;
    pub fn security_context_to_sid_default(
        scontext: *const c_char,
        scontext_len: u32,
        out_sid: *mut u32,
        def_sid: u32,
        gfp_flags: gfp_t,
    ) -> c_int;
    pub fn security_context_to_sid_force(scontext: *const c_char, scontext_len: u32, sid: *mut u32) -> c_int;
    pub fn security_port_sid(protocol: u8, port: u16, out_sid: *mut u32) -> c_int;
    pub fn security_ib_pkey_sid(subnet_prefix: u64, pkey_num: u16, out_sid: *mut u32) -> c_int;
    pub fn security_ib_endport_sid(dev_name: *const c_char, port_num: u8, out_sid: *mut u32) -> c_int;
    pub fn security_netif_sid(name: *const c_char, if_sid: *mut u32) -> c_int;
    pub fn security_node_sid(domain: u16, addr: *const c_void, addrlen: u32, out_sid: *mut u32) -> c_int;
    pub fn security_validate_transition(oldsid: u32, newsid: u32, tasksid: u32, tclass: u16) -> c_int;
    pub fn security_validate_transition_user(oldsid: u32, newsid: u32, tasksid: u32, tclass: u16) -> c_int;
    pub fn security_bounded_transition(old_sid: u32, new_sid: u32) -> c_int;
    pub fn security_sid_mls_copy(sid: u32, mls_sid: u32, new_sid: *mut u32) -> c_int;
    pub fn security_net_peersid_resolve(nlbl_sid: u32, nlbl_type: u32, xfrm_sid: u32, peer_sid: *mut u32) -> c_int;
    pub fn security_get_classes(policy: *mut selinux_policy, classes: *mut *mut *mut c_char, nclasses: *mut u32) -> c_int;
    pub fn security_get_permissions(
        policy: *mut selinux_policy,
        class: *const c_char,
        perms: *mut *mut *mut c_char,
        nperms: *mut u32,
    ) -> c_int;
    pub fn security_get_reject_unknown() -> c_int;
    pub fn security_get_allow_unknown() -> c_int;
}

pub const SECURITY_FS_USE_XATTR: c_int = 1; /* use xattr */
pub const SECURITY_FS_USE_TRANS: c_int = 2; /* use transition SIDs, e.g. devpts/tmpfs */
pub const SECURITY_FS_USE_TASK: c_int = 3; /* use task SIDs, e.g. pipefs/sockfs */
pub const SECURITY_FS_USE_GENFS: c_int = 4; /* use the genfs support */
pub const SECURITY_FS_USE_NONE: c_int = 5; /* no labeling support */
pub const SECURITY_FS_USE_MNTPOINT: c_int = 6; /* use mountpoint labeling */
pub const SECURITY_FS_USE_NATIVE: c_int = 7; /* use native label support */
pub const SECURITY_FS_USE_MAX: c_int = 7; /* Highest SECURITY_FS_USE_XXX */

extern "C" {
    pub fn security_fs_use(sb: *mut super_block) -> c_int;
    pub fn security_genfs_sid(fstype: *const c_char, path: *const c_char, sclass: u16, sid: *mut u32) -> c_int;
    pub fn selinux_policy_genfs_sid(
        policy: *mut selinux_policy,
        fstype: *const c_char,
        path: *const c_char,
        sclass: u16,
        sid: *mut u32,
    ) -> c_int;
}

/* CONFIG_NETLABEL: external declarations when enabled, inline error returns otherwise. */
#[cfg(CONFIG_NETLABEL)]
extern "C" {
    pub fn security_netlbl_secattr_to_sid(secattr: *mut netlbl_lsm_secattr, sid: *mut u32) -> c_int;
    pub fn security_netlbl_sid_to_secattr(sid: u32, secattr: *mut netlbl_lsm_secattr) -> c_int;
}

#[cfg(not(CONFIG_NETLABEL))]
#[inline]
pub unsafe fn security_netlbl_secattr_to_sid(_secattr: *mut netlbl_lsm_secattr, _sid: *mut u32) -> c_int {
    -EIDRM
}

#[cfg(not(CONFIG_NETLABEL))]
#[inline]
pub unsafe fn security_netlbl_sid_to_secattr(_sid: u32, _secattr: *mut netlbl_lsm_secattr) -> c_int {
    -ENOENT
}

extern "C" {
    pub fn security_get_initial_sid_context(sid: u32) -> *const c_char;
}

/*
 * status notifier using mmap interface
 */
extern "C" {
    pub fn selinux_kernel_status_page() -> *mut page;
}

pub const SELINUX_KERNEL_STATUS_VERSION: c_int = 1;

#[repr(C, packed)]
pub struct selinux_kernel_status {
    pub version: u32, /* version number of the structure */
    pub sequence: u32, /* sequence number of seqlock logic */
    pub enforcing: u32, /* current setting of enforcing mode */
    pub policyload: u32, /* times of policy reloaded */
    pub deny_unknown: u32, /* current setting of deny_unknown */
    /*
     * The version > 0 supports above members.
     */
}

extern "C" {
    pub fn selinux_status_update_setenforce(enforcing: bool);
    pub fn selinux_status_update_policyload(seqno: u32);
    pub fn selinux_complete_init();
    pub static mut selinux_null: path;
    pub fn selnl_notify_setenforce(val: c_int);
    pub fn selnl_notify_policyload(seqno: u32);
    pub fn selinux_nlmsg_lookup(sclass: u16, nlmsg_type: u16, perm: *mut u32) -> c_int;

    pub fn avtab_cache_init();
    pub fn ebitmap_cache_init();
    pub fn hashtab_cache_init();
    pub fn security_sidtab_hash_stats(page: *mut c_char) -> c_int;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
