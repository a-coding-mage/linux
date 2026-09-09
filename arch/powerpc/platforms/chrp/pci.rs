// SPDX-License-Identifier: GPL-2.0
/*
 * CHRP pci routines.
 */

// Linux/PowerPC dependencies are supplied by the surrounding translation.

/* LongTrail */
pub static mut gg2_pci_config_base: *mut core::ffi::c_void = core::ptr::null_mut();

/* The VLSI Golden Gate II has only 512K of PCI configuration space. */
unsafe fn gg2_read_config(bus: *mut pci_bus, devfn: u32, off: i32, len: i32, val: *mut u32) -> i32 {
    let hose = pci_bus_to_host(bus);
    if (*bus).number > 7 { return PCIBIOS_DEVICE_NOT_FOUND; }
    let cfg_data = (hose.cfg_data as *mut u8).add((((*bus).number << 16) | (devfn << 8) | off as u32) as usize);
    match len {
        1 => *val = in_8(cfg_data as *const _),
        2 => *val = in_le16(cfg_data as *const _),
        _ => *val = in_le32(cfg_data as *const _),
    }
    PCIBIOS_SUCCESSFUL
}

unsafe fn gg2_write_config(bus: *mut pci_bus, devfn: u32, off: i32, len: i32, val: u32) -> i32 {
    let hose = pci_bus_to_host(bus);
    if (*bus).number > 7 { return PCIBIOS_DEVICE_NOT_FOUND; }
    let cfg_data = (hose.cfg_data as *mut u8).add((((*bus).number << 16) | (devfn << 8) | off as u32) as usize);
    match len {
        1 => out_8(cfg_data as *mut _, val),
        2 => out_le16(cfg_data as *mut _, val),
        _ => out_le32(cfg_data as *mut _, val),
    }
    PCIBIOS_SUCCESSFUL
}

static mut gg2_pci_ops: pci_ops = pci_ops { read: Some(gg2_read_config), write: Some(gg2_write_config) };

/* Access functions for PCI config space using RTAS calls. */
unsafe fn rtas_read_config(bus: *mut pci_bus, devfn: u32, offset: i32, len: i32, val: *mut u32) -> i32 {
    let hose = pci_bus_to_host(bus);
    let addr = ((offset & 0xff) as usize) | (((devfn & 0xff) as usize) << 8)
        | (((((*bus).number - hose.first_busno) & 0xff) as usize) << 16)
        | ((hose.global_number as usize) << 24);
    let mut ret: i32 = -1;
    let rval = rtas_call(rtas_function_token(RTAS_FN_READ_PCI_CONFIG), 2, 2, &mut ret, addr, len);
    *val = ret as u32;
    if rval != 0 { PCIBIOS_DEVICE_NOT_FOUND } else { PCIBIOS_SUCCESSFUL }
}

unsafe fn rtas_write_config(bus: *mut pci_bus, devfn: u32, offset: i32, len: i32, val: u32) -> i32 {
    let hose = pci_bus_to_host(bus);
    let addr = ((offset & 0xff) as usize) | (((devfn & 0xff) as usize) << 8)
        | (((((*bus).number - hose.first_busno) & 0xff) as usize) << 16)
        | ((hose.global_number as usize) << 24);
    let rval = rtas_call(rtas_function_token(RTAS_FN_WRITE_PCI_CONFIG), 3, 1, core::ptr::null_mut(), addr, len, val);
    if rval != 0 { PCIBIOS_DEVICE_NOT_FOUND } else { PCIBIOS_SUCCESSFUL }
}

static mut rtas_pci_ops: pci_ops = pci_ops { read: Some(rtas_read_config), write: Some(rtas_write_config) };
pub static mut Hydra: *mut HydraType = core::ptr::null_mut();

unsafe fn hydra_init() -> i32 {
    let np = of_find_node_by_name(core::ptr::null_mut(), c"mac-io".as_ptr());
    let mut r = resource::default();
    if np.is_null() || of_address_to_resource(np, 0, &mut r) != 0 { of_node_put(np); return 0; }
    of_node_put(np);
    Hydra = ioremap(r.start, resource_size(&r)) as *mut HydraType;
    printk(c"Hydra Mac I/O at %llx\n".as_ptr(), r.start as u64);
    printk(c"Hydra Feature_Control was %x".as_ptr(), in_le32(&(*Hydra).Feature_Control));
    out_le32(&mut (*Hydra).Feature_Control, HYDRA_FC_SCC_CELL_EN | HYDRA_FC_SCSI_CELL_EN |
        HYDRA_FC_SCCA_ENABLE | HYDRA_FC_SCCB_ENABLE | HYDRA_FC_ARB_BYPASS |
        HYDRA_FC_MPIC_ENABLE | HYDRA_FC_SLOW_SCC_PCLK | HYDRA_FC_MPIC_IS_MASTER);
    printk(c", now %x\n".as_ptr(), in_le32(&(*Hydra).Feature_Control));
    1
}

const PRG_CL_RESET_VALID: u32 = 0x00010000;

unsafe fn setup_python(hose: *mut pci_controller, dev: *mut device_node) {
    let mut r = resource::default();
    if of_address_to_resource(dev, 0, &mut r) != 0 { printk(c"No address for Python PCI controller\n".as_ptr()); return; }
    let reg = ioremap(r.start + 0xf6000, 0x40) as *mut u32;
    BUG_ON(reg.is_null());
    let val = in_be32(reg.add(12));
    if val & PRG_CL_RESET_VALID != 0 { out_be32(reg.add(12), val & !PRG_CL_RESET_VALID); in_be32(reg.add(12)); }
    iounmap(reg as *mut _);
    setup_indirect_pci(hose, r.start + 0xf8000, r.start + 0xf8010, 0);
}

unsafe fn setup_peg2(hose: *mut pci_controller, _dev: *mut device_node) {
    let root = of_find_node_by_path(c"/".as_ptr());
    let rtas = of_find_node_by_name(root, c"rtas".as_ptr());
    if !rtas.is_null() { (*hose).ops = &mut rtas_pci_ops; of_node_put(rtas); }
    else { printk(c"RTAS supporting Pegasos OF not found, please upgrade your firmware\n".as_ptr()); }
    pci_add_flags(PCI_REASSIGN_ALL_BUS);
}

pub unsafe fn chrp_find_bridges() {
    let root = of_find_node_by_path(c"/".as_ptr());
    let machine = of_get_property(root, c"model".as_ptr(), core::ptr::null_mut());
    let mut is_longtrail = false; let mut is_mot = false; let mut is_pegasos = 0;
    if !machine.is_null() { is_longtrail = strncmp(machine, c"IBM,LongTrail".as_ptr(), 13) == 0; is_mot = strncmp(machine, c"MOT".as_ptr(), 3) == 0;
        if strncmp(machine, c"Pegasos2".as_ptr(), 8) == 0 { is_pegasos = 2; } else if strncmp(machine, c"Pegasos".as_ptr(), 7) == 0 { is_pegasos = 1; } }
    let mut index: i32 = -1;
    for_each_child_of_node!(root, dev, {
        if !of_node_is_type(dev, c"pci".as_ptr()) { continue; }
        index += 1;
        let mut r = resource::default();
        if of_address_to_resource(dev, 0, &mut r) != 0 && !is_longtrail { printk(c"Can't use %pOF: no address\n".as_ptr(), dev); continue; }
        let mut len = 0; let bus_range = of_get_property(dev, c"bus-range".as_ptr(), &mut len);
        if bus_range.is_null() || len < 2 * core::mem::size_of::<i32>() { printk(c"Can't get bus-range for %pOF\n".as_ptr(), dev); continue; }
        let hose = pcibios_alloc_controller(dev); if hose.is_null() { continue; }
        (*hose).first_busno = (*hose).self_busno = *bus_range; (*hose).last_busno = *bus_range.add(1);
        let mut model = of_get_property(dev, c"model".as_ptr(), core::ptr::null_mut()); if model.is_null() { model = c"<none>".as_ptr(); }
        if strncmp(model, c"IBM, Python".as_ptr(), 11) == 0 { setup_python(hose, dev); }
        else if is_mot || strncmp(model, c"Motorola, Grackle".as_ptr(), 17) == 0 { setup_grackle(hose); }
        else if is_longtrail { let p = ioremap(GG2_PCI_CONFIG_BASE, 0x80000); (*hose).ops = &mut gg2_pci_ops; (*hose).cfg_data = p; gg2_pci_config_base = p; }
        else if is_pegasos == 1 { setup_indirect_pci(hose, 0xfec00cf8, 0xfee00cfc, 0); }
        else if is_pegasos == 2 { setup_peg2(hose, dev); }
        else if strncmp(model, c"IBM,CPC710".as_ptr(), 10) == 0 { setup_indirect_pci(hose, r.start + 0xf8000, r.start + 0xf8010, 0); }
        else { (*hose).ops = &mut rtas_pci_ops; }
        pci_process_bridge_OF_ranges(hose, dev, index == 0);
    });
    of_node_put(root); hydra_init(); pci_create_OF_bus_map();
}

const SL82C105_IDECSR: u16 = 0x40;

unsafe fn chrp_pci_fixup_winbond_ata(sl82c105: *mut pci_dev) {
    if !machine_is(chrp) || _chrp_type != _CHRP_briq { return; }
    if ((*sl82c105).class & 5) != 5 { let mut progif = 0; pci_read_config_byte(sl82c105, PCI_CLASS_PROG, &mut progif); pci_write_config_byte(sl82c105, PCI_CLASS_PROG, progif | 5); (*sl82c105).class |= 5; pci_write_config_word(sl82c105, SL82C105_IDECSR, 3); for i in PCI_BASE_ADDRESS_0..=PCI_BASE_ADDRESS_3 { pci_write_config_dword(sl82c105, i, 0); } }
}
DECLARE_PCI_FIXUP_EARLY!(PCI_VENDOR_ID_WINBOND, PCI_DEVICE_ID_WINBOND_82C105, chrp_pci_fixup_winbond_ata);

unsafe fn chrp_pci_fixup_vt8231_ata(viaide: *mut pci_dev) {
    if !machine_is(chrp) || _chrp_type != _CHRP_Pegasos || (*viaide).irq != 14 { return; }
    let viaisa = pci_get_device(PCI_VENDOR_ID_VIA, PCI_DEVICE_ID_VIA_8231, core::ptr::null_mut()); if viaisa.is_null() { return; }
    let mut progif = 0; pci_read_config_byte(viaide, PCI_CLASS_PROG, &mut progif); pci_write_config_byte(viaide, PCI_CLASS_PROG, progif & !5); (*viaide).class &= !5; pci_dev_put(viaisa);
}
DECLARE_PCI_FIXUP_FINAL!(PCI_VENDOR_ID_VIA, PCI_DEVICE_ID_VIA_82C586_1, chrp_pci_fixup_vt8231_ata);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
