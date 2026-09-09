/* SPDX-License-Identifier: GPL-2.0 */
/* Dependencies supplied by the corresponding Linux fsnotify/inotify headers. */

#[repr(C)]
pub struct inotify_event_info {
    pub fse: fsnotify_event,
    pub mask: u32,
    pub wd: std::ffi::c_int,
    pub sync_cookie: u32,
    pub name_len: std::ffi::c_int,
    pub name: [std::ffi::c_char; 0],
}

#[repr(C)]
pub struct inotify_inode_mark {
    pub fsn_mark: fsnotify_mark,
    pub wd: std::ffi::c_int,
}

#[inline]
pub unsafe fn INOTIFY_E(fse: *mut fsnotify_event) -> *mut inotify_event_info {
    fse.cast::<u8>()
        .sub(std::mem::offset_of!(inotify_event_info, fse))
        .cast::<inotify_event_info>()
}

/*
 * INOTIFY_USER_FLAGS represents all of the mask bits that we expose to
 * userspace.  There is at least one bit (FS_EVENT_ON_CHILD) which is
 * used only internally to the kernel.
 */
pub const INOTIFY_USER_MASK: u32 = IN_ALL_EVENTS;

#[inline]
pub unsafe fn inotify_mark_user_mask(fsn_mark: *mut fsnotify_mark) -> u32 {
    let mut mask: u32 = (*fsn_mark).mask & INOTIFY_USER_MASK;

    if (*fsn_mark).flags & FSNOTIFY_MARK_FLAG_EXCL_UNLINK != 0 {
        mask |= IN_EXCL_UNLINK;
    }
    if (*fsn_mark).flags & FSNOTIFY_MARK_FLAG_IN_ONESHOT != 0 {
        mask |= IN_ONESHOT;
    }

    mask
}

unsafe extern "C" {
    pub fn inotify_ignored_and_remove_idr(
        fsn_mark: *mut fsnotify_mark,
        group: *mut fsnotify_group,
    );
    pub fn inotify_handle_inode_event(
        inode_mark: *mut fsnotify_mark,
        mask: u32,
        inode: *mut inode,
        dir: *mut inode,
        name: *const qstr,
        cookie: u32,
    ) -> std::ffi::c_int;

    pub static inotify_fsnotify_ops: fsnotify_ops;
    pub static mut inotify_inode_mark_cachep: *mut kmem_cache;
}

#[cfg(CONFIG_INOTIFY_USER)]
#[inline]
pub unsafe fn dec_inotify_instances(ucounts: *mut ucounts) {
    dec_ucount(ucounts, UCOUNT_INOTIFY_INSTANCES);
}

#[cfg(CONFIG_INOTIFY_USER)]
#[inline]
pub unsafe fn inc_inotify_watches(ucounts: *mut ucounts) -> *mut ucounts {
    inc_ucount((*ucounts).ns, (*ucounts).uid, UCOUNT_INOTIFY_WATCHES)
}

#[cfg(CONFIG_INOTIFY_USER)]
#[inline]
pub unsafe fn dec_inotify_watches(ucounts: *mut ucounts) {
    dec_ucount(ucounts, UCOUNT_INOTIFY_WATCHES);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
