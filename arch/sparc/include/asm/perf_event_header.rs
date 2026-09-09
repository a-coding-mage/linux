/* SPDX-License-Identifier: GPL-2.0 */

// CONFIG_PERF_EVENTS: the declarations below are present only when perf
// events are enabled in the build configuration.

#[cfg(feature = "CONFIG_PERF_EVENTS")]
#[inline(always)]
pub unsafe fn perf_arch_fetch_caller_regs(
    regs: *mut crate::pt_regs,
    ip: usize,
) {
    let pstate: usize;
    let asi: usize;
    let pil: usize;
    let i7: usize;
    let fp: usize;

    // The original header uses SPARC privileged-register inline assembly.
    // This is kept as target-specific Rust inline assembly.
    core::arch::asm!(
        "rdpr %pstate, {pstate}",
        "rd %asi, {asi}",
        "rdpr %pil, {pil}",
        "mov %i7, {i7}",
        "mov %i6, {fp}",
        pstate = out(reg) pstate,
        asi = out(reg) asi,
        pil = out(reg) pil,
        i7 = out(reg) i7,
        fp = out(reg) fp,
        options(nostack, preserves_flags),
    );

    (*regs).tstate = (pstate << 8) | (asi << 24) | (pil << 20);
    (*regs).tpc = ip;
    (*regs).tnpc = (*regs).tpc + 4;
    (*regs).u_regs[crate::UREG_I6] = fp;
    (*regs).u_regs[crate::UREG_I7] = i7;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
