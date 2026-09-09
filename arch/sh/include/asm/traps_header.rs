/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from <linux/compiler.h> and <asm/traps_32.h> is preserved
// here as external declarations; those headers supply the original trap-handler
// declaration details.

unsafe extern "C" {
    pub fn address_error(regs: *mut core::ffi::c_void);
    pub fn debug(regs: *mut core::ffi::c_void);
    pub fn bug(regs: *mut core::ffi::c_void);
    pub fn breakpoint(regs: *mut core::ffi::c_void);
    pub fn singlestep(regs: *mut core::ffi::c_void);
    pub fn fpu_error(regs: *mut core::ffi::c_void);
    pub fn fpu_state_restore(regs: *mut core::ffi::c_void);
    pub fn nmi(regs: *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
