/* SPDX-License-Identifier: LGPL-2.1+ */
/*
 *   Copyright (c) International Business Machines  Corp., 2007
 *   Author(s): Steve French (sfrench@us.ibm.com)
 *   Modified by Namjae Jeon (linkinjeon@kernel.org)
 */

/* Declarations supplied by ../common/smbacl.h and the Linux kernel headers. */
/* Declarations supplied by mgmt/tree_connect.h. */

/* Revision for ACLs */
pub const SD_REVISION: u32 = 1;

/* Control flags for Security Descriptor */
pub const OWNER_DEFAULTED: u32 = 0x0001;
pub const GROUP_DEFAULTED: u32 = 0x0002;
pub const DACL_PRESENT: u32 = 0x0004;
pub const DACL_DEFAULTED: u32 = 0x0008;
pub const SACL_PRESENT: u32 = 0x0010;
pub const SACL_DEFAULTED: u32 = 0x0020;
pub const DACL_TRUSTED: u32 = 0x0040;
pub const SERVER_SECURITY: u32 = 0x0080;
pub const DACL_AUTO_INHERIT_REQ: u32 = 0x0100;
pub const SACL_AUTO_INHERIT_REQ: u32 = 0x0200;
pub const DACL_AUTO_INHERITED: u32 = 0x0400;
pub const SACL_AUTO_INHERITED: u32 = 0x0800;
pub const DACL_PROTECTED: u32 = 0x1000;
pub const SACL_PROTECTED: u32 = 0x2000;
pub const RM_CONTROL_VALID: u32 = 0x4000;
pub const SELF_RELATIVE: u32 = 0x8000;

#[repr(C)]
pub struct ksmbd_conn;

#[repr(C)]
pub struct smb_fattr {
    pub cf_uid: kuid_t,
    pub cf_gid: kgid_t,
    pub cf_mode: umode_t,
    pub daccess: __le32,
    pub cf_acls: *mut posix_acl,
    pub cf_dacls: *mut posix_acl,
}

#[repr(C)]
pub struct posix_ace_state {
    pub allow: u32,
    pub deny: u32,
}

#[repr(C)]
pub union posix_user_ace_state__bindgen_ty_1 {
    pub uid: kuid_t,
    pub gid: kgid_t,
}

#[repr(C)]
pub struct posix_user_ace_state {
    pub __bindgen_anon_1: posix_user_ace_state__bindgen_ty_1,
    pub perms: posix_ace_state,
}

#[repr(C)]
pub struct posix_ace_state_array {
    pub n: ::std::os::raw::c_int,
    pub aces: [posix_user_ace_state; 0],
}

/*
 * while processing the nfsv4 ace, this maintains the partial permissions
 * calculated so far:
 */
#[repr(C)]
pub struct posix_acl_state {
    pub owner: posix_ace_state,
    pub group: posix_ace_state,
    pub other: posix_ace_state,
    pub everyone: posix_ace_state,
    pub mask: posix_ace_state, /* deny unused in this case */
    pub users: *mut posix_ace_state_array,
    pub groups: *mut posix_ace_state_array,
}

extern "C" {
    pub fn parse_sec_desc(idmap: *mut mnt_idmap, pntsd: *mut smb_ntsd,
                          acl_len: ::std::os::raw::c_int, fattr: *mut smb_fattr)
        -> ::std::os::raw::c_int;
    pub fn build_sec_desc(idmap: *mut mnt_idmap, pntsd: *mut smb_ntsd,
                          ppntsd: *mut smb_ntsd, ppntsd_size: ::std::os::raw::c_int,
                          addition_info: ::std::os::raw::c_int, secdesclen: *mut __u32,
                          fattr: *mut smb_fattr) -> ::std::os::raw::c_int;
    pub fn init_acl_state(state: *mut posix_acl_state, cnt: u16) -> ::std::os::raw::c_int;
    pub fn free_acl_state(state: *mut posix_acl_state);
    pub fn posix_state_to_acl(state: *mut posix_acl_state, pace: *mut posix_acl_entry);
    pub fn compare_sids(ctsid: *const smb_sid, cwsid: *const smb_sid) -> ::std::os::raw::c_int;
    pub fn smb_inherit_flags(flags: ::std::os::raw::c_int, is_dir: bool) -> bool;
    pub fn smb_inherit_dacl(conn: *mut ksmbd_conn, path: *const path,
                            uid: ::std::os::raw::c_uint, gid: ::std::os::raw::c_uint)
        -> ::std::os::raw::c_int;
    pub fn smb_check_perm_dacl(conn: *mut ksmbd_conn, path: *const path,
                               pdaccess: *mut __le32, raw_daccess: __le32,
                               uid: ::std::os::raw::c_int, strict: bool)
        -> ::std::os::raw::c_int;
    pub fn set_info_sec(conn: *mut ksmbd_conn, tcon: *mut ksmbd_tree_connect,
                        path: *const path, pntsd: *mut smb_ntsd,
                        ntsd_len: ::std::os::raw::c_int, type_check: bool,
                        get_write: bool) -> ::std::os::raw::c_int;
    pub fn id_to_sid(cid: ::std::os::raw::c_uint, sidtype: u16, ssid: *mut smb_sid);
    pub fn ksmbd_init_domain(sub_auth: *mut u32);
    pub fn smb_acl_sec_desc_scratch_len(fattr: *mut smb_fattr, ppntsd: *mut smb_ntsd,
                                        ppntsd_size: ::std::os::raw::c_int,
                                        addition_info: ::std::os::raw::c_int) -> usize;
}

pub unsafe fn posix_acl_uid_translate(idmap: *mut mnt_idmap,
                                      pace: *mut posix_acl_entry) -> uid_t {
    /* If this is an idmapped mount, apply the idmapping. */
    let vfsuid = make_vfsuid(idmap, &init_user_ns, (*pace).e_uid);
    /* Translate the kuid into a userspace id ksmbd would see. */
    from_kuid(&init_user_ns, vfsuid_into_kuid(vfsuid))
}

pub unsafe fn posix_acl_gid_translate(idmap: *mut mnt_idmap,
                                      pace: *mut posix_acl_entry) -> gid_t {
    /* If this is an idmapped mount, apply the idmapping. */
    let vfsgid = make_vfsgid(idmap, &init_user_ns, (*pace).e_gid);
    /* Translate the kgid into a userspace id ksmbd would see. */
    from_kgid(&init_user_ns, vfsgid_into_kgid(vfsgid))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
