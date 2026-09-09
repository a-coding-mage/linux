// SPDX-License-Identifier: GPL-2.0-only
/*
 * PowerPC implementation of rethook. This depends on kprobes.
 */

// Dependencies supplied by the kernel headers are intentionally left external.

use core::ffi::c_int;

#[repr(C)]
pub struct kprobe {
    pub addr: *mut kprobe_opcode_t,
    pub pre_handler: Option<unsafe extern "C" fn(*mut kprobe, *mut pt_regs) -> c_int>,
}

pub type kprobe_opcode_t = u32;

#[repr(C)]
pub struct pt_regs {
    pub gpr: [unsigned_long; 32],
    pub link: unsigned_long,
    pub nip: unsigned_long,
}

#[repr(C)]
pub struct rethook_node {
    pub ret_addr: unsigned_long,
    pub frame: unsigned_long,
}

pub type unsigned_long = usize;
pub type bool_ = bool;

unsafe extern "C" {
    fn rethook_trampoline_handler(regs: *mut pt_regs, frame: unsigned_long) -> c_int;
    fn regs_set_return_ip(regs: *mut pt_regs, ip: unsigned_long);
    fn register_kprobe(p: *mut kprobe) -> c_int;
}

/*
 * Function return trampoline:
 *     - init_kprobes() establishes a probepoint here
 *     - When the probed function returns, this probe
 *         causes the handlers to fire
 *
 * Original PowerPC assembly:
 * .global arch_rethook_trampoline
 * .type arch_rethook_trampoline, @function
 * arch_rethook_trampoline:
 * nop
 * blr
 * .size arch_rethook_trampoline, .-arch_rethook_trampoline
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch_rethook_trampoline() {
    core::arch::asm!("nop", "blr");
}

/*
 * Called when the probe at kretprobe trampoline is hit
 */
unsafe extern "C" fn trampoline_rethook_handler(
    p: *mut kprobe,
    regs: *mut pt_regs,
) -> c_int {
    let _ = p;
    !rethook_trampoline_handler(regs, (*regs).gpr[1])
}

// NOKPROBE_SYMBOL(trampoline_rethook_handler);

pub unsafe extern "C" fn arch_rethook_prepare(
    rh: *mut rethook_node,
    regs: *mut pt_regs,
    mcount: bool_,
) {
    let _ = mcount;
    (*rh).ret_addr = (*regs).link;
    (*rh).frame = (*regs).gpr[1];

    /* Replace the return addr with trampoline addr */
    (*regs).link = arch_rethook_trampoline as usize;
}

// NOKPROBE_SYMBOL(arch_rethook_prepare);

/* This is called from rethook_trampoline_handler(). */
pub unsafe extern "C" fn arch_rethook_fixup_return(
    regs: *mut pt_regs,
    orig_ret_address: unsigned_long,
) {
    /*
     * We get here through one of two paths:
     * 1. by taking a trap -> kprobe_handler() -> here
     * 2. by optprobe branch -> optimized_callback() -> opt_pre_handler() -> here
     *
     * When going back through (1), we need regs->nip to be setup properly
     * as it is used to determine the return address from the trap.
     * For (2), since nip is not honoured with optprobes, we instead setup
     * the link register properly so that the subsequent 'blr' in
     * arch_rethook_trampoline jumps back to the right instruction.
     *
     * For nip, we should set the address to the previous instruction since
     * we end up emulating it in kprobe_handler(), which increments the nip
     * again.
     */
    regs_set_return_ip(regs, orig_ret_address.wrapping_sub(4));
    (*regs).link = orig_ret_address;
}

// NOKPROBE_SYMBOL(arch_rethook_fixup_return);

static mut trampoline_p: kprobe = kprobe {
    addr: arch_rethook_trampoline as *mut kprobe_opcode_t,
    pre_handler: Some(trampoline_rethook_handler),
};

/* rethook initializer */
pub unsafe extern "C" fn arch_init_kprobes() -> c_int {
    register_kprobe(&raw mut trampoline_p)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
