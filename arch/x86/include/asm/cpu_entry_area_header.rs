/* SPDX-License-Identifier: GPL-2.0 */

/* C dependencies: linux/percpu-defs.h, asm/processor.h, asm/intel_ds.h,
 * and asm/pgtable_areas.h provide the referenced types and operations. */

#[cfg(all(CONFIG_X86_64, CONFIG_AMD_MEM_ENCRYPT))]
pub const VC_EXCEPTION_STKSZ: usize = EXCEPTION_STKSZ;
#[cfg(all(CONFIG_X86_64, not(CONFIG_AMD_MEM_ENCRYPT)))]
pub const VC_EXCEPTION_STKSZ: usize = 0;

/*
 * ESTACKS_MEMBERS(guardsize, optional_stack_size): one layout, instantiated
 * with and without guard pages, which enforces the same ordering and stack
 * sizes.
 */
#[cfg(CONFIG_X86_64)]
#[repr(C)]
pub struct estacks_members<const GUARDSIZE: usize, const OPTIONAL_STACK_SIZE: usize> {
    pub DF_stack_guard: [core::ffi::c_char; GUARDSIZE],
    pub DF_stack: [core::ffi::c_char; EXCEPTION_STKSZ],
    pub NMI_stack_guard: [core::ffi::c_char; GUARDSIZE],
    pub NMI_stack: [core::ffi::c_char; EXCEPTION_STKSZ],
    pub DB_stack_guard: [core::ffi::c_char; GUARDSIZE],
    pub DB_stack: [core::ffi::c_char; EXCEPTION_STKSZ],
    pub MCE_stack_guard: [core::ffi::c_char; GUARDSIZE],
    pub MCE_stack: [core::ffi::c_char; EXCEPTION_STKSZ],
    pub VC_stack_guard: [core::ffi::c_char; GUARDSIZE],
    pub VC_stack: [core::ffi::c_char; OPTIONAL_STACK_SIZE],
    pub VC2_stack_guard: [core::ffi::c_char; GUARDSIZE],
    pub VC2_stack: [core::ffi::c_char; OPTIONAL_STACK_SIZE],
    pub IST_top_guard: [core::ffi::c_char; GUARDSIZE],
}

/* The exception stacks' physical storage. No guard pages required */
#[cfg(CONFIG_X86_64)]
pub type exception_stacks = estacks_members<0, VC_EXCEPTION_STKSZ>;

/* The effective cpu entry area mapping with guard pages. */
#[cfg(CONFIG_X86_64)]
pub type cea_exception_stacks = estacks_members<PAGE_SIZE, EXCEPTION_STKSZ>;

/*
 * The exception stack ordering in [cea_]exception_stacks
 */
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

/// Size of a field, without forming a reference to any object.
#[doc(hidden)]
pub const fn __field_size<T, F>(_field: fn(&T) -> &F) -> usize {
    core::mem::size_of::<F>()
}

#[cfg(CONFIG_X86_64)]
#[macro_export]
macro_rules! CEA_ESTACK_SIZE {
    ($st:ident) => {
        ::kernel::macros::paste!(__field_size(|s: &cea_exception_stacks| &s.[<$st _stack>]))
    };
}

#[cfg(CONFIG_X86_64)]
#[macro_export]
macro_rules! CEA_ESTACK_BOT {
    ($ceastp:expr, $st:ident) => {
        ::kernel::macros::paste!(core::ptr::addr_of!((*$ceastp).[<$st _stack>]) as core::ffi::c_ulong)
    };
}

#[cfg(CONFIG_X86_64)]
#[macro_export]
macro_rules! CEA_ESTACK_TOP {
    ($ceastp:expr, $st:ident) => {
        CEA_ESTACK_BOT!($ceastp, $st) + CEA_ESTACK_SIZE!($st) as core::ffi::c_ulong
    };
}

#[cfg(CONFIG_X86_64)]
#[macro_export]
macro_rules! CEA_ESTACK_OFFS {
    ($st:ident) => {
        ::kernel::macros::paste!(core::mem::offset_of!(cea_exception_stacks, [<$st _stack>]))
    };
}

#[cfg(CONFIG_X86_64)]
pub const CEA_ESTACK_PAGES: usize = core::mem::size_of::<cea_exception_stacks>() / PAGE_SIZE;

#[cfg(CONFIG_X86_32)]
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
    #[cfg(CONFIG_X86_32)]
    pub guard_entry_stack: [core::ffi::c_char; PAGE_SIZE],
    pub entry_stack_page: entry_stack_page,
    #[cfg(CONFIG_X86_32)]
    pub guard_doublefault_stack: [core::ffi::c_char; PAGE_SIZE],
    #[cfg(CONFIG_X86_32)]
    pub doublefault_stack: doublefault_stack,
    pub tss: tss_struct,
    #[cfg(CONFIG_X86_64)]
    pub estacks: cea_exception_stacks,
    pub cpu_debug_store: debug_store,
    pub cpu_debug_buffers: debug_store_buffers,
}

pub const CPU_ENTRY_AREA_SIZE: usize = core::mem::size_of::<cpu_entry_area>();

/* DECLARE_PER_CPU(cpu_entry_area *, cpu_entry_area); */
/* DECLARE_PER_CPU(cea_exception_stacks *, cea_exception_stacks); */

extern "C" {
    pub fn setup_cpu_entry_areas();
    pub fn cea_set_pte(cea_vaddr: *mut core::ffi::c_void, pa: phys_addr_t, flags: pgprot_t);
    pub fn get_cpu_entry_area(cpu: core::ffi::c_int) -> *mut cpu_entry_area;
}

#[inline(always)]
pub unsafe fn cpu_entry_stack(cpu: core::ffi::c_int) -> *mut entry_stack {
    core::ptr::addr_of_mut!((*get_cpu_entry_area(cpu)).entry_stack_page.stack)
}

#[macro_export]
macro_rules! __this_cpu_ist_top_va {
    ($name:ident) => {
        CEA_ESTACK_TOP!(__this_cpu_read!(cea_exception_stacks), $name)
    };
}

#[macro_export]
macro_rules! __this_cpu_ist_bottom_va {
    ($name:ident) => {
        CEA_ESTACK_BOT!(__this_cpu_read!(cea_exception_stacks), $name)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
