// SPDX-License-Identifier: GPL-2.0
/* User-space Probes (UProbes) for s390. */

const UPROBE_TRAP_NR: u32 = u32::MAX;
const EMU_ILLEGAL_OP: i32 = 1;
const EMU_SPECIFICATION: i32 = 2;
const EMU_ADDRESSING: i32 = 3;

macro_rules! emu_load_ril { ($ptr:expr, $out:expr) => {{
    let mask = core::mem::size_of_val(&*$ptr) - 1; let mut rc = 0;
    if ($ptr as usize) & mask != 0 { rc = EMU_SPECIFICATION; }
    else if get_user($out, $ptr) != 0 { rc = EMU_ADDRESSING; } rc
}}; }
macro_rules! emu_store_ril { ($regs:expr, $ptr:expr, $input:expr) => {{
    let mask = core::mem::size_of_val(&*$ptr) - 1; let mut rc = 0;
    if ($ptr as usize) & mask != 0 { rc = EMU_SPECIFICATION; }
    else if put_user($input, $ptr) != 0 { rc = EMU_ADDRESSING; }
    if rc == 0 { sim_stor_event($regs, $ptr as *mut c_void, (mask + 1) as c_int); } rc
}}; }
macro_rules! emu_cmp_ril { ($regs:expr, $ptr:expr, $cmp:expr) => {{
    let mask = core::mem::size_of_val(&*$ptr) - 1; let mut rc = 0;
    if ($ptr as usize) & mask != 0 { rc = EMU_SPECIFICATION; }
    else { let mut input = core::mem::zeroed(); if get_user(&mut input, $ptr) != 0 { rc = EMU_ADDRESSING; } else if input > *$cmp { psw_bits((*$regs).psw).cc = 1; } else if input < *$cmp { psw_bits((*$regs).psw).cc = 2; } else { psw_bits((*$regs).psw).cc = 0; } } rc
}}; }

pub unsafe fn arch_uprobe_analyze_insn(auprobe: *mut arch_uprobe, _mm: *mut mm_struct, _addr: c_ulong) -> c_int {
    probe_is_prohibited_opcode((*auprobe).insn)
}

pub unsafe fn arch_uprobe_pre_xol(auprobe: *mut arch_uprobe, regs: *mut pt_regs) -> c_int {
    if psw_bits((*regs).psw).eaba == PSW_BITS_AMODE_24BIT || psw_bits((*regs).psw).eaba == PSW_BITS_AMODE_31BIT { return -EINVAL; }
    clear_thread_flag(TIF_PER_TRAP);
    (*auprobe).saved_per = psw_bits((*regs).psw).per;
    (*auprobe).saved_int_code = (*regs).int_code;
    (*regs).int_code = UPROBE_TRAP_NR;
    (*regs).psw.addr = (*current).utask.xol_vaddr;
    set_tsk_thread_flag(current, TIF_UPROBE_SINGLESTEP);
    update_cr_regs(current);
    0
}

pub unsafe fn arch_uprobe_xol_was_trapped(tsk: *mut task_struct) -> bool {
    (*task_pt_regs(tsk)).int_code != UPROBE_TRAP_NR
}

unsafe fn check_per_event(cause: u16, control: c_ulong, regs: *mut pt_regs) -> c_int {
    if (*regs).psw.mask & PSW_MASK_PER == 0 { return 0; }
    if control == 0 { return 1; }
    if control & 0x20200000 != 0 && cause & 0x2000 != 0 { return 1; }
    if cause & 0x8000 != 0 {
        if control & 0x80800000 == 0x80000000 { return 1; }
        if control & 0x80800000 == 0x80800000 && (*regs).psw.addr >= (*current).thread.per_user.start && (*regs).psw.addr <= (*current).thread.per_user.end { return 1; }
    }
    0
}

pub unsafe fn arch_uprobe_post_xol(auprobe: *mut arch_uprobe, regs: *mut pt_regs) -> c_int {
    let fixup = probe_get_fixup_type((*auprobe).insn);
    let utask = (*current).utask;
    clear_tsk_thread_flag(current, TIF_UPROBE_SINGLESTEP);
    update_cr_regs(current);
    psw_bits((*regs).psw).per = (*auprobe).saved_per;
    (*regs).int_code = (*auprobe).saved_int_code;
    if fixup & FIXUP_PSW_NORMAL != 0 { (*regs).psw.addr = (*regs).psw.addr.wrapping_add(utask.vaddr.wrapping_sub(utask.xol_vaddr)); }
    if fixup & FIXUP_RETURN_REGISTER != 0 { let reg = (((*auprobe).insn[0] & 0xf0) >> 4) as usize; (*regs).gprs[reg] = (*regs).gprs[reg].wrapping_add(utask.vaddr.wrapping_sub(utask.xol_vaddr)); }
    if fixup & FIXUP_BRANCH_NOT_TAKEN != 0 { let ilen = insn_length((*auprobe).insn[0] >> 8); if (*regs).psw.addr.wrapping_sub(utask.xol_vaddr) == ilen { (*regs).psw.addr = utask.vaddr.wrapping_add(ilen); } }
    if check_per_event((*current).thread.per_event.cause, (*current).thread.per_user.control, regs) != 0 { (*current).thread.per_event.address = utask.vaddr; set_thread_flag(TIF_PER_TRAP); }
    0
}

pub unsafe fn arch_uprobe_exception_notify(self_: *mut notifier_block, val: c_ulong, data: *mut c_void) -> c_int {
    let args = data as *mut die_args; let regs = (*args).regs;
    if !user_mode(regs) || (*regs).int_code & 0x200 != 0 { return NOTIFY_DONE; }
    match val { DIE_BPT => if uprobe_pre_sstep_notifier(regs) != 0 { return NOTIFY_STOP; }, DIE_SSTEP => if uprobe_post_sstep_notifier(regs) != 0 { return NOTIFY_STOP; }, _ => {} }
    NOTIFY_DONE
}

pub unsafe fn arch_uprobe_abort_xol(auprobe: *mut arch_uprobe, regs: *mut pt_regs) { clear_thread_flag(TIF_UPROBE_SINGLESTEP); (*regs).int_code = (*auprobe).saved_int_code; (*regs).psw.addr = (*current).utask.vaddr; (*current).thread.per_event.address = (*current).utask.vaddr; }

pub unsafe fn arch_uretprobe_hijack_return_addr(trampoline: c_ulong, regs: *mut pt_regs) -> c_ulong { let orig = (*regs).gprs[14]; (*regs).gprs[14] = trampoline; orig }
pub unsafe fn arch_uretprobe_is_alive(ret: *mut return_instance, ctx: rp_check, regs: *mut pt_regs) -> bool { if ctx == RP_CHECK_CHAIN_CALL { user_stack_pointer(regs) <= (*ret).stack } else { user_stack_pointer(regs) < (*ret).stack } }

#[repr(C, packed)]
struct insn_ril { opc0: u8, reg_opc1: u8, disp: i32 }
impl insn_ril { fn reg(&self) -> u8 { self.reg_opc1 >> 4 } fn opc1(&self) -> u8 { self.reg_opc1 & 0xf } }

#[repr(C)]
union split_register { u64_: u64, u32_: [u32; 2], u16_: [u16; 4], s64_: i64, s32_: [i32; 2], s16_: [i16; 4] }

unsafe fn sim_stor_event(regs: *mut pt_regs, addr: *mut c_void, len: c_int) {
    if (*regs).psw.mask & PSW_MASK_PER == 0 || (*current).thread.per_user.control & PER_EVENT_STORE == 0 { return; }
    if (*current).thread.per_user.start as *mut c_void > (addr as usize + len as usize) as *mut c_void || (*current).thread.per_user.end as *mut c_void < addr { return; }
    (*current).thread.per_event.address = (*regs).psw.addr; (*current).thread.per_event.cause = PER_EVENT_STORE >> 16; set_thread_flag(TIF_PER_TRAP);
}

unsafe fn handle_insn_ril(auprobe: *mut arch_uprobe, regs: *mut pt_regs) {
    let insn = &*((&(*auprobe).insn as *const _ as *const insn_ril));
    let rx = (&mut (*regs).gprs[insn.reg() as usize] as *mut _ as *mut split_register);
    let uptr = ((*regs).psw.addr as isize + (insn.disp as isize * 2)) as *mut c_void;
    let ilen = insn_length(insn.opc0); let mut rc = 0;
    match insn.opc0 { 0xc0 => if insn.opc1() == 0 { (*rx).u64_ = uptr as u64; }, 0xc4 => match insn.opc1() { 0x02 => rc = emu_load_ril!(uptr as *mut u16, &mut (*rx).u32_[1]), 0x04 => rc = emu_load_ril!(uptr as *mut i16, &mut (*rx).u64_), 0x05 => rc = emu_load_ril!(uptr as *mut i16, &mut (*rx).u32_[1]), 0x06 => rc = emu_load_ril!(uptr as *mut u16, &mut (*rx).u64_), 0x08 => rc = emu_load_ril!(uptr as *mut u64, &mut (*rx).u64_), 0x0c => rc = emu_load_ril!(uptr as *mut i32, &mut (*rx).u64_), 0x0d => rc = emu_load_ril!(uptr as *mut u32, &mut (*rx).u32_[1]), 0x0e => rc = emu_load_ril!(uptr as *mut u32, &mut (*rx).u64_), 0x07 => rc = emu_store_ril!(regs, uptr as *mut u16, &(*rx).u16_[3]), 0x0b => rc = emu_store_ril!(regs, uptr as *mut u64, &(*rx).u64_), 0x0f => rc = emu_store_ril!(regs, uptr as *mut u32, &(*rx).u32_[1]), _ => {} }, 0xc6 => match insn.opc1() { 0x04 => rc = emu_cmp_ril!(regs, uptr as *mut i16, &(*rx).s64_), 0x05 => rc = emu_cmp_ril!(regs, uptr as *mut i16, &(*rx).s32_[1]), 0x06 => rc = emu_cmp_ril!(regs, uptr as *mut u16, &(*rx).u64_), 0x07 => rc = emu_cmp_ril!(regs, uptr as *mut u16, &(*rx).u32_[1]), 0x08 => rc = emu_cmp_ril!(regs, uptr as *mut i64, &(*rx).s64_), 0x0a => rc = emu_cmp_ril!(regs, uptr as *mut u64, &(*rx).u64_), 0x0c => rc = emu_cmp_ril!(regs, uptr as *mut i32, &(*rx).s64_), 0x0d => rc = emu_cmp_ril!(regs, uptr as *mut i32, &(*rx).s32_[1]), 0x0e => rc = emu_cmp_ril!(regs, uptr as *mut u32, &(*rx).u64_), 0x0f => rc = emu_cmp_ril!(regs, uptr as *mut u32, &(*rx).u32_[1]), _ => {} }, _ => {} }
    (*regs).psw.addr = __forward_psw((*regs).psw, ilen);
    match rc { EMU_ILLEGAL_OP => { (*regs).int_code = ilen << 16 | 1; do_report_trap(regs, SIGILL, ILL_ILLOPC, core::ptr::null_mut()); }, EMU_SPECIFICATION => { (*regs).int_code = ilen << 16 | 6; do_report_trap(regs, SIGILL, ILL_ILLOPC, core::ptr::null_mut()); }, EMU_ADDRESSING => { (*regs).int_code = ilen << 16 | 5; do_report_trap(regs, SIGSEGV, SEGV_MAPERR, core::ptr::null_mut()); }, _ => {} }
}

pub unsafe fn arch_uprobe_skip_sstep(auprobe: *mut arch_uprobe, regs: *mut pt_regs) -> bool { if psw_bits((*regs).psw).eaba == PSW_BITS_AMODE_24BIT || psw_bits((*regs).psw).eaba == PSW_BITS_AMODE_31BIT { (*regs).psw.addr = __rewind_psw((*regs).psw, UPROBE_SWBP_INSN_SIZE); do_report_trap(regs, SIGILL, ILL_ILLADR, core::ptr::null_mut()); return true; } if probe_is_insn_relative_long((*auprobe).insn) { handle_insn_ril(auprobe, regs); return true; } false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
