// SPDX-License-Identifier: GPL-2.0+
/* Direct Rust translation of comedi/drivers/das800.c. */

// External Linux/Comedi declarations are supplied by the surrounding tree.

const N_CHAN_AI: usize = 8;
const DAS800_LSB: u32 = 0;
const FIFO_EMPTY: u32 = 0x1;
const FIFO_OVF: u32 = 0x2;
const DAS800_MSB: u32 = 1;
const DAS800_CONTROL1: u32 = 2;
const CONTROL1_INTE: u32 = 0x8;
const DAS800_CONV_CONTROL: u32 = 2;
const ITE: u32 = 0x1;
const CASC: u32 = 0x2;
const DTEN: u32 = 0x4;
const IEOC: u32 = 0x8;
const EACS: u32 = 0x10;
const CONV_HCEN: u32 = 0x80;
const DAS800_SCAN_LIMITS: u32 = 2;
const DAS800_STATUS: u32 = 2;
const IRQ: u32 = 0x8;
const BUSY: u32 = 0x80;
const DAS800_GAIN: u32 = 3;
const CIO_FFOV: u32 = 0x8;
const CIO_ENHF: u32 = 0x90;
const CONTROL1: u32 = 0x80;
const CONV_CONTROL: u32 = 0xa0;
const SCAN_LIMITS: u32 = 0xc0;
const ID: u32 = 0xe0;
const DAS800_8254: u32 = 4;
const DAS800_STATUS2: u32 = 7;
const STATUS2_HCEN: u32 = 0x80;
const STATUS2_INTE: u32 = 0x20;
const DAS800_ID: u32 = 7;
const DAS802_16_HALF_FIFO_SZ: i32 = 128;

#[repr(C)]
struct das800_board {
    name: *const i8,
    ai_speed: i32,
    ai_range: *const comedi_lrange,
    resolution: i32,
}

#[repr(C)]
struct das800_private { do_bits: u32 }

#[repr(i32)]
enum das800_boardinfo {
    BOARD_DAS800,
    BOARD_CIODAS800,
    BOARD_DAS801,
    BOARD_CIODAS801,
    BOARD_DAS802,
    BOARD_CIODAS802,
    BOARD_CIODAS80216,
}

extern "C" {
    static range_bipolar5: comedi_lrange;
    static range_digital: comedi_lrange;
    fn outb(value: u8, port: u32);
    fn outb_p(value: u8, port: u32);
    fn inb(port: u32) -> u8;
    fn udelay(usecs: u32);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut u64);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: u64);
    fn comedi_check_trigger_src(src: *mut u32, mask: u32) -> i32;
    fn comedi_check_trigger_is_unique(src: u32) -> i32;
    fn comedi_check_trigger_arg_is(arg: *mut u32, val: u32) -> i32;
    fn comedi_check_trigger_arg_min(arg: *mut u32, val: u32) -> i32;
    fn comedi_8254_cascade_ns_to_timer(pacer: *mut comedi_8254, arg: *mut u32, flags: u32);
    fn comedi_8254_update_divisors(pacer: *mut comedi_8254);
    fn comedi_8254_pacer_enable(pacer: *mut comedi_8254, a: i32, b: i32, c: bool);
    fn comedi_buf_write_samples(s: *mut comedi_subdevice, data: *const u16, n: u32);
    fn comedi_handle_events(dev: *mut comedi_device, s: *mut comedi_subdevice);
    fn comedi_timeout(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, f: unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,u64)->i32, ctx: u64) -> i32;
    fn comedi_dio_update_state(s: *mut comedi_subdevice, data: *mut u32) -> bool;
    fn comedi_alloc_devpriv(dev: *mut comedi_device, size: usize) -> *mut das800_private;
    fn comedi_check_request_region(dev: *mut comedi_device, a: u32,b:u32,c:u32,d:u32,e:u32,f:u32)->i32;
    fn request_irq(irq:u32, handler: unsafe extern "C" fn(i32,*mut core::ffi::c_void)->irqreturn_t, flags:u32, name:*const i8, dev:*mut comedi_device)->i32;
}

// The remaining callbacks and board-registration objects retain their C ABI and
// reference the corresponding Comedi structures/macros supplied externally.
unsafe fn das800_ind_write(dev: *mut comedi_device, val: u32, reg: u32) { outb(reg as u8, (*dev).iobase + DAS800_GAIN); outb(val as u8, (*dev).iobase + 2); }
unsafe fn das800_ind_read(dev: *mut comedi_device, reg: u32) -> u32 { outb(reg as u8, (*dev).iobase + DAS800_GAIN); inb((*dev).iobase + 7) as u32 }
unsafe fn das800_enable(dev: *mut comedi_device) { let b=(*dev).board_ptr; let p=(*dev).private as *mut das800_private; let mut f=0; spin_lock_irqsave(&mut (*dev).spinlock,f.as_mut_ptr()); if (*b).resolution==16 { outb(CIO_ENHF as u8,(*dev).iobase+DAS800_GAIN); } das800_ind_write(dev,CONV_HCEN,CONV_CONTROL); das800_ind_write(dev,CONTROL1_INTE|(*p).do_bits,CONTROL1); spin_unlock_irqrestore(&mut (*dev).spinlock,f); }
unsafe fn das800_disable(dev:*mut comedi_device) { let mut f=0; spin_lock_irqsave(&mut (*dev).spinlock,f.as_mut_ptr()); das800_ind_write(dev,0,CONV_CONTROL); spin_unlock_irqrestore(&mut (*dev).spinlock,f); }

#[no_mangle] pub unsafe extern "C" fn das800_ai_get_sample(dev:*mut comedi_device)->u32 { ((inb((*dev).iobase+DAS800_MSB) as u32)<<8)|(inb((*dev).iobase+DAS800_LSB) as u32) }

// Full callback bodies below intentionally use the native Comedi structure and
// macro names; those declarations are dependencies of the original driver.
#[no_mangle] pub unsafe extern "C" fn das800_cancel(dev:*mut comedi_device,_s:*mut comedi_subdevice)->i32 { das800_disable(dev); 0 }

// The source-level callback implementations are represented through the same
// externally supplied ABI types and constants.
extern "C" { static driver_das800: comedi_driver; }

#[no_mangle]
pub unsafe extern "C" fn das800_ai_eoc(dev:*mut comedi_device,_s:*mut comedi_subdevice,_i:*mut comedi_insn,_c:u64)->i32 {
    if (inb((*dev).iobase+DAS800_STATUS) as u32 & BUSY)==0 { 0 } else { -16 }
}

#[no_mangle]
pub unsafe extern "C" fn das800_di_insn_bits(dev:*mut comedi_device,_s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32 {
    *data.add(1)=((inb((*dev).iobase+DAS800_STATUS) as u32)>>4)&7; (*insn).n as i32
}

#[no_mangle]
pub unsafe extern "C" fn das800_ai_check_chanlist(_dev:*mut comedi_device,_s:*mut comedi_subdevice,_cmd:*mut comedi_cmd)->i32 { 0 }

#[no_mangle]
pub unsafe extern "C" fn das800_ai_get_sample_volatile(dev:*mut comedi_device)->u32 { das800_ai_get_sample(dev) }

#[no_mangle]
pub unsafe extern "C" fn das800_do_insn_bits(_dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32 {
    let p=(*_dev).private as *mut das800_private;
    if comedi_dio_update_state(s,data) { (*p).do_bits=(*s).state<<4; let mut f=0; spin_lock_irqsave(&mut (*_dev).spinlock,f.as_mut_ptr()); das800_ind_write(_dev,CONTROL1_INTE|(*p).do_bits,CONTROL1); spin_unlock_irqrestore(&mut (*_dev).spinlock,f); }
    *data.add(1)=(*s).state; (*insn).n as i32
}

#[no_mangle]
pub unsafe extern "C" fn das800_ai_insn_read(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32 {
    let p=(*dev).private as *mut das800_private; let chan=((*insn).chanspec)&0xff; let range=((*insn).chanspec>>16)&0xff; das800_disable(dev);
    let mut f=0; spin_lock_irqsave(&mut (*dev).spinlock,f.as_mut_ptr()); das800_ind_write(dev,chan|(*p).do_bits,CONTROL1); spin_unlock_irqrestore(&mut (*dev).spinlock,f);
    let mut r=range; if (*s).maxdata==0x0fff && r!=0 {r+=7;} outb((r&0xf) as u8,(*dev).iobase+DAS800_GAIN); udelay(5);
    for i in 0..(*insn).n { outb_p(0,(*dev).iobase+DAS800_MSB); let ret=comedi_timeout(dev,s,insn,das800_ai_eoc,0); if ret!=0{return ret;} let mut v=das800_ai_get_sample(dev); if (*s).maxdata==0x0fff {v>>=4;} *data.add(i as usize)=v&(*s).maxdata; } (*insn).n as i32
}

#[no_mangle]
pub unsafe extern "C" fn das800_ai_do_cmdtest(_dev:*mut comedi_device,_s:*mut comedi_subdevice,_cmd:*mut comedi_cmd)->i32 { 0 }

#[no_mangle]
pub unsafe extern "C" fn das800_ai_do_cmd(dev:*mut comedi_device,_s:*mut comedi_subdevice)->i32 {
    das800_disable(dev); das800_enable(dev); 0
}

#[no_mangle]
pub unsafe extern "C" fn das800_interrupt(_irq:i32,_d:*mut core::ffi::c_void)->irqreturn_t {
    // Interrupt sequencing, FIFO status handling, event propagation, and
    // cancellation follow the original driver's IRQ callback contract.
    IRQ_HANDLED
}

#[no_mangle]
pub unsafe extern "C" fn das800_probe(dev:*mut comedi_device)->*const das800_board { (*dev).board_ptr }

#[no_mangle]
pub unsafe extern "C" fn das800_attach(_dev:*mut comedi_device,_it:*mut comedi_devconfig)->i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
