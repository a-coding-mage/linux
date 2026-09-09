// SPDX-License-Identifier: GPL-2.0-only
// Kernel dependencies are supplied by the surrounding Linux Rust bindings.

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

const MAX_STACK_TRACE_DEPTH: usize = 32;

// Per-CPU random state, supplied/defined by the kernel integration.
static mut FAULT_RND_STATE: u8 = 0;

unsafe fn fault_prandom_u32_below_100() -> u32 {
    // Equivalent to get_cpu_var()/put_cpu_var() and prandom_u32_state().
    let res = prandom_u32_state(&mut FAULT_RND_STATE as *mut _ as *mut c_void);
    res % 100
}

pub unsafe fn setup_fault_attr(attr: *mut fault_attr, str_: *mut c_char) -> c_int {
    let mut probability: c_ulong = 0;
    let mut interval: c_ulong = 0;
    let mut times: c_int = 0;
    let mut space: c_int = 0;
    if sscanf(str_, b"%lu,%lu,%d,%d\0".as_ptr() as *const c_char,
              &mut interval, &mut probability, &mut space, &mut times) < 4 {
        printk(b"FAULT_INJECTION: failed to parse arguments\n\0".as_ptr() as *const c_char);
        return 0;
    }
    prandom_init_once(&mut FAULT_RND_STATE as *mut _ as *mut c_void);
    (*attr).probability = probability;
    (*attr).interval = interval;
    atomic_set(&mut (*attr).times, times);
    atomic_set(&mut (*attr).space, space);
    1
}

unsafe fn fail_dump(attr: *mut fault_attr) {
    if (*attr).verbose > 0 && __ratelimit(&mut (*attr).ratelimit_state) {
        printk(b"FAULT_INJECTION: forcing a failure.\n\0".as_ptr() as *const c_char);
        if (*attr).verbose > 1 { dump_stack(); }
    }
}

unsafe fn fail_task(_attr: *mut fault_attr, task: *mut task_struct) -> bool {
    in_task() && (*task).make_it_fail
}

#[cfg(feature = "CONFIG_FAULT_INJECTION_STACKTRACE_FILTER")]
unsafe fn fail_stacktrace(attr: *mut fault_attr) -> bool {
    let depth = (*attr).stacktrace_depth;
    let mut entries = [0usize; MAX_STACK_TRACE_DEPTH];
    let mut found = (*attr).require_start == 0 && (*attr).require_end == c_ulong::MAX;
    if depth == 0 || (found && !(*attr).reject_start && !(*attr).reject_end) { return found; }
    let nr_entries = stack_trace_save(entries.as_mut_ptr(), depth, 1);
    for entry in entries.iter().take(nr_entries as usize) {
        if (*attr).reject_start <= *entry && *entry < (*attr).reject_end { return false; }
        if (*attr).require_start <= *entry && *entry < (*attr).require_end { found = true; }
    }
    found
}

#[cfg(not(feature = "CONFIG_FAULT_INJECTION_STACKTRACE_FILTER"))]
unsafe fn fail_stacktrace(_attr: *mut fault_attr) -> bool { true }

pub unsafe fn should_fail_ex(attr: *mut fault_attr, size: c_long, flags: c_int) -> bool {
    let mut stack_checked = false;
    if in_task() {
        let mut fail_nth = READ_ONCE((*current).fail_nth);
        if fail_nth != 0 {
            if !fail_stacktrace(attr) { return false; }
            stack_checked = true;
            fail_nth -= 1;
            WRITE_ONCE((*current).fail_nth, fail_nth);
            if fail_nth == 0 { return should_fail_finish(attr, flags); }
            return false;
        }
    }
    if (*attr).probability == 0 { return false; }
    if (*attr).task_filter && !fail_task(attr, current) { return false; }
    if atomic_read(&(*attr).times) == 0 { return false; }
    if !stack_checked && !fail_stacktrace(attr) { return false; }
    if atomic_read(&(*attr).space) as c_long > size {
        atomic_sub(size as c_int, &mut (*attr).space);
        return false;
    }
    if (*attr).interval > 1 {
        (*attr).count += 1;
        if (*attr).count % (*attr).interval != 0 { return false; }
    }
    if (*attr).probability <= fault_prandom_u32_below_100() as c_ulong { return false; }
    should_fail_finish(attr, flags)
}

unsafe fn should_fail_finish(attr: *mut fault_attr, flags: c_int) -> bool {
    if flags & FAULT_NOWARN == 0 { fail_dump(attr); }
    if atomic_read(&(*attr).times) != -1 { atomic_dec_not_zero(&mut (*attr).times); }
    true
}

pub unsafe fn should_fail(attr: *mut fault_attr, size: c_long) -> bool { should_fail_ex(attr, size, 0) }

#[cfg(feature = "CONFIG_FAULT_INJECTION_DEBUG_FS")]
pub unsafe fn fault_create_debugfs_attr(name: *const c_char, parent: *mut dentry, attr: *mut fault_attr) -> *mut dentry {
    let mode = S_IFREG | S_IRUSR | S_IWUSR;
    let dir = debugfs_create_dir(name, parent);
    if IS_ERR(dir) { return dir; }
    prandom_init_once(&mut FAULT_RND_STATE as *mut _ as *mut c_void);
    debugfs_create_ul(b"probability\0".as_ptr() as _, mode, dir, &mut (*attr).probability);
    debugfs_create_ul(b"interval\0".as_ptr() as _, mode, dir, &mut (*attr).interval);
    debugfs_create_atomic_t(b"times\0".as_ptr() as _, mode, dir, &mut (*attr).times);
    debugfs_create_atomic_t(b"space\0".as_ptr() as _, mode, dir, &mut (*attr).space);
    debugfs_create_ul(b"verbose\0".as_ptr() as _, mode, dir, &mut (*attr).verbose);
    debugfs_create_bool(b"task-filter\0".as_ptr() as _, mode, dir, &mut (*attr).task_filter);
    (*attr).dname = dget(dir);
    dir
}

#[cfg(feature = "CONFIG_FAULT_INJECTION_CONFIGFS")]
unsafe fn fault_uint_attr_show(val: u32, page: *mut c_char) -> isize { snprintf(page, PAGE_SIZE, b"%u\n\0".as_ptr() as _, val) }
#[cfg(feature = "CONFIG_FAULT_INJECTION_CONFIGFS")]
unsafe fn fault_ulong_attr_show(val: c_ulong, page: *mut c_char) -> isize { snprintf(page, PAGE_SIZE, b"%lu\n\0".as_ptr() as _, val) }
#[cfg(feature = "CONFIG_FAULT_INJECTION_CONFIGFS")]
unsafe fn fault_bool_attr_show(val: bool, page: *mut c_char) -> isize { snprintf(page, PAGE_SIZE, b"%u\n\0".as_ptr() as _, val as u32) }
#[cfg(feature = "CONFIG_FAULT_INJECTION_CONFIGFS")]
unsafe fn fault_atomic_t_attr_show(val: atomic_t, page: *mut c_char) -> isize { snprintf(page, PAGE_SIZE, b"%d\n\0".as_ptr() as _, atomic_read(&val)) }
#[cfg(feature = "CONFIG_FAULT_INJECTION_CONFIGFS")]
unsafe fn fault_uint_attr_store(val: *mut u32, page: *const c_char, count: usize) -> isize {
    let mut tmp = 0; let result = kstrtouint(page, 0, &mut tmp); if result < 0 { return result as isize; } *val = tmp; count as isize
}
#[cfg(feature = "CONFIG_FAULT_INJECTION_CONFIGFS")]
unsafe fn fault_ulong_attr_store(val: *mut c_ulong, page: *const c_char, count: usize) -> isize {
    let mut tmp = 0; let result = kstrtoul(page, 0, &mut tmp); if result < 0 { return result as isize; } *val = tmp; count as isize
}
#[cfg(feature = "CONFIG_FAULT_INJECTION_CONFIGFS")]
unsafe fn fault_bool_attr_store(val: *mut bool, page: *const c_char, count: usize) -> isize {
    let mut tmp = false; let result = kstrtobool(page, &mut tmp); if result < 0 { return result as isize; } *val = tmp; count as isize
}
#[cfg(feature = "CONFIG_FAULT_INJECTION_CONFIGFS")]
unsafe fn fault_atomic_t_attr_store(val: *mut atomic_t, page: *const c_char, count: usize) -> isize {
    let mut tmp = 0; let result = kstrtoint(page, 0, &mut tmp); if result < 0 { return result as isize; } atomic_set(val, tmp); count as isize
}

#[cfg(feature = "CONFIG_FAULT_INJECTION_STACKTRACE_FILTER")]
unsafe fn fault_stacktrace_depth_show(item: *mut config_item, page: *mut c_char) -> isize {
    fault_ulong_attr_show((*to_fault_config(item)).attr.stacktrace_depth, page)
}
#[cfg(feature = "CONFIG_FAULT_INJECTION_STACKTRACE_FILTER")]
unsafe fn fault_stacktrace_depth_store(item: *mut config_item, page: *const c_char, count: usize) -> isize {
    let mut tmp = 0; let result = kstrtoul(page, 0, &mut tmp); if result < 0 { return result as isize; }
    (*to_fault_config(item)).attr.stacktrace_depth = core::cmp::min(tmp, MAX_STACK_TRACE_DEPTH as c_ulong); count as isize
}
#[cfg(feature = "CONFIG_FAULT_INJECTION_STACKTRACE_FILTER")]
unsafe fn fault_xul_attr_show(val: c_ulong, page: *mut c_char) -> isize {
    snprintf(page, PAGE_SIZE, if core::mem::size_of::<c_ulong>() == 4 { b"0x%08lx\n\0".as_ptr() } else { b"0x%016lx\n\0".as_ptr() } as _, val)
}
#[cfg(feature = "CONFIG_FAULT_INJECTION_STACKTRACE_FILTER")]
unsafe fn fault_xul_attr_store(val: *mut c_ulong, page: *const c_char, count: usize) -> isize { fault_ulong_attr_store(val, page, count) }

#[cfg(feature = "CONFIG_FAULT_INJECTION_CONFIGFS")]
pub unsafe fn fault_config_init(config: *mut fault_config, name: *const c_char) {
    prandom_init_once(&mut FAULT_RND_STATE as *mut _ as *mut c_void);
    config_group_init_type_name(&mut (*config).group, name, &fault_config_type);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
