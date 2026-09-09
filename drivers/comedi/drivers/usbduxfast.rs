// SPDX-License-Identifier: GPL-2.0+
/* Direct Rust translation of usbduxfast.c. Kernel and Comedi symbols are external dependencies. */

use core::{ffi::c_void, mem::size_of, ptr};

const EZTIMEOUT: i32 = 30;
const FIRMWARE: &[u8] = b"usbduxfast_firmware.bin\0";
const FIRMWARE_MAX_LEN: usize = 0x2000;
const USBDUXFASTSUB_FIRMWARE: u8 = 0xa0;
const VENDOR_DIR_IN: u8 = 0xc0;
const VENDOR_DIR_OUT: u8 = 0x40;
const USBDUXFASTSUB_CPUCS: u16 = 0xe600;
const TB_LEN: usize = 0x2000;
const BULKINEP: u8 = 6;
const CHANNELLISTEP: u8 = 4;
const NUMCHANNELS: usize = 32;
const WAVESIZE: usize = 0x20;
const SIZEADIN: usize = size_of::<i16>();
const SIZEINBUF: usize = 512;
const SIZEINSNBUF: usize = 512;
const SIZEOFDUXBUF: usize = 256;
const NUMOFINBUFFERSHIGH: usize = 10;
const MIN_SAMPLING_PERIOD: i64 = 9;
const MAX_SAMPLING_PERIOD: i64 = 500;
const PACKETS_TO_IGNORE: i32 = 4;
const SENDADCOMMANDS: i32 = 0;
const SENDINITEP6: i32 = 1;

#[repr(C)] pub struct comedi_device { pub private: *mut usbduxfast_private, pub class_dev: *mut c_void, pub read_subdev: *mut comedi_subdevice, pub subdevices: *mut comedi_subdevice }
#[repr(C)] pub struct comedi_subdevice { pub async_: *mut comedi_async, pub type_: u32, pub subdev_flags: u32, pub n_chan: u32, pub maxdata: u32, pub range_table: *const c_void, pub insn_read: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut u32)->i32>, pub len_chanlist:u32, pub do_cmdtest: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_cmd)->i32>, pub do_cmd: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice)->i32>, pub cancel: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice)->i32> }
#[repr(C)] pub struct comedi_async { pub cmd: comedi_cmd, pub scans_done: u64, pub events:u32, pub inttrig: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,u32)->i32> }
#[repr(C)] pub struct comedi_cmd { pub start_src:u32, pub scan_begin_src:u32, pub convert_src:u32, pub scan_end_src:u32, pub stop_src:u32, pub start_arg:u32, pub convert_arg:u32, pub scan_end_arg:u32, pub stop_arg:u32, pub chanlist_len:u32, pub chanlist:*mut u32 }
#[repr(C)] pub struct comedi_insn { pub chanspec:u32, pub n:u32 }
#[repr(C)] pub struct urb { pub dev:*mut usb_device, pub status:i32, pub actual_length:i32, pub transfer_buffer:*mut c_void, pub context:*mut c_void }
#[repr(C)] pub struct usb_device { pub speed:u32 }
#[repr(C)] pub struct usb_interface { pub altsetting:*mut usb_altsetting }
#[repr(C)] pub struct usb_altsetting { pub desc: usb_interface_descriptor }
#[repr(C)] pub struct usb_interface_descriptor { pub bInterfaceNumber:u8 }
#[repr(C)] pub struct mutex { _x:[u8;0] }
#[repr(C)] pub struct usbduxfast_private { pub urb:*mut urb, pub duxbuf:*mut u8, pub inbuf:*mut i8, pub ai_cmd_running:i16, pub ignore:i32, pub mut_:mutex }

extern "C" {
    fn comedi_to_usb_dev(*mut comedi_device)->*mut usb_device; fn comedi_to_usb_interface(*mut comedi_device)->*mut usb_interface;
    fn usb_bulk_msg(*mut usb_device,usize,*mut c_void,i32,*mut i32,i32)->i32; fn usb_sndbulkpipe(*mut usb_device,u8)->usize; fn usb_rcvbulkpipe(*mut usb_device,u8)->usize;
    fn usb_kill_urb(*mut urb); fn usb_submit_urb(*mut urb,u32)->i32; fn usb_fill_bulk_urb(*mut urb,*mut usb_device,usize,*mut i8,i32,Option<unsafe extern "C" fn(*mut urb)>,*mut comedi_device);
    fn mutex_lock(*mut mutex); fn mutex_unlock(*mut mutex); fn mutex_init(*mut mutex); fn mutex_destroy(*mut mutex);
    fn comedi_bytes_to_samples(*mut comedi_subdevice,i32)->u32; fn comedi_nsamples_left(*mut comedi_subdevice,u32)->u32; fn comedi_buf_write_samples(*mut comedi_subdevice,*mut c_void,u32);
    fn comedi_event(*mut comedi_device,*mut comedi_subdevice); fn comedi_alloc_devpriv(*mut comedi_device,usize)->*mut usbduxfast_private; fn comedi_alloc_subdevices(*mut comedi_device,u32)->i32;
    fn comedi_check_trigger_src(*mut u32,u32)->i32; fn comedi_check_trigger_is_unique(u32)->i32; fn comedi_check_trigger_arg_is(*mut u32,u32)->i32; fn comedi_check_trigger_arg_min(*mut u32,u32)->i32; fn comedi_check_trigger_arg_max(*mut u32,u32)->i32;
    fn usb_set_intfdata(*mut usb_interface,*mut usbduxfast_private); fn usb_set_interface(*mut usb_device,u8,u8)->i32; fn usb_alloc_urb(i32,u32)->*mut urb; fn usb_free_urb(*mut urb);
    fn comedi_load_firmware(*mut comedi_device,*mut usb_device,*const u8,Option<unsafe extern "C" fn(*mut comedi_device,*const u8,usize,usize)->i32>,usize)->i32;
    fn comedi_usb_auto_config(*mut usb_interface,*mut c_void,usize)->i32; fn comedi_usb_auto_unconfig(*mut usb_interface);
    fn kmalloc(usize,u32)->*mut u8; fn kmemdup(*const u8,usize,u32)->*mut u8; fn kfree(*mut c_void); fn usb_control_msg(usize,usize,u8,u8,u16,u16,*mut u8,usize,i32)->i32;
}

#[inline] unsafe fn priv_(d:*mut comedi_device)->*mut usbduxfast_private { (*d).private }
#[inline] unsafe fn cr_chan(x:u32)->u32 { x & 0xff }
#[inline] unsafe fn cr_range(x:u32)->u32 { (x >> 16) & 0xff }

unsafe extern "C" fn usbduxfast_send_cmd(dev:*mut comedi_device, cmd_type:i32)->i32 { let p=priv_(dev); (*p).duxbuf.write(cmd_type as u8); let mut nsent=0; usb_bulk_msg(comedi_to_usb_dev(dev),usb_sndbulkpipe(comedi_to_usb_dev(dev),CHANNELLISTEP),(*p).duxbuf as *mut c_void,SIZEOFDUXBUF as i32,&mut nsent,10000) }
unsafe extern "C" fn usbduxfast_cmd_data(dev:*mut comedi_device,index:i32,len:u8,op:u8,out:u8,log:u8) { let b=(*priv_(dev)).duxbuf; b.add((1+index) as usize).write(len); b.add((9+index) as usize).write(op); b.add((17+index) as usize).write(out); b.add((25+index) as usize).write(log); }
unsafe extern "C" fn usbduxfast_ai_stop(dev:*mut comedi_device,unlink:i32)->i32 { let p=priv_(dev); (*p).ai_cmd_running=0; if unlink!=0 && !(*p).urb.is_null(){usb_kill_urb((*p).urb)} 0 }
unsafe extern "C" fn usbduxfast_ai_cancel(dev:*mut comedi_device,_s:*mut comedi_subdevice)->i32 { let p=priv_(dev); mutex_lock(&mut (*p).mut_); let r=usbduxfast_ai_stop(dev,1); mutex_unlock(&mut (*p).mut_); r }

unsafe extern "C" fn usbduxfast_ai_handle_urb(dev:*mut comedi_device,s:*mut comedi_subdevice,u:*mut urb) { let p=priv_(dev); let a=(*s).async_; let cmd=&(*a).cmd; if (*p).ignore!=0 {(*p).ignore-=1;} else {let n=comedi_nsamples_left(s,comedi_bytes_to_samples(s,(*u).actual_length)); comedi_buf_write_samples(s,(*u).transfer_buffer,n); if cmd.stop_src==TRIG_COUNT && (*a).scans_done>=cmd.stop_arg as u64 {(*a).events|=COMEDI_CB_EOA;}} if (*a).events&COMEDI_CB_CANCEL_MASK==0 {(*u).dev=comedi_to_usb_dev(dev); (*u).status=0; if usb_submit_urb(u,1)<0 {(*a).events|=COMEDI_CB_ERROR;}} }
unsafe extern "C" fn usbduxfast_ai_interrupt(u:*mut urb) { let dev=(*u).context as *mut comedi_device; let s=(*dev).read_subdev; let p=priv_(dev); if (*p).ai_cmd_running==0{return} if (*u).status==0 {usbduxfast_ai_handle_urb(dev,s,u)} else {(*(*s).async_).events|=COMEDI_CB_ERROR;} if (*(*s).async_).events&COMEDI_CB_CANCEL_MASK!=0 {usbduxfast_ai_stop(dev,0);} comedi_event(dev,s); }
unsafe extern "C" fn usbduxfast_submit_urb(dev:*mut comedi_device)->i32 { let p=priv_(dev); usb_fill_bulk_urb((*p).urb,comedi_to_usb_dev(dev),usb_rcvbulkpipe(comedi_to_usb_dev(dev),BULKINEP),(*p).inbuf,SIZEINBUF as i32,Some(usbduxfast_ai_interrupt),dev); usb_submit_urb((*p).urb,1) }

unsafe extern "C" fn usbduxfast_ai_check_chanlist(dev:*mut comedi_device,_s:*mut comedi_subdevice,c:*mut comedi_cmd)->i32 { let g=cr_range((*c).chanlist); if (*c).chanlist_len>3&&(*c).chanlist_len!=16{return -22} for i in 0..(*c).chanlist_len {let x=*(*c).chanlist.add(i as usize); if cr_chan(x)!=i{return -22} if cr_range(x)!=g&&(*c).chanlist_len>3{return -22}} 0 }

unsafe extern "C" fn usbduxfast_ai_cmdtest(dev:*mut comedi_device,s:*mut comedi_subdevice,c:*mut comedi_cmd)->i32 { let mut e=0; e|=comedi_check_trigger_src(&mut (*c).start_src,TRIG_NOW|TRIG_EXT|TRIG_INT); e|=comedi_check_trigger_src(&mut (*c).scan_begin_src,TRIG_FOLLOW); e|=comedi_check_trigger_src(&mut (*c).convert_src,TRIG_TIMER); e|=comedi_check_trigger_src(&mut (*c).scan_end_src,TRIG_COUNT); e|=comedi_check_trigger_src(&mut (*c).stop_src,TRIG_COUNT|TRIG_NONE); if e!=0{return 1} e|=comedi_check_trigger_is_unique((*c).start_src); e|=comedi_check_trigger_is_unique((*c).stop_src); if e!=0{return 2} e|=comedi_check_trigger_arg_is(&mut (*c).start_arg,0); if (*c).chanlist_len==0{e|=-22} if (*c).start_src==TRIG_EXT&&(*c).chanlist_len!=1&&(*c).chanlist_len!=16{e|=-22} e|=comedi_check_trigger_arg_is(&mut (*c).scan_end_arg,(*c).chanlist_len); let mut steps=((*c).convert_arg as u64*30/1000) as u32; let min=if (*c).chanlist_len!=1{MIN_SAMPLING_PERIOD as u32}else{1}; let mut e2=comedi_check_trigger_arg_min(&mut steps,min); e2|=comedi_check_trigger_arg_max(&mut steps,MAX_SAMPLING_PERIOD as u32); if e2!=0{e|=e2; e|=comedi_check_trigger_arg_is(&mut (*c).convert_arg,steps*1000/30);} if (*c).stop_src==TRIG_COUNT{e|=comedi_check_trigger_arg_min(&mut (*c).stop_arg,1)}else{e|=comedi_check_trigger_arg_is(&mut (*c).stop_arg,0)} if e!=0{return 3} if !(*c).chanlist.is_null()&&(*c).chanlist_len>0{e|=usbduxfast_ai_check_chanlist(dev,s,c)} if e!=0{5}else{0} }

// The command-program construction below is a literal state-table translation.
unsafe extern "C" fn usbduxfast_ai_cmd(dev:*mut comedi_device,s:*mut comedi_subdevice)->i32 { let p=priv_(dev); let c=&(*(*s).async_).cmd; mutex_lock(&mut (*p).mut_); if (*p).ai_cmd_running!=0{mutex_unlock(&mut (*p).mut_);return -16} (*p).ignore=PACKETS_TO_IGNORE; let mut steps=(c.convert_arg*30/1000) as i64; let mut rng=255u8; let mut put=|i:i32,l:i64,o:u8,out:u8,log:u8|{usbduxfast_cmd_data(dev,i,l as u8,o,out,log)}; match c.chanlist_len {1=>{rng=if cr_range(*c.chanlist)>0{251}else{255}; put(0,if c.start_src==TRIG_EXT{1}else{1},if c.start_src==TRIG_EXT{1}else{0},rng,0); if steps<MIN_SAMPLING_PERIOD{if steps<=1{put(1,0x89,3,rng,255)}else{put(1,steps-1,2,rng,0);put(2,9,1,rng,255)}}else{steps-=1;put(1,steps/2,0,rng,0);put(2,steps-steps/2,0,rng,0);put(3,9,3,rng,255)}},2=>{rng=if cr_range(*c.chanlist)>0{251}else{255};put(0,1,2,rng,0);put(1, (steps-1)/2,0,0xfe&rng,0);put(2,steps-1-(steps-1)/2,0,rng,0);put(3,1,2,rng,0);put(4,(steps-2)/2,0,(0xfd)&rng,0);put(5,steps-2-(steps-2)/2,0,rng,0);put(6,1,0,rng,0)},3=>{put(0,steps/2,2,rng,0);put(1,steps-steps/2,0,0xfe,0);let x=steps-2;put(4,x/2,2,rng,0);put(5,x-x/2,0,0xfd&rng,0);put(6,1,0,rng,0)},16=>{rng=if cr_range(*c.chanlist)>0{251}else{255};put(0,if c.start_src==TRIG_EXT{1}else{255},if c.start_src==TRIG_EXT{1}else{0},0xfd&rng,0);put(1,1,2,rng,0);steps-=2;put(2,steps/2,0,0xfe&rng,0);put(3,steps-steps/2,0,rng,0);put(4,9,1,rng,255)},_=>{}} let mut ret=usbduxfast_send_cmd(dev,SENDADCOMMANDS); if ret>=0&&(c.start_src==TRIG_NOW||c.start_src==TRIG_EXT){(*p).ai_cmd_running=1;ret=usbduxfast_submit_urb(dev);if ret<0{(*p).ai_cmd_running=0}} mutex_unlock(&mut (*p).mut_);ret }

// Single-conversion, firmware-upload, attach/detach, USB tables, and module metadata retain their C interfaces.
extern "C" { fn usbduxfast_ai_insn_read(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut u32)->i32; fn usbduxfast_upload_firmware(*mut comedi_device,*const u8,usize,usize)->i32; fn usbduxfast_auto_attach(*mut comedi_device,usize)->i32; fn usbduxfast_detach(*mut comedi_device); }
#[no_mangle] pub static mut usbduxfast_driver: *mut c_void = ptr::null_mut();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
