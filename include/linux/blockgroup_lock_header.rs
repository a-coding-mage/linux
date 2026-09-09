/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Per-blockgroup locking for ext2 and ext3.
 *
 * Simple hashed spinlocking.
 *
 * Dependency intent: spinlock_t, spin_lock_init, NR_CPUS, and ilog2 are
 * supplied by the corresponding kernel headers/build configuration.
 */

#[cfg(CONFIG_SMP)]
pub const NR_BG_LOCKS: usize =
    4usize << ilog2(if NR_CPUS < 32 { NR_CPUS } else { 32 });

#[cfg(not(CONFIG_SMP))]
pub const NR_BG_LOCKS: usize = 1;

#[repr(C)]
pub struct bgl_lock {
    pub lock: spinlock_t,
}

#[repr(C)]
pub struct blockgroup_lock {
    pub locks: [bgl_lock; NR_BG_LOCKS],
}

extern "C" {
    pub fn spin_lock_init(lock: *mut spinlock_t);
}

#[inline]
pub unsafe fn bgl_lock_init(bgl: *mut blockgroup_lock) {
    let mut i: i32 = 0;

    while i < NR_BG_LOCKS as i32 {
        spin_lock_init(&mut (*bgl).locks[i as usize].lock as *mut spinlock_t);
        i += 1;
    }
}

#[inline]
pub unsafe fn bgl_lock_ptr(
    bgl: *mut blockgroup_lock,
    block_group: u32,
) -> *mut spinlock_t {
    &mut (*bgl).locks[(block_group & (NR_BG_LOCKS - 1) as u32) as usize].lock
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
