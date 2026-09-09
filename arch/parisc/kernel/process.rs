// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *    PARISC Architecture-dependent parts of process handling
 *    based on the work for i386
 *
 *    Copyright (C) 1999-2003 Matthew Wilcox <willy at parisc-linux.org>
 *    Copyright (C) 2000 Martin K Petersen <mkp at mkp.net>
 *    Copyright (C) 2000 John Marvin <jsm at parisc-linux.org>
 *    Copyright (C) 2000 David Huggins-Daines <dhd with pobox.org>
 *    Copyright (C) 2000-2003 Paul Bame <bame at parisc-linux.org>
 *    Copyright (C) 2000 Philipp Rumpf <prumpf with tux.org>
 *    Copyright (C) 2000 David Kennedy <dkennedy with linuxcare.com>
 *    Copyright (C) 2000 Richard Hirst <rhirst with parisc-linux.org>
 *    Copyright (C) 2000 Grant Grundler <grundler with parisc-linux.org>
 *    Copyright (C) 2001 Alan Modra <amodra at parisc-linux.org>
 *    Copyright (C) 2001-2002 Ryan Bradetich <rbrad at parisc-linux.org>
 *    Copyright (C) 2001-2014 Helge Deller <deller@gmx.de>
 *    Copyright (C) 2002 Randolph Chung <tausq with parisc-linux.org>
 */

// Dependencies are supplied by the surrounding kernel translation.

pub const COMMAND_GLOBAL: u64 = F_EXTEND(0xfffe0030);
pub const CMD_RESET: i32 = 5; // reset any module

/*
** The Wright Brothers and Gecko systems have a H/W problem
** (Lasi...'nuf said) may cause a broadcast reset to lockup
** the system. An HVERSION dependent PDC call was developed
** to perform a "safe", platform specific broadcast reset instead
** of kludging up all the code.
**
** Older machines which do not implement PDC_BROADCAST_RESET will
** return (with an error) and the regular broadcast reset can be
** issued. Obviously, if the PDC does implement PDC_BROADCAST_RESET
** the PDC call will not return (the system will be reset).
*/
pub unsafe fn machine_restart(cmd: *mut u8) {
    let _ = cmd;
    // FASTBOOT_SELFTEST_SUPPORT is a build-time condition.
    /*
     ** If user has modified the Firmware Selftest Bitmap,
     ** run the tests specified in the bitmap after the
     ** system is rebooted w/PDC_DO_RESET.
     **
     ** ftc_bitmap = 0x1AUL "Skip destructive memory tests"
     **
     ** Using "directed resets" at each processor with the MEM_TOC
     ** vector cleared will also avoid running destructive
     ** memory self tests. (Not implemented yet)
     */
    // if ftc_bitmap != 0 { pdc_do_firm_test_reset(ftc_bitmap); }
    pdc_chassis_send_status(PDC_CHASSIS_DIRECT_SHUTDOWN);
    set_eiem(0);
    pdc_do_reset();
    gsc_writel(CMD_RESET as _, COMMAND_GLOBAL as _);
    loop {}
}

pub unsafe fn machine_power_off() {
    pdc_soft_power_button(0);
    pdc_chassis_send_status(PDC_CHASSIS_DIRECT_SHUTDOWN);
    do_kernel_power_off();
    printk("Power off or press RETURN to reboot.\n");
    rcu_sysrq_start();
    lockup_detector_soft_poweroff();
    loop {
        if pdc_iodc_getc() == 13 {
            printk("Rebooting...\n");
            machine_restart(core::ptr::null_mut());
        }
    }
}

pub static mut pm_power_off: Option<unsafe extern "C" fn()> = None;

pub unsafe fn machine_halt() {
    machine_power_off();
}

pub unsafe fn flush_thread() {
    /* Only needs to handle fpu stuff or perf monitors.
    ** REVISIT: several arches implement a "lazy fpu state".
    */
}

/*
 * Idle thread support
 *
 * Detect when running on QEMU with SeaBIOS PDC Firmware and let
 * QEMU idle the host too.
 */
pub static mut running_on_qemu: i32 = 0;

/* Called from the idle thread for the CPU which has been shutdown. */
pub unsafe fn arch_cpu_idle_dead() -> ! {
    // CONFIG_HOTPLUG_CPU is a build-time condition.
    idle_task_exit();
    local_irq_disable();
    cpuhp_ap_report_dead();
    flush_cache_all_local();
    flush_tlb_all_local(core::ptr::null_mut());
    __pdc_cpu_rendezvous();
    pr_warn("PDC does not provide rendezvous function.\n");
    loop {}
}

pub unsafe fn arch_cpu_idle() {
    /* nop on real hardware, qemu will idle sleep. */
    core::arch::asm!("or %r10,%r10,%r10", options(nostack, preserves_flags));
}

pub unsafe fn parisc_idle_init() -> i32 {
    if !running_on_qemu != 0 {
        cpu_idle_poll_ctrl(1);
    }
    0
}

/* Copy architecture-specific thread state */
pub unsafe fn copy_thread(
    p: *mut task_struct,
    args: *const kernel_clone_args,
) -> i32 {
    let clone_flags: u64 = (*args).flags;
    let mut usp: usize = (*args).stack;
    let tls: usize = (*args).tls;
    let cregs: *mut pt_regs = &mut (*p).thread.regs;
    let stack: *mut core::ffi::c_void = task_stack_page(p);

    extern "C" {
        static ret_from_kernel_thread: *const core::ffi::c_void;
        static child_return: *const core::ffi::c_void;
    }

    if unlikely((*args).fn_ != core::ptr::null_mut()) {
        memset(cregs as *mut core::ffi::c_void, 0, core::mem::size_of::<pt_regs>());
        if (*args).idle { return 0; }
        (*cregs).ksp = stack as usize + FRAME_SIZE + PT_SZ_ALGN;
        (*cregs).kpc = &ret_from_kernel_thread as *const _ as usize;
        // CONFIG_64BIT is a build-time condition.
        (*cregs).gr[27] = *((*args).fn_ as *const usize).add(3);
        (*cregs).gr[26] = *((*args).fn_ as *const usize).add(2);
        (*cregs).gr[25] = (*args).fn_arg as usize;
    } else {
        if usp != 0 {
            usp = (usp + 3) & !3;
            if usp != 0 { (*cregs).gr[30] = usp; }
        }
        (*cregs).ksp = stack as usize + FRAME_SIZE;
        (*cregs).kpc = &child_return as *const _ as usize;
        if clone_flags & CLONE_SETTLS != 0 { (*cregs).cr27 = tls; }
    }
    0
}

pub unsafe fn __get_wchan(p: *mut task_struct) -> usize {
    let mut info: unwind_frame_info = core::mem::zeroed();
    let mut count = 0;
    unwind_frame_init_from_blocked_task(&mut info, p);
    loop {
        if unwind_once(&mut info) < 0 { return 0; }
        if task_is_running(p) { return 0; }
        let ip = info.ip;
        if !in_sched_functions(ip) { return ip; }
        count += 1;
        if count > MAX_UNWIND_ENTRIES { return 0; }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
