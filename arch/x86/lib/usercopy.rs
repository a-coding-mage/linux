/*
 * User address space access functions.
 *
 *  For licencing details see kernel-base/COPYING
 */

use core::ffi::c_void;

extern "C" {
    fn __access_ok(addr: *const c_void, size: c_ulong) -> bool;
    fn nmi_uaccess_okay() -> bool;
    fn pagefault_disable();
    fn instrument_copy_from_user_before(to: *mut c_void, from: *const c_void, n: c_ulong);
    fn raw_copy_from_user(to: *mut c_void, from: *const c_void, n: c_ulong) -> c_ulong;
    fn instrument_copy_from_user_after(
        to: *mut c_void,
        from: *const c_void,
        n: c_ulong,
        ret: c_ulong,
    );
    fn pagefault_enable();
}

type c_ulong = usize;

/**
 * copy_from_user_nmi - NMI safe copy from user
 * @to:\tPointer to the destination buffer
 * @from:\tPointer to a user space address of the current task
 * @n:\tNumber of bytes to copy
 *
 * Returns: The number of not copied bytes. 0 is success, i.e. all bytes copied
 *
 * Contrary to other copy_from_user() variants this function can be called
 * from NMI context. Despite the name it is not restricted to be called
 * from NMI context. It is safe to be called from any other context as
 * well. It disables pagefaults across the copy which means a fault will
 * abort the copy.
 *
 * For NMI context invocations this relies on the nested NMI work to allow
 * atomic faults from the NMI path; the nested NMI paths are careful to
 * preserve CR2.
 */
#[no_mangle]
pub unsafe extern "C" fn copy_from_user_nmi(
    to: *mut c_void,
    from: *const c_void,
    n: c_ulong,
) -> c_ulong {
    let ret: c_ulong;

    if !__access_ok(from, n) {
        return n;
    }

    if !nmi_uaccess_okay() {
        return n;
    }

    /*
     * Even though this function is typically called from NMI/IRQ context
     * disable pagefaults so that its behaviour is consistent even when
     * called from other contexts.
     */
    pagefault_disable();
    instrument_copy_from_user_before(to, from, n);
    ret = raw_copy_from_user(to, from, n);
    instrument_copy_from_user_after(to, from, n, ret);
    pagefault_enable();

    ret
}

// EXPORT_SYMBOL_GPL(copy_from_user_nmi);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
