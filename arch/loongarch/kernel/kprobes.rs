// SPDX-License-Identifier: GPL-2.0-only

// External kernel dependencies and build-time architecture definitions are supplied by
// the surrounding translation unit.
extern "C" {
    static mut current_kprobe: *mut Kprobe;
    static mut kprobe_ctlblk: KprobeCtlblk;
}

pub const KPROBE_BP_INSN: u32 = __emit_break(BRK_KPROBE_BP);
pub const KPROBE_SSTEPBP_INSN: u32 = __emit_break(BRK_KPROBE_SSTEPBP);

#[repr(C)]
pub struct LoongarchInstruction { pub word: u32 }

#[repr(C)]
pub struct AInsn {
    pub insn: *mut u32,
    pub restore: usize,
}

#[repr(C)]
pub struct Kprobe {
    pub addr: *mut u32,
    pub opcode: u32,
    pub ainsn: AInsn,
    pub pre_handler: Option<unsafe extern "C" fn(*mut Kprobe, *mut PtRegs) -> i32>,
    pub post_handler: Option<unsafe extern "C" fn(*mut Kprobe, *mut PtRegs, i32)>,
}

#[repr(C)]
pub struct PreviousKprobe { pub kp: *mut Kprobe, pub status: i32 }

#[repr(C)]
pub struct KprobeCtlblk {
    pub prev_kprobe: PreviousKprobe,
    pub kprobe_status: i32,
    pub saved_status: usize,
}

#[repr(C)]
pub struct PtRegs { pub csr_prmd: usize, pub csr_era: usize }

extern "C" {
    fn __emit_break(x: u32) -> u32;
    fn insns_not_supported(insn: LoongarchInstruction) -> bool;
    fn insns_need_simulation(insn: LoongarchInstruction) -> bool;
    fn get_insn_slot() -> *mut u32;
    fn free_insn_slot(slot: *mut u32, dirty: i32);
    fn larch_insn_text_copy(dst: *mut u32, src: *const u32, size: usize);
    fn kprobe_running() -> *mut Kprobe;
    fn get_kprobe_ctlblk() -> *mut KprobeCtlblk;
    fn get_kprobe(addr: *mut u32) -> *mut Kprobe;
    fn kprobes_inc_nmissed_count(p: *mut Kprobe);
    fn arch_simulate_insn(insn: LoongarchInstruction, regs: *mut PtRegs);
    fn instruction_pointer_set(regs: *mut PtRegs, value: usize);
    fn instruction_pointer(regs: *mut PtRegs) -> usize;
    fn reset_current_kprobe();
    fn preempt_disable();
    fn preempt_enable_no_resched();
    fn dump_kprobe(p: *mut Kprobe);
    fn kprobe_add_area_blacklist(start: usize, end: usize) -> i32;
    fn pr_warn(fmt: *const u8, ...);
    fn warn_on(x: bool);
    fn warn_on_once(x: bool);
    fn bug() -> !;
    static __irqentry_text_start: u8;
    static __irqentry_text_end: u8;
}

const LOONGARCH_INSN_SIZE: usize = 4;
const CSR_PRMD_PIE: usize = 1 << 2;
const KPROBE_REENTER: i32 = 0;
const KPROBE_HIT_SSDONE: i32 = 1;
const KPROBE_HIT_ACTIVE: i32 = 2;
const KPROBE_HIT_SS: i32 = 3;
const EILSEQ: i32 = 84;
const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const BRK_KPROBE_BP: u32 = 0;
const BRK_KPROBE_SSTEPBP: u32 = 0;

unsafe fn arch_prepare_ss_slot(p: *mut Kprobe) {
    (*p).ainsn.insn.add(0).write((*p).addr.read());
    (*p).ainsn.insn.add(1).write(KPROBE_SSTEPBP_INSN);
    (*p).ainsn.restore = (*p).addr as usize + LOONGARCH_INSN_SIZE;
}

unsafe fn arch_prepare_simulate(p: *mut Kprobe) { (*p).ainsn.restore = 0; }

pub unsafe extern "C" fn arch_prepare_kprobe(p: *mut Kprobe) -> i32 {
    let mut insn = LoongarchInstruction { word: 0 };
    if ((*p).addr as usize & 0x3) != 0 { return -EILSEQ; }
    (*p).opcode = (*p).addr.read();
    insn.word = (*p).opcode;
    if insns_not_supported(insn) { return -EINVAL; }
    if insns_need_simulation(insn) { (*p).ainsn.insn = core::ptr::null_mut(); }
    else {
        (*p).ainsn.insn = get_insn_slot();
        if (*p).ainsn.insn.is_null() { return -ENOMEM; }
    }
    if !(*p).ainsn.insn.is_null() { arch_prepare_ss_slot(p); } else { arch_prepare_simulate(p); }
    0
}

pub unsafe extern "C" fn arch_arm_kprobe(p: *mut Kprobe) {
    let insn = KPROBE_BP_INSN;
    larch_insn_text_copy((*p).addr, &insn, LOONGARCH_INSN_SIZE);
}

pub unsafe extern "C" fn arch_disarm_kprobe(p: *mut Kprobe) {
    let insn = (*p).opcode;
    larch_insn_text_copy((*p).addr, &insn, LOONGARCH_INSN_SIZE);
}

pub unsafe extern "C" fn arch_remove_kprobe(p: *mut Kprobe) {
    if !(*p).ainsn.insn.is_null() { free_insn_slot((*p).ainsn.insn, 0); (*p).ainsn.insn = core::ptr::null_mut(); }
}

unsafe fn save_previous_kprobe(kcb: *mut KprobeCtlblk) { (*kcb).prev_kprobe.kp = kprobe_running(); (*kcb).prev_kprobe.status = (*kcb).kprobe_status; }
unsafe fn restore_previous_kprobe(kcb: *mut KprobeCtlblk) { current_kprobe = (*kcb).prev_kprobe.kp; (*kcb).kprobe_status = (*kcb).prev_kprobe.status; }
unsafe fn set_current_kprobe(p: *mut Kprobe) { current_kprobe = p; }
unsafe fn save_local_irqflag(kcb: *mut KprobeCtlblk, regs: *mut PtRegs) { (*kcb).saved_status = (*regs).csr_prmd; (*regs).csr_prmd &= !CSR_PRMD_PIE; }
unsafe fn restore_local_irqflag(kcb: *mut KprobeCtlblk, regs: *mut PtRegs) { (*regs).csr_prmd = (*kcb).saved_status; }

unsafe fn post_kprobe_handler(cur: *mut Kprobe, kcb: *mut KprobeCtlblk, regs: *mut PtRegs) {
    if (*cur).ainsn.restore != 0 { instruction_pointer_set(regs, (*cur).ainsn.restore); }
    if (*kcb).kprobe_status == KPROBE_REENTER { restore_previous_kprobe(kcb); preempt_enable_no_resched(); return; }
    (*kcb).kprobe_status = KPROBE_HIT_SSDONE;
    if let Some(handler) = (*cur).post_handler { handler(cur, regs, 0); }
    reset_current_kprobe(); preempt_enable_no_resched();
}

unsafe fn setup_singlestep(p: *mut Kprobe, regs: *mut PtRegs, kcb: *mut KprobeCtlblk, reenter: i32) {
    if reenter != 0 { save_previous_kprobe(kcb); set_current_kprobe(p); (*kcb).kprobe_status = KPROBE_REENTER; } else { (*kcb).kprobe_status = KPROBE_HIT_SS; }
    if !(*p).ainsn.insn.is_null() { save_local_irqflag(kcb, regs); (*regs).csr_era = (*p).ainsn.insn as usize; }
    else { let insn = LoongarchInstruction { word: (*p).opcode }; arch_simulate_insn(insn, regs); post_kprobe_handler(p, kcb, regs); }
}

unsafe fn reenter_kprobe(p: *mut Kprobe, regs: *mut PtRegs, kcb: *mut KprobeCtlblk) -> bool {
    match (*kcb).kprobe_status {
        KPROBE_HIT_SSDONE | KPROBE_HIT_ACTIVE => { kprobes_inc_nmissed_count(p); setup_singlestep(p, regs, kcb, 1); }
        KPROBE_HIT_SS | KPROBE_REENTER => { pr_warn(b"Failed to recover from reentered kprobes.\0".as_ptr()); dump_kprobe(p); bug(); }
        _ => { warn_on(true); return false; }
    } true
}

pub unsafe extern "C" fn kprobe_breakpoint_handler(regs: *mut PtRegs) -> bool {
    let addr = (*regs).csr_era as *mut u32; preempt_disable(); let kcb = get_kprobe_ctlblk(); let cur = kprobe_running(); let p = get_kprobe(addr);
    if !p.is_null() { if !cur.is_null() { if reenter_kprobe(p, regs, kcb) { return true; } } else { set_current_kprobe(p); (*kcb).kprobe_status = KPROBE_HIT_ACTIVE; if (*p).pre_handler.map_or(true, |h| h(p, regs) == 0) { setup_singlestep(p, regs, kcb, 0); } else { reset_current_kprobe(); preempt_enable_no_resched(); } return true; } }
    if addr.read() != KPROBE_BP_INSN { (*regs).csr_era = addr as usize; preempt_enable_no_resched(); return true; }
    preempt_enable_no_resched(); false
}

pub unsafe extern "C" fn kprobe_singlestep_handler(regs: *mut PtRegs) -> bool {
    let cur = kprobe_running(); let kcb = get_kprobe_ctlblk(); let addr = instruction_pointer(regs);
    if !cur.is_null() && ((*kcb).kprobe_status & (KPROBE_HIT_SS | KPROBE_REENTER)) != 0 && ((*cur).ainsn.insn.add(1) as usize == addr) { restore_local_irqflag(kcb, regs); post_kprobe_handler(cur, kcb, regs); return true; }
    preempt_enable_no_resched(); false
}

pub unsafe extern "C" fn kprobe_fault_handler(regs: *mut PtRegs, _trapnr: i32) -> bool {
    let cur = kprobe_running(); let kcb = get_kprobe_ctlblk();
    match (*kcb).kprobe_status { KPROBE_HIT_SS | KPROBE_REENTER => { (*regs).csr_era = (*cur).addr as usize; warn_on_once(instruction_pointer(regs) == 0); if (*kcb).kprobe_status == KPROBE_REENTER { restore_previous_kprobe(kcb); } else { restore_local_irqflag(kcb, regs); reset_current_kprobe(); } preempt_enable_no_resched(); }, _ => {} } false
}

pub unsafe extern "C" fn arch_populate_kprobe_blacklist() -> i32 { kprobe_add_area_blacklist(&__irqentry_text_start as *const u8 as usize, &__irqentry_text_end as *const u8 as usize) }
pub unsafe extern "C" fn arch_init_kprobes() -> i32 { 0 }
pub unsafe extern "C" fn arch_trampoline_kprobe(_p: *mut Kprobe) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
