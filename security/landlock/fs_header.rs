// SPDX-License-Identifier: GPL-2.0-only
/*
 * Landlock - Filesystem management and hooks
 *
 * Copyright © 2017-2020 Mickaël Salaün <mic@digikod.net>
 * Copyright © 2018-2020 ANSSI
 * Copyright © 2024-2025 Microsoft Corporation
 */

/*
 * Dependencies from the original header:
 * - linux/build_bug.h
 * - linux/cleanup.h
 * - linux/fs.h
 * - linux/init.h
 * - linux/rcupdate.h
 * - access.h
 * - cred.h
 * - ruleset.h
 * - setup.h
 */

/*
 * Original C cleanup macro:
 * DEFINE_FREE(__putname, char *, if (_T) __putname(_T))
 *
 * Rust has no file-local direct equivalent for this cleanup attribute macro.
 * The external __putname symbol and any RAII wrapper are expected from the
 * surrounding translation if this cleanup class is needed.
 */

/**
 * struct landlock_inode_security - Inode security blob
 *
 * Enable to reference a &struct landlock_object tied to an inode (i.e.
 * underlying object).
 */
#[repr(C)]
pub struct landlock_inode_security {
    /**
     * @object: Weak pointer to an allocated object.  All assignments of a
     * new object are protected by the underlying inode->i_lock.  However,
     * atomically disassociating @object from the inode is only protected
     * by @object->lock, from the time @object's usage refcount drops to
     * zero to the time this pointer is nulled out (cf. release_inode() and
     * hook_sb_delete()).  Indeed, such disassociation doesn't require
     * inode->i_lock thanks to the careful rcu_access_pointer() check
     * performed by get_inode_object().
     */
    pub object: *mut landlock_object,
}

/**
 * struct landlock_file_security - File security blob
 *
 * This information is populated when opening a file in hook_file_open, and
 * tracks the relevant Landlock access rights that were available at the time
 * of opening the file. Other LSM hooks use these rights in order to authorize
 * operations on already opened files.
 */
#[repr(C)]
pub struct landlock_file_security {
    /**
     * @allowed_access: Access rights that were available at the time of
     * opening the file. This is not necessarily the full set of access
     * rights available at that time, but it's the necessary subset as
     * needed to authorize later operations on the open file.
     */
    pub allowed_access: access_mask_t,

    /*
     * CONFIG_SECURITY_LANDLOCK_LOG:
     *
     * /**
     *  * @deny_masks: Domain layer levels that deny an optional access (see
     *  * _LANDLOCK_ACCESS_FS_OPTIONAL).
     *  */
     * deny_masks_t deny_masks;
     * /**
     *  * @quiet_optional_accesses: Stores which optional accesses are covered
     *  * by quiet rules within the layer referred to in deny_masks, one access
     *  * per bit.  Does not take into account whether the quiet access bits
     *  * are actually set in the layer's corresponding landlock_hierarchy.
     *  */
     * optional_access_t quiet_optional_accesses;
     * /**
     *  * @fown_layer: Layer level of @fown_subject->domain with
     *  * LANDLOCK_SCOPE_SIGNAL.
     *  */
     * u8 fown_layer;
     */
    #[cfg(CONFIG_SECURITY_LANDLOCK_LOG)]
    pub deny_masks: deny_masks_t,
    #[cfg(CONFIG_SECURITY_LANDLOCK_LOG)]
    pub quiet_optional_accesses: optional_access_t,
    #[cfg(CONFIG_SECURITY_LANDLOCK_LOG)]
    pub fown_layer: u8,

    /**
     * @fown_subject: Landlock credential of the task that set the PID that
     * may receive a signal e.g., SIGURG when writing MSG_OOB to the
     * related socket.  This pointer is protected by the related
     * file->f_owner->lock, as for fown_struct's members: pid, uid, and
     * euid.
     */
    pub fown_subject: landlock_cred_security,
    /**
     * @fown_tg: Thread group of the task that set the file owner, pinned
     * while @fown_subject holds a domain.  It lets
     * hook_file_send_sigiotask() always allow a SIGIO delivered to the
     * owner's own process -- e.g. the thread-group leader reached through a
     * process-group owner -- matching the same-process exemption of
     * hook_task_kill().  NULL when no domain is recorded.  Protected by
     * file->f_owner->lock, like @fown_subject.
     */
    pub fown_tg: *mut pid,
}

/*
 * CONFIG_SECURITY_LANDLOCK_LOG:
 *
 * Makes sure all layers can be identified.
 * static_assert((typeof_member(struct landlock_file_security, fown_layer))~0 >=
 *               LANDLOCK_MAX_NUM_LAYERS);
 *
 * Make sure quiet_optional_accesses has enough bits to cover all optional
 * accesses.
 * static_assert(BITS_PER_TYPE(typeof_member(struct landlock_file_security,
 *                                           quiet_optional_accesses)) >=
 *               HWEIGHT(_LANDLOCK_ACCESS_FS_OPTIONAL));
 */

/**
 * struct landlock_superblock_security - Superblock security blob
 *
 * Enable hook_sb_delete() to wait for concurrent calls to release_inode().
 */
#[repr(C)]
pub struct landlock_superblock_security {
    /**
     * @inode_refs: Number of pending inodes (from this superblock) that
     * are being released by release_inode().
     * Cf. struct super_block->s_fsnotify_inode_refs .
     */
    pub inode_refs: atomic_long_t,
}

#[inline]
pub unsafe fn landlock_file(file: *const file) -> *mut landlock_file_security {
    unsafe {
        ((*file).f_security as *mut u8)
            .add(landlock_blob_sizes.lbs_file as usize)
            as *mut landlock_file_security
    }
}

#[inline]
pub unsafe fn landlock_inode(inode: *const inode) -> *mut landlock_inode_security {
    unsafe {
        ((*inode).i_security as *mut u8)
            .add(landlock_blob_sizes.lbs_inode as usize)
            as *mut landlock_inode_security
    }
}

#[inline]
pub unsafe fn landlock_superblock(
    superblock: *const super_block,
) -> *mut landlock_superblock_security {
    unsafe {
        ((*superblock).s_security as *mut u8)
            .add(landlock_blob_sizes.lbs_superblock as usize)
            as *mut landlock_superblock_security
    }
}

unsafe extern "C" {
    pub fn landlock_add_fs_hooks();

    pub fn landlock_append_fs_rule(
        ruleset: *mut landlock_ruleset,
        path: *const path,
        access_hierarchy: access_mask_t,
        flags: u32,
    ) -> i32;
}

/**
 * resolve_path_for_trace - Resolve a path for tracepoint display
 *
 * @path: The path to resolve.
 * @buf: A buffer of at least PATH_MAX bytes for the resolved path.
 *
 * Uses d_absolute_path() to produce a namespace-independent absolute path,
 * unlike d_path() which resolves relative to the process's chroot.  This
 * ensures trace output is deterministic regardless of the tracer's mount
 * namespace.
 *
 * Return: A pointer into @buf with the resolved path, or an error string
 * ("<too_long>", "<unreachable>").
 */
#[inline]
pub unsafe fn resolve_path_for_trace(path: *const path, buf: *mut core::ffi::c_char) -> *const core::ffi::c_char {
    let p: *const core::ffi::c_char;

    unsafe {
        p = d_absolute_path(path, buf, PATH_MAX);
        if !IS_ERR_OR_NULL(p) {
            return p;
        }

        if PTR_ERR(p) == -ENAMETOOLONG {
            return c"<too_long>".as_ptr();
        }

        c"<unreachable>".as_ptr()
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
