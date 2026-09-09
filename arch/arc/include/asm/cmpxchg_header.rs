/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// C dependencies: linux/build_bug.h, linux/types.h, linux/cmpxchg-emu.h,
// asm/barrier.h, and asm/smp.h.
// CONFIG_ARC_HAS_LLSC selects the corresponding implementation below.

#[cfg(CONFIG_ARC_HAS_LLSC)]
#[macro_export]
macro_rules! __cmpxchg {
    ($ptr:expr, $old:expr, $new:expr) => {{
        // if (*ptr == old), *ptr = new
        let mut _prev: _;
        unsafe {
            core::arch::asm!(
                "1: llock {prev}, [{ptr}]",
                "brne {prev}, {old}, 2f",
                "scond {new}, [{ptr}]",
                "bnz 1b",
                "2:",
                prev = lateout(reg) _prev,
                ptr = in(reg) $ptr,
                old = in(reg) $old,
                new = in(reg) $new,
                options(nostack)
            );
        }
        _prev
    }};
}

#[cfg(CONFIG_ARC_HAS_LLSC)]
#[macro_export]
macro_rules! arch_cmpxchg_relaxed {
    ($ptr:expr, $old:expr, $new:expr) => {{
        let _p_ = $ptr;
        let _o_ = $old;
        let _n_ = $new;
        let _prev_ = match core::mem::size_of_val(&_p_) {
            1 => unsafe {
                cmpxchg_emu_u8(_p_ as *mut u8, _o_ as usize, _n_ as usize) as _
            },
            4 => $crate::__cmpxchg!(_p_, _o_, _n_),
            _ => {
                BUILD_BUG!();
                unreachable!()
            }
        };
        _prev_
    }};
}

#[cfg(not(CONFIG_ARC_HAS_LLSC))]
#[macro_export]
macro_rules! arch_cmpxchg {
    ($ptr:expr, $old:expr, $new:expr) => {{
        let _p_ = $ptr as _;
        let _o_ = $old;
        let _n_ = $new;
        let mut __flags: usize = 0;
        atomic_ops_lock!(__flags);
        let _prev_ = unsafe { core::ptr::read_volatile(_p_) };
        if _prev_ == _o_ {
            unsafe { core::ptr::write_volatile(_p_, _n_); }
        }
        atomic_ops_unlock!(__flags);
        _prev_
    }};
}

/* xchg */
#[cfg(CONFIG_ARC_HAS_LLSC)]
#[macro_export]
macro_rules! __arch_xchg {
    ($ptr:expr, $val:expr) => {{
        let mut _val_ = $val;
        unsafe {
            core::arch::asm!(
                "ex {val}, [{ptr}]",
                val = inout(reg) _val_,
                ptr = in(reg) $ptr,
                options(nostack)
            );
        }
        _val_
    }};
}

#[cfg(CONFIG_ARC_HAS_LLSC)]
#[macro_export]
macro_rules! arch_xchg_relaxed {
    ($ptr:expr, $val:expr) => {{
        let _p_ = $ptr;
        let mut _val_ = $val;
        match core::mem::size_of_val(&_p_) {
            4 => _val_ = $crate::__arch_xchg!(_p_, _val_),
            _ => { BUILD_BUG!(); }
        }
        _val_
    }};
}

// EX is also available without LLSC, but must be protected by the spinlock
// to interoperate with cmpxchg() in this regime.
#[cfg(not(CONFIG_ARC_HAS_LLSC))]
#[macro_export]
macro_rules! arch_xchg {
    ($ptr:expr, $val:expr) => {{
        let _p_ = $ptr;
        let mut _val_ = $val;
        let mut __flags: usize = 0;
        atomic_ops_lock!(__flags);
        unsafe {
            core::arch::asm!(
                "ex {val}, [{ptr}]",
                val = inout(reg) _val_,
                ptr = in(reg) _p_,
                options(nostack)
            );
        }
        atomic_ops_unlock!(__flags);
        _val_
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
