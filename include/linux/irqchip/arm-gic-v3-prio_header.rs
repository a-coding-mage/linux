/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * GIC priorities from the view of the PMR/RPR.
 *
 * These values are chosen to be valid in either the absolute priority space or
 * the NS view of the priority space. The value programmed into the distributor
 * and ITS will be chosen at boot time such that these values appear in the
 * PMR/RPR.
 *
 * GICV3_PRIO_UNMASKED is the PMR view of the priority to use to permit both
 * IRQs and pseudo-NMIs.
 *
 * GICV3_PRIO_IRQ is the PMR view of the priority of regular interrupts. This
 * can be written to the PMR to mask regular IRQs.
 *
 * GICV3_PRIO_NMI is the PMR view of the priority of pseudo-NMIs. This can be
 * written to the PMR to mask pseudo-NMIs.
 *
 * On arm64 some code sections either automatically switch back to PSR.I or
 * explicitly require to not use priority masking. If bit GICV3_PRIO_PSR_I_SET
 * is included in the priority mask, it indicates that PSR.I should be set and
 * interrupt disabling temporarily does not rely on IRQ priorities.
 */
pub const GICV3_PRIO_UNMASKED: u32 = 0xe0;
pub const GICV3_PRIO_IRQ: u32 = 0xc0;
pub const GICV3_PRIO_NMI: u32 = 0x80;

pub const GICV3_PRIO_PSR_I_SET: u32 = 1 << 4;

pub const fn __gicv3_prio_to_ns(p: u32) -> u32 {
    0xff & (p << 1)
}

pub const fn __gicv3_ns_to_prio(ns: u32) -> u32 {
    0x80 | (ns >> 1)
}

pub const fn __gicv3_prio_valid_ns(p: u32) -> bool {
    __gicv3_ns_to_prio(__gicv3_prio_to_ns(p)) == p
}

const _: () = assert!(__gicv3_prio_valid_ns(GICV3_PRIO_NMI));
const _: () = assert!(__gicv3_prio_valid_ns(GICV3_PRIO_IRQ));

const _: () = assert!(GICV3_PRIO_NMI < GICV3_PRIO_IRQ);
const _: () = assert!(GICV3_PRIO_IRQ < GICV3_PRIO_UNMASKED);

const _: () = assert!(GICV3_PRIO_IRQ < (GICV3_PRIO_IRQ | GICV3_PRIO_PSR_I_SET));

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
