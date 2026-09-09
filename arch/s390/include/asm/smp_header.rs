/* SPDX-License-Identifier: GPL-2.0 */
/*
 *    Copyright IBM Corp. 1999, 2012
 *    Author(s): Denis Joseph Barrow,
 *             Martin Schwidefsky <schwidefsky@de.ibm.com>,
 */

// C dependencies: asm/processor.h, asm/lowcore.h, asm/machine.h, asm/sigp.h

pub unsafe fn raw_smp_processor_id() -> u32 {
    let mut lc_cpu_nr: usize;
    let mut cpu: u32 = 0;

    // BUILD_BUG_ON(sizeof_field(struct lowcore, cpu_nr) != sizeof(cpu));
    // lc_cpu_nr = offsetof(struct lowcore, cpu_nr);
    // The s390 ALTERNATIVE/asm_inline sequence is retained here as source intent:
    //   ly %[cpu],%[offzero](%r0)
    //   ly %[cpu],%[offalt](%r0)
    // with LOWCORE_ALT_ADDRESS selected by MFEATURE_LOWCORE.
    // TODO: map the architecture-specific lowcore offset and alternative assembly.
    lc_cpu_nr = 0;
    let _ = lc_cpu_nr;
    cpu
}

// #define arch_scale_cpu_capacity smp_cpu_get_capacity
pub use smp_cpu_get_capacity as arch_scale_cpu_capacity;

extern "C" {
    pub static mut smp_cpu_state_mutex: mutex;
    pub static mut smp_cpu_mt_shift: u32;
    pub static mut smp_cpu_mtid: u32;
    pub static mut boot_cpu_vector_save_area: [__vector128; __NUM_VXRS];
    pub static mut cpu_setup_mask: cpumask_t;

    pub fn __cpu_up(cpu: u32, tidle: *mut task_struct) -> i32;

    pub fn arch_send_call_function_single_ipi(cpu: i32);
    pub fn arch_send_call_function_ipi_mask(mask: *const cpumask);

    pub fn smp_call_ipl_cpu(func: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, data: *mut core::ffi::c_void) -> !;
    pub fn smp_emergency_stop();

    pub fn smp_find_processor_id(address: u16) -> i32;
    pub fn smp_store_status(cpu: i32) -> i32;
    pub fn smp_save_dump_ipl_cpu();
    pub fn smp_save_dump_secondary_cpus();
    pub fn smp_yield_cpu(cpu: i32);
    pub fn smp_cpu_set_polarization(cpu: i32, val: i32);
    pub fn smp_cpu_get_polarization(cpu: i32) -> i32;
    pub fn smp_cpu_set_capacity(cpu: i32, val: usize);
    pub fn smp_set_core_capacity(cpu: i32, val: usize);
    pub fn smp_cpu_get_capacity(cpu: i32) -> usize;
    pub fn smp_cpu_get_cpu_address(cpu: i32) -> i32;
    pub fn smp_fill_possible_mask();
    pub fn smp_detect_cpus();

    pub fn __pcpu_sigp(pcpu: u16, order: u32, parm: u32, data: *mut core::ffi::c_void);
    pub fn cpu_relax();

    pub fn stap() -> u16;
    pub fn smp_rescan_cpus(early: bool) -> i32;
    pub fn cpu_die() -> !;
    pub fn __cpu_die(cpu: u32);
    pub fn __cpu_disable() -> i32;
    pub fn schedule_mcck_handler();
}

pub unsafe fn smp_stop_cpu() -> ! {
    let pcpu: u16 = stap();
    loop {
        __pcpu_sigp(pcpu, SIGP_STOP, 0, core::ptr::null_mut());
        cpu_relax();
    }
}

/* Return thread 0 CPU number as base CPU */
pub unsafe fn smp_get_base_cpu(cpu: i32) -> i32 {
    cpu - (cpu % (smp_cpu_mtid as i32 + 1))
}

pub fn smp_cpus_done(_max_cpus: u32) {}

pub unsafe fn smp_yield_cpu_notrace(cpu: i32) {
    smp_yield_cpu(cpu);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
