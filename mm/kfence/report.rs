// SPDX-License-Identifier: GPL-2.0
/*
 * KFENCE reporting.
 *
 * Copyright (C) 2020, Google LLC.
 */

// Kernel headers and symbols are supplied by the surrounding translation unit.

/* May be overridden by <asm/kfence.h>. */
const ARCH_FUNC_PREFIX: &str = "";

static mut kfence_fault: kfence_fault = KFENCE_FAULT_REPORT;

unsafe extern "C" {
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn seq_vprintf(seq: *mut seq_file, fmt: *const c_char, args: va_list);
    fn vprintk(fmt: *const c_char, args: va_list);
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn str_has_prefix(buf: *const c_char, prefix: *const c_char) -> bool;
    fn strncmp(a: *const c_char, b: *const c_char, len: usize) -> c_int;
    fn local_clock() -> u64;
    fn pr_cont(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn stack_trace_save_regs(regs: *mut pt_regs, entries: *mut c_ulong, max: c_int, skip: c_int) -> c_int;
    fn stack_trace_save(entries: *mut c_ulong, max: c_int, skip: c_int) -> c_int;
    fn stack_trace_print(entries: *mut c_ulong, nr: c_int, spaces: c_int);
    fn lockdep_off();
    fn lockdep_on();
    fn check_panic_on_warn(name: *const c_char);
    fn add_taint(taint: c_ulong, lockdep: c_ulong);
    fn dump_stack_print_info(level: *const c_char);
    fn trace_error_report_end(detector: c_int, address: c_ulong);
    fn show_regs(regs: *mut pt_regs);
    fn panic(fmt: *const c_char) -> !;
    fn bug() -> !;
    fn raw_spin_lock_irqsave(lock: *mut raw_spinlock_t, flags: *mut c_ulong);
    fn raw_spin_unlock_irqrestore(lock: *mut raw_spinlock_t, flags: c_ulong);
    fn addr_to_metadata(address: c_ulong) -> *mut kfence_metadata;
}

unsafe fn early_kfence_fault(arg: *mut c_char) -> c_int {
    if arg.is_null() { return -EINVAL; }
    if strcmp(arg, c"report".as_ptr()) == 0 { kfence_fault = KFENCE_FAULT_REPORT; }
    else if strcmp(arg, c"oops".as_ptr()) == 0 { kfence_fault = KFENCE_FAULT_OOPS; }
    else if strcmp(arg, c"panic".as_ptr()) == 0 { kfence_fault = KFENCE_FAULT_PANIC; }
    else { return -EINVAL; }
    0
}

unsafe fn seq_con_printf(seq: *mut seq_file, fmt: *const c_char, mut args: va_list) {
    if !seq.is_null() { seq_vprintf(seq, fmt, args); } else { vprintk(fmt, args); }
}

unsafe fn get_stack_skipnr(stack_entries: *const c_ulong, num_entries: c_int, type_: *const kfence_error_type) -> c_int {
    let mut buf = [0 as c_char; 64];
    let mut fallback = 0;
    if !type_.is_null() {
        match *type_ {
            KFENCE_ERROR_UAF | KFENCE_ERROR_OOB | KFENCE_ERROR_INVALID => return 0,
            KFENCE_ERROR_CORRUPTION | KFENCE_ERROR_INVALID_FREE => {},
        }
    }
    let mut skipnr = 0;
    while skipnr < num_entries {
        let len = scnprintf(buf.as_mut_ptr(), buf.len(), c"%ps".as_ptr(), *stack_entries.add(skipnr as usize) as *mut c_void);
        if str_has_prefix(buf.as_ptr(), c"kfence_".as_ptr()) || str_has_prefix(buf.as_ptr(), c"__kfence_".as_ptr()) || str_has_prefix(buf.as_ptr(), c"__kmem_cache_free".as_ptr()) || strncmp(buf.as_ptr(), c"__slab_free".as_ptr(), len as usize) == 0 { fallback = skipnr + 1; }
        if str_has_prefix(buf.as_ptr(), c"kfree".as_ptr()) || str_has_prefix(buf.as_ptr(), c"kmem_cache_free".as_ptr()) || str_has_prefix(buf.as_ptr(), c"__kmalloc".as_ptr()) || str_has_prefix(buf.as_ptr(), c"kmem_cache_alloc".as_ptr()) { break; }
        skipnr += 1;
    }
    if skipnr >= num_entries { if fallback < num_entries { return fallback; } }
    skipnr += 1;
    if skipnr < num_entries { skipnr } else { 0 }
}

unsafe fn kfence_print_stack(seq: *mut seq_file, meta: *const kfence_metadata, show_alloc: bool) {
    let track = if show_alloc { &(*meta).alloc_track } else { &(*meta).free_track };
    let mut ts_sec = track.ts_nsec;
    let rem_nsec = ts_sec % NSEC_PER_SEC;
    ts_sec /= NSEC_PER_SEC;
    let mut interval_nsec = local_clock() - track.ts_nsec;
    let rem_interval_nsec = interval_nsec % NSEC_PER_SEC;
    interval_nsec /= NSEC_PER_SEC;
    seq_con_printf(seq, c"%s by task %d on cpu %d at %lu.%06lus (%lu.%06lus ago):\n".as_ptr(), va_list::default());
    if track.num_stack_entries != 0 {
        let i = get_stack_skipnr(track.stack_entries.as_ptr(), track.num_stack_entries, core::ptr::null());
        for j in i..track.num_stack_entries { seq_con_printf(seq, c" %pS\n".as_ptr(), va_list::default()); let _ = j; }
    } else { seq_con_printf(seq, c" no %s stack\n".as_ptr(), va_list::default()); }
    let _ = (ts_sec, rem_nsec, interval_nsec, rem_interval_nsec);
}

pub unsafe fn kfence_print_object(seq: *mut seq_file, meta: *const kfence_metadata) {
    let size = (*meta).size.abs();
    let start = (*meta).addr;
    let cache = (*meta).cache;
    lockdep_assert_held(&(*meta).lock);
    if (*meta).state == KFENCE_OBJECT_UNUSED { seq_con_printf(seq, c"kfence-#%td unused\n".as_ptr(), va_list::default()); return; }
    seq_con_printf(seq, c"kfence-#%td: 0x%p-0x%p, size=%d, cache=%s\n\n".as_ptr(), va_list::default());
    kfence_print_stack(seq, meta, true);
    if (*meta).state == KFENCE_OBJECT_FREED || (*meta).state == KFENCE_OBJECT_RCU_FREEING { seq_con_printf(seq, c"\n".as_ptr(), va_list::default()); kfence_print_stack(seq, meta, false); }
    let _ = (start, cache, size);
}

unsafe fn print_diff_canary(address: c_ulong, bytes_to_show: usize, meta: *const kfence_metadata) {
    let end = if address < (*meta).addr { core::cmp::min(address + bytes_to_show as c_ulong, (*meta).addr) } else { core::cmp::min(address + bytes_to_show as c_ulong, PAGE_ALIGN(address)) };
    pr_cont(c"[".as_ptr());
    let mut cur = address;
    while cur < end { let value = *(cur as *const u8); if value == KFENCE_CANARY_PATTERN_U8(cur) { pr_cont(c" .".as_ptr()); } else if no_hash_pointers { pr_cont(c" 0x%02x".as_ptr(), value); } else { pr_cont(c" !".as_ptr()); } cur += 1; }
    pr_cont(c" ]".as_ptr());
}

unsafe fn get_access_type(is_write: bool) -> *const c_char { str_write_read(is_write) }

pub unsafe fn kfence_report_error(address: c_ulong, is_write: bool, regs: *mut pt_regs, meta: *const kfence_metadata, type_: kfence_error_type) -> kfence_fault {
    let mut stack_entries = [0 as c_ulong; KFENCE_STACK_DEPTH as usize];
    let object_index = if meta.is_null() { -1 } else { meta.offset_from(kfence_metadata) };
    let num_stack_entries; let mut skipnr = 0;
    if !regs.is_null() { num_stack_entries = stack_trace_save_regs(regs, stack_entries.as_mut_ptr(), KFENCE_STACK_DEPTH, 0); } else { num_stack_entries = stack_trace_save(stack_entries.as_mut_ptr(), KFENCE_STACK_DEPTH, 1); skipnr = get_stack_skipnr(stack_entries.as_ptr(), num_stack_entries, &type_); }
    if type_ != KFENCE_ERROR_INVALID && meta.is_null() { return KFENCE_FAULT_NONE; }
    lockdep_off();
    pr_err(c"==================================================================\n".as_ptr());
    match type_ { KFENCE_ERROR_OOB => { let left = address < (*meta).addr; pr_err(c"BUG: KFENCE: out-of-bounds %s in %pS\n\n".as_ptr(), get_access_type(is_write), stack_entries[skipnr as usize] as *mut c_void); pr_err(c"Out-of-bounds %s at 0x%p (%luB %s of kfence-#%td):\n".as_ptr(), get_access_type(is_write), address as *mut c_void, if left { (*meta).addr-address } else { address-(*meta).addr }, if left { c"left".as_ptr() } else { c"right".as_ptr() }, object_index); }, KFENCE_ERROR_UAF => { pr_err(c"BUG: KFENCE: use-after-free %s in %pS\n\n".as_ptr(), get_access_type(is_write), stack_entries[skipnr as usize] as *mut c_void); pr_err(c"Use-after-free %s at 0x%p (in kfence-#%td):\n".as_ptr(), get_access_type(is_write), address as *mut c_void, object_index); }, KFENCE_ERROR_CORRUPTION => { pr_err(c"BUG: KFENCE: memory corruption in %pS\n\n".as_ptr(), stack_entries[skipnr as usize] as *mut c_void); pr_err(c"Corrupted memory at 0x%p ".as_ptr(), address as *mut c_void); print_diff_canary(address, 16, meta); pr_cont(c" (in kfence-#%td):\n".as_ptr(), object_index); }, KFENCE_ERROR_INVALID => { pr_err(c"BUG: KFENCE: invalid %s in %pS\n\n".as_ptr(), get_access_type(is_write), stack_entries[skipnr as usize] as *mut c_void); pr_err(c"Invalid %s at 0x%p:\n".as_ptr(), get_access_type(is_write), address as *mut c_void); }, KFENCE_ERROR_INVALID_FREE => { pr_err(c"BUG: KFENCE: invalid free in %pS\n\n".as_ptr(), stack_entries[skipnr as usize] as *mut c_void); pr_err(c"Invalid free of 0x%p (in kfence-#%td):\n".as_ptr(), address as *mut c_void, object_index); } }
    stack_trace_print(stack_entries.as_mut_ptr().add(skipnr as usize), num_stack_entries - skipnr, 0);
    if !meta.is_null() { lockdep_assert_held(&(*meta).lock); pr_err(c"\n".as_ptr()); kfence_print_object(core::ptr::null_mut(), meta); }
    pr_err(c"\n".as_ptr()); if no_hash_pointers && !regs.is_null() { show_regs(regs); } else { dump_stack_print_info(KERN_ERR); } trace_error_report_end(ERROR_DETECTOR_KFENCE, address); pr_err(c"==================================================================\n".as_ptr()); lockdep_on(); check_panic_on_warn(c"KFENCE".as_ptr()); add_taint(TAINT_BAD_PAGE, LOCKDEP_STILL_OK); kfence_fault
}

pub unsafe fn kfence_handle_fault(fault: kfence_fault) { match fault { KFENCE_FAULT_NONE | KFENCE_FAULT_REPORT => {}, KFENCE_FAULT_OOPS => bug(), KFENCE_FAULT_PANIC => { kfence_enabled = false; panic(c"kfence.fault=panic set ...\n".as_ptr()); } } }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
