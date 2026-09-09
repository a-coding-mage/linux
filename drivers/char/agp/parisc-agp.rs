// SPDX-License-Identifier: GPL-2.0-only
/*
 * HP Quicksilver AGP GART routines
 *
 * Copyright (c) 2006, Kyle McMartin <kyle@parisc-linux.org>
 *
 * Based on drivers/char/agpgart/hp-agp.c which is
 * (c) Copyright 2002, 2003 Hewlett-Packard Development Company, L.P.
 *	Bjorn Helgaas <bjorn.helgaas@hp.com>
 */

// Dependencies supplied by the Linux kernel and related translation units.

const DRVNAME: &str = "quicksilver";
const DRVPFX: &str = "quicksilver: ";
const AGP8X_MODE_BIT: u32 = 3;
const AGP8X_MODE: u32 = 1 << AGP8X_MODE_BIT;

extern "C" {
    static mut parisc_agp_info: _parisc_agp_info;
    static mut parisc_agp_masks: [gatt_mask; 1];
    static mut parisc_agp_sizes: [aper_size_info_fixed; 1];
    static mut agp_bridge: *mut agp_bridge_data;
    static parisc_agp_driver: agp_bridge_driver;
}

#[repr(C)]
struct _parisc_agp_info {
    ioc_regs: *mut core::ffi::c_void,
    lba_regs: *mut core::ffi::c_void,
    lba_cap_offset: i32,
    gatt: *mut u64,
    gatt_entries: u64,
    gart_base: u64,
    gart_size: u64,
    io_page_size: i32,
    io_pages_per_kpage: i32,
}

#[repr(C)]
struct gatt_mask { mask: u64, type_: i32 }
#[repr(C)]
struct aper_size_info_fixed { size: i32, num_entries: i32, page_order: i32 }

unsafe fn parisc_agp_fetch_size() -> i32 {
    let size = (parisc_agp_info.gart_size / MB(1)) as i32;
    parisc_agp_sizes[0].size = size;
    (*agp_bridge).current_size = &mut parisc_agp_sizes[0] as *mut _ as *mut core::ffi::c_void;
    size
}

unsafe fn parisc_agp_configure() -> i32 {
    let info = &mut parisc_agp_info;
    (*agp_bridge).gart_bus_addr = info.gart_base;
    (*agp_bridge).capndx = info.lba_cap_offset;
    (*agp_bridge).mode = readl(info.lba_regs.add(info.lba_cap_offset as usize + PCI_AGP_STATUS));
    0
}

unsafe fn parisc_agp_tlbflush(_mem: *mut agp_memory) {
    let info = &mut parisc_agp_info;
    asm_io_sync();
    writeq(info.gart_base | ilog2(info.gart_size), info.ioc_regs.add(IOC_PCOM));
    readq(info.ioc_regs.add(IOC_PCOM));
}

unsafe fn parisc_agp_create_gatt_table(_bridge: *mut agp_bridge_data) -> i32 {
    let info = &mut parisc_agp_info;
    for i in 0..info.gatt_entries as usize {
        *info.gatt.add(i) = cpu_to_le64((*agp_bridge).scratch_page);
    }
    0
}

unsafe fn parisc_agp_free_gatt_table(_bridge: *mut agp_bridge_data) -> i32 {
    parisc_agp_info.gatt.write(SBA_AGPGART_COOKIE);
    0
}

unsafe fn parisc_agp_insert_memory(mem: *mut agp_memory, pg_start: isize, type_: i32) -> i32 {
    let info = &mut parisc_agp_info;
    if type_ != (*mem).type_ || ((*agp_bridge).driver.as_ref().unwrap().agp_type_to_mask_type)(agp_bridge, type_) != 0 { return -EINVAL; }
    let io_pg_start = info.io_pages_per_kpage as isize * pg_start;
    let io_pg_count = info.io_pages_per_kpage as isize * (*mem).page_count as isize;
    if io_pg_start + io_pg_count > info.gatt_entries as isize { return -EINVAL; }
    let mut j = io_pg_start;
    while j < io_pg_start + io_pg_count {
        if *info.gatt.offset(j) != 0 { return -EBUSY; }
        j += 1;
    }
    if !(*mem).is_flushed { global_cache_flush(); (*mem).is_flushed = true; }
    let mut j = io_pg_start;
    for i in 0..(*mem).page_count as usize {
        let mut paddr = page_to_phys(*(*mem).pages.add(i));
        for _k in 0..info.io_pages_per_kpage {
            *info.gatt.offset(j) = cpu_to_le64(parisc_agp_mask_memory(agp_bridge, paddr, type_));
            asm_io_fdc(info.gatt.offset(j));
            j += 1;
            paddr += info.io_page_size as u64;
        }
    }
    ((*agp_bridge).driver.as_ref().unwrap().tlb_flush)(mem);
    0
}

unsafe fn parisc_agp_remove_memory(mem: *mut agp_memory, pg_start: isize, type_: i32) -> i32 {
    let info = &mut parisc_agp_info;
    if type_ != (*mem).type_ || ((*agp_bridge).driver.as_ref().unwrap().agp_type_to_mask_type)(agp_bridge, type_) != 0 { return -EINVAL; }
    let start = info.io_pages_per_kpage * pg_start as i32;
    let count = info.io_pages_per_kpage * (*mem).page_count as i32;
    for i in start..start + count { *info.gatt.offset(i as isize) = cpu_to_le64((*agp_bridge).scratch_page); }
    ((*agp_bridge).driver.as_ref().unwrap().tlb_flush)(mem);
    0
}

unsafe fn parisc_agp_mask_memory(_bridge: *mut agp_bridge_data, addr: u64, _type_: i32) -> u64 {
    let pa = addr & IOVP_MASK;
    let ci: u32;
    core::arch::asm!("lci 0({1}), {0}", out(reg) ci, in(reg) phys_to_virt(pa));
    pa | ((ci as u64 >> PAGE_SHIFT) & 0xff) | SBA_PDIR_VALID_BIT
}

unsafe fn parisc_agp_enable(bridge: *mut agp_bridge_data, mode: u32) {
    let info = &mut parisc_agp_info;
    let mut command = readl(info.lba_regs.add(info.lba_cap_offset as usize + PCI_AGP_STATUS));
    command = agp_collect_device_status(bridge, mode, command);
    command |= 0x100;
    writel(command, info.lba_regs.add(info.lba_cap_offset as usize + PCI_AGP_COMMAND));
    agp_device_command(command, (mode & AGP8X_MODE) != 0);
}

unsafe fn agp_ioc_init(ioc_regs: *mut core::ffi::c_void) -> i32 { let info=&mut parisc_agp_info; info.ioc_regs=ioc_regs; let ps=readq(info.ioc_regs.add(IOC_TCNFG)); let shift=match ps {0=>12,1=>13,2=>14,3=>16,_=>{info.gatt=core::ptr::null_mut();info.gatt_entries=0;return -ENODEV}}; info.io_page_size=1<<shift; info.io_pages_per_kpage=PAGE_SIZE/info.io_page_size; let base=readq(info.ioc_regs.add(IOC_IBASE))&!1; info.gart_base=base+PLUTO_IOVA_SIZE-PLUTO_GART_SIZE; info.gart_size=PLUTO_GART_SIZE; info.gatt_entries=info.gart_size/info.io_page_size as u64; let pdir=phys_to_virt(readq(info.ioc_regs.add(IOC_PDIR_BASE))) as *mut u64; info.gatt=pdir.add((PLUTO_IOVA_SIZE/2>>PAGE_SHIFT) as usize); if *info.gatt!=SBA_AGPGART_COOKIE {info.gatt=core::ptr::null_mut();info.gatt_entries=0;return -ENODEV} 0 }

unsafe fn lba_find_capability(cap: i32) -> i32 {
    let info=&mut parisc_agp_info; let status=readw(info.lba_regs.add(PCI_STATUS));
    if status & PCI_STATUS_CAP_LIST == 0 { return 0; }
    let mut pos=readb(info.lba_regs.add(PCI_CAPABILITY_LIST)); let mut ttl=48;
    while ttl > 0 && pos >= 0x40 { ttl-=1; pos &= !3; let id=readb(info.lba_regs.add(pos as usize + PCI_CAP_LIST_ID)); if id==0xff {break} if id as i32==cap {return pos as i32} pos=readb(info.lba_regs.add(pos as usize + PCI_CAP_LIST_NEXT)); }
    0
}

unsafe fn agp_lba_init(lba_hpa: *mut core::ffi::c_void) -> i32 {
    let info=&mut parisc_agp_info; info.lba_regs=lba_hpa; info.lba_cap_offset=lba_find_capability(PCI_CAP_ID_AGP);
    let cap=readl(lba_hpa.add(info.lba_cap_offset as usize)) & 0xff; if cap != PCI_CAP_ID_AGP as u32 {return -ENODEV} 0
}

unsafe fn parisc_agp_setup(ioc_hpa: *mut core::ffi::c_void, lba_hpa: *mut core::ffi::c_void) -> i32 {
    let fake_bridge_dev=pci_alloc_dev(core::ptr::null_mut()); if fake_bridge_dev.is_null(){return -ENOMEM}
    let mut error=agp_ioc_init(ioc_hpa); if error!=0 {kfree(fake_bridge_dev);return error}
    error=agp_lba_init(lba_hpa); if error!=0 {kfree(fake_bridge_dev);return error}
    let bridge=agp_alloc_bridge(); if bridge.is_null(){kfree(fake_bridge_dev);return -ENOMEM}
    (*bridge).driver=&parisc_agp_driver as *const _; (*fake_bridge_dev).vendor=PCI_VENDOR_ID_HP; (*fake_bridge_dev).device=PCI_DEVICE_ID_HP_PCIX_LBA; (*bridge).dev=fake_bridge_dev;
    error=agp_add_bridge(bridge); if error!=0 {kfree(fake_bridge_dev)} error
}

unsafe fn find_quicksilver(dev: *mut device, data: *mut core::ffi::c_void) -> i32 {
    let lba=data as *mut *mut parisc_device; let padev=to_parisc_device(dev); if IS_QUICKSILVER(padev) {*lba=padev} 0
}

#[no_mangle]
pub unsafe extern "C" fn parisc_agp_init() -> i32 {
    let mut err=-1; let mut lba: *mut parisc_device=core::ptr::null_mut();
    if sba_list.is_null(){return err} let sba=(*sba_list).dev; if !IS_PLUTO(sba){return err}
    device_for_each_child(&mut (*sba).dev, &mut lba as *mut _ as *mut _, find_quicksilver); if lba.is_null(){return err}
    let lbadev=parisc_get_drvdata(lba); parisc_agp_setup((*sba_list).ioc[0].ioc_hpa, (*lbadev).hba.base_addr); err=0; err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
