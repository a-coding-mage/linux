/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding x86 CPUID implementation.

/*
 * This CPUID returns feature bitmaps in EAX.
 * Guest VM uses this to detect the appropriate feature bit.
 */
pub const ACRN_CPUID_FEATURES: u32 = 0x40000001;
/* Bit 0 indicates whether guest VM is privileged */
pub const ACRN_FEATURE_PRIVILEGED_VM: u32 = 1u32 << 0;

/*
 * Timing Information.
 * This leaf returns the current TSC frequency in kHz.
 *
 * EAX: (Virtual) TSC frequency in kHz.
 * EBX, ECX, EDX: RESERVED (reserved fields are set to zero).
 */
pub const ACRN_CPUID_TIMING_INFO: u32 = 0x40000010;

unsafe extern "C" {
    pub fn acrn_setup_intr_handler(handler: Option<unsafe extern "C" fn()>);
    pub fn acrn_remove_intr_handler();
}

#[inline]
pub unsafe fn acrn_cpuid_base() -> u32 {
    if boot_cpu_has(X86_FEATURE_HYPERVISOR) {
        return cpuid_base_hypervisor(b"ACRNACRNACRN\0", 0);
    }

    0
}

#[inline]
pub unsafe fn acrn_get_tsc_khz() -> usize {
    cpuid_eax(ACRN_CPUID_TIMING_INFO) as usize
}

/*
 * Hypercalls for ACRN
 *
 * - VMCALL instruction is used to implement ACRN hypercalls.
 * - ACRN hypercall ABI:
 *   - Hypercall number is passed in R8 register.
 *   - Up to 2 arguments are passed in RDI, RSI.
 *   - Return value will be placed in RAX.
 *
 * Because GCC doesn't support R8 register as direct register constraints, use
 * supported constraint as input with a explicit MOV to R8 in beginning of asm.
 */
#[inline]
pub unsafe fn acrn_hypercall0(hcall_id: usize) -> isize {
    let result: isize;
    core::arch::asm!(
        "mov r8d, {hcall:e}",
        "vmcall",
        hcall = in(reg) hcall_id,
        lateout("rax") result,
        out("r8") _,
        options(nostack)
    );
    result
}

#[inline]
pub unsafe fn acrn_hypercall1(hcall_id: usize, param1: usize) -> isize {
    let result: isize;
    core::arch::asm!(
        "mov r8d, {hcall:e}",
        "vmcall",
        hcall = in(reg) hcall_id,
        in("rdi") param1,
        lateout("rax") result,
        out("r8") _,
        options(nostack)
    );
    result
}

#[inline]
pub unsafe fn acrn_hypercall2(hcall_id: usize, param1: usize, param2: usize) -> isize {
    let result: isize;
    core::arch::asm!(
        "mov r8d, {hcall:e}",
        "vmcall",
        hcall = in(reg) hcall_id,
        in("rdi") param1,
        in("rsi") param2,
        lateout("rax") result,
        out("r8") _,
        options(nostack)
    );
    result
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
