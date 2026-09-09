// SPDX-License-Identifier: GPL-2.0-only
/*
 * Ptrace support for Hexagon
 *
 * Copyright (c) 2010-2013, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation.

#[cfg(arch_has_single_step)]
pub unsafe fn user_enable_single_step(child: *mut task_struct) {
    pt_set_singlestep(task_pt_regs(child));
    set_tsk_thread_flag(child, TIF_SINGLESTEP);
}

#[cfg(arch_has_single_step)]
pub unsafe fn user_disable_single_step(child: *mut task_struct) {
    pt_clr_singlestep(task_pt_regs(child));
    clear_tsk_thread_flag(child, TIF_SINGLESTEP);
}

unsafe fn genregs_get(
    target: *mut task_struct,
    _regset: *const user_regset,
    mut to: membuf,
) -> i32 {
    let regs: *mut pt_regs = task_pt_regs(target);

    /*
     * The general idea here is that the copyout must happen in
     * exactly the same order in which the userspace expects these
     * regs. Now, the sequence in userspace does not match the
     * sequence in the kernel, so everything past the 32 gprs
     * happens one at a time.
     */
    membuf_write(&mut to, &(*regs).r00 as *const _, 32 * core::mem::size_of::<c_ulong>());
    /* Must be exactly same sequence as struct user_regs_struct */
    membuf_store(&mut to, (*regs).sa0);
    membuf_store(&mut to, (*regs).lc0);
    membuf_store(&mut to, (*regs).sa1);
    membuf_store(&mut to, (*regs).lc1);
    membuf_store(&mut to, (*regs).m0);
    membuf_store(&mut to, (*regs).m1);
    membuf_store(&mut to, (*regs).usr);
    membuf_store(&mut to, (*regs).preds);
    membuf_store(&mut to, (*regs).gp);
    membuf_store(&mut to, (*regs).ugp);
    membuf_store(&mut to, pt_elr(regs)); // pc
    membuf_store(&mut to, pt_cause(regs) as c_ulong); // cause
    membuf_store(&mut to, pt_badva(regs)); // badva
    #[cfg(CONFIG_HEXAGON_ARCH_VERSION = "4")]
    {
        membuf_store(&mut to, (*regs).cs0);
        membuf_store(&mut to, (*regs).cs1);
        return membuf_zero(&mut to, core::mem::size_of::<c_ulong>());
    }
    #[cfg(not(CONFIG_HEXAGON_ARCH_VERSION = "4"))]
    {
        membuf_zero(&mut to, 3 * core::mem::size_of::<c_ulong>())
    }
}

unsafe fn genregs_set(
    target: *mut task_struct,
    _regset: *const user_regset,
    mut pos: u32,
    mut count: u32,
    mut kbuf: *const core::ffi::c_void,
    mut ubuf: *const core::ffi::c_void,
) -> i32 {
    let mut ret: i32;
    let mut ignore_offset: usize;
    let mut bucket: c_ulong = 0;
    let regs: *mut pt_regs = task_pt_regs(target);

    if regs.is_null() {
        return -EIO;
    }

    ret = user_regset_copyin(&mut pos, &mut count, &mut kbuf, &mut ubuf,
        &mut (*regs).r00 as *mut _, 0, 32 * core::mem::size_of::<c_ulong>());

    macro_rules! inext {
        ($kpt_reg:expr, $usr_reg:ident) => {
            if ret == 0 {
                ret = user_regset_copyin(&mut pos, &mut count, &mut kbuf, &mut ubuf,
                    $kpt_reg, offset_of!(user_regs_struct, $usr_reg),
                    offset_of!(user_regs_struct, $usr_reg) + core::mem::size_of::<c_ulong>());
            }
        };
    }

    /* Must be exactly same sequence as struct user_regs_struct */
    inext!(&mut (*regs).sa0, sa0);
    inext!(&mut (*regs).lc0, lc0);
    inext!(&mut (*regs).sa1, sa1);
    inext!(&mut (*regs).lc1, lc1);
    inext!(&mut (*regs).m0, m0);
    inext!(&mut (*regs).m1, m1);
    inext!(&mut (*regs).usr, usr);
    inext!(&mut (*regs).preds, p3_0);
    inext!(&mut (*regs).gp, gp);
    inext!(&mut (*regs).ugp, ugp);
    inext!(&mut pt_elr(regs), pc);

    /* CAUSE and BADVA aren't writeable. */
    inext!(&mut bucket, cause);
    inext!(&mut bucket, badva);

    #[cfg(CONFIG_HEXAGON_ARCH_VERSION = "4")]
    {
        inext!(&mut (*regs).cs0, cs0);
        inext!(&mut (*regs).cs1, cs1);
        ignore_offset = offset_of!(user_regs_struct, pad1);
    }
    #[cfg(not(CONFIG_HEXAGON_ARCH_VERSION = "4"))]
    {
        ignore_offset = offset_of!(user_regs_struct, cs0);
    }

    /* Ignore the rest, if needed */
    if ret == 0 {
        user_regset_copyin_ignore(&mut pos, &mut count, &mut kbuf, &mut ubuf,
            ignore_offset, usize::MAX);
    } else {
        return ret;
    }

    /*
     * This is special; SP is actually restored by the VM via the
     * special event record which is set by the special trap.
     */
    (*regs).hvmer.vmpsp = (*regs).r29;
    0
}

#[repr(C)]
enum hexagon_regset {
    REGSET_GENERAL,
}

static hexagon_regsets: [user_regset; 1] = [user_regset {
    core_note_type: USER_REGSET_NOTE_TYPE(PRSTATUS),
    n: ELF_NGREG,
    size: core::mem::size_of::<c_ulong>(),
    align: core::mem::size_of::<c_ulong>(),
    regset_get: Some(genregs_get),
    set: Some(genregs_set),
}];

static hexagon_user_view: user_regset_view = user_regset_view {
    name: "hexagon",
    e_machine: ELF_ARCH,
    ei_osabi: ELF_OSABI,
    regsets: hexagon_regsets.as_ptr(),
    e_flags: ELF_CORE_EFLAGS,
    n: hexagon_regsets.len(),
};

pub unsafe fn task_user_regset_view(_task: *mut task_struct) -> *const user_regset_view {
    &hexagon_user_view
}

pub unsafe fn ptrace_disable(child: *mut task_struct) {
    /* Boilerplate - resolves to null inline if no HW single-step */
    user_disable_single_step(child);
}

pub unsafe fn arch_ptrace(
    child: *mut task_struct,
    request: c_long,
    addr: c_ulong,
    data: c_ulong,
) -> c_long {
    ptrace_request(child, request, addr, data)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
