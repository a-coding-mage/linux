// C conditional dependency: CONFIG_X86_32 includes <asm/pgtable_32_areas.h>.

/* Single page reserved for the readonly IDT mapping: */
pub const CPU_ENTRY_AREA_RO_IDT: usize = CPU_ENTRY_AREA_BASE;
pub const CPU_ENTRY_AREA_PER_CPU: usize =
    CPU_ENTRY_AREA_RO_IDT + PAGE_SIZE;

pub const CPU_ENTRY_AREA_RO_IDT_VADDR: *mut core::ffi::c_void =
    CPU_ENTRY_AREA_RO_IDT as *mut core::ffi::c_void;

// Build-time condition preserved from CONFIG_X86_32.
#[cfg(CONFIG_X86_32)]
pub const CPU_ENTRY_AREA_MAP_SIZE: usize =
    CPU_ENTRY_AREA_PER_CPU + (CPU_ENTRY_AREA_SIZE * NR_CPUS) - CPU_ENTRY_AREA_BASE;

#[cfg(not(CONFIG_X86_32))]
pub const CPU_ENTRY_AREA_MAP_SIZE: usize = P4D_SIZE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
