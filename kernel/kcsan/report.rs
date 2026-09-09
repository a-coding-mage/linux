// SPDX-License-Identifier: GPL-2.0
/*
 * KCSAN reporting.
 *
 * Copyright (C) 2019, Google LLC.
 */

// Linux headers and local headers are dependencies supplied by other files.

const NUM_STACK_ENTRIES: usize = 64;

#[repr(C)]
struct access_info {
    ptr: *const core::ffi::c_void,
    size: usize,
    access_type: i32,
    task_pid: i32,
    cpu_id: i32,
    ip: usize,
}

#[repr(C)]
struct other_info {
    ai: access_info,
    stack_entries: [usize; NUM_STACK_ENTRIES],
    num_stack_entries: i32,
    task: *mut task_struct,
}

#[repr(C)]
struct report_time {
    time: usize,
    frame1: usize,
    frame2: usize,
}

// Build-time constants and external kernel types/functions are supplied by the surrounding translation.
extern "C" {
    static mut other_infos: [other_info; CONFIG_KCSAN_NUM_WATCHPOINTS + NUM_SLOTS - 1];
    static mut report_times: [report_time; REPORT_TIMES_SIZE];
}

unsafe fn rate_limit_report(frame1: usize, frame2: usize) -> bool {
    let mut use_entry = &mut report_times[0] as *mut report_time;
    if CONFIG_KCSAN_REPORT_ONCE_IN_MS == 0 { return false; }
    let invalid_before = jiffies.wrapping_sub(msecs_to_jiffies(CONFIG_KCSAN_REPORT_ONCE_IN_MS));
    let mut i = 0;
    while i < REPORT_TIMES_SIZE {
        let rt = &mut report_times[i] as *mut report_time;
        if time_before((*rt).time, (*use_entry).time) { use_entry = rt; }
        if (*rt).time == 0 { break; }
        if time_before((*rt).time, invalid_before) { i += 1; continue; }
        if ((*rt).frame1 == frame1 && (*rt).frame2 == frame2) ||
           ((*rt).frame1 == frame2 && (*rt).frame2 == frame1) { return true; }
        i += 1;
    }
    (*use_entry).time = jiffies;
    (*use_entry).frame1 = frame1;
    (*use_entry).frame2 = frame2;
    false
}

unsafe fn skip_report(value_change: i32, top_frame: usize) -> bool {
    WARN_ON_ONCE(value_change == KCSAN_VALUE_CHANGE_FALSE);
    if IS_ENABLED(CONFIG_KCSAN_REPORT_VALUE_CHANGE_ONLY) && value_change == KCSAN_VALUE_CHANGE_MAYBE {
        let mut buf = [0i8; 64];
        let len = scnprintf(buf.as_mut_ptr(), buf.len(), b"%ps\0".as_ptr(), top_frame as *const core::ffi::c_void);
        if strnstr(buf.as_mut_ptr(), b"rcu_\0".as_ptr(), len).is_null() &&
           strnstr(buf.as_mut_ptr(), b"_rcu\0".as_ptr(), len).is_null() &&
           strnstr(buf.as_mut_ptr(), b"_srcu\0".as_ptr(), len).is_null() { return true; }
    }
    kcsan_skip_report_debugfs(top_frame)
}

unsafe fn get_access_type(typ: i32) -> *const u8 {
    if typ & KCSAN_ACCESS_ASSERT != 0 {
        if typ & KCSAN_ACCESS_SCOPED != 0 { return if typ & KCSAN_ACCESS_WRITE != 0 { b"assert no accesses (reordered)\0".as_ptr() } else { b"assert no writes (reordered)\0".as_ptr() }; }
        return if typ & KCSAN_ACCESS_WRITE != 0 { b"assert no accesses\0".as_ptr() } else { b"assert no writes\0".as_ptr() };
    }
    match typ {
        0 => b"read\0".as_ptr(),
        KCSAN_ACCESS_ATOMIC => b"read (marked)\0".as_ptr(),
        KCSAN_ACCESS_WRITE => b"write\0".as_ptr(),
        x if x == KCSAN_ACCESS_WRITE | KCSAN_ACCESS_ATOMIC => b"write (marked)\0".as_ptr(),
        x if x == KCSAN_ACCESS_COMPOUND | KCSAN_ACCESS_WRITE => b"read-write\0".as_ptr(),
        x if x == KCSAN_ACCESS_COMPOUND | KCSAN_ACCESS_WRITE | KCSAN_ACCESS_ATOMIC => b"read-write (marked)\0".as_ptr(),
        KCSAN_ACCESS_SCOPED => b"read (reordered)\0".as_ptr(),
        x if x == KCSAN_ACCESS_SCOPED | KCSAN_ACCESS_ATOMIC => b"read (marked, reordered)\0".as_ptr(),
        x if x == KCSAN_ACCESS_SCOPED | KCSAN_ACCESS_WRITE => b"write (reordered)\0".as_ptr(),
        x if x == KCSAN_ACCESS_SCOPED | KCSAN_ACCESS_WRITE | KCSAN_ACCESS_ATOMIC => b"write (marked, reordered)\0".as_ptr(),
        x if x == KCSAN_ACCESS_SCOPED | KCSAN_ACCESS_COMPOUND | KCSAN_ACCESS_WRITE => b"read-write (reordered)\0".as_ptr(),
        _ => { BUG(); core::ptr::null() }
    }
}

unsafe fn get_bug_type(typ: i32) -> *const u8 { if typ & KCSAN_ACCESS_ASSERT != 0 { b"assert: race\0".as_ptr() } else { b"data-race\0".as_ptr() } }

unsafe fn get_thread_desc(task_id: i32) -> *const i8 {
    if task_id != -1 { static mut BUF: [i8; 32] = [0; 32]; snprintf(BUF.as_mut_ptr(), BUF.len(), b"task %i\0".as_ptr(), task_id); return BUF.as_ptr(); }
    b"interrupt\0".as_ptr() as *const i8
}

unsafe fn get_stack_skipnr(stack_entries: *const usize, num_entries: i32) -> i32 {
    let mut buf = [0i8; 64]; let mut skip = 0;
    while skip < num_entries {
        let len = scnprintf(buf.as_mut_ptr(), buf.len(), b"%ps\0".as_ptr(), *stack_entries.offset(skip as isize));
        if !strnstr(buf.as_mut_ptr(), b"tsan_\0".as_ptr(), len).is_null() || !strnstr(buf.as_mut_ptr(), b"_once_size\0".as_ptr(), len).is_null() { skip += 1; continue; }
        let cur = strnstr(buf.as_mut_ptr(), b"kcsan_\0".as_ptr(), len);
        if !cur.is_null() && !str_has_prefix(cur.add(6), b"test\0".as_ptr()) { skip += 1; continue; }
        break;
    }
    skip
}

unsafe fn replace_stack_entry(stack_entries: *mut usize, num_entries: i32, ip: usize, replaced: *mut usize) -> i32 {
    let mut symbolsize = 0usize; let mut offset = 0usize; let target_func;
    if kallsyms_lookup_size_offset(ip, &mut symbolsize, &mut offset) { target_func = ip.wrapping_sub(offset); } else { return get_stack_skipnr(stack_entries, num_entries); }
    let mut skip = 0;
    while skip < num_entries { let funcp = stack_entries.offset(skip as isize); if !kallsyms_lookup_size_offset(*funcp, &mut symbolsize, &mut offset) { return get_stack_skipnr(stack_entries, num_entries); } if (*funcp).wrapping_sub(offset) == target_func { *replaced = *funcp; *funcp = ip; return skip; } skip += 1; }
    WARN_ONCE(true, b"Cannot find frame for %pS in stack trace\0".as_ptr(), ip); get_stack_skipnr(stack_entries, num_entries)
}

unsafe fn sanitize_stack_entries(stack_entries: *mut usize, num_entries: i32, ip: usize, replaced: *mut usize) -> i32 { if ip != 0 { replace_stack_entry(stack_entries, num_entries, ip, replaced) } else { get_stack_skipnr(stack_entries, num_entries) } }

unsafe fn sym_strcmp(addr1: *mut core::ffi::c_void, addr2: *mut core::ffi::c_void) -> i32 { let mut a=[0i8;64]; let mut b=[0i8;64]; snprintf(a.as_mut_ptr(),64,b"%pS\0".as_ptr(),addr1); snprintf(b.as_mut_ptr(),64,b"%pS\0".as_ptr(),addr2); strncmp(a.as_ptr(),b.as_ptr(),64) }

unsafe fn print_stack_trace(entries: *mut usize, n: i32, reordered: usize) { stack_trace_print(entries,n,0); if reordered != 0 { pr_err(b"  |\n  +-> reordered to: %pS\n\0".as_ptr(),reordered); } }
unsafe fn print_verbose_info(task: *mut task_struct) { if task.is_null(){return;} kcsan_restore_irqtrace(task); pr_err(b"\n\0".as_ptr()); debug_show_held_locks(task); print_irqtrace_events(task); }

unsafe fn print_report(value_change:i32, ai:*const access_info, other:*mut other_info, old:u64, new:u64, mask:u64) {
    let mut entries=[0usize;NUM_STACK_ENTRIES]; let n=stack_trace_save(entries.as_mut_ptr(),NUM_STACK_ENTRIES as i32,1); let mut reordered=0; let skip=sanitize_stack_entries(entries.as_mut_ptr(),n,(*ai).ip,&mut reordered); let this_frame=entries[skip as usize];
    if skip_report(KCSAN_VALUE_CHANGE_TRUE,this_frame){return;} let mut other_skip=0; let mut other_reordered=0; let mut other_frame=0;
    if !other.is_null(){other_skip=sanitize_stack_entries((*other).stack_entries.as_mut_ptr(),(*other).num_stack_entries,(*other).ai.ip,&mut other_reordered); other_frame=(*other).stack_entries[other_skip as usize]; if skip_report(value_change,other_frame){return;}}
    if rate_limit_report(this_frame,other_frame){return;} pr_err(b"==================================================================\n\0".as_ptr());
    if !other.is_null(){let cmp=sym_strcmp(other_frame as *mut _,this_frame as *mut _); pr_err(b"BUG: KCSAN: %s in %ps / %ps\n\0".as_ptr(),get_bug_type((*ai).access_type|(*other).ai.access_type),if cmp<0{other_frame}else{this_frame},if cmp<0{this_frame}else{other_frame});} else {pr_err(b"BUG: KCSAN: %s in %pS\n\0".as_ptr(),get_bug_type((*ai).access_type),this_frame);} pr_err(b"\n\0".as_ptr());
    if !other.is_null(){pr_err(b"%s to 0x%px of %zu bytes by %s on cpu %i:\n\0".as_ptr(),get_access_type((*other).ai.access_type),(*other).ai.ptr,(*other).ai.size,get_thread_desc((*other).ai.task_pid),(*other).ai.cpu_id); print_stack_trace((*other).stack_entries.as_mut_ptr().add(other_skip as usize),(*other).num_stack_entries-other_skip,other_reordered); if IS_ENABLED(CONFIG_KCSAN_VERBOSE){print_verbose_info((*other).task);} pr_err(b"\n%s to 0x%px of %zu bytes by %s on cpu %i:\n\0".as_ptr(),get_access_type((*ai).access_type),(*ai).ptr,(*ai).size,get_thread_desc((*ai).task_pid),(*ai).cpu_id);} else {pr_err(b"race at unknown origin, with %s to 0x%px of %zu bytes by %s on cpu %i:\n\0".as_ptr(),get_access_type((*ai).access_type),(*ai).ptr,(*ai).size,get_thread_desc((*ai).task_pid),(*ai).cpu_id);}
    print_stack_trace(entries.as_mut_ptr().add(skip as usize),n-skip,reordered); if IS_ENABLED(CONFIG_KCSAN_VERBOSE){print_verbose_info(current);} if (*ai).size<=8 {let mut diff=old^new;if mask!=0{diff&=mask;}if diff!=0{pr_err(b"\nvalue changed: 0x%0*llx -> 0x%0*llx\n\0".as_ptr(),(*ai).size*2,old,(*ai).size*2,new);if mask!=0{pr_err(b" bits changed: 0x%0*llx with mask 0x%0*llx\n\0".as_ptr(),(*ai).size*2,diff,(*ai).size*2,mask);}}} pr_err(b"\nReported by Kernel Concurrency Sanitizer on:\n\0".as_ptr()); dump_stack_print_info(KERN_DEFAULT); pr_err(b"==================================================================\n\0".as_ptr()); check_panic_on_warn(b"KCSAN\0".as_ptr());
}

unsafe fn release_report(flags:*mut usize, oi:*mut other_info){(*oi).ai.size=0;raw_spin_unlock_irqrestore(&report_lock,*flags);}

unsafe fn prepare_access_info(ptr:*const core::ffi::c_void,size:usize,access_type:i32,ip:usize)->access_info{access_info{ptr,size,access_type,task_pid:if in_task(){task_pid_nr(current)}else{-1},cpu_id:raw_smp_processor_id(),ip:if access_type&KCSAN_ACCESS_SCOPED!=0{ip}else{0}}}

pub unsafe fn kcsan_report_set_info(ptr:*const core::ffi::c_void,size:usize,access_type:i32,ip:usize,watchpoint_idx:usize){let ai=prepare_access_info(ptr,size,access_type,ip);let mut flags=0;kcsan_disable_current();lockdep_off();other_infos[watchpoint_idx].ai=ai;other_infos[watchpoint_idx].num_stack_entries=stack_trace_save(other_infos[watchpoint_idx].stack_entries.as_mut_ptr(),NUM_STACK_ENTRIES as i32,2);lockdep_on();kcsan_enable_current();}
pub unsafe fn kcsan_report_known_origin(ptr:*const core::ffi::c_void,size:usize,access_type:i32,ip:usize,value_change:i32,watchpoint_idx:usize,old:u64,new:u64,mask:u64){let ai=prepare_access_info(ptr,size,access_type,ip);let oi=&mut other_infos[watchpoint_idx] as *mut _;let mut flags=0;kcsan_disable_current();lockdep_off();raw_spin_lock_irqsave(&report_lock,&mut flags);while (*oi).ai.size==0{raw_spin_unlock_irqrestore(&report_lock,flags);cpu_relax();raw_spin_lock_irqsave(&report_lock,&mut flags);}if matching_access((*oi).ai.ptr as usize & WATCHPOINT_ADDR_MASK,(*oi).ai.size,ai.ptr as usize & WATCHPOINT_ADDR_MASK,ai.size)&&matching_access((*oi).ai.ptr as usize,(*oi).ai.size,ai.ptr as usize,ai.size)&&value_change!=KCSAN_VALUE_CHANGE_FALSE{print_report(value_change,&ai,oi,old,new,mask);}release_report(&mut flags,oi);lockdep_on();kcsan_enable_current();}
pub unsafe fn kcsan_report_unknown_origin(ptr:*const core::ffi::c_void,size:usize,access_type:i32,ip:usize,old:u64,new:u64,mask:u64){let ai=prepare_access_info(ptr,size,access_type,ip);let mut flags=0;kcsan_disable_current();lockdep_off();raw_spin_lock_irqsave(&report_lock,&mut flags);print_report(KCSAN_VALUE_CHANGE_TRUE,&ai,core::ptr::null_mut(),old,new,mask);raw_spin_unlock_irqrestore(&report_lock,flags);lockdep_on();kcsan_enable_current();}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
