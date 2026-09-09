// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel translation.

/*
 * sys_alloc_thread_area: get a yet unused TLS descriptor index.
 */
unsafe fn get_free_idx() -> i32 {
    let t = &mut (*current).thread;
    let mut idx: i32;

    idx = 0;
    while idx < GDT_ENTRY_TLS_ENTRIES {
        if desc_empty(&t.tls_array[idx as usize]) {
            return idx + GDT_ENTRY_TLS_MIN;
        }
        idx += 1;
    }
    -ESRCH
}

unsafe fn tls_desc_okay(info: *const user_desc) -> bool {
    /*
     * For historical reasons (i.e. no one ever documented how any
     * of the segmentation APIs work), user programs can and do
     * assume that a struct user_desc that's all zeros except for
     * entry_number means "no segment at all".  This never actually
     * worked.  In fact, up to Linux 3.19, a struct user_desc like
     * this would create a 16-bit read-write segment with base and
     * limit both equal to zero.
     *
     * That was close enough to "no segment at all" until we
     * hardened this function to disallow 16-bit TLS segments.  Fix
     * it up by interpreting these zeroed segments the way that they
     * were almost certainly intended to be interpreted.
     *
     * The correct way to ask for "no segment at all" is to specify
     * a user_desc that satisfies LDT_empty.  To keep everything
     * working, we accept both.
     *
     * Note that there's a similar kludge in modify_ldt -- look at
     * the distinction between modes 1 and 0x11.
     */
    if LDT_empty(info) || LDT_zero(info) {
        return true;
    }

    /*
     * espfix is required for 16-bit data segments, but espfix
     * only works for LDT segments.
     */
    if !(*info).seg_32bit {
        return false;
    }

    /* Only allow data segments in the TLS array. */
    if (*info).contents > 1 {
        return false;
    }

    /*
     * Non-present segments with DPL 3 present an interesting attack
     * surface.  The kernel should handle such segments correctly,
     * but TLS is very difficult to protect in a sandbox, so prevent
     * such segments from being created.
     *
     * If userspace needs to remove a TLS entry, it can still delete
     * it outright.
     */
    if (*info).seg_not_present {
        return false;
    }

    true
}

unsafe fn set_tls_desc(p: *mut task_struct, mut idx: i32,
                       mut info: *const user_desc, mut n: i32) {
    let t = &mut (*p).thread;
    let mut desc = t.tls_array.as_mut_ptr().add((idx - GDT_ENTRY_TLS_MIN) as usize);

    /*
     * We must not get preempted while modifying the TLS.
     */
    let cpu = get_cpu();

    while n > 0 {
        if LDT_empty(info) || LDT_zero(info) {
            core::ptr::write_bytes(desc, 0, 1);
        } else {
            fill_ldt(desc, info);
        }
        info = info.add(1);
        desc = desc.add(1);
        n -= 1;
    }

    if core::ptr::eq(t, &mut (*current).thread) {
        load_TLS(t, cpu);
    }

    put_cpu();
}

/*
 * Set a given TLS descriptor:
 */
unsafe fn do_set_thread_area(p: *mut task_struct, mut idx: i32,
                             u_info: *mut user_desc,
                             can_allocate: i32) -> i32 {
    let mut info: user_desc = core::mem::zeroed();
    let mut modified_sel: u16;

    if copy_from_user(&mut info, u_info, core::mem::size_of::<user_desc>()) != 0 {
        return -EFAULT;
    }
    if !tls_desc_okay(&info) {
        return -EINVAL;
    }
    if idx == -1 { idx = info.entry_number; }

    /* index -1 means the kernel should try to find and allocate an empty descriptor: */
    if idx == -1 && can_allocate != 0 {
        idx = get_free_idx();
        if idx < 0 { return idx; }
        if put_user(idx, &mut (*u_info).entry_number) != 0 { return -EFAULT; }
    }
    if idx < GDT_ENTRY_TLS_MIN || idx > GDT_ENTRY_TLS_MAX { return -EINVAL; }

    set_tls_desc(p, idx, &info, 1);
    /* If DS, ES, FS, or GS points to the modified segment, forcibly refresh it. */
    modified_sel = ((idx << 3) | 3) as u16;
    if p == current {
        let mut sel: u16 = 0;
        savesegment!(ds, sel); if sel == modified_sel { loadsegment!(ds, sel); }
        savesegment!(es, sel); if sel == modified_sel { loadsegment!(es, sel); }
        savesegment!(fs, sel); if sel == modified_sel { loadsegment!(fs, sel); }
        savesegment!(gs, sel); if sel == modified_sel { load_gs_index(sel); }
    } else {
        if (*p).thread.fsindex == modified_sel { (*p).thread.fsbase = info.base_addr; }
        if (*p).thread.gsindex == modified_sel { (*p).thread.gsbase = info.base_addr; }
    }
    0
}

unsafe fn set_thread_area(u_info: *mut user_desc) -> i32 {
    do_set_thread_area(current, -1, u_info, 1)
}

/* Get the current Thread-Local Storage area: */
unsafe fn fill_user_desc(info: *mut user_desc, idx: i32, desc: *const desc_struct) {
    core::ptr::write_bytes(info, 0, 1);
    (*info).entry_number = idx;
    (*info).base_addr = get_desc_base(desc);
    (*info).limit = get_desc_limit(desc);
    (*info).seg_32bit = (*desc).d;
    (*info).contents = (*desc).type_ >> 2;
    (*info).read_exec_only = !((*desc).type_ & 2);
    (*info).limit_in_pages = (*desc).g;
    (*info).seg_not_present = !(*desc).p;
    (*info).useable = (*desc).avl;
    (*info).lm = (*desc).l;
}

unsafe fn do_get_thread_area(p: *mut task_struct, mut idx: i32,
                             u_info: *mut user_desc) -> i32 {
    let mut info: user_desc = core::mem::zeroed();
    if idx == -1 && get_user(&mut idx, &mut (*u_info).entry_number) != 0 { return -EFAULT; }
    if idx < GDT_ENTRY_TLS_MIN || idx > GDT_ENTRY_TLS_MAX { return -EINVAL; }
    let index = array_index_nospec(idx - GDT_ENTRY_TLS_MIN,
                                   GDT_ENTRY_TLS_MAX - GDT_ENTRY_TLS_MIN + 1);
    fill_user_desc(&mut info, idx, &(*p).thread.tls_array[index as usize]);
    if copy_to_user(u_info, &info, core::mem::size_of::<user_desc>()) != 0 { return -EFAULT; }
    0
}

unsafe fn get_thread_area(u_info: *mut user_desc) -> i32 {
    do_get_thread_area(current, -1, u_info)
}

unsafe fn regset_tls_active(target: *mut task_struct, _regset: *const user_regset) -> i32 {
    let t = &(*target).thread;
    let mut n = GDT_ENTRY_TLS_ENTRIES;
    while n > 0 && desc_empty(&t.tls_array[(n - 1) as usize]) { n -= 1; }
    n
}

unsafe fn regset_tls_get(target: *mut task_struct, _regset: *const user_regset,
                         to: *mut membuf) -> i32 {
    let mut pos = 0;
    while (*to).left != 0 {
        let mut v: user_desc = core::mem::zeroed();
        fill_user_desc(&mut v, GDT_ENTRY_TLS_MIN + pos, &(*target).thread.tls_array[pos as usize]);
        membuf_write(to, &v, core::mem::size_of::<user_desc>());
        pos += 1;
    }
    0
}

unsafe fn regset_tls_set(target: *mut task_struct, _regset: *const user_regset,
                         pos: u32, count: u32, kbuf: *const core::ffi::c_void,
                         ubuf: *const core::ffi::c_void) -> i32 {
    let mut infobuf: [user_desc; GDT_ENTRY_TLS_ENTRIES as usize] = core::mem::zeroed();
    let info: *const user_desc;
    let size = core::mem::size_of::<user_desc>() as u32;
    if pos >= GDT_ENTRY_TLS_ENTRIES as u32 * size || pos % size != 0 || count % size != 0 { return -EINVAL; }
    if !kbuf.is_null() { info = kbuf as *const user_desc; }
    else if __copy_from_user(infobuf.as_mut_ptr() as *mut core::ffi::c_void, ubuf, count as usize) != 0 { return -EFAULT; }
    else { info = infobuf.as_ptr(); }
    let mut i = 0;
    while i < count / size {
        if !tls_desc_okay(info.add(i as usize)) { return -EINVAL; }
        i += 1;
    }
    set_tls_desc(target, GDT_ENTRY_TLS_MIN + (pos / size) as i32, info, (count / size) as i32);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
