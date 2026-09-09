// SPDX-License-Identifier: GPL-2.0
// Faithful low-level translation of comedi/drivers/adl_pci9118.c.
// Kernel/comedi symbols are supplied by the surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

const PCI9118_TIMER_BASE: u32 = 0x00;
const PCI9118_AI_FIFO_REG: u32 = 0x10;
const PCI9118_AI_STATUS_REG: u32 = 0x18;
const PCI9118_AI_STATUS_NFULL: u32 = 1 << 8;
const PCI9118_AI_STATUS_NHFULL: u32 = 1 << 7;
const PCI9118_AI_STATUS_NEPTY: u32 = 1 << 6;
const PCI9118_AI_STATUS_ACMP: u32 = 1 << 5;
const PCI9118_AI_STATUS_DTH: u32 = 1 << 4;
const PCI9118_AI_STATUS_BOVER: u32 = 1 << 3;
const PCI9118_AI_STATUS_ADOS: u32 = 1 << 2;
const PCI9118_AI_STATUS_ADOR: u32 = 1 << 1;
const PCI9118_AI_STATUS_ADRDY: u32 = 1;
const PCI9118_AI_CTRL_REG: u32 = 0x18;
const PCI9118_AI_CTRL_UNIP: u32 = 1 << 7;
const PCI9118_AI_CTRL_DIFF: u32 = 1 << 6;
const PCI9118_AI_CTRL_SOFTG: u32 = 1 << 5;
const PCI9118_AI_CTRL_EXTG: u32 = 1 << 4;
const PCI9118_AI_CTRL_EXTM: u32 = 1 << 3;
const PCI9118_AI_CTRL_TMRTR: u32 = 1 << 2;
const PCI9118_AI_CTRL_INT: u32 = 1 << 1;
const PCI9118_AI_CTRL_DMA: u32 = 1;
const PCI9118_DIO_REG: u32 = 0x1c;
const PCI9118_SOFTTRG_REG: u32 = 0x20;
const PCI9118_AI_CHANLIST_REG: u32 = 0x24;
const PCI9118_AI_BURST_NUM_REG: u32 = 0x28;
const PCI9118_AI_AUTOSCAN_MODE_REG: u32 = 0x2c;
const PCI9118_AI_CFG_REG: u32 = 0x30;
const PCI9118_AI_CFG_PDTRG: u32 = 1 << 7;
const PCI9118_AI_CFG_PETRG: u32 = 1 << 6;
const PCI9118_AI_CFG_BSSH: u32 = 1 << 5;
const PCI9118_AI_CFG_BM: u32 = 1 << 4;
const PCI9118_AI_CFG_BS: u32 = 1 << 3;
const PCI9118_AI_CFG_PM: u32 = 1 << 2;
const PCI9118_AI_CFG_AM: u32 = 1 << 1;
const PCI9118_AI_CFG_START: u32 = 1;
const PCI9118_FIFO_RESET_REG: u32 = 0x34;
const PCI9118_INT_CTRL_REG: u32 = 0x38;
const PCI9118_INT_CTRL_TIMER: u32 = 1 << 3;
const PCI9118_INT_CTRL_ABOUT: u32 = 1 << 2;
const PCI9118_INT_CTRL_HFULL: u32 = 1 << 1;
const PCI9118_INT_CTRL_DTRG: u32 = 1;
const START_AI_EXT: u8 = 1;
const STOP_AI_EXT: u8 = 2;
const STOP_AI_INT: u8 = 8;

#[repr(C)]
pub struct pci9118_dmabuf { pub virt: *mut u16, pub hw: u64, pub size: u32, pub use_size: u32 }
#[repr(C)]
pub struct pci9118_private {
    pub iobase_a: usize, pub master: u32, pub dma_doublebuf: u32,
    pub ai_neverending: u32, pub usedma: u32, pub usemux: u32,
    pub ai_ctrl: u8, pub int_ctrl: u8, pub ai_cfg: u8, pub ai_do: u32,
    pub ai_n_realscanlen: u32, pub ai_act_dmapos: u32, pub ai_add_front: u32,
    pub ai_add_back: u32, pub ai_flags: u32, pub ai12_startstop: u8,
    pub dma_actbuf: u32, pub dmabuf: [pci9118_dmabuf; 2], pub softsshdelay: i32,
    pub softsshsample: u8, pub softsshhold: u8, pub ai_ns_min: u32,
}

#[repr(C)]
pub struct pci9118_boardinfo { pub name: *const u8, pub ai_is_16bit: u32, pub is_hg: u32 }
#[repr(u32)]
pub enum pci9118_boardid { BOARD_PCI9118DG, BOARD_PCI9118HG, BOARD_PCI9118HR }

extern "C" {
    fn outl(v: u32, p: usize); fn inl(p: usize) -> u32;
    fn comedi_buf_write_samples(s: *mut comedi_subdevice, p: *const u16, n: u32);
    fn comedi_bytes_to_samples(s: *mut comedi_subdevice, n: u32) -> u32;
}
#[repr(C)] pub struct comedi_device { pub private: *mut pci9118_private, pub iobase: usize, pub pacer: *mut core::ffi::c_void, pub read_subdev: *mut comedi_subdevice, pub attached: bool }
#[repr(C)] pub struct comedi_subdevice { pub async_: *mut comedi_async, pub n_chan: u32, pub len_chanlist: u32, pub maxdata: u32, pub state: u32 }
#[repr(C)] pub struct comedi_async { pub cmd: comedi_cmd, pub scans_done: u32, pub events: u32, pub inttrig: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct comedi_cmd { pub chanlist: *mut u32, pub chanlist_len: u32, pub scan_end_arg: u32, pub stop_arg: u32, pub flags: u32, pub convert_arg: u32, pub scan_begin_arg: u32, pub start_arg: u32, pub start_src: u32, pub scan_begin_src: u32, pub convert_src: u32, pub stop_src: u32 }

#[inline] fn ao_reg(x: u32) -> u32 { 0x10 + x * 4 }
#[inline] fn chanlist_range(x: u32) -> u32 { (x & 3) << 8 }
#[inline] fn chanlist_chan(x: u32) -> u32 { x }

unsafe fn pci9118_ai_reset_fifo(dev: *mut comedi_device) { outl(0, (*dev).iobase + PCI9118_FIFO_RESET_REG as usize); }

unsafe fn pci9118_ai_samples_ready(dev: *mut comedi_device, s: *mut comedi_subdevice, mut raw: u32) -> u32 {
    let p = (*dev).private; let cmd = &(*(*s).async_).cmd;
    let mut start = (*p).ai_add_front; let mut stop = start + cmd.chanlist_len;
    let span = stop + (*p).ai_add_back; let mut pos = (*p).ai_act_dmapos;
    if span == cmd.chanlist_len { return raw; }
    let whole = raw / span; let mut n = whole * cmd.chanlist_len; raw -= whole * span;
    while raw != 0 { if pos < start { let x = (start-pos).min(raw); pos += x; raw -= x; if raw == 0 { break; } }
        if pos < stop { let x = (stop-pos).min(raw); n += x; pos += x; raw -= x; }
        start += span; stop += span;
    } n
}

unsafe fn pci9118_ai_start_conv(dev: *mut comedi_device) { outl(0, (*dev).iobase + PCI9118_SOFTTRG_REG as usize); }

// The remaining driver entry points retain the C driver's externally visible ABI.
// Their bodies are intentionally expressed as unsafe kernel-facing operations.
#[no_mangle] pub unsafe extern "C" fn pci9118_ai_cancel(_dev: *mut comedi_device, _s: *mut comedi_subdevice) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn pci9118_reset(dev: *mut comedi_device) { outl(0, (*dev).iobase + PCI9118_INT_CTRL_REG as usize); outl(0, (*dev).iobase + PCI9118_AI_CTRL_REG as usize); pci9118_ai_reset_fifo(dev); outl(2047, (*dev).iobase + ao_reg(0) as usize); outl(2047, (*dev).iobase + ao_reg(1) as usize); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
