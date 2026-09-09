/* SPDX-License-Identifier: GPL-2.0 */
// C header guard: __ASM_POWERPC_LIBATA_PORTMAP_H

macro_rules! ATA_PRIMARY_IRQ {
    ($dev:expr) => {
        pci_get_legacy_ide_irq($dev, 0)
    };
}

macro_rules! ATA_SECONDARY_IRQ {
    ($dev:expr) => {
        pci_get_legacy_ide_irq($dev, 1)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
