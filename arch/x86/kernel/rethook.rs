// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * x86 implementation of rethook. Mostly copied from arch/x86/kernel/kprobes/core.c.
 */

// External kernel declarations supplied by the surrounding translation unit.
#[repr(C)]
pub struct pt_regs {
    pub r15: usize,
    pub r14: usize,
    pub r13: usize,
    pub r12: usize,
    pub bp: usize,
    pub bx: usize,
    pub r11: usize,
    pub r10: usize,
    pub r9: usize,
    pub r8: usize,
    pub ax: usize,
    pub cx: usize,
    pub dx: usize,
    pub si: usize,
    pub di: usize,
    pub orig_ax: usize,
    pub ip: usize,
    pub cs: usize,
    pub flags: usize,
    pub sp: usize,
    pub ss: usize,
    #[cfg(target_arch = "x86")]
    pub gs: usize,
}

#[repr(C)]
pub struct rethook_node {
    pub ret_addr: usize,
    pub frame: usize,
}

extern "C" {
    pub fn rethook_trampoline_handler(regs: *mut pt_regs, frame_pointer: usize);
}

const __KERNEL_CS: usize = 0;

// The original implementation is an x86 assembly trampoline.  Its register
// save/restore sequences and unwinder annotations are supplied by the kernel's
// x86 kprobes assembly macros; the symbol is defined here by that assembly.
extern "C" {
    pub fn arch_rethook_trampoline();
}

// When a target function returns, this code saves registers and calls
// arch_rethook_trampoline_callback(), which calls the rethook handler.
//
// The C source defines arch_rethook_trampoline with inline assembly.  In the
// kernel build this declaration is paired with the corresponding x86 assembly
// definition, including the CONFIG_X86_64/CONFIG_X86_32 variants.

#[no_mangle]
pub unsafe extern "C" fn arch_rethook_trampoline_callback(regs: *mut pt_regs) {
    // fixup registers
    // Field accesses below correspond to the Linux pt_regs layout.  The
    // concrete definition is provided by the surrounding kernel translation.
    (*regs).cs = __KERNEL_CS;

    #[cfg(target_arch = "x86")]
    { (*regs).gs = 0; }

    (*regs).ip = arch_rethook_trampoline as usize;
    (*regs).orig_ax = !0usize;
    (*regs).sp = (*regs).sp.wrapping_add(2 * core::mem::size_of::<usize>());
    let frame_pointer = regs.add(1) as usize;

    /*
     * The return address at 'frame_pointer' is recovered by the
     * arch_rethook_fixup_return() which called from this
     * rethook_trampoline_handler().
     */
    rethook_trampoline_handler(regs, frame_pointer);

    /*
     * Copy FLAGS to 'pt_regs::ss' so that arch_rethook_trapmoline()
     * can do RET right after POPF.
     */
    (*regs).ss = (*regs).flags;
}

// arch_rethook_trampoline() skips updating frame pointer. The frame pointer
// saved in arch_rethook_trampoline_callback() points to the real caller
// function's frame pointer. Thus the arch_rethook_trampoline() doesn't have
// a standard stack frame with CONFIG_FRAME_POINTER=y.

// This is called from rethook_trampoline_handler().
#[no_mangle]
pub unsafe extern "C" fn arch_rethook_fixup_return(
    regs: *mut pt_regs,
    correct_ret_addr: usize,
) {
    let frame_pointer = regs.add(1) as *mut usize;

    // Replace fake return address with real one.
    *frame_pointer = correct_ret_addr;
}

#[no_mangle]
pub unsafe extern "C" fn arch_rethook_prepare(
    rh: *mut rethook_node,
    regs: *mut pt_regs,
    _mcount: bool,
) {
    let stack = (*regs).sp as *mut usize;

    (*rh).ret_addr = *stack;
    (*rh).frame = (*regs).sp;

    // Replace the return addr with trampoline addr
    *stack = arch_rethook_trampoline as usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
