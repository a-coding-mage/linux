/* SPDX-License-Identifier: GPL-2.0 */
/* Routines to test for presence/absence of hardware registers:
 * see arch/m68k/mm/hwtest.c.
 *  -- PMM <pmaydell@chiark.greenend.org.uk> 05/1998
 *
 * Removed __init from decls.  We might want them in modules, and
 * the code is tiny anyway.  16/5/98 pb
 */

// C header guard: __ASM_HWTEST_H

unsafe extern "C" {
    pub fn hwreg_present(regp: *mut core::ffi::c_void) -> core::ffi::c_int;
    pub fn hwreg_write(
        regp: *mut core::ffi::c_void,
        val: core::ffi::c_ushort,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
