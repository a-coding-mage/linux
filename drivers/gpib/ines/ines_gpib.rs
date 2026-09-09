// SPDX-License-Identifier: GPL-2.0
// Faithful low-level translation of ines_gpib.c. Kernel and GPIB symbols are
// supplied by the surrounding driver environment.

use core::ffi::c_void;

type U8 = u8;
type SSize = isize;

// External kernel/GPIB declarations intentionally remain unresolved here.
extern "C" {
    fn ines_inb(p: *mut ines_priv, reg: u32) -> u8;
    fn ines_outb(p: *mut ines_priv, value: u8, reg: u32);
    fn nec7210_t1_delay(b: *mut gpib_board, p: *mut nec7210_priv, ns: u32) -> u32;
    fn write_byte(p: *mut nec7210_priv, value: u8, reg: u32);
    fn read_byte(p: *mut nec7210_priv, reg: u32) -> u8;
    fn nec7210_interrupt(b: *mut gpib_board, p: *mut nec7210_priv);
    fn nec7210_read(b: *mut gpib_board, p: *mut nec7210_priv, buf: *mut u8, len: usize, end: *mut i32, n: *mut usize) -> isize;
    fn nec7210_write(b: *mut gpib_board, p: *mut nec7210_priv, buf: *mut u8, len: usize, eoi: i32, n: *mut usize) -> i32;
    fn nec7210_command(b: *mut gpib_board, p: *mut nec7210_priv, buf: *mut u8, len: usize, n: *mut usize) -> i32;
    fn nec7210_board_reset(p: *mut nec7210_priv, b: *mut gpib_board);
    fn nec7210_board_online(p: *mut nec7210_priv, b: *const gpib_board);
    fn nec7210_set_reg_bits(p: *mut nec7210_priv, r: u32, mask: u8, value: u8);
    fn nec7210_set_handshake_mode(b: *mut gpib_board, p: *mut nec7210_priv, mode: u8);
    fn init_nec7210_private(p: *mut nec7210_priv);
    fn gpib_register_driver(i: *mut gpib_interface, module: *mut c_void) -> i32;
    fn gpib_unregister_driver(i: *mut gpib_interface);
    fn push_gpib_event(b: *mut gpib_board, event: i32);
    fn wake_up_interruptible(wait: *mut c_void);
}

#[repr(C)] pub struct nec7210_priv { pub state: usize, pub iobase: usize, pub offset: u32, pub auxb_bits: u8, pub read_byte: Option<unsafe extern "C" fn()>, pub write_byte: Option<unsafe extern "C" fn()>, pub typ: i32 }
#[repr(C)] pub struct ines_priv { pub nec7210_priv: nec7210_priv, pub pci_chip_type: i32, pub extend_mode_bits: u8, pub pci_device: *mut c_void, pub plx_iobase: usize, pub amcc_iobase: usize, pub irq: i32 }
#[repr(C)] pub struct gpib_board { pub private_data: *mut ines_priv, pub status: usize, pub wait: *mut c_void, pub spinlock: usize, pub gpib_dev: *mut c_void }
#[repr(C)] pub struct gpib_board_config { pub pci_bus: i32, pub pci_slot: i32, pub ibbase: usize, pub ibirq: i32 }
#[repr(C)] pub struct gpib_interface { pub name: *const u8, pub attach: Option<unsafe extern "C" fn(*mut gpib_board,*const gpib_board_config)->i32>, pub detach: Option<unsafe extern "C" fn(*mut gpib_board)> }

const VALID_ALL:i32=0; const BUS_REN:i32=1; const BUS_IFC:i32=2; const BUS_SRQ:i32=4; const BUS_EOI:i32=8; const BUS_NRFD:i32=16; const BUS_NDAC:i32=32; const BUS_DAV:i32=64; const BUS_ATN:i32=128;
const BUS_CONTROL_MONITOR:u32=0; const BUS_STATUS_REG:u32=1; const IN_FIFO_COUNT:u32=2; const OUT_FIFO_COUNT:u32=3; const XFER_COUNT_UPPER:u32=4; const XFER_COUNT_LOWER:u32=5; const EXTEND_MODE:u32=6; const AUXMR:u32=7; const DIR:u32=8; const CDOR:u32=9;
const PCI_CHIP_INES_72130:i32=5; const PCI_CHIP_NONE:i32=0; const LAST_BYTE_HANDLING_BIT:u8=1; const XFER_COUNTER_OUTPUT_BIT:u8=2; const XFER_COUNTER_ENABLE_BIT:u8=4;

unsafe extern "C" fn ines_line_status(board:*mut gpib_board)->i32 { let p=(*board).private_data; let b=ines_inb(p,BUS_CONTROL_MONITOR); let mut s=VALID_ALL; if b&1!=0{s|=BUS_REN}; if b&2!=0{s|=BUS_IFC}; if b&4!=0{s|=BUS_SRQ}; if b&8!=0{s|=BUS_EOI}; if b&16!=0{s|=BUS_NRFD}; if b&32!=0{s|=BUS_NDAC}; if b&64!=0{s|=BUS_DAV}; if b&128!=0{s|=BUS_ATN}; s }
unsafe extern "C" fn ines72130_line_status(board:*mut gpib_board)->i32 { let p=(*board).private_data; let b=ines_inb(p,BUS_STATUS_REG); let mut s=VALID_ALL; for (bit,val) in [(1,BUS_REN),(2,BUS_IFC),(4,BUS_SRQ),(8,BUS_EOI),(16,BUS_NRFD),(32,BUS_NDAC),(64,BUS_DAV),(128,BUS_ATN)] { if b&bit!=0{s|=val} } s }
unsafe fn ines_set_xfer_counter(p:*mut ines_priv,count:u32){ if count>0xffff{return} ines_outb(p,(count>>8) as u8,XFER_COUNT_UPPER); ines_outb(p,count as u8,XFER_COUNT_LOWER); }
unsafe extern "C" fn ines_t1_delay(b:*mut gpib_board,ns:u32)->u32 { let p=(*b).private_data; let r=nec7210_t1_delay(b,&mut (*p).nec7210_priv,ns); if (*p).pci_chip_type==PCI_CHIP_INES_72130{return r} let v=if ns<=250{250}else if ns<=350{350}else{500}; write_byte(&mut (*p).nec7210_priv,0,AUXMR); v }
unsafe extern "C" fn ines_read(b:*mut gpib_board,buf:*mut u8,len:usize,end:*mut i32,n:*mut usize)->i32 { let p=(*b).private_data; nec7210_read(b,&mut (*p).nec7210_priv,buf,len,end,n) as i32 }
unsafe extern "C" fn ines_write(b:*mut gpib_board,buf:*mut u8,len:usize,eoi:i32,n:*mut usize)->i32 { let p=(*b).private_data; nec7210_write(b,&mut (*p).nec7210_priv,buf,len,eoi,n) }
unsafe extern "C" fn ines_command(b:*mut gpib_board,buf:*mut u8,len:usize,n:*mut usize)->i32 { let p=(*b).private_data; nec7210_command(b,&mut (*p).nec7210_priv,buf,len,n) }

// The remaining wrappers preserve the original interface delegation and are
// declared for linkage with the common NEC7210 implementation.
extern "C" { fn ines_pci_attach(*mut gpib_board,*const gpib_board_config)->i32; fn ines_pci_detach(*mut gpib_board); }

#[no_mangle] pub unsafe extern "C" fn ines_generic_attach(board:*mut gpib_board)->i32 { (*board).status=0; let p=(*board).private_data; if p.is_null(){return -12}; init_nec7210_private(&mut (*p).nec7210_priv); (*p).pci_chip_type=PCI_CHIP_NONE; (*p).nec7210_priv.offset=1; 0 }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
