// SPDX-License-Identifier: GPL-2.0-only
/*
 * Uniprocessor-only support functions.  The counterpart to kernel/smp.c
 */

use core::ffi::c_void;

// Types and functions supplied by the corresponding kernel dependencies.
#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct call_single_data_t {
    pub func: Option<unsafe extern "C" fn(*mut c_void)>,
    pub info: *mut c_void,
}

pub type smp_cond_func_t = unsafe extern "C" fn(i32, *mut c_void) -> bool;
pub type smp_call_func_t = unsafe extern "C" fn(*mut c_void);

unsafe extern "C" {
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn preempt_disable();
    fn preempt_enable();
    fn cpumask_test_cpu(cpu: i32, mask: *const cpumask) -> bool;
    fn hypervisor_pin_vcpu(cpu: i32);
}

const ENXIO: i32 = 6;

pub unsafe extern "C" fn smp_call_function_single(
    cpu: i32,
    func: Option<unsafe extern "C" fn(*mut c_void)>,
    info: *mut c_void,
    _wait: bool,
) -> i32 {
    let mut flags: usize = 0;

    if cpu != 0 {
        return -ENXIO;
    }

    local_irq_save(&mut flags as *mut usize);
    if let Some(func) = func {
        func(info);
    }
    local_irq_restore(flags);

    0
}

pub unsafe extern "C" fn smp_call_function_single_async(
    _cpu: i32,
    csd: *mut call_single_data_t,
) -> i32 {
    let mut flags: usize = 0;

    local_irq_save(&mut flags as *mut usize);
    if let Some(func) = (*csd).func {
        func((*csd).info);
    }
    local_irq_restore(flags);
    0
}

/*
 * Preemption is disabled here to make sure the cond_func is called under the
 * same conditions in UP and SMP.
 */
pub unsafe extern "C" fn on_each_cpu_cond_mask(
    cond_func: Option<smp_cond_func_t>,
    func: Option<smp_call_func_t>,
    info: *mut c_void,
    _wait: bool,
    mask: *const cpumask,
) {
    let mut flags: usize = 0;

    preempt_disable();
    if (cond_func.is_none()
        || cond_func.is_some_and(|cond_func| cond_func(0, info)))
        && cpumask_test_cpu(0, mask)
    {
        local_irq_save(&mut flags as *mut usize);
        if let Some(func) = func {
            func(info);
        }
        local_irq_restore(flags);
    }
    preempt_enable();
}

pub unsafe extern "C" fn smp_call_on_cpu(
    cpu: u32,
    func: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    par: *mut c_void,
    phys: bool,
) -> i32 {
    let ret: i32;

    if cpu != 0 {
        return -ENXIO;
    }

    if phys {
        hypervisor_pin_vcpu(0);
    }
    ret = func.map_or(0, |func| func(par));
    if phys {
        hypervisor_pin_vcpu(-1);
    }

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
