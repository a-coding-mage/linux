// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * linux/mm/process_vm_access.c
 *
 * Copyright (C) 2010-2011 Christopher Yeoh <cyeoh@au1.ibm.com>, IBM Corp.
 */

// Dependencies supplied by the surrounding kernel translation unit.

/**
 * process_vm_rw_pages - read/write pages from task specified
 * @pages: array of pointers to pages we want to copy
 * @offset: offset in page to start copying from/to
 * @len: number of bytes to copy
 * @iter: where to copy to/from locally
 * @vm_write: 0 means copy from, 1 means copy to
 * Returns 0 on success, error code otherwise
 */
unsafe fn process_vm_rw_pages(
    mut pages: *mut *mut page,
    mut offset: c_uint,
    mut len: usize,
    iter: *mut iov_iter,
    vm_write: c_int,
) -> c_int {
    /* Do the copy for each page */
    while len != 0 && iov_iter_count(iter) != 0 {
        let page = *pages;
        pages = pages.add(1);
        let mut copy = PAGE_SIZE - offset as usize;
        let copied: usize;

        if copy > len {
            copy = len;
        }

        if vm_write != 0 {
            copied = copy_page_from_iter(page, offset, copy, iter);
        } else {
            copied = copy_page_to_iter(page, offset, copy, iter);
        }

        len -= copied;
        if copied < copy && iov_iter_count(iter) != 0 {
            return -EFAULT;
        }
        offset = 0;
    }
    0
}

/* Maximum number of pages kmalloc'd to hold struct page's during copy */
const PVM_MAX_KMALLOC_PAGES: usize = 2;

/* Maximum number of pages that can be stored at a time */
const PVM_MAX_USER_PAGES: usize = PVM_MAX_KMALLOC_PAGES * PAGE_SIZE / core::mem::size_of::<*mut page>();

/**
 * process_vm_rw_single_vec - read/write pages from task specified
 * @addr: start memory address of target process
 * @len: size of area to copy to/from
 * @iter: where to copy to/from locally
 * @process_pages: struct pages area that can store at least
 *  nr_pages_to_copy struct page pointers
 * @mm: mm for task
 * @task: task to read/write from
 * @vm_write: 0 means copy from, 1 means copy to
 * Returns 0 on success or on failure error code
 */
unsafe fn process_vm_rw_single_vec(
    addr: c_ulong,
    mut len: c_ulong,
    iter: *mut iov_iter,
    process_pages: *mut *mut page,
    mm: *mut mm_struct,
    task: *mut task_struct,
    vm_write: c_int,
) -> isize {
    let mut pa = addr & PAGE_MASK;
    let mut start_offset = addr - pa;
    let mut nr_pages: c_ulong;
    let mut rc: isize = 0;
    let mut flags: c_uint = 0;

    /* Work out address and page range required */
    if len == 0 {
        return 0;
    }
    nr_pages = (addr + len - 1) / PAGE_SIZE as c_ulong - addr / PAGE_SIZE as c_ulong + 1;

    if vm_write != 0 {
        flags |= FOLL_WRITE;
    }

    while rc == 0 && nr_pages != 0 && iov_iter_count(iter) != 0 {
        let mut pinned_pages = core::cmp::min(nr_pages, PVM_MAX_USER_PAGES as c_ulong) as c_int;
        let mut locked = 1;
        let bytes: usize;

        /*
         * Get the pages we're interested in.  We must
         * access remotely because task/mm might not
         * current/current->mm
         */
        mmap_read_lock(mm);
        pinned_pages = pin_user_pages_remote(mm, pa, pinned_pages, flags, process_pages, &mut locked);
        if locked != 0 {
            mmap_read_unlock(mm);
        }
        if pinned_pages <= 0 {
            return -EFAULT as isize;
        }

        bytes = pinned_pages as usize * PAGE_SIZE - start_offset as usize;
        let bytes = if bytes > len as usize { len as usize } else { bytes };

        rc = process_vm_rw_pages(process_pages, start_offset as c_uint, bytes, iter, vm_write) as isize;
        len -= bytes as c_ulong;
        start_offset = 0;
        nr_pages -= pinned_pages as c_ulong;
        pa += pinned_pages as c_ulong * PAGE_SIZE as c_ulong;

        /* If vm_write is set, the pages need to be made dirty: */
        unpin_user_pages_dirty_lock(process_pages, pinned_pages, vm_write);
    }

    rc
}

/* Maximum number of entries for process pages array
   which lives on stack */
const PVM_MAX_PP_ARRAY_COUNT: usize = 16;

/**
 * process_vm_rw_core - core of reading/writing pages from task specified
 * @pid: PID of process to read/write from/to
 * @iter: where to copy to/from locally
 * @rvec: iovec array specifying where to copy to/from in the other process
 * @riovcnt: size of rvec array
 * @flags: currently unused
 * @vm_write: 0 if reading from other process, 1 if writing to other process
 *
 * Returns the number of bytes read/written or error code. May
 *  return less bytes than expected if an error occurs during the copying
 *  process.
 */
unsafe fn process_vm_rw_core(
    pid: pid_t,
    iter: *mut iov_iter,
    rvec: *const iovec,
    riovcnt: c_ulong,
    _flags: c_ulong,
    vm_write: c_int,
) -> isize {
    let task: *mut task_struct;
    let mut pp_stack: [*mut page; PVM_MAX_PP_ARRAY_COUNT] = [core::ptr::null_mut(); PVM_MAX_PP_ARRAY_COUNT];
    let mut process_pages = pp_stack.as_mut_ptr();
    let mm: *mut mm_struct;
    let mut i: c_ulong;
    let mut rc: isize = 0;
    let mut nr_pages: c_ulong = 0;
    let mut nr_pages_iov: c_ulong;
    let mut iov_len: isize;
    let total_len = iov_iter_count(iter);

    /*
     * Work out how many pages of struct pages we're going to need
     * when eventually calling get_user_pages
     */
    i = 0;
    while i < riovcnt {
        iov_len = (*rvec.add(i as usize)).iov_len as isize;
        if iov_len > 0 {
            nr_pages_iov = ((*rvec.add(i as usize)).iov_base as c_ulong + iov_len as c_ulong - 1)
                / PAGE_SIZE as c_ulong
                - (*rvec.add(i as usize)).iov_base as c_ulong / PAGE_SIZE as c_ulong
                + 1;
            nr_pages = core::cmp::max(nr_pages, nr_pages_iov);
        }
        i += 1;
    }

    if nr_pages == 0 {
        return 0;
    }

    if nr_pages as usize > PVM_MAX_PP_ARRAY_COUNT {
        /* For reliability don't try to kmalloc more than
           2 pages worth */
        process_pages = kmalloc(
            core::cmp::min(PVM_MAX_KMALLOC_PAGES * PAGE_SIZE, core::mem::size_of::<*mut page>() * nr_pages as usize),
            GFP_KERNEL,
        ) as *mut *mut page;

        if process_pages.is_null() {
            return -ENOMEM as isize;
        }
    }

    /* Get process information */
    task = find_get_task_by_vpid(pid);
    if task.is_null() {
        rc = -ESRCH as isize;
        if process_pages != pp_stack.as_mut_ptr() {
            kfree(process_pages as *mut core::ffi::c_void);
        }
        return rc;
    }

    mm = mm_access(task, PTRACE_MODE_ATTACH_REALCREDS);
    if IS_ERR(mm) {
        rc = PTR_ERR(mm);
        /*
         * Explicitly map EACCES to EPERM as EPERM is a more
         * appropriate error code for process_vw_readv/writev
         */
        if rc == -EACCES as isize {
            rc = -EPERM as isize;
        }
        put_task_struct(task);
        if process_pages != pp_stack.as_mut_ptr() {
            kfree(process_pages as *mut core::ffi::c_void);
        }
        return rc;
    }

    i = 0;
    while i < riovcnt && iov_iter_count(iter) != 0 && rc == 0 {
        rc = process_vm_rw_single_vec(
            (*rvec.add(i as usize)).iov_base as c_ulong,
            (*rvec.add(i as usize)).iov_len as c_ulong,
            iter, process_pages, mm, task, vm_write,
        );
        i += 1;
    }

    /* copied = space before - space after */
    let copied = total_len - iov_iter_count(iter);

    /* If we have managed to copy any data at all then
       we return the number of bytes copied. Otherwise
       we return the error code */
    if copied != 0 {
        rc = copied as isize;
    }

    mmput(mm);
    put_task_struct(task);

    if process_pages != pp_stack.as_mut_ptr() {
        kfree(process_pages as *mut core::ffi::c_void);
    }
    rc
}

/**
 * process_vm_rw - check iovecs before calling core routine
 * @pid: PID of process to read/write from/to
 * @lvec: iovec array specifying where to copy to/from locally
 * @liovcnt: size of lvec array
 * @rvec: iovec array specifying where to copy to/from in the other process
 * @riovcnt: size of rvec array
 * @flags: currently unused
 * @vm_write: 0 if reading from other process, 1 if writing to other process
 *
 * Returns the number of bytes read/written or error code. May
 *  return less bytes than expected if an error occurs during the copying
 *  process.
 */
unsafe fn process_vm_rw(
    pid: pid_t,
    lvec: *const iovec,
    liovcnt: c_ulong,
    rvec: *const iovec,
    riovcnt: c_ulong,
    flags: c_ulong,
    vm_write: c_int,
) -> isize {
    let mut iovstack_l: [iovec; UIO_FASTIOV] = core::mem::zeroed();
    let iovstack_r: [iovec; UIO_FASTIOV] = core::mem::zeroed();
    let mut iov_l = iovstack_l.as_mut_ptr();
    let mut iov_r: *mut iovec;
    let mut iter: iov_iter = core::mem::zeroed();
    let rc: isize;
    let dir = if vm_write != 0 { ITER_SOURCE } else { ITER_DEST };

    if flags != 0 {
        return -EINVAL as isize;
    }

    /* Check iovecs */
    rc = import_iovec(dir, lvec, liovcnt, UIO_FASTIOV, &mut iov_l, &mut iter);
    if rc < 0 {
        return rc;
    }
    if iov_iter_count(&mut iter) == 0 {
        kfree(iov_l as *mut core::ffi::c_void);
        return rc;
    }
    iov_r = iovec_from_user(rvec, riovcnt, UIO_FASTIOV, iovstack_r.as_ptr() as *mut iovec, in_compat_syscall());
    if IS_ERR(iov_r) {
        let err = PTR_ERR(iov_r);
        kfree(iov_l as *mut core::ffi::c_void);
        return err;
    }
    let result = process_vm_rw_core(pid, &mut iter, iov_r, riovcnt, flags, vm_write);
    if iov_r != iovstack_r.as_ptr() as *mut iovec {
        kfree(iov_r as *mut core::ffi::c_void);
    }
    kfree(iov_l as *mut core::ffi::c_void);
    result
}

unsafe fn process_vm_readv(
    pid: pid_t, lvec: *const iovec, liovcnt: c_ulong,
    rvec: *const iovec, riovcnt: c_ulong, flags: c_ulong,
) -> isize {
    process_vm_rw(pid, lvec, liovcnt, rvec, riovcnt, flags, 0)
}

unsafe fn process_vm_writev(
    pid: pid_t, lvec: *const iovec, liovcnt: c_ulong,
    rvec: *const iovec, riovcnt: c_ulong, flags: c_ulong,
) -> isize {
    process_vm_rw(pid, lvec, liovcnt, rvec, riovcnt, flags, 1)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
