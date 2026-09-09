// SPDX-License-Identifier: GPL-2.0
/*
 * sbus.c: UltraSparc SBUS controller support.
 *
 * Copyright (C) 1999 David S. Miller (davem@redhat.com)
 */

// Linux and architecture headers from the original source provide the
// external types, constants, and functions referenced below.

const MAP_BASE: u32 = 0xc0000000;

const SYSIO_IOMMUREG_BASE: usize = 0x2400;
const IOMMU_CONTROL: usize = 0x2400 - 0x2400;
const IOMMU_TSBBASE: usize = 0x2408 - 0x2400;
const IOMMU_FLUSH: usize = 0x2410 - 0x2400;
const IOMMU_VADIAG: usize = 0x4400 - 0x2400;
const IOMMU_TAGCMP: usize = 0x4408 - 0x2400;
const IOMMU_LRUDIAG: usize = 0x4500 - 0x2400;
const IOMMU_TAGDIAG: usize = 0x4580 - 0x2400;
const IOMMU_DRAMDIAG: usize = 0x4600 - 0x2400;
const IOMMU_DRAM_VALID: u64 = 1u64 << 30;

const SYSIO_STRBUFREG_BASE: usize = 0x2800;
const STRBUF_CONTROL: usize = 0x2800 - 0x2800;
const STRBUF_PFLUSH: usize = 0x2808 - 0x2800;
const STRBUF_FSYNC: usize = 0x2810 - 0x2800;
const STRBUF_DRAMDIAG: usize = 0x5000 - 0x2800;
const STRBUF_ERRDIAG: usize = 0x5400 - 0x2800;
const STRBUF_PTAGDIAG: usize = 0x5800 - 0x2800;
const STRBUF_LTAGDIAG: usize = 0x5900 - 0x2800;
const STRBUF_TAG_VALID: u64 = 0x02;

/* Enable 64-bit DVMA mode for the given device. */
pub unsafe fn sbus_set_sbus64(dev: *mut device, bursts: i32) {
    let iommu = (*dev).archdata.iommu;
    let op = to_platform_device(dev);
    let regs = of_get_property((*op).dev.of_node, b"reg\0".as_ptr() as *const i8, core::ptr::null_mut());
    if regs.is_null() {
        printk(KERN_ERR, b"sbus_set_sbus64: Cannot find regs for %pOF\n\0".as_ptr(), (*op).dev.of_node);
        return;
    }
    let slot = (*regs).which_io;
    let mut cfg_reg = (*iommu).write_complete_reg;
    cfg_reg += match slot {
        0 => 0x20, 1 => 0x28, 2 => 0x30, 3 => 0x38,
        13 => 0x40, 14 => 0x48, 15 => 0x50,
        _ => return,
    };
    let mut val = upa_readq(cfg_reg);
    if val & (1u64 << 14) != 0 { return; }
    val |= 1u64 << 14;
    if bursts & DMA_BURST8 != 0 { val |= 1u64 << 1; }
    if bursts & DMA_BURST16 != 0 { val |= 1u64 << 2; }
    if bursts & DMA_BURST32 != 0 { val |= 1u64 << 3; }
    if bursts & DMA_BURST64 != 0 { val |= 1u64 << 4; }
    upa_writeq(val, cfg_reg);
}

const SYSIO_IMAP_SLOT0: usize = 0x2c00;
const SYSIO_IMAP_SLOT1: usize = 0x2c08;
const SYSIO_IMAP_SLOT2: usize = 0x2c10;
const SYSIO_IMAP_SLOT3: usize = 0x2c18;
const SYSIO_IMAP_SCSI: usize = 0x3000;
const SYSIO_IMAP_ETH: usize = 0x3008;
const SYSIO_IMAP_BPP: usize = 0x3010;
const SYSIO_IMAP_AUDIO: usize = 0x3018;
const SYSIO_IMAP_PFAIL: usize = 0x3020;
const SYSIO_IMAP_KMS: usize = 0x3028;
const SYSIO_IMAP_FLPY: usize = 0x3030;
const SYSIO_IMAP_SHW: usize = 0x3038;
const SYSIO_IMAP_KBD: usize = 0x3040;
const SYSIO_IMAP_MS: usize = 0x3048;
const SYSIO_IMAP_SER: usize = 0x3050;
const SYSIO_IMAP_TIM0: usize = 0x3060;
const SYSIO_IMAP_TIM1: usize = 0x3068;
const SYSIO_IMAP_UE: usize = 0x3070;
const SYSIO_IMAP_CE: usize = 0x3078;
const SYSIO_IMAP_SBERR: usize = 0x3080;
const SYSIO_IMAP_PMGMT: usize = 0x3088;
const SYSIO_IMAP_GFX: usize = 0x3090;
const SYSIO_IMAP_EUPA: usize = 0x3098;

static SYSIO_IRQ_OFFSETS: [usize; 56] = [
    SYSIO_IMAP_SLOT0, SYSIO_IMAP_SLOT0, SYSIO_IMAP_SLOT0, SYSIO_IMAP_SLOT0, SYSIO_IMAP_SLOT0, SYSIO_IMAP_SLOT0, SYSIO_IMAP_SLOT0, SYSIO_IMAP_SLOT0,
    SYSIO_IMAP_SLOT1, SYSIO_IMAP_SLOT1, SYSIO_IMAP_SLOT1, SYSIO_IMAP_SLOT1, SYSIO_IMAP_SLOT1, SYSIO_IMAP_SLOT1, SYSIO_IMAP_SLOT1, SYSIO_IMAP_SLOT1,
    SYSIO_IMAP_SLOT2, SYSIO_IMAP_SLOT2, SYSIO_IMAP_SLOT2, SYSIO_IMAP_SLOT2, SYSIO_IMAP_SLOT2, SYSIO_IMAP_SLOT2, SYSIO_IMAP_SLOT2, SYSIO_IMAP_SLOT2,
    SYSIO_IMAP_SLOT3, SYSIO_IMAP_SLOT3, SYSIO_IMAP_SLOT3, SYSIO_IMAP_SLOT3, SYSIO_IMAP_SLOT3, SYSIO_IMAP_SLOT3, SYSIO_IMAP_SLOT3, SYSIO_IMAP_SLOT3,
    SYSIO_IMAP_SCSI, SYSIO_IMAP_ETH, SYSIO_IMAP_BPP, usize::MAX, SYSIO_IMAP_AUDIO, SYSIO_IMAP_PFAIL, usize::MAX, usize::MAX,
    SYSIO_IMAP_KMS, SYSIO_IMAP_FLPY, SYSIO_IMAP_SHW, SYSIO_IMAP_KBD, SYSIO_IMAP_MS, SYSIO_IMAP_SER, usize::MAX, usize::MAX,
    SYSIO_IMAP_TIM0, SYSIO_IMAP_TIM1, usize::MAX, usize::MAX, SYSIO_IMAP_UE, SYSIO_IMAP_CE, SYSIO_IMAP_SBERR, SYSIO_IMAP_PMGMT,
];

const SYSIO_ICLR_UNUSED0: usize = 0x3400;
const SYSIO_ICLR_SLOT0: usize = 0x3408;
const SYSIO_ICLR_SLOT1: usize = 0x3448;
const SYSIO_ICLR_SLOT2: usize = 0x3488;
const SYSIO_ICLR_SLOT3: usize = 0x34c8;

unsafe fn sysio_imap_to_iclr(imap: usize) -> usize { imap + SYSIO_ICLR_UNUSED0 - SYSIO_IMAP_SLOT0 }

unsafe fn sbus_build_irq(op: *mut platform_device, ino: u32) -> u32 {
    let iommu = (*op).dev.archdata.iommu;
    let reg_base = (*iommu).write_complete_reg - 0x2000;
    let imap = SYSIO_IRQ_OFFSETS[ino as usize];
    if imap == usize::MAX { prom_printf(b"get_irq_translations: Bad SYSIO INO[%x]\0".as_ptr(), ino); prom_halt(); }
    let imap = imap + reg_base;
    let (iclr, level) = if ino >= 0x20 {
        (sysio_imap_to_iclr(imap), 0)
    } else {
        let slot = ((ino & 0x18) >> 3) as usize;
        let level = (ino & 0x7) as usize;
        let base = match slot { 0 => SYSIO_ICLR_SLOT0, 1 => SYSIO_ICLR_SLOT1, 2 => SYSIO_ICLR_SLOT2, _ => SYSIO_ICLR_SLOT3 };
        (reg_base + base + (level.wrapping_sub(1)) * 8, level)
    };
    build_irq(level as i32, iclr, imap)
}

const SYSIO_UE_AFSR: usize = 0x0030;
const SYSIO_UE_AFAR: usize = 0x0038;
const SYSIO_UEAFSR_PPIO: u64 = 0x8000000000000000;
const SYSIO_UEAFSR_PDRD: u64 = 0x4000000000000000;
const SYSIO_UEAFSR_PDWR: u64 = 0x2000000000000000;
const SYSIO_UEAFSR_SPIO: u64 = 0x1000000000000000;
const SYSIO_UEAFSR_SDRD: u64 = 0x0800000000000000;
const SYSIO_UEAFSR_SDWR: u64 = 0x0400000000000000;
const SYSIO_UEAFSR_DOFF: u64 = 0x0000e00000000000;
const SYSIO_UEAFSR_SIZE: u64 = 0x00001c0000000000;
const SYSIO_UEAFSR_MID: u64 = 0x000003e000000000;

unsafe fn sysio_ue_handler(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let op = dev_id as *mut platform_device; let iommu = (*op).dev.archdata.iommu;
    let base = (*iommu).write_complete_reg - 0x2000; let afsr_reg = base + SYSIO_UE_AFSR; let afar_reg = base + SYSIO_UE_AFAR;
    let afsr = upa_readq(afsr_reg); let afar = upa_readq(afar_reg);
    let error_bits = afsr & (SYSIO_UEAFSR_PPIO | SYSIO_UEAFSR_PDRD | SYSIO_UEAFSR_PDWR | SYSIO_UEAFSR_SPIO | SYSIO_UEAFSR_SDRD | SYSIO_UEAFSR_SDWR);
    upa_writeq(error_bits, afsr_reg);
    let portid = of_getintprop_default((*op).dev.of_node, b"portid\0".as_ptr() as *const i8, -1);
    printk(b"SYSIO[%x]: Uncorrectable ECC Error, primary error type[%s]\n\0".as_ptr(), portid, if error_bits & SYSIO_UEAFSR_PPIO != 0 { b"PIO\0".as_ptr() } else if error_bits & SYSIO_UEAFSR_PDRD != 0 { b"DVMA Read\0".as_ptr() } else if error_bits & SYSIO_UEAFSR_PDWR != 0 { b"DVMA Write\0".as_ptr() } else { b"???\0".as_ptr() });
    printk(b"SYSIO[%x]: DOFF[%lx] SIZE[%lx] MID[%lx]\n\0".as_ptr(), portid, (afsr & SYSIO_UEAFSR_DOFF) >> 45, (afsr & SYSIO_UEAFSR_SIZE) >> 42, (afsr & SYSIO_UEAFSR_MID) >> 37);
    printk(b"SYSIO[%x]: AFAR[%016lx]\n\0".as_ptr(), portid, afar);
    printk(b"SYSIO[%x]: Secondary UE errors [\0".as_ptr(), portid);
    let mut reported = 0; if afsr & SYSIO_UEAFSR_SPIO != 0 { reported += 1; printk(b"(PIO)\0".as_ptr()); } if afsr & SYSIO_UEAFSR_SDRD != 0 { reported += 1; printk(b"(DVMA Read)\0".as_ptr()); } if afsr & SYSIO_UEAFSR_SDWR != 0 { reported += 1; printk(b"(DVMA Write)\0".as_ptr()); } if reported == 0 { printk(b"(none)\0".as_ptr()); } printk(b"]\n\0".as_ptr());
    IRQ_HANDLED
}

const SYSIO_CE_AFSR: usize = 0x0040; const SYSIO_CE_AFAR: usize = 0x0048;
const SYSIO_CEAFSR_PPIO: u64 = 0x8000000000000000; const SYSIO_CEAFSR_PDRD: u64 = 0x4000000000000000; const SYSIO_CEAFSR_PDWR: u64 = 0x2000000000000000;
const SYSIO_CEAFSR_SPIO: u64 = 0x1000000000000000; const SYSIO_CEAFSR_SDRD: u64 = 0x0800000000000000; const SYSIO_CEAFSR_SDWR: u64 = 0x0400000000000000;
const SYSIO_CEAFSR_ESYND: u64 = 0x00ff000000000000; const SYSIO_CEAFSR_DOFF: u64 = 0x0000e00000000000; const SYSIO_CEAFSR_SIZE: u64 = 0x00001c0000000000; const SYSIO_CEAFSR_MID: u64 = 0x000003e000000000;

unsafe fn sysio_ce_handler(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let op = dev_id as *mut platform_device; let iommu = (*op).dev.archdata.iommu; let base = (*iommu).write_complete_reg - 0x2000; let ar = base + SYSIO_CE_AFSR; let aar = base + SYSIO_CE_AFAR; let afsr = upa_readq(ar); let afar = upa_readq(aar);
    let eb = afsr & (SYSIO_CEAFSR_PPIO | SYSIO_CEAFSR_PDRD | SYSIO_CEAFSR_PDWR | SYSIO_CEAFSR_SPIO | SYSIO_CEAFSR_SDRD | SYSIO_CEAFSR_SDWR); upa_writeq(eb, ar); let portid = of_getintprop_default((*op).dev.of_node, b"portid\0".as_ptr() as *const i8, -1);
    printk(b"SYSIO[%x]: Correctable ECC Error, primary error type[%s]\n\0".as_ptr(), portid, if eb & SYSIO_CEAFSR_PPIO != 0 { b"PIO\0".as_ptr() } else if eb & SYSIO_CEAFSR_PDRD != 0 { b"DVMA Read\0".as_ptr() } else if eb & SYSIO_CEAFSR_PDWR != 0 { b"DVMA Write\0".as_ptr() } else { b"???\0".as_ptr() });
    printk(b"SYSIO[%x]: DOFF[%lx] ECC Syndrome[%lx] Size[%lx] MID[%lx]\n\0".as_ptr(), portid, (afsr & SYSIO_CEAFSR_DOFF)>>45, (afsr & SYSIO_CEAFSR_ESYND)>>48, (afsr & SYSIO_CEAFSR_SIZE)>>42, (afsr & SYSIO_CEAFSR_MID)>>37); printk(b"SYSIO[%x]: AFAR[%016lx]\n\0".as_ptr(), portid, afar); printk(b"SYSIO[%x]: Secondary CE errors [\0".as_ptr(), portid);
    let mut reported=0; if afsr&SYSIO_CEAFSR_SPIO!=0 {reported+=1; printk(b"(PIO)\0".as_ptr());} if afsr&SYSIO_CEAFSR_SDRD!=0 {reported+=1; printk(b"(DVMA Read)\0".as_ptr());} if afsr&SYSIO_CEAFSR_SDWR!=0 {reported+=1; printk(b"(DVMA Write)\0".as_ptr());} if reported==0 {printk(b"(none)\0".as_ptr());} printk(b"]\n\0".as_ptr()); IRQ_HANDLED
}

const SYSIO_SBUS_AFSR: usize = 0x2010; const SYSIO_SBUS_AFAR: usize = 0x2018;
const SYSIO_SBAFSR_PLE: u64 = 0x8000000000000000; const SYSIO_SBAFSR_PTO: u64 = 0x4000000000000000; const SYSIO_SBAFSR_PBERR: u64 = 0x2000000000000000; const SYSIO_SBAFSR_SLE: u64 = 0x1000000000000000; const SYSIO_SBAFSR_STO: u64 = 0x0800000000000000; const SYSIO_SBAFSR_SBERR: u64 = 0x0400000000000000; const SYSIO_SBAFSR_RD: u64 = 0x0000800000000000; const SYSIO_SBAFSR_SIZE: u64 = 0x00001c0000000000; const SYSIO_SBAFSR_MID: u64 = 0x000003e000000000;

unsafe fn sysio_sbus_error_handler(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let op=dev_id as *mut platform_device; let iommu=(*op).dev.archdata.iommu; let base=(*iommu).write_complete_reg-0x2000; let ar=base+SYSIO_SBUS_AFSR; let aar=base+SYSIO_SBUS_AFAR; let afsr=upa_readq(ar); let afar=upa_readq(aar); let eb=afsr&(SYSIO_SBAFSR_PLE|SYSIO_SBAFSR_PTO|SYSIO_SBAFSR_PBERR|SYSIO_SBAFSR_SLE|SYSIO_SBAFSR_STO|SYSIO_SBAFSR_SBERR); upa_writeq(eb,ar); let portid=of_getintprop_default((*op).dev.of_node,b"portid\0".as_ptr() as *const i8,-1);
    printk(b"SYSIO[%x]: SBUS Error, primary error type[%s] read(%d)\n\0".as_ptr(),portid,if eb&SYSIO_SBAFSR_PLE!=0{b"Late PIO Error\0".as_ptr()}else if eb&SYSIO_SBAFSR_PTO!=0{b"Time Out\0".as_ptr()}else if eb&SYSIO_SBAFSR_PBERR!=0{b"Error Ack\0".as_ptr()}else{b"???\0".as_ptr()},if afsr&SYSIO_SBAFSR_RD!=0{1}else{0}); printk(b"SYSIO[%x]: size[%lx] MID[%lx]\n\0".as_ptr(),portid,(afsr&SYSIO_SBAFSR_SIZE)>>42,(afsr&SYSIO_SBAFSR_MID)>>37); printk(b"SYSIO[%x]: AFAR[%016lx]\n\0".as_ptr(),portid,afar); printk(b"SYSIO[%x]: Secondary SBUS errors [\0".as_ptr(),portid); let mut reported=0; if afsr&SYSIO_SBAFSR_SLE!=0{reported+=1;printk(b"(Late PIO Error)\0".as_ptr());} if afsr&SYSIO_SBAFSR_STO!=0{reported+=1;printk(b"(Time Out)\0".as_ptr());} if afsr&SYSIO_SBAFSR_SBERR!=0{reported+=1;printk(b"(Error Ack)\0".as_ptr());} if reported==0{printk(b"(none)\0".as_ptr());} printk(b"]\n\0".as_ptr()); IRQ_HANDLED
}

const ECC_CONTROL: usize = 0x0020; const SYSIO_ECNTRL_ECCEN:u64=0x8000000000000000; const SYSIO_ECNTRL_UEEN:u64=0x4000000000000000; const SYSIO_ECNTRL_CEEN:u64=0x2000000000000000; const SYSIO_UE_INO:u32=0x34; const SYSIO_CE_INO:u32=0x35; const SYSIO_SBUSERR_INO:u32=0x36;

unsafe fn sysio_register_error_handlers(op:*mut platform_device) { let iommu=(*op).dev.archdata.iommu; let base=(*iommu).write_complete_reg-0x2000; let portid=of_getintprop_default((*op).dev.of_node,b"portid\0".as_ptr() as *const i8,-1); let irq=sbus_build_irq(op,SYSIO_UE_INO); if request_irq(irq,sysio_ue_handler,0,b"SYSIO_UE\0".as_ptr(),op as *mut core::ffi::c_void)<0{prom_printf(b"SYSIO[%x]: Cannot register UE interrupt.\n\0".as_ptr(),portid);prom_halt();} let irq=sbus_build_irq(op,SYSIO_CE_INO); if request_irq(irq,sysio_ce_handler,0,b"SYSIO_CE\0".as_ptr(),op as *mut core::ffi::c_void)<0{prom_printf(b"SYSIO[%x]: Cannot register CE interrupt.\n\0".as_ptr(),portid);prom_halt();} let irq=sbus_build_irq(op,SYSIO_SBUSERR_INO); if request_irq(irq,sysio_sbus_error_handler,0,b"SYSIO_SBERR\0".as_ptr(),op as *mut core::ffi::c_void)<0{prom_printf(b"SYSIO[%x]: Cannot register SBUS Error interrupt.\n\0".as_ptr(),portid);prom_halt();} upa_writeq(SYSIO_ECNTRL_ECCEN|SYSIO_ECNTRL_UEEN|SYSIO_ECNTRL_CEEN,base+ECC_CONTROL); let mut control=upa_readq((*iommu).write_complete_reg); control|=0x100; upa_writeq(control,(*iommu).write_complete_reg); }

unsafe fn sbus_iommu_init(op:*mut platform_device) { let dp=(*op).dev.of_node; let pr=of_get_property(dp,b"reg\0".as_ptr() as *const i8,core::ptr::null_mut()); if pr.is_null(){prom_printf(b"sbus_iommu_init: Cannot map SYSIO control registers.\n\0".as_ptr());prom_halt();} let regs=(*pr).phys_addr; let iommu=kzalloc_obj_iommu(GFP_ATOMIC); let strbuf=kzalloc_obj_strbuf(GFP_ATOMIC); if iommu.is_null()||strbuf.is_null(){kfree(iommu);kfree(strbuf);prom_printf(b"sbus_iommu_init: Fatal memory allocation error.\n\0".as_ptr());return;} (*op).dev.archdata.iommu=iommu;(*op).dev.archdata.stc=strbuf;(*op).dev.archdata.numa_node=NUMA_NO_NODE; let base=regs+SYSIO_IOMMUREG_BASE;(*iommu).iommu_control=base+IOMMU_CONTROL;(*iommu).iommu_tsbbase=base+IOMMU_TSBBASE;(*iommu).iommu_flush=base+IOMMU_FLUSH;(*iommu).iommu_tags=(*iommu).iommu_control+IOMMU_TAGDIAG-IOMMU_CONTROL; let base=regs+SYSIO_STRBUFREG_BASE;(*strbuf).strbuf_control=base+STRBUF_CONTROL;(*strbuf).strbuf_pflush=base+STRBUF_PFLUSH;(*strbuf).strbuf_fsync=base+STRBUF_FSYNC;(*strbuf).strbuf_enabled=1; (*iommu).write_complete_reg=regs+0x2000; let portid=of_getintprop_default(dp,b"portid\0".as_ptr() as *const i8,-1); printk(KERN_INFO,b"SYSIO: UPA portID %x, at %016lx\n\0".as_ptr(),portid,regs); iommu_table_init(iommu,IO_TSB_SIZE,MAP_BASE,0xffffffff,-1); upa_writeq((7u64<<16)|(1<<1)|1,(*iommu).iommu_control); for i in 0..16{upa_writeq(0,(*iommu).iommu_control+IOMMU_DRAMDIAG-IOMMU_CONTROL+i*8);upa_writeq(0,(*iommu).iommu_control+IOMMU_TAGDIAG-IOMMU_CONTROL+i*8);} upa_readq((*iommu).write_complete_reg); upa_writeq(__pa((*iommu).page_table),(*iommu).iommu_tsbbase); upa_writeq(3,(*strbuf).strbuf_control); for i in 0..16{upa_writeq(0,(*strbuf).strbuf_control+STRBUF_PTAGDIAG-STRBUF_CONTROL+i*8);upa_writeq(0,(*strbuf).strbuf_control+STRBUF_LTAGDIAG-STRBUF_CONTROL+i*8);} let mut control=upa_readq((*iommu).write_complete_reg);control|=0x3f;upa_writeq(control,(*iommu).write_complete_reg);if this_is_starfire{starfire_hookup(portid);}sysio_register_error_handlers(op); }

unsafe fn sbus_init() -> i32 { let mut dp=core::ptr::null_mut(); for_each_node_by_name(&mut dp,b"sbus\0".as_ptr() as *const i8){let op=of_find_device_by_node(dp);sbus_iommu_init(op);of_propagate_archdata(op);} 0 }

// Original source uses subsys_initcall(sbus_init).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
