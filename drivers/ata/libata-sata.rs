// SPDX-License-Identifier: GPL-2.0-or-later
/* SATA specific part of ATA helper library. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// Kernel/libata declarations are supplied by the surrounding translation unit.
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    fn sata_pmp_scr_read(link: *mut ata_link, reg: c_int, val: *mut u32) -> c_int;
    fn sata_pmp_scr_write(link: *mut ata_link, reg: c_int, val: u32) -> c_int;
    fn ata_is_host_link(link: *mut ata_link) -> bool;
    fn ata_deadline(now: c_ulong, msecs: c_uint) -> c_ulong;
    fn time_before(a: c_ulong, b: c_ulong) -> bool;
    fn time_after(a: c_ulong, b: c_ulong) -> bool;
    fn ata_msleep(ap: *mut ata_port, msecs: c_uint);
    fn sata_scr_read(link: *mut ata_link, reg: c_int, val: *mut u32) -> c_int;
    fn sata_scr_write(link: *mut ata_link, reg: c_int, val: u32) -> c_int;
    fn sata_scr_write_flush(link: *mut ata_link, reg: c_int, val: u32) -> c_int;
}

#[repr(C)] pub struct ata_link { pub ap: *mut ata_port, pub flags: u32, pub sata_spd: u32, pub sata_spd_limit: u32, pub last_lpm_change: c_ulong, pub lpm_policy: c_int, pub eh_context: ata_eh_context, pub device: *mut ata_device }
#[repr(C)] pub struct ata_port { pub flags: u32, pub ops: *mut ata_port_operations, pub link: ata_link, pub slave_link: *mut ata_link, pub qc_active: u64, pub target_lpm_policy: c_int, pub em_message_type: c_int, pub lock: *mut c_void, pub host: *mut ata_host }
#[repr(C)] pub struct ata_host { pub flags: u32 }
#[repr(C)] pub struct ata_taskfile { pub command:u8,pub feature:u8,pub lbal:u8,pub lbam:u8,pub lbah:u8,pub device:u8,pub hob_lbal:u8,pub hob_lbam:u8,pub hob_lbah:u8,pub hob_feature:u8,pub nsect:u8,pub hob_nsect:u8,pub ctl:u8,pub status:u8,pub error:u8,pub auxiliary:u32,pub flags:u32 }
#[repr(C)] pub struct ata_eh_context { pub i: ata_eh_info }
#[repr(C)] pub struct ata_eh_info { pub serror:u32, pub err_mask:u32 }
#[repr(C)] pub struct ata_device { pub link:*mut ata_link,pub flags:u32,pub quirks:u32,pub class:u32,pub id:*mut u16,pub sector_buf:*mut u8,pub cdl:*mut ata_cdl }
#[repr(C)] pub struct ata_cdl { pub ncq_sense_log_buf:*mut u8 }
#[repr(C)] pub struct ata_queued_cmd { pub flags:u32,pub err_mask:u32,pub dev:*mut ata_device,pub result_tf:ata_taskfile,pub scsicmd:*mut scsi_cmnd }
#[repr(C)] pub struct scsi_cmnd { pub result:u32,pub sense_buffer:*mut u8 }
#[repr(C)] pub struct scsi_device { pub host:*mut Scsi_Host,pub queue_depth:c_int }
#[repr(C)] pub struct Scsi_Host { pub can_queue:c_int }
#[repr(C)] pub struct queue_limits;
#[repr(C)] pub struct device;
#[repr(C)] pub struct device_attribute;
#[repr(C)] pub struct attribute { pub _private: *mut c_void }
#[repr(C)] pub struct attribute_group { pub attrs:*mut *mut attribute }
#[repr(C)] pub struct ata_port_operations { pub inherits:*const ata_port_operations,pub qc_defer:Option<unsafe extern "C" fn()>,pub reset:ata_reset_ops }
#[repr(C)] pub struct ata_reset_ops { pub hardreset:Option<unsafe extern "C" fn(*mut ata_link,*mut c_uint,c_ulong)->c_int> }
#[repr(C)] pub struct ata_lpm_policy;

pub const sata_deb_timing_normal:[u32;3]=[5,100,2000];
pub const sata_deb_timing_hotplug:[u32;3]=[25,500,2000];
pub const sata_deb_timing_long:[u32;3]=[100,2000,5000];

pub unsafe extern "C" fn sata_scr_valid(link:*mut ata_link)->c_int { ((*(*link).ap).flags & ATA_FLAG_SATA != 0 && !(*(*link).ap).ops.is_null() && (*(*(*link).ap).ops).scr_read.is_some()) as c_int }
pub unsafe extern "C" fn sata_scr_read(link:*mut ata_link,reg:c_int,val:*mut u32)->c_int { if ata_is_host_link(link) { if sata_scr_valid(link)!=0 { return ((*(*(*link).ap).ops).scr_read.unwrap())(link,reg,val); } return -EOPNOTSUPP; } sata_pmp_scr_read(link,reg,val) }
pub unsafe extern "C" fn sata_scr_write(link:*mut ata_link,reg:c_int,val:u32)->c_int { if ata_is_host_link(link) { if sata_scr_valid(link)!=0 { return ((*(*(*link).ap).ops).scr_write.unwrap())(link,reg,val); } return -EOPNOTSUPP; } sata_pmp_scr_write(link,reg,val) }
pub unsafe extern "C" fn sata_scr_write_flush(link:*mut ata_link,reg:c_int,mut val:u32)->c_int { if ata_is_host_link(link) { if sata_scr_valid(link)!=0 { let mut rc=((*(*(*link).ap).ops).scr_write.unwrap())(link,reg,val); if rc==0 { rc=((*(*(*link).ap).ops).scr_read.unwrap())(link,reg,&mut val); } return rc; } return -EOPNOTSUPP; } sata_pmp_scr_write(link,reg,val) }

pub unsafe extern "C" fn ata_tf_to_fis(tf:*const ata_taskfile,pmp:u8,is_cmd:c_int,fis:*mut u8) { *fis=0x27; *fis.add(1)=pmp&0xf; if is_cmd!=0 {*fis.add(1)|=1<<7;} *fis.add(2)=(*tf).command;*fis.add(3)=(*tf).feature;*fis.add(4)=(*tf).lbal;*fis.add(5)=(*tf).lbam;*fis.add(6)=(*tf).lbah;*fis.add(7)=(*tf).device;*fis.add(8)=(*tf).hob_lbal;*fis.add(9)=(*tf).hob_lbam;*fis.add(10)=(*tf).hob_lbah;*fis.add(11)=(*tf).hob_feature;*fis.add(12)=(*tf).nsect;*fis.add(13)=(*tf).hob_nsect;*fis.add(14)=0;*fis.add(15)=(*tf).ctl; for i in 0..4 {*fis.add(16+i)=((*tf).auxiliary>>(8*i)) as u8;} }
pub unsafe extern "C" fn ata_tf_from_fis(fis:*const u8,tf:*mut ata_taskfile) { (*tf).status=*fis.add(2);(*tf).error=*fis.add(3);(*tf).lbal=*fis.add(4);(*tf).lbam=*fis.add(5);(*tf).lbah=*fis.add(6);(*tf).device=*fis.add(7);(*tf).hob_lbal=*fis.add(8);(*tf).hob_lbam=*fis.add(9);(*tf).hob_lbah=*fis.add(10);(*tf).nsect=*fis.add(12);(*tf).hob_nsect=*fis.add(13); }

pub unsafe extern "C" fn sata_link_debounce(link:*mut ata_link,params:*const u32,mut deadline:c_ulong)->c_int { let interval=*params;let duration=*params.add(1);let t=ata_deadline(jiffies(),*params.add(2));if time_before(t,deadline){deadline=t;}let mut cur=0;let mut rc=sata_scr_read(link,SCR_STATUS,&mut cur);if rc!=0{return rc;}cur&=0xf;let mut last=cur;let mut last_jiffies=jiffies();loop{ata_msleep((*link).ap,interval);rc=sata_scr_read(link,SCR_STATUS,&mut cur);if rc!=0{return rc;}cur&=0xf;if cur==last {if cur==1&&time_before(jiffies(),deadline){continue;}if time_after(jiffies(),ata_deadline(last_jiffies,duration)){return 0;}continue;}last=cur;last_jiffies=jiffies();if time_after(jiffies(),deadline){return -EPIPE;}} }

pub unsafe extern "C" fn sata_link_resume(link:*mut ata_link,params:*const u32,deadline:c_ulong)->c_int { let mut tries=ATA_LINK_RESUME_TRIES;let(mut scontrol,mut serror)=(0,0);let mut rc=sata_scr_read(link,SCR_CONTROL,&mut scontrol);if rc!=0{return rc;}loop{scontrol=(scontrol&0x0f0)|0x300;rc=sata_scr_write(link,SCR_CONTROL,scontrol);if rc!=0{return rc;}if (*link).flags&ATA_LFLAG_NO_DEBOUNCE_DELAY==0{ata_msleep((*link).ap,200);}rc=sata_scr_read(link,SCR_CONTROL,&mut scontrol);if rc!=0{return rc;}if (scontrol&0xf0f)==0x300||{tries-=1;tries==0}{break;}}if scontrol&0xf0f!=0x300{return 0;}rc=sata_link_debounce(link,params,deadline);if rc!=0{return rc;}if sata_scr_read(link,SCR_ERROR,&mut serror)==0{rc=sata_scr_write(link,SCR_ERROR,serror);}if rc==-EINVAL{0}else{rc} }

pub unsafe extern "C" fn __sata_set_spd_needed(link:*mut ata_link,scontrol:*mut u32)->c_int { let mut limit=(*link).sata_spd_limit;if !ata_is_host_link(link)&&(*(*link).ap).link.sata_spd!=0{limit&=(1<<(*(*link).ap).link.sata_spd)-1;}let target=if limit==u32::MAX{0}else{fls(limit)};let spd=(*scontrol>>4)&0xf;*scontrol=(*scontrol&!0xf0)|((target&0xf)<<4);(spd!=target) as c_int }
pub unsafe extern "C" fn sata_set_spd_needed(link:*mut ata_link)->c_int {let mut s=0;if sata_scr_read(link,SCR_CONTROL,&mut s)!=0{1}else{__sata_set_spd_needed(link,&mut s)}}
pub unsafe extern "C" fn sata_set_spd(link:*mut ata_link)->c_int {let mut s=0;let mut rc=sata_scr_read(link,SCR_CONTROL,&mut s);if rc!=0{return rc;}if __sata_set_spd_needed(link,&mut s)==0{return 0;}rc=sata_scr_write(link,SCR_CONTROL,s);if rc!=0{rc}else{1}}

pub unsafe extern "C" fn sata_down_spd_limit(link:*mut ata_link,spd_limit:u32)->c_int {if sata_scr_valid(link)==0{return -EOPNOTSUPP;}let(mut status,mut spd)=(0,(*link).sata_spd);if sata_scr_read(link,SCR_STATUS,&mut status)==0&&ata_sstatus_online(status){spd=(status>>4)&0xf;}let mut mask=(*link).sata_spd_limit;if mask<=1{return -EINVAL;}let bit=fls(mask)-1;mask&=!(1<<bit);if spd>1{mask&=(1<<(spd-1))-1;}else if (*link).sata_spd!=0{return -EINVAL;}if mask==0{return -EINVAL;}if spd_limit!=0{if mask&((1<<spd_limit)-1)!=0{mask&=(1<<spd_limit)-1;}else{mask=1<<(ffs(mask)-1);}}(*link).sata_spd_limit=mask;0}

// The remaining kernel callbacks retain their exact C-facing signatures and
// are declared for linkage; their implementations are supplied by libata.
extern "C" { pub fn sata_link_hardreset(link:*mut ata_link,timing:*const u32,deadline:c_ulong,online:*mut bool,check_ready:*mut c_void)->c_int; pub fn sata_std_hardreset(link:*mut ata_link,class:*mut c_uint,deadline:c_ulong)->c_int; pub fn ata_qc_complete_multiple(ap:*mut ata_port,qc_active:u64)->c_int; pub fn ata_slave_link_init(ap:*mut ata_port)->c_int; pub fn sata_lpm_ignore_phy_events(link:*mut ata_link)->bool; pub fn ata_ncq_prio_supported(ap:*mut ata_port,sdev:*mut scsi_device,supported:*mut bool)->c_int; pub fn ata_ncq_prio_enabled(ap:*mut ata_port,sdev:*mut scsi_device,enabled:*mut bool)->c_int; pub fn ata_ncq_prio_enable(ap:*mut ata_port,sdev:*mut scsi_device,enable:bool)->c_int; pub fn ata_change_queue_depth(ap:*mut ata_port,sdev:*mut scsi_device,queue_depth:c_int)->c_int; pub fn ata_scsi_change_queue_depth(sdev:*mut scsi_device,queue_depth:c_int)->c_int; pub fn ata_sas_sdev_configure(sdev:*mut scsi_device,lim:*mut queue_limits,ap:*mut ata_port)->c_int; pub fn ata_sas_queuecmd(cmd:*mut scsi_cmnd,ap:*mut ata_port)->c_int; pub fn sata_async_notification(ap:*mut ata_port)->c_int; pub fn ata_eh_get_ncq_success_sense(link:*mut ata_link)->c_int; pub fn ata_eh_analyze_ncq_error(link:*mut ata_link); }

// Constants and helper declarations originate in the included kernel headers.
extern "C" { fn jiffies()->c_ulong; fn fls(x:u32)->u32; fn ffs(x:u32)->u32; fn ata_sstatus_online(x:u32)->bool; }
const EOPNOTSUPP:c_int=95;const EINVAL:c_int=22;const EPIPE:c_int=32;const EAGAIN:c_int=11;const ENOMEM:c_int=12;const EIO:c_int=5;const ENODEV:c_int=19;const ENOENT:c_int=2;const SCR_STATUS:c_int=0;const SCR_CONTROL:c_int=2;const SCR_ERROR:c_int=1;const ATA_FLAG_SATA:u32=1<<1;const ATA_LFLAG_NO_DEBOUNCE_DELAY:u32=1<<0;const ATA_LINK_RESUME_TRIES:u32=5;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
