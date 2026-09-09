/* SPDX-License-Identifier: GPL-2.0 */
/*
 * KMSAN checks to be used for one-off annotations in subsystems.
 *
 * Copyright (C) 2017-2022 Google LLC
 * Author: Alexander Potapenko <glider@google.com>
 *
 */

// Dependency supplied by the surrounding kernel translation: `gfp_t`.

#[cfg(feature = "CONFIG_KMSAN")]
unsafe extern "C" {
    /**
     * kmsan_poison_memory() - Mark the memory range as uninitialized.
     * @address: address to start with.
     * @size:    size of buffer to poison.
     * @flags:   GFP flags for allocations done by this function.
     *
     * Until other data is written to this range, KMSAN will treat it as
     * uninitialized. Error reports for this memory will reference the call site of
     * kmsan_poison_memory() as origin.
     */
    pub fn kmsan_poison_memory(address: *const core::ffi::c_void, size: usize, flags: gfp_t);

    /**
     * kmsan_unpoison_memory() -  Mark the memory range as initialized.
     * @address: address to start with.
     * @size:    size of buffer to unpoison.
     *
     * Until other data is written to this range, KMSAN will treat it as
     * initialized.
     */
    pub fn kmsan_unpoison_memory(address: *const core::ffi::c_void, size: usize);

    /**
     * kmsan_check_memory() - Check the memory range for being initialized.
     * @address: address to start with.
     * @size:    size of buffer to check.
     *
     * If any piece of the given range is marked as uninitialized, KMSAN will report
     * an error.
     */
    pub fn kmsan_check_memory(address: *const core::ffi::c_void, size: usize);

    /**
     * kmsan_copy_to_user() - Notify KMSAN about a data transfer to userspace.
     * @to:      destination address in the userspace.
     * @from:    source address in the kernel.
     * @to_copy: number of bytes to copy.
     * @left:    number of bytes not copied.
     *
     * If this is a real userspace data transfer, KMSAN checks the bytes that were
     * actually copied to ensure there was no information leak. If @to belongs to
     * the kernel space (which is possible for compat syscalls), KMSAN just copies
     * the metadata.
     */
    pub fn kmsan_copy_to_user(
        to: *mut core::ffi::c_void,
        from: *const core::ffi::c_void,
        to_copy: usize,
        left: usize,
    );

    /**
     * kmsan_memmove() - Notify KMSAN about a data copy within kernel.
     * @to:   destination address in the kernel.
     * @from: source address in the kernel.
     * @size: number of bytes to copy.
     *
     * Invoked after non-instrumented version (e.g. implemented using assembly
     * code) of memmove()/memcpy() is called, in order to copy KMSAN's metadata.
     */
    pub fn kmsan_memmove(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, to_copy: usize);
}

#[cfg(not(feature = "CONFIG_KMSAN"))]
#[inline]
pub fn kmsan_poison_memory(_address: *const core::ffi::c_void, _size: usize, _flags: gfp_t) {}

#[cfg(not(feature = "CONFIG_KMSAN"))]
#[inline]
pub fn kmsan_unpoison_memory(_address: *const core::ffi::c_void, _size: usize) {}

#[cfg(not(feature = "CONFIG_KMSAN"))]
#[inline]
pub fn kmsan_check_memory(_address: *const core::ffi::c_void, _size: usize) {}

#[cfg(not(feature = "CONFIG_KMSAN"))]
#[inline]
pub fn kmsan_copy_to_user(
    _to: *mut core::ffi::c_void,
    _from: *const core::ffi::c_void,
    _to_copy: usize,
    _left: usize,
) {
}

#[cfg(not(feature = "CONFIG_KMSAN"))]
#[inline]
pub fn kmsan_memmove(
    _to: *mut core::ffi::c_void,
    _from: *const core::ffi::c_void,
    _to_copy: usize,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
