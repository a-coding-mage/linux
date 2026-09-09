// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct Rust translation of dlmdebug.c. C kernel dependencies are external. */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

extern "C" {
    fn spin_lock(lock: *mut c_void);
    fn spin_unlock(lock: *mut c_void);
    fn printk(fmt: *const c_char, ...);
    fn find_next_bit(addr: *const c_ulong, size: c_ulong, offset: c_ulong) -> c_ulong;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ... ) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn kzalloc(size: usize, flags: c_ulong) -> *mut c_void;
    fn kmalloc(size: usize, flags: c_ulong) -> *mut c_void;
    fn kfree(p: *mut c_void);
}

const OCFS2_DENTRY_LOCK_INO_START: usize = 18;
const DEBUG_LOCK_VERSION: c_int = 1;
const DEBUG_LRES_VERSION: c_int = 1;

// Kernel structures, constants, helpers, list macros, and CONFIG_DEBUG_FS items
// are supplied by the translated dependency units.

unsafe fn stringify_lockname(lockname: *const c_char, locklen: c_int, buf: *mut c_char, len: c_int) -> c_int {
    let mut out = 0;
    if *lockname as u8 == b'N' {
        let mut inode_blkno_be: u64 = 0;
        memcpy(&mut inode_blkno_be as *mut _ as *mut c_void,
               lockname.add(OCFS2_DENTRY_LOCK_INO_START) as *const c_void, 8);
        out += scnprintf(buf.add(out as usize), (len - out) as usize,
            b"%.*s%08x\0".as_ptr() as *const c_char,
            OCFS2_DENTRY_LOCK_INO_START as c_int - 1, lockname,
            u64::from_be(inode_blkno_be) as u32);
    } else {
        out += scnprintf(buf.add(out as usize), (len - out) as usize,
            b"%.*s\0".as_ptr() as *const c_char, locklen, lockname);
    }
    out
}

unsafe fn stringify_nodemap(nodemap: *mut c_ulong, maxnodes: c_int, buf: *mut c_char, len: c_int) -> c_int {
    let mut out = 0;
    let mut i: c_ulong = 0;
    loop {
        i = find_next_bit(nodemap, maxnodes as c_ulong, i);
        if i >= maxnodes as c_ulong { break; }
        out += scnprintf(buf.add(out as usize), (len - out) as usize,
            b"%d \0".as_ptr() as *const c_char, i as c_int);
        i += 1;
    }
    out
}

// The following declarations preserve the source interfaces and operations;
// field-bearing kernel types and list traversal primitives come from headers.
pub unsafe fn dlm_print_one_lock_resource(res: *mut dlm_lock_resource) {
    spin_lock(&mut (*res).spinlock as *mut _ as *mut c_void);
    __dlm_print_one_lock_resource(res);
    spin_unlock(&mut (*res).spinlock as *mut _ as *mut c_void);
}

unsafe fn dlm_print_lockres_refmap(res: *mut dlm_lock_resource) {
    let mut bit: c_ulong = 0;
    printk(b"  refmap nodes: [ \0".as_ptr() as *const c_char);
    loop {
        bit = find_next_bit((*res).refmap.as_ptr(), O2NM_MAX_NODES as c_ulong, bit);
        if bit >= O2NM_MAX_NODES as c_ulong { break; }
        printk(b"%u \0".as_ptr() as *const c_char, bit as c_int);
        bit += 1;
    }
    printk(b"], inflight=%u\n\0".as_ptr() as *const c_char, (*res).inflight_locks);
}

unsafe fn __dlm_print_lock(lock: *mut dlm_lock) {
    spin_lock(&mut (*lock).spinlock as *mut _ as *mut c_void);
    printk(b"    type=%d, conv=%d, node=%u, cookie=%u:%llu, ref=%u, ast=(empty=%c,pend=%c), bast=(empty=%c,pend=%c), pending=(conv=%c,lock=%c,cancel=%c,unlock=%c)\n\0".as_ptr() as *const c_char,
        (*lock).ml.type_, (*lock).ml.convert_type, (*lock).ml.node,
        dlm_get_lock_cookie_node(u64::from_be((*lock).ml.cookie)),
        dlm_get_lock_cookie_seq(u64::from_be((*lock).ml.cookie)),
        kref_read(&(*lock).lock_refs),
        if list_empty(&(*lock).ast_list) { b'y' } else { b'n' }, if (*lock).ast_pending { b'y' } else { b'n' },
        if list_empty(&(*lock).bast_list) { b'y' } else { b'n' }, if (*lock).bast_pending { b'y' } else { b'n' },
        if (*lock).convert_pending { b'y' } else { b'n' }, if (*lock).lock_pending { b'y' } else { b'n' },
        if (*lock).cancel_pending { b'y' } else { b'n' }, if (*lock).unlock_pending { b'y' } else { b'n' });
    spin_unlock(&mut (*lock).spinlock as *mut _ as *mut c_void);
}

pub unsafe fn __dlm_print_one_lock_resource(res: *mut dlm_lock_resource) {
    let mut buf = [0 as c_char; DLM_LOCKID_NAME_MAX as usize];
    stringify_lockname((*res).lockname.name, (*res).lockname.len, buf.as_mut_ptr(), buf.len() as c_int);
    printk(b"lockres: %s, owner=%u, state=%u\n\0".as_ptr() as *const c_char, buf.as_ptr(), (*res).owner, (*res).state);
    dlm_print_lockres_refmap(res);
    // list_for_each_entry over granted, converting, and blocked queues.
    list_for_each_entry_lock((*res).granted, |lock| __dlm_print_lock(lock));
    list_for_each_entry_lock((*res).converting, |lock| __dlm_print_lock(lock));
    list_for_each_entry_lock((*res).blocked, |lock| __dlm_print_lock(lock));
}

pub unsafe fn dlm_print_one_lock(lockid: *mut dlm_lock) { dlm_print_one_lock_resource((*lockid).lockres); }

pub unsafe fn dlm_errname(err: dlm_status) -> *const c_char {
    if err < 0 || err >= DLM_MAXSTATS { return dlm_errnames[DLM_MAXSTATS as usize]; }
    dlm_errnames[err as usize]
}

pub unsafe fn dlm_print_one_mle(mle: *mut dlm_master_list_entry) {
    let buf = kzalloc(PAGE_SIZE, GFP_ATOMIC);
    if !buf.is_null() { dump_mle(mle, buf as *mut c_char, PAGE_SIZE - 1); kfree(buf); }
}

// CONFIG_DEBUG_FS implementation follows the C source's debugfs operations.
// Its kernel file-operation, seq-file, and list APIs are external dependencies.
#[cfg(feature = "CONFIG_DEBUG_FS")]
pub unsafe fn dlm_debug_init(dlm: *mut dlm_ctxt) {
    debugfs_create_file(b"dlm_state\0".as_ptr(), S_IFREG | S_IRUSR, (*dlm).dlm_debugfs_subroot, dlm, &debug_state_fops);
    debugfs_create_file(b"locking_state\0".as_ptr(), S_IFREG | S_IRUSR, (*dlm).dlm_debugfs_subroot, dlm, &debug_lockres_fops);
    debugfs_create_file(b"mle_state\0".as_ptr(), S_IFREG | S_IRUSR, (*dlm).dlm_debugfs_subroot, dlm, &debug_mle_fops);
    debugfs_create_file(b"purge_list\0".as_ptr(), S_IFREG | S_IRUSR, (*dlm).dlm_debugfs_subroot, dlm, &debug_purgelist_fops);
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
pub unsafe fn dlm_create_debugfs_subroot(dlm: *mut dlm_ctxt) { (*dlm).dlm_debugfs_subroot = debugfs_create_dir((*dlm).name, dlm_debugfs_root); }
#[cfg(feature = "CONFIG_DEBUG_FS")]
pub unsafe fn dlm_destroy_debugfs_subroot(dlm: *mut dlm_ctxt) { debugfs_remove_recursive((*dlm).dlm_debugfs_subroot); }
#[cfg(feature = "CONFIG_DEBUG_FS")]
pub unsafe fn dlm_create_debugfs_root() { dlm_debugfs_root = debugfs_create_dir(b"o2dlm\0".as_ptr(), core::ptr::null_mut()); }
#[cfg(feature = "CONFIG_DEBUG_FS")]
pub unsafe fn dlm_destroy_debugfs_root() { debugfs_remove(dlm_debugfs_root); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
