// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel translation:
// linux/seq_file.h, linux/kallsyms.h, linux/module.h, linux/ftrace.h,
// linux/fs.h, and trace_output.h.

#[repr(C)]
struct recursed_functions {
    ip: c_ulong,
    parent_ip: c_ulong,
}

static mut recursed_functions: [recursed_functions; CONFIG_FTRACE_RECORD_RECURSION_SIZE] =
    [recursed_functions { ip: 0, parent_ip: 0 }; CONFIG_FTRACE_RECORD_RECURSION_SIZE];
static mut nr_records: atomic_t = atomic_t { counter: 0 };

/*
 * Cache the last found function. Yes, updates to this is racey, but
 * so is memory cache ;-)
 */
static mut cached_function: c_ulong = 0;

#[no_mangle]
pub unsafe extern "C" fn ftrace_record_recursion(ip: c_ulong, parent_ip: c_ulong) {
    let mut index: c_int = 0;
    let mut i: c_int;
    let mut old: c_ulong;

    'again: loop {
        /* First check the last one recorded */
        if ip == cached_function {
            return;
        }

        i = atomic_read(&nr_records);
        /* nr_records is -1 when clearing records */
        smp_mb__after_atomic();
        if i < 0 {
            return;
        }

        /* See the C source for the concurrent-writer rationale. */
        if index < i {
            index = i;
        }
        if index >= CONFIG_FTRACE_RECORD_RECURSION_SIZE {
            return;
        }

        i = index - 1;
        while i >= 0 {
            if recursed_functions[i as usize].ip == ip {
                cached_function = ip;
                return;
            }
            i -= 1;
        }

        cached_function = ip;

        /* Add to the current location before incrementing the count. */
        old = cmpxchg(&mut recursed_functions[index as usize].ip, 0, ip);
        if old != 0 {
            /* Did something else already add this for us? */
            if old == ip {
                return;
            }
            /* Try the next location. */
            index += 1;
            continue 'again;
        }

        recursed_functions[index as usize].parent_ip = parent_ip;

        i = atomic_read(&nr_records);
        smp_mb__after_atomic();
        if i < 0 {
            cmpxchg(&mut recursed_functions[index as usize].ip, ip, 0);
        } else if i <= index {
            atomic_cmpxchg(&mut nr_records, i, index + 1);
        }
        return;
    }
}

static mut recursed_function_lock: mutex = mutex::new();
static mut tseq: *mut trace_seq = core::ptr::null_mut();

unsafe extern "C" fn recursed_function_seq_start(m: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    let mut ret: *mut core::ffi::c_void = core::ptr::null_mut();
    let index: c_int;

    mutex_lock(&mut recursed_function_lock);
    index = atomic_read(&nr_records);
    if *pos < index as loff_t {
        ret = recursed_functions.as_mut_ptr().add(*pos as usize) as *mut core::ffi::c_void;
    }

    tseq = kzalloc_obj::<trace_seq>();
    if tseq.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    trace_seq_init(tseq);
    ret
}

unsafe extern "C" fn recursed_function_seq_next(m: *mut seq_file, v: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void {
    let index = atomic_read(&nr_records);
    *pos += 1;
    if *pos < index as loff_t {
        recursed_functions.as_mut_ptr().add(*pos as usize) as *mut core::ffi::c_void
    } else {
        core::ptr::null_mut()
    }
}

unsafe extern "C" fn recursed_function_seq_stop(m: *mut seq_file, v: *mut core::ffi::c_void) {
    kfree(tseq);
    mutex_unlock(&mut recursed_function_lock);
}

unsafe extern "C" fn recursed_function_seq_show(m: *mut seq_file, v: *mut core::ffi::c_void) -> c_int {
    let record = v as *mut recursed_functions;
    let mut ret: c_int = 0;
    if !record.is_null() {
        trace_seq_print_sym(tseq, (*record).parent_ip, true);
        trace_seq_puts(tseq, ":\t");
        trace_seq_print_sym(tseq, (*record).ip, true);
        trace_seq_putc(tseq, b'\n' as c_char);
        ret = trace_print_seq(m, tseq);
    }
    ret
}

static recursed_function_seq_ops: seq_operations = seq_operations {
    start: Some(recursed_function_seq_start),
    next: Some(recursed_function_seq_next),
    stop: Some(recursed_function_seq_stop),
    show: Some(recursed_function_seq_show),
};

unsafe extern "C" fn recursed_function_open(inode: *mut inode, file: *mut file) -> c_int {
    let _guard = guard_mutex(&mut recursed_function_lock);

    /* If this file was opened for write, then erase contents */
    if ((*file).f_mode & FMODE_WRITE) != 0 && ((*file).f_flags & O_TRUNC) != 0 {
        /* disable updating records */
        atomic_set(&mut nr_records, -1);
        smp_mb__after_atomic();
        core::ptr::write_bytes(recursed_functions.as_mut_ptr(), 0, CONFIG_FTRACE_RECORD_RECURSION_SIZE);
        smp_wmb();
        /* enable them again */
        atomic_set(&mut nr_records, 0);
    }
    if ((*file).f_mode & FMODE_READ) != 0 {
        return seq_open(file, &recursed_function_seq_ops);
    }
    0
}

unsafe extern "C" fn recursed_function_write(file: *mut file, buffer: *const c_char, count: usize, ppos: *mut loff_t) -> isize {
    count as isize
}

unsafe extern "C" fn recursed_function_release(inode: *mut inode, file: *mut file) -> c_int {
    if ((*file).f_mode & FMODE_READ) != 0 {
        seq_release(inode, file);
    }
    0
}

static recursed_functions_fops: file_operations = file_operations {
    open: Some(recursed_function_open),
    write: Some(recursed_function_write),
    read: Some(seq_read),
    llseek: Some(seq_lseek),
    release: Some(recursed_function_release),
};

unsafe extern "C" fn create_recursed_functions() -> c_int {
    trace_create_file("recursed_functions", TRACE_MODE_WRITE, core::ptr::null_mut(), core::ptr::null_mut(), &recursed_functions_fops);
    0
}

fs_initcall!(create_recursed_functions);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
