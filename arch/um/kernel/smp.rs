// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2025 Ant Group
 * Author: Tiwei Bie <tiwei.btw@antgroup.com>
 *
 * Based on the previous implementation in TT mode
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// Linux/UML dependencies supplied by other translation units.

enum {
    UML_IPI_RES = 0,
    UML_IPI_CALL_SINGLE,
    UML_IPI_CALL,
    UML_IPI_STOP,
}

pub unsafe fn arch_smp_send_reschedule(cpu: i32) {
    os_send_ipi(cpu, UML_IPI_RES);
}

pub unsafe fn arch_send_call_function_single_ipi(cpu: i32) {
    os_send_ipi(cpu, UML_IPI_CALL_SINGLE);
}

pub unsafe fn arch_send_call_function_ipi_mask(mask: *const cpumask) {
    let mut cpu: i32;
    for_each_cpu!(cpu, mask);
    os_send_ipi(cpu, UML_IPI_CALL);
}

pub unsafe fn smp_send_stop() {
    let mut cpu: i32;
    let me: i32 = smp_processor_id();

    for_each_online_cpu!(cpu) {
        if cpu == me {
            continue;
        }
        os_send_ipi(cpu, UML_IPI_STOP);
    }
}

unsafe fn ipi_handler(vector: i32, regs: *mut uml_pt_regs) {
    let old_regs: *mut pt_regs = set_irq_regs(regs as *mut pt_regs);
    let cpu: i32 = raw_smp_processor_id();

    irq_enter();

    if !(*current).mm.is_null() {
        os_alarm_process((*(*current).mm).context.id.pid);
    }

    match vector {
        UML_IPI_RES => {
            inc_irq_stat!(irq_resched_count);
            scheduler_ipi();
        }
        UML_IPI_CALL_SINGLE => {
            inc_irq_stat!(irq_call_count);
            generic_smp_call_function_single_interrupt();
        }
        UML_IPI_CALL => {
            inc_irq_stat!(irq_call_count);
            generic_smp_call_function_interrupt();
        }
        UML_IPI_STOP => {
            set_cpu_online(cpu, false);
            loop {
                pause();
            }
        }
        _ => {
            pr_err!("CPU#{} received unknown IPI (vector={})!\n", cpu, vector);
        }
    }

    irq_exit();
    set_irq_regs(old_regs);
}

pub unsafe fn uml_ipi_handler(vector: i32) {
    let mut r = uml_pt_regs { is_user: 0 };

    preempt_disable();
    ipi_handler(vector, &mut r);
    preempt_enable();
}

/* AP states used only during CPU startup */
enum {
    UML_CPU_PAUSED = 0,
    UML_CPU_RUNNING,
}

static mut cpu_states: [i32; NR_CPUS as usize] = [0; NR_CPUS as usize];

unsafe fn start_secondary(_unused: *mut core::ffi::c_void) -> i32 {
    let cpu: i32 = raw_smp_processor_id();

    notify_cpu_starting(cpu);
    set_cpu_online(cpu, true);

    let err = um_setup_timer();
    if err != 0 {
        panic!("CPU#{} failed to setup timer, err = {}", cpu, err);
    }

    local_irq_enable();
    cpu_startup_entry(CPUHP_AP_ONLINE_IDLE);
    0
}

pub unsafe fn uml_start_secondary(opaque: *mut core::ffi::c_void) {
    let cpu: i32 = raw_smp_processor_id();
    let mm: *mut mm_struct = &mut init_mm;
    let idle: *mut task_struct;

    stack_protections((&mut cpu_irqstacks[cpu as usize]) as *mut _ as usize);
    set_sigstack(&mut cpu_irqstacks[cpu as usize], THREAD_SIZE);

    set_cpu_present(cpu, true);
    os_futex_wait(&mut cpu_states[cpu as usize], UML_CPU_PAUSED);

    smp_rmb(); /* paired with smp_wmb() in __cpu_up() */

    idle = cpu_tasks[cpu as usize];
    (*idle).thread_info.cpu = cpu;

    mmgrab(mm);
    (*idle).active_mm = mm;

    (*idle).thread.request.thread.proc = Some(start_secondary);
    (*idle).thread.request.thread.arg = core::ptr::null_mut();

    new_thread(task_stack_page(idle), &mut (*idle).thread.switch_buf,
               new_thread_handler);
    os_start_secondary(opaque, &mut (*idle).thread.switch_buf);
}

pub unsafe fn smp_prepare_cpus(_max_cpus: u32) {
    let me: i32 = smp_processor_id();
    let mut deadline: unsigned_long;

    os_init_smp();

    for_each_possible_cpu!(cpu) {
        if cpu == me {
            continue;
        }

        pr_debug!("Booting processor {}...\n", cpu);
        let err = os_start_cpu_thread(cpu);
        if err != 0 {
            pr_crit!("CPU#{} failed to start cpu thread, err = {}", cpu, err);
            continue;
        }

        deadline = jiffies + msecs_to_jiffies(1000);
        spin_until_cond!(cpu_present(cpu) || time_is_before_jiffies(deadline));

        if !cpu_present(cpu) {
            pr_crit!("CPU#{} failed to boot\n", cpu);
        }
    }
}

pub unsafe fn __cpu_up(cpu: u32, tidle: *mut task_struct) -> i32 {
    cpu_tasks[cpu as usize] = tidle;
    smp_wmb(); /* paired with smp_rmb() in uml_start_secondary() */
    cpu_states[cpu as usize] = UML_CPU_RUNNING;
    os_futex_wake(&mut cpu_states[cpu as usize]);
    spin_until_cond!(cpu_online(cpu));
    0
}

pub unsafe fn smp_cpus_done(_max_cpus: u32) {}

/* Set in uml_ncpus_setup */
pub static mut uml_ncpus: i32 = 1;

pub unsafe fn prefill_possible_map() {
    let mut cpu: i32 = 0;
    while cpu < uml_ncpus {
        set_cpu_possible(cpu, true);
        cpu += 1;
    }
    while cpu < NR_CPUS {
        set_cpu_possible(cpu, false);
        cpu += 1;
    }
}

unsafe fn uml_ncpus_setup(line: *mut i8, add: *mut i32) -> i32 {
    *add = 0;

    if kstrtoint(line, 10, &mut uml_ncpus) != 0 {
        os_warn!("{}: Couldn't parse '{}'\n", "uml_ncpus_setup", line);
        return -1;
    }

    uml_ncpus = clamp!(uml_ncpus, 1, NR_CPUS);
    0
}

// __uml_setup("ncpus=", uml_ncpus_setup,
// "ncpus=<# of desired CPUs>\n"
// "    This tells UML how many virtual processors to start. The maximum\n"
// "    number of supported virtual processors can be obtained by querying\n"
// "    the CONFIG_NR_CPUS option using --showconfig.\n\n");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
