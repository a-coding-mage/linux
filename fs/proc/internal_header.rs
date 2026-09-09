/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Internal procfs definitions
 *
 * Copyright (C) 2004 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* Dependencies supplied by the surrounding kernel translation. */

#[repr(C)]
pub struct ctl_table_header;
#[repr(C)]
pub struct mempolicy;

#[repr(C)]
pub struct proc_dir_entry {
    pub in_use: atomic_t,
    pub refcnt: refcount_t,
    pub pde_openers: list_head,
    pub pde_unload_lock: spinlock_t,
    pub pde_unload_completion: *mut completion,
    pub proc_iops: *const inode_operations,
    pub proc_ops: proc_dir_entry_proc_ops,
    pub seq_ops: proc_dir_entry_seq_ops,
    pub write: proc_write_t,
    pub data: *mut core::ffi::c_void,
    pub state_size: u32,
    pub low_ino: u32,
    pub nlink: nlink_t,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub size: loff_t,
    pub parent: *mut proc_dir_entry,
    pub subdir: rb_root,
    pub subdir_node: rb_node,
    pub name: *mut u8,
    pub mode: umode_t,
    pub flags: u8,
    pub namelen: u8,
    pub inline_name: [u8; 0],
}

#[repr(C)]
pub union proc_dir_entry_proc_ops {
    pub proc_ops: *const proc_ops,
    pub proc_dir_ops: *const file_operations,
}

#[repr(C)]
pub union proc_dir_entry_seq_ops {
    pub seq_ops: *const seq_operations,
    pub single_show: Option<unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void) -> i32>,
}

pub const SIZEOF_PDE: usize = if core::mem::size_of::<proc_dir_entry>() < 128 {
    128
} else if core::mem::size_of::<proc_dir_entry>() < 192 {
    192
} else if core::mem::size_of::<proc_dir_entry>() < 256 {
    256
} else if core::mem::size_of::<proc_dir_entry>() < 512 {
    512
} else {
    0
};
pub const SIZEOF_PDE_INLINE_NAME: usize = SIZEOF_PDE - core::mem::size_of::<proc_dir_entry>();

#[inline]
pub unsafe fn pde_is_permanent(pde: *const proc_dir_entry) -> bool {
    (*pde).flags & PROC_ENTRY_PERMANENT != 0
}

/* This is for builtin code, not even for modules which are compiled in. */
#[inline]
pub unsafe fn pde_make_permanent(pde: *mut proc_dir_entry) {
    /* Ensure magic flag does something. */
    assert!(PROC_ENTRY_PERMANENT != 0);
    (*pde).flags |= PROC_ENTRY_PERMANENT;
}

#[inline]
pub unsafe fn pde_has_proc_read_iter(pde: *const proc_dir_entry) -> bool {
    (*pde).flags & PROC_ENTRY_proc_read_iter != 0
}

#[inline]
pub unsafe fn pde_has_proc_compat_ioctl(pde: *const proc_dir_entry) -> bool {
    /* CONFIG_COMPAT */
    (*pde).flags & PROC_ENTRY_proc_compat_ioctl != 0
}

#[inline]
pub unsafe fn pde_has_proc_lseek(pde: *const proc_dir_entry) -> bool {
    (*pde).flags & PROC_ENTRY_proc_lseek != 0
}

pub static mut proc_dir_entry_cache: *mut kmem_cache = core::ptr::null_mut();
pub unsafe extern "C" fn pde_free(pde: *mut proc_dir_entry);

#[repr(C)]
pub union proc_op {
    pub proc_get_link: Option<unsafe extern "C" fn(*mut dentry, *mut path, *mut task_struct) -> i32>,
    pub proc_show: Option<unsafe extern "C" fn(*mut seq_file, *mut pid_namespace, *mut pid, *mut task_struct) -> i32>,
    pub lsmid: i32,
}

#[repr(C)]
pub struct proc_inode {
    pub pid: *mut pid,
    pub fd: u32,
    pub op: proc_op,
    pub pde: *mut proc_dir_entry,
    pub sysctl: *mut ctl_table_header,
    pub sysctl_entry: *const ctl_table,
    pub sibling_inodes: hlist_node,
    pub ns_ops: *const proc_ns_operations,
    pub vfs_inode: inode,
}

#[inline]
pub unsafe fn PROC_I(inode: *const inode) -> *mut proc_inode {
    container_of(inode, core::mem::offset_of!(proc_inode, vfs_inode))
}

#[inline]
pub unsafe fn PDE(inode: *const inode) -> *mut proc_dir_entry {
    (*PROC_I(inode)).pde
}

#[inline]
pub unsafe fn proc_pid(inode: *const inode) -> *mut pid {
    (*PROC_I(inode)).pid
}

#[inline]
pub unsafe fn get_proc_task(inode: *const inode) -> *mut task_struct {
    get_pid_task(proc_pid(inode), PIDTYPE_PID)
}

pub unsafe extern "C" fn task_dump_owner(task: *mut task_struct, mode: umode_t, ruid: *mut kuid_t, rgid: *mut kgid_t);
pub unsafe extern "C" fn name_to_int(qstr: *const qstr) -> u32;

/* Offset of the first process in the /proc root directory.. */
pub const FIRST_PROCESS_ENTRY: u32 = 256;

/* Worst case buffer size needed for holding an integer. */
pub const PROC_NUMBUF: u32 = 13;

/* CONFIG_PAGE_MAPCOUNT */
#[inline]
pub unsafe fn folio_precise_page_mapcount(folio: *mut folio, page: *mut page) -> i32 {
    let mut mapcount = atomic_read(&(*page)._mapcount) + 1;
    if page_mapcount_is_type(mapcount) {
        mapcount = 0;
    }
    if folio_test_large(folio) {
        mapcount += folio_entire_mapcount(folio);
    }
    mapcount
}

/* !CONFIG_PAGE_MAPCOUNT: the C implementation invokes BUILD_BUG(). */
#[inline]
pub unsafe fn folio_precise_page_mapcount_unavailable(folio: *mut folio, page: *mut page) -> i32 {
    let _ = (folio, page);
    build_bug();
}

#[inline]
pub unsafe fn folio_average_page_mapcount(folio: *mut folio) -> i32 {
    let mapcount: i32;
    let entire_mapcount: i32;
    let avg: i32;

    if !folio_test_large(folio) {
        return atomic_read(&(*folio)._mapcount) + 1;
    }

    mapcount = folio_large_mapcount(folio);
    if mapcount <= 0 {
        return 0;
    }
    entire_mapcount = folio_entire_mapcount(folio);
    if mapcount <= entire_mapcount {
        return entire_mapcount;
    }
    let mapcount = mapcount - entire_mapcount;

    /* Round to closest integer ... */
    avg = (((mapcount as u32) + folio_large_nr_pages(folio) / 2)
        >> folio_large_order(folio)) as i32;
    /* ... but return at least 1. */
    core::cmp::max(avg + entire_mapcount, 1)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
