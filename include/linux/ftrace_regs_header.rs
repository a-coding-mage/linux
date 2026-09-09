/* SPDX-License-Identifier: GPL-2.0 */

/*
 * For archs that just copy pt_regs in ftrace regs, it can use this default.
 * If an architecture does not use pt_regs, it must define all the below
 * accessor functions.
 *
 * The C preprocessor condition HAVE_ARCH_FTRACE_REGS is represented by the
 * corresponding Rust configuration feature.
 */
#[cfg(not(feature = "HAVE_ARCH_FTRACE_REGS"))]
#[repr(C)]
pub struct __arch_ftrace_regs {
    pub regs: pt_regs,
}

#[cfg(not(feature = "HAVE_ARCH_FTRACE_REGS"))]
#[inline(always)]
pub unsafe fn arch_ftrace_regs(fregs: *mut ftrace_regs) -> *mut __arch_ftrace_regs {
    fregs as *mut __arch_ftrace_regs
}

/* Declaration supplied by the surrounding kernel translation. */
#[repr(C)]
pub struct ftrace_regs {
    _private: [u8; 0],
}

#[cfg(not(feature = "HAVE_ARCH_FTRACE_REGS"))]
#[inline(always)]
pub unsafe fn ftrace_regs_get_instruction_pointer(fregs: *mut ftrace_regs) -> _ {
    instruction_pointer(&(*arch_ftrace_regs(fregs)).regs)
}

#[cfg(not(feature = "HAVE_ARCH_FTRACE_REGS"))]
#[inline(always)]
pub unsafe fn ftrace_regs_get_argument(fregs: *mut ftrace_regs, n: usize) -> _ {
    regs_get_kernel_argument(&(*arch_ftrace_regs(fregs)).regs, n)
}

#[cfg(not(feature = "HAVE_ARCH_FTRACE_REGS"))]
#[inline(always)]
pub unsafe fn ftrace_regs_get_stack_pointer(fregs: *mut ftrace_regs) -> _ {
    kernel_stack_pointer(&(*arch_ftrace_regs(fregs)).regs)
}

#[cfg(not(feature = "HAVE_ARCH_FTRACE_REGS"))]
#[inline(always)]
pub unsafe fn ftrace_regs_get_return_value(fregs: *mut ftrace_regs) -> _ {
    regs_return_value(&(*arch_ftrace_regs(fregs)).regs)
}

#[cfg(not(feature = "HAVE_ARCH_FTRACE_REGS"))]
#[inline(always)]
pub unsafe fn ftrace_regs_set_return_value(fregs: *mut ftrace_regs, ret: _) {
    regs_set_return_value(&(*arch_ftrace_regs(fregs)).regs, ret)
}

#[cfg(not(feature = "HAVE_ARCH_FTRACE_REGS"))]
#[inline(always)]
pub unsafe fn ftrace_override_function_with_return(fregs: *mut ftrace_regs) -> _ {
    override_function_with_return(&(*arch_ftrace_regs(fregs)).regs)
}

#[cfg(not(feature = "HAVE_ARCH_FTRACE_REGS"))]
#[inline(always)]
pub unsafe fn ftrace_regs_query_register_offset(name: *const core::ffi::c_char) -> _ {
    regs_query_register_offset(name)
}

#[cfg(not(feature = "HAVE_ARCH_FTRACE_REGS"))]
#[inline(always)]
pub unsafe fn ftrace_regs_get_frame_pointer(fregs: *mut ftrace_regs) -> _ {
    frame_pointer(&(*arch_ftrace_regs(fregs)).regs)
}

#[cfg(not(feature = "HAVE_ARCH_FTRACE_REGS"))]
#[inline(always)]
pub unsafe fn ftrace_partial_regs_update(_fregs: *mut ftrace_regs, _regs: *mut pt_regs) {
}

#[cfg(feature = "HAVE_ARCH_FTRACE_REGS")]
#[inline(always)]
pub unsafe fn ftrace_partial_regs_update(fregs: *mut ftrace_regs, regs: *mut pt_regs) {
    ftrace_regs_set_instruction_pointer(fregs, instruction_pointer(regs));
    ftrace_regs_set_return_value(fregs, regs_return_value(regs));
}

/*
 * ftrace_partial_regs_update - update the original ftrace_regs from regs
 * @fregs: The ftrace_regs to update from @regs
 * @regs: The partial regs from ftrace_partial_regs() that was updated
 *
 * Some architectures have the partial regs living in the ftrace_regs
 * structure, whereas other architectures need to make a different copy
 * of the @regs. If a partial @regs is retrieved by ftrace_partial_regs() and
 * if the code using @regs updates a field (like the instruction pointer or
 * stack pointer) it may need to propagate that change to the original @fregs
 * it retrieved the partial @regs from. Use this function to guarantee that
 * update happens.
 */

/* This can be overridden by the architectures. */
#[cfg(not(feature = "FTRACE_REGS_MAX_ARGS"))]
pub const FTRACE_REGS_MAX_ARGS: usize = 6;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
