// SPDX-License-Identifier: GPL-2.0-or-later
/* User-space Probes (UProbes) for x86 -- source-level Rust translation. */

// Kernel-provided types, constants, functions, and macros are supplied by the
// surrounding kernel translation unit.

const UPROBE_FIX_IP: u32 = 0x01;
const UPROBE_FIX_CALL: u32 = 0x02;
const UPROBE_FIX_SETF: u32 = 0x04;
const UPROBE_FIX_RIP_SI: u32 = 0x08;
const UPROBE_FIX_RIP_DI: u32 = 0x10;
const UPROBE_FIX_RIP_BX: u32 = 0x20;
const UPROBE_FIX_RIP_MASK: u32 = UPROBE_FIX_RIP_SI | UPROBE_FIX_RIP_DI | UPROBE_FIX_RIP_BX;
const UPROBE_TRAP_NR: u32 = u32::MAX;

#[inline]
unsafe fn opcode1(insn: *const insn) -> u8 { (*insn).opcode.bytes[0] }
#[inline]
unsafe fn opcode2(insn: *const insn) -> u8 { (*insn).opcode.bytes[1] }
#[inline]
unsafe fn opcode3(insn: *const insn) -> u8 { (*insn).opcode.bytes[2] }
#[inline]
unsafe fn modrm_reg(insn: *const insn) -> u8 { X86_MODRM_REG((*insn).modrm.value) }

#[cfg(any(CONFIG_X86_32, CONFIG_IA32_EMULATION))]
static mut GOOD_INSNS_32: [u32; 8] = [0; 8];
#[cfg(CONFIG_X86_64)]
static mut GOOD_INSNS_64: [u32; 8] = [0; 8];
static mut GOOD_2BYTE_INSNS: [u32; 8] = [0; 8];

unsafe fn is_prefix_bad(insn: *mut insn) -> bool {
    let mut p: insn_byte_t = 0;
    for_each_insn_prefix!(insn, p, {
        let attr = inat_get_opcode_attribute(p);
        match attr {
            INAT_MAKE_PREFIX!(INAT_PFX_ES) |
            INAT_MAKE_PREFIX!(INAT_PFX_CS) |
            INAT_MAKE_PREFIX!(INAT_PFX_DS) |
            INAT_MAKE_PREFIX!(INAT_PFX_SS) |
            INAT_MAKE_PREFIX!(INAT_PFX_LOCK) => return true,
            _ => {}
        }
    });
    false
}

unsafe fn uprobe_init_insn(auprobe: *mut arch_uprobe, insn: *mut insn) -> i32 {
    if is_prefix_bad(insn) || insn_masking_exception(insn) { return -ENOTSUPP; }
    let good = if (*insn).x86_64 { GOOD_INSNS_64.as_ptr() } else { GOOD_INSNS_32.as_ptr() };
    if test_bit(opcode1(insn) as usize, good as *const unsigned_long) { return 0; }
    if (*insn).opcode.nbytes == 2 && test_bit(opcode2(insn) as usize, GOOD_2BYTE_INSNS.as_ptr() as *const unsigned_long) { return 0; }
    -ENOTSUPP
}

#[repr(C)]
struct UretprobeSyscallArgs { r11: unsigned_long, cx: unsigned_long, ax: unsigned_long }

unsafe fn trampoline_check_ip(tramp: unsigned_long) -> unsigned_long {
    tramp + (uretprobe_syscall_check as unsigned_long - uretprobe_trampoline_entry as unsigned_long)
}

unsafe fn riprel_analyze(auprobe: *mut arch_uprobe, insn: *mut insn) {
    if !insn_rip_relative(insn) { return; }
    let mut cursor: *mut u8;
    if (*insn).rex_prefix.nbytes != 0 {
        cursor = (*auprobe).insn.as_mut_ptr().add(insn_offset_rex_prefix(insn) as usize);
        *cursor &= 0xfe;
    }
    if (*insn).vex_prefix.nbytes >= 3 {
        cursor = (*auprobe).insn.as_mut_ptr().add(insn_offset_vex_prefix(insn) as usize + 1);
        *cursor |= 0x60;
    }
    let reg = modrm_reg(insn);
    let mut reg2 = if (*insn).vex_prefix.nbytes != 0 { (*insn).vex_prefix.bytes[2] } else { 0xff };
    reg2 = ((reg2 >> 3) & 7) ^ 7;
    if reg != 6 && reg2 != 6 { reg2 = 6; (*auprobe).defparam.fixups |= UPROBE_FIX_RIP_SI; }
    else if reg != 7 && reg2 != 7 { reg2 = 7; (*auprobe).defparam.fixups |= UPROBE_FIX_RIP_DI; }
    else { reg2 = 3; (*auprobe).defparam.fixups |= UPROBE_FIX_RIP_BX; }
    cursor = (*auprobe).insn.as_mut_ptr().add(insn_offset_modrm(insn) as usize);
    *cursor = 0x80 | (reg << 3) | reg2;
}

unsafe fn scratch_reg(auprobe: *mut arch_uprobe, regs: *mut pt_regs) -> *mut unsigned_long {
    if (*auprobe).defparam.fixups & UPROBE_FIX_RIP_SI != 0 { return &mut (*regs).si; }
    if (*auprobe).defparam.fixups & UPROBE_FIX_RIP_DI != 0 { return &mut (*regs).di; }
    &mut (*regs).bx
}

unsafe fn riprel_pre_xol(auprobe: *mut arch_uprobe, regs: *mut pt_regs) {
    if (*auprobe).defparam.fixups & UPROBE_FIX_RIP_MASK != 0 {
        let task = current.utask;
        let sr = scratch_reg(auprobe, regs);
        (*task).autask.saved_scratch_register = *sr;
        *sr = (*task).vaddr + (*auprobe).defparam.ilen as unsigned_long;
    }
}

unsafe fn riprel_post_xol(auprobe: *mut arch_uprobe, regs: *mut pt_regs) {
    if (*auprobe).defparam.fixups & UPROBE_FIX_RIP_MASK != 0 {
        let task = current.utask;
        *scratch_reg(auprobe, regs) = (*task).autask.saved_scratch_register;
    }
}

unsafe fn sizeof_long(regs: *mut pt_regs) -> i32 { if user_64bit_mode(regs) { 8 } else { 4 } }

unsafe fn emulate_push_stack(regs: *mut pt_regs, val: unsigned_long) -> i32 {
    let sp = (*regs).sp - sizeof_long(regs) as unsigned_long;
    if copy_to_user(sp as *mut core::ffi::c_void, &val as *const _ as *const core::ffi::c_void, sizeof_long(regs) as usize) != 0 { return -EFAULT; }
    (*regs).sp = sp; 0
}

unsafe fn default_pre_xol_op(a: *mut arch_uprobe, r: *mut pt_regs) -> i32 { riprel_pre_xol(a, r); 0 }
unsafe fn default_abort_op(a: *mut arch_uprobe, r: *mut pt_regs) { riprel_post_xol(a, r); }

unsafe fn branch_is_call(a: *mut arch_uprobe) -> bool { (*a).branch.opc1 == 0xe8 }

unsafe fn branch_emulate_op(a: *mut arch_uprobe, regs: *mut pt_regs) -> bool {
    let new_ip = { (*regs).ip += (*a).branch.ilen as unsigned_long; (*regs).ip };
    let mut offs = (*a).branch.offs as i64 as unsigned_long;
    if branch_is_call(a) {
        if emulate_push_stack(regs, new_ip) != 0 { return false; }
        if shstk_push(new_ip) == -EFAULT { (*regs).sp += sizeof_long(regs) as unsigned_long; return false; }
    }
    (*regs).ip = new_ip.wrapping_add(offs); true
}

unsafe fn push_emulate_op(a: *mut arch_uprobe, regs: *mut pt_regs) -> bool {
    let src = (regs as *mut u8).add((*a).push.reg_offset as usize) as *mut unsigned_long;
    if emulate_push_stack(regs, *src) != 0 { return false; }
    (*regs).ip += (*a).push.ilen as unsigned_long; true
}

// Remaining architecture entry points retain the kernel ABI and are declared
// here for the surrounding translation unit to provide.
extern "C" {
    static mut current: task_struct;
    fn uprobe_pre_sstep_notifier(regs: *mut pt_regs) -> bool;
    fn uprobe_post_sstep_notifier(regs: *mut pt_regs) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
