// SPDX-License-Identifier: GPL-2.0

use core::ffi::c_void;

// Types and functions supplied by the kernel headers and other translation units.
#[repr(C)]
pub struct msr {
    pub q: u64,
}

#[repr(C)]
pub struct msr_info {
    pub msrs: *mut msr,
    pub msr_no: u32,
    pub reg: msr,
    pub err: i32,
}

#[repr(C)]
pub struct msr_regs_info {
    pub regs: *mut u32,
    pub err: i32,
}

#[repr(C)]
pub struct completion {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct call_single_data_t {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct cpumask {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct msr_info_completion {
    pub msr: msr_info,
    pub done: completion,
}

extern "C" {
    fn rdmsrq(msr_no: u32, q: u64);
    fn wrmsrq(msr_no: u32, q: u64);
    fn rdmsrq_safe(msr_no: u32, q: *mut u64) -> i32;
    fn wrmsrq_safe(msr_no: u32, q: u64) -> i32;
    fn rdmsr_safe_regs(regs: *mut u32) -> i32;
    fn wrmsr_safe_regs(regs: *mut u32) -> i32;
    fn smp_call_function_single(
        cpu: u32,
        func: unsafe extern "C" fn(*mut c_void),
        info: *mut c_void,
        wait: i32,
    ) -> i32;
    fn smp_call_function_single_async(
        cpu: u32,
        csd: *mut call_single_data_t,
    ) -> i32;
    fn smp_call_function_many(
        mask: *const cpumask,
        func: unsafe extern "C" fn(*mut c_void),
        info: *mut c_void,
        wait: i32,
    );
    fn get_cpu() -> i32;
    fn put_cpu();
    fn cpumask_test_cpu(cpu: i32, mask: *const cpumask) -> bool;
    fn complete(done: *mut completion);
    fn init_completion(done: *mut completion);
    fn wait_for_completion(done: *mut completion);
    fn init_csd(
        csd: *mut call_single_data_t,
        func: unsafe extern "C" fn(*mut c_void),
        info: *mut c_void,
    );
}

unsafe extern "C" fn __rdmsr_on_cpu(info: *mut c_void) {
    let rv = &mut *(info as *mut msr_info);
    let reg: *mut msr;

    if !rv.msrs.is_null() {
        reg = rv.msrs;
    } else {
        reg = &mut rv.reg;
    }

    rdmsrq(rv.msr_no, (*reg).q);
}

unsafe extern "C" fn __wrmsr_on_cpu(info: *mut c_void) {
    let rv = &mut *(info as *mut msr_info);
    let reg: *mut msr;

    if !rv.msrs.is_null() {
        reg = rv.msrs;
    } else {
        reg = &mut rv.reg;
    }

    wrmsrq(rv.msr_no, (*reg).q);
}

#[no_mangle]
pub unsafe extern "C" fn rdmsrq_on_cpu(cpu: u32, msr_no: u32, q: *mut u64) -> i32 {
    let mut rv: msr_info = core::mem::zeroed();

    rv.msr_no = msr_no;
    let err = smp_call_function_single(cpu, __rdmsr_on_cpu, &mut rv as *mut _ as *mut c_void, 1);
    *q = rv.reg.q;

    err
}

#[no_mangle]
pub unsafe extern "C" fn wrmsrq_on_cpu(cpu: u32, msr_no: u32, q: u64) -> i32 {
    let mut rv: msr_info = core::mem::zeroed();

    rv.msr_no = msr_no;
    rv.reg.q = q;
    smp_call_function_single(cpu, __wrmsr_on_cpu, &mut rv as *mut _ as *mut c_void, 1)
}

unsafe extern "C" fn __rwmsr_on_cpus(
    mask: *const cpumask,
    msr_no: u32,
    msrs: *mut msr,
    msr_func: unsafe extern "C" fn(*mut c_void),
) {
    let mut rv: msr_info = core::mem::zeroed();
    rv.msrs = msrs;
    rv.msr_no = msr_no;

    let this_cpu = get_cpu();
    if cpumask_test_cpu(this_cpu, mask) {
        msr_func(&mut rv as *mut _ as *mut c_void);
    }
    smp_call_function_many(mask, msr_func, &mut rv as *mut _ as *mut c_void, 1);
    put_cpu();
}

/* rdmsr on a bunch of CPUs
 *
 * @mask:       which CPUs
 * @msr_no:     which MSR
 * @msrs:       array of MSR values
 *
 */
#[no_mangle]
pub unsafe extern "C" fn rdmsr_on_cpus(mask: *const cpumask, msr_no: u32, msrs: *mut msr) {
    __rwmsr_on_cpus(mask, msr_no, msrs, __rdmsr_on_cpu);
}

/* wrmsr on a bunch of CPUs
 *
 * @mask:       which CPUs
 * @msr_no:     which MSR
 * @msrs:       array of MSR values
 *
 */
#[no_mangle]
pub unsafe extern "C" fn wrmsr_on_cpus(mask: *const cpumask, msr_no: u32, msrs: *mut msr) {
    __rwmsr_on_cpus(mask, msr_no, msrs, __wrmsr_on_cpu);
}

/* These "safe" variants are slower and should be used when the target MSR
   may not actually exist. */
unsafe extern "C" fn __rdmsr_safe_on_cpu(info: *mut c_void) {
    let rv = &mut *(info as *mut msr_info_completion);
    rv.msr.err = rdmsrq_safe(rv.msr.msr_no, &mut rv.msr.reg.q);
    complete(&mut rv.done);
}

unsafe extern "C" fn __wrmsr_safe_on_cpu(info: *mut c_void) {
    let rv = &mut *(info as *mut msr_info);
    rv.err = wrmsrq_safe(rv.msr_no, rv.reg.q);
}

#[no_mangle]
pub unsafe extern "C" fn wrmsrq_safe_on_cpu(cpu: u32, msr_no: u32, q: u64) -> i32 {
    let mut rv: msr_info = core::mem::zeroed();
    rv.msr_no = msr_no;
    rv.reg.q = q;
    let err = smp_call_function_single(cpu, __wrmsr_safe_on_cpu, &mut rv as *mut _ as *mut c_void, 1);
    if err != 0 { err } else { rv.err }
}

#[no_mangle]
pub unsafe extern "C" fn rdmsrq_safe_on_cpu(cpu: u32, msr_no: u32, q: *mut u64) -> i32 {
    let mut rv: msr_info_completion = core::mem::zeroed();
    let mut csd: call_single_data_t = core::mem::zeroed();
    init_csd(&mut csd, __rdmsr_safe_on_cpu, &mut rv as *mut _ as *mut c_void);
    init_completion(&mut rv.done);
    rv.msr.msr_no = msr_no;

    let mut err = smp_call_function_single_async(cpu, &mut csd);
    if err == 0 {
        wait_for_completion(&mut rv.done);
        err = rv.msr.err;
    }
    *q = rv.msr.reg.q;
    err
}

/*
 * These variants are significantly slower, but allows control over
 * the entire 32-bit GPR set.
 */
unsafe extern "C" fn __rdmsr_safe_regs_on_cpu(info: *mut c_void) {
    let rv = &mut *(info as *mut msr_regs_info);
    rv.err = rdmsr_safe_regs(rv.regs);
}

unsafe extern "C" fn __wrmsr_safe_regs_on_cpu(info: *mut c_void) {
    let rv = &mut *(info as *mut msr_regs_info);
    rv.err = wrmsr_safe_regs(rv.regs);
}

#[no_mangle]
pub unsafe extern "C" fn rdmsr_safe_regs_on_cpu(cpu: u32, regs: *mut u32) -> i32 {
    let mut rv = msr_regs_info { regs, err: -5 };
    let err = smp_call_function_single(cpu, __rdmsr_safe_regs_on_cpu, &mut rv as *mut _ as *mut c_void, 1);
    if err != 0 { err } else { rv.err }
}

#[no_mangle]
pub unsafe extern "C" fn wrmsr_safe_regs_on_cpu(cpu: u32, regs: *mut u32) -> i32 {
    let mut rv = msr_regs_info { regs, err: -5 };
    let err = smp_call_function_single(cpu, __wrmsr_safe_regs_on_cpu, &mut rv as *mut _ as *mut c_void, 1);
    if err != 0 { err } else { rv.err }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
