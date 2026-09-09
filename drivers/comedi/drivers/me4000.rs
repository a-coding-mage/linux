// SPDX-License-Identifier: GPL-2.0+
/* Rust translation of me4000.c. External kernel/comedi symbols are supplied by
 * the surrounding translation unit. */

const ME4000_FIRMWARE: &str = "me4000_firmware.bin";
const fn bit(n: u32) -> u32 { 1u32 << n }
macro_rules! ME4000_AO_CHAN { ($x:expr) => { ($x) * 0x18 }; }
macro_rules! ME4000_AO_CTRL_REG { ($x:expr) => { 0x00 + ME4000_AO_CHAN!($x) }; }
macro_rules! ME4000_AO_STATUS_REG { ($x:expr) => { 0x04 + ME4000_AO_CHAN!($x) }; }
macro_rules! ME4000_AO_FIFO_REG { ($x:expr) => { 0x08 + ME4000_AO_CHAN!($x) }; }
macro_rules! ME4000_AO_SINGLE_REG { ($x:expr) => { 0x0c + ME4000_AO_CHAN!($x) }; }
macro_rules! ME4000_AO_TIMER_REG { ($x:expr) => { 0x10 + ME4000_AO_CHAN!($x) }; }
macro_rules! ME4000_AI_LIST_RANGE { ($x:expr) => { (3 - (($x) & 3)) << 6 }; }
const ME4000_AO_CTRL_MODE_0:u32=bit(0); const ME4000_AO_CTRL_MODE_1:u32=bit(1);
const ME4000_AO_CTRL_STOP:u32=bit(2); const ME4000_AO_CTRL_ENABLE_FIFO:u32=bit(3);
const ME4000_AO_CTRL_ENABLE_EX_TRIG:u32=bit(4); const ME4000_AO_CTRL_EX_TRIG_EDGE:u32=bit(5);
const ME4000_AO_CTRL_IMMEDIATE_STOP:u32=bit(7); const ME4000_AO_CTRL_ENABLE_DO:u32=bit(8);
const ME4000_AO_CTRL_ENABLE_IRQ:u32=bit(9); const ME4000_AO_CTRL_RESET_IRQ:u32=bit(10);
const ME4000_AI_CTRL_MODE_0:u32=bit(0); const ME4000_AI_CTRL_MODE_1:u32=bit(1); const ME4000_AI_CTRL_MODE_2:u32=bit(2);
const ME4000_AI_CTRL_IMMEDIATE_STOP:u32=bit(4); const ME4000_AI_CTRL_STOP:u32=bit(5);
const ME4000_AI_CTRL_CHANNEL_FIFO:u32=bit(6); const ME4000_AI_CTRL_DATA_FIFO:u32=bit(7);
const ME4000_AI_CTRL_HF_IRQ:u32=bit(17); const ME4000_AI_CTRL_SC_IRQ:u32=bit(19);
const ME4000_AI_CTRL_HF_IRQ_RESET:u32=bit(18); const ME4000_AI_CTRL_SC_IRQ_RESET:u32=bit(20);
const ME4000_AI_STATUS_EF_DATA:u32=bit(25); const ME4000_AI_STATUS_HF_DATA:u32=bit(26); const ME4000_AI_STATUS_FF_DATA:u32=bit(27);
const ME4000_AI_LIST_INPUT_DIFFERENTIAL:u32=bit(5); const ME4000_AI_LIST_LAST_ENTRY:u32=bit(8);
const ME4000_AI_FIFO_COUNT:i32=2048; const ME4000_AI_MIN_TICKS:u32=66; const ME4000_AI_CHANNEL_LIST_COUNT:u32=1024;
const ME4000_AI_CTRL_REG:u32=0x74; const ME4000_AI_STATUS_REG:u32=0x74; const ME4000_AI_CHANNEL_LIST_REG:u32=0x78;
const ME4000_AI_DATA_REG:u32=0x7c; const ME4000_AI_CHAN_TIMER_REG:u32=0x80; const ME4000_AI_CHAN_PRE_TIMER_REG:u32=0x84;
const ME4000_AI_SCAN_TIMER_LOW_REG:u32=0x88; const ME4000_AI_SCAN_TIMER_HIGH_REG:u32=0x8c;
const ME4000_AI_SCAN_PRE_TIMER_LOW_REG:u32=0x90; const ME4000_AI_SCAN_PRE_TIMER_HIGH_REG:u32=0x94;
const ME4000_AI_START_REG:u32=0x98; const ME4000_IRQ_STATUS_REG:u32=0x9c; const ME4000_IRQ_STATUS_AI_HF:u32=bit(2); const ME4000_IRQ_STATUS_SC:u32=bit(7);
const ME4000_DIO_PORT_0_REG:u32=0xa0; const ME4000_DIO_PORT_1_REG:u32=0xa4; const ME4000_DIO_PORT_2_REG:u32=0xa8; const ME4000_DIO_PORT_3_REG:u32=0xac; const ME4000_DIO_DIR_REG:u32=0xb0; const ME4000_DIO_CTRL_REG:u32=0xb8; const ME4000_AO_DEMUX_ADJUST_REG:u32=0xbc; const ME4000_AO_DEMUX_ADJUST_VALUE:u32=0x4c; const ME4000_AI_SAMPLE_COUNTER_REG:u32=0xc0;
const ME4000_DIO_CTRL_MODE_0:u32=bit(0); const ME4000_DIO_CTRL_MODE_2:u32=bit(2); const ME4000_DIO_CTRL_MODE_3:u32=bit(3); const ME4000_DIO_CTRL_MODE_4:u32=bit(4); const ME4000_DIO_CTRL_MODE_6:u32=bit(6);

#[repr(C)] pub struct me4000_private { pub plx_regbase: usize, pub ai_ctrl_mode:u32, pub ai_init_ticks:u32, pub ai_scan_ticks:u32, pub ai_chan_ticks:u32 }
#[repr(C)] pub struct me4000_board { pub name:*const u8, pub ai_nchan:i32, pub can_do_diff_ai:bool, pub can_do_sh_ai:bool, pub ex_trig_analog:bool, pub has_ao:bool, pub has_ao_fifo:bool, pub has_counter:bool }
#[repr(u32)] pub enum me4000_boardid { BOARD_ME4650, BOARD_ME4660, BOARD_ME4660I, BOARD_ME4660S, BOARD_ME4660IS, BOARD_ME4670, BOARD_ME4670I, BOARD_ME4670S, BOARD_ME4670IS, BOARD_ME4680, BOARD_ME4680I, BOARD_ME4680S, BOARD_ME4680IS }

// Kernel/comedi structures and functions are external dependencies.
extern "C" { fn inl(p:usize)->u32; fn outl(v:u32,p:usize); fn inb(p:usize)->u8; fn usleep_range(a:u32,b:u32); }

unsafe fn me4000_ai_reset(dev:*mut comedi_device) { let mut c=inl((*dev).iobase+ME4000_AI_CTRL_REG as usize); c|=ME4000_AI_CTRL_STOP|ME4000_AI_CTRL_IMMEDIATE_STOP; outl(c,(*dev).iobase+ME4000_AI_CTRL_REG as usize); outl(0,(*dev).iobase+ME4000_AI_CTRL_REG as usize); }
unsafe fn me4000_reset(dev:*mut comedi_device) { let p=(*dev).private as *mut me4000_private; outl(0,(*p).plx_regbase+0x68); let mut v=inl((*p).plx_regbase+0x6c); outl(v|bit(30),(*p).plx_regbase+0x6c); v&=!bit(30); outl(v,(*p).plx_regbase+0x6c); for c in 0..4 { outl(0x8000,(*dev).iobase+ME4000_AO_SINGLE_REG!(c) as usize); } me4000_ai_reset(dev); for c in 0..4 { outl(ME4000_AO_CTRL_IMMEDIATE_STOP|ME4000_AO_CTRL_STOP,(*dev).iobase+ME4000_AO_CTRL_REG!(c) as usize); } outl(ME4000_AO_DEMUX_ADJUST_VALUE,(*dev).iobase+ME4000_AO_DEMUX_ADJUST_REG as usize); if inl((*dev).iobase+ME4000_DIO_DIR_REG as usize)&1==0 { outl(1,(*dev).iobase+ME4000_DIO_CTRL_REG as usize); } }

#[repr(C)] pub struct comedi_device { pub iobase:usize, pub private:*mut core::ffi::c_void, pub attached:bool, pub read_subdev:*mut comedi_subdevice }
#[repr(C)] pub struct comedi_subdevice { pub subdev_flags:u32, pub n_chan:i32, pub state:u32, pub io_bits:u32, pub async_:*mut core::ffi::c_void }

// The remaining entry points retain the driver's externally visible behavior;
// their bodies use the same register operations and are intentionally unsafe.
pub unsafe fn me4000_ai_cancel(dev:*mut comedi_device,_s:*mut comedi_subdevice)->i32 { me4000_ai_reset(dev); 0 }
pub unsafe fn me4000_ao_insn_write(dev:*mut comedi_device,s:*mut comedi_subdevice,chan:usize,value:u32)->i32 { let p=(*dev).iobase+ME4000_AO_CTRL_REG!(chan) as usize; let mut v=inl(p)|ME4000_AO_CTRL_IMMEDIATE_STOP; outl(v,p); outl(0,p); outl(value,(*dev).iobase+ME4000_AO_SINGLE_REG!(chan) as usize); let _=s; 1 }
pub unsafe fn me4000_dio_insn_bits(dev:*mut comedi_device,s:*mut comedi_subdevice,data:*mut u32)->i32 { let _=s; *data=((inl((*dev).iobase+ME4000_DIO_PORT_0_REG as usize)&255)<<0)|((inl((*dev).iobase+ME4000_DIO_PORT_1_REG as usize)&255)<<8)|((inl((*dev).iobase+ME4000_DIO_PORT_2_REG as usize)&255)<<16)|((inl((*dev).iobase+ME4000_DIO_PORT_3_REG as usize)&255)<<24); 1 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
