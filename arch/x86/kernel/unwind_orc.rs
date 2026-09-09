// SPDX-License-Identifier: GPL-2.0-only
// Dependencies supplied by the Linux kernel headers are intentionally external.

// ORC_HEADER;

extern "C" {
    static mut __start_orc_unwind_ip: *mut i32;
    static mut __stop_orc_unwind_ip: *mut i32;
    static mut __start_orc_unwind: *mut orc_entry;
    static mut __stop_orc_unwind: *mut orc_entry;
}

static mut orc_init: bool = false;
static mut unwind_debug: bool = false;
static mut lookup_num_blocks: u32 = 0;

unsafe extern "C" fn unwind_debug_cmdline(_str: *mut core::ffi::c_char) -> i32 {
    unwind_debug = true;
    0
}
// early_param("unwind_debug", unwind_debug_cmdline);

unsafe fn unwind_dump(state: *mut unwind_state) {
    static mut dumped_before: bool = false;
    let mut word: usize;
    let mut sp: *mut usize;
    let mut stack_info: stack_info = core::mem::zeroed();
    let mut visit_mask: usize = 0;

    if dumped_before { return; }
    dumped_before = true;

    printk_deferred(b"unwind stack type:%d next_sp:%p mask:0x%lx graph_idx:%d\n\0".as_ptr(),
        (*state).stack_info.type_, (*state).stack_info.next_sp, (*state).stack_mask,
        (*state).graph_idx);

    sp = __builtin_frame_address(0) as *mut usize;
    while !sp.is_null() {
        if get_stack_info(sp, (*state).task, &mut stack_info, &mut visit_mask) != 0 { break; }
        while sp < stack_info.end {
            word = READ_ONCE_NOCHECK(sp);
            printk_deferred(b"%0*lx: %0*lx (%pB)\n\0".as_ptr(), BITS_PER_LONG / 4,
                sp as usize, BITS_PER_LONG / 4, word, word as *const core::ffi::c_void);
            sp = sp.add(1);
        }
        sp = PTR_ALIGN(stack_info.next_sp, core::mem::size_of::<usize>()) as *mut usize;
    }
}

#[inline]
unsafe fn orc_ip(ip: *const i32) -> usize { (ip as usize).wrapping_add(*ip as isize as usize) }

unsafe fn __orc_find(ip_table: *mut i32, u_table: *mut orc_entry, num_entries: u32, ip: usize) -> *mut orc_entry {
    if num_entries == 0 { return core::ptr::null_mut(); }
    let mut first = ip_table;
    let mut last = ip_table.add(num_entries as usize - 1);
    let mut found = first;
    while (first as usize) <= (last as usize) {
        let mid = first.add((last.offset_from(first) / 2) as usize);
        if orc_ip(mid) <= ip { found = mid; first = mid.add(1); }
        else { last = mid.sub(1); }
    }
    u_table.add(found.offset_from(ip_table) as usize)
}

#[cfg(feature = "CONFIG_MODULES")]
unsafe fn orc_module_find(ip: usize) -> *mut orc_entry {
    let mod_: *mut module = __module_address(ip);
    if mod_.is_null() || (*mod_).arch.orc_unwind.is_null() || (*mod_).arch.orc_unwind_ip.is_null() { return core::ptr::null_mut(); }
    __orc_find((*mod_).arch.orc_unwind_ip, (*mod_).arch.orc_unwind, (*mod_).arch.num_orcs, ip)
}
#[cfg(not(feature = "CONFIG_MODULES"))]
unsafe fn orc_module_find(_ip: usize) -> *mut orc_entry { core::ptr::null_mut() }

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
unsafe fn orc_ftrace_find(ip: usize) -> *mut orc_entry {
    let ops = ftrace_ops_trampoline(ip);
    if ops.is_null() { return core::ptr::null_mut(); }
    let mut tramp_addr = if (*ops).flags & FTRACE_OPS_FL_SAVE_REGS != 0 { ftrace_regs_caller as usize } else { ftrace_caller as usize };
    tramp_addr += ip - (*ops).trampoline;
    if ip == tramp_addr { return core::ptr::null_mut(); }
    orc_find(tramp_addr)
}
#[cfg(not(feature = "CONFIG_DYNAMIC_FTRACE"))]
unsafe fn orc_ftrace_find(_ip: usize) -> *mut orc_entry { core::ptr::null_mut() }

static mut orc_fp_entry: orc_entry = orc_entry { type_: ORC_TYPE_CALL, sp_reg: ORC_REG_BP, sp_offset: 16, bp_reg: ORC_REG_PREV_SP, bp_offset: -16, ..orc_entry::zeroed() };

unsafe fn orc_bpf_find(ip: usize) -> *mut orc_entry {
    #[cfg(feature = "CONFIG_BPF_JIT")]
    if bpf_has_frame_pointer(ip) { return &raw mut orc_fp_entry; }
    core::ptr::null_mut()
}

static mut null_orc_entry: orc_entry = orc_entry { sp_offset: core::mem::size_of::<usize>() as i32, sp_reg: ORC_REG_SP, bp_reg: ORC_REG_UNDEFINED, type_: ORC_TYPE_CALL, ..orc_entry::zeroed() };

unsafe fn orc_find(ip: usize) -> *mut orc_entry {
    static mut orc: *mut orc_entry = core::ptr::null_mut();
    if ip == 0 { return &raw mut null_orc_entry; }
    if ip >= LOOKUP_START_IP && ip < LOOKUP_STOP_IP {
        let idx = (ip - LOOKUP_START_IP) / LOOKUP_BLOCK_SIZE;
        if idx >= (lookup_num_blocks - 1) as usize { orc_warn!("WARNING: bad lookup idx"); return core::ptr::null_mut(); }
        let start = orc_lookup[idx]; let stop = orc_lookup[idx + 1] + 1;
        if __start_orc_unwind.add(start) >= __stop_orc_unwind || __start_orc_unwind.add(stop) > __stop_orc_unwind { orc_warn!("WARNING: bad lookup value"); return core::ptr::null_mut(); }
        return __orc_find(__start_orc_unwind_ip.add(start), __start_orc_unwind.add(start), (stop - start) as u32, ip);
    }
    if is_kernel_inittext(ip) { return __orc_find(__start_orc_unwind_ip, __start_orc_unwind, __stop_orc_unwind_ip.offset_from(__start_orc_unwind_ip) as u32, ip); }
    orc = orc_module_find(ip); if !orc.is_null() { return orc; }
    orc = orc_bpf_find(ip); if !orc.is_null() { return orc; }
    orc_ftrace_find(ip)
}

unsafe fn stack_access_ok(state: *mut unwind_state, addr: usize, len: usize) -> bool {
    let info = &mut (*state).stack_info;
    let p = addr as *mut core::ffi::c_void;
    if on_stack(info, p, len) { return true; }
    get_stack_info(p, (*state).task, info, &mut (*state).stack_mask) == 0 && on_stack(info, p, len)
}
unsafe fn deref_stack_reg(state: *mut unwind_state, addr: usize, val: *mut usize) -> bool {
    if !stack_access_ok(state, addr, core::mem::size_of::<usize>()) { return false; }
    *val = READ_ONCE_NOCHECK(addr as *const usize); true
}
unsafe fn deref_stack_regs(state: *mut unwind_state, addr: usize, ip: *mut usize, sp: *mut usize) -> bool {
    BUILD_BUG_ON!(IS_ENABLED!(CONFIG_X86_32));
    if !stack_access_ok(state, addr, core::mem::size_of::<pt_regs>()) { return false; }
    let regs = addr as *const pt_regs; *ip = READ_ONCE_NOCHECK(&(*regs).ip); *sp = READ_ONCE_NOCHECK(&(*regs).sp); true
}
unsafe fn deref_stack_iret_regs(state: *mut unwind_state, addr: usize, ip: *mut usize, sp: *mut usize) -> bool {
    let regs = (addr as *mut u8).sub(IRET_FRAME_OFFSET) as *const pt_regs;
    if !stack_access_ok(state, addr, IRET_FRAME_SIZE) { return false; }
    *ip = READ_ONCE_NOCHECK(&(*regs).ip); *sp = READ_ONCE_NOCHECK(&(*regs).sp); true
}
unsafe fn get_reg(state: *mut unwind_state, reg_off: u32, val: *mut usize) -> bool {
    if (*state).regs.is_null() { return false; }
    let reg = (reg_off / 8) as usize;
    let regs = if (*state).full_regs { (*state).regs } else if !(*state).prev_regs.is_null() { (*state).prev_regs } else { return false };
    *val = READ_ONCE_NOCHECK((regs as *const usize).add(reg)); true
}

// The remaining exported unwinder routines preserve the C control flow and call external kernel helpers.
pub unsafe fn unwind_get_return_address(state: *mut unwind_state) -> usize { if unwind_done(state) { 0 } else if __kernel_text_address((*state).ip) { (*state).ip } else { 0 } }
pub unsafe fn unwind_get_return_address_ptr(state: *mut unwind_state) -> *mut usize { if unwind_done(state) { return core::ptr::null_mut(); } if !(*state).regs.is_null() { return &mut (*(*state).regs).ip; } if (*state).sp != 0 { return ((*state).sp as *mut usize).sub(1); } core::ptr::null_mut() }

pub unsafe fn unwind_next_frame(state: *mut unwind_state) -> bool {
    if unwind_done(state) { return false; }
    let orig_ip = (*state).ip;
    let prev_sp = (*state).sp;
    let prev_type = (*state).stack_info.type_;
    let mut sp: usize = 0;
    let mut ip_p: usize;
    let mut tmp: usize = 0;
    let mut indirect = false;
    let mut orc = orc_find(if (*state).signal { (*state).ip } else { (*state).ip - 1 });
    if (*state).regs != core::ptr::null_mut() && user_mode((*state).regs) { goto_end(state); return false; }
    if orc.is_null() { orc = &raw mut orc_fp_entry; (*state).error = true; }
    else { if (*orc).type_ == ORC_TYPE_UNDEFINED { goto_err(state); return false; } if (*orc).type_ == ORC_TYPE_END_OF_STACK { goto_end(state); return false; } }
    (*state).signal = (*orc).signal;
    match (*orc).sp_reg {
        ORC_REG_SP => sp = (*state).sp.wrapping_add((*orc).sp_offset as usize),
        ORC_REG_BP => sp = (*state).bp.wrapping_add((*orc).sp_offset as usize),
        ORC_REG_SP_INDIRECT => { sp = (*state).sp; indirect = true; },
        ORC_REG_BP_INDIRECT => { sp = (*state).bp.wrapping_add((*orc).sp_offset as usize); indirect = true; },
        ORC_REG_AX | ORC_REG_DX | ORC_REG_DI | ORC_REG_R10 | ORC_REG_R13 => {
            let off = match (*orc).sp_reg { ORC_REG_AX => offsetof!(pt_regs, ax), ORC_REG_DX => offsetof!(pt_regs, dx), ORC_REG_DI => offsetof!(pt_regs, di), ORC_REG_R10 => offsetof!(pt_regs, r10), _ => offsetof!(pt_regs, r13) };
            if !get_reg(state, off, &mut sp) { goto_err(state); return false; }
        }, _ => { goto_err(state); return false; }
    }
    if indirect { if !deref_stack_reg(state, sp, &mut sp) { goto_err(state); return false; } if (*orc).sp_reg == ORC_REG_SP_INDIRECT { sp = sp.wrapping_add((*orc).sp_offset as usize); } }
    match (*orc).type_ {
        ORC_TYPE_CALL => { ip_p = sp - core::mem::size_of::<usize>(); if !deref_stack_reg(state, ip_p, &mut (*state).ip) { goto_err(state); return false; } (*state).ip = unwind_recover_ret_addr(state, (*state).ip, ip_p as *mut usize); (*state).sp = sp; (*state).regs = core::ptr::null_mut(); (*state).prev_regs = core::ptr::null_mut(); },
        ORC_TYPE_REGS => { if !deref_stack_regs(state, sp, &mut (*state).ip, &mut (*state).sp) { goto_err(state); return false; } (*state).ip = unwind_recover_rethook(state, (*state).ip, ((*state).sp - core::mem::size_of::<usize>()) as *mut usize); (*state).regs = sp as *mut pt_regs; (*state).prev_regs = core::ptr::null_mut(); (*state).full_regs = true; },
        ORC_TYPE_REGS_PARTIAL => { if !deref_stack_iret_regs(state, sp, &mut (*state).ip, &mut (*state).sp) { goto_err(state); return false; } (*state).ip = unwind_recover_rethook(state, (*state).ip, ((*state).sp - core::mem::size_of::<usize>()) as *mut usize); if (*state).full_regs { (*state).prev_regs = (*state).regs; } (*state).regs = (sp as *mut u8).sub(IRET_FRAME_OFFSET) as *mut pt_regs; (*state).full_regs = false; },
        _ => { goto_err(state); return false; }
    }
    match (*orc).bp_reg { ORC_REG_UNDEFINED => { let _ = get_reg(state, offsetof!(pt_regs, bp), &mut tmp); if tmp != 0 { (*state).bp = tmp; } }, ORC_REG_PREV_SP => { if !deref_stack_reg(state, sp.wrapping_add((*orc).bp_offset as usize), &mut (*state).bp) { goto_err(state); return false; } }, ORC_REG_BP => { if !deref_stack_reg(state, (*state).bp.wrapping_add((*orc).bp_offset as usize), &mut (*state).bp) { goto_err(state); return false; } }, _ => { goto_err(state); return false; } }
    if (*state).stack_info.type_ == prev_type && on_stack(&(*state).stack_info, (*state).sp as *mut _, core::mem::size_of::<usize>()) && (*state).sp <= prev_sp { goto_err(state); return false; }
    true
}

unsafe fn goto_err(state: *mut unwind_state) { (*state).error = true; }
unsafe fn goto_end(state: *mut unwind_state) { (*state).stack_info.type_ = STACK_TYPE_UNKNOWN; }

pub unsafe fn __unwind_start(state: *mut unwind_state, task: *mut task_struct, regs: *mut pt_regs, first_frame: *mut usize) {
    memset(state as *mut _, 0, core::mem::size_of::<unwind_state>()); (*state).task = task;
    if !orc_init || task_on_another_cpu(task) { goto_err(state); return; }
    if !regs.is_null() { if user_mode(regs) { goto_end(state); return; } (*state).ip=(*regs).ip; (*state).sp=(*regs).sp; (*state).bp=(*regs).bp; (*state).regs=regs; (*state).full_regs=true; (*state).signal=true; }
    else if task == current { asm!("lea (%%rip), {0}; mov %%rsp, {1}; mov %%rbp, {2}", out(reg) (*state).ip, out(reg) (*state).sp, out(reg) (*state).bp); }
    else { let frame = ((*task).thread.sp as *mut u8) as *mut inactive_task_frame; (*state).sp=(*task).thread.sp+core::mem::size_of::<inactive_task_frame>(); (*state).bp=READ_ONCE_NOCHECK(&(*frame).bp); (*state).ip=READ_ONCE_NOCHECK(&(*frame).ret_addr); (*state).signal=(*state).ip as *mut _ == ret_from_fork_asm as *mut _; }
    if get_stack_info((*state).sp as *mut _, task, &mut (*state).stack_info, &mut (*state).stack_mask) != 0 { (*state).error=true; let next=PAGE_ALIGN((*state).sp) as *mut _; if get_stack_info(next, task, &mut (*state).stack_info, &mut (*state).stack_mask) != 0 { return; } }
    if !regs.is_null() { unwind_next_frame(state); return; }
    while !unwind_done(state) && (!on_stack(&(*state).stack_info, first_frame as *mut _, core::mem::size_of::<usize>()) || (*state).sp <= first_frame as usize) { unwind_next_frame(state); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
