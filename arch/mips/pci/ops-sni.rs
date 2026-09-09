/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * SNI specific PCI support for RM200/RM300.
 *
 * Copyright (C) 1997 - 2000, 2003 Ralf Baechle <ralf@linux-mips.org>
 */

/* Kernel and architecture headers from the C source provide these names. */

unsafe fn set_config_address(busno: u32, devfn: u32, reg: i32) -> i32 {
    if devfn > 255 || reg > 255 {
        return PCIBIOS_BAD_REGISTER_NUMBER;
    }

    if busno == 0 && devfn >= PCI_DEVFN(8, 0) {
        return PCIBIOS_DEVICE_NOT_FOUND;
    }

    core::ptr::write_volatile(
        PCIMT_CONFIG_ADDRESS as *mut u32,
        ((busno & 0xff) << 16) | ((devfn & 0xff) << 8) | ((reg as u32) & 0xfc),
    );

    PCIBIOS_SUCCESSFUL
}

unsafe fn pcimt_read(
    bus: *mut pci_bus,
    devfn: u32,
    reg: i32,
    size: i32,
    val: *mut u32,
) -> i32 {
    let res = set_config_address((*bus).number, devfn, reg);
    if res != 0 {
        return res;
    }

    match size {
        1 => *val = inb(PCIMT_CONFIG_DATA + ((reg as u32) & 3)),
        2 => *val = inw(PCIMT_CONFIG_DATA + ((reg as u32) & 2)),
        4 => *val = inl(PCIMT_CONFIG_DATA),
        _ => {}
    }

    0
}

unsafe fn pcimt_write(
    bus: *mut pci_bus,
    devfn: u32,
    reg: i32,
    size: i32,
    val: u32,
) -> i32 {
    let res = set_config_address((*bus).number, devfn, reg);
    if res != 0 {
        return res;
    }

    match size {
        1 => outb(val, PCIMT_CONFIG_DATA + ((reg as u32) & 3)),
        2 => outw(val, PCIMT_CONFIG_DATA + ((reg as u32) & 2)),
        4 => outl(val, PCIMT_CONFIG_DATA),
        _ => {}
    }

    0
}

pub static mut sni_pcimt_ops: pci_ops = pci_ops {
    read: pcimt_read,
    write: pcimt_write,
};

unsafe fn pcit_set_config_address(busno: u32, devfn: u32, reg: i32) -> i32 {
    if devfn > 255 || reg > 255 || busno > 255 {
        return PCIBIOS_BAD_REGISTER_NUMBER;
    }

    outl(
        (1 << 31) | ((busno & 0xff) << 16) | ((devfn & 0xff) << 8) | ((reg as u32) & 0xfc),
        0xcf8,
    );
    PCIBIOS_SUCCESSFUL
}

unsafe fn pcit_read(
    bus: *mut pci_bus,
    devfn: u32,
    reg: i32,
    size: i32,
    val: *mut u32,
) -> i32 {
    if (*bus).number == 0 {
        pcit_set_config_address(0, 0, 0x68);
        outl(inl(0xcfc) | 0xc0000000, 0xcfc);
        let res = pcit_set_config_address(0, devfn, 0);
        if res != 0 {
            return res;
        }
        outl(0xffffffff, 0xcfc);
        pcit_set_config_address(0, 0, 0x68);
        if inl(0xcfc) & 0x100000 != 0 {
            return PCIBIOS_DEVICE_NOT_FOUND;
        }
    }

    let res = pcit_set_config_address((*bus).number, devfn, reg);
    if res != 0 {
        return res;
    }

    match size {
        1 => *val = inb(PCIMT_CONFIG_DATA + ((reg as u32) & 3)),
        2 => *val = inw(PCIMT_CONFIG_DATA + ((reg as u32) & 2)),
        4 => *val = inl(PCIMT_CONFIG_DATA),
        _ => {}
    }
    0
}

unsafe fn pcit_write(
    bus: *mut pci_bus,
    devfn: u32,
    reg: i32,
    size: i32,
    val: u32,
) -> i32 {
    let res = pcit_set_config_address((*bus).number, devfn, reg);
    if res != 0 {
        return res;
    }

    match size {
        1 => outb(val, PCIMT_CONFIG_DATA + ((reg as u32) & 3)),
        2 => outw(val, PCIMT_CONFIG_DATA + ((reg as u32) & 2)),
        4 => outl(val, PCIMT_CONFIG_DATA),
        _ => {}
    }

    0
}

pub static mut sni_pcit_ops: pci_ops = pci_ops {
    read: pcit_read,
    write: pcit_write,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
