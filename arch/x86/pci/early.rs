// SPDX-License-Identifier: GPL-2.0
//
// Direct PCI access. This is used for PCI accesses in early boot before
// the PCI subsystem works.

extern "C" {
    fn outl(value: u32, port: u16);
    fn inl(port: u16) -> u32;
    fn outb(value: u8, port: u16);
    fn inb(port: u16) -> u8;
    fn outw(value: u16, port: u16);
    fn inw(port: u16) -> u16;

    static mut pci_probe: u32;
}

extern "C" {
    static PCI_PROBE_CONF1: u32;
    static PCI_PROBE_NOEARLY: u32;
}

pub unsafe fn read_pci_config(bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
    let mut v: u32;
    outl(
        0x80000000u32
            | ((bus as u32) << 16)
            | ((slot as u32) << 11)
            | ((func as u32) << 8)
            | offset as u32,
        0xcf8,
    );
    v = inl(0xcfc);
    v
}

pub unsafe fn read_pci_config_byte(bus: u8, slot: u8, func: u8, offset: u8) -> u8 {
    let mut v: u8;
    outl(
        0x80000000u32
            | ((bus as u32) << 16)
            | ((slot as u32) << 11)
            | ((func as u32) << 8)
            | offset as u32,
        0xcf8,
    );
    v = inb(0xcfcu16 + (offset & 3) as u16);
    v
}

pub unsafe fn read_pci_config_16(bus: u8, slot: u8, func: u8, offset: u8) -> u16 {
    let mut v: u16;
    outl(
        0x80000000u32
            | ((bus as u32) << 16)
            | ((slot as u32) << 11)
            | ((func as u32) << 8)
            | offset as u32,
        0xcf8,
    );
    v = inw(0xcfcu16 + (offset & 2) as u16);
    v
}

pub unsafe fn write_pci_config(bus: u8, slot: u8, func: u8, offset: u8, val: u32) {
    outl(
        0x80000000u32
            | ((bus as u32) << 16)
            | ((slot as u32) << 11)
            | ((func as u32) << 8)
            | offset as u32,
        0xcf8,
    );
    outl(val, 0xcfc);
}

pub unsafe fn write_pci_config_byte(bus: u8, slot: u8, func: u8, offset: u8, val: u8) {
    outl(
        0x80000000u32
            | ((bus as u32) << 16)
            | ((slot as u32) << 11)
            | ((func as u32) << 8)
            | offset as u32,
        0xcf8,
    );
    outb(val, 0xcfcu16 + (offset & 3) as u16);
}

pub unsafe fn write_pci_config_16(bus: u8, slot: u8, func: u8, offset: u8, val: u16) {
    outl(
        0x80000000u32
            | ((bus as u32) << 16)
            | ((slot as u32) << 11)
            | ((func as u32) << 8)
            | offset as u32,
        0xcf8,
    );
    outw(val, 0xcfcu16 + (offset & 2) as u16);
}

pub unsafe fn early_pci_allowed() -> i32 {
    ((pci_probe & (PCI_PROBE_CONF1 | PCI_PROBE_NOEARLY)) == PCI_PROBE_CONF1) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
