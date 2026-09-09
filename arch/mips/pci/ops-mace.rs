/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2000, 2001 Keith M Wesolowski
 */

// Dependencies supplied by the surrounding kernel translation.

/*
 * O2 has up to 5 PCI devices connected into the MACE bridge.  The device
 * map looks like this:
 *
 * 0  aic7xxx 0
 * 1  aic7xxx 1
 * 2  expansion slot
 * 3  N/C
 * 4  N/C
 */

#[inline]
unsafe fn mkaddr(bus: *mut pci_bus, devfn: u32, reg: u32) -> i32 {
    ((((*bus).number as u32 & 0xff) << 16)
        | ((devfn & 0xff) << 8)
        | (reg & 0xfc)) as i32
}

unsafe fn mace_pci_read_config(
    bus: *mut pci_bus,
    devfn: u32,
    reg: i32,
    size: i32,
    val: *mut u32,
) -> i32 {
    let control: u32 = (*mace).pci.control;

    /* disable master aborts interrupts during config read */
    (*mace).pci.control = control & !MACEPCI_CONTROL_MAR_INT;
    (*mace).pci.config_addr = mkaddr(bus, devfn, reg as u32);
    match size {
        1 => {
            *val = (*mace).pci.config_data.b[((reg as u32 & 3) ^ 3) as usize] as u32;
        }
        2 => {
            *val = (*mace).pci.config_data.w[(((reg as u32 >> 1) & 1) ^ 1) as usize] as u32;
        }
        4 => {
            *val = (*mace).pci.config_data.l;
        }
        _ => {}
    }
    /* ack possible master abort */
    (*mace).pci.error &= !MACEPCI_ERROR_MASTER_ABORT;
    (*mace).pci.control = control;
    /*
     * someone forgot to set the ultra bit for the onboard
     * scsi chips; we fake it here
     */
    if (*bus).number == 0
        && reg == 0x40
        && size == 4
        && (devfn == (1 << 3) || devfn == (2 << 3))
    {
        *val |= 0x1000;
    }

    // DPRINTK("read%d: reg=%08x,val=%02x\n", size * 8, reg, *val);

    PCIBIOS_SUCCESSFUL
}

unsafe fn mace_pci_write_config(
    bus: *mut pci_bus,
    devfn: u32,
    reg: i32,
    size: i32,
    val: u32,
) -> i32 {
    (*mace).pci.config_addr = mkaddr(bus, devfn, reg as u32);
    match size {
        1 => {
            (*mace).pci.config_data.b[((reg as u32 & 3) ^ 3) as usize] = val as _;
        }
        2 => {
            (*mace).pci.config_data.w[(((reg as u32 >> 1) & 1) ^ 1) as usize] = val as _;
        }
        4 => {
            (*mace).pci.config_data.l = val;
        }
        _ => {}
    }

    // DPRINTK("write%d: reg=%08x,val=%02x\n", size * 8, reg, val);

    PCIBIOS_SUCCESSFUL
}

#[no_mangle]
pub static mut mace_pci_ops: pci_ops = pci_ops {
    read: Some(mace_pci_read_config),
    write: Some(mace_pci_write_config),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
