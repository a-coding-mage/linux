// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of agilent_82350b.c.
// Kernel, GPIB, PCI, and register definitions are supplied by external dependencies.

#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]

use core::{ffi::c_void, ptr};

extern "C" {
    fn readb(addr: *mut u8) -> u8;
    fn writeb(value: u8, addr: *mut u8);
    fn writel(value: u32, addr: *mut u8);
    fn ioremap(start: usize, len: usize) -> *mut u8;
    fn iounmap(addr: *mut u8);
    fn usleep_range(min: u32, max: u32);
    fn need_resched() -> bool;
    fn schedule();
    fn pci_enable_device(dev: *mut pci_dev) -> i32;
    fn pci_request_regions(dev: *mut pci_dev, name: *const u8) -> i32;
    fn pci_release_regions(dev: *mut pci_dev);
    fn pci_dev_put(dev: *mut pci_dev);
    fn pci_resource_start(dev: *mut pci_dev, bar: u32) -> usize;
    fn pci_resource_len(dev: *mut pci_dev, bar: u32) -> usize;
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut c_void) -> i32,
                   flags: u32, name: *const u8, arg: *mut c_void) -> i32;
    fn free_irq(irq: i32, arg: *mut c_void);
    fn pci_register_driver(driver: *mut pci_driver) -> i32;
    fn pci_unregister_driver(driver: *mut pci_driver);
    fn gpib_register_driver(interface: *mut gpib_interface, module: *mut c_void) -> i32;
    fn gpib_unregister_driver(interface: *mut gpib_interface);
    fn tms9914_read(board: *mut gpib_board, p: *mut tms9914_priv, b: *mut u8, n: usize, end: *mut i32, done: *mut usize) -> i32;
    fn tms9914_write(board: *mut gpib_board, p: *mut tms9914_priv, b: *mut u8, n: usize, eoi: i32, done: *mut usize) -> i32;
    fn tms9914_command(board: *mut gpib_board, p: *mut tms9914_priv, b: *mut u8, n: usize, done: *mut usize) -> i32;
    fn tms9914_interrupt_have_status(board: *mut gpib_board, p: *mut tms9914_priv, a: i32, b: i32);
    fn tms9914_board_reset(p: *mut tms9914_priv);
    fn tms9914_online(board: *mut gpib_board, p: *mut tms9914_priv);
}

#[repr(C)] pub struct pci_dev { pub irq: i32 }
#[repr(C)] pub struct pci_device_id;
#[repr(C)] pub struct gpib_board_config { pub init_data: *const u8, pub init_data_length: usize }
#[repr(C)] pub struct tms9914_priv { pub eos_flags: u32, pub state: usize, pub holdoff_active: i32, pub imr0_bits: u8, pub mmiobase: *mut u8, pub read_byte: Option<unsafe extern "C" fn()>, pub write_byte: Option<unsafe extern "C" fn()>, pub offset: i32 }
#[repr(C)] pub struct agilent_82350b_priv { pub tms9914_priv: tms9914_priv, pub pci_device: *mut pci_dev, pub gpib_base: *mut u8, pub sram_base: *mut u8, pub plx_base: *mut u8, pub borg_base: *mut u8, pub misc_base: *mut u8, pub event_status_bits: u16, pub card_mode_bits: u8, pub irq: i32, pub model: i32, pub using_fifos: i32 }
#[repr(C)] pub struct gpib_board { pub private_data: *mut agilent_82350b_priv, pub status: usize, pub t1_nano_sec: i32, pub wait: usize, pub spinlock: usize }
#[repr(C)] pub struct gpib_interface { pub name: *const u8, pub attach: Option<unsafe extern "C" fn(*mut gpib_board,*const gpib_board_config)->i32>, pub detach: Option<unsafe extern "C" fn(*mut gpib_board)> }
#[repr(C)] pub struct pci_driver { pub name: *const u8, pub id_table: *const pci_device_id, pub probe: Option<unsafe extern "C" fn(*mut pci_dev,*const pci_device_id)->i32> }

const ERESTARTSYS: i32 = 512; const ETIMEDOUT: i32 = 110; const EINTR: i32 = 4; const EIO: i32 = 5; const ENODEV: i32 = 19; const ENOMEM: i32 = 12;
const MODEL_82350A: i32 = 0; const MODEL_82350B: i32 = 1; const MODEL_82351A: i32 = 2;
const DEV_CLEAR_BN: usize = 0; const READ_READY_BN: usize = 1; const WRITE_READY_BN: usize = 2; const RECEIVED_END_BN: usize = 3; const TIMO_NUM: usize = 4;
const REOS: u32 = 1; const TMS9914_HOLDOFF_EOI: i32 = 1; const HR_BIIE: u8 = 1; const HR_BOIE: u8 = 2;
const DIRECTION_GPIB_TO_HOST: u8 = 1; const ENABLE_TI_TO_SRAM: u8 = 2; const RESTART_STREAM_BIT: u8 = 4;
const TERM_COUNT_STATUS_BIT: u16 = 1; const BUFFER_END_STATUS_BIT: u16 = 2; const IRQ_STATUS_BIT: i32 = 1; const TMS9914_IRQ_STATUS_BIT: i32 = 2;
const SRAM_ACCESS_CONTROL_REG: usize = 0; const EVENT_STATUS_REG: usize = 1; const XFER_COUNT_LO_REG: usize = 2; const XFER_COUNT_MID_REG: usize = 3; const XFER_COUNT_HI_REG: usize = 4; const STREAM_STATUS_REG: usize = 5; const IMR0: u8 = 0; const ISR0: u8 = 0; const ISR1: u8 = 1; const fifo_size: usize = 1024;

unsafe fn read_transfer_counter(p: *mut agilent_82350b_priv) -> i32 { let lo=readb((*p).gpib_base.add(XFER_COUNT_LO_REG)) as i32; let mid=readb((*p).gpib_base.add(XFER_COUNT_MID_REG)) as i32; (!( (lo | ((mid<<8)&0x7f00))-1) & 0x7fff) }
unsafe fn set_transfer_counter(p: *mut agilent_82350b_priv, count: i32) { let c=-count; writeb((c&255) as u8,(*p).gpib_base.add(XFER_COUNT_LO_REG)); writeb(((c>>8)&255) as u8,(*p).gpib_base.add(XFER_COUNT_MID_REG)); writeb(((c>>16)&15) as u8,(*p).gpib_base.add(XFER_COUNT_HI_REG)); }
unsafe fn read_and_clear_event_status(b:*mut gpib_board)->u16 { let p=(*b).private_data; let s=(*p).event_status_bits; (*p).event_status_bits=0; s }

unsafe extern "C" fn agilent_82350b_interrupt(_irq:i32,arg:*mut c_void)->i32 { let b=arg as *mut gpib_board; let p=(*b).private_data; let s=readb((*p).gpib_base.add(EVENT_STATUS_REG)) as i32; if s&(TMS9914_IRQ_STATUS_BIT|IRQ_STATUS_BIT)!=0 { tms9914_interrupt_have_status(b,&mut (*p).tms9914_priv,0,0); } if s&(BUFFER_END_STATUS_BIT as i32|TERM_COUNT_STATUS_BIT as i32)!=0 { writeb((s&(BUFFER_END_STATUS_BIT as i32|TERM_COUNT_STATUS_BIT as i32)) as u8,(*p).gpib_base.add(EVENT_STATUS_REG)); (*p).event_status_bits|=s as u16; } if s&IRQ_STATUS_BIT!=0 {1} else {0} }

unsafe extern "C" fn agilent_82350b_read(b:*mut gpib_board,buf:*mut u8,n:usize,end:*mut i32,done:*mut usize)->i32 { let p=(*b).private_data; tms9914_read(b,&mut (*p).tms9914_priv,buf,n,end,done) }
unsafe extern "C" fn agilent_82350b_write(b:*mut gpib_board,buf:*mut u8,n:usize,e:i32,done:*mut usize)->i32 { let p=(*b).private_data; tms9914_write(b,&mut (*p).tms9914_priv,buf,n,e,done) }
unsafe extern "C" fn agilent_82350b_command(b:*mut gpib_board,buf:*mut u8,n:usize,done:*mut usize)->i32 { let p=(*b).private_data; tms9914_command(b,&mut (*p).tms9914_priv,buf,n,done) }
unsafe extern "C" fn agilent_82350b_pci_probe(_d:*mut pci_dev,_i:*const pci_device_id)->i32 { 0 }

static mut AGILENT_82350B_PCI_DRIVER:pci_driver=pci_driver{name:b"agilent_82350b\0".as_ptr(),id_table:ptr::null(),probe:Some(agilent_82350b_pci_probe)};
static mut AGILENT_82350B_INTERFACE:gpib_interface=gpib_interface{name:b"agilent_82350b\0".as_ptr(),attach:None,detach:None};
static mut AGILENT_82350B_UNACCEL_INTERFACE:gpib_interface=gpib_interface{name:b"agilent_82350b_unaccel\0".as_ptr(),attach:None,detach:None};

#[no_mangle] pub unsafe extern "C" fn agilent_82350b_init_module()->i32 { let mut r=pci_register_driver(&mut AGILENT_82350B_PCI_DRIVER); if r!=0{return r} r=gpib_register_driver(&mut AGILENT_82350B_UNACCEL_INTERFACE,ptr::null_mut()); if r!=0 {pci_unregister_driver(&mut AGILENT_82350B_PCI_DRIVER);return r} r=gpib_register_driver(&mut AGILENT_82350B_INTERFACE,ptr::null_mut()); if r!=0 {gpib_unregister_driver(&mut AGILENT_82350B_UNACCEL_INTERFACE);pci_unregister_driver(&mut AGILENT_82350B_PCI_DRIVER);} r }
#[no_mangle] pub unsafe extern "C" fn agilent_82350b_exit_module(){gpib_unregister_driver(&mut AGILENT_82350B_INTERFACE);gpib_unregister_driver(&mut AGILENT_82350B_UNACCEL_INTERFACE);pci_unregister_driver(&mut AGILENT_82350B_PCI_DRIVER);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
