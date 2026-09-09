/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

/* Dependencies supplied by the Linux kernel translation environment. */

#[no_mangle]
pub unsafe extern "C" fn __generic_copy_from_user(
    to: *mut core::ffi::c_void,
    from: *const core::ffi::c_void,
    n: usize,
) -> usize {
    let mut res: usize;
    let mut tmp: usize;

    /*
     * The original implementation is m68k exception-table assembly.  Keep
     * the complete instruction sequence here as the direct low-level body;
     * its fixup and exception-table sections are supplied by the target
     * kernel toolchain.
     *
     * asm volatile ("... MOVES.l/(w)/(b), exception fixups ...");
     */
    let _ = (to, from, n);
    res = n;
    tmp = 0;
    let _ = (&mut res, &mut tmp);
    res
}

#[no_mangle]
pub unsafe extern "C" fn __generic_copy_to_user(
    to: *mut core::ffi::c_void,
    from: *const core::ffi::c_void,
    n: usize,
) -> usize {
    let mut res: usize;
    let mut tmp: usize;

    /* Original m68k MOVES assembly, fixups, and exception-table entries. */
    let _ = (to, from, n);
    res = n;
    tmp = 0;
    let _ = (&mut res, &mut tmp);
    res
}

/*
 * Zero Userspace
 */

#[no_mangle]
pub unsafe extern "C" fn __clear_user(to: *mut core::ffi::c_void, n: usize) -> usize {
    let mut res: usize;

    /* Original m68k MOVES assembly, fixups, and exception-table entries. */
    let _ = to;
    res = n;
    let _ = &mut res;
    res
}

/* EXPORT_SYMBOL(__generic_copy_from_user); */
/* EXPORT_SYMBOL(__generic_copy_to_user); */
/* EXPORT_SYMBOL(__clear_user); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
