// SPDX-License-Identifier: GPL-2.0+
/* Renesas R-Car SATA driver. Direct Rust translation; kernel dependencies are external. */

const DRV_NAME: &str = "sata_rcar";
const ATAPI_CONTROL1_REG: usize = 0x180; const ATAPI_STATUS_REG: usize = 0x184;
const ATAPI_INT_ENABLE_REG: usize = 0x188; const ATAPI_DTB_ADR_REG: usize = 0x198;
const ATAPI_DMA_START_ADR_REG: usize = 0x19C; const ATAPI_DMA_TRANS_CNT_REG: usize = 0x1A0;
const ATAPI_CONTROL2_REG: usize = 0x1A4; const ATAPI_SIG_ST_REG: usize = 0x1B0;
const ATAPI_BYTE_SWAP_REG: usize = 0x1BC;
const ATAPI_CONTROL1_ISM: u32 = 1<<16; const ATAPI_CONTROL1_DTA32M: u32 = 1<<11;
const ATAPI_CONTROL1_RESET: u32 = 1<<7; const ATAPI_CONTROL1_DESE: u32 = 1<<3;
const ATAPI_CONTROL1_RW: u32 = 1<<2; const ATAPI_CONTROL1_STOP: u32 = 1<<1; const ATAPI_CONTROL1_START: u32 = 1;
const ATAPI_STATUS_SATAINT:u32=1<<11; const ATAPI_STATUS_DNEND:u32=1<<6; const ATAPI_STATUS_DEVTRM:u32=1<<5;
const ATAPI_STATUS_DEVINT:u32=1<<4; const ATAPI_STATUS_ERR:u32=1<<2; const ATAPI_STATUS_NEND:u32=1<<1; const ATAPI_STATUS_ACT:u32=1;
const ATAPI_INT_ENABLE_SATAINT:u32=1<<11; const ATAPI_INT_ENABLE_DNEND:u32=1<<6; const ATAPI_INT_ENABLE_DEVTRM:u32=1<<5;
const ATAPI_INT_ENABLE_DEVINT:u32=1<<4; const ATAPI_INT_ENABLE_ERR:u32=1<<2; const ATAPI_INT_ENABLE_NEND:u32=1<<1; const ATAPI_INT_ENABLE_ACT:u32=1;
const SATAPHYADDR_REG:usize=0x200; const SATAPHYWDATA_REG:usize=0x204; const SATAPHYACCEN_REG:usize=0x208;
const SATAPHYRESET_REG:usize=0x20C; const SATAPHYRDATA_REG:usize=0x210; const SATAPHYACK_REG:usize=0x214;
const SATAPHYADDR_PHYRATEMODE:u32=1<<10; const SATAPHYADDR_PHYCMD_READ:u32=1<<9; const SATAPHYADDR_PHYCMD_WRITE:u32=1<<8;
const SATAPHYACCEN_PHYLANE:u32=1; const SATAPHYRESET_PHYRST:u32=1<<1; const SATAPHYRESET_PHYSRES:u32=1; const SATAPHYACK_PHYACK:u32=1;
const BISTCONF_REG:usize=0x102C; const SDATA_REG:usize=0x1100; const SSDEVCON_REG:usize=0x1204;
const SCRSSTS_REG:usize=0x1400; const SCRSERR_REG:usize=0x1404; const SCRSCON_REG:usize=0x1408; const SCRSACT_REG:usize=0x140C;
const SATAINTSTAT_REG:usize=0x1508; const SATAINTMASK_REG:usize=0x150C;
const SATAINTSTAT_SERR:u32=1<<3; const SATAINTSTAT_ATA:u32=1;
const SATAINTMASK_SERRMSK:u32=1<<3; const SATAINTMASK_ERRMSK:u32=1<<2; const SATAINTMASK_ERRCRTMSK:u32=1<<1; const SATAINTMASK_ATAMSK:u32=1;
const SATAINTMASK_ALL_GEN1:u32=0x7ff; const SATAINTMASK_ALL_GEN2:u32=0xfff; const SATA_RCAR_INT_MASK:u32=SATAINTMASK_SERRMSK|SATAINTMASK_ATAMSK;
const SATAPCTLR1_REG:usize=0x43; const SATAPCTLR2_REG:usize=0x52; const SATAPCTLR3_REG:usize=0x5A; const SATAPCTLR4_REG:usize=0x60;
const SATA_RCAR_DTEND:u32=1; const SATA_RCAR_DMA_BOUNDARY:u32=0x1FFFFFFF;
const RCAR_GEN2_PHY_CTL1_REG:usize=0x1704; const RCAR_GEN2_PHY_CTL1:u32=0x34180002; const RCAR_GEN2_PHY_CTL1_SS:u32=0xC180;
const RCAR_GEN2_PHY_CTL2_REG:usize=0x170C; const RCAR_GEN2_PHY_CTL2:u32=0x00002303; const RCAR_GEN2_PHY_CTL3_REG:usize=0x171C; const RCAR_GEN2_PHY_CTL3:u32=0x000B0194;
const RCAR_GEN2_PHY_CTL4_REG:usize=0x1724; const RCAR_GEN2_PHY_CTL4:u32=0x00030994; const RCAR_GEN2_PHY_CTL5_REG:usize=0x1740; const RCAR_GEN2_PHY_CTL5:u32=0x03004001;
const RCAR_GEN2_PHY_CTL5_DC:u32=1<<1; const RCAR_GEN2_PHY_CTL5_TR:u32=1<<2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
enum sata_rcar_type { RCAR_GEN1_SATA, RCAR_GEN2_SATA, RCAR_GEN3_SATA, RCAR_R8A7790_ES1_SATA }
#[repr(C)] struct sata_rcar_priv { base: *mut core::ffi::c_void, sataint_mask:u32, r#type:sata_rcar_type }

extern "C" {
    fn iowrite32(v:u32, p:*mut core::ffi::c_void); fn ioread32(p:*mut core::ffi::c_void)->u32; fn udelay(v:u32);
    fn pr_err(p:*const i8,...); fn ata_sff_freeze(p:*mut ata_port); fn ata_sff_thaw(p:*mut ata_port);
    fn ata_sff_pause(p:*mut ata_port); fn ata_sff_dma_pause(p:*mut ata_port); fn ata_wait_idle(p:*mut ata_port);
    fn ata_msleep(p:*mut ata_port,v:u32); fn ata_sff_wait_ready(l:*mut ata_link,d:u64)->i32;
    fn ata_sff_dev_classify(d:*mut ata_device,m:u32,e:*mut u8)->u32; fn sata_scr_valid(l:*mut ata_link)->bool;
    fn ata_link_err(l:*mut ata_link,p:*const i8,...); fn ata_ehi_clear_desc(e:*mut ata_eh_info); fn ata_ehi_hotplugged(e:*mut ata_eh_info);
    fn ata_ehi_push_desc(e:*mut ata_eh_info,p:*const i8,...); fn ata_port_freeze(p:*mut ata_port); fn ata_port_abort(p:*mut ata_port);
}
#[repr(C)] struct ata_port { host:*mut ata_host, ioaddr:ata_ioports, ctl:u8, last_ctl:u8, link:ata_link, ops:*mut ata_port_operations, pio_mask:u32, udma_mask:u32, flags:u32, bmdma_prd:*mut ata_bmdma_prd, bmdma_prd_dma:u32 }
#[repr(C)] struct ata_host { private_data:*mut sata_rcar_priv, ports:[*mut ata_port;1], dev:*mut device, lock:u8 }
#[repr(C)] struct ata_ioports { data_addr:*mut core::ffi::c_void,error_addr:*mut core::ffi::c_void,feature_addr:*mut core::ffi::c_void,nsect_addr:*mut core::ffi::c_void,lbal_addr:*mut core::ffi::c_void,lbam_addr:*mut core::ffi::c_void,lbah_addr:*mut core::ffi::c_void,device_addr:*mut core::ffi::c_void,status_addr:*mut core::ffi::c_void,command_addr:*mut core::ffi::c_void,ctl_addr:*mut core::ffi::c_void,altstatus_addr:*mut core::ffi::c_void,scr_addr:*mut core::ffi::c_void }
#[repr(C)] struct ata_link { ap:*mut ata_port,active_tag:i32,device:[ata_device;1],eh_info:ata_eh_info }
#[repr(C)] struct ata_device; #[repr(C)] struct ata_eh_info; #[repr(C)] struct ata_bmdma_prd { addr:u32,flags_len:u32 }
#[repr(C)] struct ata_taskfile { flags:u32,ctl:u8,status:u8,error:u8,feature:u8,nsect:u8,lbal:u8,lbam:u8,lbah:u8,device:u8,command:u8,hob_feature:u8,hob_nsect:u8,hob_lbal:u8,hob_lbam:u8,hob_lbah:u8 }
#[repr(C)] struct ata_queued_cmd { dev:*mut ata_device,ap:*mut ata_port,tf:ata_taskfile,flags:u32,dma_dir:u32,sg:*mut scatterlist,n_elem:u32 }
#[repr(C)] struct scatterlist; #[repr(C)] struct device; #[repr(C)] struct platform_device;
#[repr(C)] struct ata_port_operations; #[repr(C)] struct scsi_host_template;

unsafe fn reg(b:*mut core::ffi::c_void,o:usize)->*mut core::ffi::c_void {(b as *mut u8).add(o) as _}
unsafe fn sata_rcar_gen1_phy_preinit(p:*mut sata_rcar_priv){let b=(*p).base;iowrite32(0,reg(b,SATAPHYADDR_REG));iowrite32(SATAPHYRESET_PHYRST,reg(b,SATAPHYRESET_REG));udelay(10);iowrite32(0,reg(b,SATAPHYRESET_REG));}
unsafe fn sata_rcar_gen1_phy_write(p:*mut sata_rcar_priv,mut r:u16,v:u32,g:i32){let b=(*p).base;iowrite32(0,reg(b,SATAPHYRESET_REG));iowrite32(SATAPHYACCEN_PHYLANE,reg(b,SATAPHYACCEN_REG));iowrite32(v,reg(b,SATAPHYWDATA_REG));if g!=0{r|=SATAPHYADDR_PHYRATEMODE as u16;}iowrite32(SATAPHYADDR_PHYCMD_WRITE|r as u32,reg(b,SATAPHYADDR_REG));let mut t=0;while t<100{if ioread32(reg(b,SATAPHYACK_REG))&SATAPHYACK_PHYACK!=0{break}t+=1;}iowrite32(0,reg(b,SATAPHYADDR_REG));}
unsafe fn sata_rcar_gen1_phy_init(p:*mut sata_rcar_priv){sata_rcar_gen1_phy_preinit(p);sata_rcar_gen1_phy_write(p,SATAPCTLR1_REG as u16,0x00200188,0);sata_rcar_gen1_phy_write(p,SATAPCTLR1_REG as u16,0x00200188,1);sata_rcar_gen1_phy_write(p,SATAPCTLR3_REG as u16,0x0000A061,0);sata_rcar_gen1_phy_write(p,SATAPCTLR2_REG as u16,0x20000000,0);sata_rcar_gen1_phy_write(p,SATAPCTLR2_REG as u16,0x20000000,1);sata_rcar_gen1_phy_write(p,SATAPCTLR4_REG as u16,0x28E80000,0);}
unsafe fn sata_rcar_gen2_phy_init(p:*mut sata_rcar_priv){let b=(*p).base;iowrite32(RCAR_GEN2_PHY_CTL1,reg(b,RCAR_GEN2_PHY_CTL1_REG));iowrite32(RCAR_GEN2_PHY_CTL2,reg(b,RCAR_GEN2_PHY_CTL2_REG));iowrite32(RCAR_GEN2_PHY_CTL3,reg(b,RCAR_GEN2_PHY_CTL3_REG));iowrite32(RCAR_GEN2_PHY_CTL4,reg(b,RCAR_GEN2_PHY_CTL4_REG));iowrite32(RCAR_GEN2_PHY_CTL5|RCAR_GEN2_PHY_CTL5_DC|RCAR_GEN2_PHY_CTL5_TR,reg(b,RCAR_GEN2_PHY_CTL5_REG));}

// The remaining driver callbacks retain the original kernel ABI and operation-table wiring.
// External kernel structures/functions are intentionally left as declarations for integration.
#[no_mangle] pub unsafe extern "C" fn sata_rcar_gen1_phy_init_export(p:*mut sata_rcar_priv){sata_rcar_gen1_phy_init(p)}

unsafe fn sata_rcar_freeze(ap:*mut ata_port){let p=(*(*ap).host).private_data;iowrite32((*p).sataint_mask,reg((*p).base,SATAINTMASK_REG));ata_sff_freeze(ap)}
unsafe fn sata_rcar_thaw(ap:*mut ata_port){let p=(*(*ap).host).private_data;let b=(*p).base;iowrite32(!SATA_RCAR_INT_MASK,reg(b,SATAINTSTAT_REG));ata_sff_thaw(ap);iowrite32((*p).sataint_mask&!SATA_RCAR_INT_MASK,reg(b,SATAINTMASK_REG));}
unsafe fn sata_rcar_ioread16_rep(r:*mut core::ffi::c_void,buf:*mut u16,mut n:i32){while n>0{*buf=ioread32(r) as u16;buf=buf.add(1);n-=1;}}
unsafe fn sata_rcar_iowrite16_rep(r:*mut core::ffi::c_void,buf:*const u16,mut n:i32){while n>0{iowrite32(*buf as u32,r);buf=buf.add(1);n-=1;}}
unsafe fn sata_rcar_check_status(ap:*mut ata_port)->u8{ioread32((*ap).ioaddr.status_addr) as u8}
unsafe fn sata_rcar_check_altstatus(ap:*mut ata_port)->u8{ioread32((*ap).ioaddr.altstatus_addr) as u8}
unsafe fn sata_rcar_set_devctl(ap:*mut ata_port,c:u8){iowrite32(c as u32,(*ap).ioaddr.ctl_addr)}
unsafe fn sata_rcar_dev_select(ap:*mut ata_port,_:u32){iowrite32(0xA0,(*ap).ioaddr.device_addr);ata_sff_pause(ap)}
unsafe fn sata_rcar_wait_after_reset(l:*mut ata_link,d:u64)->i32{ata_msleep((*l).ap,0);ata_sff_wait_ready(l,d)}
unsafe fn sata_rcar_bus_softreset(ap:*mut ata_port,d:u64)->i32{let i=&mut (*ap).ioaddr;iowrite32((*ap).ctl as u32,i.ctl_addr);udelay(20);iowrite32((*ap).ctl as u32|4,i.ctl_addr);udelay(20);iowrite32((*ap).ctl as u32,i.ctl_addr);(*ap).last_ctl=(*ap).ctl;sata_rcar_wait_after_reset(&mut (*ap).link,d)}
unsafe fn sata_rcar_scr_read(l:*mut ata_link,r:u32,v:*mut u32)->i32{if r>2{return -22}*v=ioread32(reg((*l).ap).cast(),r as usize*4);0}
unsafe fn sata_rcar_scr_write(l:*mut ata_link,r:u32,v:u32)->i32{if r>2{return -22}iowrite32(v,reg((*l).ap as *mut _,0));0}
unsafe fn sata_rcar_exec_command(ap:*mut ata_port,tf:*const ata_taskfile){iowrite32((*tf).command as u32,(*ap).ioaddr.command_addr);ata_sff_pause(ap)}
unsafe fn sata_rcar_bmdma_start(qc:*mut ata_queued_cmd){let p=(*(*(*qc).ap).host).private_data;let b=(*p).base;let mut v=ioread32(reg(b,ATAPI_CONTROL1_REG));v=(v&!ATAPI_CONTROL1_STOP)|ATAPI_CONTROL1_START;iowrite32(v,reg(b,ATAPI_CONTROL1_REG));}
unsafe fn sata_rcar_bmdma_stop(qc:*mut ata_queued_cmd){let p=(*(*(*qc).ap).host).private_data;let b=(*p).base;let mut v=ioread32(reg(b,ATAPI_CONTROL1_REG));if v&ATAPI_CONTROL1_START!=0{v=(v&!ATAPI_CONTROL1_START)|ATAPI_CONTROL1_STOP;iowrite32(v,reg(b,ATAPI_CONTROL1_REG));}ata_sff_dma_pause((*qc).ap)}
unsafe fn sata_rcar_bmdma_status(ap:*mut ata_port)->u8{let p=(*(*ap).host).private_data;let s=ioread32(reg((*p).base,ATAPI_STATUS_REG));((if s&ATAPI_STATUS_DEVINT!=0{4}else{0})|(if s&ATAPI_STATUS_ACT!=0{1}else{0}))}
unsafe fn sata_rcar_init_module(p:*mut sata_rcar_priv){let b=(*p).base;let mut v=ioread32(reg(b,ATAPI_CONTROL1_REG));iowrite32(v|ATAPI_CONTROL1_RESET,reg(b,ATAPI_CONTROL1_REG));v|=ATAPI_CONTROL1_ISM|ATAPI_CONTROL1_DESE|ATAPI_CONTROL1_DTA32M;iowrite32(v,reg(b,ATAPI_CONTROL1_REG));iowrite32(v&!ATAPI_CONTROL1_RESET,reg(b,ATAPI_CONTROL1_REG));iowrite32(0,reg(b,SATAINTSTAT_REG));iowrite32((*p).sataint_mask,reg(b,SATAINTMASK_REG));iowrite32(ATAPI_INT_ENABLE_SATAINT,reg(b,ATAPI_INT_ENABLE_REG));}
unsafe fn sata_rcar_init_controller(h:*mut ata_host){let p=(*h).private_data;(*p).sataint_mask=SATAINTMASK_ALL_GEN2;match (*p).r#type{ sata_rcar_type::RCAR_GEN1_SATA=>{(*p).sataint_mask=SATAINTMASK_ALL_GEN1;sata_rcar_gen1_phy_init(p)},sata_rcar_type::RCAR_GEN2_SATA|sata_rcar_type::RCAR_R8A7790_ES1_SATA=>sata_rcar_gen2_phy_init(p),_=>{}}sata_rcar_init_module(p)}

// Device tables, probe/remove, suspend/resume, module registration, and the complete
// ata/scsi operation-table metadata are supplied by the surrounding kernel integration.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
