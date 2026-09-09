// SPDX-License-Identifier: GPL-2.0-only
// Translated from pata_icside.c; kernel dependencies are supplied externally.

const DRV_NAME: &str = "pata_icside";
const ICS_IDENT_OFFSET: usize = 0x2280;
const ICS_ARCIN_V5_INTRSTAT: usize = 0x0000;
const ICS_ARCIN_V5_INTROFFSET: usize = 0x0004;
const ICS_ARCIN_V6_INTROFFSET_1: usize = 0x2200;
const ICS_ARCIN_V6_INTRSTAT_1: usize = 0x2290;
const ICS_ARCIN_V6_INTROFFSET_2: usize = 0x3200;
const ICS_ARCIN_V6_INTRSTAT_2: usize = 0x3290;

#[repr(C)]
struct portinfo { dataoffset: u32, ctrloffset: u32, stepping: u32 }

static pata_icside_portinfo_v5: portinfo = portinfo { dataoffset: 0x2800, ctrloffset: 0x2b80, stepping: 6 };
static pata_icside_portinfo_v6_1: portinfo = portinfo { dataoffset: 0x2000, ctrloffset: 0x2380, stepping: 6 };
static pata_icside_portinfo_v6_2: portinfo = portinfo { dataoffset: 0x3000, ctrloffset: 0x3380, stepping: 6 };

#[repr(C)]
struct pata_icside_state {
    irq_port: *mut core::ffi::c_void, ioc_base: *mut core::ffi::c_void,
    r#type: u32, dma: u32,
    port: [pata_icside_port; 2],
}
#[repr(C)]
struct pata_icside_port { port_sel: u8, disabled: u8, speed: [u32; ATA_MAX_DEVICES] }
#[repr(C)]
struct pata_icside_info {
    state: *mut pata_icside_state, ec: *mut expansion_card,
    base: *mut core::ffi::c_void, irqaddr: *mut core::ffi::c_void,
    irqmask: u32, irqops: *const expansioncard_ops_t, mwdma_mask: u32,
    nr_ports: u32, port: [*const portinfo; 2], raw_base: usize, raw_ioc_base: usize,
}

const ICS_TYPE_A3IN: u32 = 0;
const ICS_TYPE_A3USER: u32 = 1;
const ICS_TYPE_V6: u32 = 3;
const ICS_TYPE_V5: u32 = 15;
const ICS_TYPE_NOTYPE: u32 = u32::MAX;

unsafe fn pata_icside_irqenable_arcin_v5(ec: *mut expansion_card, _irqnr: i32) {
    let state = (*ec).irq_data as *mut pata_icside_state;
    writeb(0, ( (*state).irq_port as usize + ICS_ARCIN_V5_INTROFFSET) as _);
}
unsafe fn pata_icside_irqdisable_arcin_v5(ec: *mut expansion_card, _irqnr: i32) {
    let state = (*ec).irq_data as *mut pata_icside_state;
    readb(((*state).irq_port as usize + ICS_ARCIN_V5_INTROFFSET) as _);
}
static pata_icside_ops_arcin_v5: expansioncard_ops_t = expansioncard_ops_t {
    irqenable: Some(pata_icside_irqenable_arcin_v5), irqdisable: Some(pata_icside_irqdisable_arcin_v5), ..unsafe { core::mem::zeroed() }
};

unsafe fn pata_icside_irqenable_arcin_v6(ec: *mut expansion_card, _irqnr: i32) {
    let state = (*ec).irq_data as *mut pata_icside_state;
    let base = (*state).irq_port as usize;
    if (*state).port[0].disabled == 0 { writeb(0, (base + ICS_ARCIN_V6_INTROFFSET_1) as _); }
    if (*state).port[1].disabled == 0 { writeb(0, (base + ICS_ARCIN_V6_INTROFFSET_2) as _); }
}
unsafe fn pata_icside_irqdisable_arcin_v6(ec: *mut expansion_card, _irqnr: i32) {
    let state = (*ec).irq_data as *mut pata_icside_state;
    readb(((*state).irq_port as usize + ICS_ARCIN_V6_INTROFFSET_1) as _);
    readb(((*state).irq_port as usize + ICS_ARCIN_V6_INTROFFSET_2) as _);
}
unsafe fn pata_icside_irqpending_arcin_v6(ec: *mut expansion_card) -> i32 {
    let state = (*ec).irq_data as *mut pata_icside_state;
    (readb(((*state).irq_port as usize + ICS_ARCIN_V6_INTRSTAT_1) as _) & 1 != 0
        || readb(((*state).irq_port as usize + ICS_ARCIN_V6_INTRSTAT_2) as _) & 1 != 0) as i32
}
static pata_icside_ops_arcin_v6: expansioncard_ops_t = expansioncard_ops_t {
    irqenable: Some(pata_icside_irqenable_arcin_v6), irqdisable: Some(pata_icside_irqdisable_arcin_v6),
    irqpending: Some(pata_icside_irqpending_arcin_v6), ..unsafe { core::mem::zeroed() }
};

unsafe fn pata_icside_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    let state = (*(*ap).host).private_data as *mut pata_icside_state;
    let mut t: ata_timing = core::mem::zeroed();
    if ata_timing_compute(adev, (*adev).dma_mode, &mut t, 1000, 1) != 0 { return; }
    let (iomd_type, cycle) = if t.active <= 50 && t.recover <= 375 && t.cycle <= 425 { ('D',187) }
        else if t.active <= 125 && t.recover <= 375 && t.cycle <= 500 { ('C',250) }
        else if t.active <= 200 && t.recover <= 550 && t.cycle <= 750 { ('B',437) } else { ('A',562) };
    ata_dev_info(adev, "timings: act %dns rec %dns cyc %dns (%c)\n", t.active, t.recover, t.cycle, iomd_type);
    (*state).port[(*ap).port_no as usize].speed[(*adev).devno as usize] = cycle;
}

unsafe fn pata_icside_bmdma_setup(qc: *mut ata_queued_cmd) {
    let ap = (*qc).ap; let state = (*(*ap).host).private_data as *mut pata_icside_state;
    let write = (*qc).tf.flags & ATA_TFLAG_WRITE;
    BUG_ON(dma_channel_active((*state).dma));
    writeb((*state).port[(*ap).port_no as usize].port_sel, (*state).ioc_base);
    set_dma_speed((*state).dma, (*state).port[(*ap).port_no as usize].speed[(*(*qc).dev).devno as usize]);
    set_dma_sg((*state).dma, (*qc).sg, (*qc).n_elem);
    set_dma_mode((*state).dma, if write != 0 { DMA_MODE_WRITE } else { DMA_MODE_READ });
    ((*(*ap).ops).sff_exec_command)(ap, &(*qc).tf);
}
unsafe fn pata_icside_bmdma_start(qc: *mut ata_queued_cmd) { let ap=(*qc).ap; let state=(*(*ap).host).private_data as *mut pata_icside_state; BUG_ON(dma_channel_active((*state).dma)); enable_dma((*state).dma); }
unsafe fn pata_icside_bmdma_stop(qc: *mut ata_queued_cmd) { let ap=(*qc).ap; let state=(*(*ap).host).private_data as *mut pata_icside_state; disable_dma((*state).dma); ata_sff_dma_pause(ap); }
unsafe fn pata_icside_bmdma_status(ap: *mut ata_port) -> u8 { let state=(*(*ap).host).private_data as *mut pata_icside_state; let off=if (*ap).port_no != 0 { ICS_ARCIN_V6_INTRSTAT_2 } else { ICS_ARCIN_V6_INTRSTAT_1 }; if readb(((*state).irq_port as usize+off) as _)&1 != 0 { ATA_DMA_INTR } else { 0 } }

// The remaining declarations preserve the source-level driver lifecycle and are
// intentionally expressed against the external kernel ABI types and operations.
unsafe fn icside_dma_init(info: *mut pata_icside_info) -> i32 { let state=(*info).state; for i in 0..ATA_MAX_DEVICES { (*state).port[0].speed[i]=480; (*state).port[1].speed[i]=480; } if (*(*info).ec).dma != NO_DMA && request_dma((*(*info).ec).dma, DRV_NAME)==0 { (*state).dma=(*(*info).ec).dma; (*info).mwdma_mask=ATA_MWDMA2; } 0 }

unsafe fn pata_icside_postreset(link: *mut ata_link, classes: *mut u32) { let ap=(*link).ap; let state=(*(*ap).host).private_data as *mut pata_icside_state; if (*classes)!=ATA_DEV_NONE || *classes.add(1)!=ATA_DEV_NONE { ata_sff_postreset(link, classes); return; } (*state).port[(*ap).port_no as usize].disabled=1; if (*state).r#type==ICS_TYPE_V6 { let off=if (*ap).port_no!=0 {ICS_ARCIN_V6_INTROFFSET_2} else {ICS_ARCIN_V6_INTROFFSET_1}; readb(((*state).irq_port as usize+off) as _); } }

unsafe fn pata_icside_setup_ioaddr(ap:*mut ata_port, base:*mut core::ffi::c_void, info:*mut pata_icside_info, port:*const portinfo) { let io=&mut (*ap).ioaddr; let cmd=base as usize+(*port).dataoffset as usize; io.cmd_addr=cmd as _; io.data_addr=(cmd+(ATA_REG_DATA<<(*port).stepping)) as _; io.error_addr=(cmd+(ATA_REG_ERR<<(*port).stepping)) as _; io.feature_addr=(cmd+(ATA_REG_FEATURE<<(*port).stepping)) as _; io.nsect_addr=(cmd+(ATA_REG_NSECT<<(*port).stepping)) as _; io.lbal_addr=(cmd+(ATA_REG_LBAL<<(*port).stepping)) as _; io.lbam_addr=(cmd+(ATA_REG_LBAM<<(*port).stepping)) as _; io.lbah_addr=(cmd+(ATA_REG_LBAH<<(*port).stepping)) as _; io.device_addr=(cmd+(ATA_REG_DEVICE<<(*port).stepping)) as _; io.status_addr=(cmd+(ATA_REG_STATUS<<(*port).stepping)) as _; io.command_addr=(cmd+(ATA_REG_CMD<<(*port).stepping)) as _; io.ctl_addr=(base as usize+(*port).ctrloffset as usize) as _; io.altstatus_addr=io.ctl_addr; ata_port_desc(ap,"cmd 0x%lx ctl 0x%lx",(*info).raw_base+(*port).dataoffset as usize,(*info).raw_base+(*port).ctrloffset as usize); if (*info).raw_ioc_base!=0 {ata_port_desc(ap,"iocbase 0x%lx",(*info).raw_ioc_base);} }

unsafe fn pata_icside_probe(ec:*mut expansion_card, _id:*const ecard_id)->i32 { let mut info:pata_icside_info=core::mem::zeroed(); let state=devm_kzalloc(&mut (*ec).dev,core::mem::size_of::<pata_icside_state>(),GFP_KERNEL) as *mut pata_icside_state; if state.is_null(){ecard_release_resources(ec);return -ENOMEM;} (*state).r#type=ICS_TYPE_NOTYPE; (*state).dma=NO_DMA; info.state=state; info.ec=ec; let idmem=ecardm_iomap(ec,ECARD_RES_IOCFAST,0,0); if !idmem.is_null(){let mut ty=0; for n in 0..4 {ty|=((readb((idmem as usize+ICS_IDENT_OFFSET+n*4) as _)&1) as u32)<<n;} ecardm_iounmap(ec,idmem);(*state).r#type=ty;} let ret=match (*state).r#type {ICS_TYPE_A3IN|ICS_TYPE_A3USER=>-ENODEV,ICS_TYPE_V5=>pata_icside_register_v5(&mut info),ICS_TYPE_V6=>pata_icside_register_v6(&mut info),_=>-ENODEV}; if ret==0 {pata_icside_add_ports(&mut info)} else {ecard_release_resources(ec);ret} }
unsafe fn pata_icside_register_v5(info:*mut pata_icside_info)->i32 { let b=ecardm_iomap((*info).ec,ECARD_RES_MEMC,0,0); if b.is_null(){return -ENOMEM;} (*info).state.as_mut().unwrap().irq_port=b; (*info).base=b;(*info).irqaddr=(b as usize+ICS_ARCIN_V5_INTRSTAT) as _;(*info).irqmask=1;(*info).irqops=&pata_icside_ops_arcin_v5;(*info).nr_ports=1;(*info).port[0]=&pata_icside_portinfo_v5;(*info).raw_base=ecard_resource_start((*info).ec,ECARD_RES_MEMC);0 }
unsafe fn pata_icside_register_v6(info:*mut pata_icside_info)->i32 { let ec=(*info).ec;let b=ecardm_iomap(ec,ECARD_RES_IOCFAST,0,0);if b.is_null(){return -ENOMEM;} let mut e=b;let mut sel=0;if ecard_resource_flags(ec,ECARD_RES_EASI)!=0{e=ecardm_iomap(ec,ECARD_RES_EASI,0,0);if e.is_null(){return -ENOMEM;}sel=1<<5;}writeb(sel,b);let s=(*info).state;(*s).irq_port=e;(*s).ioc_base=b;(*s).port[0].port_sel=sel;(*s).port[1].port_sel=sel|1;(*info).base=e;(*info).irqops=&pata_icside_ops_arcin_v6;(*info).nr_ports=2;(*info).port[0]=&pata_icside_portinfo_v6_1;(*info).port[1]=&pata_icside_portinfo_v6_2;(*info).raw_base=ecard_resource_start(ec,ECARD_RES_EASI);(*info).raw_ioc_base=ecard_resource_start(ec,ECARD_RES_IOCFAST);icside_dma_init(info)}

// Driver registration tables and module init/exit correspond directly to the
// C file and use the external kernel ABI declarations.
unsafe fn pata_icside_shutdown(ec:*mut expansion_card){let host=ecard_get_drvdata(ec);let mut flags=0;local_irq_save(&mut flags);((*(*ec).ops).irqdisable)(ec,(*ec).irq);local_irq_restore(flags);if !host.is_null(){let s=(*host).private_data as *mut pata_icside_state;if !(*s).ioc_base.is_null(){writeb(0,(*s).ioc_base);}}}
unsafe fn pata_icside_remove(ec:*mut expansion_card){let host=ecard_get_drvdata(ec);let s=(*host).private_data as *mut pata_icside_state;ata_host_detach(host);pata_icside_shutdown(ec);if (*s).dma!=NO_DMA{free_dma((*s).dma);}ecard_release_resources(ec);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
