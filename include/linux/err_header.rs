/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_void;

/*
 * Kernel pointers have redundant information, so we can use a
 * scheme where we can return either an error code or a normal
 * pointer with the same return value.
 *
 * This should be a per-architecture thing, to allow different
 * error and pointer decisions.
 */
pub const MAX_ERRNO: usize = 4095;

/**
 * IS_ERR_VALUE - Detect an error pointer.
 * @x: The pointer to check.
 *
 * Like IS_ERR(), but does not generate a compiler warning if result is unused.
 */
#[inline(always)]
pub fn IS_ERR_VALUE(x: usize) -> bool {
    x >= (0usize).wrapping_sub(MAX_ERRNO)
}

/**
 * ERR_PTR - Create an error pointer.
 * @error: A negative error code.
 *
 * Encodes @error into a pointer value. Users should consider the result
 * opaque and not assume anything about how the error is encoded.
 *
 * Return: A pointer with @error encoded within its value.
 */
#[inline(always)]
pub fn ERR_PTR(error: isize) -> *mut c_void {
    error as usize as *mut c_void
}

/**
 * INIT_ERR_PTR - Init a const error pointer.
 * @error: A negative error code.
 *
 * Like ERR_PTR(), but usable to initialize static variables.
 */
#[inline(always)]
pub const fn INIT_ERR_PTR(error: isize) -> *const c_void {
    error as usize as *const c_void
}

/* Return the pointer in the percpu address space. */
#[inline(always)]
pub fn ERR_PTR_PCPU(error: isize) -> *mut c_void {
    ERR_PTR(error)
}

/* Cast an error pointer to __iomem. */
#[inline(always)]
pub fn IOMEM_ERR_PTR(error: isize) -> *mut c_void {
    ERR_PTR(error)
}

/**
 * PTR_ERR - Extract the error code from an error pointer.
 * @ptr: An error pointer.
 * Return: The error code within @ptr.
 */
#[inline(always)]
pub fn PTR_ERR(ptr: *const c_void) -> isize {
    ptr as usize as isize
}

/* Read an error pointer from the percpu address space. */
#[inline(always)]
pub fn PTR_ERR_PCPU(ptr: *const c_void) -> isize {
    PTR_ERR(ptr)
}

/**
 * IS_ERR - Detect an error pointer.
 * @ptr: The pointer to check.
 * Return: true if @ptr is an error pointer, false otherwise.
 */
#[inline(always)]
pub fn IS_ERR(ptr: *const c_void) -> bool {
    IS_ERR_VALUE(ptr as usize)
}

/* Read an error pointer from the percpu address space. */
#[inline(always)]
pub fn IS_ERR_PCPU(ptr: *const c_void) -> bool {
    IS_ERR(ptr)
}

/**
 * IS_ERR_OR_NULL - Detect an error pointer or a null pointer.
 * @ptr: The pointer to check.
 *
 * Like IS_ERR(), but also returns true for a null pointer.
 */
#[inline(always)]
pub fn IS_ERR_OR_NULL(ptr: *const c_void) -> bool {
    ptr.is_null() || IS_ERR_VALUE(ptr as usize)
}

/**
 * ERR_CAST - Explicitly cast an error-valued pointer to another pointer type
 * @ptr: The pointer to cast.
 *
 * Explicitly cast an error-valued pointer to another pointer type in such a
 * way as to make it clear that's what's going on.
 */
#[inline(always)]
pub fn ERR_CAST(ptr: *const c_void) -> *mut c_void {
    /* cast away the const */
    ptr as *mut c_void
}

/**
 * PTR_ERR_OR_ZERO - Extract the error code from a pointer if it has one.
 * @ptr: A potential error pointer.
 *
 * Convenience function that can be used inside a function that returns
 * an error code to propagate errors received as error pointers.
 * For example, ``return PTR_ERR_OR_ZERO(ptr);`` replaces:
 *
 * .. code-block:: c
 *
 *\tif (IS_ERR(ptr))
 *\t\treturn PTR_ERR(ptr);
 *\telse
 *\t\treturn 0;
 *
 * Return: The error code within @ptr if it is an error pointer; 0 otherwise.
 */
#[inline(always)]
pub fn PTR_ERR_OR_ZERO(ptr: *const c_void) -> i32 {
    if IS_ERR(ptr) {
        PTR_ERR(ptr) as i32
    } else {
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
