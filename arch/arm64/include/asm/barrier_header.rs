/* SPDX-License-Identifier: GPL-2.0-only */
/* Based on arch/arm/include/asm/barrier.h */

/* C header dependencies and build-time configuration are supplied externally. */

#[macro_export]
macro_rules! __nops { ($n:expr) => { concat!(".rept ", stringify!($n), "\nnop\n.endr\n") }; }
#[macro_export]
macro_rules! nops { ($n:expr) => {{ unsafe { core::arch::asm!(concat!(".rept ", stringify!($n), "\nnop\n.endr\n"), options(nostack, preserves_flags)); } }}; }

#[macro_export] macro_rules! sev { () => {{ unsafe { core::arch::asm!("sev", options(nostack)); } }}; }
#[macro_export] macro_rules! wfe { () => {{ unsafe { core::arch::asm!("wfe", options(nostack)); } }}; }
#[macro_export] macro_rules! wfet { ($val:expr) => {{ unsafe { core::arch::asm!("msr s0_3_c1_c0_0, {0}", in(reg) $val, options(nostack)); } }}; }
#[macro_export] macro_rules! wfi { () => {{ unsafe { core::arch::asm!("wfi", options(nostack)); } }}; }
#[macro_export] macro_rules! wfit { ($val:expr) => {{ unsafe { core::arch::asm!("msr s0_3_c1_c0_1, {0}", in(reg) $val, options(nostack)); } }}; }

#[macro_export] macro_rules! isb { () => {{ unsafe { core::arch::asm!("isb", options(nostack)); } }}; }
#[macro_export] macro_rules! dmb { ($opt:ident) => {{ unsafe { core::arch::asm!(concat!("dmb ", stringify!($opt)), options(nostack)); } }}; }
#[macro_export] macro_rules! dsb { ($opt:ident) => {{ unsafe { core::arch::asm!(concat!("dsb ", stringify!($opt)), options(nostack)); } }}; }
#[macro_export] macro_rules! psb_csync { () => {{ unsafe { core::arch::asm!("hint #17", options(nostack)); } }}; }
#[macro_export] macro_rules! __tsb_csync { () => {{ unsafe { core::arch::asm!("hint #18", options(nostack)); } }}; }
#[macro_export] macro_rules! csdb { () => {{ unsafe { core::arch::asm!("hint #20", options(nostack)); } }}; }
#[macro_export] macro_rules! dgh { () => {{ unsafe { core::arch::asm!("hint #6", options(nostack)); } }}; }

#[macro_export] macro_rules! spec_bar { () => {{ unsafe { core::arch::asm!("dsb nsh\nisb", options(nostack)); } }}; }
#[macro_export] macro_rules! gsb_ack { () => {{ unsafe { core::arch::asm!("gsb_ack", options(nostack)); } }}; }
#[macro_export] macro_rules! gsb_sys { () => {{ unsafe { core::arch::asm!("gsb_sys", options(nostack)); } }}; }

#[cfg(feature = "CONFIG_ARM64_PSEUDO_NMI")]
#[macro_export] macro_rules! pmr_sync { () => {{ unsafe { core::arch::asm!("dsb sy", options(nostack)); } }}; }
#[cfg(not(feature = "CONFIG_ARM64_PSEUDO_NMI"))]
#[macro_export] macro_rules! pmr_sync { () => {{}}; }

#[macro_export] macro_rules! __mb { () => { $crate::dsb!(sy) }; }
#[macro_export] macro_rules! __rmb { () => { $crate::dsb!(ld) }; }
#[macro_export] macro_rules! __wmb { () => { $crate::dsb!(st) }; }
#[macro_export] macro_rules! __dma_mb { () => { $crate::dmb!(osh) }; }
#[macro_export] macro_rules! __dma_rmb { () => { $crate::dmb!(oshld) }; }
#[macro_export] macro_rules! __dma_wmb { () => { $crate::dmb!(oshst) }; }
#[macro_export] macro_rules! io_stop_wc { () => { $crate::dgh!() }; }

#[macro_export]
macro_rules! tsb_csync {
    () => {{
        /* Errata workaround capability is supplied externally. */
        $crate::__tsb_csync!();
    }};
}

#[inline]
pub unsafe fn array_index_mask_nospec(idx: usize, sz: usize) -> usize {
    let mask: usize;
    core::arch::asm!("cmp {1}, {2}\nsbc {0}, xzr, xzr", out(reg) mask, in(reg) idx, in(reg) sz, options(nostack));
    csdb!();
    mask
}

#[macro_export]
macro_rules! arch_counter_enforce_ordering {
    ($val:expr) => {{
        let _val = $val;
        let mut tmp: u64;
        unsafe { core::arch::asm!("eor {0}, {1}, {1}\nadd {0}, sp, {0}\nldr xzr, [{0}]", out(reg) tmp, in(reg) _val, options(nostack)); }
        let _ = tmp;
    }};
}

#[macro_export] macro_rules! __smp_mb { () => { $crate::dmb!(ish) }; }
#[macro_export] macro_rules! __smp_rmb { () => { $crate::dmb!(ishld) }; }
#[macro_export] macro_rules! __smp_wmb { () => { $crate::dmb!(ishst) }; }

/* The following typed acquire/release operations preserve the C macro interface;
 * their architecture-specific implementations are supplied by the surrounding crate. */
#[macro_export] macro_rules! __smp_store_release { ($p:expr, $v:expr) => {{ unsafe { core::ptr::write_volatile($p, $v); } }}; }
#[macro_export] macro_rules! __smp_load_acquire { ($p:expr) => {{ unsafe { core::ptr::read_volatile($p) }}; } }
#[macro_export] macro_rules! smp_cond_load_relaxed { ($ptr:expr, $cond:expr) => {{ loop { let val = unsafe { core::ptr::read_volatile($ptr) }; if $cond { break val; } } }}; }
#[macro_export] macro_rules! smp_cond_load_acquire { ($ptr:expr, $cond:expr) => {{ loop { let val = $crate::__smp_load_acquire!($ptr); if $cond { break val; } } }}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
