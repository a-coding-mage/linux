/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by linux/wait.h.

#[repr(C)]
pub struct fs_pin {
    pub wait: wait_queue_head_t,
    pub done: ::core::ffi::c_int,
    pub s_list: hlist_node,
    pub m_list: hlist_node,
    pub kill: Option<unsafe extern "C" fn(*mut fs_pin)>,
}

#[repr(C)]
pub struct vfsmount {
    _private: [u8; 0],
}

#[inline]
pub unsafe fn init_fs_pin(
    p: *mut fs_pin,
    kill: Option<unsafe extern "C" fn(*mut fs_pin)>,
) {
    init_waitqueue_head(&mut (*p).wait);
    INIT_HLIST_NODE(&mut (*p).s_list);
    INIT_HLIST_NODE(&mut (*p).m_list);
    (*p).kill = kill;
}

unsafe extern "C" {
    pub fn pin_remove(p: *mut fs_pin);
    pub fn pin_insert(p: *mut fs_pin, m: *mut vfsmount);
    pub fn pin_kill(p: *mut fs_pin);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
