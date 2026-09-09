// Translated from the C header asm/intel_ds.h.
// The original include supplies PAGE_SIZE and related kernel definitions.

pub const BTS_BUFFER_SIZE: usize = PAGE_SIZE << 4;
pub const PEBS_BUFFER_SHIFT: usize = 4;
pub const PEBS_BUFFER_SIZE: usize = PAGE_SIZE << PEBS_BUFFER_SHIFT;

/*
 * The largest PEBS record could consume a page, ensure
 * a record at least can be written after triggering PMI.
 */
pub const ARCH_PEBS_THRESH_MULTI: usize = (PEBS_BUFFER_SIZE - PAGE_SIZE) >> PEBS_BUFFER_SHIFT;
pub const ARCH_PEBS_THRESH_SINGLE: usize = 1;

/* The maximal number of PEBS events: */
pub const MAX_PEBS_EVENTS_FMT4: usize = 8;
pub const MAX_PEBS_EVENTS: usize = 32;
pub const MAX_PEBS_EVENTS_MASK: u64 = (1u64 << MAX_PEBS_EVENTS) - 1;
pub const MAX_FIXED_PEBS_EVENTS: usize = 16;

/*
 * A debug store configuration.
 *
 * We only support architectures that use 64bit fields.
 *
 * The C declaration is __aligned(PAGE_SIZE); PAGE_SIZE is expected to be
 * provided by the surrounding kernel bindings. Rust repr(C) preserves the
 * field layout; the alignment requirement is retained as an explicit note
 * because repr(align(...)) requires a literal alignment value.
 */
#[repr(C)]
pub struct debug_store {
    pub bts_buffer_base: u64,
    pub bts_index: u64,
    pub bts_absolute_maximum: u64,
    pub bts_interrupt_threshold: u64,
    pub pebs_buffer_base: u64,
    pub pebs_index: u64,
    pub pebs_absolute_maximum: u64,
    pub pebs_interrupt_threshold: u64,
    pub pebs_event_reset: [u64; MAX_PEBS_EVENTS + MAX_FIXED_PEBS_EVENTS],
}

// DECLARE_PER_CPU_PAGE_ALIGNED(struct debug_store, cpu_debug_store);
// Declaration supplied by the surrounding per-CPU bindings.
extern "C" {
    pub static mut cpu_debug_store: debug_store;
}

#[repr(C)]
pub struct debug_store_buffers {
    pub bts_buffer: [core::ffi::c_char; BTS_BUFFER_SIZE],
    pub pebs_buffer: [core::ffi::c_char; PEBS_BUFFER_SIZE],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
