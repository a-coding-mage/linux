/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Translated from fs_context.h.  Included kernel declarations are external dependencies. */

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smb_version {
    Smb_1 = 1,
    Smb_20,
    Smb_21,
    Smb_30,
    Smb_302,
    Smb_311,
    Smb_3any,
    Smb_default,
    Smb_version_err,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cifs_cache_param { Opt_cache_loose, Opt_cache_strict, Opt_cache_none, Opt_cache_ro, Opt_cache_rw, Opt_cache_err }

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cifs_reparse_parm { Opt_reparse_default, Opt_reparse_none, Opt_reparse_nfs, Opt_reparse_wsl, Opt_reparse_err }

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cifs_symlink_parm {
    Opt_symlink_default, Opt_symlink_none, Opt_symlink_native, Opt_symlink_unix,
    Opt_symlink_mfsymlinks, Opt_symlink_sfu, Opt_symlink_nfs, Opt_symlink_wsl,
    Opt_symlink_err,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cifs_sec_param {
    Opt_sec_krb5, Opt_sec_krb5i, Opt_sec_krb5p, Opt_sec_ntlmsspi, Opt_sec_ntlmssp,
    Opt_sec_ntlmv2, Opt_sec_ntlmv2i, Opt_sec_none, Opt_sec_err,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cifs_upcall_target_param { Opt_upcall_target_mount, Opt_upcall_target_application, Opt_upcall_target_err }

/* The anonymous option enums retain C's sequential discriminants. */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cifs_param {
    Opt_user_xattr, Opt_forceuid, Opt_forcegid, Opt_noblocksend, Opt_noautotune, Opt_lease,
    Opt_nosparse, Opt_hard, Opt_soft, Opt_perm, Opt_nodelete, Opt_mapposix, Opt_mapchars,
    Opt_nomapchars, Opt_sfu, Opt_nodfs, Opt_posixpaths, Opt_unix, Opt_nocase, Opt_brl,
    Opt_handlecache, Opt_forcemandatorylock, Opt_setuidfromacl, Opt_setuids, Opt_dynperm,
    Opt_intr, Opt_strictsync, Opt_serverino, Opt_rwpidforward, Opt_cifsacl, Opt_acl,
    Opt_locallease, Opt_sign, Opt_ignore_signature, Opt_seal, Opt_noac, Opt_fsc,
    Opt_mfsymlinks, Opt_multiuser, Opt_sloppy, Opt_nosharesock, Opt_persistent, Opt_resilient,
    Opt_tcp_nodelay, Opt_domainauto, Opt_rdma, Opt_modesid, Opt_rootfs, Opt_multichannel,
    Opt_compress, Opt_witness, Opt_is_upcall_target_mount, Opt_is_upcall_target_application,
    Opt_unicode, Opt_backupuid, Opt_backupgid, Opt_uid, Opt_cruid, Opt_gid, Opt_port,
    Opt_file_mode, Opt_dirmode, Opt_min_enc_offload, Opt_retrans, Opt_blocksize, Opt_rasize,
    Opt_rsize, Opt_wsize, Opt_actimeo, Opt_acdirmax, Opt_acregmax, Opt_closetimeo,
    Opt_echo_interval, Opt_max_credits, Opt_max_cached_dirs, Opt_snapshot, Opt_max_channels,
    Opt_handletimeout, Opt_source, Opt_user, Opt_pass, Opt_pass2, Opt_ip, Opt_domain,
    Opt_srcaddr, Opt_iocharset, Opt_netbiosname, Opt_servern, Opt_nbsessinit, Opt_ver,
    Opt_vers, Opt_sec, Opt_cache, Opt_reparse, Opt_upcalltarget, Opt_nativesocket,
    Opt_symlink, Opt_symlinkroot, Opt_ignore, Opt_err,
}

#[repr(C)]
pub struct smb3_fs_context {
    pub forceuid_specified: bool, pub forcegid_specified: bool, pub uid_specified: bool,
    pub cruid_specified: bool, pub gid_specified: bool, pub sloppy: bool, pub got_ip: bool,
    pub got_version: bool, pub got_rsize: bool, pub got_wsize: bool, pub got_bsize: bool,
    pub port: u16,
    pub username: *mut ::std::os::raw::c_char, pub password: *mut ::std::os::raw::c_char,
    pub password2: *mut ::std::os::raw::c_char, pub domainname: *mut ::std::os::raw::c_char,
    pub source: *mut ::std::os::raw::c_char, pub server_hostname: *mut ::std::os::raw::c_char,
    pub UNC: *mut ::std::os::raw::c_char, pub nodename: *mut ::std::os::raw::c_char,
    pub workstation_name: [::std::os::raw::c_char; CIFS_MAX_WORKSTATION_LEN],
    pub iocharset: *mut ::std::os::raw::c_char,
    pub source_rfc1001_name: [::std::os::raw::c_char; RFC1001_NAME_LEN_WITH_NULL],
    pub target_rfc1001_name: [::std::os::raw::c_char; RFC1001_NAME_LEN_WITH_NULL],
    pub rfc1001_sessinit: i32,
    pub cred_uid: kuid_t, pub linux_uid: kuid_t, pub linux_gid: kgid_t,
    pub backupuid: kuid_t, pub backupgid: kgid_t, pub file_mode: umode_t, pub dir_mode: umode_t,
    pub sectype: securityEnum, pub upcall_target: upcall_target_enum, pub sign: bool,
    pub ignore_signature: bool, pub retry: bool, pub intr: bool, pub setuids: bool,
    pub setuidfromacl: bool, pub override_uid: bool, pub override_gid: bool, pub dynperm: bool,
    pub noperm: bool, pub nodelete: bool, pub mode_ace: bool, pub no_psx_acl: bool,
    pub cifs_acl: bool, pub backupuid_specified: bool, pub backupgid_specified: bool,
    pub no_xattr: bool, pub server_ino: bool, pub direct_io: bool, pub strict_io: bool,
    pub cache_ro: bool, pub cache_rw: bool, pub remap: bool, pub sfu_remap: bool,
    pub posix_paths: bool, pub no_linux_ext: bool, pub linux_ext: bool, pub sfu_emul: bool,
    pub nullauth: bool, pub nocase: bool, pub nobrl: bool, pub nohandlecache: bool,
    pub mand_lock: bool, pub seal: bool, pub nodfs: bool, pub local_lease: bool,
    pub noblocksnd: bool, pub noautotune: bool, pub nostrictsync: bool, pub no_lease: bool,
    pub no_sparse: bool, pub fsc: bool, pub mfsymlinks: bool, pub multiuser: bool,
    pub rwpidforward: bool, pub nosharesock: bool, pub persistent: bool, pub nopersistent: bool,
    pub resilient: bool, pub domainauto: bool, pub rdma: bool, pub multichannel: bool,
    pub multichannel_specified: bool, pub max_channels_specified: bool, pub use_client_guid: bool,
    pub client_guid: [u8; SMB2_CLIENT_GUID_SIZE], pub vol_rsize: u32, pub vol_wsize: u32,
    pub bsize: u32, pub rasize: u32, pub rsize: u32, pub wsize: u32, pub min_offload: u32,
    pub retrans: u32, pub sockopt_tcp_nodelay: bool, pub acregmax: ::std::os::raw::c_ulong,
    pub acdirmax: ::std::os::raw::c_ulong, pub closetimeo: ::std::os::raw::c_ulong,
    pub ops: *mut smb_version_operations, pub vals: *mut smb_version_values,
    pub prepath: *mut ::std::os::raw::c_char, pub dstaddr: sockaddr_storage,
    pub srcaddr: sockaddr_storage, pub local_nls: *mut nls_table, pub echo_interval: u32,
    pub snapshot_time: u64, pub handle_timeout: u32, pub max_credits: u32,
    pub max_channels: u32, pub max_cached_dirs: u32, pub compress: bool, pub rootfs: bool,
    pub witness: bool, pub unicode: i32, pub leaf_fullpath: *mut ::std::os::raw::c_char,
    pub dfs_root_ses: *mut cifs_ses, pub dfs_automount: bool, pub reparse_type: cifs_reparse_type,
    pub symlink_type: cifs_symlink_type, pub nonativesocket: bool, pub dfs_conn: bool,
    pub dns_dom: *mut ::std::os::raw::c_char, pub symlinkroot: *mut ::std::os::raw::c_char,
}

extern "C" {
    pub static smb3_fs_parameters: fs_parameter_spec;
    pub fn smb3_init_fs_context(fc: *mut fs_context) -> i32;
    pub fn smb3_cleanup_fs_context_contents(ctx: *mut smb3_fs_context);
    pub fn smb3_cleanup_fs_context(ctx: *mut smb3_fs_context);
    pub fn smb3_fs_context_dup(new_ctx: *mut smb3_fs_context, ctx: *mut smb3_fs_context) -> i32;
    pub fn smb3_sync_session_ctx_passwords(cifs_sb: *mut cifs_sb_info, ses: *mut cifs_ses) -> i32;
    pub fn smb3_update_mnt_flags(cifs_sb: *mut cifs_sb_info) -> u32;
    pub fn cifs_sanitize_prepath(prepath: *mut ::std::os::raw::c_char, gfp: gfp_t) -> *mut ::std::os::raw::c_char;
    pub static mut cifs_mount_mutex: mutex;
}

pub const SMB3_MAX_DCLOSETIMEO: i32 = 1 << 30;
pub const SMB3_DEF_DCLOSETIMEO: i32 = 1 * HZ;
pub const MAX_CACHED_FIDS: i32 = 16;

/* External kernel types/constants are supplied by the translated dependency headers. */

#[inline]
pub unsafe fn smb3_fc2context(fc: *const fs_context) -> *mut ::std::ffi::c_void {
    (*fc).fs_private
}

#[inline]
pub unsafe fn cifs_symlink_type(cifs_sb: *mut cifs_sb_info) -> cifs_symlink_type {
    let posix = (*cifs_sb_master_tcon(cifs_sb)).posix_extensions;
    let ctx = (*cifs_sb).ctx;
    if (*ctx).symlink_type != CIFS_SYMLINK_TYPE_DEFAULT { return (*ctx).symlink_type; }
    if (*ctx).mfsymlinks { return CIFS_SYMLINK_TYPE_MFSYMLINKS; }
    if (*ctx).sfu_emul { return CIFS_SYMLINK_TYPE_SFU; }
    if (*ctx).linux_ext && !(*ctx).no_linux_ext {
        return if posix { CIFS_SYMLINK_TYPE_NATIVE } else { CIFS_SYMLINK_TYPE_UNIX };
    }
    if (*ctx).reparse_type != CIFS_REPARSE_TYPE_NONE { return CIFS_SYMLINK_TYPE_NATIVE; }
    CIFS_SYMLINK_TYPE_NONE
}

#[inline]
pub unsafe fn cifs_mount_lock() { mutex_lock(&raw mut cifs_mount_mutex); }

#[inline]
pub unsafe fn cifs_mount_unlock() { mutex_unlock(&raw mut cifs_mount_mutex); }

#[inline]
pub unsafe fn cifs_io_align(fc: *mut fs_context, name: *const ::std::os::raw::c_char, mut size: usize) -> usize {
    if size == 0 || size % PAGE_SIZE != 0 {
        cifs_errorf!(fc, "unaligned %s, making it a multiple of %lu bytes\n", name, PAGE_SIZE);
        size = std::cmp::max(size - (size % PAGE_SIZE), PAGE_SIZE);
    }
    size
}

#[macro_export]
macro_rules! cifs_errorf {
    ($fc:expr, $fmt:expr $(, $arg:expr)*) => {{
        unsafe { errorf($fc, $fmt $(, $arg)*); cifs_dbg(VFS, $fmt $(, $arg)*); }
    }};
}

#[inline]
pub unsafe fn cifs_negotiate_rsize(server: *mut TCP_Server_Info, ctx: *mut smb3_fs_context, tcon: *mut cifs_tcon) {
    let mut size = std::cmp::max((*(*server).ops).negotiate_rsize(tcon, ctx), PAGE_SIZE);
    if (*ctx).rsize != 0 { size = std::cmp::max(std::cmp::min((*ctx).rsize, size), PAGE_SIZE); }
    (*ctx).rsize = size - (size % PAGE_SIZE);
}

#[inline]
pub unsafe fn cifs_negotiate_wsize(server: *mut TCP_Server_Info, ctx: *mut smb3_fs_context, tcon: *mut cifs_tcon) {
    let mut size = std::cmp::max((*(*server).ops).negotiate_wsize(tcon, ctx), PAGE_SIZE);
    if (*ctx).wsize != 0 { size = std::cmp::max(std::cmp::min((*ctx).wsize, size), PAGE_SIZE); }
    (*ctx).wsize = size - (size % PAGE_SIZE);
}

#[inline]
pub unsafe fn cifs_negotiate_iosize(server: *mut TCP_Server_Info, ctx: *mut smb3_fs_context, tcon: *mut cifs_tcon) {
    cifs_negotiate_rsize(server, ctx, tcon);
    cifs_negotiate_wsize(server, ctx, tcon);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
