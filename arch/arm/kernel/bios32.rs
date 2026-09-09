// SPDX-License-Identifier: GPL-2.0
/* Literal low-level translation of arm/kernel/bios32.c.  Kernel-provided
 * types, constants, functions, and fixup registration macros are external. */

static mut DEBUG_PCI: i32 = 0;

unsafe fn pcibios_bus_report_status(bus: *mut pci_bus, status_mask: u32, warn: i32) {
    let mut dev: *mut pci_dev;
    list_for_each_entry!(dev, (*bus).devices, bus_list) {
        let mut status: u16 = 0;
        if (*(*dev).bus).number == 0 && (*dev).devfn == 0 { continue; }
        pci_read_config_word(dev, PCI_STATUS, &mut status);
        if status == 0xffff || (status as u32 & status_mask) == 0 { continue; }
        pci_write_config_word(dev, PCI_STATUS, status & status_mask as u16);
        if warn != 0 { printk!("(%s: %04X) ", pci_name(dev), status); }
    }
    list_for_each_entry!(dev, (*bus).devices, bus_list) {
        if !(*dev).subordinate.is_null() { pcibios_bus_report_status((*dev).subordinate, status_mask, warn); }
    }
}

pub unsafe fn pcibios_report_status(status_mask: u32, warn: i32) {
    let mut bus: *mut pci_bus;
    list_for_each_entry!(bus, pci_root_buses, node) { pcibios_bus_report_status(bus, status_mask, warn); }
}

unsafe fn pci_fixup_83c553(dev: *mut pci_dev) {
    pci_write_config_dword(dev, PCI_BASE_ADDRESS_0, PCI_BASE_ADDRESS_SPACE_MEMORY);
    pci_write_config_word(dev, PCI_COMMAND, PCI_COMMAND_IO);
    (*dev).resource[0].end -= (*dev).resource[0].start;
    (*dev).resource[0].start = 0;
    pci_write_config_byte(dev, 0x48, 0xff);
    pci_write_config_byte(dev, 0x42, 0x01);
    pci_write_config_byte(dev, 0x40, 0x22);
    pci_write_config_byte(dev, 0x83, 0x02);
    pci_write_config_byte(dev, 0x80, 0x11);
    pci_write_config_byte(dev, 0x81, 0x00);
    pci_write_config_word(dev, 0x44, 0xb000);
    outb(0x08, 0x4d1);
}
// DECLARE_PCI_FIXUP_HEADER(PCI_VENDOR_ID_WINBOND, PCI_DEVICE_ID_WINBOND_83C553, pci_fixup_83c553);

unsafe fn pci_fixup_unassign(dev: *mut pci_dev) {
    (*dev).resource[0].end -= (*dev).resource[0].start;
    (*dev).resource[0].start = 0;
}
// DECLARE_PCI_FIXUP_HEADER(PCI_VENDOR_ID_WINBOND2, PCI_DEVICE_ID_WINBOND2_89C940F, pci_fixup_unassign);

unsafe fn pci_fixup_dec21285(dev: *mut pci_dev) {
    if (*dev).devfn == 0 {
        (*dev).class &= 0xff;
        (*dev).class |= PCI_CLASS_BRIDGE_HOST << 8;
        let mut r: *mut resource;
        pci_dev_for_each_resource!(dev, r) { (*r).start = 0; (*r).end = 0; (*r).flags = 0; }
    }
}
// DECLARE_PCI_FIXUP_HEADER(PCI_VENDOR_ID_DEC, PCI_DEVICE_ID_DEC_21285, pci_fixup_dec21285);

unsafe fn pci_fixup_ide_bases(dev: *mut pci_dev) {
    if ((*dev).class >> 8) != PCI_CLASS_STORAGE_IDE { return; }
    let mut r: *mut resource;
    pci_dev_for_each_resource!(dev, r) {
        if ((*r).start & !0x80) == 0x374 { (*r).start |= 2; (*r).end = (*r).start; }
    }
}
// DECLARE_PCI_FIXUP_HEADER(PCI_ANY_ID, PCI_ANY_ID, pci_fixup_ide_bases);

unsafe fn pci_fixup_dec21142(dev: *mut pci_dev) { pci_write_config_dword(dev, 0x40, 0x80000000); }
// DECLARE_PCI_FIXUP_HEADER(PCI_VENDOR_ID_DEC, PCI_DEVICE_ID_DEC_21142, pci_fixup_dec21142);

unsafe fn pci_fixup_cy82c693(dev: *mut pci_dev) {
    if ((*dev).class >> 8) == PCI_CLASS_STORAGE_IDE {
        let (base0, base1) = if (*dev).class & 0x80 != 0 { (0x1f0, 0x3f4) } else { (0x170, 0x374) };
        pci_write_config_dword(dev, PCI_BASE_ADDRESS_0, base0 | PCI_BASE_ADDRESS_SPACE_IO);
        pci_write_config_dword(dev, PCI_BASE_ADDRESS_1, base1 | PCI_BASE_ADDRESS_SPACE_IO);
        for i in 0..2 { (*dev).resource[i].start = 0; (*dev).resource[i].end = 0; (*dev).resource[i].flags = 0; }
    } else if PCI_FUNC((*dev).devfn) == 0 {
        pci_write_config_byte(dev, 0x4b, 14); pci_write_config_byte(dev, 0x4c, 15);
        pci_write_config_byte(dev, 0x4d, 0x41); pci_write_config_byte(dev, 0x44, 0x17); pci_write_config_byte(dev, 0x45, 0x03);
    }
}
// DECLARE_PCI_FIXUP_HEADER(PCI_VENDOR_ID_CONTAQ, PCI_DEVICE_ID_CONTAQ_82C693, pci_fixup_cy82c693);

#[inline] unsafe fn pdev_bad_for_parity(dev: *mut pci_dev) -> bool {
    ((*dev).vendor == PCI_VENDOR_ID_INTERG && ((*dev).device == PCI_DEVICE_ID_INTERG_2000 || (*dev).device == PCI_DEVICE_ID_INTERG_2010)) ||
    ((*dev).vendor == PCI_VENDOR_ID_ITE && (*dev).device == PCI_DEVICE_ID_ITE_8152)
}

pub unsafe fn pcibios_fixup_bus(bus: *mut pci_bus) {
    let mut features: u16 = PCI_COMMAND_SERR | PCI_COMMAND_PARITY | PCI_COMMAND_FAST_BACK;
    let mut dev: *mut pci_dev;
    list_for_each_entry!(dev, (*bus).devices, bus_list) {
        let mut status = 0u16; pci_read_config_word(dev, PCI_STATUS, &mut status);
        if status & PCI_STATUS_FAST_BACK == 0 { features &= !PCI_COMMAND_FAST_BACK; }
        if pdev_bad_for_parity(dev) { features &= !(PCI_COMMAND_SERR | PCI_COMMAND_PARITY); }
        match (*dev).class >> 8 {
            PCI_CLASS_BRIDGE_PCI => { pci_read_config_word(dev, PCI_BRIDGE_CONTROL, &mut status); status |= PCI_BRIDGE_CTL_PARITY|PCI_BRIDGE_CTL_MASTER_ABORT; status &= !(PCI_BRIDGE_CTL_BUS_RESET|PCI_BRIDGE_CTL_FAST_BACK); pci_write_config_word(dev, PCI_BRIDGE_CONTROL, status); }
            PCI_CLASS_BRIDGE_CARDBUS => { pci_read_config_word(dev, PCI_CB_BRIDGE_CONTROL, &mut status); status |= PCI_CB_BRIDGE_CTL_PARITY|PCI_CB_BRIDGE_CTL_MASTER_ABORT; pci_write_config_word(dev, PCI_CB_BRIDGE_CONTROL, status); }
            _ => {}
        }
    }
    list_for_each_entry!(dev, (*bus).devices, bus_list) { let mut cmd=0u16; pci_read_config_word(dev, PCI_COMMAND, &mut cmd); cmd |= features; pci_write_config_word(dev, PCI_COMMAND, cmd); pci_write_config_byte(dev, PCI_CACHE_LINE_SIZE, L1_CACHE_BYTES >> 2); }
    if !(*bus).self_.is_null() && (*(*bus).self_).hdr_type == PCI_HEADER_TYPE_BRIDGE { if features & PCI_COMMAND_FAST_BACK != 0 { (*bus).bridge_ctl |= PCI_BRIDGE_CTL_FAST_BACK; } if features & PCI_COMMAND_PARITY != 0 { (*bus).bridge_ctl |= PCI_BRIDGE_CTL_PARITY; } }
    pr_info!("PCI: bus%d: Fast back to back transfers %s\n", (*bus).number, str_enabled_disabled(features & PCI_COMMAND_FAST_BACK));
}

unsafe fn pcibios_swizzle(dev: *mut pci_dev, pin: *mut u8) -> u8 { let sys=(*dev).sysdata; let oldpin=*pin; let slot=if !(*sys).swizzle.is_none() { ((*sys).swizzle.unwrap())(dev,pin) } else { pci_common_swizzle(dev,pin) }; if DEBUG_PCI != 0 { printk!("PCI: %s swizzling pin %d => pin %d slot %d\n",pci_name(dev),oldpin,*pin,slot); } slot as u8 }
unsafe fn pcibios_map_irq(dev: *const pci_dev, slot:u8, pin:u8) -> i32 { let sys=(*dev).sysdata; let mut irq=-1; if !(*sys).map_irq.is_none() { irq=((*sys).map_irq.unwrap())(dev,slot,pin); } if DEBUG_PCI != 0 { printk!("PCI: %s mapping slot %d pin %d => irq %d\n",pci_name(dev),slot,pin,irq); } irq }

unsafe fn pcibios_init_hw(parent:*mut device, hw:*mut hw_pci, head:*mut list_head) {
    let mut sys:*mut pci_sys_data=core::ptr::null_mut(); let mut busnr=0; let mut nr=0;
    while nr < (*hw).nr_controllers { let bridge=pci_alloc_host_bridge(core::mem::size_of::<pci_sys_data>()); if bridge.is_null(){break;} sys=pci_host_bridge_priv(bridge); (*sys).busnr=busnr; (*sys).swizzle=(*hw).swizzle; (*sys).map_irq=(*hw).map_irq; INIT_LIST_HEAD!(&mut (*sys).resources); if !(*hw).private_data.is_null(){(*sys).private_data=*(*hw).private_data.add(nr as usize);} let mut ret=((*hw).setup)(nr,sys); if ret>0 { ret=pcibios_init_resource(nr,sys); if ret!=0 {pci_free_host_bridge(bridge);break;} (*bridge).map_irq=Some(pcibios_map_irq); (*bridge).swizzle_irq=Some(pcibios_swizzle); if !(*hw).scan.is_none(){ret=((*hw).scan.unwrap())(nr,bridge);} else {list_splice_init!(&mut (*sys).resources,&mut (*bridge).windows);(*bridge).dev.parent=parent;(*bridge).sysdata=sys;(*bridge).busnr=(*sys).busnr;(*bridge).ops=(*hw).ops;ret=pci_scan_root_bus_bridge(bridge);} if ret<0 {pci_free_host_bridge(bridge);break;} (*sys).bus=(*bridge).bus;busnr=(*sys).busn_res.end+1;list_add!(&mut (*sys).node,head);} else {pci_free_host_bridge(bridge);if ret<0{break;}} nr+=1;
    }
}

pub unsafe fn pci_common_init_dev(parent:*mut device, hw:*mut hw_pci) { let mut head=list_head::default(); pci_add_flags(PCI_REASSIGN_ALL_BUS); if !(*hw).preinit.is_none(){((*hw).preinit.unwrap())();} pcibios_init_hw(parent,hw,&mut head); if !(*hw).postinit.is_none(){((*hw).postinit.unwrap())();} let mut sys:*mut pci_sys_data; list_for_each_entry!(sys,head,node){let bus=(*sys).bus;if pci_has_flag(PCI_PROBE_ONLY){pci_bus_claim_resources(bus);}else{pci_bus_size_bridges(bus);pci_bus_assign_resources(bus);let mut child:*mut pci_bus;list_for_each_entry!(child,(*bus).children,node){pcie_bus_configure_settings(child);}}pci_bus_add_devices(bus);} }

#[cfg(not(feature="CONFIG_PCI_HOST_ITE8152"))]
pub unsafe fn pcibios_set_master(_dev:*mut pci_dev) {}

pub unsafe fn pcibios_align_resource(data:*mut core::ffi::c_void,res:*const resource,empty_res:*const resource,size:u64,align:u64)->u64 { let dev=data as *mut pci_dev; let mut start=(*res).start; if (*res).flags&IORESOURCE_IO!=0 && start&0x300!=0 {start=(start+0x3ff)&!0x3ff;} let bridge=pci_find_host_bridge((*dev).bus); if !(*bridge).align_resource.is_none(){return ((*bridge).align_resource.unwrap())(dev,res,start,size,align);} if (*res).flags&IORESOURCE_MEM!=0{return pci_align_resource(dev,res,empty_res,size,align);} start }

// Remaining declarations retain the C implementation's external kernel interactions.
pub unsafe fn pcibios_init_resource(busnr:i32, sys:*mut pci_sys_data) -> i32 { if list_empty!((*sys).resources) { pci_add_resource_offset!((*sys).resources, iomem_resource, (*sys).mem_offset); } let mut window:*mut resource_entry; resource_list_for_each_entry!(window, (*sys).resources) { if resource_type((*window).res)==IORESOURCE_IO { return 0; } } (*sys).io_res.start = if busnr * SZ_64K != 0 { busnr as u64 * SZ_64K } else { pcibios_min_io }; (*sys).io_res.end=(busnr as u64+1)*SZ_64K-1; (*sys).io_res.flags=IORESOURCE_IO; (*sys).io_res.name=(*sys).io_res_name; sprintf!((*sys).io_res_name,"PCI%d I/O",busnr); let ret=request_resource(&ioport_resource,&mut (*sys).io_res); if ret != 0 { pr_err!("PCI: unable to allocate I/O port region (%d)\n",ret); return ret; } pci_add_resource_offset!((*sys).resources,(*sys).io_res,(*sys).io_offset); 0 }

pub unsafe fn pcibios_setup(str_: *mut i8) -> *mut i8 { if strcmp!(str_,"debug") == 0 { DEBUG_PCI=1; return core::ptr::null_mut(); } str_ }
pub unsafe fn pci_map_io_early(pfn: u64) { let mut d=map_desc { virtual_:PCI_IO_VIRT_BASE, type_:MT_DEVICE, length:SZ_64K, pfn }; iotable_init(&mut d,1); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
