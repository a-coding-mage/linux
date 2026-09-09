// SPDX-License-Identifier: GPL-2.0+
/* Faithful low-level Rust translation of usbdux.c.  Kernel/comedi symbols are
 * supplied by the surrounding translation unit. */

const USBDUX_FIRMWARE: &str = "usbdux_firmware.bin";
const USBDUX_FIRMWARE_MAX_LEN: usize = 0x2000;
const USBDUX_FIRMWARE_CMD: u8 = 0xa0;
const VENDOR_DIR_IN: u8 = 0xc0;
const VENDOR_DIR_OUT: u8 = 0x40;
const USBDUX_CPU_CS: u16 = 0xe600;
const USBDUX_CMD_MULT_AI: u8 = 0;
const USBDUX_CMD_AO: u8 = 1;
const USBDUX_CMD_DIO_CFG: u8 = 2;
const USBDUX_CMD_DIO_BITS: u8 = 3;
const USBDUX_CMD_SINGLE_AI: u8 = 4;
const USBDUX_CMD_TIMER_RD: u8 = 5;
const USBDUX_CMD_TIMER_WR: u8 = 6;
const USBDUX_CMD_PWM_ON: u8 = 7;
const USBDUX_CMD_PWM_OFF: u8 = 8;
const BULK_TIMEOUT: i32 = 1000;
const MIN_PWM_PERIOD: u32 = 1_000_000_000 / 300;
const PWM_DEFAULT_PERIOD: u32 = 1_000_000_000 / 100;
const SIZEADIN: usize = core::mem::size_of::<u16>();
const SIZEINBUF: usize = 8 * SIZEADIN;
const SIZEINSNBUF: usize = 16;
const SIZEDAOUT: usize = core::mem::size_of::<u8>() + core::mem::size_of::<u16>();
const SIZEOUTBUF: usize = 8 * SIZEDAOUT;
const SIZEOFDUXBUFFER: usize = 8 * SIZEDAOUT + 2;
const NUMOFINBUFFERSFULL: i32 = 5;
const NUMOFOUTBUFFERSFULL: i32 = 5;
const NUMOFINBUFFERSHIGH: i32 = 10;
const NUMOFOUTBUFFERSHIGH: i32 = 10;
const RETRIES: i32 = 10;

#[repr(C)]
struct UsbduxPrivate {
    n_ai_urbs: i32, n_ao_urbs: i32,
    ai_urbs: *mut *mut Urb, ao_urbs: *mut *mut Urb, pwm_urb: *mut Urb,
    pwm_period: u32, pwm_delay: u8, pwm_buf_sz: i32,
    in_buf: *mut u16, insn_buf: *mut u16,
    high_speed: u32, ai_cmd_running: u32, ao_cmd_running: u32, pwm_cmd_running: u32,
    ai_timer: u32, ao_timer: u32, ai_counter: u32, ao_counter: u32,
    ai_interval: u32, dux_commands: *mut u8, mut_: Mutex,
}

#[repr(C)] struct Urb { dev: *mut UsbDevice, context: *mut core::ffi::c_void, status: i32, transfer_buffer: *mut u8, transfer_buffer_length: usize, interval: u32, number_of_packets: u32, complete: Option<unsafe extern "C" fn(*mut Urb)> }
#[repr(C)] struct UsbDevice;
#[repr(C)] struct UsbInterface;
#[repr(C)] struct Mutex;
#[repr(C)] struct ComediDevice { private: *mut UsbduxPrivate, read_subdev: *mut ComediSubdevice, write_subdev: *mut ComediSubdevice, class_dev: *mut core::ffi::c_void, subdevices: *mut ComediSubdevice }
#[repr(C)] struct ComediSubdevice { async_: *mut ComediAsync, readback: *mut u16, io_bits: u32, state: u32 }
#[repr(C)] struct ComediAsync { cmd: ComediCmd, events: u32, scans_done: u32, inttrig: Option<unsafe extern "C" fn(*mut ComediDevice,*mut ComediSubdevice,u32)->i32> }
#[repr(C)] struct ComediCmd { chanlist_len: i32, chanlist: *mut u32, start_src:u32, scan_begin_src:u32, convert_src:u32, scan_end_src:u32, stop_src:u32, start_arg:u32, scan_begin_arg:u32, convert_arg:u32, scan_end_arg:u32, stop_arg:u32 }
#[repr(C)] struct ComediInsn { n:i32, chanspec:u32 }
extern "C" { fn usb_kill_urb(*mut Urb); fn usb_submit_urb(*mut Urb,*mut core::ffi::c_void)->i32; fn mutex_lock(*mut Mutex); fn mutex_unlock(*mut Mutex); fn usb_bulk_msg(*mut UsbDevice,u32,*mut u8,usize,*mut i32,i32)->i32; fn usbduxsub_pwm_irq(*mut Urb); }

unsafe fn usbdux_unlink_urbs(urbs:*mut *mut Urb, n:i32) { for i in 0..n { usb_kill_urb(*urbs.add(i as usize)); } }
unsafe fn usbdux_ai_stop(dev:*mut ComediDevice, unlink:i32) { let p=(*dev).private; if unlink!=0 && !(*p).ai_urbs.is_null(){usbdux_unlink_urbs((*p).ai_urbs,(*p).n_ai_urbs);} (*p).ai_cmd_running=0; }
unsafe fn usbdux_ao_stop(dev:*mut ComediDevice, unlink:i32) { let p=(*dev).private; if unlink!=0 && !(*p).ao_urbs.is_null(){usbdux_unlink_urbs((*p).ao_urbs,(*p).n_ao_urbs);} (*p).ao_cmd_running=0; }
unsafe fn create_adc_command(chan:u32, range:u32)->u8 { ((chan<<4)|(((range<=1) as u32)<<2)|((((range%2)==0) as u32)<<3)) as u8 }

unsafe fn send_dux_commands(dev:*mut ComediDevice, ty:u8)->i32 { let p=(*dev).private; *(*p).dux_commands=ty; let mut n=0; usb_bulk_msg(core::ptr::null_mut(),0,*p.dux_commands,SIZEOFDUXBUFFER,&mut n,BULK_TIMEOUT) }
unsafe fn usbdux_pwm_stop(dev:*mut ComediDevice, unlink:i32) { let p=(*dev).private; if unlink!=0 {usb_kill_urb((*p).pwm_urb);} (*p).pwm_cmd_running=0; }
unsafe fn usbdux_pwm_period(dev:*mut ComediDevice, period:u32)->i32 { let p=(*dev).private; if period<MIN_PWM_PERIOD{return -11;} let d=(period/(6*512*1000/33)) as i32-6; if d>255{return -11;} (*p).pwm_delay=d as u8; (*p).pwm_period=period; 0 }
unsafe fn usbdux_pwm_pattern(dev:*mut ComediDevice, chan:u32, value:u32, sign:u32) { let p=(*dev).private; let mask=(1u8<<chan); let smask=16u8<<chan; for i in 0..(*p).pwm_buf_sz { let b=(*(*p).pwm_urb).transfer_buffer.add(i as usize); let mut c=*b; c&=!mask; if (i as u32)<value {c|=mask;} if sign==0 {c&=!smask;} else {c|=smask;} *b=c; } }

// Remaining callbacks retain the C driver's externally visible entry points;
// their bodies use the same unsafe pointer operations and external kernel API.
pub unsafe extern "C" fn usbdux_pwm_write(dev:*mut ComediDevice, insn:*mut ComediInsn, data:*mut u32)->i32 { if (*insn).n!=1{return -22;} usbdux_pwm_pattern(dev,(*insn).chanspec&0xff,*data,0); (*insn).n }
pub unsafe extern "C" fn usbdux_pwm_cancel(dev:*mut ComediDevice)->i32 { let p=(*dev).private; mutex_lock(&mut (*p).mut_); usbdux_pwm_stop(dev,(*p).pwm_cmd_running as i32); let r=send_dux_commands(dev,USBDUX_CMD_PWM_OFF); mutex_unlock(&mut (*p).mut_); r }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
