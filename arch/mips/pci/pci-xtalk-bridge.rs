// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2003 Christoph Hellwig (hch@lst.de)
 * Copyright (C) 1999, 2000, 04 Ralf Baechle (ralf@linux-mips.org)
 * Copyright (C) 1999, 2000 Silicon Graphics, Inc.
 */

const CRC16_INIT: u16 = 0;
const CRC16_VALID: u16 = 0xb001;

/* Common phys<->dma mapping for platforms using pci xtalk bridge */
pub unsafe fn phys_to_dma(dev: *mut device, paddr: phys_addr_t) -> dma_addr_t {
    let pdev = to_pci_dev(dev);
    let bc = BRIDGE_CONTROLLER((*pdev).bus);
    (*bc).baddr.wrapping_add(paddr)
}

pub unsafe fn dma_to_phys(_dev: *mut device, dma_addr: dma_addr_t) -> phys_addr_t {
    dma_addr & !(0xffUL << 56)
}

/* Most of the IOC3 PCI config registers aren't present; emulate what is needed. */
unsafe fn ioc3_cfg_rd(addr: *mut c_void, where_: c_int, size: c_int,
                      value: *mut u32, sid: u32) -> c_int {
    let mut cf: u32 = 0;
    match where_ & !3 {
        0x00..=0x10 | 0x40..=0x44 => {
            if get_dbe(&mut cf, addr as *mut u32) != 0 { return PCIBIOS_DEVICE_NOT_FOUND; }
        }
        0x2c => cf = sid,
        0x3c => cf = 0x00000100,
        _ => {}
    }
    let shift = ((where_ & 3) << 3) as u32;
    let mask = 0xffffffffu32 >> (((4 - size) << 3) as u32);
    *value = (cf >> shift) & mask;
    PCIBIOS_SUCCESSFUL
}

unsafe fn ioc3_cfg_wr(addr: *mut c_void, where_: c_int, size: c_int, value: u32) -> c_int {
    let mut cf: u32 = 0;
    if (where_ >= 0x14 && where_ < 0x40) || where_ >= 0x48 { return PCIBIOS_SUCCESSFUL; }
    if get_dbe(&mut cf, addr as *mut u32) != 0 { return PCIBIOS_DEVICE_NOT_FOUND; }
    let shift = ((where_ & 3) << 3) as u32;
    let mask = 0xffffffffu32 >> (((4 - size) << 3) as u32);
    let smask = mask << shift;
    cf = (cf & !smask) | ((value & mask) << shift);
    if put_dbe(cf, addr as *mut u32) != 0 { return PCIBIOS_DEVICE_NOT_FOUND; }
    PCIBIOS_SUCCESSFUL
}

unsafe fn bridge_disable_swapping(dev: *mut pci_dev) {
    let bc = BRIDGE_CONTROLLER((*dev).bus);
    let slot = PCI_SLOT((*dev).devfn);
    bridge_clr(bc, (*bc).base.as_ref().unwrap().b_device[slot as usize].reg, BRIDGE_DEV_SWAP_DIR);
    bridge_read(bc, b_widget.w_tflush);
}

/* The Bridge ASIC supports type 0 and type 1 access. */
unsafe fn pci_conf0_read_config(bus: *mut pci_bus, devfn: c_uint, where_: c_int,
                                size: c_int, value: *mut u32) -> c_int {
    let bc = BRIDGE_CONTROLLER(bus); let bridge = (*bc).base; let slot = PCI_SLOT(devfn); let fn_ = PCI_FUNC(devfn);
    let mut addr = &mut (*bridge).b_type0_cfg_dev[slot as usize].f[fn_ as usize].c[PCI_VENDOR_ID] as *mut _ as *mut c_void;
    let mut cf = 0u32;
    if get_dbe(&mut cf, addr as *mut u32) != 0 { return PCIBIOS_DEVICE_NOT_FOUND; }
    if cf == (PCI_VENDOR_ID_SGI | (PCI_DEVICE_ID_SGI_IOC3 << 16)) {
        addr = &mut (*bridge).b_type0_cfg_dev[slot as usize].f[fn_ as usize].l[(where_ >> 2) as usize] as *mut _ as *mut c_void;
        return ioc3_cfg_rd(addr, where_, size, value, (*bc).ioc3_sid[slot as usize]);
    }
    addr = &mut (*bridge).b_type0_cfg_dev[slot as usize].f[fn_ as usize].c[(where_ ^ (4-size)) as usize] as *mut _ as *mut c_void;
    let res = if size == 1 { get_dbe(value as *mut u8, addr as *mut u8) } else if size == 2 { get_dbe(value as *mut u16, addr as *mut u16) } else { get_dbe(value, addr as *mut u32) };
    if res != 0 { PCIBIOS_DEVICE_NOT_FOUND } else { PCIBIOS_SUCCESSFUL }
}

unsafe fn pci_conf1_read_config(bus: *mut pci_bus, devfn: c_uint, where_: c_int, size: c_int, value: *mut u32) -> c_int {
    let bc = BRIDGE_CONTROLLER(bus); let bridge = (*bc).base; let busno = (*bus).number; let slot = PCI_SLOT(devfn); let fn_ = PCI_FUNC(devfn);
    bridge_write(bc, b_pci_cfg, (busno << 16) | (slot << 11));
    let mut addr = &mut (*bridge).b_type1_cfg.c[((fn_ << 8) | PCI_VENDOR_ID) as usize] as *mut _ as *mut c_void; let mut cf=0u32;
    if get_dbe(&mut cf, addr as *mut u32) != 0 { return PCIBIOS_DEVICE_NOT_FOUND; }
    if cf == (PCI_VENDOR_ID_SGI | (PCI_DEVICE_ID_SGI_IOC3 << 16)) { addr=&mut (*bridge).b_type1_cfg.c[((fn_<<8)|(where_ & !3)) as usize] as *mut _ as *mut c_void; return ioc3_cfg_rd(addr,where_,size,value,(*bc).ioc3_sid[slot as usize]); }
    addr=&mut (*bridge).b_type1_cfg.c[((fn_<<8)|(where_^(4-size))) as usize] as *mut _ as *mut c_void;
    let res=if size==1{get_dbe(value as *mut u8,addr as *mut u8)}else if size==2{get_dbe(value as *mut u16,addr as *mut u16)}else{get_dbe(value,addr as *mut u32)};
    if res!=0 {PCIBIOS_DEVICE_NOT_FOUND} else {PCIBIOS_SUCCESSFUL}
}

unsafe fn pci_read_config(bus:*mut pci_bus,devfn:c_uint,where_:c_int,size:c_int,value:*mut u32)->c_int { if !pci_is_root_bus(bus){pci_conf1_read_config(bus,devfn,where_,size,value)}else{pci_conf0_read_config(bus,devfn,where_,size,value)} }

unsafe fn pci_conf0_write_config(bus:*mut pci_bus,devfn:c_uint,where_:c_int,size:c_int,value:u32)->c_int {
    let bc=BRIDGE_CONTROLLER(bus); let bridge=(*bc).base; let slot=PCI_SLOT(devfn); let fn_=PCI_FUNC(devfn); let mut addr=&mut (*bridge).b_type0_cfg_dev[slot as usize].f[fn_ as usize].c[PCI_VENDOR_ID] as *mut _ as *mut c_void; let mut cf=0u32;
    if get_dbe(&mut cf,addr as *mut u32)!=0{return PCIBIOS_DEVICE_NOT_FOUND;} if cf==(PCI_VENDOR_ID_SGI|(PCI_DEVICE_ID_SGI_IOC3<<16)){addr=&mut (*bridge).b_type0_cfg_dev[slot as usize].f[fn_ as usize].l[(where_>>2) as usize] as *mut _ as *mut c_void;return ioc3_cfg_wr(addr,where_,size,value);} addr=&mut (*bridge).b_type0_cfg_dev[slot as usize].f[fn_ as usize].c[(where_^(4-size)) as usize] as *mut _ as *mut c_void; let res=if size==1{put_dbe(value,addr as *mut u8)}else if size==2{put_dbe(value,addr as *mut u16)}else{put_dbe(value,addr as *mut u32)}; if res!=0{PCIBIOS_DEVICE_NOT_FOUND}else{PCIBIOS_SUCCESSFUL}
}

unsafe fn pci_conf1_write_config(bus:*mut pci_bus,devfn:c_uint,where_:c_int,size:c_int,value:u32)->c_int { let bc=BRIDGE_CONTROLLER(bus);let bridge=(*bc).base;let slot=PCI_SLOT(devfn);let fn_=PCI_FUNC(devfn);let busno=(*bus).number;bridge_write(bc,b_pci_cfg,(busno<<16)|(slot<<11));let mut addr=&mut (*bridge).b_type1_cfg.c[((fn_<<8)|PCI_VENDOR_ID) as usize] as *mut _ as *mut c_void;let mut cf=0u32;if get_dbe(&mut cf,addr as *mut u32)!=0{return PCIBIOS_DEVICE_NOT_FOUND;}if cf==(PCI_VENDOR_ID_SGI|(PCI_DEVICE_ID_SGI_IOC3<<16)){addr=&mut (*bridge).b_type0_cfg_dev[slot as usize].f[fn_ as usize].l[(where_>>2) as usize] as *mut _ as *mut c_void;return ioc3_cfg_wr(addr,where_,size,value);}addr=&mut (*bridge).b_type1_cfg.c[((fn_<<8)|(where_^(4-size))) as usize] as *mut _ as *mut c_void;let res=if size==1{put_dbe(value,addr as *mut u8)}else if size==2{put_dbe(value,addr as *mut u16)}else{put_dbe(value,addr as *mut u32)};if res!=0{PCIBIOS_DEVICE_NOT_FOUND}else{PCIBIOS_SUCCESSFUL} }
unsafe fn pci_write_config(bus:*mut pci_bus,devfn:c_uint,where_:c_int,size:c_int,value:u32)->c_int{if !pci_is_root_bus(bus){pci_conf1_write_config(bus,devfn,where_,size,value)}else{pci_conf0_write_config(bus,devfn,where_,size,value)}}

static mut bridge_pci_ops: pci_ops = pci_ops { read: pci_read_config, write: pci_write_config };

#[repr(C)] pub struct bridge_irq_chip_data { pub bc:*mut bridge_controller, pub nasid:nasid_t }

unsafe fn bridge_set_affinity(d:*mut irq_data,mask:*const cpumask,force:bool)->c_int { irq_chip_set_affinity_parent(d,mask,force) }
pub static mut bridge_irq_chip: irq_chip = irq_chip { name: "BRIDGE", irq_mask: irq_chip_mask_parent, irq_unmask: irq_chip_unmask_parent, irq_set_affinity: bridge_set_affinity };

unsafe fn bridge_map_irq(dev:*const pci_dev,slot:u8,mut pin:u8)->c_int { let bc=BRIDGE_CONTROLLER((*dev).bus); pin=match pin{PCI_INTERRUPT_UNKNOWN|PCI_INTERRUPT_INTA|PCI_INTERRUPT_INTC=>0,PCI_INTERRUPT_INTB|PCI_INTERRUPT_INTD=>1,_=>pin}; let irq=(*bc).pci_int[slot as usize][pin as usize]; if irq==-1 { let info=irq_alloc_info{ctrl:bc,nasid:(*bc).nasid,pin:(*bc).int_mapping[slot as usize][pin as usize]}; let irq=irq_domain_alloc_irqs((*bc).domain,1,(*bc).nasid,&info); if irq<0{return irq;} (*bc).pci_int[slot as usize][pin as usize]=irq;return irq;} irq }

const fn IOC3_SID(sid:u32)->u32 { PCI_VENDOR_ID_SGI | (sid<<16) }
unsafe fn bridge_setup_ip27_baseio6g(bc:*mut bridge_controller){(*bc).ioc3_sid[2]=IOC3_SID(IOC3_SUBSYS_IP27_BASEIO6G);(*bc).ioc3_sid[6]=IOC3_SID(IOC3_SUBSYS_IP27_MIO);(*bc).int_mapping[2][1]=4;(*bc).int_mapping[6][1]=6;}
unsafe fn bridge_setup_ip27_baseio(bc:*mut bridge_controller){(*bc).ioc3_sid[2]=IOC3_SID(IOC3_SUBSYS_IP27_BASEIO);(*bc).int_mapping[2][1]=4;}
unsafe fn bridge_setup_ip29_baseio(bc:*mut bridge_controller){(*bc).ioc3_sid[2]=IOC3_SID(IOC3_SUBSYS_IP29_SYSBOARD);(*bc).int_mapping[2][1]=3;}
unsafe fn bridge_setup_ip30_sysboard(bc:*mut bridge_controller){(*bc).ioc3_sid[2]=IOC3_SID(IOC3_SUBSYS_IP30_SYSBOARD);(*bc).int_mapping[2][1]=4;}
unsafe fn bridge_setup_menet(bc:*mut bridge_controller){for i in 0..3{(*bc).ioc3_sid[i]=IOC3_SID(IOC3_SUBSYS_MENET);}(*bc).ioc3_sid[3]=IOC3_SID(IOC3_SUBSYS_MENET4);}
unsafe fn bridge_setup_io7(bc:*mut bridge_controller){(*bc).ioc3_sid[4]=IOC3_SID(IOC3_SUBSYS_IO7);}
unsafe fn bridge_setup_io8(bc:*mut bridge_controller){(*bc).ioc3_sid[4]=IOC3_SID(IOC3_SUBSYS_IO8);}
unsafe fn bridge_setup_io9(bc:*mut bridge_controller){(*bc).ioc3_sid[1]=IOC3_SID(IOC3_SUBSYS_IO9);}
unsafe fn bridge_setup_ip34_fuel_sysboard(bc:*mut bridge_controller){(*bc).ioc3_sid[4]=IOC3_SID(IOC3_SUBSYS_IP34_SYSBOARD);}

#[repr(C)] struct bridge_ioc3_devid_entry { match_: *const u8, setup: unsafe fn(*mut bridge_controller) }
static bridge_ioc3_devid: [bridge_ioc3_devid_entry;14] = [
    bridge_ioc3_devid_entry{match_:b"030-0734-\0".as_ptr(),setup:bridge_setup_ip27_baseio6g}, bridge_ioc3_devid_entry{match_:b"030-0880-\0".as_ptr(),setup:bridge_setup_ip27_baseio6g},
    bridge_ioc3_devid_entry{match_:b"030-1023-\0".as_ptr(),setup:bridge_setup_ip27_baseio}, bridge_ioc3_devid_entry{match_:b"030-1124-\0".as_ptr(),setup:bridge_setup_ip27_baseio},
    bridge_ioc3_devid_entry{match_:b"030-1025-\0".as_ptr(),setup:bridge_setup_ip29_baseio}, bridge_ioc3_devid_entry{match_:b"030-1244-\0".as_ptr(),setup:bridge_setup_ip29_baseio},
    bridge_ioc3_devid_entry{match_:b"030-1389-\0".as_ptr(),setup:bridge_setup_ip29_baseio}, bridge_ioc3_devid_entry{match_:b"030-0887-\0".as_ptr(),setup:bridge_setup_ip30_sysboard},
    bridge_ioc3_devid_entry{match_:b"030-1467-\0".as_ptr(),setup:bridge_setup_ip30_sysboard}, bridge_ioc3_devid_entry{match_:b"030-0873-\0".as_ptr(),setup:bridge_setup_menet},
    bridge_ioc3_devid_entry{match_:b"030-1557-\0".as_ptr(),setup:bridge_setup_io7}, bridge_ioc3_devid_entry{match_:b"030-1673-\0".as_ptr(),setup:bridge_setup_io8},
    bridge_ioc3_devid_entry{match_:b"030-1771-\0".as_ptr(),setup:bridge_setup_io9}, bridge_ioc3_devid_entry{match_:b"030-1707-\0".as_ptr(),setup:bridge_setup_ip34_fuel_sysboard}];

unsafe fn bridge_setup_board(bc:*mut bridge_controller,partnum:*const u8){for e in bridge_ioc3_devid.iter(){if strncmp(partnum,e.match_,strlen(e.match_))==0{(e.setup)(bc);}}}
unsafe fn bridge_nvmem_match(dev:*mut device,data:*const c_void)->c_int{let name=dev_name(dev);let prefix=data as *const i8;if strlen(name)<strlen(prefix){0}else if memcmp(prefix,name,strlen(prefix))==0{1}else{0}}
unsafe fn bridge_get_partnum(baddr:u64,partnum:*mut u8)->c_int{let mut prefix=[0i8;24];snprintf(prefix.as_mut_ptr(),prefix.len(),b"bridge-%012llx-0b-\0".as_ptr() as *const i8,baddr);let nvmem=nvmem_device_find(prefix.as_ptr(),bridge_nvmem_match);if IS_ERR(nvmem){return PTR_ERR(nvmem);}let mut prom=[0u8;64];let ret=nvmem_device_read(nvmem,0,64,prom.as_mut_ptr());nvmem_device_put(nvmem);if ret!=64{return ret;}if crc16(CRC16_INIT,prom.as_ptr(),32)!=CRC16_VALID||crc16(CRC16_INIT,prom.as_ptr().add(32),32)!=CRC16_VALID{return -EINVAL;}let mut j=0;for i in 0..19{if prom[i+11]!=b' '{*partnum.add(j)=prom[i+11];j+=1;}}for i in 0..6{if prom[i+32]!=b' '{*partnum.add(j)=prom[i+32];j+=1;}}*partnum.add(j)=0;0}

// The remaining probe/remove and platform-driver structures retain their C control flow
// through the external kernel declarations used by this translation unit.
unsafe fn bridge_remove(pdev:*mut platform_device){let bus=platform_get_drvdata(pdev);let bc=BRIDGE_CONTROLLER(bus);let fn_=(*(*bc).domain).fwnode;irq_domain_remove((*bc).domain);irq_domain_free_fwnode(fn_);pci_lock_rescan_remove();pci_stop_root_bus(bus);pci_remove_root_bus(bus);pci_unlock_rescan_remove();}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
