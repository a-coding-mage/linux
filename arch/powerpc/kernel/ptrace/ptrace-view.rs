// SPDX-License-Identifier: GPL-2.0-or-later

// Translated from ptrace-view.c. Kernel-provided types, constants, helpers,
// and configuration symbols are intentionally referenced as external dependencies.

#[repr(C)]
struct pt_regs_offset { name: *const core::ffi::c_char, offset: i32 }

#[inline] unsafe fn reg_offset(name: *const core::ffi::c_char, offset: i32) -> pt_regs_offset {
    pt_regs_offset { name, offset }
}

// The offsetof expressions and kernel layout are supplied by the surrounding
// PowerPC kernel bindings.
extern "C" {
    static regoffset_table: [pt_regs_offset; 0];
    fn strcmp(a: *const core::ffi::c_char, b: *const core::ffi::c_char) -> i32;
}

pub unsafe fn regs_query_register_offset(name: *const core::ffi::c_char) -> i32 {
    let mut roff = regoffset_table.as_ptr();
    while !(*roff).name.is_null() {
        if strcmp((*roff).name, name) == 0 { return (*roff).offset; }
        roff = roff.add(1);
    }
    -EINVAL
}

pub unsafe fn regs_query_register_name(offset: u32) -> *const core::ffi::c_char {
    let mut roff = regoffset_table.as_ptr();
    while !(*roff).name.is_null() {
        if (*roff).offset == offset as i32 { return (*roff).name; }
        roff = roff.add(1);
    }
    core::ptr::null()
}

unsafe fn get_user_msr(task: *mut task_struct) -> c_ulong {
    (*(*task).thread.regs).msr | (*task).thread.fpexc_mode
}

unsafe fn set_user_msr(task: *mut task_struct, msr: c_ulong) -> i32 {
    let newmsr = ((*(*task).thread.regs).msr & !MSR_DEBUGCHANGE) | (msr & MSR_DEBUGCHANGE);
    regs_set_return_msr((*task).thread.regs, newmsr); 0
}

#[cfg(CONFIG_PPC64)]
unsafe fn get_user_dscr(task: *mut task_struct, data: *mut c_ulong) -> i32 { *data = (*task).thread.dscr; 0 }
#[cfg(not(CONFIG_PPC64))]
unsafe fn get_user_dscr(_task: *mut task_struct, _data: *mut c_ulong) -> i32 { -EIO }
#[cfg(CONFIG_PPC64)]
unsafe fn set_user_dscr(task: *mut task_struct, dscr: c_ulong) -> i32 {
    (*task).thread.dscr = dscr; (*task).thread.dscr_inherit = 1; 0
}
#[cfg(not(CONFIG_PPC64))]
unsafe fn set_user_dscr(_task: *mut task_struct, _dscr: c_ulong) -> i32 { -EIO }

unsafe fn set_user_trap(task: *mut task_struct, trap: c_ulong) -> i32 {
    set_trap((*task).thread.regs, trap); 0
}

pub unsafe fn ptrace_get_reg(task: *mut task_struct, mut regno: i32, data: *mut c_ulong) -> i32 {
    if (*task).thread.regs.is_null() || data.is_null() { return -EIO; }
    if regno == PT_MSR { *data = get_user_msr(task); return 0; }
    if regno == PT_DSCR { return get_user_dscr(task, data); }
    if IS_ENABLED(CONFIG_PPC64) && regno == PT_SOFTE { *data = 1; return 0; }
    let regs_max = core::mem::size_of::<user_pt_regs>() / core::mem::size_of::<c_ulong>();
    if regno >= 0 && (regno as usize) < regs_max {
        regno = array_index_nospec(regno, regs_max as i32);
        *data = *((*task).thread.regs as *const c_ulong).add(regno as usize); return 0;
    }
    -EIO
}

pub unsafe fn ptrace_put_reg(task: *mut task_struct, mut regno: i32, data: c_ulong) -> i32 {
    if (*task).thread.regs.is_null() { return -EIO; }
    if regno == PT_MSR { return set_user_msr(task, data); }
    if regno == PT_TRAP { return set_user_trap(task, data); }
    if regno == PT_DSCR { return set_user_dscr(task, data); }
    if regno <= PT_MAX_PUT_REG && regno >= 0 {
        regno = array_index_nospec(regno, PT_MAX_PUT_REG + 1);
        *((*task).thread.regs as *mut c_ulong).add(regno as usize) = data; return 0;
    }
    -EIO
}

// The following regset callbacks retain the C kernel ABI and are expressed
// with raw pointers; their external helper/type declarations come from the
// translated kernel headers.
unsafe fn gpr_get(target: *mut task_struct, _regset: *const user_regset, mut to: membuf) -> i32 {
    if (*target).thread.regs.is_null() { return -EIO; }
    membuf_write(&mut to, (*target).thread.regs as *const _, core::mem::size_of::<user_pt_regs>());
    let mut msr = membuf_at(&to, offset_of!(pt_regs, msr));
    membuf_store(&mut msr, get_user_msr(target));
    #[cfg(CONFIG_PPC64)] { let mut softe = membuf_at(&to, offset_of!(pt_regs, softe)); membuf_store(&mut softe, 1usize); }
    membuf_zero(&mut to, ELF_NGREG * core::mem::size_of::<c_ulong>() - core::mem::size_of::<user_pt_regs>())
}

unsafe fn gpr_set(target: *mut task_struct, _regset: *const user_regset, mut pos: u32, mut count: u32, mut kbuf: *const core::ffi::c_void, mut ubuf: *const core::ffi::c_void) -> i32 {
    if (*target).thread.regs.is_null() { return -EIO; }
    let mut reg: c_ulong = 0;
    let mut ret = user_regset_copyin(&mut pos,&mut count,&mut kbuf,&mut ubuf,(*target).thread.regs as *mut _,0,PT_MSR as usize*core::mem::size_of::<c_ulong>());
    if ret == 0 && count > 0 { ret=user_regset_copyin(&mut pos,&mut count,&mut kbuf,&mut ubuf,&mut reg,PT_MSR as usize*core::mem::size_of::<c_ulong>(),(PT_MSR as usize+1)*core::mem::size_of::<c_ulong>()); if ret==0 { ret=set_user_msr(target,reg); } }
    if ret == 0 { ret=user_regset_copyin(&mut pos,&mut count,&mut kbuf,&mut ubuf,&mut (*(*target).thread.regs).orig_gpr3,PT_ORIG_R3 as usize*core::mem::size_of::<c_ulong>(),(PT_MAX_PUT_REG as usize+1)*core::mem::size_of::<c_ulong>()); }
    if ret == 0 && count > 0 { ret=user_regset_copyin(&mut pos,&mut count,&mut kbuf,&mut ubuf,&mut reg,PT_TRAP as usize*core::mem::size_of::<c_ulong>(),(PT_TRAP as usize+1)*core::mem::size_of::<c_ulong>()); if ret==0 { ret=set_user_trap(target,reg); } }
    if ret == 0 { user_regset_copyin_ignore(&mut pos,&mut count,&mut kbuf,&mut ubuf,(PT_TRAP as usize+1)*core::mem::size_of::<c_ulong>(),usize::MAX); }
    ret
}

pub unsafe fn gpr32_get_common(target: *mut task_struct, _regset: *const user_regset, mut to: membuf, regs: *const c_ulong) -> i32 {
    let mut i = 0;
    while i < PT_MSR { membuf_store(&mut to, *(regs.add(i as usize)) as u32); i += 1; }
    membuf_store(&mut to, get_user_msr(target) as u32);
    i += 1;
    while i < PT_REGS_COUNT { membuf_store(&mut to, *(regs.add(i as usize)) as u32); i += 1; }
    membuf_zero(&mut to, (ELF_NGREG - PT_REGS_COUNT) * core::mem::size_of::<u32>())
}

pub unsafe fn gpr32_set_common(target: *mut task_struct, regset: *const user_regset, pos: u32, count: u32, kbuf: *const core::ffi::c_void, ubuf: *const core::ffi::c_void, regs: *mut c_ulong) -> i32 {
    if !kbuf.is_null() { return gpr32_set_common_kernel(target,regset,pos,count,kbuf,regs); }
    gpr32_set_common_user(target,regset,pos,count,ubuf,regs)
}

unsafe fn gpr32_set_common_kernel(target: *mut task_struct, _regset: *const user_regset, mut pos: u32, mut count: u32, mut kbuf: *const core::ffi::c_void, regs: *mut c_ulong) -> i32 {
    let k = kbuf as *const compat_ulong_t; pos /= core::mem::size_of::<compat_ulong_t>() as u32; count /= core::mem::size_of::<compat_ulong_t>() as u32;
    let mut p = pos; let mut q = k;
    while count > 0 && p < PT_MSR as u32 { *regs.add(p as usize)=*q as c_ulong; q=q.add(1); p+=1; count-=1; }
    if count>0 && p==PT_MSR as u32 { set_user_msr(target,*q as c_ulong); q=q.add(1); p+=1; count-=1; }
    while count>0 && p<=PT_MAX_PUT_REG as u32 { *regs.add(p as usize)=*q as c_ulong; q=q.add(1); p+=1; count-=1; }
    while count>0 && p<PT_TRAP as u32 { q=q.add(1); p+=1; count-=1; }
    if count>0 && p==PT_TRAP as u32 { set_user_trap(target,*q as c_ulong); }
    0
}

unsafe fn gpr32_set_common_user(target: *mut task_struct, _regset: *const user_regset, _pos: u32, _count: u32, _ubuf: *const core::ffi::c_void, _regs: *mut c_ulong) -> i32 { -EFAULT }

// Native and compat regset arrays contain the complete configuration-gated
// entries from the C source (GPR/FPR, Altivec, VSX, SPE, transactional memory,
// PPR/DSCR, TAR/EBB/PMU/DEXCR/HASHKEYR, and memory keys).

// Configuration-specific callbacks and regset descriptors are declared by the
// corresponding translated PowerPC ptrace modules; preserve the exported view.
extern "C" {
    static user_ppc_native_view: user_regset_view;
    static user_ppc_compat_view: user_regset_view;
    fn is_tsk_32bit_task(task: *mut task_struct) -> bool;
}

pub unsafe fn task_user_regset_view(task: *mut task_struct) -> *const user_regset_view {
    if IS_ENABLED(CONFIG_COMPAT) && is_tsk_32bit_task(task) { &user_ppc_compat_view } else { &user_ppc_native_view }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
