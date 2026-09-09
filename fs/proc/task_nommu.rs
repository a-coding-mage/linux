// SPDX-License-Identifier: GPL-2.0

// Translated from task_nommu.c. Kernel types, constants, and functions are
// supplied by the surrounding kernel bindings.

pub unsafe fn task_mem(m: *mut seq_file, mm: *mut mm_struct) {
    let mut vmi = VMA_ITERATOR!(mm, 0);
    let mut bytes: c_ulong = 0;
    let mut sbytes: c_ulong = 0;
    let mut slack: c_ulong = 0;
    let mut vma: *mut vm_area_struct;
    let mut region: *mut vm_region;
    let mut size: c_ulong;

    mmap_read_lock(mm);
    for_each_vma!(vmi, vma) {
        bytes = bytes.wrapping_add(kobjsize(vma));
        region = (*vma).vm_region;
        if !region.is_null() {
            size = kobjsize(region);
            size = size.wrapping_add((*region).vm_end.wrapping_sub((*region).vm_start));
        } else {
            size = (*vma).vm_end.wrapping_sub((*vma).vm_start);
        }

        if atomic_read(&(*mm).mm_count) > 1 || is_nommu_shared_mapping((*vma).vm_flags) {
            sbytes = sbytes.wrapping_add(size);
        } else {
            bytes = bytes.wrapping_add(size);
            if !region.is_null() {
                slack = (*region).vm_end.wrapping_sub((*vma).vm_end);
            }
        }
    }

    if atomic_read(&(*mm).mm_count) > 1 {
        sbytes = sbytes.wrapping_add(kobjsize(mm));
    } else {
        bytes = bytes.wrapping_add(kobjsize(mm));
    }

    if !(*current).fs.is_null() && (*(*current).fs).users > 1 {
        sbytes = sbytes.wrapping_add(kobjsize((*current).fs));
    } else {
        bytes = bytes.wrapping_add(kobjsize((*current).fs));
    }

    if !(*current).files.is_null() && atomic_read(&(*(*current).files).count) > 1 {
        sbytes = sbytes.wrapping_add(kobjsize((*current).files));
    } else {
        bytes = bytes.wrapping_add(kobjsize((*current).files));
    }

    if !(*current).sighand.is_null() && refcount_read(&(*(*current).sighand).count) > 1 {
        sbytes = sbytes.wrapping_add(kobjsize((*current).sighand));
    } else {
        bytes = bytes.wrapping_add(kobjsize((*current).sighand));
    }

    bytes = bytes.wrapping_add(kobjsize(current));
    mmap_read_unlock(mm);
    seq_printf(m, "Mem:\t%8lu bytes\nSlack:\t%8lu bytes\nShared:\t%8lu bytes\n", bytes, slack, sbytes);
}

pub unsafe fn task_vsize(mm: *mut mm_struct) -> c_ulong {
    let mut vmi = VMA_ITERATOR!(mm, 0);
    let mut vma: *mut vm_area_struct;
    let mut vsize: c_ulong = 0;
    mmap_read_lock(mm);
    for_each_vma!(vmi, vma) {
        vsize = vsize.wrapping_add((*vma).vm_end.wrapping_sub((*vma).vm_start));
    }
    mmap_read_unlock(mm);
    vsize
}

pub unsafe fn task_statm(mm: *mut mm_struct, shared: *mut c_ulong, text: *mut c_ulong,
                         data: *mut c_ulong, resident: *mut c_ulong) -> c_ulong {
    let mut vmi = VMA_ITERATOR!(mm, 0);
    let mut vma: *mut vm_area_struct;
    let mut size = kobjsize(mm);
    mmap_read_lock(mm);
    for_each_vma!(vmi, vma) {
        size = size.wrapping_add(kobjsize(vma));
        let region = (*vma).vm_region;
        if !region.is_null() {
            size = size.wrapping_add(kobjsize(region));
            size = size.wrapping_add((*region).vm_end.wrapping_sub((*region).vm_start));
        }
    }
    *text = (PAGE_ALIGN!((*mm).end_code).wrapping_sub((*mm).start_code & PAGE_MASK!())) >> PAGE_SHIFT;
    *data = (PAGE_ALIGN!((*mm).start_stack).wrapping_sub((*mm).start_data & PAGE_MASK!())) >> PAGE_SHIFT;
    mmap_read_unlock(mm);
    size >>= PAGE_SHIFT;
    size = size.wrapping_add(*text).wrapping_add(*data);
    *resident = size;
    size
}

unsafe fn nommu_vma_show(m: *mut seq_file, vma: *mut vm_area_struct) -> c_int {
    let mm = (*vma).vm_mm;
    let mut ino: c_ulong = 0;
    let mut dev: dev_t = 0;
    let mut pgoff: c_ulonglong = 0;
    let flags = (*vma).vm_flags;
    let file = (*vma).vm_file;
    if !file.is_null() {
        let inode = file_inode((*vma).vm_file);
        dev = (*(*inode).i_sb).s_dev;
        ino = (*inode).i_ino;
        pgoff = ((*vma).vm_pgoff as loff_t).wrapping_shl(PAGE_SHIFT) as c_ulonglong;
    }
    seq_setwidth(m, 25 + core::mem::size_of::<*mut c_void>() * 6 - 1);
    seq_printf(m, "%08lx-%08lx %c%c%c%c %08llx %02x:%02x %lu ",
        (*vma).vm_start, (*vma).vm_end,
        if flags & VM_READ != 0 { 'r' } else { '-' },
        if flags & VM_WRITE != 0 { 'w' } else { '-' },
        if flags & VM_EXEC != 0 { 'x' } else { '-' },
        if flags & VM_MAYSHARE != 0 { if flags & VM_SHARED != 0 { 'S' } else { 's' } } else { 'p' },
        pgoff, MAJOR!(dev), MINOR!(dev), ino);
    if !file.is_null() {
        seq_pad(m, ' ');
        seq_path(m, file_user_path(file), "");
    } else if !mm.is_null() && vma_is_initial_stack(vma) {
        seq_pad(m, ' ');
        seq_puts(m, "[stack]");
    }
    seq_putc(m, '\n');
    0
}

unsafe fn show_map(m: *mut seq_file, p: *mut c_void) -> c_int { nommu_vma_show(m, p as *mut vm_area_struct) }

unsafe fn proc_get_vma(priv_: *mut proc_maps_private, ppos: *mut loff_t) -> *mut vm_area_struct {
    let vma = vma_next(&mut (*priv_).iter);
    if !vma.is_null() { *ppos = (*vma).vm_start; } else { *ppos = -1isize as loff_t; }
    vma
}

unsafe fn m_start(m: *mut seq_file, ppos: *mut loff_t) -> *mut c_void {
    let priv_ = (*m).private as *mut proc_maps_private;
    let last_addr = *ppos;
    if last_addr == -1isize as loff_t { return core::ptr::null_mut(); }
    (*priv_).task = get_proc_task((*priv_).inode);
    if (*priv_).task.is_null() { return ERR_PTR!(-ESRCH); }
    let mm = (*priv_).lock_ctx.mm;
    if mm.is_null() || mmget_not_zero(mm) == 0 {
        put_task_struct((*priv_).task); (*priv_).task = core::ptr::null_mut(); return core::ptr::null_mut();
    }
    if mmap_read_lock_killable(mm) != 0 {
        mmput(mm); put_task_struct((*priv_).task); (*priv_).task = core::ptr::null_mut(); return ERR_PTR!(-EINTR);
    }
    vma_iter_init(&mut (*priv_).iter, mm, last_addr);
    proc_get_vma(priv_, ppos) as *mut c_void
}

unsafe fn m_stop(m: *mut seq_file, _v: *mut c_void) {
    let priv_ = (*m).private as *mut proc_maps_private;
    let mm = (*priv_).lock_ctx.mm;
    if (*priv_).task.is_null() { return; }
    mmap_read_unlock(mm); mmput(mm); put_task_struct((*priv_).task); (*priv_).task = core::ptr::null_mut();
}

unsafe fn m_next(m: *mut seq_file, _p: *mut c_void, ppos: *mut loff_t) -> *mut c_void {
    proc_get_vma((*m).private as *mut proc_maps_private, ppos) as *mut c_void
}

static proc_pid_maps_ops: seq_operations = seq_operations { start: Some(m_start), next: Some(m_next), stop: Some(m_stop), show: Some(show_map) };

unsafe fn maps_open(inode: *mut inode, file: *mut file, ops: *const seq_operations) -> c_int {
    let priv_ = __seq_open_private(file, ops, core::mem::size_of::<proc_maps_private>()) as *mut proc_maps_private;
    if priv_.is_null() { return -ENOMEM; }
    (*priv_).inode = inode;
    (*priv_).lock_ctx.mm = proc_mem_open(inode, PTRACE_MODE_READ);
    if IS_ERR_OR_NULL!((*priv_).lock_ctx.mm) {
        let err = if !(*priv_).lock_ctx.mm.is_null() { PTR_ERR!((*priv_).lock_ctx.mm) } else { -ESRCH };
        seq_release_private(inode, file); return err;
    }
    0
}

unsafe fn map_release(inode: *mut inode, file: *mut file) -> c_int {
    let seq = (*file).private_data as *mut seq_file;
    let priv_ = (*seq).private as *mut proc_maps_private;
    if !(*priv_).lock_ctx.mm.is_null() { mmdrop((*priv_).lock_ctx.mm); }
    seq_release_private(inode, file)
}

unsafe fn pid_maps_open(inode: *mut inode, file: *mut file) -> c_int { maps_open(inode, file, &proc_pid_maps_ops) }

pub static proc_pid_maps_operations: file_operations = file_operations {
    open: Some(pid_maps_open), read: Some(seq_read), llseek: Some(seq_lseek), release: Some(map_release),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
