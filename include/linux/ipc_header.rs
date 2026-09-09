/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux headers:
// linux/spinlock_types.h, linux/uidgid.h, linux/rhashtable-types.h,
// uapi/linux/ipc.h, and linux/refcount.h.

/* used by in-kernel data structures */
#[repr(C)]
pub struct kern_ipc_perm {
    pub lock: spinlock_t,
    pub deleted: bool,
    pub id: i32,
    pub key: key_t,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub cuid: kuid_t,
    pub cgid: kgid_t,
    pub mode: umode_t,
    pub seq: core::ffi::c_ulong,
    pub security: *mut core::ffi::c_void,

    pub khtnode: rhash_head,

    pub rcu: rcu_head,
    pub refcount: refcount_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
