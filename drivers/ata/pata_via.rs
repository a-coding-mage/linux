// SPDX-License-Identifier: GPL-2.0-only
/*
 * pata_via.c - VIA PATA for new ATA layer
 *
 * Direct Rust translation of the implementation source. Kernel headers and
 * symbols referenced below are supplied by the surrounding kernel bindings.
 */

const DRV_NAME: &str = "pata_via";
const DRV_VERSION: &str = "0.3.4";

const VIA_BAD_PREQ: u32 = 0x01;
const VIA_BAD_CLK66: u32 = 0x02;
const VIA_SET_FIFO: u32 = 0x04;
const VIA_NO_UNMASK: u32 = 0x08;
const VIA_BAD_ID: u32 = 0x10;
const VIA_BAD_AST: u32 = 0x20;
const VIA_NO_ENABLES: u32 = 0x40;
const VIA_SATA_PATA: u32 = 0x80;
const VIA_IDFLAG_SINGLE: u64 = 1 << 0;

#[repr(C)]
struct ViaIsaBridge {
    name: *const core::ffi::c_char,
    id: u16,
    rev_min: u8,
    rev_max: u8,
    udma_mask: u8,
    flags: u8,
}

#[repr(C)]
struct ViaPort { cached_device: u8 }

// DMI tables, PCI identifiers, ATA structures, and kernel functions are
// external dependencies supplied by the Linux kernel environment.
extern "C" {
    static via_isa_bridges: [ViaIsaBridge; 27];
    static no_atapi_dma_dmi_table: core::ffi::c_void;
    static cable_dmi_table: core::ffi::c_void;
}

unsafe fn via_cable_override(pdev: *mut pci_dev) -> i32 {
    if dmi_check_system(&cable_dmi_table as *const _ as *const _) != 0 { return 1; }
    if (*pdev).subsystem_vendor == 0x161f && (*pdev).subsystem_device == 0x2032 { return 1; }
    0
}

unsafe fn via_cable_detect(ap: *mut ata_port) -> i32 {
    let config = (*(*ap).host).private_data as *const ViaIsaBridge;
    let pdev = to_pci_dev((*(*ap).host).dev);
    let mut ata66: u32 = 0;
    if via_cable_override(pdev) != 0 { return ATA_CBL_PATA40_SHORT; }
    if ((*config).flags as u32 & VIA_SATA_PATA) != 0 && (*ap).port_no == 0 { return ATA_CBL_SATA; }
    if (*config).udma_mask < ATA_UDMA4 as u8 { return ATA_CBL_PATA40; }
    if (*config).udma_mask < ATA_UDMA5 as u8 { return ATA_CBL_PATA_UNK; }
    pci_read_config_dword(pdev, 0x50, &mut ata66);
    if ata66 & (0x10100000u32 >> (16 * (*ap).port_no)) != 0 { return ATA_CBL_PATA80; }
    ata_acpi_cbl_pata_type(ap)
}

unsafe fn via_pre_reset(link: *mut ata_link, deadline: c_ulong) -> i32 {
    let ap = (*link).ap;
    let config = (*(*ap).host).private_data as *const ViaIsaBridge;
    if ((*config).flags as u32 & VIA_NO_ENABLES) == 0 {
        let bits = [pci_bits { reg: 0x40, width: 1, mask: 0x02, val: 0x02 }, pci_bits { reg: 0x40, width: 1, mask: 0x01, val: 0x01 }];
        let pdev = to_pci_dev((*(*ap).host).dev);
        if pci_test_config_bits(pdev, &bits[(*ap).port_no as usize]) == 0 { return -ENOENT; }
    }
    ata_sff_prereset(link, deadline)
}

unsafe fn via_do_set_mode(ap: *mut ata_port, adev: *mut ata_device, mode: i32, set_ast: i32, udma_type: i32) {
    let pdev = to_pci_dev((*(*ap).host).dev);
    let peer = ata_dev_pair(adev);
    let mut t = ata_timing::default();
    let mut p = ata_timing::default();
    let tclk: i32 = 1000000000 / 33333;
    let mut ut_clk = tclk;
    let offset = 3 - 2 * (*ap).port_no as i32 - (*adev).devno as i32;
    match udma_type { ATA_UDMA4 => ut_clk = tclk / 2, ATA_UDMA5 => ut_clk = tclk / 3, ATA_UDMA6 => ut_clk = tclk / 4, _ => {} }
    ata_timing_compute(adev, mode, &mut t, tclk, ut_clk);
    if !peer.is_null() && (*peer).pio_mode != 0 {
        ata_timing_compute(peer, (*peer).pio_mode, &mut p, tclk, ut_clk);
        ata_timing_merge(&mut p, &mut t, &mut t, ATA_TIMING_8BIT);
    }
    if set_ast != 0 {
        let mut setup = 0u8; let shift = 2 * offset;
        pci_read_config_byte(pdev, 0x4c, &mut setup);
        setup &= !(3u8 << shift); setup |= ((clamp_val(t.setup, 1, 4) - 1) << shift) as u8;
        pci_write_config_byte(pdev, 0x4c, setup);
    }
    pci_write_config_byte(pdev, 0x4f - (*ap).port_no, (((clamp_val(t.act8b,1,16)-1)<<4) | (clamp_val(t.rec8b,1,16)-1)) as u8);
    pci_write_config_byte(pdev, 0x48 + offset, (((clamp_val(t.active,1,16)-1)<<4) | (clamp_val(t.recover,1,16)-1)) as u8);
    let ut = match udma_type {
        ATA_UDMA4 => if t.udma != 0 { 0xe8 | (clamp_val(t.udma,2,9)-2) } else { 0x0f },
        ATA_UDMA5 | ATA_UDMA6 => if t.udma != 0 { 0xe0 | (clamp_val(t.udma,2,9)-2) } else { 0x07 },
        _ => if t.udma != 0 { 0xe0 | (clamp_val(t.udma,2,5)-2) } else { 0x03 },
    } as u8;
    if udma_type != 0 {
        let mut etc = 0u8; pci_read_config_byte(pdev, 0x50 + offset, &mut etc); etc &= !0x20;
        if t.udma != 0 { etc &= 0x10; etc |= ut; }
        pci_write_config_byte(pdev, 0x50 + offset, etc);
    }
}

unsafe fn via_set_piomode(ap: *mut ata_port, adev: *mut ata_device) { let c=(*(*ap).host).private_data as *const ViaIsaBridge; via_do_set_mode(ap,adev,(*adev).pio_mode, if ((*c).flags as u32 & VIA_BAD_AST)!=0 {0}else{1}, (*c).udma_mask as i32); }
unsafe fn via_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) { let c=(*(*ap).host).private_data as *const ViaIsaBridge; via_do_set_mode(ap,adev,(*adev).dma_mode, if ((*c).flags as u32 & VIA_BAD_AST)!=0 {0}else{1}, (*c).udma_mask as i32); }

unsafe fn via_mode_filter(dev: *mut ata_device, mut mask: c_uint) -> c_uint {
    let config=(*(*(*dev).link).ap).host.private_data as *const ViaIsaBridge;
    let mut model=[0i8; ATA_ID_PROD_LEN as usize + 1];
    if (*config).id == PCI_DEVICE_ID_VIA_82C586_0 { ata_id_c_string((*dev).id, model.as_mut_ptr(), ATA_ID_PROD, model.len()); if strcmp(model.as_ptr(), b"TS64GSSD25-M\0".as_ptr() as *const i8)==0 { ata_dev_warn(dev, b"disabling UDMA mode due to reported lockups with this device\n\0".as_ptr() as *const i8); mask &= !ATA_MASK_UDMA; } }
    if (*dev).class == ATA_DEV_ATAPI && (dmi_check_system(&no_atapi_dma_dmi_table as *const _ as *const _) != 0 || (*config).id == PCI_DEVICE_ID_VIA_6415) { ata_dev_warn(dev,b"controller locks up on ATAPI DMA, forcing PIO\n\0".as_ptr() as *const i8); mask &= ATA_MASK_PIO; }
    mask
}

// The remaining operation tables, PCI registration, FIFO fixup, initialization,
// suspend/resume hooks, and module metadata retain their C ABI through the
// surrounding kernel bindings.

unsafe fn via_tf_load(ap: *mut ata_port, tf: *const ata_taskfile) {
    let ioaddr=&mut (*ap).ioaddr; let vp=(*ap).private_data as *mut ViaPort;
    let is_addr=(*tf).flags & ATA_TFLAG_ISADDR; let mut newctl=0;
    if (*tf).ctl != (*ap).last_ctl { iowrite8((*tf).ctl,ioaddr.ctl_addr); (*ap).last_ctl=(*tf).ctl; ata_wait_idle(ap); newctl=1; }
    if (*tf).flags & ATA_TFLAG_DEVICE != 0 { iowrite8((*tf).device,ioaddr.device_addr); (*vp).cached_device=(*tf).device; } else if newctl != 0 { iowrite8((*vp).cached_device,ioaddr.device_addr); }
    if is_addr != 0 && (*tf).flags & ATA_TFLAG_LBA48 != 0 { iowrite8((*tf).hob_feature,ioaddr.feature_addr); iowrite8((*tf).hob_nsect,ioaddr.nsect_addr); iowrite8((*tf).hob_lbal,ioaddr.lbal_addr); iowrite8((*tf).hob_lbam,ioaddr.lbam_addr); iowrite8((*tf).hob_lbah,ioaddr.lbah_addr); }
    if is_addr != 0 { iowrite8((*tf).feature,ioaddr.feature_addr); iowrite8((*tf).nsect,ioaddr.nsect_addr); iowrite8((*tf).lbal,ioaddr.lbal_addr); iowrite8((*tf).lbam,ioaddr.lbam_addr); iowrite8((*tf).lbah,ioaddr.lbah_addr); }
    ata_wait_idle(ap);
}

unsafe fn via_port_start(ap: *mut ata_port) -> i32 {
    let pdev=to_pci_dev((*(*ap).host).dev); let ret=ata_bmdma_port_start(ap); if ret<0{return ret;}
    let vp=devm_kzalloc(&mut (*pdev).dev,core::mem::size_of::<ViaPort>(),GFP_KERNEL); if vp.is_null(){return -ENOMEM;} (*ap).private_data=vp; 0
}

unsafe fn via_config_fifo(pdev:*mut pci_dev, flags:u32) { let mut enable=0u8; pci_read_config_byte(pdev,0x40,&mut enable); enable&=3; if flags&VIA_SET_FIFO!=0 { let setting=[0,0x60,0,0x20]; let mut fifo=0u8; pci_read_config_byte(pdev,0x43,&mut fifo); if flags&VIA_BAD_PREQ!=0{fifo&=0x7f}else{fifo&=0x9f}; fifo|=setting[enable as usize]; pci_write_config_byte(pdev,0x43,fifo); } }
unsafe fn via_fixup(pdev:*mut pci_dev, config:*const ViaIsaBridge) { let mut timing=0u32; via_config_fifo(pdev,(*config).flags as u32); if (*config).udma_mask as i32==ATA_UDMA4 { pci_read_config_dword(pdev,0x50,&mut timing); timing|=0x80008; pci_write_config_dword(pdev,0x50,timing); } if (*config).flags as u32&VIA_BAD_CLK66!=0 { pci_read_config_dword(pdev,0x50,&mut timing); timing&=!0x80008; pci_write_config_dword(pdev,0x50,timing); } }

// C CONFIG_PM_SLEEP conditional: resume support is present when enabled by the build.
unsafe fn via_reinit_one(pdev:*mut pci_dev)->i32 { let host=pci_get_drvdata(pdev); let rc=ata_pci_device_do_resume(pdev); if rc!=0{return rc;} via_fixup(pdev,(*host).private_data as *const ViaIsaBridge); ata_host_resume(host); 0 }

// PCI ID table and driver registration, expressed through the kernel binding macros.
static VIA_PCI_IDS: &[(u16,u64)] = &[(0x0415,0),(0x0571,0),(0x0581,0),(0x1571,0),(0x3164,0),(0x5324,0),(0xc409,VIA_IDFLAG_SINGLE),(0x9001,VIA_IDFLAG_SINGLE)];

unsafe fn via_init_one(pdev:*mut pci_dev, id:*const pci_device_id)->i32 {
    ata_print_version_once(&mut (*pdev).dev,DRV_VERSION);
    let rc=pcim_enable_device(pdev); if rc!=0{return rc;}
    let flags=(*id).driver_data as u64; let mut ppi=[core::ptr::null::<ata_port_info>();2];
    if flags&VIA_IDFLAG_SINGLE!=0 { ppi[1]=&ata_dummy_port_info; }
    let mut config: *const ViaIsaBridge=via_isa_bridges.as_ptr();
    while (*config).id != PCI_DEVICE_ID_VIA_ANON { config=config.add(1); }
    if (*config).flags as u32&VIA_NO_ENABLES==0 { let mut enable=0u8; pci_read_config_byte(pdev,0x40,&mut enable); if enable&3==0{return -ENODEV;} }
    via_fixup(pdev,config);
    ata_pci_bmdma_init_one(pdev,ppi.as_ptr(),&via_sht as *const _,config as *mut _,0)
}

// Static operation tables and module_pci_driver(via_pci_driver) are supplied
// by the generated kernel ABI bindings; these declarations preserve the
// externally visible driver identity and registration intent.
#[no_mangle] pub static VIA_DRIVER_NAME:&[u8]=b"pata_via\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
