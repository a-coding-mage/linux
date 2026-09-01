// SPDX-License-Identifier: GPL-2.0
/*
 * Copied from the kernel sources:
 *
 * Copyright IBM Corp. 1999, 2009
 *
 * Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
 */

/*
 * Force strict CPU ordering.
 * And yes, this is required on UP too when we're talking
 * to devices.
 */

// C conditional:
//   #ifdef CONFIG_HAVE_MARCH_Z196_FEATURES
// Fast-BCR without checkpoint synchronization
#[cfg(CONFIG_HAVE_MARCH_Z196_FEATURES)]
pub const __ASM_BARRIER: &str = "bcr 14,0\n";

#[cfg(not(CONFIG_HAVE_MARCH_Z196_FEATURES))]
pub const __ASM_BARRIER: &str = "bcr 15,0\n";

#[cfg(CONFIG_HAVE_MARCH_Z196_FEATURES)]
#[inline(always)]
pub unsafe fn mb() {
    unsafe {
        core::arch::asm!("bcr 14,0", options(nostack, preserves_flags));
    }
}

#[cfg(not(CONFIG_HAVE_MARCH_Z196_FEATURES))]
#[inline(always)]
pub unsafe fn mb() {
    unsafe {
        core::arch::asm!("bcr 15,0", options(nostack, preserves_flags));
    }
}

#[inline(always)]
pub unsafe fn rmb() {
    unsafe {
        mb();
    }
}

#[inline(always)]
pub unsafe fn wmb() {
    unsafe {
        mb();
    }
}

#[macro_export]
macro_rules! mb {
    () => {{
        unsafe {
            $crate::mb();
        }
    }};
}

#[macro_export]
macro_rules! rmb {
    () => {{
        unsafe {
            $crate::rmb();
        }
    }};
}

#[macro_export]
macro_rules! wmb {
    () => {{
        unsafe {
            $crate::wmb();
        }
    }};
}

#[macro_export]
macro_rules! smp_store_release {
    ($p:expr, $v:expr) => {{
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        unsafe {
            core::ptr::write_volatile($p, $v);
        }
    }};
}

#[macro_export]
macro_rules! smp_load_acquire {
    ($p:expr) => {{
        let ___p1 = unsafe { core::ptr::read_volatile($p) };
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        ___p1
    }};
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
