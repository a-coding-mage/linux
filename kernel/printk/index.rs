// SPDX-License-Identifier: GPL-2.0
/*
 * Userspace indexing of printk formats
 */

// C dependencies supplied by the kernel and by internal.h are intentionally
// referenced here rather than reimplemented in this translation unit.

extern "C" {
    static mut __start_printk_index: *mut *mut pi_entry;
    static mut __stop_printk_index: *mut *mut pi_entry;
}

/* The base dir for module formats, typically debugfs/printk/index/ */
static mut dfs_index: *mut dentry = core::ptr::null_mut();

unsafe fn pi_get_entry(mod_: *const module, pos: loff_t) -> *mut pi_entry {
    let entries: *mut *mut pi_entry;
    let nr_entries: usize;

    // CONFIG_MODULES is a build-time condition in the C source.
    #[cfg(CONFIG_MODULES)]
    if !mod_.is_null() {
        entries = (*((mod_) as *mut module)).printk_index_start;
        nr_entries = (*((mod_) as *mut module)).printk_index_size as usize;
    } else
    #[cfg(CONFIG_MODULES)]
    {
        /* vmlinux, comes from linker symbols */
        entries = __start_printk_index;
        nr_entries = __stop_printk_index.offset_from(__start_printk_index) as usize;
    }

    #[cfg(not(CONFIG_MODULES))]
    {
        /* vmlinux, comes from linker symbols */
        entries = __start_printk_index;
        nr_entries = __stop_printk_index.offset_from(__start_printk_index) as usize;
    }

    if pos >= nr_entries as loff_t {
        return core::ptr::null_mut();
    }

    *entries.add(pos as usize)
}

unsafe fn pi_next(s: *mut seq_file, _v: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void {
    let mod_ = (*(*s).file).f_inode.i_private as *const module;
    let entry = pi_get_entry(mod_, *pos);

    *pos += 1;

    entry as *mut core::ffi::c_void
}

unsafe fn pi_start(s: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    /*
     * Make show() print the header line. Do not update *pos because
     * pi_next() still has to return the entry at index 0 later.
     */
    if *pos == 0 {
        return SEQ_START_TOKEN;
    }

    pi_next(s, core::ptr::null_mut(), pos)
}

/*
 * We need both ESCAPE_ANY and explicit characters from ESCAPE_SPECIAL in @only
 * because otherwise ESCAPE_NAP will cause double quotes and backslashes to be
 * ignored for quoting.
 */
macro_rules! seq_escape_printf_format {
    ($s:expr, $src:expr) => {
        seq_escape_str($s, $src, ESCAPE_ANY | ESCAPE_NAP | ESCAPE_APPEND, "\\\"")
    };
}

unsafe fn pi_show(s: *mut seq_file, v: *mut core::ffi::c_void) -> c_int {
    let entry = v as *const pi_entry;
    let mut level = LOGLEVEL_DEFAULT;
    let mut flags: printk_info_flags = 0;
    let mut prefix_len: u16 = 0;

    if v == SEQ_START_TOKEN {
        seq_puts(s, "# <level/flags> filename:line function \"format\"\n");
        return 0;
    }

    if (*entry).fmt.is_null() {
        return 0;
    }

    if !(*entry).level.is_null() {
        printk_parse_prefix((*entry).level, &mut level, &mut flags);
    } else {
        prefix_len = printk_parse_prefix((*entry).fmt, &mut level, &mut flags);
    }

    if flags & LOG_CONT != 0 {
        /*
         * LOGLEVEL_DEFAULT here means "use the same level as the
         * message we're continuing from", not the default message
         * loglevel, so don't display it as such.
         */
        if level == LOGLEVEL_DEFAULT {
            seq_puts(s, "<c>");
        } else {
            seq_printf(s, "<%d,c>", level);
        }
    } else {
        seq_printf(s, "<%d>", level);
    }

    seq_printf(s, " {}:{} {} \"", (*entry).file, (*entry).line, (*entry).func);
    if !(*entry).subsys_fmt_prefix.is_null() {
        seq_escape_printf_format!(s, (*entry).subsys_fmt_prefix);
    }
    seq_escape_printf_format!(s, (*entry).fmt.add(prefix_len as usize));
    seq_puts(s, "\"\n");

    0
}

unsafe fn pi_stop(_p: *mut seq_file, _v: *mut core::ffi::c_void) {}

static dfs_index_sops: seq_operations = seq_operations {
    start: Some(pi_start),
    next: Some(pi_next),
    show: Some(pi_show),
    stop: Some(pi_stop),
};

// DEFINE_SEQ_ATTRIBUTE(dfs_index);

#[cfg(CONFIG_MODULES)]
unsafe fn pi_get_module_name(mod_: *mut module) -> *const c_char {
    if !mod_.is_null() { (*mod_).name } else { c"vmlinux".as_ptr() }
}

#[cfg(not(CONFIG_MODULES))]
unsafe fn pi_get_module_name(_mod_: *mut module) -> *const c_char {
    c"vmlinux".as_ptr()
}

unsafe fn pi_create_file(mod_: *mut module) {
    debugfs_create_file(pi_get_module_name(mod_), 0o444, dfs_index, mod_, &dfs_index_fops);
}

#[cfg(CONFIG_MODULES)]
unsafe fn pi_remove_file(mod_: *mut module) {
    debugfs_lookup_and_remove(pi_get_module_name(mod_), dfs_index);
}

#[cfg(CONFIG_MODULES)]
unsafe fn pi_module_notify(_nb: *mut notifier_block, op: c_ulong, data: *mut core::ffi::c_void) -> c_int {
    let mod_ = data as *mut module;

    match op {
        MODULE_STATE_COMING => pi_create_file(mod_),
        MODULE_STATE_GOING => pi_remove_file(mod_),
        _ => { /* we don't care about other module states */ }
    }

    NOTIFY_OK
}

#[cfg(CONFIG_MODULES)]
static mut module_printk_fmts_nb: notifier_block = notifier_block {
    notifier_call: Some(pi_module_notify),
};

#[cfg(CONFIG_MODULES)]
unsafe fn pi_setup_module_notifier() {
    register_module_notifier(&mut module_printk_fmts_nb);
}

#[cfg(not(CONFIG_MODULES))]
unsafe fn pi_setup_module_notifier() {}

unsafe fn pi_init() -> c_int {
    let dfs_root = debugfs_create_dir(c"printk".as_ptr(), core::ptr::null_mut());

    dfs_index = debugfs_create_dir(c"index".as_ptr(), dfs_root);
    pi_setup_module_notifier();
    pi_create_file(core::ptr::null_mut());

    0
}

/* debugfs comes up on core and must be initialised first */
// postcore_initcall(pi_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
