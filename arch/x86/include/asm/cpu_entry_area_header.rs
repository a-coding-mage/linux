/* SPDX-License-Identifier: GPL-2.0 */

/* C dependencies: linux/percpu-defs.h, asm/processor.h, asm/intel_ds.h,
 * and asm/pgtable_areas.h provide the referenced types and operations. */

#[cfg(target_arch = "x86_64")]
pub const VC_EXCEPTION_STKSZ: usize = {
    #[cfg(feature = "CONFIG_AMD_MEM_ENCRYPT")]
    {
        EXCEPTION_STKSZ
    }
    #[cfg(not(feature = "CONFIG_AMD_MEM_ENCRYPT"))]
    {
        0
    }
};

/* Macro to enforce the same ordering and stack sizes. */
#[macro_export]
macro_rules! ESTACKS_MEMBERS {
    ($guardsize:expr, $optional_stack_size:expr) => {
        pub DF_stack_guard: [core::ffi::c_char; $guardsize],
        pub DF_stack: [core::ffi::c_char; EXCEPTION_STKSZ],
        pub NMI_stack_guard: [core::ffi::c_char; $guardsize],
        pub NMI_stack: [core::ffi::c_char; EXCEPTION_STKSZ],
        pub DB_stack_guard: [core::ffi::c_char; $guardsize],
        pub DB_stack: [core::ffi::c_char; EXCEPTION_STKSZ],
        pub MCE_stack_guard: [core::ffi::c_char; $guardsize],
        pub MCE_stack: [core::ffi::c_char; EXCEPTION_STKSZ],
        pub VC_stack_guard: [core::ffi::c_char; $guardsize],
        pub VC_stack: [core::ffi::c_char; $optional_stack_size],
        pub VC2_stack_guard: [core::ffi::c_char; $guardsize],
        pub VC2_stack: [core::ffi::c_char; $optional_stack_size],
        pub IST_top_guard: [core::ffi::c_char; $guardsize],
    };
}

/* The exception stacks' physical storage. No guard pages required. */
#[cfg(target_arch = "x86_64")]
#[repr(C)]
pub struct exception_stacks {
    ESTACKS_MEMBERS!(0, VC_EXCEPTION_STKSZ);
}

/* The effective cpu entry area mapping with guard pages. */
#[cfg(target_arch = "x86_64")]
#[repr(C)]
pub struct cea_exception_stacks {
    ESTACKS_MEMBERS!(PAGE_SIZE, EXCEPTION_STKSZ);
}

/* The exception stack ordering in [cea_]exception_stacks. */
#[repr(C)]
pub enum exception_stack_ordering {
    ESTACK_DF,
    ESTACK_NMI,
    ESTACK_DB,
    ESTACK_MCE,
    ESTACK_VC,
    ESTACK_VC2,
    N_EXCEPTION_STACKS,
}

#[cfg(target_arch = "x86_64")]
#[macro_export]
macro_rules! CEA_ESTACK_SIZE {
    ($st:ident) => {
        core::mem::size_of_val(unsafe { &(*(core::ptr::null::<cea_exception_stacks>())).$st##_stack })
    };
}

#[cfg(target_arch = "x86_64")]
#[macro_export]
macro_rules! CEA_ESTACK_BOT {
    ($ceastp:expr, $st:ident) => {
        (&unsafe { (*$ceastp).$st##_stack } as *const _ as usize)
    };
}

#[cfg(target_arch = "x86_64")]
#[macro_export]
macro_rules! CEA_ESTACK_TOP {
    ($ceastp:expr, $st:ident) => {
        CEA_ESTACK_BOT!($ceastp, $st) + CEA_ESTACK_SIZE!($st)
    };
}

#[cfg(target_arch = "x86_64")]
#[macro_export]
macro_rules! CEA_ESTACK_OFFS {
    ($st:ident) => {
        core::mem::offset_of!(cea_exception_stacks, $st##_stack)
    };
}

#[cfg(target_arch = "x86_64")]
pub const CEA_ESTACK_PAGES: usize = core::mem::size_of::<cea_exception_stacks>() / PAGE_SIZE;

#[cfg(target_arch = "x86")]
#[repr(C, align(4096))]
pub struct doublefault_stack {
    pub stack: [core::ffi::c_ulong; (PAGE_SIZE - core::mem::size_of::<x86_hw_tss>()) / core::mem::size_of::<core::ffi::c_ulong>()],
    pub tss: x86_hw_tss,
}

/* cpu_entry_area is a percpu region that contains things needed by the CPU
 * and early entry/exit code. Real types aren't used for all fields here to
 * avoid circular header dependencies.
 *
 * Every field is a virtual alias of some other allocated backing store.
 * There is no direct allocation of a struct cpu_entry_area. */
#[repr(C)]
pub struct cpu_entry_area {
    pub gdt: [core::ffi::c_char; PAGE_SIZE],
    #[cfg(target_arch = "x86")]
    pub guard_entry_stack: [core::ffi::c_char; PAGE_SIZE],
    pub entry_stack_page: entry_stack_page,
    #[cfg(target_arch = "x86")]
    pub guard_doublefault_stack: [core::ffi::c_char; PAGE_SIZE],
    #[cfg(target_arch = "x86")]
    pub doublefault_stack: doublefault_stack,
    pub tss: tss_struct,
    #[cfg(target_arch = "x86_64")]
    pub estacks: cea_exception_stacks,
    pub cpu_debug_store: debug_store,
    pub cpu_debug_buffers: debug_store_buffers,
}

pub const CPU_ENTRY_AREA_SIZE: usize = core::mem::size_of::<cpu_entry_area>();

/* DECLARE_PER_CPU(struct cpu_entry_area *, cpu_entry_area); */
/* DECLARE_PER_CPU(struct cea_exception_stacks *, cea_exception_stacks); */

extern "C" {
    pub fn setup_cpu_entry_areas();
    pub fn cea_set_pte(cea_vaddr: *mut core::ffi::c_void, pa: phys_addr_t, flags: pgprot_t);
    pub fn get_cpu_entry_area(cpu: core::ffi::c_int) -> *mut cpu_entry_area;
}

#[inline(always)]
pub unsafe fn cpu_entry_stack(cpu: core::ffi::c_int) -> *mut entry_stack {
    &mut (*get_cpu_entry_area(cpu)).entry_stack_page.stack
}

/* __this_cpu_ist_top_va(name) and __this_cpu_ist_bottom_va(name) retain their
 * per-CPU read semantics and expand through CEA_ESTACK_TOP/BOT in the kernel. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
