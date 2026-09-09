// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Spin and read/write lock operations.
 *
 * Copyright (C) 2001-2004 Paul Mackerras <paulus@au.ibm.com>, IBM
 * Copyright (C) 2001 Anton Blanchard <anton@au.ibm.com>, IBM
 * Copyright (C) 2002 Dave Engebretsen <engebret@us.ibm.com>, IBM
 *   Rework to support virtual processors
 */

/* waiting for a spinlock... */
#[cfg(CONFIG_PPC_SPLPAR)]
pub unsafe fn splpar_spin_yield(lock: *mut arch_spinlock_t) {
    let lock_value: u32;
    let holder_cpu: u32;
    let yield_count: u32;

    lock_value = (*lock).slock;
    if lock_value == 0 {
        return;
    }
    holder_cpu = lock_value & 0xffff;
    BUG_ON(holder_cpu >= NR_CPUS);

    yield_count = yield_count_of(holder_cpu);
    if (yield_count & 1) == 0 {
        return; /* virtual cpu is currently running */
    }
    rmb();
    if (*lock).slock != lock_value {
        return; /* something has changed */
    }
    yield_to_preempted(holder_cpu, yield_count);
}

/*
 * Waiting for a read lock or a write lock on a rwlock...
 * This turns out to be the same for read and write locks, since
 * we only know the holder if it is write-locked.
 */
#[cfg(CONFIG_PPC_SPLPAR)]
pub unsafe fn splpar_rw_yield(rw: *mut arch_rwlock_t) {
    let lock_value: i32;
    let holder_cpu: u32;
    let yield_count: u32;

    lock_value = (*rw).lock;
    if lock_value >= 0 {
        return; /* no write lock at present */
    }
    holder_cpu = (lock_value as u32) & 0xffff;
    BUG_ON(holder_cpu >= NR_CPUS);

    yield_count = yield_count_of(holder_cpu);
    if (yield_count & 1) == 0 {
        return; /* virtual cpu is currently running */
    }
    rmb();
    if (*rw).lock != lock_value {
        return; /* something has changed */
    }
    yield_to_preempted(holder_cpu, yield_count);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
