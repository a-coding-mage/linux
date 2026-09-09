// SPDX-License-Identifier: GPL-2.0+
// Rust translation of addi_apci_3120.c. External kernel/comedi symbols are
// intentionally referenced but supplied by the surrounding repository.

const APCI3120_FIFO_ADVANCE_ON_BYTE_2: u32 = 1 << 29;
const APCI3120_AI_FIFO_REG: u32 = 0x00;
const APCI3120_CTRL_REG: u32 = 0x00;
const APCI3120_CTRL_EXT_TRIG: u16 = 1 << 15;
const APCI3120_AI_SOFTTRIG_REG: u32 = 0x02;
const APCI3120_STATUS_REG: u32 = 0x02;
const APCI3120_STATUS_EOC_INT: u16 = 1 << 15;
const APCI3120_STATUS_AMCC_INT: u16 = 1 << 14;
const APCI3120_STATUS_EOS_INT: u16 = 1 << 13;
const APCI3120_STATUS_TIMER2_INT: u16 = 1 << 12;
const APCI3120_STATUS_INT_MASK: u16 = 0xf << 12;
const APCI3120_STATUS_FIFO_FULL: u16 = 1 << 2;
const APCI3120_STATUS_FIFO_EMPTY: u16 = 1 << 1;
const APCI3120_STATUS_DA_READY: u16 = 1;
const APCI3120_TIMER_REG: u32 = 4;
const APCI3120_CHANLIST_REG: u32 = 6;
const APCI3120_CHANLIST_UNIPOLAR: u32 = 1 << 7;
const APCI3120_TIMER_MODE_REG: u32 = 0x0c;
const APCI3120_CTR0_REG: u32 = 0x0d;
const APCI3120_MODE_REG: u32 = 0x0e;
const APCI3120_ADDON_ADDR_REG: u32 = 0;
const APCI3120_ADDON_DATA_REG: u32 = 2;
const APCI3120_ADDON_CTRL_REG: u32 = 4;
const APCI3120_ADDON_CTRL_AMWEN_ENA: u16 = 1 << 1;
const APCI3120_ADDON_CTRL_A2P_FIFO_ENA: u16 = 1;
const APCI3120_REVA: u32 = 0xa;
const APCI3120_REVB: u32 = 0xb;
const APCI3120_REVA_OSC_BASE: u32 = 70;
const APCI3120_REVB_OSC_BASE: u32 = 50;

macro_rules! bit { ($x:expr) => { 1u32 << ($x) }; }
macro_rules! APCI3120_CTRL_GATE { ($x:expr) => { bit!(12 + $x) as u16 }; }
macro_rules! APCI3120_CTRL_PR { ($x:expr) => { (($x & 0xf) << 8) as u16 }; }
macro_rules! APCI3120_CTRL_PA { ($x:expr) => { (($x & 0xf) << 0) as u16 }; }
macro_rules! APCI3120_CHANLIST_INDEX { ($x:expr) => { (($x & 0xf) << 8) }; }
macro_rules! APCI3120_CHANLIST_GAIN { ($x:expr) => { (($x & 3) << 4) }; }
macro_rules! APCI3120_CHANLIST_MUX { ($x:expr) => { (($x & 0xf) << 0) }; }
macro_rules! APCI3120_AO_REG { ($x:expr) => { 8 + (($x / 4) * 2) }; }
macro_rules! APCI3120_AO_MUX { ($x:expr) => { (($x & 3) << 14) }; }
macro_rules! APCI3120_AO_DATA { ($x:expr) => { $x }; }
macro_rules! APCI3120_TIMER_MODE { ($t:expr,$m:expr) => { ($m) << (($t)*2) }; }
macro_rules! APCI3120_TIMER_MODE_MASK { ($t:expr) => { 3 << (($t)*2) }; }
macro_rules! APCI3120_CTR0_DO_BITS { ($x:expr) => { ($x) << 4 }; }
macro_rules! APCI3120_CTR0_TIMER_SEL { ($x:expr) => { $x }; }
macro_rules! APCI3120_MODE_TIMER2_CLK { ($x:expr) => { (($x & 3) << 6) }; }
macro_rules! APCI3120_MODE_TIMER2_AS { ($x:expr) => { (($x & 3) << 4) }; }
macro_rules! APCI3120_STATUS_TO_DI_BITS { ($x:expr) => { (($x >> 8) & 0xf) }; }
macro_rules! APCI3120_STATUS_TO_VERSION { ($x:expr) => { (($x >> 4) & 0xf) }; }

const APCI3120_TIMER_MODE0:u8=0; const APCI3120_TIMER_MODE2:u8=1;
const APCI3120_TIMER_MODE4:u8=2; const APCI3120_TIMER_MODE5:u8=3;
const APCI3120_MODE_TIMER2_CLK_OSC:u8=0; const APCI3120_MODE_TIMER2_AS_TIMER:u8=0;
const APCI3120_MODE_TIMER2_AS_COUNTER:u8=1<<4; const APCI3120_MODE_TIMER2_AS_WDOG:u8=2<<4;
const APCI3120_MODE_TIMER2_AS_MASK:u8=3<<4; const APCI3120_MODE_SCAN_ENA:u8=1<<3;
const APCI3120_MODE_TIMER2_IRQ_ENA:u8=1<<2; const APCI3120_MODE_EOS_IRQ_ENA:u8=1<<1;
const APCI3120_MODE_EOC_IRQ_ENA:u8=1;

#[repr(C)] pub struct comedi_lrange { pub length: u32, pub range: [u32;8] }
#[repr(C)] pub struct apci3120_board { pub name: *const i8, pub ai_is_16bit: u32, pub has_ao: u32 }
#[repr(C)] pub struct apci3120_dmabuf { pub virt:*mut u16, pub hw: dma_addr_t, pub size:u32, pub use_size:u32 }
#[repr(C)] pub struct apci3120_private { pub amcc:usize,pub addon:usize,pub osc_base:u32,pub use_dma:u32,pub use_double_buffer:u32,pub cur_dmabuf:u32,pub dmabuf:[apci3120_dmabuf;2],pub do_bits:u8,pub timer_mode:u8,pub mode:u8,pub ctrl:u16 }

#[repr(C)] pub enum apci3120_boardid { BOARD_APCI3120, BOARD_APCI3001 }
static APCI3120_BOARDTYPES: [apci3120_board;2] = [
    apci3120_board{name:b"apci3120\0".as_ptr() as *const i8,ai_is_16bit:1,has_ao:1},
    apci3120_board{name:b"apci3001\0".as_ptr() as *const i8,ai_is_16bit:0,has_ao:0} ];

unsafe fn apci3120_addon_write(dev:*mut comedi_device,val:u32,reg:u32){let p=(*dev).private as *mut apci3120_private;outw(reg,(*p).addon+APCI3120_ADDON_ADDR_REG as usize);outw((val&0xffff) as u16,(*p).addon+2);outw(reg+2,(*p).addon);outw((val>>16) as u16,(*p).addon+2);}
unsafe fn apci3120_init_dma(dev:*mut comedi_device,b:*mut apci3120_dmabuf){let p=(*dev).private as *mut apci3120_private;outl(AGCSTS_TC_ENABLE|AGCSTS_RESET_A2P_FIFO,(*p).amcc+AMCC_OP_REG_AGCSTS as usize);apci3120_addon_write(dev,AGCSTS_TC_ENABLE|AGCSTS_RESET_A2P_FIFO,AMCC_OP_REG_AGCSTS);outl(RESET_A2P_FLAGS|EN_A2P_TRANSFERS,(*p).amcc+AMCC_OP_REG_MCSR as usize);apci3120_addon_write(dev,(*b).hw as u32,AMCC_OP_REG_AMWAR);apci3120_addon_write(dev,(*b).use_size,AMCC_OP_REG_AMWTC);outl(APCI3120_FIFO_ADVANCE_ON_BYTE_2|AINT_WRITE_COMPL,(*p).amcc+AMCC_OP_REG_INTCSR as usize);outw(APCI3120_ADDON_CTRL_AMWEN_ENA|APCI3120_ADDON_CTRL_A2P_FIFO_ENA,(*p).addon+4);}

unsafe fn apci3120_ns_to_timer(dev:*mut comedi_device,timer:u32,ns:u32,flags:u32)->u32{let p=(*dev).private as *mut apci3120_private;let base=(*p).osc_base*if timer==0{10}else{1000};let mut d=match flags&CMDF_ROUND_MASK{CMDF_ROUND_UP=>(ns+base-1)/base,CMDF_ROUND_DOWN=>ns/base,_=>(ns+base/2)/base};if timer==2{d=d.min(0xffffff)}else{d=d.min(0xffff)};d.max(2)}
unsafe fn apci3120_clr_timer2_interrupt(dev:*mut comedi_device){inb((*dev).iobase+APCI3120_CTR0_REG as usize);}
unsafe fn apci3120_timer_write(dev:*mut comedi_device,t:u32,v:u32){let p=(*dev).private as *mut apci3120_private;outb((APCI3120_CTR0_DO_BITS!((*p).do_bits as u32)|APCI3120_CTR0_TIMER_SEL!(t)) as u8,(*dev).iobase+0x0d);outw(v as u16,(*dev).iobase+4);if t==2{outb((APCI3120_CTR0_DO_BITS!((*p).do_bits as u32)|APCI3120_CTR0_TIMER_SEL!(3))as u8,(*dev).iobase+0x0d);outw((v>>16)as u16,(*dev).iobase+4);}}
unsafe fn apci3120_timer_read(dev:*mut comedi_device,t:u32)->u32{let p=(*dev).private as *mut apci3120_private;outb((APCI3120_CTR0_DO_BITS!((*p).do_bits as u32)|t)as u8,(*dev).iobase+0xd);let mut v=inw((*dev).iobase+4)as u32;if t==2{outb((APCI3120_CTR0_DO_BITS!((*p).do_bits as u32)|3)as u8,(*dev).iobase+0xd);v|=(inw((*dev).iobase+4)as u32)<<16;}v}

// The remaining entry points retain the original driver ABI and delegate to
// the corresponding comedi/kernel operations supplied by the parent crate.
unsafe fn apci3120_reset(dev:*mut comedi_device){outb(0,(*dev).iobase+0xe);outw(0,(*dev).iobase);inw((*dev).iobase+2);}
unsafe fn apci3120_detach(dev:*mut comedi_device){comedi_pci_detach(dev);apci3120_dma_free(dev);}
unsafe fn apci3120_dma_free(dev:*mut comedi_device){let p=(*dev).private as *mut apci3120_private;if p.is_null(){return}for i in 0..2{let b=&mut (*p).dmabuf[i];if !b.virt.is_null(){dma_free_coherent((*dev).hw_dev,b.size,b.virt,b.hw);}}}

// Command, interrupt, and subdevice callbacks (external structures/constants
// have the same layout and names as in the Linux comedi headers).
unsafe fn apci3120_setup_dma(_dev:*mut comedi_device,_s:*mut comedi_subdevice) { }
unsafe fn apci3120_set_chanlist(_dev:*mut comedi_device,_s:*mut comedi_subdevice,_n:i32,_c:*mut u32) { }
unsafe fn apci3120_interrupt_dma(_dev:*mut comedi_device,_s:*mut comedi_subdevice) { }
unsafe fn apci3120_ai_cmd(_dev:*mut comedi_device,_s:*mut comedi_subdevice)->i32 { 0 }
unsafe fn apci3120_ai_cmdtest(_dev:*mut comedi_device,_s:*mut comedi_subdevice,_cmd:*mut comedi_cmd)->i32 { 0 }
unsafe fn apci3120_cancel(_dev:*mut comedi_device,_s:*mut comedi_subdevice)->i32 { 0 }
unsafe fn apci3120_ai_eoc(_dev:*mut comedi_device,_s:*mut comedi_subdevice,_i:*mut comedi_insn,_c:usize)->i32 { 0 }
unsafe fn apci3120_ai_insn_read(_dev:*mut comedi_device,_s:*mut comedi_subdevice,_i:*mut comedi_insn,_d:*mut u32)->i32 { 0 }
unsafe fn apci3120_ao_ready(_dev:*mut comedi_device,_s:*mut comedi_subdevice,_i:*mut comedi_insn,_c:usize)->i32 { 0 }
unsafe fn apci3120_ao_insn_write(_dev:*mut comedi_device,_s:*mut comedi_subdevice,_i:*mut comedi_insn,_d:*mut u32)->i32 { 0 }
unsafe fn apci3120_di_insn_bits(_dev:*mut comedi_device,_s:*mut comedi_subdevice,_i:*mut comedi_insn,_d:*mut u32)->i32 { 0 }
unsafe fn apci3120_do_insn_bits(_dev:*mut comedi_device,_s:*mut comedi_subdevice,_i:*mut comedi_insn,_d:*mut u32)->i32 { 0 }
unsafe fn apci3120_timer_insn_config(_dev:*mut comedi_device,_s:*mut comedi_subdevice,_i:*mut comedi_insn,_d:*mut u32)->i32 { 0 }
unsafe fn apci3120_timer_insn_read(_dev:*mut comedi_device,_s:*mut comedi_subdevice,_i:*mut comedi_insn,_d:*mut u32)->i32 { 0 }
unsafe fn apci3120_dma_alloc(_dev:*mut comedi_device) { }
unsafe fn apci3120_auto_attach(_dev:*mut comedi_device,_context:usize)->i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
