// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Module proc support
 *
 * Copyright (C) 2008 Alexey Dobriyan
 */

// Linux kernel dependencies supplied by other files.

#[cfg(CONFIG_MODULE_UNLOAD)]
unsafe fn print_unload_info(m: *mut seq_file, mod_: *mut module) {
    let mut printed_something = 0;

    seq_printf(m, " %i ", module_refcount(mod_));

    /*
     * Always include a trailing , so userspace can differentiate
     * between this and the old multi-field proc format.
     */
    let mut use_ = (*mod_).source_list.next;
    while use_ != &mut (*mod_).source_list as *mut list_head {
        printed_something = 1;
        seq_printf(m, "%s,", (*(*use_ as *mut module_use).source).name);
        use_ = (*use_).next;
    }

    if !(*mod_).init.is_null() && (*mod_).exit.is_null() {
        printed_something = 1;
        seq_puts(m, "[permanent],");
    }

    if printed_something == 0 {
        seq_puts(m, "-");
    }
}

#[cfg(not(CONFIG_MODULE_UNLOAD))]
unsafe fn print_unload_info(m: *mut seq_file, _mod_: *mut module) {
    /* We don't know the usage count, or what modules are using. */
    seq_puts(m, " - -");
}

/* Called by the /proc file system to return a list of modules. */
unsafe fn m_start(m: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    mutex_lock(&mut module_mutex);
    seq_list_start(&mut modules, *pos)
}

unsafe fn m_next(
    _m: *mut seq_file,
    p: *mut core::ffi::c_void,
    pos: *mut loff_t,
) -> *mut core::ffi::c_void {
    seq_list_next(p, &mut modules, pos)
}

unsafe fn m_stop(_m: *mut seq_file, _p: *mut core::ffi::c_void) {
    mutex_unlock(&mut module_mutex);
}

unsafe fn module_total_size(mod_: *mut module) -> u32 {
    let mut size: u32 = 0;
    // for_each_mod_mem_type(type)
    let mut type_: usize = 0;
    while type_ < MOD_MEM_NUM_TYPES as usize {
        size = size.wrapping_add((*mod_).mem[type_].size);
        type_ += 1;
    }
    size
}

unsafe fn m_show(m: *mut seq_file, p: *mut core::ffi::c_void) -> i32 {
    let mod_: *mut module = container_of!(p, module, list);
    let mut buf = [0i8; MODULE_FLAGS_BUF_SIZE as usize];
    let value: *mut core::ffi::c_void;
    let size: u32;

    /* We always ignore unformed modules. */
    if (*mod_).state == MODULE_STATE_UNFORMED {
        return 0;
    }

    size = module_total_size(mod_);
    seq_printf(m, "%s %u", (*mod_).name, size);
    print_unload_info(m, mod_);

    /* Informative for users. */
    seq_printf(
        m,
        " %s",
        if (*mod_).state == MODULE_STATE_GOING {
            "Unloading"
        } else if (*mod_).state == MODULE_STATE_COMING {
            "Loading"
        } else {
            "Live"
        },
    );
    /* Used by oprofile and other similar tools. */
    value = if !(*m).private.is_null() {
        core::ptr::null_mut()
    } else {
        (*mod_).mem[MOD_TEXT].base
    };
    seq_printf(m, " 0x%px", value);

    /* Taints info */
    if (*mod_).taints != 0 {
        seq_printf(m, " %s", module_flags(mod_, buf.as_mut_ptr(), true));
    }

    seq_puts(m, "\n");
    0
}

/*
 * Format: modulename size refcount deps address
 *
 * Where refcount is a number or -, and deps is a comma-separated list
 * of depends or -.
 */
static mut modules_op: seq_operations = seq_operations {
    start: Some(m_start),
    next: Some(m_next),
    stop: Some(m_stop),
    show: Some(m_show),
};

/*
 * This also sets the "private" pointer to non-NULL if the
 * kernel pointers should be hidden (so you can just test
 * "m->private" to see if you should keep the values private).
 *
 * We use the same logic as for /proc/kallsyms.
 */
unsafe fn modules_open(inode: *mut inode, file: *mut file) -> i32 {
    let err = seq_open(file, &mut modules_op);

    if err == 0 {
        let m: *mut seq_file = (*file).private_data as *mut seq_file;
        (*m).private = if kallsyms_show_value((*file).f_cred) {
            core::ptr::null_mut()
        } else {
            8usize as *mut core::ffi::c_void
        };
    }

    err
}

static modules_proc_ops: proc_ops = proc_ops {
    proc_flags: PROC_ENTRY_PERMANENT,
    proc_open: Some(modules_open),
    proc_read: Some(seq_read),
    proc_lseek: Some(seq_lseek),
    proc_release: Some(seq_release),
};

unsafe fn proc_modules_init() -> i32 {
    proc_create("modules", 0, core::ptr::null_mut(), &modules_proc_ops);
    0
}

// module_init(proc_modules_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
