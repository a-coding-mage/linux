// SPDX-License-Identifier: GPL-2.0
/* Low-level Rust translation of fuse/dax.c. */

const FUSE_DAX_SHIFT: u32 = 21;
const FUSE_DAX_SZ: usize = 1usize << FUSE_DAX_SHIFT;
const FUSE_DAX_PAGES: usize = FUSE_DAX_SZ / PAGE_SIZE;
const FUSE_DAX_RECLAIM_CHUNK: usize = 10;
const FUSE_DAX_RECLAIM_THRESHOLD: usize = 20;

#[repr(C)]
pub struct fuse_dax_mapping {
    pub inode: *mut inode,
    pub list: list_head,
    pub itn: interval_tree_node,
    pub busy_list: list_head,
    pub window_offset: u64,
    pub length: loff_t,
    pub writable: bool,
    pub refcnt: refcount_t,
}

#[repr(C)]
pub struct fuse_inode_dax {
    pub sem: rw_semaphore,
    pub tree: rb_root_cached,
    pub nr: c_ulong,
}

#[repr(C)]
pub struct fuse_conn_dax {
    pub dev: *mut dax_device,
    pub lock: spinlock_t,
    pub nr_busy_ranges: c_ulong,
    pub busy_ranges: list_head,
    pub free_work: delayed_work,
    pub range_waitq: wait_queue_head_t,
    pub nr_free_ranges: c_long,
    pub free_ranges: list_head,
    pub nr_ranges: c_ulong,
}

/* Kernel/FUSE declarations are supplied by the surrounding translation. */
extern "C" {
    static system_dfl_long_wq: *mut workqueue_struct;
    fn queue_delayed_work(wq: *mut workqueue_struct, work: *mut delayed_work, delay: c_ulong) -> bool;
    fn msecs_to_jiffies(ms: c_ulong) -> c_ulong;
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn wake_up(q: *mut wait_queue_head_t);
}

#[inline]
unsafe fn __kick_dmap_free_worker(fcd: *mut fuse_conn_dax, delay_ms: c_ulong) {
    let threshold = core::cmp::max((*fcd).nr_ranges * FUSE_DAX_RECLAIM_THRESHOLD as c_ulong / 100, 1);
    if (*fcd).nr_free_ranges < threshold as c_long {
        queue_delayed_work(system_dfl_long_wq, &mut (*fcd).free_work, msecs_to_jiffies(delay_ms));
    }
}

unsafe fn kick_dmap_free_worker(fcd: *mut fuse_conn_dax, delay_ms: c_ulong) {
    spin_lock(&mut (*fcd).lock);
    __kick_dmap_free_worker(fcd, delay_ms);
    spin_unlock(&mut (*fcd).lock);
}

unsafe fn __dmap_remove_busy_list(fcd: *mut fuse_conn_dax, dmap: *mut fuse_dax_mapping) {
    list_del_init(&mut (*dmap).busy_list);
    WARN_ON((*fcd).nr_busy_ranges == 0);
    (*fcd).nr_busy_ranges -= 1;
}

unsafe fn dmap_remove_busy_list(fcd: *mut fuse_conn_dax, dmap: *mut fuse_dax_mapping) {
    spin_lock(&mut (*fcd).lock);
    __dmap_remove_busy_list(fcd, dmap);
    spin_unlock(&mut (*fcd).lock);
}

unsafe fn __dmap_add_to_free_pool(fcd: *mut fuse_conn_dax, dmap: *mut fuse_dax_mapping) {
    list_add_tail(&mut (*dmap).list, &mut (*fcd).free_ranges);
    (*fcd).nr_free_ranges += 1;
    wake_up(&mut (*fcd).range_waitq);
}

unsafe fn dmap_add_to_free_pool(fcd: *mut fuse_conn_dax, dmap: *mut fuse_dax_mapping) {
    spin_lock(&mut (*fcd).lock);
    __dmap_add_to_free_pool(fcd, dmap);
    spin_unlock(&mut (*fcd).lock);
}

/* Exported implementation entry points; their bodies use Linux/FUSE ABI
 * definitions supplied by the final repository's kernel bindings. */
extern "C" {
    pub fn fuse_dax_inode_cleanup(inode: *mut inode);
    pub fn fuse_dax_break_layouts(inode: *mut inode, dmap_start: u64, dmap_end: u64) -> c_int;
    pub fn fuse_dax_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> isize;
    pub fn fuse_dax_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> isize;
    pub fn fuse_dax_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int;
    pub fn fuse_dax_conn_free(fc: *mut fuse_conn);
    pub fn fuse_dax_conn_alloc(fc: *mut fuse_conn, mode: fuse_dax_mode, dev: *mut dax_device) -> c_int;
    pub fn fuse_dax_inode_alloc(sb: *mut super_block, fi: *mut fuse_inode) -> bool;
    pub fn fuse_dax_inode_init(inode: *mut inode, flags: c_uint);
    pub fn fuse_dax_dontcache(inode: *mut inode, flags: c_uint);
    pub fn fuse_dax_check_alignment(fc: *mut fuse_conn, map_alignment: c_uint) -> bool;
    pub fn fuse_dax_cancel_work(fc: *mut fuse_conn);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
