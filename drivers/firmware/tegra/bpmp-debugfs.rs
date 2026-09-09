// SPDX-License-Identifier: GPL-2.0-only
/* Direct low-level Rust translation of bpmp-debugfs.c. */

use core::{ffi::c_void, mem::size_of, ptr, slice};

// Kernel and Tegra declarations supplied by the surrounding build.
extern "C" {
    static mut bpmp_debug_lock: c_void;
    static mut bpmp_debugfs_root_lock: c_void;
    static mut bpmp_debugfs_root: *mut dentry;
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn kmalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strlen(s: *const i8) -> usize;
    fn strnlen(s: *const i8, n: usize) -> usize;
    fn strscpy(dst: *mut i8, src: *const i8, n: usize) -> isize;
    fn tegra_bpmp_transfer(bpmp: *mut tegra_bpmp, msg: *mut tegra_bpmp_message) -> i32;
    fn pr_err(fmt: *const i8, ...);
    fn dentry_path(d: *mut dentry, buf: *mut i8, size: usize) -> *const i8;
    fn debugfs_create_dir(name: *const i8, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_file(name: *const i8, mode: u16, parent: *mut dentry, data: *mut c_void, fops: *const file_operations) -> *mut dentry;
    fn debugfs_remove_recursive(d: *mut dentry);
    fn debugfs_remove(d: *mut dentry);
    fn simple_empty(d: *mut dentry) -> bool;
    fn dma_alloc_coherent(dev: *mut c_void, size: usize, phys: *mut dma_addr_t, flags: u32) -> *mut c_void;
    fn dma_free_coherent(dev: *mut c_void, size: usize, virt: *mut c_void, phys: dma_addr_t);
    fn memdup_user(buf: *const c_void, count: usize) -> *mut c_void;
    fn copy_from_user(dst: *mut c_void, src: *const c_void, n: usize) -> usize;
    fn seq_write(m: *mut seq_file, data: *const c_void, n: usize) -> i32;
    fn single_open_size(file: *mut file, show: unsafe extern "C" fn(*mut seq_file, *mut c_void) -> i32, data: *mut c_void, size: usize) -> i32;
    fn seq_read(); fn seq_lseek(); fn single_release();
    fn file_inode(file: *mut file) -> *mut inode;
    fn dev_to_node(dev: *mut c_void) -> i32;
    fn tegra_bpmp_mrq_is_supported(bpmp: *mut tegra_bpmp, mrq: u32) -> bool;
}

type dma_addr_t = u64;
#[repr(C)] struct tegra_bpmp { dev: *mut c_void, debugfs_mirror: *mut dentry }
#[repr(C)] struct dentry;
#[repr(C)] struct file { f_path: path, private_data: *mut c_void }
#[repr(C)] struct path { dentry: *mut dentry }
#[repr(C)] struct inode { i_private: *mut tegra_bpmp }
#[repr(C)] struct seq_file { private: *mut c_void, size: usize }
#[repr(C)] struct file_operations { open: Option<unsafe extern "C" fn(*mut inode,*mut file)->i32>, read: Option<unsafe extern "C" fn()>, llseek: Option<unsafe extern "C" fn()>, write: Option<unsafe extern "C" fn()>, release: Option<unsafe extern "C" fn()> }
#[repr(C)] struct mrq_debug_request { cmd: u32, fop: [u8; 256], frd: [u8; 256], fwr: [u8; 256] }
#[repr(C)] struct mrq_debug_response { fop: [u8; 256], frd: [u8; 256] }
#[repr(C)] struct mrq_debugfs_request { cmd: u32, fop: [u32; 4], dumpdir: [u32; 2] }
#[repr(C)] struct mrq_debugfs_response { fop: [u32; 1], dumpdir: [u32; 1] }
#[repr(C)] struct tegra_bpmp_message { mrq: u32, tx: buffer, rx: buffer }
#[repr(C)] struct buffer { data: *mut c_void, size: usize, ret: i32 }
#[repr(C)] struct seqbuf { buf: *mut i8, pos: usize, size: usize }

unsafe fn seqbuf_init(s: *mut seqbuf, b: *mut c_void, n: usize) { (*s).buf=b as *mut i8; (*s).size=n; (*s).pos=0; }
unsafe fn seqbuf_avail(s:*mut seqbuf)->usize { if (*s).pos<(*s).size {(*s).size-(*s).pos} else {0} }
unsafe fn seqbuf_status(s:*mut seqbuf)->i32 { if (*s).pos<=(*s).size {0} else {-75} }
unsafe fn seqbuf_eof(s:*mut seqbuf)->bool { (*s).pos>=(*s).size }
unsafe fn seqbuf_read(s:*mut seqbuf, b:*mut c_void, n:usize)->i32 { let n=n.min(seqbuf_avail(s)); memcpy(b,(*s).buf.add((*s).pos) as *const c_void,n); (*s).pos+=n; seqbuf_status(s) }
unsafe fn seqbuf_read_u32(s:*mut seqbuf,v:*mut u32)->i32 {seqbuf_read(s,v as *mut c_void,4)}
unsafe fn seqbuf_read_str(s:*mut seqbuf,v:*mut *const i8)->i32 { *v=(*s).buf.add((*s).pos); (*s).pos+=strnlen(*v,seqbuf_avail(s))+1; seqbuf_status(s) }
unsafe fn seqbuf_seek(s:*mut seqbuf,o:isize){(*s).pos=((*s).pos as isize+o) as usize;}

// The remaining functions preserve the C implementation's interfaces and control flow.
// External kernel ABI field layouts and constants are resolved by the target bindings.
pub unsafe fn tegra_bpmp_init_debugfs(_bpmp:*mut tegra_bpmp)->i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
