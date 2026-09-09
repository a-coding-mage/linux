/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of nfs/internal.h. Included kernel types and constants are external dependencies. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_void};

pub const NFS_SB_MASK: u32 = SB_RDONLY | SB_NOSUID | SB_NODEV | SB_NOEXEC | SB_SYNCHRONOUS;
pub const NFS_MAX_SECFLAVORS: u32 = 12;
pub const NFS_UNSPEC_PORT: c_int = -1;
pub const NFS_UNSPEC_RETRANS: u32 = u32::MAX;
pub const NFS_UNSPEC_TIMEO: u32 = u32::MAX;

pub enum export_operations {}
pub enum nfs_string {}
pub enum nfs_pageio_descriptor {}
pub enum super_block {}
pub enum nfs_fattr {}
pub enum dentry {}
pub enum fs_context {}
pub enum nfs_subversion {}
pub enum net {}
pub enum rpc_timeout {}
pub enum cred {}
pub enum xprtsec_parms {}
pub enum nfs_auth_info {}
pub enum nfs_fh {}
pub enum nfs_server {}
pub enum nfs_client {}
pub enum rpc_program {}
pub enum rpc_clnt {}
pub enum inode {}
pub enum nfs4_sessionid {}
pub enum nfs_pgio_header {}
pub enum nfs_rw_ops {}
pub enum nfs_pgio_mirror {}
pub enum nfs_open_context {}
pub enum rpc_procinfo {}
pub enum xdr_stream {}
pub enum nfs_entry {}
pub enum nfs4_label {}
pub enum nfs_inode {}
pub enum shrinker {}
pub enum shrink_control {}
pub enum mnt_idmap {}
pub enum file {}
pub enum file_lock {}
pub enum address_space {}
pub enum workqueue_struct {}
pub enum writeback_control {}
pub enum wait_bit_key {}
pub enum file_kattr {}
pub enum work_struct {}
pub enum nfs_file_localio {}
pub enum nfsd_file {}
pub enum nfs_commit_data {}
pub enum rpc_call_ops {}
pub enum super_operations {}
pub enum kiocb {}
pub enum iov_iter {}
pub enum pipe_inode_info {}
pub enum vm_area_desc {}
pub enum nfs_page {}
pub enum nfs_lock_context {}
pub enum nfs_pgio_ops {}
pub enum svc_version {}
pub enum file_system_type {}

pub type fmode_t = u32;
pub type rpc_authflavor_t = u32;
pub type loff_t = i64;
pub type umode_t = u16;
pub type dev_t = u64;
pub type gfp_t = u32;
pub type ssize_t = isize;
pub type size_t = usize;
pub type u32 = core::ffi::c_uint;

#[repr(C)] pub struct sockaddr { pub _opaque: [u8; 0] }
#[repr(C)] pub struct sockaddr_storage { pub _opaque: [u8; 128] }

#[repr(C)]
pub struct nfs_client_initdata {
    pub init_flags: usize, pub hostname: *const c_char, pub addr: *const sockaddr_storage,
    pub nodename: *const c_char, pub ip_addr: *const c_char, pub addrlen: size_t,
    pub nfs_mod: *mut nfs_subversion, pub proto: c_int, pub minorversion: u32,
    pub nconnect: u32, pub max_connect: u32, pub net: *mut net,
    pub timeparms: *const rpc_timeout, pub cred: *const cred, pub xprtsec: *mut xprtsec_parms,
    pub connect_timeout: usize, pub reconnect_timeout: usize,
}

#[repr(C)]
pub struct nfs_fs_context {
    pub internal: bool, pub skip_reconfig_option_check: bool, pub need_mount: bool, pub sloppy: bool,
    pub flags: u32, pub rsize: u32, pub wsize: u32, pub timeo: u32, pub retrans: u32,
    pub acregmin: u32, pub acregmax: u32, pub acdirmin: u32, pub acdirmax: u32, pub namlen: u32,
    pub options: u32, pub bsize: u32, pub auth_info: nfs_auth_info, pub selected_flavor: rpc_authflavor_t,
    pub xprtsec: xprtsec_parms, pub client_address: *mut c_char, pub version: u32, pub minorversion: u32,
    pub fscache_uniq: *mut c_char, pub protofamily: u16, pub mountfamily: u16,
    pub has_sec_mnt_opts: bool, pub lock_status: c_int,
    pub mount_server: nfs_mount_server, pub nfs_server: nfs_server_address,
    pub mntfh: *mut nfs_fh, pub server: *mut nfs_server, pub nfs_mod: *mut nfs_subversion,
    pub clone_data: nfs_clone_mount,
}
#[repr(C)] pub union nfs_sockaddr { pub address: sockaddr, pub _address: sockaddr_storage }
#[repr(C)] pub struct nfs_mount_server { pub addr: nfs_sockaddr, pub addrlen: size_t, pub hostname: *mut c_char, pub version: u32, pub port: c_int, pub protocol: u16 }
#[repr(C)] pub struct nfs_server_address { pub addr: nfs_sockaddr, pub addrlen: size_t, pub hostname: *mut c_char, pub export_path: *mut c_char, pub port: c_int, pub protocol: u16, pub nconnect: u16, pub max_connect: u16, pub export_path_len: u16 }
#[repr(C)] pub struct nfs_clone_mount { pub sb: *mut super_block, pub dentry: *mut dentry, pub fattr: *mut nfs_fattr }

#[repr(C)] pub enum nfs_lock_status { NFS_LOCK_NOT_SET=0, NFS_LOCK_LOCK=1, NFS_LOCK_NOLOCK=2 }
#[repr(C)] pub struct nfs_mount_request { pub sap:*mut sockaddr_storage, pub salen:size_t, pub hostname:*mut c_char, pub dirpath:*mut c_char, pub version:u32, pub protocol:u16, pub fh:*mut nfs_fh, pub noresvport:c_int, pub auth_flav_len:*mut u32, pub auth_flavs:*mut rpc_authflavor_t, pub net:*mut net }
#[repr(C)] pub struct nfs_local_dio { pub mem_align:u32, pub offset_align:u32, pub middle_offset:loff_t, pub end_offset:loff_t, pub start_len:ssize_t, pub middle_len:ssize_t, pub end_len:ssize_t }

pub const CONFIG_PROC_FS: bool = false;
pub const CONFIG_NFS_V4: bool = true;

/* C inline helpers and variadic logging macros retain their semantics through these declarations. */
extern "C" {
    pub static nfs_export_ops: export_operations;
    pub fn nfs_mount(info: *mut nfs_mount_request, timeo:c_int, retrans:c_int) -> c_int;
    pub fn nfs_attr_check_mountpoint(parent:*mut super_block, fattr:*mut nfs_fattr);
    pub fn nfs_attr_use_mounted_on_fileid(fattr:*mut nfs_fattr) -> c_int;
    pub fn nfs_lookup_is_soft_revalidate(dentry:*const dentry) -> bool;
    pub fn flags_to_mode(flags:c_int) -> fmode_t;
    pub fn nfs_fc2context(fc:*const fs_context) -> *mut c_void;
    pub fn nfs_match_open_context(ctx1:*const nfs_open_context, ctx2:*const nfs_open_context) -> bool;
    pub fn nfs_access_xattr_mask(server:*const nfs_server) -> u32;
    pub fn nfs4_label_alloc(server:*mut nfs_server, flags:gfp_t) -> *mut nfs4_label;
    pub fn nfs4_label_copy(dst:*mut nfs4_label, src:*mut nfs4_label) -> *mut nfs4_label;
    pub fn nfs_zap_label_cache_locked(nfsi:*mut nfs_inode);
    pub fn nfs_local_probe_async(clp:*mut nfs_client);
    pub fn nfs_local_open_fh(clp:*mut nfs_client, cred:*const cred, fh:*mut nfs_fh, nfl:*mut nfs_file_localio, mode:fmode_t) -> *mut nfsd_file;
    pub fn nfs_local_doio(clp:*mut nfs_client, localio:*mut nfsd_file, hdr:*mut nfs_pgio_header, ops:*const rpc_call_ops) -> c_int;
    pub fn nfs_local_commit(localio:*mut nfsd_file, data:*mut nfs_commit_data, ops:*const rpc_call_ops) -> c_int;
    pub fn nfs_server_is_local(clp:*const nfs_client) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
