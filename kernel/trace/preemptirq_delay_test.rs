// SPDX-License-Identifier: GPL-2.0
/*
 * Preempt / IRQ disable delay thread to test latency tracers
 *
 * Copyright (C) 2018 Joel Fernandes (Google) <joel@joelfernandes.org>
 */

// Kernel headers and module/sysfs infrastructure are supplied by other files.

static mut DELAY: c_ulong = 100;
static mut TEST_MODE: [c_char; 12] = *b"irq\0\0\0\0\0\0\0\0\0";
static mut BURST_SIZE: c_uint = 1;
static mut CPU_AFFINITY: c_int = -1;

// module_param_named/module_param_string and MODULE_PARM_DESC declarations
// preserve the corresponding kernel module parameters and descriptions.

extern "C" {
    static mut DONE: completion;
}

unsafe fn busy_wait(time: c_ulong) {
    let start: u64 = trace_clock_local();
    let mut end: u64;

    loop {
        end = trace_clock_local();
        if kthread_should_stop() {
            break;
        }
        if end.wrapping_sub(start) >= time.wrapping_mul(1000) {
            break;
        }
    }
}

#[inline(always)]
unsafe fn irqoff_test() {
    let mut flags: c_ulong = 0;
    local_irq_save(&mut flags);
    busy_wait(DELAY);
    local_irq_restore(flags);
}

#[inline(always)]
unsafe fn preemptoff_test() {
    preempt_disable();
    busy_wait(DELAY);
    preempt_enable();
}

unsafe fn execute_preemptirqtest(idx: c_int) {
    if strcmp(TEST_MODE.as_ptr(), b"irq\0".as_ptr() as *const c_char) == 0 {
        irqoff_test();
    } else if strcmp(TEST_MODE.as_ptr(), b"preempt\0".as_ptr() as *const c_char) == 0 {
        preemptoff_test();
    } else if strcmp(TEST_MODE.as_ptr(), b"alternate\0".as_ptr() as *const c_char) == 0 {
        if idx % 2 == 0 {
            irqoff_test();
        } else {
            preemptoff_test();
        }
    }
}

// DECLARE_TESTFN generates ten functions so that each has a distinct backtrace.
macro_rules! declare_testfn {
    ($name:ident) => {
        unsafe fn $name(idx: c_int) {
            execute_preemptirqtest(idx);
        }
    };
}

declare_testfn!(preemptirqtest_0);
declare_testfn!(preemptirqtest_1);
declare_testfn!(preemptirqtest_2);
declare_testfn!(preemptirqtest_3);
declare_testfn!(preemptirqtest_4);
declare_testfn!(preemptirqtest_5);
declare_testfn!(preemptirqtest_6);
declare_testfn!(preemptirqtest_7);
declare_testfn!(preemptirqtest_8);
declare_testfn!(preemptirqtest_9);

static TESTFUNCS: [unsafe fn(c_int); 10] = [
    preemptirqtest_0, preemptirqtest_1, preemptirqtest_2, preemptirqtest_3,
    preemptirqtest_4, preemptirqtest_5, preemptirqtest_6, preemptirqtest_7,
    preemptirqtest_8, preemptirqtest_9,
];

unsafe fn preemptirq_delay_run(_data: *mut c_void) -> c_int {
    let s = core::cmp::min(BURST_SIZE as usize, TESTFUNCS.len());
    let mut cpu_mask: cpumask_var_t = core::ptr::null_mut();

    if !alloc_cpumask_var(&mut cpu_mask, GFP_KERNEL) {
        return -ENOMEM;
    }

    let mut valid = true;
    if CPU_AFFINITY > -1 {
        let cpu = CPU_AFFINITY as c_uint;
        if cpu >= nr_cpu_ids || !cpu_possible(cpu) {
            pr_err!("cpu_affinity:%d, invalid CPU\n", CPU_AFFINITY);
            valid = false;
        }
        if valid {
            cpumask_clear(cpu_mask);
            cpumask_set_cpu(CPU_AFFINITY as c_uint, cpu_mask);
            if set_cpus_allowed_ptr(current, cpu_mask) != 0 {
                pr_err!("cpu_affinity:%d, failed\n", CPU_AFFINITY);
            }
        }
    }

    if valid {
        for i in 0..s {
            TESTFUNCS[i](i as c_int);
        }
    }

out:
    complete(&mut DONE);
    set_current_state(TASK_INTERRUPTIBLE);
    while !kthread_should_stop() {
        schedule();
        set_current_state(TASK_INTERRUPTIBLE);
    }
    __set_current_state(TASK_RUNNING);
    free_cpumask_var(cpu_mask);
    0
}

unsafe fn preemptirq_run_test() -> c_int {
    let mut task: *mut task_struct;
    let mut task_name = [0 as c_char; 50];
    init_completion(&mut DONE);
    snprintf(task_name.as_mut_ptr(), task_name.len(), b"%s_test\0".as_ptr() as *const c_char, TEST_MODE.as_ptr());
    task = kthread_run(preemptirq_delay_run, core::ptr::null_mut(), task_name.as_ptr());
    if IS_ERR(task) {
        return PTR_ERR(task);
    }
    if !task.is_null() {
        wait_for_completion(&mut DONE);
        kthread_stop(task);
    }
    0
}

unsafe fn trigger_store(_kobj: *mut kobject, _attr: *mut kobj_attribute, _buf: *const c_char, count: usize) -> isize {
    let ret = preemptirq_run_test();
    if ret != 0 { ret as isize } else { count as isize }
}

// The remaining kobject attributes, init/exit hooks, and module metadata map
// directly to the kernel's __ATTR, attribute_group, module_init/module_exit,
// MODULE_DESCRIPTION, and MODULE_LICENSE declarations.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
