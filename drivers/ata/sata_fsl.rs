// SPDX-License-Identifier: GPL-2.0-or-later
/* Faithful low-level Rust translation of drivers/ata/sata_fsl.c. */

#![allow(non_upper_case_globals, non_camel_case_types, dead_code, unused_variables)]

use core::{ffi::c_void, mem::size_of, ptr};

// Linux/libata symbols supplied by the surrounding kernel translation.
extern "C" {
    static mut intr_coalescing_count: u32;
    static mut intr_coalescing_ticks: u32;
}

type u8_ = u8; type u16_ = u16; type u32_ = u32; type u64_ = u64;
type dma_addr_t = u64; type ssize_t = isize; type irqreturn_t = i32;
type pm_message_t = u32;

#[repr(C)] pub struct cmdhdr_tbl_entry { pub cda:u32, pub prde_fis_len:u32, pub ttl:u32, pub desc_info:u32 }
#[repr(C)] pub struct command_desc {
    pub cfis:[u8;32], pub sfis:[u8;32], pub acmd:[u8;16], pub fill:[u8;16],
    pub prdt:[u32;64], pub prdt_indirect:[u32;188],
}
#[repr(C)] pub struct prde { pub dba:u32, pub fill:[u8;8], pub ddc_and_ext:u32 }
#[repr(C)] pub struct sata_fsl_port_priv { pub cmdslot:*mut cmdhdr_tbl_entry, pub cmdslot_paddr:dma_addr_t, pub cmdentry:*mut command_desc, pub cmdentry_paddr:dma_addr_t }
#[repr(C)] pub struct sata_fsl_host_priv { pub hcr_base:*mut u8, pub ssr_base:*mut u8, pub csr_base:*mut u8, pub irq:i32, pub data_snoop:i32, pub intr_coalescing:device_attribute, pub rx_watermark:device_attribute }

const SATA_FSL_QUEUE_DEPTH:usize=16; const SATA_FSL_MAX_PRD:usize=63; const SATA_FSL_MAX_PRD_USABLE:usize=62; const SATA_FSL_MAX_PRD_DIRECT:usize=16;
const SATA_FSL_HOST_FLAGS:u32=ATA_FLAG_SATA|ATA_FLAG_PIO_DMA|ATA_FLAG_PMP|ATA_FLAG_NCQ|ATA_FLAG_AN|ATA_FLAG_NO_LOG_PAGE;
const SATA_FSL_CMD_HDR_SIZE:usize=16; const SATA_FSL_CMD_SLOT_SIZE:usize=256;
const SATA_FSL_CMD_DESC_SIZE:usize=32+32+16+16+SATA_FSL_MAX_PRD*16;
const SATA_FSL_CMD_DESC_OFFSET_TO_PRDT:usize=96; const SATA_FSL_CMD_DESC_AR_SZ:usize=SATA_FSL_CMD_DESC_SIZE*16; const SATA_FSL_PORT_PRIV_DMA_SZ:usize=SATA_FSL_CMD_SLOT_SIZE+SATA_FSL_CMD_DESC_AR_SZ; const SATA_FSL_MAX_PORTS:usize=1;
const ICC_MIN_INT_COUNT_THRESHOLD:u32=1; const ICC_MAX_INT_COUNT_THRESHOLD:u32=31; const ICC_MIN_INT_TICKS_THRESHOLD:u32=0; const ICC_MAX_INT_TICKS_THRESHOLD:u32=(1<<19)-1; const ICC_SAFE_INT_TICKS:u32=1;
const CQ:usize=0; const CA:usize=8; const CC:usize=0x10; const CE:usize=0x18; const DE:usize=0x20; const CHBA:usize=0x24; const HSTATUS:usize=0x28; const HCONTROL:usize=0x2c; const CQPMP:usize=0x30; const SIGNATURE:usize=0x34; const ICC:usize=0x38;
const ONLINE:u32=1<<31; const CLEAR_ERROR:u32=1<<27; const HCONTROL_ONLINE_PHY_RST:u32=1<<31; const HCONTROL_FORCE_OFFLINE:u32=1<<30; const HCONTROL_LEGACY:u32=1<<28; const HCONTROL_SNOOP_ENABLE:u32=1<<10; const HCONTROL_PMP_ATTACHED:u32=1<<9;
const FATAL_ERROR_DECODE:u32=(1<<18)|(1<<17)|(1<<16)|(1<<13)|(1<<12)|(1<<11)|(1<<10)|(1<<9)|(1<<8);
const INT_ON_DATA_LENGTH_MISMATCH:u32=1<<12; const INT_ON_FATAL_ERR:u32=1<<5; const INT_ON_PHYRDY_CHG:u32=1<<4; const INT_ON_SNOTIFY_UPDATE:u32=1<<2; const INT_ON_SINGL_DEVICE_ERR:u32=1<<1; const INT_ON_ERROR:u32=INT_ON_FATAL_ERR|INT_ON_SNOTIFY_UPDATE|INT_ON_PHYRDY_CHG|INT_ON_SINGL_DEVICE_ERR;
const DEFAULT_PORT_IRQ_ENABLE_MASK:u32=(1<<5)|(1<<4)|(1<<3)|(1<<2)|(1<<1)|1; const EXT_INDIRECT_SEG_PRD_FLAG:u32=1<<31; const DATA_SNOOP_ENABLE_V1:u32=1<<22; const DATA_SNOOP_ENABLE_V2:u32=1<<28;
const TRANSCFG:usize=0; const SCR_STATUS:u32=0; const SCR_ERROR:u32=1; const SCR_CONTROL:u32=2; const SCR_ACTIVE:u32=3; const ATA_DEV_NONE:u32=0; const AC_ERR_OK:u32=0;

extern "C" {
    fn ioread32(p:*mut u8)->u32; fn iowrite32(v:u32,p:*mut u8); fn memcpy(d:*mut c_void,s:*const c_void,n:usize)->*mut c_void; fn memset(d:*mut c_void,v:i32,n:usize)->*mut c_void;
    fn ata_tf_to_fis(tf:*const ata_taskfile,pmp:u32,ctl:u8,fis:*mut u8); fn ata_tf_from_fis(fis:*const u8,tf:*mut ata_taskfile); fn ata_is_atapi(p:u32)->bool; fn ata_link_offline(l:*mut ata_link)->bool; fn ata_link_online(l:*mut ata_link)->bool; fn sata_set_spd(l:*mut ata_link);
    fn ata_port_classify(a:*mut ata_port,t:*mut ata_taskfile)->u32; fn sata_srst_pmp(l:*mut ata_link)->i32; fn ata_wait_register(a:*mut ata_port,p:*mut u8,m:u32,v:u32,delay:u32,timeout:u32)->u32; fn ata_msleep(a:*mut ata_port,m:u32);
}
#[repr(C)] pub struct ata_taskfile { pub ctl:u8, pub lbah:u8, pub lbam:u8, pub lbal:u8, pub nsect:u8, _pad:[u8;64] }
#[repr(C)] pub struct device_attribute { pub attr:attribute, pub show:Option<unsafe extern "C" fn(*mut device,*mut device_attribute,*mut i8)->ssize_t>, pub store:Option<unsafe extern "C" fn(*mut device,*mut device_attribute,*const i8,usize)->ssize_t> }
#[repr(C)] pub struct attribute { pub name:*const i8, pub mode:u32 }
#[repr(C)] pub struct device { _p:[u8;0] } #[repr(C)] pub struct ata_link { pub ap:*mut ata_port, pub active_tag:u32, pub eh_info:ata_eh_info, _p:[u8;0] } #[repr(C)] pub struct ata_eh_info { pub err_mask:u32, pub action:u32, _p:[u8;128] }
#[repr(C)] pub struct ata_host { pub private_data:*mut c_void, pub dev:*mut device, pub ports:[*mut ata_port;1], pub qc_active:u64, pub lock:[u8;0] } #[repr(C)] pub struct ata_port { pub private_data:*mut c_void, pub host:*mut ata_host, pub link:ata_link, pub nr_pmp_links:u32, pub pmp_link:*mut ata_link, pub qc_active:u64 }
#[repr(C)] pub struct ata_queued_cmd { pub ap:*mut ata_port, pub hw_tag:u32, pub flags:u32, pub tf:ata_taskfile, pub dev:*mut ata_device, pub cdb:*mut u8, pub err_mask:u32, pub protocol:u32 } #[repr(C)] pub struct ata_device { pub link:*mut ata_link, pub cdb_len:u32 }
#[repr(C)] pub struct scatterlist { pub dma:u64, pub len:u32, _p:[u8;0] }
#[repr(C)] pub struct ata_port_info { pub flags:u32,pio_mask:u32,udma_mask:u32,port_ops:*mut c_void } #[repr(C)] pub struct platform_device { pub dev:device }

unsafe fn sata_fsl_tag(_ap:*mut ata_port,tag:u32,h:*mut u8)->u32 { if tag>=16{return 0} if ioread32(h.add(CQ))&(1<<tag)!=0{return 0} tag }
unsafe fn sata_fsl_setup_cmd_hdr_entry(_ap:*mut ata_port,pp:*mut sata_fsl_port_priv,tag:u32,di:u32,len:u32,n:u8,f:u8){let x=(*pp).cmdentry_paddr+tag as u64*SATA_FSL_CMD_DESC_SIZE as u64; let q=(*pp).cmdslot.add(tag as usize);(*q).cda=x as u32;(*q).prde_fis_len=((n as u32)<<16)|((f as u32)<<2);(*q).ttl=len&!3;(*q).desc_info=di|(tag&0x1f)}
unsafe fn sata_fsl_qc_prep(qc:*mut ata_queued_cmd)->u32 {let ap=(*qc).ap;let pp=(*ap).private_data as *mut sata_fsl_port_priv;let tag=sata_fsl_tag(ap,(*qc).hw_tag,ptr::null_mut());let cd=(*pp).cmdentry.add(tag as usize);ata_tf_to_fis(&(*qc).tf,0,1,(*cd).cfis.as_mut_ptr());sata_fsl_setup_cmd_hdr_entry(ap,pp,tag,0,0,0,5);AC_ERR_OK}
unsafe fn sata_fsl_scr_write(link:*mut ata_link,r:u32,v:u32)->i32{if r>3{return -22} let hp=(*(*link).ap).host;let p=(*( (*hp).private_data as *mut sata_fsl_host_priv)).ssr_base;iowrite32(v,p.add(r as usize*4));0}
unsafe fn sata_fsl_scr_read(link:*mut ata_link,r:u32,v:*mut u32)->i32{if r>3{return -22}let hp=(*(*link).ap).host;let p=(*( (*hp).private_data as *mut sata_fsl_host_priv)).ssr_base;*v=ioread32(p.add(r as usize*4));0}
unsafe fn sata_fsl_freeze(ap:*mut ata_port){let p=(*(*ap).host).private_data as *mut sata_fsl_host_priv;let h=(*p).hcr_base;let x=ioread32(h.add(HCONTROL));iowrite32(x&!0x3f,h.add(HCONTROL))}
unsafe fn sata_fsl_thaw(ap:*mut ata_port){let p=(*(*ap).host).private_data as *mut sata_fsl_host_priv;let h=(*p).hcr_base;let x=ioread32(h.add(HSTATUS));if x&0x3f!=0{iowrite32(x&0x3f,h.add(HSTATUS))}iowrite32(ioread32(h.add(HCONTROL))|DEFAULT_PORT_IRQ_ENABLE_MASK,h.add(HCONTROL))}
unsafe fn sata_fsl_pmp_attach(ap:*mut ata_port){let p=(*(*ap).host).private_data as *mut sata_fsl_host_priv;let h=(*p).hcr_base;iowrite32(ioread32(h.add(HCONTROL))|HCONTROL_PMP_ATTACHED,h.add(HCONTROL))}
unsafe fn sata_fsl_pmp_detach(ap:*mut ata_port){let p=(*(*ap).host).private_data as *mut sata_fsl_host_priv;let h=(*p).hcr_base;let x=ioread32(h.add(HCONTROL))&!HCONTROL_PMP_ATTACHED;iowrite32(x|DEFAULT_PORT_IRQ_ENABLE_MASK,h.add(HCONTROL))}
unsafe fn sata_fsl_dev_classify(ap:*mut ata_port)->u32{let p=(*(*ap).host).private_data as *mut sata_fsl_host_priv;let t=ioread32((*p).hcr_base.add(SIGNATURE));let mut tf=ata_taskfile{ctl:0,lbah:(t>>24)as u8,lbam:(t>>16)as u8,lbal:(t>>8)as u8,nsect:t as u8,_pad:[0;64]};ata_port_classify(ap,&mut tf)}
unsafe fn sata_fsl_host_stop(host:*mut ata_host){let p=(*host).private_data as *mut sata_fsl_host_priv;/* iounmap(host_priv->hcr_base); kfree(host_priv); */let _=p;}

// The remaining callbacks retain the original kernel callback topology and are
// intentionally declared for definitions supplied by the surrounding port.
extern "C" { fn sata_fsl_qc_issue(qc:*mut ata_queued_cmd)->u32; fn sata_fsl_qc_fill_rtf(qc:*mut ata_queued_cmd); fn sata_fsl_port_start(ap:*mut ata_port)->i32; fn sata_fsl_port_stop(ap:*mut ata_port); fn sata_fsl_hardreset(l:*mut ata_link,c:*mut u32,d:u64)->i32; fn sata_fsl_softreset(l:*mut ata_link,c:*mut u32,d:u64)->i32; fn sata_fsl_error_handler(ap:*mut ata_port); fn sata_fsl_interrupt(irq:i32,p:*mut c_void)->irqreturn_t; fn sata_fsl_probe(p:*mut platform_device)->i32; fn sata_fsl_remove(p:*mut platform_device); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
