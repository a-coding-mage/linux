// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of tnt4882_gpib.c.
// Kernel/GPIB symbols below are supplied by the surrounding translated tree.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_int, c_void};

#[repr(C)] pub struct nec7210_priv { pub mmiobase: *mut c_void, pub iobase: u32, pub offset: c_int, pub r#type: c_int, pub auxa_bits: u16, pub state: c_ulong, pub srq_pending: c_int, pub register_page_lock: c_void }
#[repr(C)] pub struct mite_struct { pub next: *mut mite_struct, pub used: c_int, pub daq_io_addr: *mut c_void, pub pcidev: *mut c_void }
#[repr(C)] pub struct gpib_board { pub private_data: *mut tnt4882_priv, pub status: c_ulong, pub spinlock: c_void, pub wait: c_void, pub minor: c_int, pub gpib_dev: *mut c_void }
#[repr(C)] pub struct gpib_board_config { pub pci_bus: c_int, pub pci_slot: c_int, pub ibbase: u32, pub ibirq: c_int }
#[repr(C)] pub struct gpib_interface { pub name: *const u8, pub attach: Option<unsafe extern "C" fn(*mut gpib_board,*const gpib_board_config)->c_int>, pub detach: Option<unsafe extern "C" fn(*mut gpib_board)>, pub read: Option<unsafe extern "C" fn(*mut gpib_board,*mut u8,usize,*mut c_int,*mut usize)->c_int>, pub write: Option<unsafe extern "C" fn(*mut gpib_board,*mut u8,usize,c_int,*mut usize)->c_int>, pub command: Option<unsafe extern "C" fn(*mut gpib_board,*mut u8,usize,*mut usize)->c_int> }
type c_ulong = usize;
#[repr(C)] pub struct pnp_dev { pub card: *mut c_void }
#[repr(C)] pub struct pci_dev;
#[repr(C)] pub struct pci_device_id;

extern "C" {
    static mut mite_devices: *mut mite_struct;
    fn ioread8(p:*mut c_void)->u8; fn ioread16(p:*mut c_void)->u16; fn iowrite8(v:u8,p:*mut c_void); fn iowrite16(v:u16,p:*mut c_void); fn udelay(v:u32);
    fn spin_lock_irqsave(l:*mut c_void,f:*mut c_ulong); fn spin_unlock_irqrestore(l:*mut c_void,f:c_ulong);
    fn nec7210_t1_delay(*mut gpib_board,*mut nec7210_priv,u32)->u32; fn nec7210_set_handshake_mode(*mut gpib_board,*mut nec7210_priv,c_int); fn write_byte(*mut nec7210_priv,u16,u32)->u8; fn read_byte(*mut nec7210_priv,u32)->u8;
    fn nec7210_set_reg_bits(*mut nec7210_priv,u32,u32,u32); fn nec7210_interrupt(*mut gpib_board,*mut nec7210_priv); fn wake_up_interruptible(*mut c_void); fn push_gpib_event(*mut gpib_board,c_int);
    fn nec7210_read(*mut gpib_board,*mut nec7210_priv,*mut u8,usize,*mut c_int,*mut usize)->c_int; fn nec7210_write(*mut gpib_board,*mut nec7210_priv,*mut u8,usize,c_int,*mut usize)->c_int; fn nec7210_command(*mut gpib_board,*mut nec7210_priv,*mut u8,usize,*mut usize)->c_int;
    fn nec7210_read_data_in(*mut gpib_board,*mut nec7210_priv,*mut c_int); fn nec7210_take_control(*mut gpib_board,*mut nec7210_priv,c_int)->c_int; fn nec7210_go_to_standby(*mut gpib_board,*mut nec7210_priv)->c_int; fn nec7210_request_system_control(*mut gpib_board,*mut nec7210_priv,c_int)->c_int; fn nec7210_interface_clear(*mut gpib_board,*mut nec7210_priv,c_int); fn nec7210_remote_enable(*mut gpib_board,*mut nec7210_priv,c_int); fn nec7210_enable_eos(*mut gpib_board,*mut nec7210_priv,u8,c_int)->c_int; fn nec7210_disable_eos(*mut gpib_board,*mut nec7210_priv); fn nec7210_update_status_nolock(*mut gpib_board,*mut nec7210_priv); fn nec7210_primary_address(*mut gpib_board,*mut nec7210_priv,u32)->c_int; fn nec7210_secondary_address(*mut gpib_board,*mut nec7210_priv,u32,c_int)->c_int; fn nec7210_parallel_poll(*mut gpib_board,*mut nec7210_priv,*mut u8)->c_int; fn nec7210_parallel_poll_configure(*mut gpib_board,*mut nec7210_priv,u8); fn nec7210_parallel_poll_response(*mut gpib_board,*mut nec7210_priv,c_int); fn nec7210_serial_poll_response(*mut gpib_board,*mut nec7210_priv,u8); fn nec7210_serial_poll_status(*mut gpib_board,*mut nec7210_priv)->u8; fn nec7210_return_to_local(*mut gpib_board,*mut nec7210_priv); fn nec7210_board_reset(*mut nec7210_priv,*mut gpib_board); fn nec7210_board_online(*mut nec7210_priv,*mut gpib_board); fn init_nec7210_private(*mut nec7210_priv);
}

const TNT4882:c_int=1; const TNT5004:c_int=2; const NAT4882:c_int=3; const NEC7210:c_int=4;
const CSR:u32=0; const SASR:u32=1; const ISR0:u32=2; const BSR:u32=3; const KEYREG:u32=4; const IMR0:u32=5; const BCR:u32=6; const STS2:u32=7; const STS1:u32=8; const FIFOB:u32=9; const IMR1:u32=10; const IMR2:u32=11; const CCR:u32=12; const CFG:u32=13; const CMDR:u32=14; const CNT0:u32=15; const CNT1:u32=16; const CNT2:u32=17; const CNT3:u32=18; const IMR3:u32=19; const ISR3:u32=20; const HSSEL:u32=21; const ACCWR:u32=22; const AUXCR:u32=23; const SWAPPED_AUXCR:u32=24; const INTRT:u32=25; const AUXMR:u32=26; const SPMR:u32=27; const CPTR:u32=28;
const AUX_PAGEIN:u8=0; const tnt_pagein_offset:u32=0; const MSTD:u16=0; const USTD:u16=0; const AUXRI:u16=0; const AUX_FH:u16=0; const AUX_HLDI:u16=0; const AUX_7210:u16=0; const AUXRG:u16=0; const AUX_REQT:u16=0; const AUX_REQF:u16=0; const PPR:u16=0;
const VALID_ALL:c_int=0; const BUS_REN:c_int=1; const BUS_IFC:c_int=2; const BUS_SRQ:c_int=4; const BUS_EOI:c_int=8; const BUS_NRFD:c_int=16; const BUS_NDAC:c_int=32; const BUS_DAV:c_int=64; const BUS_ATN:c_int=128; const BCSR_REN_BIT:i32=1; const BCSR_IFC_BIT:i32=2; const BCSR_SRQ_BIT:i32=4; const BCSR_EOI_BIT:i32=8; const BCSR_NRFD_BIT:i32=16; const BCSR_NDAC_BIT:i32=32; const BCSR_DAV_BIT:i32=64; const BCSR_ATN_BIT:i32=128;
const AEFN:i32=1; const BEFN:i32=2; const AFFN:i32=4; const BFFN:i32=8; const S_DONE:i32=1; const S_HALT:i32=2; const HR_ENDIE:u32=1; const HR_DMAI:u32=2; const HR_DMAO:u32=4; const HR_ERRIE:u32=8; const HR_ERR:u32=16; const HR_HLDA:u16=1; const HR_HLDE:u16=2; const TNT_B_16BIT:u32=1; const TNT_IN:u32=2; const TNT_CCEN:u32=4; const TNT_COMMAND:u32=8; const RESET_FIFO:u16=1; const GO:u16=2; const STOP:u16=4; const HR_DONE:u16=1; const HR_NEF:u16=2; const HR_NFF:u16=4; const HR_INTR:u16=8; const HR_TLCI:u16=16; const TNT_IFCI_BIT:i32=1; const TNT_IMR0_ALWAYS_BITS:u16=0; const TNT_ATNI_BIT:u16=0; const TNT_IFCIE_BIT:u16=0; const TNT_ONE_CHIP_BIT:u16=0; const NODMA:u16=0; const SOFT_RESET:u16=0; const RPP2_BIT:u16=0; const NTNL_BIT:u16=0; const request_service_bit:u8=0x40;

#[repr(C)] pub struct tnt4882_priv { pub nec7210_priv: nec7210_priv, pub mite:*mut mite_struct, pub pnp_dev:*mut pnp_dev, pub irq:u32, pub imr0_bits:u16, pub imr3_bits:u16, pub auxg_bits:u16 }

#[inline] unsafe fn tnt_paged_readb(p:*mut tnt4882_priv,o:usize)->u16 { iowrite8(AUX_PAGEIN,(*p).nec7210_priv.mmiobase.add((AUXMR*(*p).nec7210_priv.offset as u32) as usize) as *mut c_void); udelay(1); ioread8((*p).nec7210_priv.mmiobase.add(o) as *mut c_void) as u16 }
#[inline] unsafe fn tnt_paged_writeb(p:*mut tnt4882_priv,v:u16,o:usize){iowrite8(AUX_PAGEIN,(*p).nec7210_priv.mmiobase.add((AUXMR*(*p).nec7210_priv.offset as u32) as usize) as *mut c_void);udelay(1);iowrite8(v as u8,(*p).nec7210_priv.mmiobase.add(o) as *mut c_void);}
#[inline] unsafe fn tnt_readb(p:*mut tnt4882_priv,o:u32)->u16 { let a=(*p).nec7210_priv.mmiobase.add(o as usize) as *mut c_void; match o { CSR|SASR|ISR0|BSR => match (*p).nec7210_priv.r#type { TNT4882|TNT5004=>ioread8(a) as u16,NAT4882=>tnt_paged_readb(p,(o-tnt_pagein_offset) as usize),_=>0 }, _=>ioread8(a) as u16 } }
#[inline] unsafe fn tnt_writeb(p:*mut tnt4882_priv,v:u16,o:u32){let a=(*p).nec7210_priv.mmiobase.add(o as usize) as *mut c_void;match o{KEYREG|IMR0|BCR=>match (*p).nec7210_priv.r#type{TNT4882|TNT5004=>iowrite8(v as u8,a),NAT4882=>tnt_paged_writeb(p,v,(o-tnt_pagein_offset) as usize),_=>{}},_=>iowrite8(v as u8,a)}}

unsafe fn fifo_word_available(p:*mut tnt4882_priv)->bool{let s=tnt_readb(p,STS2) as i32;(s&AEFN)!=0&&(s&BEFN)!=0}
unsafe fn fifo_byte_available(p:*mut tnt4882_priv)->bool{let s=tnt_readb(p,STS2) as i32;(s&AEFN)!=0||(s&BEFN)!=0}
unsafe fn fifo_xfer_done(p:*mut tnt4882_priv)->bool{(tnt_readb(p,STS1) as i32&(S_DONE|S_HALT))!=0}
unsafe fn drain_fifo_words(p:*mut tnt4882_priv,b:*mut u8,n:usize)->usize{let mut c=0;while fifo_word_available(p)&&c+2<=n{let w=ioread16((*p).nec7210_priv.mmiobase.add(FIFOB as usize) as *mut c_void);*b.add(c)=w as u8;*b.add(c+1)=(w>>8) as u8;c+=2;}c}

unsafe fn tnt4882_line_status(b:*const gpib_board)->c_int{let p=(*b).private_data;let s=tnt_readb(p,BSR) as i32;let mut r=VALID_ALL;if s&(BCSR_REN_BIT)!=0{r|=BUS_REN}if s&BCSR_IFC_BIT!=0{r|=BUS_IFC}if s&BCSR_SRQ_BIT!=0{r|=BUS_SRQ}if s&BCSR_EOI_BIT!=0{r|=BUS_EOI}if s&BCSR_NRFD_BIT!=0{r|=BUS_NRFD}if s&BCSR_NDAC_BIT!=0{r|=BUS_NDAC}if s&BCSR_DAV_BIT!=0{r|=BUS_DAV}if s&BCSR_ATN_BIT!=0{r|=BUS_ATN}r}
unsafe fn fifo_space_available(p:*mut tnt4882_priv)->bool{let s=tnt_readb(p,STS2) as i32;(s&AFFN)!=0&&(s&BFFN)!=0}
unsafe fn tnt_transfer_count(p:*mut tnt4882_priv)->u32{let mut c=tnt_readb(p,CNT0) as u32;c|=(tnt_readb(p,CNT1) as u32)<<8;c|=(tnt_readb(p,CNT2) as u32)<<16;c|=(tnt_readb(p,CNT3) as u32)<<24;c.wrapping_neg()}

// The remaining driver entry points retain the C ABI and delegate to the corresponding
// translated NEC7210/GPIB primitives.  Their declarations are intentionally external.
extern "C" { pub fn tnt4882_internal_interrupt(board:*mut gpib_board)->c_int; pub fn tnt4882_accel_read(board:*mut gpib_board,buffer:*mut u8,length:usize,end:*mut c_int,bytes_read:*mut usize)->c_int; pub fn tnt4882_accel_write(board:*mut gpib_board,buffer:*mut u8,length:usize,send_eoi:c_int,bytes_written:*mut usize)->c_int; pub fn tnt4882_command(board:*mut gpib_board,buffer:*mut u8,length:usize,bytes_written:*mut usize)->c_int; pub fn tnt4882_init_module()->c_int; pub fn tnt4882_exit_module(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
