// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OpenRISC ptrace.c
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * Modifications for the OpenRISC architecture:
 * Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
 * Copyright (C) 2005 Gyorgy Jeney <nog@bsemi.com>
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 */

// Dependencies are supplied by the surrounding kernel translation.

/*
 * Copy the thread state to a regset that can be interpreted by userspace.
 *
 * It doesn't matter what our internal pt_regs structure looks like.  The
 * important thing is that we export a consistent view of the thread state
 * to userspace.  As such, we need to make sure that the regset remains ABI
 * compatible as defined by the struct user_regs_struct:
 *
 * (Each item is a 32-bit word)
 * r0 = 0 (exported for clarity)
 * 31 GPRS r1-r31
 * PC (Program counter)
 * SR (Supervision register)
 */
unsafe fn genregs_get(
    target: *mut task_struct,
    _regset: *const user_regset,
    mut to: membuf,
) -> c_int {
    let regs = task_pt_regs(target);

    membuf_zero(&mut to, 4);
    membuf_write(&mut to, (*regs).gpr.as_ptr().add(1) as *const c_void, 31 * 4);
    membuf_store(&mut to, (*regs).pc);
    membuf_store(&mut to, (*regs).sr)
}

/* Set the thread state from a regset passed in via ptrace */
unsafe fn genregs_set(
    target: *mut task_struct,
    _regset: *const user_regset,
    mut pos: c_uint,
    mut count: c_uint,
    mut kbuf: *const c_void,
    mut ubuf: *const c_void,
) -> c_int {
    let regs = task_pt_regs(target);
    let mut ret: c_int;

    user_regset_copyin_ignore(&mut pos, &mut count, &mut kbuf, &mut ubuf, 0, 4);
    ret = user_regset_copyin(
        &mut pos, &mut count, &mut kbuf, &mut ubuf,
        (*regs).gpr.as_mut_ptr().add(1) as *mut c_void, 4, 4 * 32,
    );
    if ret == 0 {
        ret = user_regset_copyin(
            &mut pos, &mut count, &mut kbuf, &mut ubuf,
            &mut (*regs).pc as *mut _ as *mut c_void, 4 * 32, 4 * 33,
        );
    }
    if ret == 0 {
        user_regset_copyin_ignore(&mut pos, &mut count, &mut kbuf, &mut ubuf, 4 * 33, -1);
    }
    ret
}

#[cfg(CONFIG_FPU)]
unsafe fn fpregs_get(target: *mut task_struct, _regset: *const user_regset, mut to: membuf) -> c_int {
    membuf_store(&mut to, (*target).thread.fpcsr)
}

#[cfg(CONFIG_FPU)]
unsafe fn fpregs_set(
    target: *mut task_struct, _regset: *const user_regset,
    mut pos: c_uint, mut count: c_uint, mut kbuf: *const c_void, mut ubuf: *const c_void,
) -> c_int {
    user_regset_copyin(&mut pos, &mut count, &mut kbuf, &mut ubuf,
                       &mut (*target).thread.fpcsr as *mut _ as *mut c_void, 0, 4)
}

#[repr(C)]
enum or1k_regset { REGSET_GENERAL, #[cfg(CONFIG_FPU)] REGSET_FPU }

// Register-set descriptors and the native view retain the C kernel ABI layout.
static or1k_regsets: [user_regset; 2] = [
    user_regset { core: USER_REGSET_NOTE_TYPE(PRSTATUS), n: ELF_NGREG, size: size_of::<c_long>(), align: size_of::<c_long>(), regset_get: Some(genregs_get), set: Some(genregs_set) },
    user_regset { core: USER_REGSET_NOTE_TYPE(PRFPREG), n: size_of::<__or1k_fpu_state>() / size_of::<c_long>(), size: size_of::<c_long>(), align: size_of::<c_long>(), regset_get: Some(fpregs_get), set: Some(fpregs_set) },
];

static user_or1k_native_view: user_regset_view = user_regset_view {
    name: b"or1k\0".as_ptr() as *const c_char, e_machine: EM_OPENRISC,
    regsets: or1k_regsets.as_ptr(), n: or1k_regsets.len(),
};

#[no_mangle]
pub unsafe extern "C" fn task_user_regset_view(_task: *mut task_struct) -> *const user_regset_view {
    &user_or1k_native_view
}

#[repr(C)]
struct pt_regs_offset { name: *const c_char, offset: c_int }

static regoffset_table: [pt_regs_offset; 35] = [
    pt_regs_offset { name: b"sr\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, sr) as c_int },
    pt_regs_offset { name: b"sp\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, sp) as c_int },
    pt_regs_offset { name: b"gpr2\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr2) as c_int },
    pt_regs_offset { name: b"gpr3\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr3) as c_int },
    pt_regs_offset { name: b"gpr4\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr4) as c_int },
    pt_regs_offset { name: b"gpr5\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr5) as c_int },
    pt_regs_offset { name: b"gpr6\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr6) as c_int },
    pt_regs_offset { name: b"gpr7\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr7) as c_int },
    pt_regs_offset { name: b"gpr8\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr8) as c_int },
    pt_regs_offset { name: b"gpr9\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr9) as c_int },
    pt_regs_offset { name: b"gpr10\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr10) as c_int },
    pt_regs_offset { name: b"gpr11\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr11) as c_int },
    pt_regs_offset { name: b"gpr12\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr12) as c_int },
    pt_regs_offset { name: b"gpr13\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr13) as c_int },
    pt_regs_offset { name: b"gpr14\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr14) as c_int },
    pt_regs_offset { name: b"gpr15\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr15) as c_int },
    pt_regs_offset { name: b"gpr16\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr16) as c_int },
    pt_regs_offset { name: b"gpr17\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr17) as c_int },
    pt_regs_offset { name: b"gpr18\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr18) as c_int },
    pt_regs_offset { name: b"gpr19\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr19) as c_int },
    pt_regs_offset { name: b"gpr20\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr20) as c_int },
    pt_regs_offset { name: b"gpr21\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr21) as c_int },
    pt_regs_offset { name: b"gpr22\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr22) as c_int },
    pt_regs_offset { name: b"gpr23\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr23) as c_int },
    pt_regs_offset { name: b"gpr24\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr24) as c_int },
    pt_regs_offset { name: b"gpr25\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr25) as c_int },
    pt_regs_offset { name: b"gpr26\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr26) as c_int },
    pt_regs_offset { name: b"gpr27\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr27) as c_int },
    pt_regs_offset { name: b"gpr28\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr28) as c_int },
    pt_regs_offset { name: b"gpr29\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr29) as c_int },
    pt_regs_offset { name: b"gpr30\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr30) as c_int },
    pt_regs_offset { name: b"gpr31\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, gpr31) as c_int },
    pt_regs_offset { name: b"pc\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, pc) as c_int },
    pt_regs_offset { name: b"orig_gpr11\0".as_ptr() as *const c_char, offset: offset_of!(pt_regs, orig_gpr11) as c_int },
    pt_regs_offset { name: core::ptr::null(), offset: 0 },
];

pub unsafe extern "C" fn regs_query_register_offset(name: *const c_char) -> c_int {
    let mut roff = regoffset_table.as_ptr();
    while !(*roff).name.is_null() {
        if strcmp((*roff).name, name) == 0 { return (*roff).offset; }
        roff = roff.add(1);
    }
    -EINVAL
}

unsafe fn regs_within_kernel_stack(regs: *mut pt_regs, addr: c_ulong) -> bool {
    (addr & !(THREAD_SIZE - 1)) == (kernel_stack_pointer(regs) & !(THREAD_SIZE - 1))
}

pub unsafe extern "C" fn regs_get_kernel_stack_nth(regs: *mut pt_regs, n: c_uint) -> c_ulong {
    let addr = (kernel_stack_pointer(regs) as *mut c_ulong).add(n as usize);
    if regs_within_kernel_stack(regs, addr as c_ulong) { *addr } else { 0 }
}

pub unsafe extern "C" fn ptrace_disable(child: *mut task_struct) {
    pr_debug!("ptrace_disable(): TODO\n");
    user_disable_single_step(child);
    clear_tsk_thread_flag(child, TIF_SYSCALL_TRACE);
}

pub unsafe extern "C" fn arch_ptrace(child: *mut task_struct, request: c_long, addr: c_ulong, data: c_ulong) -> c_long {
    match request { _ => ptrace_request(child, request, addr, data) }
}

pub unsafe extern "C" fn do_syscall_trace_enter(regs: *mut pt_regs) -> c_long {
    let mut ret: c_long = 0;
    if test_thread_flag(TIF_SYSCALL_TRACE) && !ptrace_report_syscall_permit_entry(regs) { ret = -1; }
    audit_syscall_entry((*regs).gpr[11], (*regs).gpr[3], (*regs).gpr[4], (*regs).gpr[5], (*regs).gpr[6]);
    if ret != 0 { ret } else { (*regs).gpr[11] as c_long }
}

pub unsafe extern "C" fn do_syscall_trace_leave(regs: *mut pt_regs) {
    audit_syscall_exit(regs);
    let step = test_thread_flag(TIF_SINGLESTEP);
    if step || test_thread_flag(TIF_SYSCALL_TRACE) { ptrace_report_syscall_exit(regs, step); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
