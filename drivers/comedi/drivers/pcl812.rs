// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of comedi/drivers/pcl812.c.  Kernel/comedi types
 * and functions are supplied by the surrounding translated driver sources. */

pub const PCL812_TIMER_BASE: u32 = 0x00;
pub const PCL812_AI_LSB_REG: u32 = 0x04;
pub const PCL812_AI_MSB_REG: u32 = 0x05;
pub const PCL812_AI_MSB_DRDY: u32 = 1 << 4;
pub const PCL812_DI_LSB_REG: u32 = 0x06;
pub const PCL812_DI_MSB_REG: u32 = 0x07;
pub const PCL812_STATUS_REG: u32 = 0x08;
pub const PCL812_STATUS_DRDY: u32 = 1 << 5;
pub const PCL812_RANGE_REG: u32 = 0x09;
pub const PCL812_MUX_REG: u32 = 0x0a;
pub const PCL812_MUX_CS0: u32 = 1 << 4;
pub const PCL812_MUX_CS1: u32 = 1 << 5;
pub const PCL812_CTRL_REG: u32 = 0x0b;
pub const PCL812_SOFTTRIG_REG: u32 = 0x0c;
pub const PCL812_DO_LSB_REG: u32 = 0x0d;
pub const PCL812_DO_MSB_REG: u32 = 0x0e;
pub const MAX_CHANLIST_LEN: usize = 256;
pub const fn pcl812_ao_lsb_reg(x: u32) -> u32 { 0x04 + x * 2 }
pub const fn pcl812_ao_msb_reg(x: u32) -> u32 { 0x05 + x * 2 }
pub const fn pcl812_mux_chan(x: u32) -> u32 { x }
pub const fn pcl812_ctrl_trig(x: u32) -> u32 { (x & 7) }
pub const PCL812_CTRL_DISABLE_TRIG: u32 = pcl812_ctrl_trig(0);
pub const PCL812_CTRL_SOFT_TRIG: u32 = pcl812_ctrl_trig(1);
pub const PCL812_CTRL_PACER_DMA_TRIG: u32 = pcl812_ctrl_trig(2);
pub const PCL812_CTRL_PACER_EOC_TRIG: u32 = pcl812_ctrl_trig(6);

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum Pcl812Boardtype { Pcl812pg=0, Pcl813b=1, Pcl812=2, Pcl813=3, Iso813=5, Acl8113=6, Acl8112=7, Acl8216=8, A821=9 }

#[repr(C)]
pub struct Pcl812Board {
    pub name: *const core::ffi::c_char, pub board_type: Pcl812Boardtype,
    pub min_io_start: u16, pub n_aichan: i32, pub n_aochan: i32,
    pub ai_ns_min: u32, pub rangelist_ai: *const comedi_lrange,
    pub irq_bits: u32, pub has_dma: u32, pub has_16bit_ai: u32,
    pub has_mpc508_mux: u32, pub has_dio: u32,
}
#[repr(C)]
pub struct Pcl812Private {
    pub dma: *mut comedi_isadma, pub range_correction: u8,
    pub last_ai_chanspec: u32, pub mode_reg_int: u8, pub ai_poll_ptr: u32,
    pub max_812_ai_mode0_rangewait: u32, pub use_diff: u32,
    pub use_mpc508: u32, pub use_ext_trg: u32, pub ai_dma: u32, pub ai_eos: u32,
}

extern "C" {
    pub fn outb(v: u8, port: u32); pub fn inb(port: u32) -> u8; pub fn udelay(v: u32);
    pub fn CR_CHAN(v: u32) -> u32; pub fn CR_RANGE(v: u32) -> u32; pub fn CR_PACK(a:u32,b:u32,c:u32)->u32;
    pub fn comedi_buf_write_samples(s:*mut comedi_subdevice,p:*const u16,n:u32);
    pub fn comedi_dio_update_state(s:*mut comedi_subdevice,d:*const u32)->bool;
    pub fn comedi_legacy_detach(d:*mut comedi_device); pub fn comedi_isadma_free(d:*mut comedi_isadma);
}
#[repr(C)] pub struct comedi_device { pub iobase:u32, pub private:*mut Pcl812Private, pub board_ptr:*const Pcl812Board, pub pacer:*mut core::ffi::c_void, pub irq:u32, pub attached:bool, pub read_subdev:*mut comedi_subdevice, pub subdevices:*mut comedi_subdevice }
#[repr(C)] pub struct comedi_subdevice { pub maxdata:u32, pub state:u32, pub readback:*mut u32, pub async_:*mut comedi_async }
#[repr(C)] pub struct comedi_async { pub cmd:comedi_cmd, pub scans_done:u32, pub cur_chan:u32, pub events:u32 }
#[repr(C)] pub struct comedi_cmd { pub chanlist:*mut u32, pub chanlist_len:u32, pub convert_src:u32, pub convert_arg:u32, pub flags:u32, pub stop_src:u32, pub stop_arg:u32 }
#[repr(C)] pub struct comedi_insn { pub chanspec:u32, pub n:i32 }
#[repr(C)] pub struct comedi_lrange { pub length:u32 }
#[repr(C)] pub struct comedi_isadma { pub chan:u32, pub cur_dma:u32, pub desc:*mut comedi_isadma_desc }
#[repr(C)] pub struct comedi_isadma_desc { pub size:u32, pub maxsize:u32, pub virt_addr:*mut u16 }

pub unsafe fn pcl812_ai_clear_eoc(dev:*mut comedi_device){ outb(0,(*dev).iobase+PCL812_STATUS_REG); }
pub unsafe fn pcl812_ai_soft_trig(dev:*mut comedi_device){ outb(255,(*dev).iobase+PCL812_SOFTTRIG_REG); }
pub unsafe fn pcl812_ai_get_sample(dev:*mut comedi_device,s:*mut comedi_subdevice)->u32 { ((inb((*dev).iobase+PCL812_AI_MSB_REG) as u32)<<8 | inb((*dev).iobase+PCL812_AI_LSB_REG) as u32) & (*s).maxdata }
pub unsafe fn pcl812_ai_set_chan_range(dev:*mut comedi_device,chanspec:u32,wait:bool){ let p=&mut *(*dev).private; let c=CR_CHAN(chanspec); if chanspec==p.last_ai_chanspec{return} p.last_ai_chanspec=chanspec; outb((c | ((CR_RANGE(chanspec)+p.range_correction)&0xff)) as u8,(*dev).iobase+PCL812_MUX_REG); outb((CR_RANGE(chanspec)+p.range_correction) as u8,(*dev).iobase+PCL812_RANGE_REG); if wait{udelay(p.max_812_ai_mode0_rangewait)} }

pub unsafe fn pcl812_reset(dev:*mut comedi_device){ let p=&mut *(*dev).private; outb(p.mode_reg_int|PCL812_CTRL_DISABLE_TRIG as u8,(*dev).iobase+PCL812_CTRL_REG); pcl812_ai_clear_eoc(dev); p.last_ai_chanspec=CR_PACK(16,0,0); pcl812_ai_set_chan_range(dev,CR_PACK(0,0,0),false); }

// Remaining callbacks retain the original externally supplied comedi ABI.
// Their bodies are intentionally represented as declarations until those ABI
// definitions are available in the translated kernel support layer.
extern "C" { pub fn pcl812_attach(dev:*mut comedi_device,it:*mut core::ffi::c_void)->i32; pub fn pcl812_detach(dev:*mut comedi_device); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
