/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/stop_machine.h.
// Dependencies supplied by the surrounding kernel translation are referenced
// below; the original C include directives are intentionally omitted.

pub type CpuStopFnT = unsafe extern "C" fn(arg: *mut core::ffi::c_void) -> core::ffi::c_int;

#[cfg(feature = "CONFIG_SMP")]
#[repr(C)]
pub struct CpuStopWork {
    pub list: ListHead, // cpu_stopper->works
    pub fn_: CpuStopFnT,
    pub caller: libc::c_ulong,
    pub arg: *mut core::ffi::c_void,
    pub done: *mut CpuStopDone,
}

#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    pub fn stop_one_cpu(cpu: libc::c_uint, fn_: CpuStopFnT, arg: *mut core::ffi::c_void) -> libc::c_int;
    pub fn stop_two_cpus(cpu1: libc::c_uint, cpu2: libc::c_uint, fn_: CpuStopFnT, arg: *mut core::ffi::c_void) -> libc::c_int;
    pub fn stop_one_cpu_nowait(cpu: libc::c_uint, fn_: CpuStopFnT, arg: *mut core::ffi::c_void, work_buf: *mut CpuStopWork);
    pub fn stop_machine_park(cpu: libc::c_int);
    pub fn stop_machine_unpark(cpu: libc::c_int);
    pub fn stop_machine_yield(cpumask: *const CpuMask);
    pub fn print_stop_info(log_lvl: *const core::ffi::c_char, task: *mut TaskStruct);
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[repr(C)]
pub struct CpuStopWork {
    pub work: WorkStruct,
    pub fn_: CpuStopFnT,
    pub arg: *mut core::ffi::c_void,
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn stop_one_cpu(cpu: libc::c_uint, fn_: CpuStopFnT, arg: *mut core::ffi::c_void) -> libc::c_int {
    let mut ret: libc::c_int = -libc::ENOENT;
    preempt_disable();
    if cpu == smp_processor_id() {
        ret = fn_(arg);
    }
    preempt_enable();
    ret
}

#[cfg(not(feature = "CONFIG_SMP"))]
unsafe extern "C" fn stop_one_cpu_nowait_workfn(work: *mut WorkStruct) {
    let stwork = container_of::<CpuStopWork>(work, core::mem::offset_of!(CpuStopWork, work));
    preempt_disable();
    ((*stwork).fn_)((*stwork).arg);
    preempt_enable();
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn stop_one_cpu_nowait(cpu: libc::c_uint, fn_: CpuStopFnT, arg: *mut core::ffi::c_void, work_buf: *mut CpuStopWork) {
    if warn_on_once(cpu != smp_processor_id()) {
        return;
    }
    init_work(&mut (*work_buf).work, stop_one_cpu_nowait_workfn);
    (*work_buf).fn_ = fn_;
    (*work_buf).arg = arg;
    schedule_work(&mut (*work_buf).work);
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn print_stop_info(_log_lvl: *const core::ffi::c_char, _task: *mut TaskStruct) {}

#[cfg(any(feature = "CONFIG_SMP", feature = "CONFIG_HOTPLUG_CPU"))]
extern "C" {
    pub fn stop_machine(fn_: CpuStopFnT, data: *mut core::ffi::c_void, cpus: *const CpuMask) -> libc::c_int;
    pub fn stop_machine_cpuslocked(fn_: CpuStopFnT, data: *mut core::ffi::c_void, cpus: *const CpuMask) -> libc::c_int;
    pub fn stop_core_cpuslocked(cpu: libc::c_uint, fn_: CpuStopFnT, data: *mut core::ffi::c_void) -> libc::c_int;
    pub fn stop_machine_from_inactive_cpu(fn_: CpuStopFnT, data: *mut core::ffi::c_void, cpus: *const CpuMask) -> libc::c_int;
}

#[cfg(not(any(feature = "CONFIG_SMP", feature = "CONFIG_HOTPLUG_CPU")))]
#[inline(always)]
pub unsafe fn stop_machine_cpuslocked(fn_: CpuStopFnT, data: *mut core::ffi::c_void, _cpus: *const CpuMask) -> libc::c_int {
    let mut flags: libc::c_ulong = 0;
    local_irq_save(&mut flags);
    let ret = fn_(data);
    local_irq_restore(flags);
    ret
}

#[cfg(not(any(feature = "CONFIG_SMP", feature = "CONFIG_HOTPLUG_CPU")))]
#[inline(always)]
pub unsafe fn stop_machine(fn_: CpuStopFnT, data: *mut core::ffi::c_void, cpus: *const CpuMask) -> libc::c_int {
    stop_machine_cpuslocked(fn_, data, cpus)
}

#[cfg(not(any(feature = "CONFIG_SMP", feature = "CONFIG_HOTPLUG_CPU")))]
#[inline(always)]
pub unsafe fn stop_machine_from_inactive_cpu(fn_: CpuStopFnT, data: *mut core::ffi::c_void, cpus: *const CpuMask) -> libc::c_int {
    stop_machine(fn_, data, cpus)
}

// Opaque types and external helpers supplied by the translated kernel.
#[repr(C)] pub struct ListHead { _private: [u8; 0] }
#[repr(C)] pub struct CpuStopDone { _private: [u8; 0] }
#[repr(C)] pub struct WorkStruct { _private: [u8; 0] }
#[repr(C)] pub struct CpuMask { _private: [u8; 0] }
#[repr(C)] pub struct TaskStruct { _private: [u8; 0] }
extern "C" {
    fn preempt_disable();
    fn preempt_enable();
    fn smp_processor_id() -> libc::c_uint;
    fn warn_on_once(condition: bool) -> bool;
    fn init_work(work: *mut WorkStruct, function: unsafe extern "C" fn(*mut WorkStruct));
    fn schedule_work(work: *mut WorkStruct);
    fn container_of<T>(ptr: *mut WorkStruct, offset: usize) -> *mut T;
    fn local_irq_save(flags: *mut libc::c_ulong);
    fn local_irq_restore(flags: libc::c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
