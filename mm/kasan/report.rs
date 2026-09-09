// SPDX-License-Identifier: GPL-2.0
/*
 * This file contains common KASAN error reporting code.
 *
 * Copyright (c) 2014 Samsung Electronics Co., Ltd.
 * Author: Andrey Ryabinin <ryabinin.a.a@gmail.com>
 *
 * Some code borrowed from https://github.com/xairy/kasan-prototype by
 *        Andrey Konovalov <andreyknvl@gmail.com>
 */

// C dependencies supplied by the surrounding kernel translation unit.

static mut kasan_flags: c_ulong = 0;

const KASAN_BIT_REPORTED: usize = 0;
const KASAN_BIT_MULTI_SHOT: usize = 1;

#[repr(C)]
enum kasan_arg_fault {
    KASAN_ARG_FAULT_DEFAULT,
    KASAN_ARG_FAULT_REPORT,
    KASAN_ARG_FAULT_PANIC,
    KASAN_ARG_FAULT_PANIC_ON_WRITE,
}

static mut kasan_arg_fault: kasan_arg_fault = kasan_arg_fault::KASAN_ARG_FAULT_DEFAULT;

/* kasan.fault=report/panic */
unsafe extern "C" fn early_kasan_fault(arg: *mut c_char) -> c_int {
    if arg.is_null() { return -EINVAL; }
    if strcmp(arg, b"report\0".as_ptr() as *const c_char) == 0 {
        kasan_arg_fault = kasan_arg_fault::KASAN_ARG_FAULT_REPORT;
    } else if strcmp(arg, b"panic\0".as_ptr() as *const c_char) == 0 {
        kasan_arg_fault = kasan_arg_fault::KASAN_ARG_FAULT_PANIC;
    } else if strcmp(arg, b"panic_on_write\0".as_ptr() as *const c_char) == 0 {
        kasan_arg_fault = kasan_arg_fault::KASAN_ARG_FAULT_PANIC_ON_WRITE;
    } else { return -EINVAL; }
    0
}

unsafe extern "C" fn kasan_set_multi_shot(_str: *mut c_char) -> c_int {
    set_bit(KASAN_BIT_MULTI_SHOT, &raw mut kasan_flags);
    1
}

/*
 * This function is used to check whether KASAN reports are suppressed for
 * software KASAN modes via kasan_disable/enable_current() critical sections.
 */
unsafe fn report_suppressed_sw() -> bool {
    // CONFIG_KASAN_GENERIC || CONFIG_KASAN_SW_TAGS
    if (*current).kasan_depth != 0 { return true; }
    false
}

unsafe fn report_suppress_start() {
    // CONFIG_KASAN_HW_TAGS
    kasan_disable_current();
}

unsafe fn report_suppress_stop() { kasan_enable_current(); }

unsafe fn report_enabled() -> bool {
    if test_bit(KASAN_BIT_MULTI_SHOT, &raw mut kasan_flags) { return true; }
    !test_and_set_bit(KASAN_BIT_REPORTED, &raw mut kasan_flags)
}

unsafe fn kasan_save_enable_multi_shot() -> bool {
    test_and_set_bit(KASAN_BIT_MULTI_SHOT, &raw mut kasan_flags)
}

unsafe fn kasan_restore_multi_shot(enabled: bool) {
    if !enabled { clear_bit(KASAN_BIT_MULTI_SHOT, &raw mut kasan_flags); }
}

static mut kasan_kunit_executing: bool = false;

unsafe fn kasan_kunit_test_suite_start() { WRITE_ONCE(&raw mut kasan_kunit_executing, true); }
unsafe fn kasan_kunit_test_suite_end() { WRITE_ONCE(&raw mut kasan_kunit_executing, false); }
unsafe fn kasan_kunit_test_suite_executing() -> bool { READ_ONCE(&raw mut kasan_kunit_executing) }

unsafe fn fail_non_kasan_kunit_test() {
    let test: *mut kunit;
    if kasan_kunit_test_suite_executing() { return; }
    test = (*current).kunit_test;
    if !test.is_null() { kunit_set_failure(test); }
}

static mut report_lock: raw_spinlock_t = DEFINE_RAW_SPINLOCK();

unsafe fn start_report(flags: *mut c_ulong) {
    fail_non_kasan_kunit_test();
    disable_trace_on_warning();
    lockdep_off();
    report_suppress_start();
    raw_spin_lock_irqsave(&raw mut report_lock, flags);
    pr_err(b"==================================================================\n\0".as_ptr() as *const c_char);
}

unsafe fn end_report(flags: *mut c_ulong, addr: *const c_void, is_write: bool) {
    if !addr.is_null() { trace_error_report_end(ERROR_DETECTOR_KASAN, addr as c_ulong); }
    pr_err(b"==================================================================\n\0".as_ptr() as *const c_char);
    raw_spin_unlock_irqrestore(&raw mut report_lock, *flags);
    if !test_bit(KASAN_BIT_MULTI_SHOT, &raw mut kasan_flags) { check_panic_on_warn(b"KASAN\0".as_ptr() as *const c_char); }
    match kasan_arg_fault {
        kasan_arg_fault::KASAN_ARG_FAULT_DEFAULT | kasan_arg_fault::KASAN_ARG_FAULT_REPORT => {},
        kasan_arg_fault::KASAN_ARG_FAULT_PANIC => panic(b"kasan.fault=panic set ...\n\0".as_ptr() as *const c_char),
        kasan_arg_fault::KASAN_ARG_FAULT_PANIC_ON_WRITE => if is_write { panic(b"kasan.fault=panic_on_write set ...\n\0".as_ptr() as *const c_char); },
    }
    add_taint(TAINT_BAD_PAGE, LOCKDEP_NOW_UNRELIABLE);
    lockdep_on();
    report_suppress_stop();
}

unsafe fn print_error_description(info: *mut kasan_report_info) {
    pr_err(b"BUG: KASAN: %s in %pS\n\0".as_ptr() as *const c_char, (*info).bug_type, (*info).ip as *const c_void);
    if (*info).type_ != KASAN_REPORT_ACCESS {
        pr_err(b"Free of addr %px by task %s/%d\n\0".as_ptr() as *const c_char, (*info).access_addr, (*current).comm.as_ptr(), task_pid_nr(current)); return;
    }
    if (*info).access_size != 0 { pr_err(b"%s of size %zu at addr %px by task %s/%d\n\0".as_ptr() as *const c_char, if (*info).is_write { b"Write\0".as_ptr() } else { b"Read\0".as_ptr() }, (*info).access_size, (*info).access_addr, (*current).comm.as_ptr(), task_pid_nr(current)); }
    else { pr_err(b"%s at addr %px by task %s/%d\n\0".as_ptr() as *const c_char, if (*info).is_write { b"Write\0".as_ptr() } else { b"Read\0".as_ptr() }, (*info).access_addr, (*current).comm.as_ptr(), task_pid_nr(current)); }
}

unsafe fn print_track(track: *mut kasan_track, prefix: *const c_char) {
    // CONFIG_KASAN_EXTRA_INFO: timestamp formatting is preserved by the C ABI helper path.
    pr_err(b"%s by task %u:\n\0".as_ptr() as *const c_char, prefix, (*track).pid);
    if (*track).stack != 0 { stack_depot_print((*track).stack); } else { pr_err(b"(stack is not available)\n\0".as_ptr() as *const c_char); }
}

unsafe fn addr_to_page(addr: *const c_void) -> *mut page { if virt_addr_valid(addr) { virt_to_head_page(addr) } else { core::ptr::null_mut() } }

unsafe fn describe_object_addr(addr: *const c_void, info: *mut kasan_report_info) {
    let access_addr = addr as c_ulong; let object_addr = (*info).object as c_ulong;
    let (rel_type, rel_bytes) = if access_addr < object_addr { (b"to the left\0".as_ptr(), object_addr - access_addr) } else if access_addr >= object_addr + (*info).alloc_size { (b"to the right\0".as_ptr(), access_addr - object_addr - (*info).alloc_size) } else { (b"inside\0".as_ptr(), access_addr - object_addr) };
    let mut region_state = b"\0".as_ptr();
    if strcmp((*info).bug_type, b"slab-out-of-bounds\0".as_ptr() as *const c_char) == 0 { region_state = b"allocated \0".as_ptr(); }
    else if strcmp((*info).bug_type, b"slab-use-after-free\0".as_ptr() as *const c_char) == 0 { region_state = b"freed \0".as_ptr(); }
    pr_err(b"The buggy address belongs to the object at %px\n which belongs to the cache %s of size %d\n\0".as_ptr() as *const c_char, (*info).object, (*(*info).cache).name, (*(*info).cache).object_size);
    pr_err(b"The buggy address is located %d bytes %s of\n %s%zu-byte region [%px, %px)\n\0".as_ptr() as *const c_char, rel_bytes, rel_type, region_state, (*info).alloc_size, object_addr as *const c_void, (object_addr + (*info).alloc_size) as *const c_void);
}

unsafe fn describe_object_stacks(info: *mut kasan_report_info) { if (*info).alloc_track.stack != 0 { print_track(&raw mut (*info).alloc_track, b"Allocated\0".as_ptr() as *const c_char); pr_err(b"\n\0".as_ptr() as *const c_char); } if (*info).free_track.stack != 0 { print_track(&raw mut (*info).free_track, b"Freed\0".as_ptr() as *const c_char); pr_err(b"\n\0".as_ptr() as *const c_char); } kasan_print_aux_stacks((*info).cache, (*info).object); }
unsafe fn describe_object(addr: *const c_void, info: *mut kasan_report_info) { if kasan_stack_collection_enabled() { describe_object_stacks(info); } describe_object_addr(addr, info); }
unsafe fn kernel_or_module_addr(addr: *const c_void) -> bool { is_kernel(addr as c_ulong) || is_module_address(addr as c_ulong) }
unsafe fn init_task_stack_addr(addr: *const c_void) -> bool { addr >= (&raw const init_thread_union.stack) as *const _ as *const c_void && addr <= ((&raw const init_thread_union.stack) as *const _ as *const u8).add(core::mem::size_of_val(&init_thread_union.stack)) as *const c_void }

unsafe fn print_address_description(addr: *mut c_void, _tag: u8, info: *mut kasan_report_info) {
    let mut page = addr_to_page(addr); dump_stack_lvl(KERN_ERR); pr_err(b"\n\0".as_ptr() as *const c_char);
    if !(*info).cache.is_null() && !(*info).object.is_null() { describe_object(addr, info); pr_err(b"\n\0".as_ptr() as *const c_char); }
    if kernel_or_module_addr(addr) && !init_task_stack_addr(addr) { pr_err(b"The buggy address belongs to the variable:\n %pS\n\n\0".as_ptr() as *const c_char, addr); }
    if object_is_on_stack(addr) { kasan_print_address_stack_frame(addr); pr_err(b"\n\0".as_ptr() as *const c_char); }
    if is_vmalloc_addr(addr) { pr_err(b"The buggy address belongs to a\0".as_ptr() as *const c_char); if !vmalloc_dump_obj(addr) { pr_cont(b" vmalloc virtual mapping\n\0".as_ptr() as *const c_char); } page = vmalloc_to_page(addr); }
    if !page.is_null() { pr_err(b"The buggy address belongs to the physical page:\n\0".as_ptr() as *const c_char); dump_page(page, b"kasan: bad access detected\0".as_ptr() as *const c_char); pr_err(b"\n\0".as_ptr() as *const c_char); }
}

unsafe fn meta_row_is_guilty(row: *const c_void, addr: *const c_void) -> bool { row <= addr && addr < (row as *const u8).add(META_MEM_BYTES_PER_ROW) as *const c_void }
unsafe fn meta_pointer_offset(row: *const c_void, addr: *const c_void) -> c_int { 3 + (BITS_PER_LONG / 8) * 2 + ((addr as usize - row as usize) / KASAN_GRANULE_SIZE * 3 + 1) as c_int }

unsafe fn print_memory_metadata(addr: *const c_void) {
    let mut row = ((addr as usize & !(META_MEM_BYTES_PER_ROW - 1)) - META_ROWS_AROUND_ADDR * META_MEM_BYTES_PER_ROW) as *mut c_void;
    pr_err(b"Memory state around the buggy address:\n\0".as_ptr() as *const c_char);
    for i in -(META_ROWS_AROUND_ADDR as c_int)..=(META_ROWS_AROUND_ADDR as c_int) {
        let mut metadata = [0i8; META_BYTES_PER_ROW]; let buffer = [0i8; 4 + (BITS_PER_LONG / 8) * 2];
        snprintf(buffer.as_ptr() as *mut c_char, buffer.len(), if i == 0 { b">%px: \0" } else { b" %px: \0" }.as_ptr() as *const c_char, row);
        kasan_metadata_fetch_row(metadata.as_mut_ptr(), row);
        print_hex_dump(KERN_ERR, buffer.as_ptr(), DUMP_PREFIX_NONE, META_BYTES_PER_ROW, 1, metadata.as_ptr(), META_BYTES_PER_ROW, 0);
        if meta_row_is_guilty(row, addr) { pr_err(b"%*c\n\0".as_ptr() as *const c_char, meta_pointer_offset(row, addr), '^' as c_int); }
        row = row.add(META_MEM_BYTES_PER_ROW);
    }
}

unsafe fn print_report(info: *mut kasan_report_info) { let addr = kasan_reset_tag((*info).access_addr as *mut c_void); let tag = get_tag((*info).access_addr as *mut c_void); print_error_description(info); if addr_has_metadata(addr) { kasan_print_tags(tag, (*info).first_bad_addr); } pr_err(b"\n\0".as_ptr() as *const c_char); if addr_has_metadata(addr) { print_address_description(addr, tag, info); print_memory_metadata((*info).first_bad_addr); } else { dump_stack_lvl(KERN_ERR); } }

unsafe fn complete_report_info(info: *mut kasan_report_info) {
    let addr = kasan_reset_tag((*info).access_addr as *mut c_void); let slab;
    if (*info).type_ == KASAN_REPORT_ACCESS { (*info).first_bad_addr = kasan_find_first_bad_addr((*info).access_addr as *mut c_void, (*info).access_size); } else { (*info).first_bad_addr = addr; }
    slab = kasan_addr_to_slab(addr); if !slab.is_null() { (*info).cache = (*slab).slab_cache; (*info).object = nearest_obj((*info).cache, slab, addr); (*info).alloc_size = kasan_get_alloc_size((*info).object, (*info).cache); if (*info).alloc_size == 0 { (*info).alloc_size = (*(*info).cache).object_size; } } else { (*info).cache = core::ptr::null_mut(); (*info).object = core::ptr::null_mut(); }
    match (*info).type_ { KASAN_REPORT_INVALID_FREE => (*info).bug_type = b"invalid-free\0".as_ptr() as *const c_char, KASAN_REPORT_DOUBLE_FREE => (*info).bug_type = b"double-free\0".as_ptr() as *const c_char, _ => {} }
    kasan_complete_mode_report_info(info);
}

unsafe fn kasan_report_invalid_free(ptr: *mut c_void, ip: c_ulong, type_: kasan_report_type) {
    let mut flags = 0; let mut info: kasan_report_info = core::mem::zeroed(); if !report_enabled() { return; } start_report(&raw mut flags); info.type_ = type_; info.access_addr = ptr as *const c_void; info.access_size = 0; info.is_write = false; info.ip = ip; complete_report_info(&raw mut info); print_report(&raw mut info); end_report(&raw mut flags, ptr, true);
}

unsafe fn kasan_report(addr: *const c_void, size: usize, is_write: bool, ip: c_ulong) -> bool {
    let ua_flags = user_access_save(); let mut irq_flags = 0; let mut info: kasan_report_info = core::mem::zeroed(); let mut ret = true;
    if report_suppressed_sw() || !report_enabled() { ret = false; user_access_restore(ua_flags); return ret; }
    start_report(&raw mut irq_flags); info.type_ = KASAN_REPORT_ACCESS; info.access_addr = addr; info.access_size = size; info.is_write = is_write; info.ip = ip; complete_report_info(&raw mut info); print_report(&raw mut info); end_report(&raw mut irq_flags, addr, is_write); user_access_restore(ua_flags); ret
}

// CONFIG_KASAN_HW_TAGS
unsafe fn kasan_report_async() { let mut flags = 0; if !report_enabled() { return; } start_report(&raw mut flags); pr_err(b"BUG: KASAN: invalid-access\n\0".as_ptr() as *const c_char); pr_err(b"Asynchronous fault: no details available\n\n\0".as_ptr() as *const c_char); dump_stack_lvl(KERN_ERR); end_report(&raw mut flags, core::ptr::null(), true); }

// CONFIG_KASAN_GENERIC || CONFIG_KASAN_SW_TAGS
unsafe fn kasan_non_canonical_hook(addr: c_ulong) {
    if addr < KASAN_SHADOW_OFFSET { return; }
    let mut orig_addr = kasan_shadow_to_mem(addr as *mut c_void) as c_ulong; let user_orig_addr = set_tag(orig_addr as *mut c_void, 0) as c_ulong;
    let bug_type;
    if user_orig_addr < PAGE_SIZE { bug_type = b"null-ptr-deref\0"; orig_addr = user_orig_addr; } else if user_orig_addr < TASK_SIZE { bug_type = b"probably user-memory-access\0"; orig_addr = user_orig_addr; } else if addr_in_shadow(addr as *mut c_void) { bug_type = b"probably wild-memory-access\0"; } else { bug_type = b"maybe wild-memory-access\0"; }
    pr_alert(b"KASAN: %s in range [0x%016lx-0x%016lx]\n\0".as_ptr() as *const c_char, bug_type.as_ptr() as *const c_char, orig_addr, orig_addr + KASAN_GRANULE_SIZE - 1);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
