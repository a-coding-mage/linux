/* SPDX-License-Identifier: GPL-2.0 */
/* smp.h: Sparc specific SMP stuff.
 *
 * Copyright (C) 1996 David S. Miller (davem@caip.rutgers.edu)
 */

/* C header dependencies are supplied by the surrounding translation unit. */

/* CONFIG_SMP */

#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    pub static mut boot_cpu_id: u8;
    pub static mut cpu_callin_map: [core::ffi::c_ulong; NR_CPUS];
    pub static mut smp_commenced_mask: cpumask_t;
    pub static mut smp_penguin_ctable: linux_prom_registers;

    pub fn cpu_panic();

    pub fn sun4m_init_smp();
    pub fn sun4d_init_smp();

    pub fn smp_callin();
    pub fn smp_store_cpu_info(cpu: i32);

    pub fn smp_resched_interrupt();
    pub fn smp_call_function_single_interrupt();
    pub fn smp_call_function_interrupt();

    pub fn smp_bogo(file: *mut seq_file);
    pub fn smp_info(file: *mut seq_file);

    pub fn arch_send_call_function_single_ipi(cpu: i32);
    pub fn arch_send_call_function_ipi_mask(mask: *const cpumask);

    pub fn hard_smp_processor_id() -> i32;
}

#[cfg(feature = "CONFIG_SMP")]
#[repr(C)]
pub struct sparc32_ipi_ops {
    pub cross_call: Option<unsafe extern "C" fn(
        func: *mut core::ffi::c_void,
        mask: cpumask_t,
        arg1: core::ffi::c_ulong,
        arg2: core::ffi::c_ulong,
        arg3: core::ffi::c_ulong,
        arg4: core::ffi::c_ulong,
    )>,
    pub resched: Option<unsafe extern "C" fn(cpu: i32)>,
    pub single: Option<unsafe extern "C" fn(cpu: i32)>,
    pub mask_one: Option<unsafe extern "C" fn(cpu: i32)>,
}

#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    pub static sparc32_ipi_ops: *const sparc32_ipi_ops;
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn xc0(func: *mut core::ffi::c_void) {
    ((*sparc32_ipi_ops).cross_call.unwrap())(func, *cpu_online_mask, 0, 0, 0, 0);
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn xc1(func: *mut core::ffi::c_void, arg1: core::ffi::c_ulong) {
    ((*sparc32_ipi_ops).cross_call.unwrap())(func, *cpu_online_mask, arg1, 0, 0, 0);
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn xc2(
    func: *mut core::ffi::c_void,
    arg1: core::ffi::c_ulong,
    arg2: core::ffi::c_ulong,
) {
    ((*sparc32_ipi_ops).cross_call.unwrap())(func, *cpu_online_mask, arg1, arg2, 0, 0);
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn xc3(
    func: *mut core::ffi::c_void,
    arg1: core::ffi::c_ulong,
    arg2: core::ffi::c_ulong,
    arg3: core::ffi::c_ulong,
) {
    ((*sparc32_ipi_ops).cross_call.unwrap())(func, *cpu_online_mask, arg1, arg2, arg3, 0);
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn xc4(
    func: *mut core::ffi::c_void,
    arg1: core::ffi::c_ulong,
    arg2: core::ffi::c_ulong,
    arg3: core::ffi::c_ulong,
    arg4: core::ffi::c_ulong,
) {
    ((*sparc32_ipi_ops).cross_call.unwrap())(func, *cpu_online_mask, arg1, arg2, arg3, arg4);
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub fn cpu_logical_map(cpu: i32) -> i32 {
    cpu
}

#[cfg(feature = "CONFIG_SMP")]
pub unsafe fn raw_smp_processor_id() -> i32 {
    (*current_thread_info()).cpu
}

#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    pub fn smp_setup_cpu_possible_map();
}

pub const MSG_CROSS_CALL: u32 = 0x0005;
pub const MBOX_STOPCPU: u8 = 0xFB;
pub const MBOX_IDLECPU: u8 = 0xFC;
pub const MBOX_IDLECPU2: u8 = 0xFD;
pub const MBOX_STOPCPU2: u8 = 0xFE;

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub fn hard_smp_processor_id() -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub fn smp_setup_cpu_possible_map() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
