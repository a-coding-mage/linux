/* SPDX-License-Identifier: GPL-2.0 */

// The original C header includes linux/atomic.h and linux/linkage.h.

/*
 * This should be the same as the max(NUM_X_SOURCES) for all the
 * different m68k hosts compiled into the kernel.
 * Currently the Atari has 72 and the Amiga 24, but if both are
 * supported in the kernel it is better to make room for 72.
 * With EtherNAT add-on card on Atari, the highest interrupt
 * number is 140 so NR_IRQS needs to be 141.
 */
#[cfg(feature = "CONFIG_COLDFIRE")]
pub const NR_IRQS: usize = 256;
#[cfg(all(
    not(feature = "CONFIG_COLDFIRE"),
    any(
        feature = "CONFIG_VME",
        feature = "CONFIG_SUN3",
        feature = "CONFIG_SUN3X",
        feature = "CONFIG_VIRT"
    )
))]
pub const NR_IRQS: usize = 200;
#[cfg(all(
    not(feature = "CONFIG_COLDFIRE"),
    not(any(feature = "CONFIG_VME", feature = "CONFIG_SUN3", feature = "CONFIG_SUN3X", feature = "CONFIG_VIRT")),
    feature = "CONFIG_ATARI"
))]
pub const NR_IRQS: usize = 141;
#[cfg(all(not(feature = "CONFIG_COLDFIRE"), not(any(feature = "CONFIG_VME", feature = "CONFIG_SUN3", feature = "CONFIG_SUN3X", feature = "CONFIG_VIRT")), not(feature = "CONFIG_ATARI"), feature = "CONFIG_MAC"))]
pub const NR_IRQS: usize = 72;
#[cfg(all(not(feature = "CONFIG_COLDFIRE"), not(any(feature = "CONFIG_VME", feature = "CONFIG_SUN3", feature = "CONFIG_SUN3X", feature = "CONFIG_VIRT")), not(feature = "CONFIG_ATARI"), not(feature = "CONFIG_MAC"), feature = "CONFIG_Q40"))]
pub const NR_IRQS: usize = 43;
#[cfg(all(not(feature = "CONFIG_COLDFIRE"), not(any(feature = "CONFIG_VME", feature = "CONFIG_SUN3", feature = "CONFIG_SUN3X", feature = "CONFIG_VIRT")), not(feature = "CONFIG_ATARI"), not(feature = "CONFIG_MAC"), not(feature = "CONFIG_Q40"), any(feature = "CONFIG_AMIGA", not(feature = "CONFIG_MMU"))))]
pub const NR_IRQS: usize = 32;
#[cfg(all(not(feature = "CONFIG_COLDFIRE"), not(any(feature = "CONFIG_VME", feature = "CONFIG_SUN3", feature = "CONFIG_SUN3X", feature = "CONFIG_VIRT")), not(feature = "CONFIG_ATARI"), not(feature = "CONFIG_MAC"), not(feature = "CONFIG_Q40"), not(any(feature = "CONFIG_AMIGA", not(feature = "CONFIG_MMU"))), feature = "CONFIG_APOLLO"))]
pub const NR_IRQS: usize = 24;
#[cfg(all(not(feature = "CONFIG_COLDFIRE"), not(any(feature = "CONFIG_VME", feature = "CONFIG_SUN3", feature = "CONFIG_SUN3X", feature = "CONFIG_VIRT")), not(feature = "CONFIG_ATARI"), not(feature = "CONFIG_MAC"), not(feature = "CONFIG_Q40"), not(any(feature = "CONFIG_AMIGA", not(feature = "CONFIG_MMU"))), not(feature = "CONFIG_APOLLO"))]
pub const NR_IRQS: usize = 8;

#[repr(C)]
pub struct irq_data;
#[repr(C)]
pub struct irq_chip;
#[repr(C)]
pub struct irq_desc;
#[repr(C)]
pub struct pt_regs;
pub type atomic_t = core::ffi::c_int;

pub const IRQ_SPURIOUS: u32 = 0;
pub const IRQ_AUTO_1: u32 = 1; /* level 1 interrupt */
pub const IRQ_AUTO_2: u32 = 2; /* level 2 interrupt */
pub const IRQ_AUTO_3: u32 = 3; /* level 3 interrupt */
pub const IRQ_AUTO_4: u32 = 4; /* level 4 interrupt */
pub const IRQ_AUTO_5: u32 = 5; /* level 5 interrupt */
pub const IRQ_AUTO_6: u32 = 6; /* level 6 interrupt */
pub const IRQ_AUTO_7: u32 = 7; /* level 7 interrupt (non-maskable) */
pub const IRQ_USER: u32 = 8;

#[cfg(any(feature = "CONFIG_M68020", feature = "CONFIG_M68030", feature = "CONFIG_M68040", feature = "CONFIG_M68060"))]
extern "C" {
    pub fn m68k_irq_startup(data: *mut irq_data) -> u32;
    pub fn m68k_irq_startup_irq(irq: u32) -> u32;
    pub fn m68k_irq_shutdown(data: *mut irq_data);
    pub fn m68k_setup_auto_interrupt(handler: Option<unsafe extern "C" fn(u32, *mut pt_regs)>);
    pub fn m68k_setup_user_interrupt(vec: u32, cnt: u32);
    pub fn m68k_setup_irq_controller(chip: *mut irq_chip, handle: Option<unsafe extern "C" fn(*mut irq_desc)>, irq: u32, cnt: u32);
    pub fn irq_canonicalize(irq: u32) -> u32;
}

#[cfg(not(any(feature = "CONFIG_M68020", feature = "CONFIG_M68030", feature = "CONFIG_M68040", feature = "CONFIG_M68060")))]
#[inline]
pub const fn irq_canonicalize(irq: u32) -> u32 { irq }

// The C declaration uses the asmlinkage calling convention.
extern "C" {
    pub fn do_IRQ(irq: core::ffi::c_int, regs: *mut pt_regs);
    pub static mut irq_err_count: atomic_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
