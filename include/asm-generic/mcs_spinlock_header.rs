#[repr(C)]
pub struct mcs_spinlock {
    pub next: *mut mcs_spinlock,
    pub locked: i32, /* 1 if lock acquired */
    pub count: i32,  /* nesting count, see qspinlock.c */
}

/*
 * Architectures can define their own:
 *
 *   arch_mcs_spin_lock_contended(l)
 *   arch_mcs_spin_unlock_contended(l)
 *
 * See kernel/locking/mcs_spinlock.c.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
