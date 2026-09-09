/* SPDX-License-Identifier: GPL-2.0 */
// Kprobes and Optprobes common header.
// Dependencies formerly supplied by asm/asm.h, asm/frame.h, and asm/insn.h
// are expected to be supplied by the surrounding translation unit.

#[cfg(target_arch = "x86_64")]
pub const SAVE_REGS_STRING: &str =
    "\tsubq $24, %rsp\n"
    "\tpushq %rdi\n"
    "\tpushq %rsi\n"
    "\tpushq %rdx\n"
    "\tpushq %rcx\n"
    "\tpushq %rax\n"
    "\tpushq %r8\n"
    "\tpushq %r9\n"
    "\tpushq %r10\n"
    "\tpushq %r11\n"
    "\tpushq %rbx\n"
    "\tpushq %rbp\n"
    "\tpushq %r12\n"
    "\tpushq %r13\n"
    "\tpushq %r14\n"
    "\tpushq %r15\n";
// ENCODE_FRAME_POINTER is supplied externally and occurs at the end of SAVE_REGS_STRING.

#[cfg(target_arch = "x86_64")]
pub const RESTORE_REGS_STRING: &str =
    "\tpopq %r15\n"
    "\tpopq %r14\n"
    "\tpopq %r13\n"
    "\tpopq %r12\n"
    "\tpopq %rbp\n"
    "\tpopq %rbx\n"
    "\tpopq %r11\n"
    "\tpopq %r10\n"
    "\tpopq %r9\n"
    "\tpopq %r8\n"
    "\tpopq %rax\n"
    "\tpopq %rcx\n"
    "\tpopq %rdx\n"
    "\tpopq %rsi\n"
    "\tpopq %rdi\n"
    "\taddq $24, %rsp\n";

#[cfg(not(target_arch = "x86_64"))]
pub const SAVE_REGS_STRING: &str =
    "\tsubl $4*4, %esp\n"
    "\tpushl %fs\n"
    "\tpushl %es\n"
    "\tpushl %ds\n"
    "\tpushl %eax\n"
    "\tpushl %ebp\n"
    "\tpushl %edi\n"
    "\tpushl %esi\n"
    "\tpushl %edx\n"
    "\tpushl %ecx\n"
    "\tpushl %ebx\n";
// ENCODE_FRAME_POINTER is supplied externally and occurs at the end of SAVE_REGS_STRING.

#[cfg(not(target_arch = "x86_64"))]
pub const RESTORE_REGS_STRING: &str =
    "\tpopl %ebx\n"
    "\tpopl %ecx\n"
    "\tpopl %edx\n"
    "\tpopl %esi\n"
    "\tpopl %edi\n"
    "\tpopl %ebp\n"
    "\tpopl %eax\n"
    "\taddl $7*4, %esp\n";

extern "C" {
    pub fn can_boost(insn: *mut insn, orig_addr: *mut core::ffi::c_void) -> bool;
    pub fn recover_probed_instruction(buf: *mut kprobe_opcode_t, addr: c_ulong) -> c_ulong;
    pub fn __copy_instruction(
        dest: *mut u8,
        src: *mut u8,
        real: *mut u8,
        insn: *mut insn,
    ) -> c_int;
    pub fn synthesize_reljump(dest: *mut core::ffi::c_void, from: *mut core::ffi::c_void, to: *mut core::ffi::c_void);
    pub fn synthesize_relcall(dest: *mut core::ffi::c_void, from: *mut core::ffi::c_void, to: *mut core::ffi::c_void);
}

#[cfg(feature = "CONFIG_OPTPROBES")]
extern "C" {
    pub fn setup_detour_execution(p: *mut kprobe, regs: *mut pt_regs, reenter: c_int) -> c_int;
    pub fn __recover_optprobed_insn(buf: *mut kprobe_opcode_t, addr: c_ulong) -> c_ulong;
}

#[cfg(not(feature = "CONFIG_OPTPROBES"))]
pub unsafe fn setup_detour_execution(_p: *mut kprobe, _regs: *mut pt_regs, _reenter: c_int) -> c_int { 0 }

#[cfg(not(feature = "CONFIG_OPTPROBES"))]
pub unsafe fn __recover_optprobed_insn(_buf: *mut kprobe_opcode_t, addr: c_ulong) -> c_ulong { addr }

// Types and aliases below are supplied by the surrounding translation unit.
type c_int = core::ffi::c_int;
type c_ulong = core::ffi::c_ulong;
// struct insn; struct kprobe; struct pt_regs; type kprobe_opcode_t = ...;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
