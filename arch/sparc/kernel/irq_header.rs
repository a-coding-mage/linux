/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct irq_bucket {
    pub next: *mut irq_bucket,
    pub real_irq: ::core::ffi::c_uint,
    pub irq: ::core::ffi::c_uint,
    pub pil: ::core::ffi::c_uint,
}

#[macro_export]
macro_rules! SUN4M_HARD_INT {
    ($x:expr) => { 0x000000001u64 << ($x) };
}

#[macro_export]
macro_rules! SUN4M_SOFT_INT {
    ($x:expr) => { 0x000010000u64 << ($x) };
}

pub const SUN4D_MAX_BOARD: ::core::ffi::c_int = 10;
pub const SUN4D_MAX_IRQ: ::core::ffi::c_int = (SUN4D_MAX_BOARD + 2) << 5;

/* Map between the irq identifier used in hw to the
 * irq_bucket. The map is sufficient large to hold
 * the sun4d hw identifiers.
 */
extern "C" {
    pub static mut irq_map: [*mut irq_bucket; SUN4D_MAX_IRQ as usize];
}

/* sun4m specific type definitions */

/* This maps direct to CPU specific interrupt registers */
#[repr(C)]
pub struct sun4m_irq_percpu {
    pub pending: u32,
    pub clear: u32,
    pub set: u32,
}

/* This maps direct to global interrupt registers */
#[repr(C)]
pub struct sun4m_irq_global {
    pub pending: u32,
    pub mask: u32,
    pub mask_clear: u32,
    pub mask_set: u32,
    pub interrupt_target: u32,
}

extern "C" {
    pub static mut sun4m_irq_percpu: [*mut sun4m_irq_percpu; SUN4M_NCPUS as usize];
    pub static mut sun4m_irq_global: *mut sun4m_irq_global;
}

/* The following definitions describe the individual platform features: */
pub const FEAT_L10_CLOCKSOURCE: ::core::ffi::c_int = 1 << 0; /* L10 timer is used as a clocksource */
pub const FEAT_L10_CLOCKEVENT: ::core::ffi::c_int = 1 << 1; /* L10 timer is used as a clockevent */
pub const FEAT_L14_ONESHOT: ::core::ffi::c_int = 1 << 2; /* L14 timer clockevent can oneshot */

/*
 * Platform specific configuration
 * The individual platforms assign their platform
 * specifics in their init functions.
 */
#[repr(C)]
pub struct sparc_config {
    pub init_timers: Option<unsafe extern "C" fn()>,
    pub build_device_irq: Option<unsafe extern "C" fn(
        op: *mut platform_device,
        real_irq: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_uint>,

    /* generic clockevent features - see FEAT_* above */
    pub features: ::core::ffi::c_int,

    /* clock rate used for clock event timer */
    pub clock_rate: ::core::ffi::c_int,

    /* one period for clock source timer */
    pub cs_period: ::core::ffi::c_uint,

    /* function to obtain offsett for cs period */
    pub get_cycles_offset: Option<unsafe extern "C" fn() -> ::core::ffi::c_uint>,

    pub clear_clock_irq: Option<unsafe extern "C" fn()>,
    pub load_profile_irq: Option<unsafe extern "C" fn(
        cpu: ::core::ffi::c_int,
        limit: ::core::ffi::c_uint,
    )>,
}

extern "C" {
    pub static mut sparc_config: sparc_config;

    pub fn irq_alloc(real_irq: ::core::ffi::c_uint, pil: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    pub fn irq_link(irq: ::core::ffi::c_uint);
    pub fn irq_unlink(irq: ::core::ffi::c_uint);
    pub fn handler_irq(pil: ::core::ffi::c_uint, regs: *mut pt_regs);

    pub fn leon_get_irqmask(irq: ::core::ffi::c_uint) -> ::core::ffi::c_ulong;

    /* irq_32.c */
    pub fn sparc_floppy_irq(irq: ::core::ffi::c_int, dev_id: *mut ::core::ffi::c_void, regs: *mut pt_regs);

    /* sun4m_irq.c */
    pub fn sun4m_nmi(regs: *mut pt_regs);

    /* sun4d_irq.c */
    pub fn sun4d_handler_irq(pil: ::core::ffi::c_uint, regs: *mut pt_regs);
}

#[cfg(feature = "CONFIG_SMP")]
/* All SUN4D IPIs are sent on this IRQ, may be shared with hard IRQs */
pub const SUN4D_IPI_IRQ: ::core::ffi::c_int = 13;

#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    pub fn sun4d_ipi_interrupt();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
