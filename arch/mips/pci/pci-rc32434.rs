/*
 *  BRIEF MODULE DESCRIPTION
 *     PCI initialization for IDT EB434 board
 *
 *  Copyright 2004 IDT Inc. (rischelp@idt.com)
 *
 *  This program is free software; you can redistribute  it and/or modify it
 *  under  the terms of  the GNU General  Public License as published by the
 *  Free Software Foundation; either version 2 of the License, or (at your
 *  option) any later version.
 */

// C dependencies supplied by the surrounding kernel translation.

const PCI_ACCESS_READ: u32 = 0;
const PCI_ACCESS_WRITE: u32 = 1;

static mut korina_cnfg_regs: [u32; 25] = [
    KORINA_CNFG1, KORINA_CNFG2, KORINA_CNFG3, KORINA_CNFG4,
    KORINA_CNFG5, KORINA_CNFG6, KORINA_CNFG7, KORINA_CNFG8,
    KORINA_CNFG9, KORINA_CNFG10, KORINA_CNFG11, KORINA_CNFG12,
    KORINA_CNFG13, KORINA_CNFG14, KORINA_CNFG15, KORINA_CNFG16,
    KORINA_CNFG17, KORINA_CNFG18, KORINA_CNFG19, KORINA_CNFG20,
    KORINA_CNFG21, KORINA_CNFG22, KORINA_CNFG23, KORINA_CNFG24,
];

static mut rc32434_res_pci_mem1: resource = resource {
    name: b"PCI MEM1\0".as_ptr() as *const i8, start: 0x50000000,
    end: 0x5fffffff, flags: IORESOURCE_MEM, sibling: core::ptr::null_mut(),
    child: &mut rc32434_res_pci_mem2,
};
static mut rc32434_res_pci_mem2: resource = resource {
    name: b"PCI Mem2\0".as_ptr() as *const i8, start: 0x60000000,
    end: 0x6fffffff, flags: IORESOURCE_MEM, parent: &mut rc32434_res_pci_mem1,
    sibling: core::ptr::null_mut(), child: core::ptr::null_mut(),
};
static mut rc32434_res_pci_io1: resource = resource {
    name: b"PCI I/O1\0".as_ptr() as *const i8, start: 0x18800000,
    end: 0x188fffff, flags: IORESOURCE_IO,
};

const PCI_MEM1_START: usize = PCI_ADDR_START;
const PCI_MEM1_END: usize = PCI_ADDR_START + CPUTOPCI_MEM_WIN - 1;
const PCI_MEM2_START: usize = PCI_ADDR_START + CPUTOPCI_MEM_WIN;
const PCI_MEM2_END: usize = PCI_ADDR_START + 2 * CPUTOPCI_MEM_WIN - 1;
const PCI_IO1_START: usize = PCI_ADDR_START + 2 * CPUTOPCI_MEM_WIN;
const PCI_IO1_END: usize = PCI_ADDR_START + 2 * CPUTOPCI_MEM_WIN + CPUTOPCI_IO_WIN - 1;
const PCI_IO2_START: usize = PCI_ADDR_START + 2 * CPUTOPCI_MEM_WIN + CPUTOPCI_IO_WIN;
const PCI_IO2_END: usize = PCI_ADDR_START + 2 * CPUTOPCI_MEM_WIN + 2 * CPUTOPCI_IO_WIN - 1;

extern "C" {
    static mut rc32434_pci_ops: pci_ops;
    static mut rc32434_pci: *mut rc32434_pci_regs;
    static mut rc32434_pci_msg: *mut rc32434_pci_msg_regs;
    static mut ioport_resource: resource;
    fn ioremap(addr: usize, size: usize) -> *mut core::ffi::c_void;
    fn resource_size(res: *const resource) -> usize;
    fn register_pci_controller(controller: *mut pci_controller);
    fn rc32434_sync();
    fn pr_err(fmt: *const u8, ...);
    fn pr_info(fmt: *const u8, ...);
}

static mut rc32434_controller2: pci_controller = pci_controller::default();
static mut rc32434_controller: pci_controller = pci_controller {
    pci_ops: &mut rc32434_pci_ops,
    mem_resource: &mut rc32434_res_pci_mem1,
    io_resource: &mut rc32434_res_pci_io1,
    mem_offset: 0,
    io_offset: 0,
    ..pci_controller::default()
};

#[cfg(target_endian = "big")]
const PCI_ENDIAN_FLAG: u32 = PCILBAC_sb_m;
#[cfg(not(target_endian = "big"))]
const PCI_ENDIAN_FLAG: u32 = 0;

unsafe fn rc32434_pcibridge_init() -> i32 {
    let mut pcicvalue: u32;
    let mut pcicdata: u32 = 0;
    let mut dummyread: u32;
    let mut pcicntlval: u32;
    let mut pci_config_addr: u32;
    let mut loop_count: i32;

    pcicvalue = (*rc32434_pci).pcic;
    pcicvalue = (pcicvalue >> PCIM_SHFT) & PCIM_BIT_LEN;
    if !(pcicvalue == PCIM_H_EA || pcicvalue == PCIM_H_IA_FIX || pcicvalue == PCIM_H_IA_RR) {
        pr_err(b"PCI init error!!!\n\0".as_ptr());
        return -1;
    }
    pcicdata |= PCI_CTL_IGM | PCI_CTL_EAP | PCI_CTL_EN;
    (*rc32434_pci).pcic = pcicdata;
    loop {
        pcicdata = (*rc32434_pci).pcis;
        if pcicdata & PCI_STAT_RIP == 0 { break; }
    }
    (*rc32434_pci).pcis = 0;
    (*rc32434_pci).pcism = 0xffffffff;
    (*rc32434_pci).pcidac = 0;
    (*rc32434_pci).pcidas = 0;
    (*rc32434_pci).pcidasm = 0x7f;
    (*rc32434_pci_msg).pciiic = 0;
    (*rc32434_pci_msg).pciiim = 0xffffffff;
    (*rc32434_pci_msg).pciioic = 0;
    (*rc32434_pci_msg).pciioim = 0;
    (*rc32434_pci).pcilba[0].address = PCI_ADDR_START as u32;
    (*rc32434_pci).pcilba[0].mapping = PCI_ADDR_START as u32;
    (*rc32434_pci).pcilba[0].control = ((SIZE_256MB & 0x1f) << PCI_LBAC_SIZE_BIT) | PCI_ENDIAN_FLAG;
    dummyread = (*rc32434_pci).pcilba[0].control;
    (*rc32434_pci).pcilba[1].address = 0x60000000;
    (*rc32434_pci).pcilba[1].mapping = 0x60000000;
    (*rc32434_pci).pcilba[1].control = ((SIZE_256MB & 0x1f) << PCI_LBAC_SIZE_BIT) | PCI_ENDIAN_FLAG;
    dummyread = (*rc32434_pci).pcilba[1].control;
    (*rc32434_pci).pcilba[2].address = 0x18c00000;
    (*rc32434_pci).pcilba[2].mapping = 0x18ffffff;
    (*rc32434_pci).pcilba[2].control = ((SIZE_4MB & 0x1f) << PCI_LBAC_SIZE_BIT) | PCI_ENDIAN_FLAG;
    dummyread = (*rc32434_pci).pcilba[2].control;
    (*rc32434_pci).pcilba[3].address = 0x18800000;
    (*rc32434_pci).pcilba[3].mapping = 0x18800000;
    (*rc32434_pci).pcilba[3].control = (((SIZE_1MB & 0x1ff) << PCI_LBAC_SIZE_BIT) | PCI_LBAC_MSI) | PCI_ENDIAN_FLAG;
    dummyread = (*rc32434_pci).pcilba[3].control;
    pci_config_addr = 0x80000004;
    loop_count = 0;
    while loop_count < 24 {
        (*rc32434_pci).pcicfga = pci_config_addr;
        dummyread = (*rc32434_pci).pcicfga;
        (*rc32434_pci).pcicfgd = korina_cnfg_regs[loop_count as usize];
        dummyread = (*rc32434_pci).pcicfgd;
        pci_config_addr += 4;
        loop_count += 1;
    }
    (*rc32434_pci).pcitc = ((PCITC_RTIMER_VAL & 0xff) << PCI_TC_RTIMER_BIT) | ((PCITC_DTIMER_VAL & 0xff) << PCI_TC_DTIMER_BIT);
    pcicntlval = (*rc32434_pci).pcic;
    pcicntlval &= !PCI_CTL_TNR;
    (*rc32434_pci).pcic = pcicntlval;
    pcicntlval = (*rc32434_pci).pcic;
    0
}

unsafe fn rc32434_pci_init() -> i32 {
    pr_info(b"PCI: Initializing PCI\n\0".as_ptr());
    ioport_resource.start = rc32434_res_pci_io1.start;
    ioport_resource.end = rc32434_res_pci_io1.end;
    rc32434_pcibridge_init();
    let io_map_base = ioremap(rc32434_res_pci_io1.start, resource_size(&rc32434_res_pci_io1));
    if io_map_base.is_null() { return -ENOMEM; }
    rc32434_controller.io_map_base = io_map_base as usize - rc32434_res_pci_io1.start;
    register_pci_controller(&mut rc32434_controller);
    rc32434_sync();
    0
}

// arch_initcall(rc32434_pci_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
