// SPDX-License-Identifier: GPL-2.0
/*
 * (C) 2001 Clemson University and The University of Chicago
 *
 * See COPYING in top-level directory.
 */

// C dependencies: protocol.h, orangefs-kernel.h

/* tags assigned to kernel upcall operations */
static mut next_tag_value: __u64 = 0;
static mut next_tag_value_lock: DEFINE_SPINLOCK = DEFINE_SPINLOCK::new();

/* the orangefs memory caches */

/* a cache for orangefs upcall/downcall operations */
static mut op_cache: *mut kmem_cache = core::ptr::null_mut();

pub unsafe fn op_cache_initialize() -> i32 {
    op_cache = kmem_cache_create_usercopy(
        b"orangefs_op_cache\0".as_ptr() as *const core::ffi::c_char,
        core::mem::size_of::<orangefs_kernel_op_s>(),
        0,
        0,
        core::mem::offset_of!(orangefs_kernel_op_s, tag),
        core::mem::offset_of!(orangefs_kernel_op_s, upcall)
            + core::mem::size_of::<orangefs_upcall_s>()
            - core::mem::offset_of!(orangefs_kernel_op_s, tag),
        core::ptr::null_mut(),
    );

    if op_cache.is_null() {
        gossip_err(b"Cannot create orangefs_op_cache\n\0".as_ptr() as *const core::ffi::c_char);
        return -ENOMEM;
    }

    /* initialize our atomic tag counter */
    spin_lock(&mut next_tag_value_lock);
    next_tag_value = 100;
    spin_unlock(&mut next_tag_value_lock);
    0
}

pub unsafe fn op_cache_finalize() -> i32 {
    kmem_cache_destroy(op_cache);
    0
}

pub unsafe fn get_opname_string(new_op: *mut orangefs_kernel_op_s) -> *const core::ffi::c_char {
    if !new_op.is_null() {
        let type_ = (*new_op).upcall.type_;

        if type_ == ORANGEFS_VFS_OP_FILE_IO { return b"OP_FILE_IO\0".as_ptr() as *const _; }
        else if type_ == ORANGEFS_VFS_OP_LOOKUP { return b"OP_LOOKUP\0".as_ptr() as *const _; }
        else if type_ == ORANGEFS_VFS_OP_CREATE { return b"OP_CREATE\0".as_ptr() as *const _; }
        else if type_ == ORANGEFS_VFS_OP_GETATTR { return b"OP_GETATTR\0".as_ptr() as *const _; }
        else if type_ == ORANGEFS_VFS_OP_REMOVE { return b"OP_REMOVE\0".as_ptr() as *const _; }
        else if type_ == ORANGEFS_VFS_OP_MKDIR { return b"OP_MKDIR\0".as_ptr() as *const _; }
        else if type_ == ORANGEFS_VFS_OP_READDIR { return b"OP_READDIR\0".as_ptr() as *const _; }
        else if type_ == ORANGEFS_VFS_OP_READDIRPLUS { return b"OP_READDIRPLUS\0".as_ptr() as *const _; }
        else if type_ == ORANGEFS_VFS_OP_SETATTR { return b"OP_SETATTR\0".as_ptr() as *const _; }
        else if type_ == ORANGEFS_VFS_OP_SYMLINK { return b"OP_SYMLINK\0".as_ptr() as *const _; }
        else if type_ == ORANGEFS_VFS_OP_RENAME { return b"OP_RENAME\0".as_ptr() as *const _; }
        else if type_ == ORANGEFS_VFS_OP_STATFS { return b"OP_STATFS\0".as_ptr() as *const _; }
        else if type_ == ORANGEFS_VFS_OP_TRUNCATE { return b"OP_TRUNCATE\0".as_ptr() as *const _; }
        else if type_ == ORANGEFS_VFS_OP_RA_FLUSH { return b"OP_RA_FLUSH\0".as_ptr() as *const _; }
        else if type_ == ORANGEFS_VFS_OP_FS_MOUNT { return b"OP_FS_MOUNT\0".as_ptr() as *const _; }
        else if type_ == ORANGEFS_VFS_OP_FS_UMOUNT { return b"OP_FS_UMOUNT\0".as_ptr() as *const _; }
        else if type_ == ORANGEFS_VFS_OP_GETXATTR { return b"OP_GETXATTR\0".as_ptr() as *const _; }
        else if type_ == ORANGEFS_VFS_OP_SETXATTR { return b"OP_SETXATTR\0".as_ptr() as *const _; }
        else if type_ == ORANGEFS_VFS_OP_LISTXATTR { return b"OP_LISTXATTR\0".as_ptr() as *const _; }
        else if type_ == ORANGEFS_VFS_OP_REMOVEXATTR { return b"OP_REMOVEXATTR\0".as_ptr() as *const _; }
        else if type_ == ORANGEFS_VFS_OP_PARAM { return b"OP_PARAM\0".as_ptr() as *const _; }
        else if type_ == ORANGEFS_VFS_OP_PERF_COUNT { return b"OP_PERF_COUNT\0".as_ptr() as *const _; }
        else if type_ == ORANGEFS_VFS_OP_CANCEL { return b"OP_CANCEL\0".as_ptr() as *const _; }
        else if type_ == ORANGEFS_VFS_OP_FSYNC { return b"OP_FSYNC\0".as_ptr() as *const _; }
        else if type_ == ORANGEFS_VFS_OP_FSKEY { return b"OP_FSKEY\0".as_ptr() as *const _; }
        else if type_ == ORANGEFS_VFS_OP_FEATURES { return b"OP_FEATURES\0".as_ptr() as *const _; }
    }
    b"OP_UNKNOWN?\0".as_ptr() as *const _
}

pub unsafe fn orangefs_new_tag(op: *mut orangefs_kernel_op_s) {
    spin_lock(&mut next_tag_value_lock);
    (*op).tag = next_tag_value;
    next_tag_value = next_tag_value.wrapping_add(1);
    if next_tag_value == 0 { next_tag_value = 100; }
    spin_unlock(&mut next_tag_value_lock);
}

pub unsafe fn op_alloc(type_: __s32) -> *mut orangefs_kernel_op_s {
    let new_op = kmem_cache_zalloc(op_cache, GFP_KERNEL) as *mut orangefs_kernel_op_s;
    if !new_op.is_null() {
        INIT_LIST_HEAD(&mut (*new_op).list);
        spin_lock_init(&mut (*new_op).lock);
        init_completion(&mut (*new_op).waitq);
        (*new_op).upcall.type_ = ORANGEFS_VFS_OP_INVALID;
        (*new_op).downcall.type_ = ORANGEFS_VFS_OP_INVALID;
        (*new_op).downcall.status = -1;
        (*new_op).op_state = OP_VFS_STATE_UNKNOWN;
        orangefs_new_tag(new_op);
        (*new_op).upcall.type_ = type_;
        (*new_op).attempts = 0;
        gossip_debug(GOSSIP_CACHE_DEBUG, b"Alloced OP (%p: %llu %s)\n\0".as_ptr() as *const _, new_op, llu((*new_op).tag), get_opname_string(new_op));
        (*new_op).upcall.uid = from_kuid(&init_user_ns, current_fsuid());
        (*new_op).upcall.gid = from_kgid(&init_user_ns, current_fsgid());
    } else {
        gossip_err(b"op_alloc: kmem_cache_zalloc failed!\n\0".as_ptr() as *const _);
    }
    new_op
}

pub unsafe fn op_release(orangefs_op: *mut orangefs_kernel_op_s) {
    if !orangefs_op.is_null() {
        gossip_debug(GOSSIP_CACHE_DEBUG, b"Releasing OP (%p: %llu)\n\0".as_ptr() as *const _, orangefs_op, llu((*orangefs_op).tag));
        kmem_cache_free(op_cache, orangefs_op as *mut _);
    } else {
        gossip_err(b"NULL pointer in op_release\n\0".as_ptr() as *const _);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
