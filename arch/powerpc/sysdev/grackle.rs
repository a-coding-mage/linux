// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Functions for setting up and using a MPC106 northbridge
 * Extracted from arch/powerpc/platforms/powermac/pci.c.
 *
 * Copyright (C) 2003 Benjamin Herrenschmuidt (benh@kernel.crashing.org)
 * Copyright (C) 1997 Paul Mackerras (paulus@samba.org)
 */

// External declarations supplied by the surrounding kernel sources.

const GRACKLE_PICR1_LOOPSNOOP: u32 = 0x0000_0010;

#[inline]
fn grackle_cfa(b: u32, d: u32, o: u32) -> u32 {
    0x80 | (b << 8) | (d << 16) | ((o & !3) << 24)
}

#[inline]
unsafe fn grackle_set_loop_snoop(bp: *mut pci_controller, enable: i32) {
    let mut val: u32;

    out_be32((*bp).cfg_addr, grackle_cfa(0, 0, 0xa8));
    val = in_le32((*bp).cfg_data);
    val = if enable != 0 {
        val | GRACKLE_PICR1_LOOPSNOOP
    } else {
        val & !GRACKLE_PICR1_LOOPSNOOP
    };
    out_be32((*bp).cfg_addr, grackle_cfa(0, 0, 0xa8));
    out_le32((*bp).cfg_data, val);
    let _ = in_le32((*bp).cfg_data);
}

pub unsafe fn setup_grackle(hose: *mut pci_controller) {
    setup_indirect_pci(hose, 0xfec0_0000, 0xfee0_0000, 0);
    if of_machine_is_compatible("PowerMac1,1") {
        pci_add_flags(PCI_REASSIGN_ALL_BUS);
    }
    if of_machine_is_compatible("AAPL,PowerBook1998") {
        grackle_set_loop_snoop(hose, 1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
