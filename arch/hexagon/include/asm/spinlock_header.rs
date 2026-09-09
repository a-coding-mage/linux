/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Spinlock support for the Hexagon architecture
 *
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

// C header dependencies: asm/irqflags.h, asm/barrier.h, asm/processor.h

/*
 * This file is pulled in for SMP builds.
 * Really need to check all the barrier stuff for "true" SMP
 */

/* Hexagon inline assembly is retained verbatim; the referenced lock types and
 * smp_mb are supplied by the corresponding architecture dependencies. */

pub unsafe fn arch_read_lock(lock: *mut arch_rwlock_t) {
    core::arch::asm!(
        "1:\tR6 = memw_locked({0});\n\t",
        "{{ P3 = cmp.ge(R6,#0); R6 = add(R6,#1);}}\n\t",
        "{{ if (!P3) jump 1b; }}\n\t",
        "memw_locked({0},P3) = R6;\n\t",
        "{{ if (!P3) jump 1b; }}\n\t",
        in(reg) &mut (*lock).lock,
        options(nostack)
    );
}

pub unsafe fn arch_read_unlock(lock: *mut arch_rwlock_t) {
    core::arch::asm!(
        "1:\tR6 = memw_locked({0});\n\t",
        "R6 = add(R6,#-1);\n\t",
        "memw_locked({0},P3) = R6\n\t",
        "if (!P3) jump 1b;\n\t",
        in(reg) &mut (*lock).lock,
        options(nostack)
    );
}

/* I think this returns 0 on fail, 1 on success. */
pub unsafe fn arch_read_trylock(lock: *mut arch_rwlock_t) -> i32 {
    let mut temp: i32;
    core::arch::asm!(
        "R6 = memw_locked({1});\n\t",
        "{{ {0} = #0; P3 = cmp.ge(R6,#0); R6 = add(R6,#1);}}\n\t",
        "{{ if (!P3) jump 1f; }}\n\t",
        "memw_locked({1},P3) = R6;\n\t",
        "{{ {0} = P3 }}\n1:\n",
        lateout(reg) temp,
        in(reg) &mut (*lock).lock,
        options(nostack)
    );
    temp
}

/* Stuffs a -1 in the lock value? */
pub unsafe fn arch_write_lock(lock: *mut arch_rwlock_t) {
    core::arch::asm!(
        "1:\tR6 = memw_locked({0})\n\t",
        "{{ P3 = cmp.eq(R6,#0); R6 = #-1;}}\n\t",
        "{{ if (!P3) jump 1b; }}\n\t",
        "memw_locked({0},P3) = R6;\n\t",
        "{{ if (!P3) jump 1b; }}\n\t",
        in(reg) &mut (*lock).lock,
        options(nostack)
    );
}

pub unsafe fn arch_write_trylock(lock: *mut arch_rwlock_t) -> i32 {
    let mut temp: i32;
    core::arch::asm!(
        "R6 = memw_locked({1})\n\t",
        "{{ {0} = #0; P3 = cmp.eq(R6,#0); R6 = #-1;}}\n\t",
        "{{ if (!P3) jump 1f; }}\n\t",
        "memw_locked({1},P3) = R6;\n\t",
        "{0} = P3;\n1:\n",
        lateout(reg) temp,
        in(reg) &mut (*lock).lock,
        options(nostack)
    );
    temp
}

pub unsafe fn arch_write_unlock(lock: *mut arch_rwlock_t) {
    smp_mb();
    (*lock).lock = 0;
}

pub unsafe fn arch_spin_lock(lock: *mut arch_spinlock_t) {
    core::arch::asm!(
        "1:\tR6 = memw_locked({0});\n\t",
        "P3 = cmp.eq(R6,#0);\n\t",
        "{{ if (!P3) jump 1b; R6 = #1; }}\n\t",
        "memw_locked({0},P3) = R6;\n\t",
        "{{ if (!P3) jump 1b; }}\n\t",
        in(reg) &mut (*lock).lock,
        options(nostack)
    );
}

pub unsafe fn arch_spin_unlock(lock: *mut arch_spinlock_t) {
    smp_mb();
    (*lock).lock = 0;
}

pub unsafe fn arch_spin_trylock(lock: *mut arch_spinlock_t) -> u32 {
    let mut temp: i32;
    core::arch::asm!(
        "R6 = memw_locked({1});\n\t",
        "P3 = cmp.eq(R6,#0);\n\t",
        "{{ if (!P3) jump 1f; R6 = #1; {0} = #0; }}\n\t",
        "memw_locked({1},P3) = R6;\n\t",
        "{0} = P3;\n1:\n",
        lateout(reg) temp,
        in(reg) &mut (*lock).lock,
        options(nostack)
    );
    temp as u32
}

/* SMP spinlocks are intended to allow only a single CPU at the lock */
#[inline]
pub unsafe fn arch_spin_is_locked(x: *const arch_spinlock_t) -> bool {
    (*x).lock != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
