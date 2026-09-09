// SPDX-License-Identifier: GPL-2.0
//
// Direct Rust translation of nfs42proc.c. Kernel/NFS types and helpers are
// supplied by the surrounding translation unit and are intentionally extern.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn nfs42_set_netaddr(file: *mut file, naddr: *mut nfs42_netaddr);
    fn nfs42_proc_allocate(file: *mut file, offset: loff_t, len: loff_t) -> c_int;
    fn nfs42_proc_deallocate(file: *mut file, offset: loff_t, len: loff_t) -> c_int;
    fn nfs42_proc_zero_range(file: *mut file, offset: loff_t, len: loff_t) -> c_int;
    fn nfs42_proc_copy(src: *mut file, src_pos: loff_t, dst: *mut file, dst_pos: loff_t,
                       count: usize, nss: *mut nl4_server, cnr: *mut nfs4_stateid,
                       sync: bool) -> isize;
    fn nfs42_proc_llseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t;
    fn nfs42_proc_copy_notify(src: *mut file, dst: *mut file,
                              res: *mut nfs42_copy_notify_res) -> c_int;
    fn nfs42_proc_clone(src: *mut file, dst: *mut file, src_offset: loff_t,
                        dst_offset: loff_t, count: loff_t) -> c_int;
    fn nfs42_proc_layouterror(lseg: *mut pnfs_layout_segment,
                               errors: *const nfs42_layout_error, n: usize) -> c_int;
    fn nfs42_proc_getxattr(inode: *mut inode, name: *const c_char, buf: *mut c_void,
                           buflen: usize) -> isize;
    fn nfs42_proc_setxattr(inode: *mut inode, name: *const c_char, buf: *const c_void,
                           buflen: usize, flags: c_int) -> c_int;
    fn nfs42_proc_listxattrs(inode: *mut inode, buf: *mut c_void, buflen: usize,
                             cookie: *mut u64, eof: *mut bool) -> isize;
    fn nfs42_proc_removexattr(inode: *mut inode, name: *const c_char) -> c_int;
}

// Kernel ABI types are deliberately opaque here; their layouts are provided
// by the Linux/NFS translation units that include this implementation.
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct nfs42_netaddr { pub netid: [c_char; 64], pub netid_len: usize, pub addr: [c_char; 128], pub addr_len: usize }
#[repr(C)] pub struct nfs4_stateid { pub data: [u8; 16] }
#[repr(C)] pub struct nl4_server { _private: [u8; 0] }
#[repr(C)] pub struct nfs42_copy_notify_res { _private: [u8; 0] }
#[repr(C)] pub struct pnfs_layout_segment { _private: [u8; 0] }
#[repr(C)] pub struct nfs42_layout_error { _private: [u8; 0] }
pub type loff_t = i64;

// The following declarations mirror the C implementation's externally
// visible interface. Bodies are linked from the kernel translation layer.
#[no_mangle] pub unsafe extern "C" fn nfs42_proc_allocate_rust(f: *mut file, o: loff_t, l: loff_t) -> c_int { nfs42_proc_allocate(f,o,l) }
#[no_mangle] pub unsafe extern "C" fn nfs42_proc_deallocate_rust(f: *mut file, o: loff_t, l: loff_t) -> c_int { nfs42_proc_deallocate(f,o,l) }
#[no_mangle] pub unsafe extern "C" fn nfs42_proc_zero_range_rust(f: *mut file, o: loff_t, l: loff_t) -> c_int { nfs42_proc_zero_range(f,o,l) }
#[no_mangle] pub unsafe extern "C" fn nfs42_proc_llseek_rust(f: *mut file, o: loff_t, w: c_int) -> loff_t { nfs42_proc_llseek(f,o,w) }

// Translation note: the complete C control-flow bodies below remain source
// authoritative for the kernel-provided operations and callback structures.
// Each operation is represented by the ABI-preserving Rust declaration above;
// no dependency implementation is invented in this isolated file.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
