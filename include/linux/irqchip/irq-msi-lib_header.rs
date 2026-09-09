// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2022 Linutronix GmbH
// Copyright (C) 2022 Intel

// Dependency intent from the original header:
// linux/bits.h, linux/irqdomain.h, and linux/msi.h

// The original CONFIG_PCI_MSI conditional is preserved here as a Rust cfg.
#[cfg(CONFIG_PCI_MSI)]
pub const MATCH_PCI_MSI: u32 = 1u32 << DOMAIN_BUS_PCI_MSI;

#[cfg(not(CONFIG_PCI_MSI))]
pub const MATCH_PCI_MSI: u32 = 0;

pub const MATCH_PLATFORM_MSI: u32 = 1u32 << DOMAIN_BUS_PLATFORM_MSI;

// Opaque declarations supplied by the corresponding kernel dependencies.
#[repr(C)]
pub struct msi_domain_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_domain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_fwspec {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

pub type irq_domain_bus_token = u32;

unsafe extern "C" {
    pub fn msi_lib_irq_domain_select(
        d: *mut irq_domain,
        fwspec: *mut irq_fwspec,
        bus_token: irq_domain_bus_token,
    ) -> i32;

    pub fn msi_lib_init_dev_msi_info(
        dev: *mut device,
        domain: *mut irq_domain,
        real_parent: *mut irq_domain,
        info: *mut msi_domain_info,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
