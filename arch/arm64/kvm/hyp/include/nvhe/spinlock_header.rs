/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * A stand-alone ticket spinlock implementation for use by the non-VHE
 * KVM hypervisor code running at EL2.
 *
 * Copyright (C) 2020 Google LLC
 * Author: Will Deacon <will@kernel.org>
 *
 * Heavily based on the implementation removed by c11090474d70 which was:
 * Copyright (C) 2012 ARM Ltd.
 */

// C dependencies: asm/alternative.h, asm/lse.h, and asm/rwonce.h.

#[repr(C)]
pub union hyp_spinlock {
    pub __val: u32,
    #[cfg(target_endian = "big")]
    pub fields: hyp_spinlock_fields_be,
    #[cfg(not(target_endian = "big"))]
    pub fields: hyp_spinlock_fields_le,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hyp_spinlock_fields_be {
    pub next: u16,
    pub owner: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hyp_spinlock_fields_le {
    pub owner: u16,
    pub next: u16,
}

pub type hyp_spinlock_t = hyp_spinlock;

pub const __HYP_SPIN_LOCK_INITIALIZER: hyp_spinlock_t = hyp_spinlock { __val: 0 };
pub const __HYP_SPIN_LOCK_UNLOCKED: hyp_spinlock_t = __HYP_SPIN_LOCK_INITIALIZER;

#[macro_export]
macro_rules! DEFINE_HYP_SPINLOCK {
    ($x:ident) => {
        static mut $x: $crate::hyp_spinlock_t = $crate::__HYP_SPIN_LOCK_UNLOCKED;
    };
}

#[macro_export]
macro_rules! hyp_spin_lock_init {
    ($l:expr) => {{
        unsafe { *($l) = $crate::__HYP_SPIN_LOCK_UNLOCKED; }
    }};
}

#[inline]
pub unsafe fn hyp_spin_lock(lock: *mut hyp_spinlock_t) {
    // The C implementation selects the LL/SC or LSE sequence through
    // ARM64_LSE_ATOMIC_INSN and executes the ticket acquisition protocol.
    // Keep the architecture-specific operation as inline assembly; the
    // alternative-selection macro is supplied by the including build.
    core::arch::asm!(
        "prfm pstl1strm, [{lock}]",
        "1:",
        "ldaxr {lockval:w}, [{lock}]",
        "add {newval:w}, {lockval:w}, #(1 << 16)",
        "stxr {tmp:w}, {newval:w}, [{lock}]",
        "cbnz {tmp:w}, 1b",
        "eor {newval:w}, {lockval:w}, {lockval:w}, ror #16",
        "cbz {newval:w}, 3f",
        "sevl",
        "2:",
        "wfe",
        "ldaxrh {tmp:w}, [{owner}]",
        "eor {newval:w}, {tmp:w}, {lockval:w}, lsr #16",
        "cbnz {newval:w}, 2b",
        "3:",
        lock = inout(reg) lock => _,
        owner = in(reg) (lock as *mut u8).add(2),
        lockval = out(reg) _,
        newval = out(reg) _,
        tmp = out(reg) _,
        options(nostack)
    );
}

#[inline]
pub unsafe fn hyp_spin_unlock(lock: *mut hyp_spinlock_t) {
    core::arch::asm!(
        "ldrh {tmp:w}, [{lock}]",
        "add {tmp:w}, {tmp:w}, #1",
        "stlrh {tmp:w}, [{lock}]",
        lock = in(reg) (lock as *mut u8).add(2),
        tmp = out(reg) _,
        options(nostack)
    );
}

#[inline]
pub unsafe fn hyp_spin_is_locked(lock: *mut hyp_spinlock_t) -> bool {
    let lockval = core::ptr::read_volatile(lock);
    #[cfg(target_endian = "big")]
    let fields = lockval.fields;
    #[cfg(not(target_endian = "big"))]
    let fields = lockval.fields;
    fields.owner != fields.next
}

#[cfg(feature = "CONFIG_NVHE_EL2_DEBUG")]
#[inline]
pub unsafe fn hyp_assert_lock_held(lock: *mut hyp_spinlock_t) {
    // static_branch_likely(&kvm_protected_mode_initialized) gates BUG_ON.
    if hyp_spin_is_locked(lock) {
        return;
    }
    panic!("BUG_ON(!hyp_spin_is_locked(lock))");
}

#[cfg(not(feature = "CONFIG_NVHE_EL2_DEBUG"))]
#[inline]
pub unsafe fn hyp_assert_lock_held(_lock: *mut hyp_spinlock_t) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
