/*
 *  linux/arch/m68k/kernel/ptrace.c
 *
 *  Copyright (C) 1994 by Hamish Macdonald
 *  Taken from linux/kernel/ptrace.c and modified for M680x0.
 *  linux/kernel/ptrace.c is by Ross Biro 1/23/92, edited by Linus Torvalds
 *
 * This file is subject to the terms and conditions of the GNU General
 * Public License.  See the file COPYING in the main directory of this
 * archive for more details.
 */

/* Kernel and architecture dependencies are supplied by the surrounding tree. */

/* does not yet catch signals sent when the child dies.
 * in exit.c or in signal.c.
 */

/* determines which bits in the SR the user has access to. */
/* 1 = access 0 = no access */
const SR_MASK: c_ulong = 0x001f;

/* sets the trace bits. */
const TRACE_BITS: c_ulong = 0xC000;
const T1_BIT: c_ulong = 0x8000;
const T0_BIT: c_ulong = 0x4000;

/* Find the stack offset for a register, relative to thread.esp0. */
macro_rules! PT_REG { ($reg:ident) => { unsafe { &(*(core::ptr::null::<pt_regs>())).$reg as *const _ as isize } }; }
macro_rules! SW_REG { ($reg:ident) => { unsafe { (&(*(core::ptr::null::<switch_stack>())).$reg as *const _ as isize) - core::mem::size_of::<switch_stack>() as isize } }; }

/* Mapping from PT_xxx to the stack offset at which the register is saved. */
static REGOFF: [c_int; 19] = [
    PT_REG!(d1) as c_int, PT_REG!(d2) as c_int, PT_REG!(d3) as c_int,
    PT_REG!(d4) as c_int, PT_REG!(d5) as c_int, SW_REG!(d6) as c_int,
    SW_REG!(d7) as c_int, PT_REG!(a0) as c_int, PT_REG!(a1) as c_int,
    PT_REG!(a2) as c_int, SW_REG!(a3) as c_int, SW_REG!(a4) as c_int,
    SW_REG!(a5) as c_int, SW_REG!(a6) as c_int, PT_REG!(d0) as c_int,
    -1, PT_REG!(orig_d0) as c_int, PT_REG!(sr) as c_int, PT_REG!(pc) as c_int,
];

/* Get contents of register REGNO in task TASK. */
#[inline]
unsafe fn get_reg(task: *mut task_struct, regno: c_int) -> c_long {
    let mut addr: *mut c_ulong;
    if regno == PT_USP { addr = &mut (*task).thread.usp; }
    else if regno < REGOFF.len() as c_int { addr = ((*task).thread.esp0 + REGOFF[regno as usize] as usize) as *mut c_ulong; }
    else { return 0; }
    if regno == PT_SR || regno == PT_PC {
        let stkadj = *((*task).thread.esp0 + PT_REG!(stkadj) as usize) as *const c_long;
        addr = (addr as usize).wrapping_add(*stkadj as usize) as *mut c_ulong;
        if regno == PT_SR { return *(addr as *const c_ushort) as c_long; }
    }
    *addr as c_long
}

/* Write contents of register REGNO in task TASK. */
#[inline]
unsafe fn put_reg(task: *mut task_struct, regno: c_int, data: c_ulong) -> c_int {
    let mut addr: *mut c_ulong;
    if regno == PT_USP { addr = &mut (*task).thread.usp; }
    else if regno < REGOFF.len() as c_int { addr = ((*task).thread.esp0 + REGOFF[regno as usize] as usize) as *mut c_ulong; }
    else { return -1; }
    if regno == PT_SR || regno == PT_PC {
        let stkadj = *((*task).thread.esp0 + PT_REG!(stkadj) as usize) as *const c_long;
        addr = (addr as usize).wrapping_add(*stkadj as usize) as *mut c_ulong;
        if regno == PT_SR { *(addr as *mut c_ushort) = data as c_ushort; return 0; }
    }
    *addr = data; 0
}

/* Make sure the single step bit is not set. */
#[inline]
unsafe fn singlestep_disable(child: *mut task_struct) {
    let tmp = (get_reg(child, PT_SR) as c_ulong) & !TRACE_BITS;
    put_reg(child, PT_SR, tmp);
    clear_tsk_thread_flag(child, TIF_DELAYED_TRACE);
}

/* Called by kernel/ptrace.c when detaching.. */
pub unsafe fn ptrace_disable(child: *mut task_struct) { singlestep_disable(child); }

pub unsafe fn user_enable_single_step(child: *mut task_struct) {
    let tmp = (get_reg(child, PT_SR) as c_ulong) & !TRACE_BITS;
    put_reg(child, PT_SR, tmp | T1_BIT);
    set_tsk_thread_flag(child, TIF_DELAYED_TRACE);
}

#[cfg(feature = "CONFIG_MMU")]
pub unsafe fn user_enable_block_step(child: *mut task_struct) {
    let tmp = (get_reg(child, PT_SR) as c_ulong) & !TRACE_BITS;
    put_reg(child, PT_SR, tmp | T0_BIT);
}

pub unsafe fn user_disable_single_step(child: *mut task_struct) { singlestep_disable(child); }

pub unsafe fn arch_ptrace(child: *mut task_struct, request: c_long, addr: c_ulong, data: c_ulong) -> c_long {
    let mut tmp: c_ulong = 0;
    let mut i: c_int;
    let mut ret: c_int = 0;
    let regno = addr >> 2;
    let mut datap = data as *mut c_ulong;
    match request {
        PTRACE_PEEKUSR => {
            if addr & 3 != 0 { return -EIO; }
            if regno < 19 { tmp = get_reg(child, regno as c_int) as c_ulong; }
            else if regno >= 21 && regno < 49 {
                tmp = (*child).thread.fp[(regno - 21) as usize];
                if FPU_IS_EMU && regno < 45 && regno % 3 == 0 { tmp = ((tmp & 0xffff0000) << 15) | ((tmp & 0x0000ffff) << 16); }
            } else {
                #[cfg(not(feature = "CONFIG_MMU"))] {
                    if regno == 49 { tmp = (*(*child).mm).start_code; }
                    else if regno == 50 { tmp = (*(*child).mm).start_data; }
                    else if regno == 51 { tmp = (*(*child).mm).end_code; }
                    else { return -EIO; }
                }
                #[cfg(feature = "CONFIG_MMU")] { return -EIO; }
            }
            ret = put_user(tmp, datap); 
        }
        PTRACE_POKEUSR => {
            if addr & 3 != 0 { return -EIO; }
            let mut value = data;
            if regno as c_int == PT_SR { value &= SR_MASK; value |= get_reg(child, PT_SR) as c_ulong & !SR_MASK; }
            if regno < 19 { if put_reg(child, regno as c_int, value) != 0 { return -EIO; } }
            else if regno >= 21 && regno < 48 {
                if FPU_IS_EMU && regno < 45 && regno % 3 == 0 { value <<= 15; value = (value & 0xffff0000) | ((value & 0x0000ffff) >> 1); }
                (*child).thread.fp[(regno - 21) as usize] = value;
            } else { return -EIO; }
        }
        PTRACE_GETREGS => { for i in 0..19 { tmp = get_reg(child, i) as c_ulong; ret = put_user(tmp, datap); if ret != 0 { break; } datap = datap.add(1); } }
        PTRACE_SETREGS => { for i in 0..19 { ret = get_user(&mut tmp, datap); if ret != 0 { break; } if i == PT_SR { tmp &= SR_MASK; tmp |= get_reg(child, PT_SR) as c_ulong & !SR_MASK; } put_reg(child, i, tmp); datap = datap.add(1); } }
        PTRACE_GETFPREGS => { if copy_to_user(datap as *mut _, &(*child).thread.fp as *const _ as *const _, core::mem::size_of::<user_m68kfp_struct>()) != 0 { ret = -EFAULT; } }
        PTRACE_SETFPREGS => { if copy_from_user(&mut (*child).thread.fp as *mut _ as *mut _, datap as *const _, core::mem::size_of::<user_m68kfp_struct>()) != 0 { ret = -EFAULT; } }
        PTRACE_GET_THREAD_AREA => { ret = put_user(task_thread_info(child).tp_value, datap); }
        _ => { ret = ptrace_request(child, request, addr, data); }
    }
    ret as c_long
}

pub unsafe fn syscall_trace_enter() -> c_int {
    let mut ret = 0;
    if test_thread_flag(TIF_SYSCALL_TRACE) { ret = !ptrace_report_syscall_permit_entry(task_pt_regs(current)); }
    if !seccomp_permit_syscall() { return -1; }
    ret
}

pub unsafe fn syscall_trace_leave() { if test_thread_flag(TIF_SYSCALL_TRACE) { ptrace_report_syscall_exit(task_pt_regs(current), 0); } }

/* The regset definitions below are enabled only for ELF FDPIC core dumps. */
#[cfg(all(feature = "CONFIG_BINFMT_ELF_FDPIC", feature = "CONFIG_ELF_CORE"))]
unsafe fn m68k_regset_get(target: *mut task_struct, _regset: *const user_regset, mut to: membuf) -> c_int {
    let ptregs = task_pt_regs(target);
    let mut uregs: [u32; ELF_NGREG] = [0; ELF_NGREG];
    ELF_CORE_COPY_REGS(uregs.as_mut_ptr(), ptregs);
    membuf_write(&mut to, uregs.as_ptr() as *const _, core::mem::size_of_val(&uregs))
}

#[cfg(all(feature = "CONFIG_BINFMT_ELF_FDPIC", feature = "CONFIG_ELF_CORE"))]
#[repr(C)]
enum m68k_regset {
    REGSET_GPR,
    #[cfg(feature = "CONFIG_FPU")]
    REGSET_FPU,
}

#[cfg(all(feature = "CONFIG_BINFMT_ELF_FDPIC", feature = "CONFIG_ELF_CORE"))]
static M68K_USER_REGSETS: [user_regset; 2] = [
    user_regset {
        core_note_type: PRSTATUS,
        n: ELF_NGREG,
        size: core::mem::size_of::<u32>(),
        align: core::mem::size_of::<u16>(),
        regset_get: Some(m68k_regset_get),
    },
    user_regset {
        core_note_type: PRFPREG,
        n: core::mem::size_of::<user_m68kfp_struct>() / core::mem::size_of::<u32>(),
        size: core::mem::size_of::<u32>(),
        align: core::mem::size_of::<u32>(),
        regset_get: None,
    },
];

#[cfg(all(feature = "CONFIG_BINFMT_ELF_FDPIC", feature = "CONFIG_ELF_CORE"))]
static USER_M68K_VIEW: user_regset_view = user_regset_view {
    name: "m68k",
    e_machine: EM_68K,
    ei_osabi: ELF_OSABI,
    regsets: M68K_USER_REGSETS.as_ptr(),
    n: M68K_USER_REGSETS.len(),
};

#[cfg(all(feature = "CONFIG_BINFMT_ELF_FDPIC", feature = "CONFIG_ELF_CORE"))]
pub unsafe fn task_user_regset_view(_task: *mut task_struct) -> *const user_regset_view {
    &USER_M68K_VIEW
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
