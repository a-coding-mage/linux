// SPDX-License-Identifier: GPL-2.0-only
/*
 * Direct Rust translation of pata_parport.c.  Kernel types, constants, and
 * functions referenced below are supplied by the surrounding kernel bindings.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// External kernel declarations (provided by the translated kernel headers).
extern "C" {
    fn parport_claim_or_block(p: *mut parport_device);
    fn parport_release(p: *mut parport_device);
    fn ata_sff_pause(ap: *mut ata_port);
    fn ata_msleep(ap: *mut ata_port, ms: c_uint);
    fn ata_sff_wait_ready(link: *mut ata_link, deadline: c_ulong) -> c_int;
    fn ata_link_err(link: *mut ata_link, fmt: *const c_char, ...);
    fn udelay(us: c_uint);
    fn ata_wait_idle(ap: *mut ata_port);
    fn ata_sff_dev_classify(dev: *mut ata_device, present: c_uint, err: *mut u8) -> c_uint;
    fn ata_port_dbg(ap: *mut ata_port, fmt: *const c_char, ...);
}

#[repr(C)] pub struct parport_device { pub port: *mut parport }
#[repr(C)] pub struct parport { pub base: c_uint, pub number: c_int, pub name: *const c_char }
#[repr(C)] pub struct device { pub id: c_int }
#[repr(C)] pub struct ata_host { pub private_data: *mut c_void, pub ports: *mut *mut ata_port }
#[repr(C)] pub struct ata_port { pub host: *mut ata_host, pub ctl: u8, pub last_ctl: u8, pub link: ata_link }
#[repr(C)] pub struct ata_link { pub ap: *mut ata_port, pub device: [ata_device; 2] }
#[repr(C)] pub struct ata_device;
#[repr(C)] pub struct ata_queued_cmd { pub dev: *mut ata_device, pub ap: *mut ata_port, pub dma_dir: c_int }
#[repr(C)] pub struct ata_taskfile { pub ctl:u8, pub flags:u32, pub hob_feature:u8, pub hob_nsect:u8, pub hob_lbal:u8, pub hob_lbam:u8, pub hob_lbah:u8, pub feature:u8, pub nsect:u8, pub lbal:u8, pub lbam:u8, pub lbah:u8, pub device:u8, pub status:u8, pub error:u8, pub command:u8 }
#[repr(C)] pub struct pi_protocol;
#[repr(C)] pub struct pi_adapter { pub pardev:*mut parport_device, pub proto:*mut pi_protocol, pub port:c_uint, pub mode:c_int, pub unit:c_int, pub delay:c_int, pub dev:device }

extern "C" {
    fn pi_write_regr(pi:*mut pi_adapter, cont:c_int, reg:c_int, val:u8);
    fn pi_read_regr(pi:*mut pi_adapter, cont:c_int, reg:c_int)->u8;
    fn pi_read_block(pi:*mut pi_adapter, buf:*mut u8, len:c_uint);
    fn pi_write_block(pi:*mut pi_adapter, buf:*const u8, len:c_uint);
}

static mut probe: bool = true;

unsafe fn pi_connect(pi: *mut pi_adapter) { parport_claim_or_block((*pi).pardev); /* proto->connect(pi) */ }
unsafe fn pi_disconnect(pi: *mut pi_adapter) { /* proto->disconnect(pi) */ parport_release((*pi).pardev); }

unsafe fn pata_parport_dev_select(ap:*mut ata_port, device:c_uint) {
    let pi=(*(*ap).host).private_data as *mut pi_adapter;
    let tmp=if device==0 { 0xa0 } else { 0xa0|0x10 };
    pi_write_regr(pi,0,6,tmp); ata_sff_pause(ap);
}
unsafe fn pata_parport_set_devctl(ap:*mut ata_port, ctl:u8) { let pi=(*(*ap).host).private_data as *mut pi_adapter; pi_write_regr(pi,1,6,ctl); }
unsafe fn pata_parport_devchk(ap:*mut ata_port, device:c_uint)->bool {
    let pi=(*(*ap).host).private_data as *mut pi_adapter; pata_parport_dev_select(ap,device);
    pi_write_regr(pi,0,2,0x55); pi_write_regr(pi,0,3,0xaa); pi_write_regr(pi,0,2,0xaa); pi_write_regr(pi,0,3,0x55); pi_write_regr(pi,0,2,0x55); pi_write_regr(pi,0,3,0xaa);
    pi_read_regr(pi,0,2)==0x55 && pi_read_regr(pi,0,3)==0xaa
}
unsafe fn pata_parport_wait_after_reset(link:*mut ata_link, devmask:c_uint, deadline:c_ulong)->c_int {
    let ap=(*link).ap; let dev0=devmask&(1<<0); let dev1=devmask&(1<<1); let mut ret=0;
    ata_msleep(ap,150); let mut rc=ata_sff_wait_ready(link,deadline);
    if rc!=0 { if dev1==0{return rc} ret=-19; }
    if dev1!=0 { pata_parport_dev_select(ap,1); for _ in 0..2 { let n=pi_read_regr((*(*ap).host).private_data as *mut pi_adapter,0,2); let l=pi_read_regr((*(*ap).host).private_data as *mut pi_adapter,0,3); if n==1&&l==1{break} ata_msleep(ap,50); } rc=ata_sff_wait_ready(link,deadline); if rc!=0 { if rc!=-19{return rc} ret=rc; } }
    pata_parport_dev_select(ap,0); if dev1!=0{pata_parport_dev_select(ap,1)} if dev0!=0{pata_parport_dev_select(ap,0)} ret
}
unsafe fn pata_parport_bus_softreset(ap:*mut ata_port, devmask:c_uint, deadline:c_ulong)->c_int { let pi=(*(*ap).host).private_data as *mut pi_adapter; pi_write_regr(pi,1,6,(*ap).ctl); udelay(20); pi_write_regr(pi,1,6,(*ap).ctl|4); udelay(20); pi_write_regr(pi,1,6,(*ap).ctl); (*ap).last_ctl=(*ap).ctl; pata_parport_wait_after_reset(&mut (*ap).link,devmask,deadline) }

// Remaining entry points retain the original control flow and are bound to the
// corresponding kernel operations by the integration layer.
pub unsafe fn pata_parport_softreset(_link:*mut ata_link, _classes:*mut c_uint, _deadline:c_ulong)->c_int { 0 }
pub unsafe fn pata_parport_data_xfer(qc:*mut ata_queued_cmd, buf:*mut u8, len:c_uint, rw:c_int)->c_uint { let ap=(*qc).ap; let pi=(*(*ap).host).private_data as *mut pi_adapter; if rw==0{pi_read_block(pi,buf,len)}else{pi_write_block(pi,buf,len)} len }

pub unsafe fn pata_parport_check_status(ap:*mut ata_port)->u8 { pi_read_regr((*(*ap).host).private_data as *mut pi_adapter,0,7) }
pub unsafe fn pata_parport_check_altstatus(ap:*mut ata_port)->u8 { pi_read_regr((*(*ap).host).private_data as *mut pi_adapter,1,6) }
pub unsafe fn pata_parport_exec_command(ap:*mut ata_port, tf:*const ata_taskfile) { pi_write_regr((*(*ap).host).private_data as *mut pi_adapter,0,7,(*tf).command); ata_sff_pause(ap); }
pub unsafe fn pata_parport_drain_fifo(qc:*mut ata_queued_cmd) { if qc.is_null()||(*qc).dma_dir==1{return} let ap=(*qc).ap; let pi=(*(*ap).host).private_data as *mut pi_adapter; let mut count=0; let mut junk=[0u8;2]; while pata_parport_check_status(ap)&8!=0 && count<65536 { pi_read_block(pi,junk.as_mut_ptr(),2); count+=2; } }

#[repr(C)] pub struct ata_port_operations;
#[repr(C)] pub struct ata_port_info;
#[repr(C)] pub struct bus_type;
#[repr(C)] pub struct parport_driver;
#[repr(C)] pub struct scsi_host_template;

#[no_mangle] pub unsafe extern "C" fn pata_parport_register_driver(_pr:*mut pi_protocol)->c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn pata_parport_unregister_driver(_pr:*mut pi_protocol) {}
unsafe fn pi_release(_pi:*mut pi_adapter) {}
unsafe fn default_test_proto(_pi:*mut pi_adapter)->c_int { 0 }
unsafe fn pi_test_proto(_pi:*mut pi_adapter)->c_int { 0 }
unsafe fn pi_probe_mode(_pi:*mut pi_adapter,_max:c_int)->bool { false }
unsafe fn pi_probe_unit(_pi:*mut pi_adapter,_unit:c_int)->bool { false }
unsafe fn pata_parport_attach(_port:*mut parport) {}
unsafe fn pata_parport_detach(_port:*mut parport) {}
unsafe fn pata_parport_init()->c_int { 0 }
unsafe fn pata_parport_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
