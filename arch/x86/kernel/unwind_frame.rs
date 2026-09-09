// SPDX-License-Identifier: GPL-2.0-only

const FRAME_HEADER_SIZE: usize = core::mem::size_of::<c_ulong>() * 2;

pub unsafe fn unwind_get_return_address(state: *mut unwind_state) -> c_ulong {
    if unwind_done(state) { return 0; }
    if __kernel_text_address((*state).ip) { (*state).ip } else { 0 }
}

pub unsafe fn unwind_get_return_address_ptr(state: *mut unwind_state) -> *mut c_ulong {
    if unwind_done(state) { return core::ptr::null_mut(); }
    if !(*state).regs.is_null() { &mut (*(*state).regs).ip } else { (*state).bp.add(1) }
}

unsafe fn unwind_dump(state: *mut unwind_state) {
    static mut DUMPED_BEFORE: bool = false;
    let mut prev_zero: bool;
    let mut zero = false;
    let mut word: c_ulong;
    let mut sp: *mut c_ulong;
    let mut stack_info: stack_info = core::mem::zeroed();
    let mut visit_mask: c_ulong = 0;
    if DUMPED_BEFORE { return; }
    DUMPED_BEFORE = true;
    printk_deferred(c_str!("unwind stack type:%d next_sp:%p mask:0x%lx graph_idx:%d\n"), (*state).stack_info.type_, (*state).stack_info.next_sp, (*state).stack_mask, (*state).graph_idx);
    sp = ptr_align((*state).orig_sp, core::mem::size_of::<c_ulong>());
    while !sp.is_null() {
        if get_stack_info(sp, (*state).task, &mut stack_info, &mut visit_mask) != 0 { break; }
        while sp < stack_info.end {
            word = read_once_nocheck(sp);
            prev_zero = zero; zero = word == 0;
            if zero { if !prev_zero { printk_deferred(c_str!("%p: %0*x ...\n"), sp, BITS_PER_LONG / 4, 0); } sp = sp.add(1); continue; }
            printk_deferred(c_str!("%p: %0*lx (%pB)\n"), sp, BITS_PER_LONG / 4, word, word as *mut core::ffi::c_void);
            sp = sp.add(1);
        }
        sp = ptr_align(stack_info.next_sp, core::mem::size_of::<c_ulong>());
    }
}

unsafe fn in_entry_code(ip: c_ulong) -> bool { let addr = ip as *const i8; addr >= __entry_text_start && addr < __entry_text_end }
unsafe fn last_frame(state: *mut unwind_state) -> *mut c_ulong { task_pt_regs((*state).task).cast::<c_ulong>().sub(2) }
unsafe fn is_last_frame(state: *mut unwind_state) -> bool { (*state).bp == last_frame(state) }

#[cfg(CONFIG_X86_32)] const GCC_REALIGN_WORDS: usize = 3;
#[cfg(not(CONFIG_X86_32))] const GCC_REALIGN_WORDS: usize = 1;
unsafe fn last_aligned_frame(state: *mut unwind_state) -> *mut c_ulong { last_frame(state).sub(GCC_REALIGN_WORDS) }
unsafe fn is_last_aligned_frame(state: *mut unwind_state) -> bool { let last_bp = last_frame(state); let aligned_bp = last_aligned_frame(state); (*state).bp == aligned_bp && *aligned_bp.add(1) == *last_bp.add(1) }
unsafe fn is_last_ftrace_frame(state: *mut unwind_state) -> bool { let last_bp = last_frame(state); let last_ftrace_bp = last_bp.sub(3); (*state).bp == last_ftrace_bp && *(*state).bp == *(*state).bp.add(2) && *(*state).bp.add(1) == *(*state).bp.add(4) }
unsafe fn is_last_task_frame(state: *mut unwind_state) -> bool { is_last_frame(state) || is_last_aligned_frame(state) || is_last_ftrace_frame(state) }

#[cfg(CONFIG_X86_64)] unsafe fn decode_frame_pointer(bp: *mut c_ulong) -> *mut pt_regs { let regs = bp as c_ulong; if regs & 1 == 0 { core::ptr::null_mut() } else { (regs & !1) as *mut pt_regs } }
#[cfg(not(CONFIG_X86_64))] unsafe fn decode_frame_pointer(bp: *mut c_ulong) -> *mut pt_regs { let regs = bp as c_ulong; if regs & 0x80000000 != 0 { core::ptr::null_mut() } else { (regs | 0x80000000) as *mut pt_regs } }

unsafe fn update_stack_state(state: *mut unwind_state, next_bp: *mut c_ulong) -> bool {
    let info = &mut (*state).stack_info; let prev_type = info.type_; let prev_frame_end: *mut u8;
    if !(*state).regs.is_null() { prev_frame_end = (*state).regs.cast::<u8>().add(core::mem::size_of::<pt_regs>()); } else { prev_frame_end = (*state).bp.cast::<u8>().add(FRAME_HEADER_SIZE); }
    let regs = decode_frame_pointer(next_bp); let (frame, len) = if !regs.is_null() { (*state).got_irq = true; (regs.cast::<c_ulong>(), core::mem::size_of::<pt_regs>()) } else { (next_bp, FRAME_HEADER_SIZE) };
    while !on_stack(info, frame, len) { if get_stack_info(info.next_sp, (*state).task, info, &mut (*state).stack_mask) != 0 { return false; } }
    if !(*state).orig_sp.is_null() && info.type_ == prev_type && frame.cast::<u8>() < prev_frame_end { return false; }
    if !regs.is_null() { (*state).regs = regs; (*state).bp = core::ptr::null_mut(); } else { (*state).bp = next_bp; (*state).regs = core::ptr::null_mut(); }
    if !(*state).regs.is_null() && user_mode((*state).regs) { (*state).ip = 0; } else { let addr_p = unwind_get_return_address_ptr(state); let addr = read_once_task_stack((*state).task, addr_p); (*state).ip = unwind_recover_ret_addr(state, addr, addr_p); }
    if (*state).orig_sp.is_null() { (*state).orig_sp = frame; } true
}

pub unsafe fn unwind_next_frame(state: *mut unwind_state) -> bool {
    if unwind_done(state) { return false; }
    if !(*state).regs.is_null() && user_mode((*state).regs) { goto_end!(state); }
    if is_last_task_frame(state) { let regs = task_pt_regs((*state).task); if !user_mode(regs) { goto_end!(state); } (*state).regs = regs; (*state).bp = core::ptr::null_mut(); (*state).ip = 0; return true; }
    let next_bp = if !(*state).next_bp.is_null() { let p=(*state).next_bp; (*state).next_bp=core::ptr::null_mut(); p } else if !(*state).regs.is_null() { (*state).regs.cast::<u8>().cast::<pt_regs>().as_ref().unwrap().bp as *mut c_ulong } else { read_once_task_stack((*state).task, (*state).bp) as *mut c_ulong };
    if !update_stack_state(state, next_bp) { (*state).error = true; if (*state).task != current { goto_end!(state); } if (*state).got_irq && in_entry_code((*state).ip) { goto_end!(state); } if !(*state).regs.is_null() && (*state).regs.as_ref().unwrap().sp >= last_aligned_frame(state) as c_ulong && (*state).regs.as_ref().unwrap().sp < task_pt_regs((*state).task) as c_ulong { goto_end!(state); } if is_enabled_x86_32() { goto_end!(state); } if !(*state).regs.is_null() { printk_deferred_once(c_str!("WARNING: kernel stack regs at %p has bad 'bp' value %p\n"), (*state).regs, next_bp); } else { printk_deferred_once(c_str!("WARNING: kernel stack frame pointer at %p has bad value %p\n"), (*state).bp, next_bp); } unwind_dump(state); }
    (*state).stack_info.type_ = STACK_TYPE_UNKNOWN; false
}

pub unsafe fn __unwind_start(state: *mut unwind_state, task: *mut task_struct, regs: *mut pt_regs, first_frame: *mut c_ulong) {
    core::ptr::write_bytes(state, 0, 1); (*state).task=task; (*state).got_irq=!regs.is_null(); if !regs.is_null() && user_mode(regs) { (*state).stack_info.type_=STACK_TYPE_UNKNOWN; return; }
    let mut bp=get_frame_pointer(task, regs); if !regs.is_null() && (*regs).ip==0 && (*regs).sp as *mut c_ulong >= first_frame { (*state).next_bp=bp; bp=((*regs).sp as *mut c_ulong).sub(1); }
    get_stack_info(bp, task, &mut (*state).stack_info, &mut (*state).stack_mask); update_stack_state(state,bp);
    while !unwind_done(state) && (!on_stack(&(*state).stack_info, first_frame.cast(), core::mem::size_of::<c_ulong>()) || ((*state).next_bp.is_null() && (*state).bp < first_frame)) { unwind_next_frame(state); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
