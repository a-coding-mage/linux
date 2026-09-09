/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2003 Christoph Hellwig (hch@lst.de)
 * Copyright (C) 1999, 2000, 04 Ralf Baechle (ralf@linux-mips.org)
 * Copyright (C) 1999, 2000 Silicon Graphics, Inc.
 */

/* Dependencies are supplied by the surrounding kernel translation. */

#[cfg(CONFIG_NUMA)]
pub unsafe fn pcibus_to_node(bus: *mut pci_bus) -> i32 {
    let bc: *mut bridge_controller = BRIDGE_CONTROLLER(bus);

    (*bc).nasid
}

#[cfg(CONFIG_NUMA)]
// EXPORT_SYMBOL(pcibus_to_node);

unsafe fn ip29_fixup_phy(dev: *mut pci_dev) {
    let nasid: i32 = pcibus_to_node((*dev).bus);
    let mut sid: u32 = 0;

    if nasid != 1 {
        return; /* only needed on second module */
    }

    /* enable ethernet PHY on IP29 systemboard */
    pci_read_config_dword(dev, PCI_SUBSYSTEM_VENDOR_ID, &mut sid);
    if sid == (PCI_VENDOR_ID_SGI | (IOC3_SUBSYS_IP29_SYSBOARD << 16)) {
        REMOTE_HUB_S(nasid, MD_LED0, 0x09);
    }
}

// DECLARE_PCI_FIXUP_FINAL(PCI_VENDOR_ID_SGI, PCI_DEVICE_ID_SGI_IOC3,
//                         ip29_fixup_phy);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
