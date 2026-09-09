/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by asm/cpu-features.h, asm/cache.h, and asm/processor_32.h

/*
 * CPU type and hardware bug flags. Kept separately for each CPU.
 *
 * Each one of these also needs a CONFIG_CPU_SUBTYPE_xxx entry
 * in arch/sh/mm/Kconfig, as well as an entry in arch/sh/kernel/setup.c
 * for parsing the subtype in get_cpu_subtype().
 */
#[repr(C)]
pub enum cpu_type {
    // SH-2 types
    CPU_SH7619,
    CPU_J2,

    // SH-2A types
    CPU_SH7201,
    CPU_SH7203,
    CPU_SH7206,
    CPU_SH7263,
    CPU_SH7264,
    CPU_SH7269,
    CPU_MXG,

    // SH-3 types
    CPU_SH7705,
    CPU_SH7706,
    CPU_SH7707,
    CPU_SH7708,
    CPU_SH7708S,
    CPU_SH7708R,
    CPU_SH7709,
    CPU_SH7709A,
    CPU_SH7710,
    CPU_SH7712,
    CPU_SH7720,
    CPU_SH7721,
    CPU_SH7729,

    // SH-4 types
    CPU_SH7750,
    CPU_SH7750S,
    CPU_SH7750R,
    CPU_SH7751,
    CPU_SH7751R,
    CPU_SH7760,
    CPU_SH4_202,
    CPU_SH4_501,

    // SH-4A types
    CPU_SH7763,
    CPU_SH7770,
    CPU_SH7780,
    CPU_SH7781,
    CPU_SH7785,
    CPU_SH7786,
    CPU_SH7723,
    CPU_SH7724,
    CPU_SH7757,
    CPU_SH7734,
    CPU_SHX3,

    // SH4AL-DSP types
    CPU_SH7343,
    CPU_SH7722,
    CPU_SH7366,
    CPU_SH7372,

    // Unknown subtype
    CPU_SH_NONE,
}

#[repr(C)]
pub enum cpu_family {
    CPU_FAMILY_SH2,
    CPU_FAMILY_SH2A,
    CPU_FAMILY_SH3,
    CPU_FAMILY_SH4,
    CPU_FAMILY_SH4A,
    CPU_FAMILY_SH4AL_DSP,
    CPU_FAMILY_UNKNOWN,
}

/* TLB information structure, defined for both I and D TLB, per-processor. */
#[repr(C)]
pub struct tlb_info {
    pub next: u64,
    pub first: u64,
    pub last: u64,
    pub entries: u32,
    pub step: u32,
    pub flags: libc::c_ulong,
}

#[repr(C)]
pub struct sh_cpuinfo {
    pub type_: u32,
    pub family: u32,
    pub cut_major: libc::c_int,
    pub cut_minor: libc::c_int,
    pub loops_per_jiffy: libc::c_ulong,
    pub asid_cache: libc::c_ulong,
    pub icache: cache_info,
    pub dcache: cache_info,
    pub scache: cache_info,
    pub itlb: tlb_info,
    pub dtlb: tlb_info,
    pub phys_bits: u32,
    pub flags: libc::c_ulong,
}

extern "C" {
    pub static mut cpu_data: [sh_cpuinfo; 0];
    pub fn smp_processor_id() -> libc::c_int;
    pub fn raw_smp_processor_id() -> libc::c_int;

    pub fn default_idle();
    pub fn stop_this_cpu(arg: *mut libc::c_void);

    pub static mut fake_swapper_regs: pt_regs;
    pub fn cpu_init();
    pub fn cpu_probe();

    pub static mut xstate_size: u32;
    pub fn free_thread_xstate(tsk: *mut task_struct);
    pub static mut task_xstate_cachep: *mut kmem_cache;

    pub fn get_unalign_ctl(tsk: *mut task_struct, addr: libc::c_ulong) -> libc::c_int;
    pub fn set_unalign_ctl(tsk: *mut task_struct, val: u32) -> libc::c_int;

    pub static mut mem_init_done: u32;
    pub fn get_cpu_subtype(c: *mut sh_cpuinfo) -> *const libc::c_char;
    pub static cpuinfo_op: seq_operations;

    pub fn generic_mode_pins() -> libc::c_int;
    pub fn test_mode_pin(pin: libc::c_int) -> libc::c_int;

    #[cfg(CONFIG_VSYSCALL)]
    pub fn vsyscall_init() -> libc::c_int;

    #[cfg(CONFIG_CPU_SH2A)]
    pub fn instruction_size(insn: u32) -> u32;

    pub fn select_idle_routine();
}

// Forward declarations and types supplied by other translated headers:
// seq_operations, task_struct, pt_regs, kmem_cache, cache_info.

pub const SH_THREAD_UAC_NOPRINT: libc::c_uint = 1 << 0;
pub const SH_THREAD_UAC_SIGBUS: libc::c_uint = 1 << 1;
pub const SH_THREAD_UAC_MASK: libc::c_uint = SH_THREAD_UAC_NOPRINT | SH_THREAD_UAC_SIGBUS;

pub const MODE_PIN0: libc::c_uint = 1 << 0;
pub const MODE_PIN1: libc::c_uint = 1 << 1;
pub const MODE_PIN2: libc::c_uint = 1 << 2;
pub const MODE_PIN3: libc::c_uint = 1 << 3;
pub const MODE_PIN4: libc::c_uint = 1 << 4;
pub const MODE_PIN5: libc::c_uint = 1 << 5;
pub const MODE_PIN6: libc::c_uint = 1 << 6;
pub const MODE_PIN7: libc::c_uint = 1 << 7;
pub const MODE_PIN8: libc::c_uint = 1 << 8;
pub const MODE_PIN9: libc::c_uint = 1 << 9;
pub const MODE_PIN10: libc::c_uint = 1 << 10;
pub const MODE_PIN11: libc::c_uint = 1 << 11;
pub const MODE_PIN12: libc::c_uint = 1 << 12;
pub const MODE_PIN13: libc::c_uint = 1 << 13;
pub const MODE_PIN14: libc::c_uint = 1 << 14;
pub const MODE_PIN15: libc::c_uint = 1 << 15;

// C macros preserved as Rust macros where their external dependencies are defined elsewhere.
#[macro_export]
macro_rules! GET_UNALIGN_CTL { ($tsk:expr, $addr:expr) => { get_unalign_ctl($tsk, $addr) }; }
#[macro_export]
macro_rules! SET_UNALIGN_CTL { ($tsk:expr, $val:expr) => { set_unalign_ctl($tsk, $val) }; }
#[cfg(not(CONFIG_VSYSCALL))]
#[macro_export]
macro_rules! vsyscall_init { () => { () }; }
#[cfg(not(CONFIG_CPU_SH2A))]
#[macro_export]
macro_rules! instruction_size { ($insn:expr) => { 2 }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
