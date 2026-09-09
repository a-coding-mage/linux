// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux kernel and architecture headers.

use core::ffi::c_void;

extern "C" {
    fn __insn_has_delay_slot(insn: mips_instruction) -> i32;
    fn __insn_is_compact_branch(insn: mips_instruction) -> bool;
    fn __compute_return_epc_for_insn(regs: *mut pt_regs, insn: mips_instruction);
    fn pr_notice(fmt: *const u8, ...);
    fn kmap_local_page(page: *mut page) -> *mut c_void;
    fn kunmap_local(addr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, len: usize) -> *mut c_void;
    fn flush_icache_range(start: c_ulong, end: c_ulong);
    fn instruction_pointer(regs: *mut pt_regs) -> c_ulong;
    fn instruction_pointer_set(regs: *mut pt_regs, value: c_ulong);
    fn user_mode(regs: *mut pt_regs) -> bool;
    fn uprobe_pre_sstep_notifier(regs: *mut pt_regs) -> bool;
    fn uprobe_post_sstep_notifier(regs: *mut pt_regs) -> bool;
    fn warn_on(condition: bool) -> bool;
}

type c_ulong = usize;
type uprobe_opcode_t = u32;

#[repr(C)]
pub union mips_instruction {
    pub word: u32,
    pub i_format: mips_i_format,
    pub r_format: mips_r_format,
    pub u_format: mips_u_format,
}

#[repr(C)]
pub struct mips_i_format { pub opcode: u32, pub rt: u32 }
#[repr(C)]
pub struct mips_r_format { pub func: u32 }
#[repr(C)]
pub struct mips_u_format { pub rt: u32 }

#[repr(C)] pub struct arch_uprobe { pub insn: [u32; 2], pub ixol: [u32; 2], pub resume_epc: c_ulong }
#[repr(C)] pub struct mm_struct;
#[repr(C)] pub struct page;
#[repr(C)] pub struct notifier_block;
#[repr(C)] pub struct die_args { pub regs: *mut pt_regs }
#[repr(C)] pub struct pt_regs { pub cp0_epc: c_ulong, pub regs: [c_ulong; 32] }
#[repr(C)] pub struct task_struct { pub thread: thread_struct, pub utask: *mut uprobe_task }
#[repr(C)] pub struct thread_struct { pub trap_nr: c_ulong }
#[repr(C)] pub struct uprobe_task { pub autask: arch_uprobe_task, pub xol_vaddr: c_ulong, pub vaddr: c_ulong }
#[repr(C)] pub struct arch_uprobe_task { pub saved_trap_nr: c_ulong }

const EINVAL: i32 = 22;
const ULONG_MAX: c_ulong = c_ulong::MAX;
const UPROBE_TRAP_NR: c_ulong = ULONG_MAX;
const PAGE_MASK: c_ulong = !(4096 - 1);
const UPROBE_BRK_UPROBE_XOL: u32 = 0;
const NOTIFY_DONE: i32 = 0;
const NOTIFY_STOP: i32 = 0x8000;
const DIE_UPROBE: c_ulong = 0;
const DIE_UPROBE_XOL: c_ulong = 0;
const spec_op: u32 = 0;
const bcond_op: u32 = 1;
const break_op: u32 = 0;
const teq_op: u32 = 1;
const tge_op: u32 = 2;
const tgeu_op: u32 = 3;
const tlt_op: u32 = 4;
const tltu_op: u32 = 5;
const tne_op: u32 = 6;
const teqi_op: u32 = 1;
const tgei_op: u32 = 2;
const tgeiu_op: u32 = 3;
const tlti_op: u32 = 4;
const tltiu_op: u32 = 5;
const tnei_op: u32 = 6;

static unsafe fn insn_has_delay_slot(insn: mips_instruction) -> usize {
    __insn_has_delay_slot(insn) as usize
}

pub unsafe fn arch_uprobe_analyze_insn(aup: *mut arch_uprobe, _mm: *mut mm_struct, addr: c_ulong) -> i32 {
    if addr & 0x03 != 0 { return -EINVAL; }
    let inst = mips_instruction { word: (*aup).insn[0] };
    if __insn_is_compact_branch(inst) {
        pr_notice(b"Uprobes for compact branches are not supported\0".as_ptr());
        return -EINVAL;
    }
    (*aup).ixol[0] = (*aup).insn[insn_has_delay_slot(inst)];
    (*aup).ixol[1] = UPROBE_BRK_UPROBE_XOL;
    0
}

pub unsafe fn is_trap_insn(insn: *mut uprobe_opcode_t) -> bool {
    let inst = mips_instruction { word: *insn };
    match inst.i_format.opcode {
        spec_op => matches!(inst.r_format.func, break_op | teq_op | tge_op | tgeu_op | tlt_op | tltu_op | tne_op),
        bcond_op => matches!(inst.u_format.rt, teqi_op | tgei_op | tgeiu_op | tlti_op | tltiu_op | tnei_op),
        _ => false,
    }
}

pub unsafe fn arch_uprobe_pre_xol(aup: *mut arch_uprobe, regs: *mut pt_regs, current: *mut task_struct) -> i32 {
    let utask = (*current).utask;
    (*aup).resume_epc = (*regs).cp0_epc + 4;
    if insn_has_delay_slot(mips_instruction { word: (*aup).insn[0] }) != 0 {
        __compute_return_epc_for_insn(regs, mips_instruction { word: (*aup).insn[0] });
        (*aup).resume_epc = (*regs).cp0_epc;
    }
    (*utask).autask.saved_trap_nr = (*current).thread.trap_nr;
    (*current).thread.trap_nr = UPROBE_TRAP_NR;
    (*regs).cp0_epc = (*utask).xol_vaddr;
    0
}

pub unsafe fn arch_uprobe_post_xol(aup: *mut arch_uprobe, regs: *mut pt_regs, current: *mut task_struct) -> i32 {
    let utask = (*current).utask;
    (*current).thread.trap_nr = (*utask).autask.saved_trap_nr;
    (*regs).cp0_epc = (*aup).resume_epc;
    0
}

pub unsafe fn arch_uprobe_xol_was_trapped(tsk: *mut task_struct) -> bool {
    (*tsk).thread.trap_nr != UPROBE_TRAP_NR
}

pub unsafe fn arch_uprobe_exception_notify(self_: *mut notifier_block, val: c_ulong, data: *mut c_void) -> i32 {
    let args = data as *mut die_args;
    let regs = (*args).regs;
    if warn_on(regs.is_null()) { return NOTIFY_DONE; }
    if !user_mode(regs) { return NOTIFY_DONE; }
    match val {
        DIE_UPROBE => if uprobe_pre_sstep_notifier(regs) { return NOTIFY_STOP; },
        DIE_UPROBE_XOL => if uprobe_post_sstep_notifier(regs) { return NOTIFY_STOP; },
        _ => {}
    }
    let _ = self_;
    0
}

pub unsafe fn arch_uprobe_abort_xol(aup: *mut arch_uprobe, regs: *mut pt_regs, current: *mut task_struct) {
    let _ = aup;
    let utask = (*current).utask;
    (*current).thread.trap_nr = (*utask).autask.saved_trap_nr;
    instruction_pointer_set(regs, (*utask).vaddr);
}

pub unsafe fn arch_uretprobe_hijack_return_addr(trampoline_vaddr: c_ulong, regs: *mut pt_regs) -> c_ulong {
    let ra = (*regs).regs[31];
    (*regs).regs[31] = trampoline_vaddr;
    ra
}

pub unsafe fn arch_uprobe_copy_ixol(page_: *mut page, vaddr: c_ulong, src: *mut c_void, len: c_ulong) {
    let kaddr = kmap_local_page(page_) as c_ulong;
    let kstart = kaddr + (vaddr & !PAGE_MASK);
    memcpy(kstart as *mut c_void, src, len);
    flush_icache_range(kstart, kstart + len);
    kunmap_local(kaddr as *mut c_void);
}

pub unsafe fn uprobe_get_swbp_addr(regs: *mut pt_regs) -> c_ulong { instruction_pointer(regs) }

pub unsafe fn arch_uprobe_skip_sstep(_auprobe: *mut arch_uprobe, _regs: *mut pt_regs) -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
