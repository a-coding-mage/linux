// SPDX-License-Identifier: GPL-2.0
/* pci_psycho.c: PSYCHO/U2P specific PCI controller support. */

// Linux/kernel, PCI, OF, platform, SPARC, and local headers supply the
// declarations referenced below.

const DRIVER_NAME: &str = "psycho";
const PFX: &str = "psycho: ";

const PSYCHO_CONTROL: u64 = 0x0010;
const PSYCHO_CONTROL_APCKEN: u64 = 0x8;
const PSYCHO_CONTROL_APERR: u64 = 0x4;
const PSYCHO_CONTROL_IAP: u64 = 0x2;
const PSYCHO_CONTROL_MODE: u64 = 0x1;
const PSYCHO_PCIA_CTRL: u64 = 0x2000;
const PSYCHO_PCIB_CTRL: u64 = 0x4000;
const PSYCHO_PCICTRL_SBH_ERR: u64 = 0x0000000800000000;
const PSYCHO_PCICTRL_SERR: u64 = 0x0000000400000000;
const PSYCHO_PCICTRL_ARB_PARK: u64 = 0x200000;
const PSYCHO_PCICTRL_SBH_INT: u64 = 0x400;
const PSYCHO_PCICTRL_EEN: u64 = 0x100;
const PSYCHO_PCICTRL_AEN: u64 = 0x3f;

const PSYCHO_STRBUF_CONTROL_A: u64 = 0x2800;
const PSYCHO_STRBUF_CONTROL_B: u64 = 0x4800;
const PSYCHO_STRBUF_CTRL_LPTR: u64 = 0xf0;
const PSYCHO_STRBUF_CTRL_LENAB: u64 = 0x8;
const PSYCHO_STRBUF_CTRL_RRDIS: u64 = 0x4;
const PSYCHO_STRBUF_CTRL_ENAB: u64 = 0x1;
const PSYCHO_STRBUF_FLUSH_A: u64 = 0x2808;
const PSYCHO_STRBUF_FLUSH_B: u64 = 0x4808;
const PSYCHO_STRBUF_FSYNC_A: u64 = 0x2810;
const PSYCHO_STRBUF_FSYNC_B: u64 = 0x4810;
const PSYCHO_STC_ERR_A: u64 = 0xb400;
const PSYCHO_STC_ERR_B: u64 = 0xc400;
const PSYCHO_STC_TAG_A: u64 = 0xb800;
const PSYCHO_STC_TAG_B: u64 = 0xc800;
const PSYCHO_STC_LINE_A: u64 = 0xb900;
const PSYCHO_STC_LINE_B: u64 = 0xc900;

const PSYCHO_UE_AFSR: u64 = 0x30;
const PSYCHO_UE_AFAR: u64 = 0x38;
const PSYCHO_UEAFSR_PPIO: u64 = 0x8000000000000000;
const PSYCHO_UEAFSR_PDRD: u64 = 0x4000000000000000;
const PSYCHO_UEAFSR_PDWR: u64 = 0x2000000000000000;
const PSYCHO_UEAFSR_SPIO: u64 = 0x1000000000000000;
const PSYCHO_UEAFSR_SDRD: u64 = 0x0800000000000000;
const PSYCHO_UEAFSR_SDWR: u64 = 0x0400000000000000;
const PSYCHO_UEAFSR_BMSK: u64 = 0x0000ffff00000000;
const PSYCHO_UEAFSR_DOFF: u64 = 0x00000000e0000000;
const PSYCHO_UEAFSR_MID: u64 = 0x1f000000;
const PSYCHO_UEAFSR_BLK: u64 = 0x800000;

const PSYCHO_CE_AFSR: u64 = 0x40;
const PSYCHO_CE_AFAR: u64 = 0x40;
const PSYCHO_CEAFSR_PPIO: u64 = 0x8000000000000000;
const PSYCHO_CEAFSR_PDRD: u64 = 0x4000000000000000;
const PSYCHO_CEAFSR_PDWR: u64 = 0x2000000000000000;
const PSYCHO_CEAFSR_SPIO: u64 = 0x1000000000000000;
const PSYCHO_CEAFSR_SDRD: u64 = 0x0800000000000000;
const PSYCHO_CEAFSR_SDWR: u64 = 0x0400000000000000;
const PSYCHO_CEAFSR_ESYND: u64 = 0x00ff000000000000;
const PSYCHO_CEAFSR_BMSK: u64 = 0x0000ffff00000000;
const PSYCHO_CEAFSR_DOFF: u64 = 0x00000000e0000000;
const PSYCHO_CEAFSR_MID: u64 = 0x1f000000;
const PSYCHO_CEAFSR_BLK: u64 = 0x800000;

const PSYCHO_PCI_AFSR_A: u64 = 0x2010;
const PSYCHO_PCI_AFSR_B: u64 = 0x4010;
const PSYCHO_PCI_AFAR_A: u64 = 0x2018;
const PSYCHO_PCI_AFAR_B: u64 = 0x4018;
const PSYCHO_ECC_CTRL: u64 = 0x20;
const PSYCHO_ECCCTRL_EE: u64 = 0x8000000000000000;
const PSYCHO_ECCCTRL_UE: u64 = 0x4000000000000000;
const PSYCHO_ECCCTRL_CE: u64 = 0x2000000000000000;

unsafe fn psycho_ue_intr(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let pbm = dev_id as *mut pci_pbm_info;
    let afsr_reg = (*pbm).controller_regs + PSYCHO_UE_AFSR as usize;
    let afar_reg = (*pbm).controller_regs + PSYCHO_UE_AFAR as usize;
    let afar = upa_readq(afar_reg);
    let afsr = upa_readq(afsr_reg);
    let error_bits = afsr & (PSYCHO_UEAFSR_PPIO | PSYCHO_UEAFSR_PDRD | PSYCHO_UEAFSR_PDWR |
        PSYCHO_UEAFSR_SPIO | PSYCHO_UEAFSR_SDRD | PSYCHO_UEAFSR_SDWR);
    if error_bits == 0 { return IRQ_NONE; }
    upa_writeq(error_bits, afsr_reg);
    printk!("%s: Uncorrectable Error, primary error type[%s]\n", (*pbm).name,
        if error_bits & PSYCHO_UEAFSR_PPIO != 0 { "PIO" } else if error_bits & PSYCHO_UEAFSR_PDRD != 0 { "DMA Read" } else if error_bits & PSYCHO_UEAFSR_PDWR != 0 { "DMA Write" } else { "???" });
    printk!("%s: bytemask[%04lx] dword_offset[%lx] UPA_MID[%02lx] was_block(%d)\n", (*pbm).name,
        (afsr & PSYCHO_UEAFSR_BMSK) >> 32, (afsr & PSYCHO_UEAFSR_DOFF) >> 29,
        (afsr & PSYCHO_UEAFSR_MID) >> 24, if afsr & PSYCHO_UEAFSR_BLK != 0 { 1 } else { 0 });
    printk!("%s: UE AFAR [%016lx]\n", (*pbm).name, afar);
    printk!("%s: UE Secondary errors [", (*pbm).name);
    let mut reported = 0;
    if afsr & PSYCHO_UEAFSR_SPIO != 0 { reported += 1; printk!("(PIO)"); }
    if afsr & PSYCHO_UEAFSR_SDRD != 0 { reported += 1; printk!("(DMA Read)"); }
    if afsr & PSYCHO_UEAFSR_SDWR != 0 { reported += 1; printk!("(DMA Write)"); }
    if reported == 0 { printk!("(none)"); }
    printk!("]\n");
    psycho_check_iommu_error(pbm, afsr, afar, UE_ERR);
    if !(*pbm).sibling.is_null() { psycho_check_iommu_error((*pbm).sibling, afsr, afar, UE_ERR); }
    IRQ_HANDLED
}

unsafe fn psycho_ce_intr(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let pbm = dev_id as *mut pci_pbm_info;
    let afsr_reg = (*pbm).controller_regs + PSYCHO_CE_AFSR as usize;
    let afar_reg = (*pbm).controller_regs + PSYCHO_CE_AFAR as usize;
    let afar = upa_readq(afar_reg); let afsr = upa_readq(afsr_reg);
    let error_bits = afsr & (PSYCHO_CEAFSR_PPIO | PSYCHO_CEAFSR_PDRD | PSYCHO_CEAFSR_PDWR |
        PSYCHO_CEAFSR_SPIO | PSYCHO_CEAFSR_SDRD | PSYCHO_CEAFSR_SDWR);
    if error_bits == 0 { return IRQ_NONE; }
    upa_writeq(error_bits, afsr_reg);
    printk!("%s: Correctable Error, primary error type[%s]\n", (*pbm).name,
        if error_bits & PSYCHO_CEAFSR_PPIO != 0 { "PIO" } else if error_bits & PSYCHO_CEAFSR_PDRD != 0 { "DMA Read" } else if error_bits & PSYCHO_CEAFSR_PDWR != 0 { "DMA Write" } else { "???" });
    printk!("%s: syndrome[%02lx] bytemask[%04lx] dword_offset[%lx] UPA_MID[%02lx] was_block(%d)\n", (*pbm).name,
        (afsr & PSYCHO_CEAFSR_ESYND) >> 48, (afsr & PSYCHO_CEAFSR_BMSK) >> 32,
        (afsr & PSYCHO_CEAFSR_DOFF) >> 29, (afsr & PSYCHO_CEAFSR_MID) >> 24,
        if afsr & PSYCHO_CEAFSR_BLK != 0 { 1 } else { 0 });
    printk!("%s: CE AFAR [%016lx]\n", (*pbm).name, afar);
    printk!("%s: CE Secondary errors [", (*pbm).name);
    let mut reported = 0;
    if afsr & PSYCHO_CEAFSR_SPIO != 0 { reported += 1; printk!("(PIO)"); }
    if afsr & PSYCHO_CEAFSR_SDRD != 0 { reported += 1; printk!("(DMA Read)"); }
    if afsr & PSYCHO_CEAFSR_SDWR != 0 { reported += 1; printk!("(DMA Write)"); }
    if reported == 0 { printk!("(none)"); }
    printk!("]\n"); IRQ_HANDLED
}

const PSYCHO_IRQ_RETRY: u64 = 0x1a00;
const PSYCHO_PCIA_DIAG: u64 = 0x2020;
const PSYCHO_PCIB_DIAG: u64 = 0x4020;
const PSYCHO_PCIDIAG_DDWSYNC: u64 = 0x10;

unsafe fn psycho_controller_hwinit(pbm: *mut pci_pbm_info) {
    upa_writeq(5, (*pbm).controller_regs + PSYCHO_IRQ_RETRY as usize);
    let mut tmp = upa_readq((*pbm).controller_regs + PSYCHO_PCIA_CTRL as usize); tmp |= PSYCHO_PCICTRL_AEN; upa_writeq(tmp, (*pbm).controller_regs + PSYCHO_PCIA_CTRL as usize);
    tmp = upa_readq((*pbm).controller_regs + PSYCHO_PCIB_CTRL as usize); tmp |= PSYCHO_PCICTRL_AEN; upa_writeq(tmp, (*pbm).controller_regs + PSYCHO_PCIB_CTRL as usize);
    tmp = upa_readq((*pbm).controller_regs + PSYCHO_PCIA_DIAG as usize); tmp |= PSYCHO_PCIDIAG_DDWSYNC; upa_writeq(tmp, (*pbm).controller_regs + PSYCHO_PCIA_DIAG as usize);
    tmp = upa_readq((*pbm).controller_regs + PSYCHO_PCIB_DIAG as usize); tmp |= PSYCHO_PCIDIAG_DDWSYNC; upa_writeq(tmp, (*pbm).controller_regs + PSYCHO_PCIB_DIAG as usize);
}

unsafe fn psycho_pbm_strbuf_init(pbm: *mut pci_pbm_info, is_pbm_a: i32) {
    let base = (*pbm).controller_regs;
    if is_pbm_a != 0 { (*pbm).stc.strbuf_control=base+PSYCHO_STRBUF_CONTROL_A as usize; (*pbm).stc.strbuf_pflush=base+PSYCHO_STRBUF_FLUSH_A as usize; (*pbm).stc.strbuf_fsync=base+PSYCHO_STRBUF_FSYNC_A as usize; (*pbm).stc.strbuf_err_stat=base+PSYCHO_STC_ERR_A as usize; (*pbm).stc.strbuf_tag_diag=base+PSYCHO_STC_TAG_A as usize; (*pbm).stc.strbuf_line_diag=base+PSYCHO_STC_LINE_A as usize; }
    else { (*pbm).stc.strbuf_control=base+PSYCHO_STRBUF_CONTROL_B as usize; (*pbm).stc.strbuf_pflush=base+PSYCHO_STRBUF_FLUSH_B as usize; (*pbm).stc.strbuf_fsync=base+PSYCHO_STRBUF_FSYNC_B as usize; (*pbm).stc.strbuf_err_stat=base+PSYCHO_STC_ERR_B as usize; (*pbm).stc.strbuf_tag_diag=base+PSYCHO_STC_TAG_B as usize; (*pbm).stc.strbuf_line_diag=base+PSYCHO_STC_LINE_B as usize; }
    (*pbm).stc.strbuf_ctxflush=0; (*pbm).stc.strbuf_ctxmatch_base=0;
    (*pbm).stc.strbuf_flushflag = (((&(*pbm).stc.__flushflag_buf[0] as *const _ as usize + 63) & !63) as *mut _);
    (*pbm).stc.strbuf_flushflag_pa = __pa((*pbm).stc.strbuf_flushflag as usize);
    let mut control=upa_readq((*pbm).stc.strbuf_control); control |= PSYCHO_STRBUF_CTRL_ENAB; control &= !(PSYCHO_STRBUF_CTRL_LENAB|PSYCHO_STRBUF_CTRL_LPTR); upa_writeq(control,(*pbm).stc.strbuf_control); (*pbm).stc.strbuf_enabled=1;
}

unsafe fn psycho_pbm_init(pbm:*mut pci_pbm_info, op:*mut platform_device, is_pbm_a:i32) { psycho_pbm_init_common(pbm,op,"PSYCHO",PBM_CHIP_TYPE_PSYCHO); psycho_pbm_strbuf_init(pbm,is_pbm_a); psycho_scan_bus(pbm,&mut (*op).dev); }
unsafe fn psycho_scan_bus(pbm:*mut pci_pbm_info,parent:*mut device) { pbm_config_busmastering(pbm); (*pbm).is_66mhz_capable=0; (*pbm).pci_bus=pci_scan_one_pbm(pbm,parent); psycho_register_error_handlers(pbm); }

// The remaining probe/driver registration follows the same external kernel
// declarations and preserves the original platform-driver entry points.
unsafe fn psycho_find_sibling(upa_portid:u32)->*mut pci_pbm_info { let mut pbm=pci_pbm_root; while !pbm.is_null() { if (*pbm).portid==upa_portid{return pbm;} pbm=(*pbm).next;} core::ptr::null_mut() }

unsafe fn psycho_register_error_handlers(pbm:*mut pci_pbm_info) {
    let op=of_find_device_by_node((*(*pbm).op).dev.of_node); if op.is_null(){return;} if (*op).archdata.num_irqs<6{return;}
    let _=request_irq((*op).archdata.irqs[1],psycho_ue_intr,IRQF_SHARED,"PSYCHO_UE",pbm);
    let _=request_irq((*op).archdata.irqs[2],psycho_ce_intr,IRQF_SHARED,"PSYCHO_CE",pbm);
    let err=request_irq((*op).archdata.irqs[0],psycho_pcierr_intr,IRQF_SHARED,"PSYCHO_PCIERR",pbm);
    if err!=0 { printk!(KERN_WARNING "%s: Could not register PCIERR, err=%d\n",(*pbm).name,err); }
    upa_writeq(PSYCHO_ECCCTRL_EE|PSYCHO_ECCCTRL_UE|PSYCHO_ECCCTRL_CE,(*pbm).controller_regs+PSYCHO_ECC_CTRL as usize);
    for off in [PSYCHO_PCIA_CTRL,PSYCHO_PCIB_CTRL] { let mut tmp=upa_readq((*pbm).controller_regs+off as usize); tmp|=PSYCHO_PCICTRL_SERR|PSYCHO_PCICTRL_SBH_ERR|PSYCHO_PCICTRL_EEN; tmp&=!PSYCHO_PCICTRL_SBH_INT; upa_writeq(tmp,(*pbm).controller_regs+off as usize); }
}
unsafe fn pbm_config_busmastering(pbm:*mut pci_pbm_info) { let addr=psycho_pci_config_mkaddr(pbm,(*pbm).pci_first_busno,0,PCI_CACHE_LINE_SIZE); pci_config_write8(addr,64/std::mem::size_of::<u32>()); let addr=psycho_pci_config_mkaddr(pbm,(*pbm).pci_first_busno,0,PCI_LATENCY_TIMER); pci_config_write8(addr,64); }

const PSYCHO_CONFIGSPACE:u64=0x1000000;
unsafe fn psycho_pbm_probe(op:*mut platform_device)->i32 {
    let dp=(*op).dev.of_node; let upa_portid=of_getintprop_default(dp,"upa-portid",0xff); let pbm=kzalloc_obj::<pci_pbm_info>(); if pbm.is_null(){printk!(KERN_ERR "{}Cannot allocate pci_pbm_info.\n",PFX);return -ENOMEM;}
    (*pbm).sibling=psycho_find_sibling(upa_portid); if (*pbm).sibling.is_null(){(*pbm).iommu=kzalloc_obj::<iommu>(); if (*pbm).iommu.is_null(){kfree(pbm);return -ENOMEM;}} else {(*pbm).iommu=(*(*pbm).sibling).iommu;}
    (*pbm).portid=upa_portid; let pr_regs=of_get_property(dp,"reg",core::ptr::null_mut()); if pr_regs.is_null(){kfree(pbm);return -ENODEV;}
    let is_pbm_a=if ((*pr_regs).phys_addr&0x6000)==0x2000{1}else{0}; (*pbm).controller_regs=(*pr_regs.add(2)).phys_addr; (*pbm).config_space=(*pbm).controller_regs+PSYCHO_CONFIGSPACE as usize;
    if is_pbm_a!=0 {(*pbm).pci_afsr=(*pbm).controller_regs+PSYCHO_PCI_AFSR_A as usize;(*pbm).pci_afar=(*pbm).controller_regs+PSYCHO_PCI_AFAR_A as usize;(*pbm).pci_csr=(*pbm).controller_regs+PSYCHO_PCIA_CTRL as usize;} else {(*pbm).pci_afsr=(*pbm).controller_regs+PSYCHO_PCI_AFSR_B as usize;(*pbm).pci_afar=(*pbm).controller_regs+PSYCHO_PCI_AFAR_B as usize;(*pbm).pci_csr=(*pbm).controller_regs+PSYCHO_PCIB_CTRL as usize;}
    psycho_controller_hwinit(pbm); if (*pbm).sibling.is_null(){let err=psycho_iommu_init(pbm,128,0xc0000000,0xffffffff,PSYCHO_CONTROL);if err!=0{kfree((*pbm).iommu);kfree(pbm);return err;}if this_is_starfire{starfire_hookup((*pbm).portid);}}
    psycho_pbm_init(pbm,op,is_pbm_a);(*pbm).next=pci_pbm_root;pci_pbm_root=pbm;if !(*pbm).sibling.is_null(){(*(*pbm).sibling).sibling=pbm;}dev_set_drvdata(&mut (*op).dev,pbm);0
}

static mut psycho_driver: platform_driver = platform_driver { driver: driver { name: DRIVER_NAME, of_match_table: psycho_match }, probe: Some(psycho_pbm_probe) };
static psycho_match: [of_device_id;2] = [of_device_id { name:"pci", compatible:"pci108e,8000" }, of_device_id { name:core::ptr::null(), compatible:core::ptr::null() }];
unsafe fn psycho_init()->i32 { platform_driver_register(&mut psycho_driver) }
// subsys_initcall(psycho_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
