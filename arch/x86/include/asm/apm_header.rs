/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Machine specific APM BIOS functions for generic.
 *  Split out from apm.c by Osamu Tomita <tomita@cinet.co.jp>
 */

// `APM_ZERO_SEGS` is a build-time configuration option from the C header.
// When enabled, the BIOS call preserves and clears the segment registers.

#[cfg(APM_ZERO_SEGS)]
macro_rules! apm_do_zero_segs {
    () => {
        "pushl %ds\n\tpushl %es\n\txorl %edx, %edx\n\tmov %dx, %ds\n\tmov %dx, %es\n\tmov %dx, %fs\n\tmov %dx, %gs\n\t"
    };
}

#[cfg(not(APM_ZERO_SEGS))]
macro_rules! apm_do_zero_segs {
    () => {
        ""
    };
}

#[cfg(APM_ZERO_SEGS)]
macro_rules! apm_do_pop_segs {
    () => {
        "popl %es\n\tpopl %ds\n\t"
    };
}

#[cfg(not(APM_ZERO_SEGS))]
macro_rules! apm_do_pop_segs {
    () => {
        ""
    };
}

/// N.B. We do NOT need a cld after the BIOS call because we always save and
/// restore the flags.
#[inline]
pub unsafe fn apm_bios_call_asm(
    func: u32,
    ebx_in: u32,
    ecx_in: u32,
    eax: *mut u32,
    ebx: *mut u32,
    ecx: *mut u32,
    edx: *mut u32,
    esi: *mut u32,
) {
    let mut eax_out: u32;
    let mut ebx_out: u32;
    let mut ecx_out: u32;
    let mut edx_out: u32;
    let mut esi_out: u32;
    core::arch::asm!(
        apm_do_zero_segs!(),
        "pushl %edi",
        "pushl %ebp",
        "lcall *%cs:apm_bios_entry",
        "setc %al",
        "popl %ebp",
        "popl %edi",
        apm_do_pop_segs!(),
        inlateout("eax") func => eax_out,
        inlateout("ebx") ebx_in => ebx_out,
        inlateout("ecx") ecx_in => ecx_out,
        lateout("edx") edx_out,
        lateout("esi") esi_out,
    );
    *eax = eax_out;
    *ebx = ebx_out;
    *ecx = ecx_out;
    *edx = edx_out;
    *esi = esi_out;
}

#[inline]
pub unsafe fn apm_bios_call_simple_asm(
    func: u32,
    ebx_in: u32,
    ecx_in: u32,
    eax: *mut u32,
) -> bool {
    let mut error: u8;
    let mut cx: u32;
    let mut dx: u32;
    let mut si: u32;
    let mut eax_out: u32;

    /// N.B. We do NOT need a cld after the BIOS call because we always save
    /// and restore the flags.
    core::arch::asm!(
        apm_do_zero_segs!(),
        "pushl %edi",
        "pushl %ebp",
        "lcall *%cs:apm_bios_entry",
        "setc %bl",
        "popl %ebp",
        "popl %edi",
        apm_do_pop_segs!(),
        inlateout("eax") func => eax_out,
        lateout("ebx") error,
        lateout("ecx") cx,
        lateout("edx") dx,
        lateout("esi") si,
    );
    *eax = eax_out;
    error != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
