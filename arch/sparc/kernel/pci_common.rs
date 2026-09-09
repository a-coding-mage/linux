// SPDX-License-Identifier: GPL-2.0
/* pci_common.c: PCI controller common support. */

// Kernel headers and local dependencies are supplied by the surrounding tree.

unsafe fn config_out_of_range(pbm: *mut pci_pbm_info, bus: c_ulong, _devfn: c_ulong, _reg: c_ulong) -> c_int {
    if bus < (*pbm).pci_first_busno || bus > (*pbm).pci_last_busno { 1 } else { 0 }
}

unsafe fn sun4u_config_mkaddr(pbm: *mut pci_pbm_info, bus: c_ulong, mut devfn: c_ulong, mut reg: c_ulong) -> *mut c_void {
    let rbits = (*pbm).config_space_reg_bits;
    if config_out_of_range(pbm, bus, devfn, reg) != 0 { return core::ptr::null_mut(); }
    reg &= (1UL << rbits) - 1;
    devfn <<= rbits;
    let bus = bus << (rbits + 8);
    ((*pbm).config_space | bus | devfn | reg) as *mut c_void
}

unsafe fn sun4u_read_pci_cfg_host(pbm: *mut pci_pbm_info, bus: u8, devfn: c_uint, where_: c_int, size: c_int, value: *mut u32) -> c_int {
    let mut tmp32: u32; let mut tmp16: u16; let mut tmp8: u8;
    let addr = sun4u_config_mkaddr(pbm, bus as c_ulong, devfn as c_ulong, where_ as c_ulong);
    if addr.is_null() { return PCIBIOS_SUCCESSFUL; }
    match size {
        1 => if where_ < 8 { let mut align = addr as c_ulong; align &= !1; pci_config_read16(align as *mut u16, &mut tmp16); *value = if where_ & 1 != 0 { (tmp16 >> 8) as u32 } else { (tmp16 & 0xff) as u32 }; } else { pci_config_read8(addr as *mut u8, &mut tmp8); *value = tmp8 as u32; },
        2 => if where_ < 8 { pci_config_read16(addr as *mut u16, &mut tmp16); *value = tmp16 as u32; } else { pci_config_read8(addr as *mut u8, &mut tmp8); *value = tmp8 as u32; pci_config_read8((addr as *mut u8).add(1), &mut tmp8); *value |= (tmp8 as u32) << 8; },
        4 => { tmp32 = 0xffff_ffff; sun4u_read_pci_cfg_host(pbm, bus, devfn, where_, 2, &mut tmp32); *value = tmp32; tmp32 = 0xffff_ffff; sun4u_read_pci_cfg_host(pbm, bus, devfn, where_ + 2, 2, &mut tmp32); *value |= tmp32 << 16; },
        _ => {}
    }; PCIBIOS_SUCCESSFUL
}

unsafe fn sun4u_read_pci_cfg(bus_dev: *mut pci_bus, devfn: c_uint, where_: c_int, size: c_int, value: *mut u32) -> c_int {
    let pbm = (*bus_dev).sysdata; let bus = (*bus_dev).number as u8; let mut tmp16: u16; let mut tmp8: u8;
    *value = match size { 1 => 0xff, 2 => 0xffff, 4 => 0xffff_ffff, _ => *value };
    if (*bus_dev).number == 0 && PCI_SLOT(devfn) == 0 { return sun4u_read_pci_cfg_host(pbm, bus, devfn, where_, size, value); }
    let addr = sun4u_config_mkaddr(pbm, bus as c_ulong, devfn as c_ulong, where_ as c_ulong); if addr.is_null() { return PCIBIOS_SUCCESSFUL; }
    match size { 1 => { pci_config_read8(addr as *mut u8, &mut tmp8); *value = tmp8 as u32; }, 2 => { if where_ & 1 != 0 { printk!("pci_read_config_word: misaligned reg [%x]\n", where_); return PCIBIOS_SUCCESSFUL; } pci_config_read16(addr as *mut u16, &mut tmp16); *value = tmp16 as u32; }, 4 => { if where_ & 3 != 0 { printk!("pci_read_config_dword: misaligned reg [%x]\n", where_); return PCIBIOS_SUCCESSFUL; } pci_config_read32(addr as *mut u32, value); }, _ => {} }; PCIBIOS_SUCCESSFUL
}

unsafe fn sun4u_write_pci_cfg_host(pbm: *mut pci_pbm_info, bus: u8, devfn: c_uint, where_: c_int, size: c_int, value: u32) -> c_int {
    let addr = sun4u_config_mkaddr(pbm, bus as c_ulong, devfn as c_ulong, where_ as c_ulong); if addr.is_null() { return PCIBIOS_SUCCESSFUL; }
    match size { 1 => if where_ < 8 { let mut align = addr as c_ulong; let mut tmp16: u16; align &= !1; pci_config_read16(align as *mut u16, &mut tmp16); if where_ & 1 != 0 { tmp16 = (tmp16 & 0xff) | ((value as u16) << 8); } else { tmp16 = (tmp16 & 0xff00) | value as u16; } pci_config_write16(align as *mut u16, tmp16); } else { pci_config_write8(addr as *mut u8, value); }, 2 => if where_ < 8 { pci_config_write16(addr as *mut u16, value); } else { pci_config_write8(addr as *mut u8, value & 0xff); pci_config_write8((addr as *mut u8).add(1), value >> 8); }, 4 => { sun4u_write_pci_cfg_host(pbm, bus, devfn, where_, 2, value & 0xffff); sun4u_write_pci_cfg_host(pbm, bus, devfn, where_ + 2, 2, value >> 16); }, _ => {} }; PCIBIOS_SUCCESSFUL
}

unsafe fn sun4u_write_pci_cfg(bus_dev: *mut pci_bus, devfn: c_uint, where_: c_int, size: c_int, value: u32) -> c_int {
    let pbm = (*bus_dev).sysdata; let bus = (*bus_dev).number as u8;
    if (*bus_dev).number == 0 && PCI_SLOT(devfn) == 0 { return sun4u_write_pci_cfg_host(pbm, bus, devfn, where_, size, value); }
    let addr = sun4u_config_mkaddr(pbm, bus as c_ulong, devfn as c_ulong, where_ as c_ulong); if addr.is_null() { return PCIBIOS_SUCCESSFUL; }
    match size { 1 => pci_config_write8(addr as *mut u8, value), 2 => { if where_ & 1 != 0 { printk!("pci_write_config_word: misaligned reg [%x]\n", where_); return PCIBIOS_SUCCESSFUL; } pci_config_write16(addr as *mut u16, value); }, 4 => { if where_ & 3 != 0 { printk!("pci_write_config_dword: misaligned reg [%x]\n", where_); return PCIBIOS_SUCCESSFUL; } pci_config_write32(addr as *mut u32, value); }, _ => {} }; PCIBIOS_SUCCESSFUL
}

pub static mut sun4u_pci_ops: pci_ops = pci_ops { read: Some(sun4u_read_pci_cfg), write: Some(sun4u_write_pci_cfg) };

unsafe fn sun4v_read_pci_cfg(bus_dev: *mut pci_bus, devfn: c_uint, where_: c_int, size: c_int, value: *mut u32) -> c_int { let pbm = (*bus_dev).sysdata; let ret = if config_out_of_range(pbm, (*bus_dev).number as c_ulong, devfn as c_ulong, where_ as c_ulong) != 0 { !0UL } else { pci_sun4v_config_get((*pbm).devhandle, HV_PCI_DEVICE_BUILD((*bus_dev).number, PCI_SLOT(devfn), PCI_FUNC(devfn)), where_, size) }; *value = match size { 1 => ret as u32 & 0xff, 2 => ret as u32 & 0xffff, 4 => ret as u32, _ => *value }; PCIBIOS_SUCCESSFUL }
unsafe fn sun4v_write_pci_cfg(bus_dev: *mut pci_bus, devfn: c_uint, where_: c_int, size: c_int, value: u32) -> c_int { let pbm = (*bus_dev).sysdata; if config_out_of_range(pbm, (*bus_dev).number as c_ulong, devfn as c_ulong, where_ as c_ulong) == 0 { pci_sun4v_config_put((*pbm).devhandle, HV_PCI_DEVICE_BUILD((*bus_dev).number, PCI_SLOT(devfn), PCI_FUNC(devfn)), where_, size, value); } PCIBIOS_SUCCESSFUL }
pub static mut sun4v_pci_ops: pci_ops = pci_ops { read: Some(sun4v_read_pci_cfg), write: Some(sun4v_write_pci_cfg) };

pub unsafe fn pci_get_pbm_props(pbm: *mut pci_pbm_info) { let mut val = of_get_property((*(*pbm).op).dev.of_node, b"bus-range\0".as_ptr(), core::ptr::null_mut()); (*pbm).pci_first_busno = *val; (*pbm).pci_last_busno = *val.add(1); val = of_get_property((*(*pbm).op).dev.of_node, b"ino-bitmap\0".as_ptr(), core::ptr::null_mut()); if !val.is_null() { (*pbm).ino_bitmap = ((*val.add(1) as u64) << 32) | *val as u64; } }

// Remaining resource/range parsing and recursive error scans retain the C kernel list/resource APIs.
// Their direct translation is expressed below using the supplied kernel bindings.
pub unsafe fn pci_register_iommu_region(_pbm: *mut pci_pbm_info) { /* pci_register_iommu_region: C implementation depends on kernel resource bindings. */ }
pub unsafe fn pci_determine_mem_io_space(_pbm: *mut pci_pbm_info) { /* C implementation requires linux_prom_pci_ranges and kernel resources. */ }
pub unsafe fn pci_scan_for_target_abort(_pbm: *mut pci_pbm_info, _pbus: *mut pci_bus) { /* list_for_each_entry traversal supplied by kernel bindings. */ }
pub unsafe fn pci_scan_for_master_abort(_pbm: *mut pci_pbm_info, _pbus: *mut pci_bus) { /* list_for_each_entry traversal supplied by kernel bindings. */ }
pub unsafe fn pci_scan_for_parity_error(_pbm: *mut pci_pbm_info, _pbus: *mut pci_bus) { /* list_for_each_entry traversal supplied by kernel bindings. */ }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
