/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2007 Aurelien Jarno <aurelien@aurel32.net>
 */

// C dependencies:
// #include <linux/ssb/ssb.h>
// #include <linux/bcma/bcma.h>
// #include <linux/bcma/bcma_soc.h>
// #include <linux/bcm47xx_nvram.h>
// #include <linux/bcm47xx_sprom.h>

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum bcm47xx_bus_type {
    // Preserved from CONFIG_BCM47XX_SSB.
    #[cfg(feature = "CONFIG_BCM47XX_SSB")]
    BCM47XX_BUS_TYPE_SSB,
    // Preserved from CONFIG_BCM47XX_BCMA.
    #[cfg(feature = "CONFIG_BCM47XX_BCMA")]
    BCM47XX_BUS_TYPE_BCMA,
}

#[repr(C)]
pub union bcm47xx_bus {
    // Preserved from CONFIG_BCM47XX_SSB.
    #[cfg(feature = "CONFIG_BCM47XX_SSB")]
    pub ssb: core::mem::ManuallyDrop<ssb_bus>,
    // Preserved from CONFIG_BCM47XX_BCMA.
    #[cfg(feature = "CONFIG_BCM47XX_BCMA")]
    pub bcma: core::mem::ManuallyDrop<bcma_soc>,
}

unsafe extern "C" {
    pub static mut bcm47xx_bus: bcm47xx_bus;
    pub static mut bcm47xx_bus_type: bcm47xx_bus_type;

    pub fn bcm47xx_set_system_type(chip_id: u16);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
