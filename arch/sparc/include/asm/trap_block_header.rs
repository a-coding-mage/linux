/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies supplied by other translation units:
// linux/threads.h, asm/hypervisor.h, asm/asi.h, asm/scratchpad.h

#[cfg(not(feature = "assembler"))]
#[repr(C, align(64))]
pub struct trap_per_cpu {
    /* D-cache line 1: Basic thread information, cpu and device mondo queues */
    pub thread: *mut thread_info,
    pub pgd_paddr: ::core::ffi::c_ulong,
    pub cpu_mondo_pa: ::core::ffi::c_ulong,
    pub dev_mondo_pa: ::core::ffi::c_ulong,

    /* D-cache line 2: Error Mondo Queue and kernel buffer pointers */
    pub resum_mondo_pa: ::core::ffi::c_ulong,
    pub resum_kernel_buf_pa: ::core::ffi::c_ulong,
    pub nonresum_mondo_pa: ::core::ffi::c_ulong,
    pub nonresum_kernel_buf_pa: ::core::ffi::c_ulong,

    /* Dcache lines 3, 4, 5, and 6: Hypervisor Fault Status */
    pub fault_info: hv_fault_status,

    /* Dcache line 7: Physical addresses of CPU send mondo block and CPU list. */
    pub cpu_mondo_block_pa: ::core::ffi::c_ulong,
    pub cpu_list_pa: ::core::ffi::c_ulong,
    pub tsb_huge: ::core::ffi::c_ulong,
    pub tsb_huge_temp: ::core::ffi::c_ulong,

    /* Dcache line 8: IRQ work list, and keep trap_block a power-of-2 in size. */
    pub irq_worklist_pa: ::core::ffi::c_ulong,
    pub cpu_mondo_qmask: ::core::ffi::c_uint,
    pub dev_mondo_qmask: ::core::ffi::c_uint,
    pub resum_qmask: ::core::ffi::c_uint,
    pub nonresum_qmask: ::core::ffi::c_uint,
    pub __per_cpu_base: ::core::ffi::c_ulong,
}

extern "C" {
    pub static mut trap_block: [trap_per_cpu; NR_CPUS];
    pub fn init_cur_cpu_trap(thread: *mut thread_info);
    pub fn setup_tba();
    pub static mut ncpus_probed: ::core::ffi::c_int;
    pub static mut cpu_mondo_counter: [u64; NR_CPUS];
    pub fn real_hard_smp_processor_id() -> ::core::ffi::c_ulong;
}

#[repr(C)]
pub struct cpuid_patch_entry {
    pub addr: ::core::ffi::c_uint,
    pub cheetah_safari: [::core::ffi::c_uint; 4],
    pub cheetah_jbus: [::core::ffi::c_uint; 4],
    pub starfire: [::core::ffi::c_uint; 4],
    pub sun4v: [::core::ffi::c_uint; 4],
}

extern "C" {
    pub static mut __cpuid_patch: cpuid_patch_entry;
    pub static mut __cpuid_patch_end: cpuid_patch_entry;
}

#[repr(C)]
pub struct sun4v_1insn_patch_entry {
    pub addr: ::core::ffi::c_uint,
    pub insn: ::core::ffi::c_uint,
}

extern "C" {
    pub static mut __sun4v_1insn_patch: sun4v_1insn_patch_entry;
    pub static mut __sun4v_1insn_patch_end: sun4v_1insn_patch_entry;
    pub static mut __fast_win_ctrl_1insn_patch: sun4v_1insn_patch_entry;
    pub static mut __fast_win_ctrl_1insn_patch_end: sun4v_1insn_patch_entry;
    pub static mut __sun_m7_1insn_patch: sun4v_1insn_patch_entry;
    pub static mut __sun_m7_1insn_patch_end: sun4v_1insn_patch_entry;
}

#[repr(C)]
pub struct sun4v_2insn_patch_entry {
    pub addr: ::core::ffi::c_uint,
    pub insns: [::core::ffi::c_uint; 2],
}

extern "C" {
    pub static mut __sun4v_2insn_patch: sun4v_2insn_patch_entry;
    pub static mut __sun4v_2insn_patch_end: sun4v_2insn_patch_entry;
    pub static mut __sun_m7_2insn_patch: sun4v_2insn_patch_entry;
    pub static mut __sun_m7_2insn_patch_end: sun4v_2insn_patch_entry;
}

pub const TRAP_PER_CPU_THREAD: usize = 0x00;
pub const TRAP_PER_CPU_PGD_PADDR: usize = 0x08;
pub const TRAP_PER_CPU_CPU_MONDO_PA: usize = 0x10;
pub const TRAP_PER_CPU_DEV_MONDO_PA: usize = 0x18;
pub const TRAP_PER_CPU_RESUM_MONDO_PA: usize = 0x20;
pub const TRAP_PER_CPU_RESUM_KBUF_PA: usize = 0x28;
pub const TRAP_PER_CPU_NONRESUM_MONDO_PA: usize = 0x30;
pub const TRAP_PER_CPU_NONRESUM_KBUF_PA: usize = 0x38;
pub const TRAP_PER_CPU_FAULT_INFO: usize = 0x40;
pub const TRAP_PER_CPU_CPU_MONDO_BLOCK_PA: usize = 0xc0;
pub const TRAP_PER_CPU_CPU_LIST_PA: usize = 0xc8;
pub const TRAP_PER_CPU_TSB_HUGE: usize = 0xd0;
pub const TRAP_PER_CPU_TSB_HUGE_TEMP: usize = 0xd8;
pub const TRAP_PER_CPU_IRQ_WORKLIST_PA: usize = 0xe0;
pub const TRAP_PER_CPU_CPU_MONDO_QMASK: usize = 0xe8;
pub const TRAP_PER_CPU_DEV_MONDO_QMASK: usize = 0xec;
pub const TRAP_PER_CPU_RESUM_QMASK: usize = 0xf0;
pub const TRAP_PER_CPU_NONRESUM_QMASK: usize = 0xf4;
pub const TRAP_PER_CPU_PER_CPU_BASE: usize = 0xf8;
pub const TRAP_BLOCK_SZ_SHIFT: u32 = 8;

// The following C preprocessor macros emit SPARC assembly and are retained here
// as source-level documentation; their assembler implementation is supplied by
// the SPARC low-level entry code. CONFIG_SMP selects the per-CPU variants.
// __GET_CPUID(REG), TRAP_LOAD_TRAP_BLOCK(DEST, TMP), TRAP_LOAD_PGD_PHYS(DEST, TMP),
// TRAP_LOAD_IRQ_WORK_PA(DEST, TMP), TRAP_LOAD_THREAD_REG(DEST, TMP), and
// LOAD_PER_CPU_BASE(DEST, THR, REG1, REG2, REG3) preserve the original macro
// interfaces and SPARC instruction sequences.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
