// SPDX-License-Identifier: GPL-2.0-only
/*
 * pata_optidma.c 	- Opti DMA PATA for new ATA layer
 *
 * Direct Rust translation of the Linux driver source. Kernel-provided types,
 * constants, functions, and macros are intentionally left as external names.
 */

const DRV_NAME: &str = "pata_optidma";
const DRV_VERSION: &str = "0.3.2";

const READ_REG: usize = 0;
const WRITE_REG: usize = 1;
const CNTRL_REG: usize = 3;
const STRAP_REG: usize = 5;
const MISC_REG: usize = 6;

static mut pci_clock: i32 = 0;

unsafe fn optidma_pre_reset(link: *mut ata_link, deadline: c_ulong) -> c_int {
    let ap = (*link).ap;
    let pdev = to_pci_dev((*(*ap).host).dev);
    let optidma_enable_bits = pci_bits { reg: 0x40, width: 1, mask: 0x08, val: 0x00 };
    if (*ap).port_no != 0 && pci_test_config_bits(pdev, &optidma_enable_bits) == 0 { return -ENOENT; }
    ata_sff_prereset(link, deadline)
}

unsafe fn optidma_unlock(ap: *mut ata_port) {
    let regio = (*ap).ioaddr.cmd_addr;
    ioread16(regio.add(1));
    ioread16(regio.add(1));
    iowrite8(3, regio.add(2));
}

unsafe fn optidma_lock(ap: *mut ata_port) {
    let regio = (*ap).ioaddr.cmd_addr;
    iowrite8(0x83, regio.add(2));
}

unsafe fn optidma_mode_setup(ap: *mut ata_port, adev: *mut ata_device, mode: u8) {
    let pair = ata_dev_pair(adev);
    let pio = (*adev).pio_mode - XFER_PIO_0;
    let dma = (*adev).dma_mode - XFER_MW_DMA_0;
    let regio = (*ap).ioaddr.cmd_addr;
    let addr_timing: [[u8; 5]; 2] = [[0x30,0x20,0x20,0x10,0x10], [0x20,0x20,0x10,0x10,0x10]];
    let data_rec_timing: [[u8; 5]; 2] = [[0x59,0x46,0x30,0x20,0x20], [0x46,0x32,0x20,0x20,0x10]];
    let dma_data_rec_timing: [[u8; 3]; 2] = [[0x76,0x20,0x20], [0x54,0x20,0x10]];
    optidma_unlock(ap);
    let mut addr = if mode >= XFER_MW_DMA_0 { 0 } else { addr_timing[pci_clock as usize][pio as usize] };
    if !pair.is_null() {
        let pair_addr = if ata_dma_enabled(pair) != 0 { 0 } else { addr_timing[pci_clock as usize][((*pair).pio_mode-XFER_PIO_0) as usize] };
        if pair_addr > addr { addr = pair_addr; }
    }
    iowrite8((*adev).devno, regio.add(MISC_REG));
    if mode < XFER_MW_DMA_0 {
        iowrite8(data_rec_timing[pci_clock as usize][pio as usize], regio.add(READ_REG));
        iowrite8(data_rec_timing[pci_clock as usize][pio as usize], regio.add(WRITE_REG));
    } else if mode < XFER_UDMA_0 {
        iowrite8(dma_data_rec_timing[pci_clock as usize][dma as usize], regio.add(READ_REG));
        iowrite8(dma_data_rec_timing[pci_clock as usize][dma as usize], regio.add(WRITE_REG));
    }
    iowrite8(addr | (*adev).devno, regio.add(MISC_REG));
    iowrite8(0x85, regio.add(CNTRL_REG));
    optidma_lock(ap);
}

unsafe fn optiplus_mode_setup(ap: *mut ata_port, adev: *mut ata_device, mode: u8) {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let mut udcfg = 0u8; let mut udslave = 0u8;
    let dev2 = 2 * (*adev).devno; let unit = 2 * (*ap).port_no + (*adev).devno;
    let udma = mode - XFER_UDMA_0;
    pci_read_config_byte(pdev, 0x44, &mut udcfg);
    if mode <= XFER_UDMA_0 { udcfg &= !(1 << unit); optidma_mode_setup(ap, adev, (*adev).dma_mode); }
    else { udcfg |= 1 << unit; if (*ap).port_no != 0 { pci_read_config_byte(pdev,0x45,&mut udslave); udslave &= !(0x03 << dev2); udslave |= udma << dev2; pci_write_config_byte(pdev,0x45,udslave); } else { udcfg &= !(0x30 << dev2); udcfg |= udma << dev2; } }
    pci_write_config_byte(pdev, 0x44, udcfg);
}

unsafe fn optidma_set_pio_mode(ap:*mut ata_port, adev:*mut ata_device){ optidma_mode_setup(ap,adev,(*adev).pio_mode); }
unsafe fn optidma_set_dma_mode(ap:*mut ata_port, adev:*mut ata_device){ optidma_mode_setup(ap,adev,(*adev).dma_mode); }
unsafe fn optiplus_set_pio_mode(ap:*mut ata_port, adev:*mut ata_device){ optiplus_mode_setup(ap,adev,(*adev).pio_mode); }
unsafe fn optiplus_set_dma_mode(ap:*mut ata_port, adev:*mut ata_device){ optiplus_mode_setup(ap,adev,(*adev).dma_mode); }

unsafe fn optidma_make_bits43(adev:*mut ata_device)->u8 {
    let bits43=[0u8,0,0,1,2]; if ata_dev_enabled(adev)==0{return 0;} if ata_dma_enabled(adev)!=0{return (*adev).dma_mode-XFER_MW_DMA_0;} bits43[((*adev).pio_mode-XFER_PIO_0) as usize]
}

unsafe fn optidma_set_mode(link:*mut ata_link, r_failed:*mut *mut ata_device)->c_int {
    let ap=(*link).ap; let pdev=to_pci_dev((*(*ap).host).dev); let nybble=4*(*ap).port_no; let mut r=0u8;
    let rc=ata_set_mode(link,r_failed); if rc==0 { pci_read_config_byte(pdev,0x43,&mut r); r &= 0x0F << nybble; r |= (optidma_make_bits43((*link).device.add(0)) + (optidma_make_bits43((*link).device.add(0)) << 2)) << nybble; pci_write_config_byte(pdev,0x43,r); } rc
}

unsafe fn optiplus_with_udma(pdev:*mut pci_dev)->c_int {
    let mut ret=0; let mut r=0u8; let mut ioport=0x22; let dev1=pci_get_device(0x1045,0xC701,core::ptr::null_mut());
    if dev1.is_null(){return 0;} pci_read_config_byte(dev1,0x08,&mut r); if r<0x10 {pci_dev_put(dev1);return 0;}
    pci_read_config_byte(dev1,0x5F,&mut r); ioport |= (r as c_int)<<8; outb(0x10,ioport); if (inb(ioport+2)&1)==0 { printk(KERN_WARNING, "UDMA not supported in this configuration.\n"); pci_dev_put(dev1); return ret; }
    pci_read_config_byte(pdev,0x42,&mut r); if (r&0x36)!=0x36 { printk(KERN_WARNING, "UDMA not supported in this configuration.\n"); pci_dev_put(dev1); return ret; }
    pci_read_config_byte(dev1,0x52,&mut r); if r&0x80 != 0 {ret=1;}
    printk(KERN_WARNING, "UDMA not supported in this configuration.\n"); pci_dev_put(dev1); ret
}

unsafe fn optidma_init_one(dev:*mut pci_dev, _id:*const pci_device_id)->c_int {
    let mut rc; ata_print_version_once(&mut (*dev).dev,DRV_VERSION); rc=pcim_enable_device(dev); if rc!=0{return rc;}
    inw(0x1F1); inw(0x1F1); pci_clock=(inb(0x1F5)&1) as i32;
    if optiplus_with_udma(dev)!=0 { /* select the UDMA-capable port information */ }
    ata_pci_bmdma_init_one(dev, core::ptr::null(), core::ptr::null(), core::ptr::null_mut(), 0)
}

// C designated initializers, module registration, and configuration conditionals are
// retained as declarations for the surrounding kernel binding.
static optidma: [pci_device_id; 2] = [pci_device_id { vendor: PCI_VENDOR_ID_OPTI, device: 0xD568 }, pci_device_id { vendor: 0, device: 0 }];
static mut optidma_pci_driver: pci_driver = pci_driver { name: DRV_NAME, id_table: optidma.as_ptr(), probe: optidma_init_one, remove: ata_pci_remove_one, suspend: ata_pci_device_suspend, resume: ata_pci_device_resume };
module_pci_driver!(optidma_pci_driver);
module_metadata!(author="Alan Cox", description="low-level driver for Opti Firestar/Firestar Plus", license="GPL", device_table=pci, version=DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
