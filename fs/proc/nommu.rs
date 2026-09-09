// SPDX-License-Identifier: GPL-2.0-or-later
/* nommu.c: mmu-less memory info files
 *
 * Copyright (C) 2004 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Linux kernel dependencies supplied by other translation units.

/* display a single region to a sequenced file */
unsafe fn nommu_region_show(m: *mut seq_file, region: *mut vm_region) -> c_int {
    let mut ino: c_ulong = 0;
    let mut file: *mut file = core::ptr::null_mut();
    let mut dev: dev_t = 0;
    let flags: c_int;

    flags = (*region).vm_flags;
    file = (*region).vm_file;

    if !file.is_null() {
        let inode: *mut inode = file_inode((*region).vm_file);
        dev = (*(*inode).i_sb).s_dev;
        ino = (*inode).i_ino;
    }

    seq_setwidth(m, 25 + core::mem::size_of::<*mut c_void>() * 6 - 1);
    seq_printf(
        m,
        c"%08lx-%08lx %c%c%c%c %08llx %02x:%02x %lu ",
        (*region).vm_start,
        (*region).vm_end,
        if flags & VM_READ != 0 { b'r' as c_char } else { b'-' as c_char },
        if flags & VM_WRITE != 0 { b'w' as c_char } else { b'-' as c_char },
        if flags & VM_EXEC != 0 { b'x' as c_char } else { b'-' as c_char },
        if flags & VM_MAYSHARE != 0 {
            if flags & VM_SHARED != 0 { b'S' as c_char } else { b's' as c_char }
        } else { b'p' as c_char },
        ((*region).vm_pgoff as loff_t) << PAGE_SHIFT,
        MAJOR(dev),
        MINOR(dev),
        ino,
    );

    if !file.is_null() {
        seq_pad(m, b' ' as c_char);
        seq_path(m, file_user_path(file), c"");
    }

    seq_putc(m, b'\n' as c_char);
    0
}

/* display a list of all the REGIONs the kernel knows about
 * - nommu kernels have a single flat list
 */
unsafe fn nommu_region_list_show(m: *mut seq_file, _p: *mut c_void) -> c_int {
    let p: *mut rb_node = _p as *mut rb_node;
    nommu_region_show(m, rb_entry(p, core::mem::size_of::<vm_region>(), vm_rb))
}

unsafe fn nommu_region_list_start(_m: *mut seq_file, _pos: *mut loff_t) -> *mut c_void {
    let mut p: *mut rb_node;
    let mut pos: loff_t = *_pos;

    down_read(&mut nommu_region_sem);

    p = rb_first(&mut nommu_region_tree);
    while !p.is_null() {
        if pos == 0 { return p as *mut c_void; }
        pos -= 1;
        p = rb_next(p);
    }
    core::ptr::null_mut()
}

unsafe fn nommu_region_list_stop(_m: *mut seq_file, _v: *mut c_void) {
    up_read(&mut nommu_region_sem);
}

unsafe fn nommu_region_list_next(_m: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    *pos += 1;
    rb_next(v as *mut rb_node) as *mut c_void
}

static proc_nommu_region_list_seqop: seq_operations = seq_operations {
    start: Some(nommu_region_list_start),
    next: Some(nommu_region_list_next),
    stop: Some(nommu_region_list_stop),
    show: Some(nommu_region_list_show),
};

unsafe fn proc_nommu_init() -> c_int {
    proc_create_seq(c"maps", S_IRUGO, core::ptr::null_mut(), &proc_nommu_region_list_seqop);
    0
}

// fs_initcall(proc_nommu_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
