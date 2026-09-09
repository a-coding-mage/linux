/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies supplied by other translation units are intentionally not
// implemented here.

/* Uprobes on this MM assume 32-bit code */
pub const MM_CONTEXT_UPROBE_IA32: u32 = 0;
/* vsyscall page is accessible on this MM */
pub const MM_CONTEXT_HAS_VSYSCALL: u32 = 1;
/* Do not allow changing LAM mode */
pub const MM_CONTEXT_LOCK_LAM: u32 = 2;
/* Allow LAM and SVA coexisting */
pub const MM_CONTEXT_FORCE_TAGGED_SVA: u32 = 3;
/* Tracks mm_cpumask */
pub const MM_CONTEXT_NOTRACK: u32 = 4;

/*
 * x86 has arch-specific MMU state beyond what lives in mm_struct.
 */
#[repr(C)]
pub struct mm_context_t {
    /*
     * ctx_id uniquely identifies this mm_struct.  A ctx_id will never
     * be reused, and zero is not a valid ctx_id.
     */
    pub ctx_id: u64,

    /*
     * Any code that needs to do any sort of TLB flushing for this
     * mm will first make its changes to the page tables, then
     * increment tlb_gen, then flush.  This lets the low-level
     * flushing code keep track of what needs flushing.
     *
     * This is not used on Xen PV.
     */
    pub tlb_gen: atomic64_t,

    pub next_trim_cpumask: usize,

    #[cfg(CONFIG_MODIFY_LDT_SYSCALL)]
    pub ldt_usr_sem: rw_semaphore,
    #[cfg(CONFIG_MODIFY_LDT_SYSCALL)]
    pub ldt: *mut ldt_struct,

    pub flags: usize,

    #[cfg(CONFIG_ADDRESS_MASKING)]
    /* Active LAM mode: X86_CR3_LAM_U48 or X86_CR3_LAM_U57 or 0 (disabled) */
    pub lam_cr3_mask: usize,

    #[cfg(CONFIG_ADDRESS_MASKING)]
    /* Significant bits of the virtual address. Excludes tag bits. */
    pub untag_mask: u64,

    pub lock: mutex,
    pub vdso: *mut core::ffi::c_void, /* vdso base address */
    pub vdso_image: *const vdso_image, /* vdso image in use */

    pub perf_rdpmc_allowed: atomic_t, /* nonzero if rdpmc is allowed */
    #[cfg(CONFIG_X86_INTEL_MEMORY_PROTECTION_KEYS)]
    /*
     * One bit per protection key says whether userspace can
     * use it or not.  protected by mmap_lock.
     */
    pub pkey_allocation_map: u16,
    #[cfg(CONFIG_X86_INTEL_MEMORY_PROTECTION_KEYS)]
    pub execute_only_pkey: i16,

    #[cfg(CONFIG_BROADCAST_TLB_FLUSH)]
    /*
     * The global ASID will be a non-zero value when the process has
     * the same ASID across all CPUs, allowing it to make use of
     * hardware-assisted remote TLB invalidation like AMD INVLPGB.
     */
    pub global_asid: u16,

    #[cfg(CONFIG_BROADCAST_TLB_FLUSH)]
    /* The process is transitioning to a new global ASID number. */
    pub asid_transition: bool,
}

pub type __mm_context_t = mm_context_t;

extern "C" {
    pub fn leave_mm();
}

// #define leave_mm leave_mm


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
