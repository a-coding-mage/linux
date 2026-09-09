// SPDX-License-Identifier: GPL-2.0
/*  linux/arch/sparc/kernel/process.c
 *
 *  Copyright (C) 1995, 2008 David S. Miller (davem@davemloft.net)
 *  Copyright (C) 1996 Eddie C. Dost   (ecd@skynet.be)
 */

/* This file handles the architecture-dependent parts of process handling.. */

/* Dependencies are supplied by the surrounding kernel translation unit. */

pub static mut sparc_idle: Option<unsafe extern "C" fn()> = None;
pub static mut pm_power_off: Option<unsafe extern "C" fn()> = Some(machine_power_off);
pub static mut scons_pwroff: i32 = 1;

extern "C" {
    fn fpsave(a: *mut libc::c_ulong, b: *mut libc::c_ulong, c: *mut libc::c_void, d: *mut libc::c_ulong);
    static mut last_task_used_math: *mut task_struct;
    static mut current_set: [*mut thread_info; NR_CPUS];
    fn ret_from_fork();
    fn ret_from_kernel_thread();
}

#[no_mangle]
pub unsafe extern "C" fn arch_cpu_idle() {
    if let Some(f) = sparc_idle { f(); }
}

#[no_mangle]
pub unsafe extern "C" fn machine_halt() {
    local_irq_enable();
    mdelay(8);
    local_irq_disable();
    prom_halt();
    panic!("Halt failed!");
}

#[no_mangle]
pub unsafe extern "C" fn machine_restart(cmd: *mut libc::c_char) {
    local_irq_enable();
    mdelay(8);
    local_irq_disable();
    let p = strchr(reboot_command, b'\n' as libc::c_int);
    if !p.is_null() { *p = 0; }
    if !cmd.is_null() { prom_reboot(cmd); }
    if *reboot_command != 0 { prom_reboot(reboot_command); }
    prom_feval(b"reset\0".as_ptr() as *const libc::c_char);
    panic!("Reboot failed!");
}

#[no_mangle]
pub unsafe extern "C" fn machine_power_off() {
    if !auxio_power_register.is_null() &&
       (!of_node_is_type(of_console_device, b"serial\0".as_ptr() as *const libc::c_char) || scons_pwroff != 0) {
        let mut power_register = sbus_readb(auxio_power_register);
        power_register |= AUXIO_POWER_OFF;
        sbus_writeb(power_register, auxio_power_register);
    }
    machine_halt();
}

#[no_mangle]
pub unsafe extern "C" fn show_regs(r: *mut pt_regs) {
    let rw = (*r).u_regs[14] as *mut reg_window32;
    show_regs_print_info(KERN_DEFAULT);
    printk(b"PSR: %08lx PC: %08lx NPC: %08lx Y: %08lx    %s\n\0".as_ptr() as _, (*r).psr, (*r).pc, (*r).npc, (*r).y, print_tainted());
    printk(b"PC: <%pS>\n\0".as_ptr() as _, (*r).pc as *const libc::c_void);
    printk(b"%G: %08lx %08lx  %08lx %08lx  %08lx %08lx  %08lx %08lx\n\0".as_ptr() as _, (*r).u_regs[0], (*r).u_regs[1], (*r).u_regs[2], (*r).u_regs[3], (*r).u_regs[4], (*r).u_regs[5], (*r).u_regs[6], (*r).u_regs[7]);
    printk(b"%O: %08lx %08lx  %08lx %08lx  %08lx %08lx  %08lx %08lx\n\0".as_ptr() as _, (*r).u_regs[8], (*r).u_regs[9], (*r).u_regs[10], (*r).u_regs[11], (*r).u_regs[12], (*r).u_regs[13], (*r).u_regs[14], (*r).u_regs[15]);
    printk(b"RPC: <%pS>\n\0".as_ptr() as _, (*r).u_regs[15] as *const libc::c_void);
    printk(b"%L: %08lx %08lx  %08lx %08lx  %08lx %08lx  %08lx %08lx\n\0".as_ptr() as _, (*rw).locals[0], (*rw).locals[1], (*rw).locals[2], (*rw).locals[3], (*rw).locals[4], (*rw).locals[5], (*rw).locals[6], (*rw).locals[7]);
    printk(b"%I: %08lx %08lx  %08lx %08lx  %08lx %08lx  %08lx %08lx\n\0".as_ptr() as _, (*rw).ins[0], (*rw).ins[1], (*rw).ins[2], (*rw).ins[3], (*rw).ins[4], (*rw).ins[5], (*rw).ins[6], (*rw).ins[7]);
}

pub unsafe extern "C" fn show_stack(mut tsk: *mut task_struct, ksp: *mut libc::c_ulong, loglvl: *const libc::c_char) {
    let mut _ksp = ksp;
    if tsk.is_null() { tsk = current; }
    if tsk == current && _ksp.is_null() { core::arch::asm!("mov {}, fp", out(reg) _ksp); }
    let task_base = task_stack_page(tsk) as usize;
    let mut fp = _ksp as usize;
    let mut count = 0;
    loop {
        if fp < task_base + core::mem::size_of::<thread_info>() || fp >= task_base + (PAGE_SIZE << 1) { break; }
        let rw = fp as *mut reg_window32;
        let pc = (*rw).ins[7];
        printk(b"%s[%08lx : \0".as_ptr() as _, loglvl, pc);
        printk(b"%s%pS ] \0".as_ptr() as _, loglvl, pc as *const libc::c_void);
        fp = (*rw).ins[6] as usize;
        count += 1;
        if count >= 16 { break; }
    }
    printk(b"%s\n\0".as_ptr() as _, loglvl);
}

pub unsafe extern "C" fn exit_thread(tsk: *mut task_struct) {
    // CONFIG_SMP selects the thread-flag test in the original source.
    if last_task_used_math == tsk {
        put_psr(get_psr() | PSR_EF);
        fpsave((*tsk).thread.float_regs.as_mut_ptr(), &mut (*tsk).thread.fsr, (*tsk).thread.fpqueue.as_mut_ptr() as _, &mut (*tsk).thread.fpqdepth);
        last_task_used_math = core::ptr::null_mut();
    }
}

pub unsafe extern "C" fn flush_thread() {
    (*current_thread_info()).w_saved = 0;
    if last_task_used_math == current {
        put_psr(get_psr() | PSR_EF);
        fpsave((*current).thread.float_regs.as_mut_ptr(), &mut (*current).thread.fsr, (*current).thread.fpqueue.as_mut_ptr() as _, &mut (*current).thread.fpqdepth);
        last_task_used_math = core::ptr::null_mut();
    }
}

unsafe fn clone_stackframe(dst: *mut sparc_stackf, src: *mut sparc_stackf) -> *mut sparc_stackf {
    let mut tmp: *mut sparc_stackf = core::ptr::null_mut();
    if get_user(&mut tmp, &mut (*src).fp) != 0 { return core::ptr::null_mut(); }
    let size = (tmp as usize).wrapping_sub(src as usize);
    let sp = (dst as usize).wrapping_sub(size) as *mut sparc_stackf;
    if __copy_user(sp as _, src as _, size) != 0 { return core::ptr::null_mut(); }
    if put_user(dst as usize, &mut (*sp).fp) != 0 { return core::ptr::null_mut(); }
    sp
}

pub unsafe extern "C" fn copy_thread(p: *mut task_struct, args: *const kernel_clone_args) -> i32 {
    let clone_flags = (*args).flags;
    let tls = (*args).tls;
    let ti = task_thread_info(p);
    let regs = current_pt_regs();
    let sp = if (*args).stack != 0 { (*args).stack } else { (*regs).u_regs[UREG_FP] };
    let new_stack = (task_stack_page(p) as usize + THREAD_SIZE - STACKFRAME_SZ - TRACEREG_SZ) as *mut libc::c_char;
    let childregs = (new_stack as usize + STACKFRAME_SZ) as *mut pt_regs;
    (*ti).ksp = new_stack as usize;
    (*p).thread.kregs = childregs;
    if (*args).fn_.is_some() {
        let psr;
        memset(new_stack as _, 0, STACKFRAME_SZ + TRACEREG_SZ);
        (*ti).kpc = ret_from_kernel_thread as usize - 8;
        (*childregs).u_regs[UREG_G1] = (*args).fn_ as usize;
        (*childregs).u_regs[UREG_G2] = (*args).fn_arg as usize;
        psr = get_psr();
        (*childregs).psr = psr;
        (*ti).kpsr = psr | PSR_PIL;
        (*ti).kwim = 1 << (((psr & PSR_CWP) + 1) % nwindows);
        return 0;
    }
    memcpy(new_stack as _, (regs as usize - STACKFRAME_SZ) as _, STACKFRAME_SZ + TRACEREG_SZ);
    (*childregs).u_regs[UREG_FP] = sp;
    (*ti).kpc = ret_from_fork as usize - 8;
    (*ti).kpsr = (*current).thread.fork_kpsr | PSR_PIL;
    (*ti).kwim = (*current).thread.fork_kwim;
    if sp != (*regs).u_regs[UREG_FP] {
        let childstack = clone_stackframe((sp & !0xf) as *mut sparc_stackf, (*regs).u_regs[UREG_FP] as *mut sparc_stackf);
        if childstack.is_null() { return -EFAULT; }
        (*childregs).u_regs[UREG_FP] = childstack as usize;
    }
    (*childregs).u_regs[UREG_I0] = if (*regs).u_regs[UREG_G1] == __NR_clone3 { 0 } else { (*current).pid as usize };
    if (*regs).u_regs[UREG_G1] != __NR_clone3 { (*childregs).u_regs[UREG_I1] = 1; (*regs).u_regs[UREG_I1] = 0; }
    if clone_flags & CLONE_SETTLS != 0 { (*childregs).u_regs[UREG_G7] = tls; }
    0
}

pub unsafe extern "C" fn __get_wchan(task: *mut task_struct) -> usize {
    let task_base = task as usize;
    let mut fp = (*task_thread_info(task)).ksp;
    let mut count = 0;
    loop {
        if fp < task_base + core::mem::size_of::<thread_info>() || fp >= task_base + 2 * PAGE_SIZE { break; }
        let rw = fp as *mut reg_window32;
        let pc = (*rw).ins[7];
        if !in_sched_functions(pc) { return pc; }
        fp = (*rw).ins[6];
        count += 1;
        if count >= 16 { break; }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
