/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/threads.h, linux/cpumask.h, linux/bitops.h, asm/pal.h

/* HACK: Cabrio WHAMI return value is bogus if more than 8 bits used.. :-( */

#[inline]
pub unsafe fn __hard_smp_processor_id() -> u8 {
    let mut r0: u8;
    core::arch::asm!(
        "call_pal {pal} #whami",
        pal = const PAL_whami,
        lateout("$0") r0,
        clobber_abi("C"),
    );
    r0
}

#[cfg(CONFIG_SMP)]
#[repr(C, align(64))]
pub struct cpuinfo_alpha {
    pub loops_per_jiffy: libc::c_ulong,
    pub last_asn: libc::c_ulong,
    pub need_new_asn: libc::c_int,
    pub asn_lock: libc::c_int,
    pub ipi_count: libc::c_ulong,
    pub prof_multiplier: libc::c_ulong,
    pub prof_counter: libc::c_ulong,
    pub mcheck_expected: u8,
    pub mcheck_taken: u8,
    pub mcheck_extra: u8,
}

#[cfg(CONFIG_SMP)]
extern "C" {
    pub static mut cpu_data: [cpuinfo_alpha; NR_CPUS];

    pub static mut smp_num_cpus: libc::c_int;
    pub fn arch_send_call_function_single_ipi(cpu: libc::c_int);
    pub fn arch_send_call_function_ipi_mask(mask: *const struct_cpumask);
}

#[cfg(CONFIG_SMP)]
#[inline]
pub unsafe fn hard_smp_processor_id() -> libc::c_int {
    __hard_smp_processor_id() as libc::c_int
}

#[cfg(CONFIG_SMP)]
#[inline]
pub unsafe fn raw_smp_processor_id() -> libc::c_int {
    (*current_thread_info()).cpu
}

#[cfg(not(CONFIG_SMP))]
#[inline]
pub const fn hard_smp_processor_id() -> libc::c_int {
    0
}

#[cfg(not(CONFIG_SMP))]
macro_rules! smp_call_function_on_cpu {
    ($func:expr, $info:expr, $wait:expr, $cpu:expr) => {{
        let _ = (&$func, &$info, &$wait, &$cpu);
        0
    }};
}

pub const NO_PROC_ID: libc::c_int = -1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
