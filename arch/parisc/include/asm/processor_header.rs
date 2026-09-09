/* SPDX-License-Identifier: GPL-2.0 */
/*
 * include/asm-parisc/processor.h
 *
 * Copyright (C) 1994 Linus Torvalds
 * Copyright (C) 2001 Grant Grundler
 */

/* C header dependencies are supplied by the surrounding kernel translation. */

pub const HAVE_ARCH_PICK_MMAP_LAYOUT: bool = true;

#[inline]
pub unsafe fn task_size_of(tsk: *const task_struct) -> c_ulong {
    (*tsk).thread.task_size
}

#[inline]
pub unsafe fn task_size() -> c_ulong {
    task_size_of(current)
}

#[inline]
pub unsafe fn task_unmapped_base() -> c_ulong {
    (*current).thread.map_base
}

pub const DEFAULT_TASK_SIZE32: c_ulong = 0xFFF00000;
pub const DEFAULT_MAP_BASE32: c_ulong = 0x40000000;

#[cfg(CONFIG_64BIT)]
pub const DEFAULT_TASK_SIZE: c_ulong = MAX_ADDRESS - 0x0f000000;
#[cfg(CONFIG_64BIT)]
pub const DEFAULT_MAP_BASE: c_ulong = 0x200000000;
#[cfg(not(CONFIG_64BIT))]
pub const DEFAULT_TASK_SIZE: c_ulong = DEFAULT_TASK_SIZE32;
#[cfg(not(CONFIG_64BIT))]
pub const DEFAULT_MAP_BASE: c_ulong = DEFAULT_MAP_BASE32;

/* XXX: STACK_TOP actually should be STACK_BOTTOM for parisc.
 * prumpf */
#[macro_export]
macro_rules! TASK_SIZE_OF { ($tsk:expr) => {{ unsafe { (*$tsk).thread.task_size } }}; }
#[macro_export]
macro_rules! TASK_SIZE { () => {{ unsafe { (*current).thread.task_size } }}; }
#[macro_export]
macro_rules! TASK_UNMAPPED_BASE { () => {{ unsafe { (*current).thread.map_base } }}; }
#[macro_export]
macro_rules! STACK_TOP { () => { TASK_SIZE!() }; }
pub const STACK_TOP_MAX: c_ulong = DEFAULT_TASK_SIZE;

extern "C" {
    pub fn mmap_upper_limit(rlim_stack: *const rlimit) -> c_ulong;
    pub fn calc_max_stack_size(stack_max: c_ulong) -> c_ulong;
}

/*
 * Data detected about CPUs at boot time which is the same for all CPU's.
 * HP boxes are SMP - ie identical processors.
 *
 * FIXME: some CPU rev info may be processor specific...
 */
#[repr(C)]
pub struct system_cpuinfo_parisc {
    pub cpu_count: c_uint,
    pub cpu_hz: c_uint,
    pub hversion: c_uint,
    pub sversion: c_uint,
    pub cpu_type: cpu_type,
    pub pdc: system_cpuinfo_parisc_pdc,
    pub cpu_name: *const c_char, /* e.g. "PA7300LC (PCX-L2)" */
    pub family_name: *const c_char, /* e.g. "1.1e" */
}

#[repr(C)]
pub struct system_cpuinfo_parisc_pdc {
    pub model: pdc_model,
    pub versions: c_ulong,
    pub cpuid: c_ulong,
    pub capabilities: c_ulong,
    pub sys_model_name: [c_char; 81], /* PDC-ROM returnes this model name */
}

/* Per CPU data structure - ie varies per CPU.  */
#[repr(C)]
pub struct cpuinfo_parisc {
    pub it_value: c_ulong, /* Interval Timer at last timer Intr */
    pub irq_count: c_ulong, /* number of IRQ's since boot */
    pub cpuid: c_ulong, /* aka slot_number or set to NO_PROC_ID */
    pub hpa: c_ulong, /* Host Physical address */
    pub txn_addr: c_ulong, /* MMIO addr of EIR or id_eid */
    #[cfg(CONFIG_SMP)]
    pub pending_ipi: c_ulong, /* bitmap of type ipi_message_type */
    pub bh_count: c_ulong, /* number of times bh was invoked */
    pub fp_rev: c_ulong,
    pub fp_model: c_ulong,
    pub cpu_num: c_ulong, /* CPU number from PAT firmware */
    pub cpu_loc: c_ulong, /* CPU location from PAT firmware */
    pub state: c_uint,
    pub dev: *mut parisc_device,
}

extern "C" {
    pub static mut boot_cpu_data: system_cpuinfo_parisc;
    pub static mut time_keeper_id: c_int; /* CPU used for timekeeping */
}

#[inline]
pub unsafe fn cpu_hversion() -> c_uint {
    (boot_cpu_data.hversion >> 4) & 0x0FFF
}

#[repr(C)]
pub struct thread_struct {
    pub regs: pt_regs,
    pub task_size: c_ulong,
    pub map_base: c_ulong,
    pub flags: c_ulong,
}

#[inline]
pub unsafe fn task_pt_regs(tsk: *mut task_struct) -> *mut pt_regs {
    &mut (*tsk).thread.regs
}

/* Thread struct flags. */
pub const PARISC_UAC_NOPRINT: c_ulong = 1 << 0; /* see prctl and unaligned.c */
pub const PARISC_UAC_SIGBUS: c_ulong = 1 << 1;
pub const PARISC_KERNEL_DEATH: c_ulong = 1 << 31; /* see die_if_kernel()... */
pub const PARISC_UAC_SHIFT: c_ulong = 0;
pub const PARISC_UAC_MASK: c_ulong = PARISC_UAC_NOPRINT | PARISC_UAC_SIGBUS;

#[macro_export]
macro_rules! SET_UNALIGN_CTL {
    ($task:expr, $value:expr) => {{
        unsafe {
            (*$task).thread.flags = ((*$task).thread.flags & !PARISC_UAC_MASK)
                | ((($value as c_ulong) << PARISC_UAC_SHIFT) & PARISC_UAC_MASK);
        }
        0
    }};
}

#[macro_export]
macro_rules! GET_UNALIGN_CTL {
    ($task:expr, $addr:expr) => {{
        unsafe {
            put_user(((*$task).thread.flags & PARISC_UAC_MASK) >> PARISC_UAC_SHIFT,
                     $addr as *mut c_int);
        }
    }};
}

/* INIT_THREAD initializes regs to zero and uses the configured task/map defaults. */
pub const INIT_THREAD_TASK_SIZE: c_ulong = DEFAULT_TASK_SIZE;
pub const INIT_THREAD_MAP_BASE: c_ulong = DEFAULT_MAP_BASE;
pub const INIT_THREAD_FLAGS: c_ulong = 0;

extern "C" {
    pub fn show_trace(task: *mut task_struct, stack: *mut c_ulong);
}

/*
 * Start user thread in another space.
 *
 * Note that we set both the iaoq and r31 to the new pc. When
 * the kernel initially calls execve it will return through an
 * rfi path that will use the values in the iaoq. The execve
 * syscall path will return through the gateway page, and
 * that uses r31 to branch to.
 *
 * For ELF we clear r23, because the dynamic linker uses it to pass
 * the address of the finalizer function.
 *
 * We also initialize sr3 to an illegal value (illegal for our
 * implementation, not for the architecture).
 */
pub type elf_caddr_t = c_uint;

/* The ELF ABI and PA-RISC stack layout comments are retained from the C header. */
/*
 * The ELF abi wants things done a "wee bit" differently than som does.
 * Supporting this behavior here avoids having our own version of
 * create_elf_tables. argc is passed in r25 and argv in r24.
 * The initial stack contains the magic frame, save area, ELF auxiliary
 * data, environment and argument pointers, argc, slack, filename, and
 * strings, with the stack base at TASK_SIZE - rlim_max.
 */

#[inline]
pub unsafe fn user_wide_mode() -> bool {
    !is_32bit_task()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
