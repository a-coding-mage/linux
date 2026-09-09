// SPDX-License-Identifier: GPL-2.0-or-later
/* PowerNV LPC bus handling. */

// Kernel and architecture dependencies are supplied by the surrounding tree.

static mut opal_lpc_chip_id: i32 = -1;

unsafe fn opal_lpc_inb(port: u64) -> u8 {
    let mut data: u32 = 0;
    if opal_lpc_chip_id < 0 || port > 0xffff { return 0xff; }
    let rc = opal_lpc_read(opal_lpc_chip_id, OPAL_LPC_IO, port, &mut data, 1);
    if rc != 0 { 0xff } else { u32::from_be(data) as u8 }
}

unsafe fn __opal_lpc_inw(port: u64) -> u16 {
    let mut data: u32 = 0;
    if opal_lpc_chip_id < 0 || port > 0xfffe { return 0xffff; }
    if port & 1 != 0 { return (opal_lpc_inb(port) as u16) << 8 | opal_lpc_inb(port + 1) as u16; }
    let rc = opal_lpc_read(opal_lpc_chip_id, OPAL_LPC_IO, port, &mut data, 2);
    if rc != 0 { 0xffff } else { u32::from_be(data) as u16 }
}
unsafe fn opal_lpc_inw(port: u64) -> u16 { u16::from_le(__opal_lpc_inw(port)) }

unsafe fn __opal_lpc_inl(port: u64) -> u32 {
    let mut data: u32 = 0;
    if opal_lpc_chip_id < 0 || port > 0xfffc { return 0xffff_ffff; }
    if port & 3 != 0 {
        return (opal_lpc_inb(port) as u32) << 24 | (opal_lpc_inb(port + 1) as u32) << 16 |
            (opal_lpc_inb(port + 2) as u32) << 8 | opal_lpc_inb(port + 3) as u32;
    }
    let rc = opal_lpc_read(opal_lpc_chip_id, OPAL_LPC_IO, port, &mut data, 4);
    if rc != 0 { 0xffff_ffff } else { u32::from_be(data) }
}
unsafe fn opal_lpc_inl(port: u64) -> u32 { u32::from_le(__opal_lpc_inl(port)) }

unsafe fn opal_lpc_outb(val: u8, port: u64) {
    if opal_lpc_chip_id < 0 || port > 0xffff { return; }
    opal_lpc_write(opal_lpc_chip_id, OPAL_LPC_IO, port, val as u32, 1);
}
unsafe fn __opal_lpc_outw(val: u16, port: u64) {
    if opal_lpc_chip_id < 0 || port > 0xfffe { return; }
    if port & 1 != 0 { opal_lpc_outb((val >> 8) as u8, port); opal_lpc_outb(val as u8, port + 1); return; }
    opal_lpc_write(opal_lpc_chip_id, OPAL_LPC_IO, port, val as u32, 2);
}
unsafe fn opal_lpc_outw(val: u16, port: u64) { __opal_lpc_outw(val.to_le(), port) }
unsafe fn __opal_lpc_outl(val: u32, port: u64) {
    if opal_lpc_chip_id < 0 || port > 0xfffc { return; }
    if port & 3 != 0 {
        opal_lpc_outb((val >> 24) as u8, port); opal_lpc_outb((val >> 16) as u8, port + 1);
        opal_lpc_outb((val >> 8) as u8, port + 2); opal_lpc_outb(val as u8, port + 3); return;
    }
    opal_lpc_write(opal_lpc_chip_id, OPAL_LPC_IO, port, val, 4);
}
unsafe fn opal_lpc_outl(val: u32, port: u64) { __opal_lpc_outl(val.to_le(), port) }

unsafe fn opal_lpc_insb(p: u64, b: *mut u8, mut c: u64) { while c != 0 { *b = opal_lpc_inb(p); b = b.add(1); c -= 1; } }
unsafe fn opal_lpc_insw(p: u64, b: *mut u16, mut c: u64) { while c != 0 { *b = __opal_lpc_inw(p); b = b.add(1); c -= 1; } }
unsafe fn opal_lpc_insl(p: u64, b: *mut u32, mut c: u64) { while c != 0 { *b = __opal_lpc_inl(p); b = b.add(1); c -= 1; } }
unsafe fn opal_lpc_outsb(p: u64, b: *const u8, mut c: u64) { while c != 0 { opal_lpc_outb(*b, p); b = b.add(1); c -= 1; } }
unsafe fn opal_lpc_outsw(p: u64, b: *const u16, mut c: u64) { while c != 0 { __opal_lpc_outw(*b, p); b = b.add(1); c -= 1; } }
unsafe fn opal_lpc_outsl(p: u64, b: *const u32, mut c: u64) { while c != 0 { __opal_lpc_outl(*b, p); b = b.add(1); c -= 1; } }

// The ppc_pci_io table is supplied by asm/isa-bridge.h; initialize it with the
// functions above in the target kernel's native representation.
static opal_lpc_io: ppc_pci_io = ppc_pci_io {
    inb: opal_lpc_inb, inw: opal_lpc_inw, inl: opal_lpc_inl,
    outb: opal_lpc_outb, outw: opal_lpc_outw, outl: opal_lpc_outl,
    insb: opal_lpc_insb, insw: opal_lpc_insw, insl: opal_lpc_insl,
    outsb: opal_lpc_outsb, outsw: opal_lpc_outsw, outsl: opal_lpc_outsl,
};

// CONFIG_DEBUG_FS-dependent debugfs implementation is intentionally retained
// as a conditional translation because its kernel framework types are external.
#[cfg(CONFIG_DEBUG_FS)]
struct lpc_debugfs_entry { lpc_type: OpalLPCAddressType }

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn lpc_debug_read(filp: *mut file, ubuf: *mut u8, count: usize, ppos: *mut i64) -> isize {
    let lpc = (*filp).private_data as *mut lpc_debugfs_entry;
    if !access_ok(ubuf, count) { return -EFAULT; }
    let mut todo = count as u32;
    while todo != 0 {
        let pos = *ppos as u32;
        let mut len = 1;
        if (*lpc).lpc_type == OPAL_LPC_FW { if todo > 3 && pos & 3 == 0 { len = 4; } else if todo > 1 && pos & 1 == 0 { len = 2; } }
        let mut data = 0u32;
        if opal_lpc_read(opal_lpc_chip_id, (*lpc).lpc_type, pos as u64, &mut data, len) != 0 { return -ENXIO; }
        let value = match len { 4 => data, 2 => data >> 16, _ => data >> 24 };
        if __put_user(value, ubuf, len) != 0 { return -EFAULT; }
        *ppos += len as i64; ubuf = ubuf.add(len as usize); todo -= len;
    }
    count as isize
}

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn lpc_debug_write(filp: *mut file, ubuf: *const u8, count: usize, ppos: *mut i64) -> isize {
    let lpc = (*filp).private_data as *mut lpc_debugfs_entry;
    if !access_ok(ubuf, count) { return -EFAULT; }
    let mut todo = count as u32;
    while todo != 0 {
        let pos = *ppos as u32;
        let mut len = 1;
        if (*lpc).lpc_type == OPAL_LPC_FW { if todo > 3 && pos & 3 == 0 { len = 4; } else if todo > 1 && pos & 1 == 0 { len = 2; } }
        let mut data = 0u32;
        if __get_user(&mut data, ubuf, len) != 0 { return -EFAULT; }
        data = match len { 4 => data.to_be(), 2 => (data as u16).to_be() as u32, _ => data };
        if opal_lpc_write(opal_lpc_chip_id, (*lpc).lpc_type, pos as u64, data, len) != 0 { return -ENXIO; }
        *ppos += len as i64; ubuf = ubuf.add(len as usize); todo -= len;
    }
    count as isize
}

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn opal_lpc_debugfs_create_type(folder: *mut dentry, fname: *const u8, ty: OpalLPCAddressType) -> i32 {
    let entry = kzalloc::<lpc_debugfs_entry>();
    if entry.is_null() { return -ENOMEM; }
    (*entry).lpc_type = ty;
    debugfs_create_file(fname, 0o600, folder, entry, &lpc_fops); 0
}

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn opal_lpc_init_debugfs() -> i32 {
    if opal_lpc_chip_id < 0 { return -ENODEV; }
    let root = debugfs_create_dir("lpc", arch_debugfs_dir);
    let mut rc = 0;
    rc |= opal_lpc_debugfs_create_type(root, "io", OPAL_LPC_IO);
    rc |= opal_lpc_debugfs_create_type(root, "mem", OPAL_LPC_MEM);
    rc |= opal_lpc_debugfs_create_type(root, "fw", OPAL_LPC_FW); rc
}

unsafe fn opal_lpc_init() {
    let mut np: *mut device_node = core::ptr::null_mut();
    for_each_compatible_node(np, core::ptr::null_mut(), "ibm,power8-lpc") {
        if !of_device_is_available(np) || !of_property_present(np, "primary") { continue; }
        opal_lpc_chip_id = of_get_ibm_chip_id(np); of_node_put(np); break;
    }
    if opal_lpc_chip_id < 0 { return; }
    if of_property_present(np, "ranges") { pr_info!("OPAL: Found memory mapped LPC bus on chip {}\n", opal_lpc_chip_id); isa_bridge_init_non_pci(np); }
    else { pr_info!("OPAL: Found non-mapped LPC bus on chip {}\n", opal_lpc_chip_id); ppc_pci_io = opal_lpc_io; isa_io_special = true; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
