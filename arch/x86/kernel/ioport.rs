// SPDX-License-Identifier: GPL-2.0
/*
 * This contains the io-permission bitmap code - written by obz, with changes
 * by Linus. 32/64 bits code unification by Miguel Botón.
 */

// Dependencies supplied by the surrounding kernel translation.

#[cfg(CONFIG_X86_IOPL_IOPERM)]
static mut io_bitmap_sequence: atomic64_t = atomic64_t::new(0);

#[cfg(CONFIG_X86_IOPL_IOPERM)]
pub unsafe fn io_bitmap_share(tsk: *mut task_struct) {
    /* Can be NULL when current->thread.iopl_emul == 3 */
    if !(*current).thread.io_bitmap.is_null() {
        /*
         * Take a refcount on current's bitmap. It can be used by
         * both tasks as long as none of them changes the bitmap.
         */
        refcount_inc(&mut (*(*current).thread.io_bitmap).refcnt);
        (*tsk).thread.io_bitmap = (*current).thread.io_bitmap;
    }
    set_tsk_thread_flag(tsk, TIF_IO_BITMAP);
}

#[cfg(CONFIG_X86_IOPL_IOPERM)]
unsafe fn task_update_io_bitmap() {
    let tsk: *mut task_struct = current;
    let t: *mut thread_struct = &mut (*tsk).thread;

    if (*t).iopl_emul == 3 || !(*t).io_bitmap.is_null() {
        /* TSS update is handled on exit to user space */
        set_tsk_thread_flag(tsk, TIF_IO_BITMAP);
    } else {
        clear_tsk_thread_flag(tsk, TIF_IO_BITMAP);
        /* Invalidate TSS */
        preempt_disable();
        tss_update_io_bitmap();
        preempt_enable();
    }
}

#[cfg(CONFIG_X86_IOPL_IOPERM)]
pub unsafe fn io_bitmap_exit(tsk: *mut task_struct) {
    let iobm: *mut io_bitmap = (*tsk).thread.io_bitmap;

    (*tsk).thread.io_bitmap = core::ptr::null_mut();
    /*
     * Don't touch the TSS when invoked on a failed fork(). TSS
     * reflects the state of @current and not the state of @tsk.
     */
    if tsk == current {
        task_update_io_bitmap();
    }
    if !iobm.is_null() && refcount_dec_and_test(&mut (*iobm).refcnt) {
        kfree(iobm as *mut core::ffi::c_void);
    }
}

/*
 * This changes the io permissions bitmap in the current task.
 */
#[cfg(CONFIG_X86_IOPL_IOPERM)]
pub unsafe fn ksys_ioperm(from: usize, num: usize, turn_on: i32) -> isize {
    let t: *mut thread_struct = &mut (*current).thread;
    let mut i: u32;
    let mut max_long: u32;
    let mut iobm: *mut io_bitmap = (*t).io_bitmap;

    if from.wrapping_add(num) <= from || from.wrapping_add(num) > IO_BITMAP_BITS {
        return -EINVAL as isize;
    }
    if turn_on != 0 && (!capable(CAP_SYS_RAWIO) || security_locked_down(LOCKDOWN_IOPORT) != 0) {
        return -EPERM as isize;
    }

    /*
     * If it's the first ioperm() call in this thread's lifetime, set the
     * IO bitmap up. ioperm() is much less timing critical than clone(),
     * this is why we delay this operation until now:
     */
    if iobm.is_null() {
        /* No point to allocate a bitmap just to clear permissions */
        if turn_on == 0 {
            return 0;
        }
        iobm = kmalloc_obj::<io_bitmap>();
        if iobm.is_null() {
            return -ENOMEM as isize;
        }

        core::ptr::write_bytes((*iobm).bitmap.as_mut_ptr(), 0xff, (*iobm).bitmap.len());
        refcount_set(&mut (*iobm).refcnt, 1);
    }

    /*
     * If the bitmap is not shared, then nothing can take a refcount as
     * current can obviously not fork at the same time. If it's shared
     * duplicate it and drop the refcount on the original one.
     */
    if refcount_read(&(*iobm).refcnt) > 1 {
        let new_iobm = kmemdup(iobm as *const core::ffi::c_void, core::mem::size_of::<io_bitmap>(), GFP_KERNEL) as *mut io_bitmap;
        if new_iobm.is_null() {
            return -ENOMEM as isize;
        }
        iobm = new_iobm;
        refcount_set(&mut (*iobm).refcnt, 1);
        io_bitmap_exit(current);
    }

    /*
     * Store the bitmap pointer (might be the same if the task already
     * head one). Must be done here so freeing the bitmap when all
     * permissions are dropped has the pointer set up.
     */
    (*t).io_bitmap = iobm;
    /* Mark it active for context switching and exit to user mode */
    set_thread_flag(TIF_IO_BITMAP);

    /*
     * Update the tasks bitmap. The update of the TSS bitmap happens on
     * exit to user mode. So this needs no protection.
     */
    if turn_on != 0 {
        bitmap_clear((*iobm).bitmap.as_mut_ptr(), from, num);
    } else {
        bitmap_set((*iobm).bitmap.as_mut_ptr(), from, num);
    }

    /*
     * Search for a (possibly new) maximum. This is simple and stupid,
     * to keep it obviously correct:
     */
    max_long = u32::MAX;
    i = 0;
    while i < IO_BITMAP_LONGS {
        if (*iobm).bitmap[i as usize] != !0usize {
            max_long = i;
        }
        i += 1;
    }
    /* All permissions dropped? */
    if max_long == u32::MAX {
        io_bitmap_exit(current);
        return 0;
    }

    (*iobm).max = ((max_long + 1) as usize) * core::mem::size_of::<usize>();

    /*
     * Update the sequence number to force a TSS update on return to
     * user mode.
     */
    (*iobm).sequence = atomic64_inc_return(&mut io_bitmap_sequence);

    0
}

#[cfg(CONFIG_X86_IOPL_IOPERM)]
pub unsafe fn ioperm(from: usize, num: usize, turn_on: i32) -> isize {
    ksys_ioperm(from, num, turn_on)
}

#[cfg(CONFIG_X86_IOPL_IOPERM)]
pub unsafe fn iopl(level: u32) -> isize {
    let t: *mut thread_struct = &mut (*current).thread;
    let old: u32;

    if level > 3 {
        return -EINVAL as isize;
    }

    old = (*t).iopl_emul;

    /* No point in going further if nothing changes */
    if level == old {
        return 0;
    }

    /* Trying to gain more privileges? */
    if level > old {
        if !capable(CAP_SYS_RAWIO) || security_locked_down(LOCKDOWN_IOPORT) != 0 {
            return -EPERM as isize;
        }
    }

    (*t).iopl_emul = level;
    task_update_io_bitmap();
    0
}

#[cfg(not(CONFIG_X86_IOPL_IOPERM))]
pub unsafe fn ksys_ioperm(_from: usize, _num: usize, _turn_on: i32) -> isize {
    -ENOSYS as isize
}

#[cfg(not(CONFIG_X86_IOPL_IOPERM))]
pub unsafe fn ioperm(_from: usize, _num: usize, _turn_on: i32) -> isize {
    -ENOSYS as isize
}

#[cfg(not(CONFIG_X86_IOPL_IOPERM))]
pub unsafe fn iopl(_level: u32) -> isize {
    -ENOSYS as isize
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
