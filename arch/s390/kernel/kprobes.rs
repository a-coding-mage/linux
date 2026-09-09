// SPDX-License-Identifier: GPL-2.0+
/* Kernel Probes (KProbes), s390 port. */

// Linux headers and NOKPROBE_SYMBOL annotations are supplied by other files.

pub static mut CURRENT_KPROBE: *mut kprobe = core::ptr::null_mut();
pub static mut KPROBE_CTLBLK: kprobe_ctlblk = unsafe { core::mem::zeroed() };
pub static mut KRETPROBE_BLACKLIST: [kretprobe_blackpoint; 0] = [];

pub unsafe fn alloc_insn_page() -> *mut core::ffi::c_void {
    let page = execmem_alloc(EXECMEM_KPROBES, PAGE_SIZE);
    if page.is_null() { return core::ptr::null_mut(); }
    set_memory_rox(page as usize, 1);
    page
}

unsafe fn copy_instruction(p: *mut kprobe) {
    let mut insn: [kprobe_opcode_t; MAX_INSN_SIZE] = core::mem::zeroed();
    let len = insn_length((*(*p).addr >> 8) as _);
    core::ptr::copy_nonoverlapping((*p).addr, insn.as_mut_ptr(), len as usize);
    (*p).opcode = insn[0];
    if probe_is_insn_relative_long(&insn[0]) {
        let disp = *(insn.as_ptr().add(1) as *const i32) as i64;
        let addr = (*p).addr as usize as u64;
        let new_addr = (*p).ainsn.insn as usize as u64;
        let new_disp = ((addr.wrapping_add((disp * 2) as u64)).wrapping_sub(new_addr)) / 2;
        *(insn.as_mut_ptr().add(1) as *mut i32) = new_disp as i32;
    }
    s390_kernel_write((*p).ainsn.insn, insn.as_ptr() as *const _, len);
}

unsafe fn can_probe(paddr: usize) -> bool {
    if paddr & 1 != 0 { return false; }
    let mut offset = 0usize;
    if !kallsyms_lookup_size_offset(paddr, core::ptr::null_mut(), &mut offset) { return false; }
    let mut addr = paddr - offset;
    while addr < paddr {
        let mut insn: kprobe_opcode_t = 0;
        if copy_from_kernel_nofault(&mut insn as *mut _, addr as *const _, core::mem::size_of::<kprobe_opcode_t>()) != 0 { return false; }
        if insn >> 8 == 0 {
            if insn != BREAKPOINT_INSTRUCTION { return false; }
            let kp = get_kprobe(addr as *mut _);
            if kp.is_null() { return false; }
            insn = (*kp).opcode;
        }
        addr += insn_length((insn >> 8) as _) as usize;
    }
    addr == paddr
}

pub unsafe fn arch_prepare_kprobe(p: *mut kprobe) -> i32 {
    if !can_probe((*p).addr as usize) || probe_is_prohibited_opcode((*p).addr) { return -EINVAL; }
    (*p).ainsn.insn = get_insn_slot();
    if (*p).ainsn.insn.is_null() { return -ENOMEM; }
    copy_instruction(p); 0
}

#[repr(C)]
pub struct swap_insn_args { pub p: *mut kprobe, pub arm_kprobe: u32 }

unsafe fn swap_instruction(data: *mut core::ffi::c_void) -> i32 {
    let args = &*(data as *const swap_insn_args);
    let opc: u16 = if args.arm_kprobe != 0 { BREAKPOINT_INSTRUCTION } else { (*args.p).opcode };
    s390_kernel_write((*args.p).addr, &opc as *const _ as *const _, core::mem::size_of::<u16>()); 0
}

pub unsafe fn arch_arm_kprobe(p: *mut kprobe) {
    let mut args = swap_insn_args { p, arm_kprobe: 1 };
    if cpu_has_seq_insn() { swap_instruction(&mut args as *mut _ as _); text_poke_sync(); }
    else { stop_machine_cpuslocked(swap_instruction, &mut args as *mut _ as _, core::ptr::null_mut()); }
}
pub unsafe fn arch_disarm_kprobe(p: *mut kprobe) {
    let mut args = swap_insn_args { p, arm_kprobe: 0 };
    if cpu_has_seq_insn() { swap_instruction(&mut args as *mut _ as _); text_poke_sync(); }
    else { stop_machine_cpuslocked(swap_instruction, &mut args as *mut _ as _, core::ptr::null_mut()); }
}
pub unsafe fn arch_remove_kprobe(p: *mut kprobe) { if !(*p).ainsn.insn.is_null() { free_insn_slot((*p).ainsn.insn, 0); (*p).ainsn.insn = core::ptr::null_mut(); } }

unsafe fn enable_singlestep(kcb: *mut kprobe_ctlblk, regs: *mut pt_regs, ip: usize) {
    let mut ctl: [ctlreg; 3] = core::mem::zeroed();
    ctl[0].val = PER_EVENT_IFETCH; ctl[1].val = ip; ctl[2].val = ip;
    __local_ctl_store(9, 11, (*kcb).kprobe_saved_ctl.as_mut_ptr());
    (*kcb).kprobe_saved_imask = (*regs).psw.mask & (PSW_MASK_PER | PSW_MASK_IO | PSW_MASK_EXT);
    __local_ctl_load(9, 11, ctl.as_ptr());
    (*regs).psw.mask |= PSW_MASK_PER; (*regs).psw.mask &= !(PSW_MASK_IO | PSW_MASK_EXT); (*regs).psw.addr = ip;
}
unsafe fn disable_singlestep(kcb: *mut kprobe_ctlblk, regs: *mut pt_regs, ip: usize) {
    __local_ctl_load(9, 11, (*kcb).kprobe_saved_ctl.as_ptr());
    (*regs).psw.mask &= !PSW_MASK_PER; (*regs).psw.mask |= (*kcb).kprobe_saved_imask; (*regs).psw.addr = ip;
}
unsafe fn push_kprobe(kcb: *mut kprobe_ctlblk, p: *mut kprobe) { (*kcb).prev_kprobe.kp = __this_cpu_read(current_kprobe); (*kcb).prev_kprobe.status = (*kcb).kprobe_status; __this_cpu_write(current_kprobe, p); }
unsafe fn pop_kprobe(kcb: *mut kprobe_ctlblk) { __this_cpu_write(current_kprobe, (*kcb).prev_kprobe.kp); (*kcb).kprobe_status = (*kcb).prev_kprobe.status; (*kcb).prev_kprobe.kp = core::ptr::null_mut(); }

unsafe fn kprobe_reenter_check(kcb: *mut kprobe_ctlblk, p: *mut kprobe) {
    match (*kcb).kprobe_status { KPROBE_HIT_SSDONE | KPROBE_HIT_ACTIVE => kprobes_inc_nmissed_count(p), KPROBE_HIT_SS | KPROBE_REENTER | _ => { pr_err("Failed to recover from reentered kprobes.\n"); dump_kprobe(p); BUG(); } }
}

unsafe fn kprobe_handler(regs: *mut pt_regs) -> i32 {
    preempt_disable(); let kcb = get_kprobe_ctlblk(); let p = get_kprobe(((*regs).psw.addr - 2) as *mut _);
    if !p.is_null() { if kprobe_running() { kprobe_reenter_check(kcb, p); push_kprobe(kcb, p); (*kcb).kprobe_status = KPROBE_REENTER; } else { push_kprobe(kcb, p); (*kcb).kprobe_status = KPROBE_HIT_ACTIVE; if let Some(h) = (*p).pre_handler { if h(p, regs) != 0 { pop_kprobe(kcb); preempt_enable_no_resched(); return 1; } } (*kcb).kprobe_status = KPROBE_HIT_SS; } enable_singlestep(kcb, regs, (*p).ainsn.insn as usize); return 1; }
    preempt_enable_no_resched(); 0
}

unsafe fn resume_execution(p: *mut kprobe, regs: *mut pt_regs) { let kcb = get_kprobe_ctlblk(); let mut ip = (*regs).psw.addr; let fixup = probe_get_fixup_type((*p).ainsn.insn); if fixup & FIXUP_PSW_NORMAL != 0 { ip += (*p).addr as usize - (*p).ainsn.insn as usize; } if fixup & FIXUP_BRANCH_NOT_TAKEN != 0 { let ilen = insn_length((*p).ainsn.insn[0] >> 8) as usize; if ip - (*p).ainsn.insn as usize == ilen { ip = (*p).addr as usize + ilen; } } if fixup & FIXUP_RETURN_REGISTER != 0 { let reg = (((*p).ainsn.insn[0] & 0xf0) >> 4) as usize; (*regs).gprs[reg] += (*p).addr as usize - (*p).ainsn.insn as usize; } disable_singlestep(kcb, regs, ip); }

unsafe fn post_kprobe_handler(regs: *mut pt_regs) -> i32 { let kcb = get_kprobe_ctlblk(); let p = kprobe_running(); if p.is_null() { return 0; } resume_execution(p, regs); if (*kcb).kprobe_status != KPROBE_REENTER { if let Some(h) = (*p).post_handler { (*kcb).kprobe_status = KPROBE_HIT_SSDONE; h(p, regs, 0); } } pop_kprobe(kcb); preempt_enable_no_resched(); if (*regs).psw.mask & PSW_MASK_PER != 0 { 0 } else { 1 } }

unsafe fn kprobe_trap_handler(regs: *mut pt_regs, _trapnr: i32) -> i32 { let kcb = get_kprobe_ctlblk(); let p = kprobe_running(); match (*kcb).kprobe_status { KPROBE_HIT_SS | KPROBE_REENTER => { disable_singlestep(kcb, regs, (*p).addr as usize); pop_kprobe(kcb); preempt_enable_no_resched(); }, KPROBE_HIT_ACTIVE | KPROBE_HIT_SSDONE => { if fixup_exception(regs) { return 1; } }, _ => {} } 0 }
pub unsafe fn kprobe_fault_handler(regs: *mut pt_regs, trapnr: i32) -> i32 { if (*regs).psw.mask & (PSW_MASK_IO | PSW_MASK_EXT) != 0 { local_irq_disable(); } let ret = kprobe_trap_handler(regs, trapnr); if (*regs).psw.mask & (PSW_MASK_IO | PSW_MASK_EXT) != 0 { local_irq_restore((*regs).psw.mask & !PSW_MASK_PER); } ret }

pub unsafe fn kprobe_exceptions_notify(_self: *mut notifier_block, val: usize, data: *mut core::ffi::c_void) -> i32 { let args = &*(data as *const die_args); let regs = args.regs; let mut ret = NOTIFY_DONE; if (*regs).psw.mask & (PSW_MASK_IO | PSW_MASK_EXT) != 0 { local_irq_disable(); } match val { DIE_BPT => if kprobe_handler(regs) != 0 { ret = NOTIFY_STOP; }, DIE_SSTEP => if post_kprobe_handler(regs) != 0 { ret = NOTIFY_STOP; }, DIE_TRAP => if !preemptible() && !kprobe_running().is_null() && kprobe_trap_handler(regs, args.trapnr) != 0 { ret = NOTIFY_STOP; }, _ => {} } if (*regs).psw.mask & (PSW_MASK_IO | PSW_MASK_EXT) != 0 { local_irq_restore((*regs).psw.mask & !PSW_MASK_PER); } ret }

pub unsafe fn arch_init_kprobes() -> i32 { 0 }
pub unsafe fn arch_populate_kprobe_blacklist() -> i32 { kprobe_add_area_blacklist(__irqentry_text_start as usize, __irqentry_text_end as usize) }
pub unsafe fn arch_trampoline_kprobe(_p: *mut kprobe) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
