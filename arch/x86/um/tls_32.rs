/*
 * Copyright (C) 2005 Paolo 'Blaisorblade' Giarrusso <blaisorblade@yahoo.it>
 * Licensed under the GPL
 */

// Dependencies are supplied by the surrounding kernel/UML translation unit.

static mut HOST_SUPPORTS_TLS: i32 = -1;
pub static mut host_gdt_entry_tls_min: i32 = 0;

unsafe fn do_set_thread_area(task: *mut task_struct, info: *mut user_desc) -> i32 {
    if (*info).entry_number < host_gdt_entry_tls_min
        || (*info).entry_number >= host_gdt_entry_tls_min + GDT_ENTRY_TLS_ENTRIES
    {
        return -EINVAL;
    }

    if using_seccomp {
        let idx = (*info).entry_number - host_gdt_entry_tls_min;
        let data = (*(*task).mm).context.id.stack as *mut stub_data;
        (*data).arch_data.tls[idx as usize] = *info;
        (*data).arch_data.sync |= BIT(idx);
        return 0;
    }

    let ret = os_set_thread_area(info, (*task).mm.context.id.pid);
    if ret != 0 {
        printk!(KERN_ERR "PTRACE_SET_THREAD_AREA failed, err = %d, index = %d\n", ret, (*info).entry_number);
    }
    ret
}

unsafe fn get_free_idx(task: *mut task_struct) -> i32 {
    let t = &(*task).thread;
    for idx in 0..GDT_ENTRY_TLS_ENTRIES {
        if !t.arch.tls_array[idx as usize].present {
            return idx + GDT_ENTRY_TLS_MIN;
        }
    }
    -ESRCH
}

unsafe fn clear_user_desc(info: *mut user_desc) {
    // Postcondition: LDT_empty(info) returns true.
    core::ptr::write_bytes(info as *mut u8, 0, core::mem::size_of::<user_desc>());
    // Check the LDT_empty or the i386 sys_get_thread_area code - we obtain indeed an empty user_desc.
    (*info).read_exec_only = 1;
    (*info).seg_not_present = 1;
}

const O_FORCE: i32 = 1;

unsafe fn load_TLS(flags: i32, to: *mut task_struct) -> i32 {
    let mut ret = 0;
    for idx in GDT_ENTRY_TLS_MIN..GDT_ENTRY_TLS_MAX {
        let curr = &mut (*to).thread.arch.tls_array[(idx - GDT_ENTRY_TLS_MIN) as usize];
        if !curr.present {
            if !curr.flushed {
                clear_user_desc(&mut curr.tls);
                curr.tls.entry_number = idx;
            } else {
                WARN_ON!(!LDT_empty(&curr.tls));
                continue;
            }
        }
        if flags & O_FORCE == 0 && curr.flushed {
            continue;
        }
        ret = do_set_thread_area(current, &mut curr.tls);
        if ret != 0 { break; }
        curr.flushed = 1;
    }
    ret
}

unsafe fn needs_TLS_update(task: *mut task_struct) -> i32 {
    for i in GDT_ENTRY_TLS_MIN..GDT_ENTRY_TLS_MAX {
        let curr = &(*task).thread.arch.tls_array[(i - GDT_ENTRY_TLS_MIN) as usize];
        if !curr.flushed { return 1; }
    }
    0
}

pub unsafe fn clear_flushed_tls(task: *mut task_struct) {
    for i in GDT_ENTRY_TLS_MIN..GDT_ENTRY_TLS_MAX {
        let curr = &mut (*task).thread.arch.tls_array[(i - GDT_ENTRY_TLS_MIN) as usize];
        if curr.present { curr.flushed = 0; }
    }
}

pub unsafe fn arch_switch_tls(to: *mut task_struct) -> i32 {
    if HOST_SUPPORTS_TLS == 0 { return 0; }
    if likely!(!(*to).mm.is_null()) { return load_TLS(O_FORCE, to); }
    0
}

unsafe fn set_tls_entry(task: *mut task_struct, info: *mut user_desc, idx: i32, flushed: i32) -> i32 {
    if idx < GDT_ENTRY_TLS_MIN || idx > GDT_ENTRY_TLS_MAX { return -EINVAL; }
    let entry = &mut (*task).thread.arch.tls_array[(idx - GDT_ENTRY_TLS_MIN) as usize];
    entry.tls = *info;
    entry.present = 1;
    entry.flushed = flushed;
    0
}

pub unsafe fn arch_set_tls(new: *mut task_struct, tls: c_ulong) -> i32 {
    let mut info: user_desc = core::mem::zeroed();
    if copy_from_user(&mut info, tls as *const _, core::mem::size_of::<user_desc>()) != 0 { return -EFAULT; }
    if LDT_empty(&info) { return -EINVAL; }
    let idx = info.entry_number;
    set_tls_entry(new, &mut info, idx, 0)
}

unsafe fn get_tls_entry(task: *mut task_struct, info: *mut user_desc, idx: i32) -> i32 {
    if idx < GDT_ENTRY_TLS_MIN || idx > GDT_ENTRY_TLS_MAX { return -EINVAL; }
    let entry = &(*task).thread.arch.tls_array[(idx - GDT_ENTRY_TLS_MIN) as usize];
    if entry.present { *info = entry.tls; } else { clear_user_desc(info); (*info).entry_number = idx; }
    if unlikely!(task == current && !entry.flushed) {
        printk!(KERN_ERR "get_tls_entry: task with pid %d got here without flushed TLS.", current.pid);
    }
    0
}

pub unsafe fn set_thread_area(user_desc: *mut user_desc) -> i32 {
    if HOST_SUPPORTS_TLS == 0 { return -ENOSYS; }
    let mut info: user_desc = core::mem::zeroed();
    if copy_from_user(&mut info, user_desc, core::mem::size_of::<user_desc>()) != 0 { return -EFAULT; }
    let mut idx = info.entry_number;
    if idx == -1 {
        idx = get_free_idx(current); if idx < 0 { return idx; }
        info.entry_number = idx;
        if put_user(idx, &mut (*user_desc).entry_number) != 0 { return -EFAULT; }
    }
    let ret = do_set_thread_area(current, &mut info);
    if ret != 0 { return ret; }
    set_tls_entry(current, &mut info, idx, 1)
}

pub unsafe fn ptrace_set_thread_area(child: *mut task_struct, idx: i32, user_desc: *mut user_desc) -> i32 {
    if HOST_SUPPORTS_TLS == 0 { return -EIO; }
    let mut info: user_desc = core::mem::zeroed();
    if copy_from_user(&mut info, user_desc, core::mem::size_of::<user_desc>()) != 0 { return -EFAULT; }
    set_tls_entry(child, &mut info, idx, 0)
}

pub unsafe fn get_thread_area(user_desc: *mut user_desc) -> i32 {
    if HOST_SUPPORTS_TLS == 0 { return -ENOSYS; }
    let mut info: user_desc = core::mem::zeroed();
    let idx = (*user_desc).entry_number;
    let mut ret = get_tls_entry(current, &mut info, idx);
    if ret < 0 { return ret; }
    if copy_to_user(user_desc, &info, core::mem::size_of::<user_desc>()) != 0 { ret = -EFAULT; }
    ret
}

pub unsafe fn ptrace_get_thread_area(child: *mut task_struct, idx: i32, user_desc: *mut user_desc) -> i32 {
    if HOST_SUPPORTS_TLS == 0 { return -EIO; }
    let mut info: user_desc = core::mem::zeroed();
    let mut ret = get_tls_entry(child, &mut info, idx);
    if ret < 0 { return ret; }
    if copy_to_user(user_desc, &info, core::mem::size_of::<user_desc>()) != 0 { ret = -EFAULT; }
    ret
}

unsafe fn __setup_host_supports_tls() -> i32 {
    check_host_supports_tls(&mut HOST_SUPPORTS_TLS, &mut host_gdt_entry_tls_min);
    if HOST_SUPPORTS_TLS != 0 {
        printk!(KERN_INFO "Host TLS support detected\n");
        printk!(KERN_INFO "Detected host type: ");
        match host_gdt_entry_tls_min {
            GDT_ENTRY_TLS_MIN_I386 => printk!(KERN_CONT "i386"),
            GDT_ENTRY_TLS_MIN_X86_64 => printk!(KERN_CONT "x86_64"),
            _ => {}
        }
        printk!(KERN_CONT " (GDT indexes %d to %d)\n", host_gdt_entry_tls_min, host_gdt_entry_tls_min + GDT_ENTRY_TLS_ENTRIES);
    } else {
        printk!(KERN_ERR "  Host TLS support NOT detected! TLS support inside UML will not work\n");
    }
    0
}

// __initcall(__setup_host_supports_tls);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
