/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/******************************************************************************
 *
 * Name: aclinuxex.h - Extra OS specific defines, etc. for Linux
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

/* The declarations below are conditional on the C __KERNEL__ build. */

/* The following definitions are used when ACPI_USE_NATIVE_DIVIDE is absent. */
#[macro_export]
macro_rules! ACPI_DIV_64_BY_32 {
    ($n_hi:expr, $n_lo:expr, $d32:expr, $q32:expr, $r32:expr) => {{
        let mut __n: u64 = (($n_hi as u64) << 32) | ($n_lo as u64);
        $r32 = __n % ($d32 as u64);
        __n /= $d32 as u64;
        $q32 = __n as u32;
    }};
}

#[macro_export]
macro_rules! ACPI_SHIFT_RIGHT_64 {
    ($n_hi:expr, $n_lo:expr) => {{
        $n_lo >>= 1;
        $n_lo |= (($n_hi & 1) << 31);
        $n_hi >>= 1;
    }};
}

/* Overrides for in-kernel ACPICA. */
extern "C" {
    pub fn acpi_os_initialize() -> acpi_status;
    pub fn acpi_os_terminate() -> acpi_status;
}

#[macro_export]
macro_rules! acpi_os_allocate {
    ($size:expr) => {
        kmalloc($size, if irqs_disabled() { GFP_ATOMIC } else { GFP_KERNEL })
    };
}

#[macro_export]
macro_rules! acpi_os_allocate_zeroed {
    ($size:expr) => {
        kzalloc($size, if irqs_disabled() { GFP_ATOMIC } else { GFP_KERNEL })
    };
}

#[macro_export]
macro_rules! acpi_os_acquire_object {
    ($cache:expr) => {
        kmem_cache_zalloc($cache, if irqs_disabled() { GFP_ATOMIC } else { GFP_KERNEL })
    };
}

#[inline]
pub unsafe fn acpi_os_free(memory: *mut core::ffi::c_void) {
    kfree(memory);
}

#[inline]
pub unsafe fn acpi_os_get_thread_id() -> acpi_thread_id {
    current as usize as acpi_thread_id
}

#[macro_export]
macro_rules! acpi_os_create_lock {
    ($handle:expr) => {{
        let lock = ACPI_ALLOCATE(core::mem::size_of::<spinlock_t>()) as *mut spinlock_t;
        if !lock.is_null() {
            *$handle = lock;
            spin_lock_init(*$handle);
        }
        if !lock.is_null() { AE_OK } else { AE_NO_MEMORY }
    }};
}

#[macro_export]
macro_rules! acpi_os_create_raw_lock {
    ($handle:expr) => {{
        let lock = ACPI_ALLOCATE(core::mem::size_of::<raw_spinlock_t>()) as *mut raw_spinlock_t;
        if !lock.is_null() {
            *$handle = lock;
            raw_spin_lock_init(*$handle);
        }
        if !lock.is_null() { AE_OK } else { AE_NO_MEMORY }
    }};
}

#[inline]
pub unsafe fn acpi_os_acquire_raw_lock(lockp: acpi_raw_spinlock) -> acpi_cpu_flags {
    let mut flags: acpi_cpu_flags = core::mem::zeroed();
    raw_spin_lock_irqsave(lockp, &mut flags);
    flags
}

#[inline]
pub unsafe fn acpi_os_release_raw_lock(lockp: acpi_raw_spinlock, flags: acpi_cpu_flags) {
    raw_spin_unlock_irqrestore(lockp, flags);
}

#[inline]
pub unsafe fn acpi_os_delete_raw_lock(handle: acpi_raw_spinlock) {
    ACPI_FREE(handle);
}

#[inline]
pub unsafe fn acpi_os_readable(_pointer: *mut core::ffi::c_void, _length: acpi_size) -> u8 {
    TRUE
}

#[inline]
pub fn acpi_os_initialize_debugger() -> acpi_status {
    AE_OK
}

#[inline]
pub fn acpi_os_terminate_debugger() {
}

/* OSL interfaces added by Linux. */
pub use strscpy_pad as acpi_ut_safe_strncpy;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
