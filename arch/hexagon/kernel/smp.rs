// SPDX-License-Identifier: GPL-2.0-only
/*
 * SMP support for Hexagon
 *
 * Copyright (c) 2010-2012, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the Linux kernel and Hexagon architecture headers.

const BASE_IPI_IRQ: usize = 26;

/*
 * cpu_possible_mask needs to be filled out prior to setup_per_cpu_areas
 * (which is prior to any of our smp_prepare_cpu crap), in order to set
 * up the...  per_cpu areas.
 */

#[repr(C)]
pub struct ipi_data {
    pub bits: usize,
}

static mut ipi_data: ipi_data = ipi_data { bits: 0 };

unsafe fn __handle_ipi(ops: *mut usize, _ipi: *mut ipi_data, _cpu: i32) {
    let mut msg: usize = 0;
    loop {
        msg = find_next_bit(ops, BITS_PER_LONG, msg.wrapping_add(1));

        match msg {
            IPI_TIMER => ipi_timer(),
            IPI_CALL_FUNC => generic_smp_call_function_interrupt(),
            IPI_CPU_STOP => {
                /*
                 * call vmstop()
                 */
                __vmstop();
            }
            IPI_RESCHEDULE => scheduler_ipi(),
            _ => {}
        }

        if msg >= BITS_PER_LONG {
            break;
        }
    }
}

/*  Used for IPI call from other CPU's to unmask int  */
pub unsafe extern "C" fn smp_vm_unmask_irq(info: *mut core::ffi::c_void) {
    __vmintop_locen(info as isize);
}

/*
 * This is based on Alpha's IPI stuff.
 * Supposed to take (int, void*) as args now.
 * Specifically, first arg is irq, second is the irq_desc.
 */

unsafe extern "C" fn handle_ipi(_irq: i32, _desc: *mut core::ffi::c_void) -> irqreturn_t {
    let cpu = smp_processor_id();
    let ipi = &mut ipi_data as *mut ipi_data;
    let mut ops: usize;

    while {
        ops = xchg(&mut (*ipi).bits, 0);
        ops != 0
    } {
        __handle_ipi(&mut ops, ipi, cpu);
    }
    IRQ_HANDLED
}

pub unsafe extern "C" fn send_ipi(cpumask: *const cpumask, msg: ipi_message_type) {
    let mut flags: usize = 0;
    let mut cpu: usize;
    let mut retval: usize;

    local_irq_save(&mut flags);

    for_each_cpu!(cpu, cpumask, {
        let ipi = &mut ipi_data as *mut ipi_data;

        set_bit(msg as usize, &mut (*ipi).bits);
        /*  Possible barrier here  */
        retval = __vmintop_post(BASE_IPI_IRQ + cpu);

        if retval != 0 {
            printk(KERN_ERR, b"interrupt %ld not configured?\n\0".as_ptr(),
                (BASE_IPI_IRQ + cpu) as isize);
        }
    });

    local_irq_restore(flags);
}

/*
 * interrupts should already be disabled from the VM
 * SP should already be correct; need to set THREADINFO_REG
 * to point to current thread info
 */

unsafe extern "C" fn start_secondary() {
    let mut thread_ptr: usize;
    let cpu: u32;
    let irq: u32;

    /*  Calculate thread_info pointer from stack pointer  */
    core::arch::asm!("{0} = SP", out(reg) thread_ptr);

    thread_ptr &= !(THREAD_SIZE - 1);

    core::arch::asm!("{0} = {1}", const QUOTED_THREADINFO_REG, in(reg) thread_ptr);

    /*  Set the memory struct  */
    mmgrab(&init_mm);
    (*current).active_mm = &mut init_mm;

    cpu = smp_processor_id() as u32;

    irq = BASE_IPI_IRQ as u32 + cpu;
    if request_irq(irq, Some(handle_ipi), IRQF_TRIGGER_RISING, b"ipi_handler\0".as_ptr(), core::ptr::null_mut()) != 0 {
        pr_err(b"Failed to request irq %u (ipi_handler)\n\0".as_ptr(), irq);
    }

    /*  Register the clock_event dummy  */
    setup_percpu_clockdev();

    printk(KERN_INFO, b"%s cpu %d\n\0".as_ptr(), b"start_secondary\0".as_ptr(), current_thread_info().cpu);

    notify_cpu_starting(cpu);
    set_cpu_online(cpu, true);
    local_irq_enable();
    cpu_startup_entry(CPUHP_AP_ONLINE_IDLE);
}

/*
 * called once for each present cpu
 * apparently starts up the CPU and then
 * maintains control until "cpu_online(cpu)" is set.
 */

pub unsafe extern "C" fn __cpu_up(cpu: u32, idle: *mut task_struct) -> i32 {
    let thread = (*idle).stack as *mut thread_info;
    let stack_start: *mut core::ffi::c_void;

    (*thread).cpu = cpu;

    /*  Boot to the head.  */
    stack_start = (thread as *mut u8).add(THREAD_SIZE) as *mut core::ffi::c_void;
    __vmstart(start_secondary, stack_start);

    while !cpu_online(cpu) {
        barrier();
    }

    0
}

pub unsafe extern "C" fn smp_cpus_done(_max_cpus: u32) {}

pub unsafe extern "C" fn smp_prepare_cpus(max_cpus: u32) {
    let irq = BASE_IPI_IRQ as i32;

    /*
     * should eventually have some sort of machine
     * descriptor that has this stuff
     */

    /*  Right now, let's just fake it. */
    for i in 0..max_cpus {
        set_cpu_present(i, true);
    }

    /*  Also need to register the interrupts for IPI  */
    if max_cpus > 1 {
        if request_irq(irq as u32, Some(handle_ipi), IRQF_TRIGGER_RISING,
            b"ipi_handler\0".as_ptr(), core::ptr::null_mut()) != 0 {
            pr_err(b"Failed to request irq %d (ipi_handler)\n\0".as_ptr(), irq);
        }
    }
}

pub unsafe extern "C" fn arch_smp_send_reschedule(cpu: i32) {
    send_ipi(cpumask_of(cpu), IPI_RESCHEDULE);
}

pub unsafe extern "C" fn smp_send_stop() {
    let mut targets: cpumask;
    cpumask_copy(&mut targets, cpu_online_mask);
    cpumask_clear_cpu(smp_processor_id() as u32, &mut targets);
    send_ipi(&targets, IPI_CPU_STOP);
}

pub unsafe extern "C" fn arch_send_call_function_single_ipi(cpu: i32) {
    send_ipi(cpumask_of(cpu), IPI_CALL_FUNC);
}

pub unsafe extern "C" fn arch_send_call_function_ipi_mask(mask: *const cpumask) {
    send_ipi(mask, IPI_CALL_FUNC);
}

pub unsafe extern "C" fn smp_start_cpus() {
    for i in 0..NR_CPUS {
        set_cpu_possible(i as u32, true);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
