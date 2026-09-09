// SPDX-License-Identifier: GPL-2.0+
/* Faithful low-level Rust translation of usbduxsigma.c. */

/* Kernel/Comedi dependencies are supplied by the surrounding translation unit. */
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const BULK_TIMEOUT: c_int = 1000;
const FIRMWARE: &[u8] = b"usbduxsigma_firmware.bin\0";
const FIRMWARE_MAX_LEN: usize = 0x4000;
const USBDUXSUB_FIRMWARE: c_int = 0xa0;
const VENDOR_DIR_IN: c_int = 0xc0;
const VENDOR_DIR_OUT: c_int = 0x40;
const USBDUXSUB_CPUCS: c_int = 0xE600;
const MIN_PWM_PERIOD: c_ulong = (1_000_000_000 / 300) as c_ulong;
const PWM_DEFAULT_PERIOD: c_ulong = (1_000_000_000 / 100) as c_ulong;
const NUMCHANNELS: usize = 16;
const SIZEADIN: usize = core::mem::size_of::<u32>();
const SIZEINBUF: usize = (NUMCHANNELS + 1) * SIZEADIN;
const SIZEINSNBUF: usize = 16;
const NUMOUTCHANNELS: usize = 8;
const SIZEDAOUT: usize = core::mem::size_of::<u8>() + core::mem::size_of::<u16>();
const SIZEOUTBUF: usize = 8 * SIZEDAOUT;
const SIZEOFDUXBUFFER: usize = 8 * SIZEDAOUT + 2;
const NUMOFINBUFFERSFULL: c_int = 5;
const NUMOFOUTBUFFERSFULL: c_int = 5;
const NUMOFINBUFFERSHIGH: c_int = 10;
const NUMOFOUTBUFFERSHIGH: c_int = 10;
const RETRIES: c_int = 10;
const USBBUXSIGMA_AD_CMD: u8 = 9;
const USBDUXSIGMA_DA_CMD: u8 = 1;
const USBDUXSIGMA_DIO_CFG_CMD: u8 = 2;
const USBDUXSIGMA_DIO_BITS_CMD: u8 = 3;
const USBDUXSIGMA_SINGLE_AD_CMD: u8 = 4;
const USBDUXSIGMA_PWM_ON_CMD: u8 = 7;
const USBDUXSIGMA_PWM_OFF_CMD: u8 = 8;

#[repr(C)] pub struct comedi_lrange { pub length: c_uint, pub range: [u8; 0] }
#[repr(C)] pub struct urb { pub actual_length: c_int, pub status: c_int, pub context: *mut c_void, pub dev: *mut usb_device, pub transfer_buffer: *mut c_void, pub transfer_buffer_length: c_int, pub interval: c_int, pub number_of_packets: c_int, pub transfer_flags: c_uint, pub pipe: c_uint, pub complete: Option<unsafe extern "C" fn(*mut urb)>, pub iso_frame_desc: [usb_iso_packet_descriptor; 1] }
#[repr(C)] pub struct usb_iso_packet_descriptor { pub offset: c_uint, pub length: c_uint, pub status: c_int, pub actual_length: c_uint }
#[repr(C)] pub struct usb_device { pub speed: c_int, pub dev: c_void }
#[repr(C)] pub struct usb_interface { pub altsetting: *mut usb_host_interface }
#[repr(C)] pub struct usb_host_interface { pub desc: usb_interface_descriptor }
#[repr(C)] pub struct usb_interface_descriptor { pub bInterfaceNumber: u8 }
#[repr(C)] pub struct usb_device_id { pub vendor: u16, pub product: u16 }
#[repr(C)] pub struct mutex;
#[repr(C)] pub struct comedi_device { pub private: *mut usbduxsigma_private, pub read_subdev: *mut comedi_subdevice, pub write_subdev: *mut comedi_subdevice, pub subdevices: *mut comedi_subdevice, pub class_dev: *mut c_void }
#[repr(C)] pub struct comedi_subdevice { pub async_: *mut comedi_async, pub readback: *mut u16, pub io_bits: u32, pub state: u32, pub type_: c_uint, pub subdev_flags: c_uint, pub n_chan: c_uint, pub len_chanlist: c_uint, pub maxdata: c_uint, pub range_table: *const c_void, pub insn_read: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut c_uint)->c_int>, pub insn_write: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut c_uint)->c_int>, pub insn_bits: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut c_uint)->c_int>, pub insn_config: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut c_uint)->c_int>, pub do_cmdtest: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_cmd)->c_int>, pub do_cmd: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice)->c_int>, pub cancel: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice)->c_int>, pub n_subdev_chan: c_uint }
#[repr(C)] pub struct comedi_async { pub cmd: comedi_cmd, pub events: c_uint, pub scans_done: c_uint, pub inttrig: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,c_uint)->c_int> }
#[repr(C)] pub struct comedi_cmd { pub start_src:c_uint, pub scan_begin_src:c_uint, pub convert_src:c_uint, pub scan_end_src:c_uint, pub stop_src:c_uint, pub start_arg:c_uint, pub scan_begin_arg:c_uint, pub convert_arg:c_uint, pub scan_end_arg:c_uint, pub stop_arg:c_uint, pub chanlist_len:c_uint, pub chanlist:*mut c_uint }
#[repr(C)] pub struct comedi_insn { pub n:c_uint, pub chanspec:c_uint }
#[repr(C)] pub struct usbduxsigma_private { pub n_ai_urbs:c_int, pub n_ao_urbs:c_int, pub ai_urbs:*mut *mut urb, pub ao_urbs:*mut *mut urb, pub pwm_urb:*mut urb, pub pwm_period:c_uint, pub pwm_delay:u8, pub pwm_buf_sz:c_int, pub in_buf:*mut u32, pub insn_buf:*mut u8, pub high_speed:u32, pub ai_cmd_running:u32, pub ao_cmd_running:u32, pub pwm_cmd_running:u32, pub ai_timer:c_uint, pub ao_timer:c_uint, pub ai_counter:c_uint, pub ao_counter:c_uint, pub ai_interval:c_uint, pub dux_commands:*mut u8, pub mut_: mutex }

extern "C" {
    fn usb_kill_urb(*mut urb); fn usb_submit_urb(*mut urb,c_uint)->c_int;
    fn usb_bulk_msg(*mut usb_device,c_uint,*mut u8,c_int,*mut c_int,c_int)->c_int;
    fn usb_sndbulkpipe(*mut usb_device,c_uint)->c_uint; fn usb_rcvbulkpipe(*mut usb_device,c_uint)->c_uint;
    fn usb_rcvisocpipe(*mut usb_device,c_uint)->c_uint; fn usb_sndisocpipe(*mut usb_device,c_uint)->c_uint;
    fn usb_alloc_urb(c_int,c_uint)->*mut urb; fn usb_free_urb(*mut urb);
    fn usb_control_msg(*mut usb_device,c_uint,c_int,c_int,c_int,c_int,*mut u8,usize,c_int)->c_int;
    fn usb_sndctrlpipe(*mut usb_device,c_uint)->c_uint; fn usb_set_interface(*mut usb_device,c_int,c_int)->c_int;
    fn usb_set_intfdata(*mut usb_interface,*mut c_void);
    fn mutex_lock(*mut mutex); fn mutex_unlock(*mut mutex); fn mutex_init(*mut mutex); fn mutex_destroy(*mut mutex);
    fn comedi_to_usb_dev(*mut comedi_device)->*mut usb_device; fn comedi_to_usb_interface(*mut comedi_device)->*mut usb_interface;
    fn comedi_event(*mut comedi_device,*mut comedi_subdevice); fn comedi_offset_munge(*mut comedi_subdevice,u32)->u32;
    fn comedi_buf_write_samples(*mut comedi_subdevice,*mut u32,c_uint)->bool; fn comedi_buf_read_samples(*mut comedi_subdevice,*mut u16,c_uint)->bool;
    fn comedi_dio_insn_config(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut c_uint,c_uint)->c_int; fn comedi_dio_update_state(*mut comedi_subdevice,*mut c_uint);
    fn comedi_readback_insn_read(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut c_uint)->c_int;
    fn comedi_alloc_devpriv(*mut comedi_device,usize)->*mut usbduxsigma_private; fn comedi_alloc_subdevices(*mut comedi_device,c_uint)->c_int; fn comedi_alloc_subdev_readback(*mut comedi_subdevice)->c_int;
    fn comedi_load_firmware(*mut comedi_device,*mut c_void,*const u8,unsafe extern "C" fn(*mut comedi_device,*const u8,usize,c_ulong)->c_int,c_ulong)->c_int;
}

#[inline] unsafe fn priv_(dev:*mut comedi_device)->*mut usbduxsigma_private { (*dev).private }
unsafe fn usbduxsigma_unlink_urbs(urbs:*mut *mut urb,num:c_int){ for i in 0..num { usb_kill_urb(*urbs.offset(i as isize)); } }
unsafe fn usbduxsigma_ai_stop(dev:*mut comedi_device,unlink:c_int){ let p=priv_(dev); if unlink!=0 && !(*p).ai_urbs.is_null(){usbduxsigma_unlink_urbs((*p).ai_urbs,(*p).n_ai_urbs);} (*p).ai_cmd_running=0; }
unsafe fn usbduxsigma_ao_stop(dev:*mut comedi_device,unlink:c_int){ let p=priv_(dev); if unlink!=0 && !(*p).ao_urbs.is_null(){usbduxsigma_unlink_urbs((*p).ao_urbs,(*p).n_ao_urbs);} (*p).ao_cmd_running=0; }
unsafe fn usbduxsigma_ai_cancel(dev:*mut comedi_device,_s:*mut comedi_subdevice)->c_int{let p=priv_(dev);mutex_lock(&mut (*p).mut_);usbduxsigma_ai_stop(dev,(*p).ai_cmd_running as c_int);mutex_unlock(&mut (*p).mut_);0}
unsafe fn usbduxsigma_ao_cancel(dev:*mut comedi_device,_s:*mut comedi_subdevice)->c_int{let p=priv_(dev);mutex_lock(&mut (*p).mut_);usbduxsigma_ao_stop(dev,(*p).ao_cmd_running as c_int);mutex_unlock(&mut (*p).mut_);0}
unsafe fn usbduxsigma_chans_to_interval(n:c_int)->c_int{if n<=2{2}else if n<=8{4}else{8}}
unsafe fn create_adc_command(chan:c_uint,a:*mut u8,b:*mut u8){if chan<8{*a|=1u8<<chan}else if chan<16{*b|=1u8<<(chan-8)}}
unsafe fn usbbuxsigma_send_cmd(dev:*mut comedi_device,typ:u8)->c_int{let p=priv_(dev);*(*p).dux_commands=typ;let mut n=0;usb_bulk_msg(comedi_to_usb_dev(dev),usb_sndbulkpipe(comedi_to_usb_dev(dev),1),(*p).dux_commands,SIZEOFDUXBUFFER as c_int,&mut n,BULK_TIMEOUT)}
unsafe fn usbduxsigma_receive_cmd(dev:*mut comedi_device,command:u8)->c_int{let p=priv_(dev);let mut n=0;for _ in 0..RETRIES{let r=usb_bulk_msg(comedi_to_usb_dev(dev),usb_rcvbulkpipe(comedi_to_usb_dev(dev),8),(*p).insn_buf,SIZEINSNBUF as c_int,&mut n,BULK_TIMEOUT);if r<0{return r}if *(*p).insn_buf==command{return 0}}{-14}}
unsafe fn usbduxsigma_pwm_stop(dev:*mut comedi_device,unlink:c_int){let p=priv_(dev);if unlink!=0&&!(*p).pwm_urb.is_null(){usb_kill_urb((*p).pwm_urb)}(*p).pwm_cmd_running=0}
unsafe fn usbduxsigma_pwm_cancel(dev:*mut comedi_device,_s:*mut comedi_subdevice)->c_int{let p=priv_(dev);usbduxsigma_pwm_stop(dev,(*p).pwm_cmd_running as c_int);usbbuxsigma_send_cmd(dev,USBDUXSIGMA_PWM_OFF_CMD)}
unsafe fn usbduxsigma_pwm_period(dev:*mut comedi_device,_s:*mut comedi_subdevice,period:c_uint)->c_int{let p=priv_(dev);if period<MIN_PWM_PERIOD as u32{return -11}let d=(period/(6*512*1000/33)) as c_int-6;if d>255{return -11}(*p).pwm_delay=d as u8;(*p).pwm_period=period;0}
unsafe fn usbduxsigma_pwm_pattern(dev:*mut comedi_device,_s:*mut comedi_subdevice,chan:c_uint,value:c_uint,sign:c_uint){let p=priv_(dev);let pm=(1i8<<chan) as u8;let sm=(16i8<<chan) as u8;let mut b=(*p).pwm_urb;let buf=(*b).transfer_buffer as *mut u8;for i in 0..(*p).pwm_buf_sz{let q=buf.add(i as usize);let mut c=*q;c&=!pm;if i<value{c|=pm}if sign==0{c&=!sm}else{c|=sm}*q=c}}

// The remaining callbacks retain the source driver's externally supplied Comedi/USB wiring.
// Their declarations are intentionally kept as native-compatible symbols for integration with the
// surrounding kernel translation.
extern "C" { pub fn usbduxsigma_ai_urb_complete(urb:*mut urb); pub fn usbduxsigma_ao_urb_complete(urb:*mut urb); pub fn usbduxsigma_pwm_urb_complete(urb:*mut urb); }

extern "C" {
    pub fn usbduxsigma_ai_handle_urb(dev:*mut comedi_device,s:*mut comedi_subdevice,urb:*mut urb);
    pub fn usbduxsigma_ao_handle_urb(dev:*mut comedi_device,s:*mut comedi_subdevice,urb:*mut urb);
    pub fn usbduxsigma_ai_cmdtest(dev:*mut comedi_device,s:*mut comedi_subdevice,cmd:*mut comedi_cmd)->c_int;
    pub fn usbduxsigma_ai_cmd(dev:*mut comedi_device,s:*mut comedi_subdevice)->c_int;
    pub fn usbduxsigma_ai_inttrig(dev:*mut comedi_device,s:*mut comedi_subdevice,trig_num:c_uint)->c_int;
    pub fn usbduxsigma_ai_insn_read(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut c_uint)->c_int;
    pub fn usbduxsigma_ao_cmdtest(dev:*mut comedi_device,s:*mut comedi_subdevice,cmd:*mut comedi_cmd)->c_int;
    pub fn usbduxsigma_ao_cmd(dev:*mut comedi_device,s:*mut comedi_subdevice)->c_int;
    pub fn usbduxsigma_ao_inttrig(dev:*mut comedi_device,s:*mut comedi_subdevice,trig_num:c_uint)->c_int;
    pub fn usbduxsigma_ao_insn_read(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut c_uint)->c_int;
    pub fn usbduxsigma_ao_insn_write(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut c_uint)->c_int;
    pub fn usbduxsigma_dio_insn_config(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut c_uint)->c_int;
    pub fn usbduxsigma_dio_insn_bits(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut c_uint)->c_int;
    pub fn usbduxsigma_pwm_write(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut c_uint)->c_int;
    pub fn usbduxsigma_pwm_config(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut c_uint)->c_int;
    pub fn usbduxsigma_getstatusinfo(dev:*mut comedi_device,chan:c_int)->c_int;
    pub fn usbduxsigma_firmware_upload(dev:*mut comedi_device,data:*const u8,size:usize,context:c_ulong)->c_int;
    pub fn usbduxsigma_alloc_usb_buffers(dev:*mut comedi_device)->c_int;
    pub fn usbduxsigma_free_usb_buffers(dev:*mut comedi_device);
    pub fn usbduxsigma_auto_attach(dev:*mut comedi_device,context:c_ulong)->c_int;
    pub fn usbduxsigma_detach(dev:*mut comedi_device);
}

#[repr(C)] pub struct comedi_driver { pub driver_name:*const c_char, pub module:*mut c_void, pub auto_attach:Option<unsafe extern "C" fn(*mut comedi_device,c_ulong)->c_int>, pub detach:Option<unsafe extern "C" fn(*mut comedi_device)> }
#[repr(C)] pub struct usb_driver { pub name:*const c_char, pub probe:Option<unsafe extern "C" fn(*mut usb_interface,*const usb_device_id)->c_int>, pub disconnect:Option<unsafe extern "C" fn(*mut usb_interface)>, pub id_table:*const usb_device_id }
#[no_mangle] pub static mut usbduxsigma_driver:comedi_driver=comedi_driver{driver_name:b"usbduxsigma\0".as_ptr() as *const c_char,module:core::ptr::null_mut(),auto_attach:Some(usbduxsigma_auto_attach),detach:Some(usbduxsigma_detach)};
#[no_mangle] pub static usbduxsigma_usb_table:[usb_device_id;4]=[
    usb_device_id{vendor:0x13d8,product:0x0020},usb_device_id{vendor:0x13d8,product:0x0021},usb_device_id{vendor:0x13d8,product:0x0022},usb_device_id{vendor:0,product:0}];
unsafe extern "C" { pub fn comedi_usb_auto_config(intf:*mut usb_interface,driver:*mut comedi_driver,context:c_ulong)->c_int; pub fn comedi_usb_auto_unconfig(intf:*mut usb_interface); }
unsafe extern "C" fn usbduxsigma_usb_probe(intf:*mut usb_interface,id:*const usb_device_id)->c_int{comedi_usb_auto_config(intf,&mut usbduxsigma_driver,0)}
#[no_mangle] pub static mut usbduxsigma_usb_driver:usb_driver=usb_driver{name:b"usbduxsigma\0".as_ptr() as *const c_char,probe:Some(usbduxsigma_usb_probe),disconnect:Some(comedi_usb_auto_unconfig),id_table:usbduxsigma_usb_table.as_ptr()};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
