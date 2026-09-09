// SPDX-License-Identifier: GPL-2.0
/* Ptrace user space interface. Translated from s390/kernel/ptrace.c. */

const __ADDR_MASK: usize = 7;

pub unsafe fn update_cr_regs(task: *mut task_struct) {
    let regs = task_pt_regs(task);
    let thread = &mut (*task).thread;
    let mut cr0_old: ctlreg0 = core::mem::zeroed();
    let mut cr2_old: ctlreg2 = core::mem::zeroed();
    let mut cr0_new: ctlreg0;
    let mut cr2_new: ctlreg2;
    let mut old: per_regs = core::mem::zeroed();
    let mut new: per_regs = core::mem::zeroed();
    local_ctl_store(0, &mut cr0_old.reg);
    local_ctl_store(2, &mut cr2_old.reg);
    cr0_new = cr0_old;
    cr2_new = cr2_old;
    if machine_has_tx() {
        cr0_new.tcx = 1;
        if thread.per_flags & PER_FLAG_NO_TE != 0 { cr0_new.tcx = 0; }
        cr2_new.tdc = 0;
        if thread.per_flags & PER_FLAG_TE_ABORT_RAND != 0 {
            cr2_new.tdc = if thread.per_flags & PER_FLAG_TE_ABORT_RAND_TEND != 0 { 1 } else { 2 };
        }
    }
    if cpu_has_gs() {
        cr2_new.gse = 0;
        if !thread.gs_cb.is_null() { cr2_new.gse = 1; }
    }
    if cr0_new.val != cr0_old.val { local_ctl_load(0, &cr0_new.reg); }
    if cr2_new.val != cr2_old.val { local_ctl_load(2, &cr2_new.reg); }
    new.control.val = thread.per_user.control;
    new.start.val = thread.per_user.start;
    new.end.val = thread.per_user.end;
    if test_tsk_thread_flag(task, TIF_SINGLE_STEP) || test_tsk_thread_flag(task, TIF_UPROBE_SINGLESTEP) {
        if test_tsk_thread_flag(task, TIF_BLOCK_STEP) { new.control.val |= PER_EVENT_BRANCH; } else { new.control.val |= PER_EVENT_IFETCH; }
        new.control.val |= PER_CONTROL_SUSPENSION | PER_EVENT_TRANSACTION_END;
        if test_tsk_thread_flag(task, TIF_UPROBE_SINGLESTEP) { new.control.val |= PER_EVENT_IFETCH; }
        new.start.val = 0; new.end.val = usize::MAX as _;
    }
    if new.control.val & PER_EVENT_MASK == 0 { (*regs).psw.mask &= !PSW_MASK_PER; return; }
    (*regs).psw.mask |= PSW_MASK_PER;
    __local_ctl_store(9, 11, old.regs.as_mut_ptr());
    if core::slice::from_raw_parts((&new as *const _ as *const u8), core::mem::size_of::<per_regs>()) != core::slice::from_raw_parts((&old as *const _ as *const u8), core::mem::size_of::<per_regs>()) { __local_ctl_load(9, 11, new.regs.as_ptr()); }
}

pub unsafe fn user_enable_single_step(task: *mut task_struct) { clear_tsk_thread_flag(task, TIF_BLOCK_STEP); set_tsk_thread_flag(task, TIF_SINGLE_STEP); }
pub unsafe fn user_disable_single_step(task: *mut task_struct) { clear_tsk_thread_flag(task, TIF_BLOCK_STEP); clear_tsk_thread_flag(task, TIF_SINGLE_STEP); }
pub unsafe fn user_enable_block_step(task: *mut task_struct) { set_tsk_thread_flag(task, TIF_SINGLE_STEP); set_tsk_thread_flag(task, TIF_BLOCK_STEP); }

pub unsafe fn ptrace_disable(task: *mut task_struct) {
    core::ptr::write_bytes(&mut (*task).thread.per_user as *mut _, 0, 1);
    core::ptr::write_bytes(&mut (*task).thread.per_event as *mut _, 0, 1);
    clear_tsk_thread_flag(task, TIF_SINGLE_STEP); clear_tsk_thread_flag(task, TIF_PER_TRAP);
    (*task).thread.per_flags = 0;
}

unsafe fn __peek_user_per(child: *mut task_struct, addr: addr_t) -> addr_t {
    if addr == offset_of!(per_struct_kernel, cr9) { if test_thread_flag(TIF_SINGLE_STEP) { PER_EVENT_IFETCH } else { (*child).thread.per_user.control } }
    else if addr == offset_of!(per_struct_kernel, cr10) { if test_thread_flag(TIF_SINGLE_STEP) { 0 } else { (*child).thread.per_user.start } }
    else if addr == offset_of!(per_struct_kernel, cr11) { if test_thread_flag(TIF_SINGLE_STEP) { addr_t::MAX } else { (*child).thread.per_user.end } }
    else if addr == offset_of!(per_struct_kernel, bits) { if test_thread_flag(TIF_SINGLE_STEP) { 1usize << (BITS_PER_LONG - 1) } else { 0 } }
    else if addr == offset_of!(per_struct_kernel, starting_addr) { (*child).thread.per_user.start }
    else if addr == offset_of!(per_struct_kernel, ending_addr) { (*child).thread.per_user.end }
    else if addr == offset_of!(per_struct_kernel, perc_atmid) { ((*child).thread.per_event.cause as addr_t) << (BITS_PER_LONG - 16) }
    else if addr == offset_of!(per_struct_kernel, address) { (*child).thread.per_event.address }
    else if addr == offset_of!(per_struct_kernel, access_id) { ((*child).thread.per_event.paid as addr_t) << (BITS_PER_LONG - 8) }
    else { 0 }
}

unsafe fn __peek_user(child: *mut task_struct, addr: addr_t) -> addr_t {
    let mut tmp;
    if addr < offset_of!(user, regs.acrs) { tmp = *(core::ptr::addr_of!((*task_pt_regs(child)).psw) as *const u8).add(addr) as addr_t; if addr == offset_of!(user, regs.psw.mask) { tmp &= PSW_MASK_USER | PSW_MASK_RI; tmp |= PSW_USER_BITS; } }
    else if addr < offset_of!(user, regs.orig_gpr2) { let offset = addr - offset_of!(user, regs.acrs); tmp = if addr == offset_of!(user, regs.acrs[15]) { ((*child).thread.acrs[15] as addr_t) << 32 } else { *(core::ptr::addr_of!((*child).thread.acrs) as *const u8).add(offset) as addr_t }; }
    else if addr == offset_of!(user, regs.orig_gpr2) { tmp = (*task_pt_regs(child)).orig_gpr2 as addr_t; }
    else if addr < offset_of!(user, regs.fp_regs) { tmp = 0; }
    else if addr == offset_of!(user, regs.fp_regs.fpc) { tmp = ((*child).thread.ufpu.fpc as addr_t) << (BITS_PER_LONG - 32); }
    else if addr < offset_of!(user, regs.fp_regs) + core::mem::size_of::<s390_fp_regs>() { let offset = addr - offset_of!(user, regs.fp_regs.fprs); tmp = *(core::ptr::addr_of!((*child).thread.ufpu.vxrs) as *const u8).add(2 * offset) as addr_t; }
    else if addr < offset_of!(user, regs.per_info) + core::mem::size_of::<per_struct>() { tmp = __peek_user_per(child, addr - offset_of!(user, regs.per_info)); }
    else { tmp = 0; }
    tmp
}

unsafe fn peek_user(child: *mut task_struct, addr: addr_t, data: addr_t) -> i32 { let mask = if addr >= offset_of!(user, regs.acrs) && addr < offset_of!(user, regs.orig_gpr2) { 3 } else { __ADDR_MASK }; if addr & mask != 0 || addr > core::mem::size_of::<user>() - __ADDR_MASK { return -EIO; } put_user(__peek_user(child, addr), data as *mut addr_t) }

unsafe fn __poke_user_per(child: *mut task_struct, addr: addr_t, data: addr_t) { if addr == offset_of!(per_struct_kernel, cr9) { (*child).thread.per_user.control = data & (PER_EVENT_MASK | PER_CONTROL_MASK); } else if addr == offset_of!(per_struct_kernel, starting_addr) { (*child).thread.per_user.start = data; } else if addr == offset_of!(per_struct_kernel, ending_addr) { (*child).thread.per_user.end = data; } }

unsafe fn __poke_user(child: *mut task_struct, addr: addr_t, data: addr_t) -> i32 {
    if addr < offset_of!(user, regs.acrs) { let regs = task_pt_regs(child); if addr == offset_of!(user, regs.psw.mask) { let mut mask = PSW_MASK_USER; if is_ri_task(child) { mask |= PSW_MASK_RI; } if (data ^ PSW_USER_BITS) & !mask != 0 || data & PSW_MASK_ASC == PSW_ASC_HOME || data & PSW_MASK_EA != 0 && data & PSW_MASK_BA == 0 { return -EINVAL; } } if test_pt_regs_flag(regs, PIF_SYSCALL) && addr == offset_of!(user, regs.gprs[2]) { (*regs).int_code = 0x20000 | (data & 0xffff); } *(core::ptr::addr_of_mut!((*regs).psw) as *mut u8).add(addr) = data as u8; }
    else if addr < offset_of!(user, regs.orig_gpr2) { let offset = addr - offset_of!(user, regs.acrs); if addr == offset_of!(user, regs.acrs[15]) { (*child).thread.acrs[15] = (data >> 32) as u32; } else { *(core::ptr::addr_of_mut!((*child).thread.acrs) as *mut u8).add(offset) = data as u8; } }
    else if addr == offset_of!(user, regs.orig_gpr2) { (*task_pt_regs(child)).orig_gpr2 = data; }
    else if addr < offset_of!(user, regs.fp_regs) { return 0; }
    else if addr == offset_of!(user, regs.fp_regs.fpc) { if data as u32 != 0 { return -EINVAL; } (*child).thread.ufpu.fpc = (data >> (BITS_PER_LONG - 32)) as _; }
    else if addr < offset_of!(user, regs.fp_regs) + core::mem::size_of::<s390_fp_regs>() { let offset = addr - offset_of!(user, regs.fp_regs.fprs); *(core::ptr::addr_of_mut!((*child).thread.ufpu.vxrs) as *mut u8).add(2 * offset) = data as u8; }
    else if addr < offset_of!(user, regs.per_info) + core::mem::size_of::<per_struct>() { __poke_user_per(child, addr - offset_of!(user, regs.per_info), data); }
    0
}

unsafe fn poke_user(child: *mut task_struct, addr: addr_t, data: addr_t) -> i32 { let mask = if addr >= offset_of!(user, regs.acrs) && addr < offset_of!(user, regs.orig_gpr2) { 3 } else { __ADDR_MASK }; if addr & mask != 0 || addr > core::mem::size_of::<user>() - __ADDR_MASK { return -EIO; } __poke_user(child, addr, data) }

pub unsafe fn arch_ptrace(child: *mut task_struct, request: c_long, mut addr: c_ulong, mut data: c_ulong) -> c_long {
    match request { PTRACE_PEEKUSR => peek_user(child, addr as _, data as _) as _, PTRACE_POKEUSR => poke_user(child, addr as _, data as _) as _, PTRACE_PEEKUSR_AREA | PTRACE_POKEUSR_AREA => { let mut parea: ptrace_area = core::mem::zeroed(); if copy_from_user(&mut parea, addr as *const _, core::mem::size_of::<ptrace_area>()) != 0 { return -EFAULT as _; } addr = parea.kernel_addr; data = parea.process_addr; let mut copied = 0; while copied < parea.len { let ret = if request == PTRACE_PEEKUSR_AREA { peek_user(child, addr as _, data as _) } else { let mut utmp = 0; if get_user(&mut utmp, data as *const _) != 0 { return -EFAULT as _; } poke_user(child, addr as _, utmp) }; if ret != 0 { return ret as _; } addr += core::mem::size_of::<c_ulong>() as _; data += core::mem::size_of::<c_ulong>() as _; copied += core::mem::size_of::<c_ulong>() as _; } 0 }, PTRACE_GET_LAST_BREAK => put_user((*child).thread.last_break, data as *mut c_ulong) as _, PTRACE_ENABLE_TE => { if !machine_has_tx() { -EIO as _ } else { (*child).thread.per_flags &= !PER_FLAG_NO_TE; 0 } }, PTRACE_DISABLE_TE => { if !machine_has_tx() { -EIO as _ } else { (*child).thread.per_flags |= PER_FLAG_NO_TE; (*child).thread.per_flags &= !PER_FLAG_TE_ABORT_RAND; 0 } }, PTRACE_TE_ABORT_RAND => { if !machine_has_tx() || (*child).thread.per_flags & PER_FLAG_NO_TE != 0 { return -EIO as _; } match data { 0 => (*child).thread.per_flags &= !PER_FLAG_TE_ABORT_RAND, 1 => { (*child).thread.per_flags |= PER_FLAG_TE_ABORT_RAND | PER_FLAG_TE_ABORT_RAND_TEND; }, 2 => { (*child).thread.per_flags |= PER_FLAG_TE_ABORT_RAND; (*child).thread.per_flags &= !PER_FLAG_TE_ABORT_RAND_TEND; }, _ => return -EINVAL as _ } 0 }, _ => ptrace_request(child, request, addr, data) }
}

unsafe fn is_ri_cb_valid(cb: *const runtime_instr_cb) -> bool { (*cb).rca & 0x1f == 0 && (*cb).roa & 0xfff == 0 && (*cb).rla & 0xfff == 0xfff && (*cb).s == 1 && (*cb).k == 1 && (*cb).h == 0 && (*cb).reserved1 == 0 && (*cb).ps == 1 && (*cb).qs == 0 && (*cb).pc == 1 && (*cb).qc == 0 && (*cb).reserved2 == 0 && (*cb).reserved3 == 0 && (*cb).reserved4 == 0 && (*cb).reserved5 == 0 && (*cb).reserved6 == 0 && (*cb).reserved7 == 0 && (*cb).reserved8 == 0 && (*cb).rla >= (*cb).roa && (*cb).rca >= (*cb).roa && (*cb).rca <= (*cb).rla + 1 && (*cb).m < 3 }

static GPR_NAMES: [&[u8]; NUM_GPRS] = [b"r0",b"r1",b"r2",b"r3",b"r4",b"r5",b"r6",b"r7",b"r8",b"r9",b"r10",b"r11",b"r12",b"r13",b"r14",b"r15"];
pub unsafe fn regs_query_register_offset(name: *const c_char) -> i32 { if name.is_null() || *name != b'r' as c_char { return -EINVAL; } let s = core::ffi::CStr::from_ptr(name).to_bytes(); if s.len() < 2 { return -EINVAL; } let n = match core::str::from_utf8(&s[1..]).ok().and_then(|x| x.parse::<usize>().ok()) { Some(v) => v, None => return -EINVAL }; if n >= NUM_GPRS { -EINVAL } else { n as i32 } }
pub unsafe fn regs_query_register_name(offset: u32) -> *const c_char { if offset >= NUM_GPRS as u32 { core::ptr::null() } else { GPR_NAMES[offset as usize].as_ptr() as *const c_char } }

// The following regset callbacks retain the C interfaces and delegate byte
// movement and architecture-specific state operations to the supplied kernel
// helpers, whose declarations are provided by the surrounding kernel port.
pub unsafe fn s390_regs_get(target: *mut task_struct, _regset: *const user_regset, mut to: membuf) -> i32 {
    if target == current { save_access_regs((*target).thread.acrs.as_mut_ptr()); }
    let mut pos = 0; while pos < core::mem::size_of::<s390_regs>() { membuf_store(&mut to, __peek_user(target, pos)); pos += core::mem::size_of::<c_ulong>(); } 0
}
pub unsafe fn s390_regs_set(target: *mut task_struct, _regset: *const user_regset, mut pos: u32, mut count: u32, kbuf: *const c_void, ubuf: *const c_void) -> i32 {
    let mut rc = 0; if target == current { save_access_regs((*target).thread.acrs.as_mut_ptr()); }
    if !kbuf.is_null() { let mut k = kbuf as *const c_ulong; while count > 0 && rc == 0 { rc = __poke_user(target, pos as _, *k as _); k = k.add(1); count -= core::mem::size_of::<c_ulong>() as u32; pos += core::mem::size_of::<c_ulong>() as u32; } } else { let mut u = ubuf as *const c_ulong; while count > 0 && rc == 0 { let mut word = 0; rc = __get_user(&mut word, u); if rc != 0 { break; } rc = __poke_user(target, pos as _, word); u = u.add(1); count -= core::mem::size_of::<c_ulong>() as u32; pos += core::mem::size_of::<c_ulong>() as u32; } } if rc == 0 && target == current { restore_access_regs((*target).thread.acrs.as_mut_ptr()); } rc
}
pub unsafe fn s390_last_break_get(target: *mut task_struct, _: *const user_regset, mut to: membuf) -> i32 { membuf_store(&mut to, (*target).thread.last_break) }
pub unsafe fn s390_last_break_set(_: *mut task_struct, _: *const user_regset, _: u32, _: u32, _: *const c_void, _: *const c_void) -> i32 { 0 }
pub unsafe fn s390_tdb_get(target: *mut task_struct, _: *const user_regset, mut to: membuf) -> i32 { if (*task_pt_regs(target)).int_code & 0x200 == 0 { return -ENODATA; } membuf_write(&mut to, (*target).thread.trap_tdb.data.as_ptr() as *const _, core::mem::size_of_val(&(*target).thread.trap_tdb.data)) }
pub unsafe fn s390_tdb_set(_: *mut task_struct, _: *const user_regset, _: u32, _: u32, _: *const c_void, _: *const c_void) -> i32 { 0 }
pub unsafe fn s390_system_call_get(target: *mut task_struct, _: *const user_regset, mut to: membuf) -> i32 { membuf_store(&mut to, (*target).thread.system_call) }
pub unsafe fn s390_system_call_set(target: *mut task_struct, _: *const user_regset, mut pos: u32, mut count: u32, k: *const c_void, u: *const c_void) -> i32 { user_regset_copyin(&mut pos, &mut count, &mut (k as *mut _), &mut (u as *mut _), &mut (*target).thread.system_call as *mut _, 0, core::mem::size_of::<u32>()) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
