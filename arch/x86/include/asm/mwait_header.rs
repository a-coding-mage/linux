/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux::sched, linux::sched::idle, asm::cpufeature, asm::nospec_branch.

pub const MWAIT_SUBSTATE_MASK: u32 = 0xf;
pub const MWAIT_CSTATE_MASK: u32 = 0xf;
pub const MWAIT_SUBSTATE_SIZE: u32 = 4;

#[inline(always)]
pub const fn MWAIT_HINT2CSTATE(hint: u32) -> u32 {
    (hint >> MWAIT_SUBSTATE_SIZE) & MWAIT_CSTATE_MASK
}

#[inline(always)]
pub const fn MWAIT_HINT2SUBSTATE(hint: u32) -> u32 {
    hint & MWAIT_CSTATE_MASK
}

pub const MWAIT_C1_SUBSTATE_MASK: u32 = 0xf0;

pub const CPUID5_ECX_EXTENSIONS_SUPPORTED: u32 = 0x1;
pub const CPUID5_ECX_INTERRUPT_BREAK: u32 = 0x2;

pub const MWAIT_ECX_INTERRUPT_BREAK: u32 = 0x1;
pub const MWAITX_ECX_TIMER_ENABLE: u32 = BIT(1);
pub const MWAITX_MAX_WAIT_CYCLES: u32 = u32::MAX;
pub const MWAITX_DISABLE_CSTATES: u32 = 0xf0;
pub const TPAUSE_C01_STATE: u32 = 1;
pub const TPAUSE_C02_STATE: u32 = 0;

#[inline(always)]
unsafe fn __monitor(eax: *const core::ffi::c_void, ecx: u32, edx: u32) {
    // Use the instruction mnemonic with implicit operands, as the LLVM
    // assembler fails to assemble the mnemonic with explicit operands.
    core::arch::asm!("monitor", in("rax") eax, in("rcx") ecx, in("rdx") edx);
}

#[inline(always)]
unsafe fn __monitorx(eax: *const core::ffi::c_void, ecx: u32, edx: u32) {
    core::arch::asm!("monitorx", in("rax") eax, in("rcx") ecx, in("rdx") edx);
}

#[inline(always)]
unsafe fn __mwait(eax: u32, ecx: u32) {
    // Use the instruction mnemonic with implicit operands, as the LLVM
    // assembler fails to assemble the mnemonic with explicit operands.
    core::arch::asm!("mwait", in("eax") eax, in("ecx") ecx);
}

/*
 * MWAITX allows for a timer expiration to get the core out a wait state in
 * addition to the default MWAIT exit condition of a store appearing at a
 * monitored virtual address.
 *
 * Registers and the MWAIT/MWAITX comparison are as documented by the C header.
 */
#[inline(always)]
unsafe fn __mwaitx(eax: u32, ebx: u32, ecx: u32) {
    // No need for TSA buffer clearing on AMD
    core::arch::asm!("mwaitx", in("eax") eax, in("ebx") ebx, in("ecx") ecx);
}

/* Re-enable interrupts immediately before mwait, with no intervening instruction. */
#[inline(always)]
unsafe fn __sti_mwait(eax: u32, ecx: u32) {
    core::arch::asm!("sti; mwait", in("eax") eax, in("ecx") ecx);
}

#[inline(always)]
unsafe fn mwait_idle_with_hints(eax: u32, ecx: u32) {
    if need_resched() {
        return;
    }

    x86_idle_clear_cpu_buffers();

    'out: {
      if static_cpu_has_bug(X86_BUG_MONITOR) || !current_set_polling_and_test() {
        let addr: *const core::ffi::c_void =
            (&(*current_thread_info()).flags) as *const _ as *const core::ffi::c_void;

        alternative_input!("", "clflush (%[addr])", X86_BUG_CLFLUSH_MONITOR, addr = in("rax") addr);
        __monitor(addr, 0, 0);

        if need_resched() {
            break 'out;
        }

        if ecx & 1 != 0 {
            __mwait(eax, ecx);
        } else {
            __sti_mwait(eax, ecx);
            raw_local_irq_disable();
        }
      }
    }

    current_clr_polling();
}

#[inline(always)]
unsafe fn __tpause(ecx: u32, edx: u32, eax: u32) {
    // "tpause %ecx"
    core::arch::asm!(".byte 0x66, 0x0f, 0xae, 0xf1", in("ecx") ecx, in("edx") edx, in("eax") eax);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
