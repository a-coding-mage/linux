// SPDX-License-Identifier: GPL-2.0
/*
 * I/O and data path helper functionality.
 *
 * Borrowed from NFS Copyright (c) 2016 Trond Myklebust
 */

use core::ptr;

// Linux kernel declarations supplied by the surrounding netfs implementation.
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct task_struct;
#[repr(C)]
pub struct address_space {
    pub nrpages: usize,
}
#[repr(C)]
pub struct rw_semaphore;
#[repr(C)]
pub struct spinlock;

#[repr(C)]
pub struct inode {
    pub i_rwsem: rw_semaphore,
    pub i_mapping: *mut address_space,
}

#[repr(C)]
pub struct netfs_inode {
    pub inode: inode,
    pub flags: usize,
    pub lock: spinlock,
    pub wb_queue: list_head,
}

#[repr(C)]
struct netfs_wb_waiter {
    link: list_head, // Link in ictx->wb_queue
    waiter: *mut task_struct, // Waiter task; cleared when lock granted
}

extern "C" {
    fn inode_dio_finished(inode: *mut inode) -> bool;
    fn inode_dio_wait_interruptible(inode: *mut inode);
    fn down_read_interruptible(sem: *mut rw_semaphore) -> i32;
    fn down_write_killable(sem: *mut rw_semaphore) -> i32;
    fn up_read(sem: *mut rw_semaphore);
    fn up_write(sem: *mut rw_semaphore);
    fn downgrade_write(sem: *mut rw_semaphore);
    fn unmap_mapping_range(mapping: *mut address_space, start: u64, len: u64, even_cows: i32);
    fn filemap_fdatawait(mapping: *mut address_space) -> i32;
    fn spin_lock(lock: *mut spinlock);
    fn spin_unlock(lock: *mut spinlock);
    fn get_task_struct(task: *mut task_struct);
    fn put_task_struct(task: *mut task_struct);
    fn schedule();
    fn wake_up_process(task: *mut task_struct);
    fn current_task() -> *mut task_struct;
    fn netfs_inode(inode: *mut inode) -> *mut netfs_inode;
    fn netfs_stat(counter: *mut usize);
    static mut netfs_n_wb_lock_skip: usize;
    static mut netfs_n_wb_lock_wait: usize;
}

const NETFS_ICTX_ODIRECT: usize = 0;
const NETFS_ICTX_WB_LOCK: usize = 1;
const ERESTARTSYS: i32 = 512;
const TASK_UNINTERRUPTIBLE: i32 = 2;
const TASK_RUNNING: i32 = 0;

unsafe fn test_bit(bit: usize, flags: *const usize) -> bool {
    (*flags & (1usize << bit)) != 0
}

unsafe fn set_bit(bit: usize, flags: *mut usize) {
    *flags |= 1usize << bit;
}

unsafe fn clear_bit(bit: usize, flags: *mut usize) {
    *flags &= !(1usize << bit);
}

unsafe fn test_and_set_bit_lock(bit: usize, flags: *mut usize) -> bool {
    let old = test_bit(bit, flags);
    set_bit(bit, flags);
    old
}

unsafe fn clear_bit_unlock(bit: usize, flags: *mut usize) {
    clear_bit(bit, flags);
}

unsafe fn netfs_inode_dio_wait_interruptible(inode: *mut inode) -> i32 {
    if inode_dio_finished(inode) { return 0; }
    inode_dio_wait_interruptible(inode);
    if !inode_dio_finished(inode) { -ERESTARTSYS } else { 0 }
}

// Call with exclusively locked inode->i_rwsem.
unsafe fn netfs_block_o_direct(ictx: *mut netfs_inode) -> i32 {
    if !test_bit(NETFS_ICTX_ODIRECT, &(*ictx).flags) { return 0; }
    clear_bit(NETFS_ICTX_ODIRECT, &mut (*ictx).flags);
    netfs_inode_dio_wait_interruptible(&mut (*ictx).inode)
}

pub unsafe extern "C" fn netfs_start_io_read(inode: *mut inode) -> i32 {
    let ictx = netfs_inode(inode);
    if down_read_interruptible(&mut (*inode).i_rwsem) < 0 { return -ERESTARTSYS; }
    if !test_bit(NETFS_ICTX_ODIRECT, &(*ictx).flags) { return 0; }
    up_read(&mut (*inode).i_rwsem);
    if down_write_killable(&mut (*inode).i_rwsem) < 0 { return -ERESTARTSYS; }
    if netfs_block_o_direct(ictx) < 0 { up_write(&mut (*inode).i_rwsem); return -ERESTARTSYS; }
    downgrade_write(&mut (*inode).i_rwsem);
    0
}

pub unsafe extern "C" fn netfs_end_io_read(inode: *mut inode) { up_read(&mut (*inode).i_rwsem); }

pub unsafe extern "C" fn netfs_start_io_write(inode: *mut inode) -> i32 {
    let ictx = netfs_inode(inode);
    if down_write_killable(&mut (*inode).i_rwsem) < 0 { return -ERESTARTSYS; }
    if netfs_block_o_direct(ictx) < 0 { up_write(&mut (*inode).i_rwsem); return -ERESTARTSYS; }
    downgrade_write(&mut (*inode).i_rwsem);
    0
}

pub unsafe extern "C" fn netfs_end_io_write(inode: *mut inode) { up_read(&mut (*inode).i_rwsem); }

unsafe fn netfs_block_buffered(inode: *mut inode) -> i32 {
    let ictx = netfs_inode(inode);
    if !test_bit(NETFS_ICTX_ODIRECT, &(*ictx).flags) {
        set_bit(NETFS_ICTX_ODIRECT, &mut (*ictx).flags);
        if (*(*inode).i_mapping).nrpages != 0 {
            unmap_mapping_range((*inode).i_mapping, 0, 0, 0);
            let ret = filemap_fdatawait((*inode).i_mapping);
            if ret < 0 { clear_bit(NETFS_ICTX_ODIRECT, &mut (*ictx).flags); return ret; }
        }
    }
    0
}

pub unsafe extern "C" fn netfs_start_io_direct(inode: *mut inode) -> i32 {
    if down_read_interruptible(&mut (*inode).i_rwsem) < 0 { return -ERESTARTSYS; }
    let ictx = netfs_inode(inode);
    if test_bit(NETFS_ICTX_ODIRECT, &(*ictx).flags) { return 0; }
    up_read(&mut (*inode).i_rwsem);
    if down_write_killable(&mut (*inode).i_rwsem) < 0 { return -ERESTARTSYS; }
    let ret = netfs_block_buffered(inode);
    if ret < 0 { up_write(&mut (*inode).i_rwsem); return ret; }
    downgrade_write(&mut (*inode).i_rwsem);
    0
}

pub unsafe extern "C" fn netfs_end_io_direct(inode: *mut inode) { up_read(&mut (*inode).i_rwsem); }

unsafe fn netfs_wb_begin_wait(ictx: *mut netfs_inode) -> bool {
    let mut waiter: netfs_wb_waiter = core::mem::zeroed();
    let tsk = current_task();
    let mut got = false;
    spin_lock(&mut (*ictx).lock);
    if test_and_set_bit_lock(NETFS_ICTX_WB_LOCK, &mut (*ictx).flags) {
        get_task_struct(tsk);
        waiter.waiter = tsk;
        // list_add_tail(&waiter.link, &ictx->wb_queue)
        waiter.link.prev = (*ictx).wb_queue.prev;
        waiter.link.next = &mut (*ictx).wb_queue;
        (*ictx).wb_queue.prev = &mut waiter.link;
        got = false;
    } else { got = true; }
    spin_unlock(&mut (*ictx).lock);
    if !got {
        loop {
            // set_current_state(TASK_UNINTERRUPTIBLE)
            if ptr::read_volatile(&waiter.waiter).is_null() { break; }
            schedule();
        }
    }
    // __set_current_state(TASK_RUNNING)
    true
}

pub unsafe extern "C" fn netfs_wb_begin(ictx: *mut netfs_inode, nowait: bool) -> bool {
    if !test_and_set_bit_lock(NETFS_ICTX_WB_LOCK, &mut (*ictx).flags) { return true; }
    if nowait { netfs_stat(&mut netfs_n_wb_lock_skip); return false; }
    netfs_stat(&mut netfs_n_wb_lock_wait);
    netfs_wb_begin_wait(ictx)
}

pub unsafe extern "C" fn netfs_wb_end(ictx: *mut netfs_inode) {
    spin_lock(&mut (*ictx).lock);
    // list_first_entry_or_null(&ictx->wb_queue, struct netfs_wb_waiter, link)
    let waiter: *mut netfs_wb_waiter = ptr::null_mut();
    if !waiter.is_null() {
        let tsk = (*waiter).waiter;
        ptr::write_volatile(&mut (*waiter).waiter, ptr::null_mut());
        wake_up_process(tsk);
        put_task_struct(tsk);
    } else {
        clear_bit_unlock(NETFS_ICTX_WB_LOCK, &mut (*ictx).flags);
    }
    spin_unlock(&mut (*ictx).lock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
