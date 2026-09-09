/* SPDX-License-Identifier: GPL-2.0-or-later */

// External declarations supplied by the surrounding PowerPC perf code.
extern "C" {
    pub fn perf_callchain_user_64(
        entry: *mut perf_callchain_entry_ctx,
        regs: *mut pt_regs,
    );
    pub fn perf_callchain_user_32(
        entry: *mut perf_callchain_entry_ctx,
        regs: *mut pt_regs,
    );
}

/// External types supplied by other translation units.
pub enum perf_callchain_entry_ctx {}
pub enum pt_regs {}

#[inline]
pub unsafe fn invalid_user_sp(sp: c_ulong) -> bool {
    let mask: c_ulong = if is_32bit_task() { 3 } else { 7 };
    let top: c_ulong = STACK_TOP - if is_32bit_task() { 16 } else { 32 };

    sp == 0 || (sp & mask) != 0 || sp > top
}

/*
 * On 32-bit we just access the address and let hash_page create a
 * HPTE if necessary, so there is no need to fall back to reading
 * the page tables.  Since this is called at interrupt level,
 * do_page_fault() won't treat a DSI as a page fault.
 */
#[inline]
pub unsafe fn __read_user_stack(
    ptr: *const core::ffi::c_void,
    ret: *mut core::ffi::c_void,
    size: usize,
) -> c_int {
    let addr = ptr as c_ulong;

    if addr > TASK_SIZE - size as c_ulong || (addr & (size as c_ulong - 1)) != 0 {
        return -EFAULT;
    }

    copy_from_user_nofault(ret, ptr, size)
}

// Required external symbols and C-compatible integer types.
type c_ulong = core::ffi::c_ulong;
type c_int = core::ffi::c_int;

extern "C" {
    fn is_32bit_task() -> bool;
    fn copy_from_user_nofault(
        ret: *mut core::ffi::c_void,
        ptr: *const core::ffi::c_void,
        size: usize,
    ) -> c_int;
}

// Build-provided constants.
extern "C" {
    static STACK_TOP: c_ulong;
    static TASK_SIZE: c_ulong;
    static EFAULT: c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
