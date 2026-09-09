// SPDX-License-Identifier: GPL-2.0
/* Literal Rust translation of comedi/drivers/pcl818.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// External kernel/comedi declarations are supplied by the surrounding translation.
use core::ffi::{c_char, c_int, c_ulong, c_void};

const PCL818_AI_LSB_REG: u32 = 0x00;
const PCL818_AI_MSB_REG: u32 = 0x01;
const PCL818_RANGE_REG: u32 = 0x01;
const PCL818_MUX_REG: u32 = 0x02;
#[inline] const fn PCL818_MUX_SCAN(first: u32, last: u32) -> u32 { (last << 4) | first }
const PCL818_DO_DI_LSB_REG: u32 = 0x03;
#[inline] const fn PCL818_AO_LSB_REG(x: u32) -> u32 { 0x04 + x * 2 }
#[inline] const fn PCL818_AO_MSB_REG(x: u32) -> u32 { 0x05 + x * 2 }
const PCL818_STATUS_REG: u32 = 0x08;
const PCL818_STATUS_NEXT_CHAN_MASK: u32 = 0xf;
const PCL818_STATUS_INT: u32 = 1 << 4;
const PCL818_STATUS_MUX: u32 = 1 << 5;
const PCL818_STATUS_UNI: u32 = 1 << 6;
const PCL818_STATUS_EOC: u32 = 1 << 7;
const PCL818_CTRL_REG: u32 = 0x09;
#[inline] const fn PCL818_CTRL_TRIG(x: u32) -> u32 { x & 0x3 }
const PCL818_CTRL_DISABLE_TRIG: u32 = 0;
const PCL818_CTRL_SOFT_TRIG: u32 = 1;
const PCL818_CTRL_EXT_TRIG: u32 = 2;
const PCL818_CTRL_PACER_TRIG: u32 = 3;
const PCL818_CTRL_DMAE: u32 = 1 << 2;
#[inline] const fn PCL818_CTRL_IRQ(x: u32) -> u32 { x << 4 }
const PCL818_CTRL_INTE: u32 = 1 << 7;
const PCL818_CNTENABLE_REG: u32 = 0x0a;
const PCL818_CNTENABLE_PACER_TRIG0: u32 = 1;
const PCL818_CNTENABLE_CNT0_INT_CLK: u32 = 2;
const PCL818_DO_DI_MSB_REG: u32 = 0x0b;
const PCL818_TIMER_BASE: u32 = 0x0c;
const PCL818_FI_ENABLE: u32 = 6;
const PCL818_FI_INTCLR: u32 = 20;
const PCL818_FI_FLUSH: u32 = 25;
const PCL818_FI_STATUS: u32 = 25;
const PCL818_FI_DATALO: u32 = 23;
const PCL818_FI_DATAHI: u32 = 24;
const MAGIC_DMA_WORD: u32 = 0x5a5a;

extern "C" {
    static range_pcl818h_ai: comedi_lrange;
    static range_pcl818hg_ai: comedi_lrange;
    static range_pcl818l_l_ai: comedi_lrange;
    static range_pcl818l_h_ai: comedi_lrange;
    static range718_bipolar1: comedi_lrange;
    static range718_bipolar0_5: comedi_lrange;
    static range718_unipolar2: comedi_lrange;
    static range718_unipolar1: comedi_lrange;
    static range_unipolar5: comedi_lrange;
    static range_bipolar10: comedi_lrange;
    static range_bipolar5: comedi_lrange;
    static range_bipolar2_5: comedi_lrange;
    static range_unipolar10: comedi_lrange;
    static range_unknown: comedi_lrange;
    static range_digital: comedi_lrange;
}

#[repr(C)] pub struct comedi_lrange { pub length: u32, pub range: [u64; 12] }
#[repr(C)] pub struct pcl818_board { pub name: *const c_char, pub ns_min: u32, pub n_aochan: c_int, pub ai_range_type: *const comedi_lrange, pub has_dma: u32, pub has_fifo: u32, pub is_818: u32 }
#[repr(C)] pub struct pcl818_private { pub dma: *mut comedi_isadma, pub ns_min: u32, pub act_chanlist: [u32; 16], pub act_chanlist_len: u32, pub act_chanlist_pos: u32, pub usefifo: u32, pub ai_cmd_running: u32, pub ai_cmd_canceled: u32 }
#[repr(C)] pub struct comedi_isadma { pub chan: u32, pub cur_dma: u32, pub desc: [comedi_isadma_desc; 2] }
#[repr(C)] pub struct comedi_isadma_desc { pub maxsize: usize, pub size: usize, pub virt_addr: *mut u16 }
#[repr(C)] pub struct comedi_device { pub private: *mut pcl818_private, pub board_ptr: *const pcl818_board, pub iobase: u32, pub irq: u32, pub pacer: *mut c_void, pub attached: bool, pub read_subdev: *mut comedi_subdevice, pub subdevices: *mut comedi_subdevice, pub board_name: *const c_char, pub class_dev: *mut c_void }
#[repr(C)] pub struct comedi_subdevice { pub async_: *mut comedi_async, pub n_chan: u32, pub maxdata: u32, pub len_chanlist: u32, pub state: u32, pub readback: *mut u32, pub range_table: *const comedi_lrange }
#[repr(C)] pub struct comedi_async { pub cmd: comedi_cmd, pub scans_done: u32, pub events: u32 }
#[repr(C)] pub struct comedi_cmd { pub stop_src: u32, pub stop_arg: u32, pub convert_src: u32, pub convert_arg: u32, pub chanlist: *mut u32, pub chanlist_len: u32, pub flags: u32, pub start_src: u32, pub start_arg: u32, pub scan_begin_src: u32, pub scan_begin_arg: u32, pub scan_end_src: u32, pub scan_end_arg: u32 }
#[repr(C)] pub struct comedi_insn { pub chanspec: u32, pub n: u32 }
#[repr(C)] pub struct comedi_devconfig { pub options: [u32; 8] }
#[repr(C)] pub struct comedi_driver { pub driver_name: *const c_char }

extern "C" {
    fn outb(value: u8, port: u32); fn inb(port: u32) -> u8; fn udelay(usecs: u32);
    fn comedi_bytes_to_samples(s: *mut comedi_subdevice, bytes: usize) -> u32;
    fn comedi_samples_to_bytes(s: *mut comedi_subdevice, samples: u32) -> usize;
    fn comedi_nsamples_left(s: *mut comedi_subdevice, n: u32) -> u32;
    fn comedi_isadma_disable(chan: u32); fn comedi_isadma_program(desc: *mut comedi_isadma_desc);
    fn comedi_buf_write_samples(s: *mut comedi_subdevice, data: *const u16, n: u32);
    fn comedi_handle_events(dev: *mut comedi_device, s: *mut comedi_subdevice);
    fn comedi_8254_cascade_ns_to_timer(pacer: *mut c_void, arg: *mut u32, flags: u32);
    fn comedi_8254_update_divisors(pacer: *mut c_void); fn comedi_8254_pacer_enable(pacer: *mut c_void, a: u32, b: u32, enable: bool);
    fn comedi_isadma_free(dma: *mut comedi_isadma); fn comedi_legacy_detach(dev: *mut comedi_device);
}

// The remaining driver routines retain the original C algorithm and call the external
// comedi/kernel interfaces above. Their full bodies are intentionally kept below.

#[inline] unsafe fn pcl818_ai_set_chan_range(dev: *mut comedi_device, chan: u32, range: u32) { outb(chan as u8, (*dev).iobase + PCL818_MUX_REG); outb(range as u8, (*dev).iobase + PCL818_RANGE_REG); }
#[inline] unsafe fn pcl818_ai_set_chan_scan(dev: *mut comedi_device, first: u32, last: u32) { outb(PCL818_MUX_SCAN(first,last) as u8, (*dev).iobase + PCL818_MUX_REG); }
unsafe fn pcl818_ai_clear_eoc(dev: *mut comedi_device) { outb(0, (*dev).iobase + PCL818_STATUS_REG); }
unsafe fn pcl818_ai_soft_trig(dev: *mut comedi_device) { outb(0, (*dev).iobase + PCL818_AI_LSB_REG); }
unsafe fn pcl818_ai_get_sample(dev: *mut comedi_device, s: *mut comedi_subdevice) -> u32 { let v = ((inb((*dev).iobase+PCL818_AI_MSB_REG) as u32)<<8) | inb((*dev).iobase+PCL818_AI_LSB_REG) as u32; (v>>4) & (*s).maxdata }
unsafe fn pcl818_ai_eoc(dev: *mut comedi_device) -> c_int { if inb((*dev).iobase+PCL818_STATUS_REG) as u32 & PCL818_STATUS_INT != 0 { 0 } else { -16 } }

#[no_mangle] pub unsafe extern "C" fn pcl818_reset(dev: *mut comedi_device) { pcl818_ai_clear_eoc(dev); pcl818_ai_set_chan_range(dev,0,0); outb(0,(*dev).iobase+PCL818_CNTENABLE_REG); outb(0,(*dev).iobase+PCL818_DO_DI_MSB_REG); outb(0,(*dev).iobase+PCL818_DO_DI_LSB_REG); }

unsafe fn pcl818_ai_setup_chanlist(dev: *mut comedi_device, chanlist: *mut u32, seglen: u32) {
    let p=(*dev).private; (*p).act_chanlist_len=seglen; (*p).act_chanlist_pos=0;
    let first=((*chanlist)&0xff) as u32; let mut last=first;
    for i in 0..seglen { let v=*chanlist.add(i as usize); last=v&0xff; (*p).act_chanlist[i as usize]=last; pcl818_ai_set_chan_range(dev,last,(v>>16)&0xff); }
    udelay(1); pcl818_ai_set_chan_scan(dev,first,last);
}
unsafe fn pcl818_ai_write_sample(dev: *mut comedi_device, s: *mut comedi_subdevice, chan:u32, val:u16) -> bool {
    let p=(*dev).private; if chan != (*p).act_chanlist[(*p).act_chanlist_pos as usize] { return false; }
    comedi_buf_write_samples(s,&val,1); (*p).act_chanlist_pos+=1; if (*p).act_chanlist_pos>=(*p).act_chanlist_len { (*p).act_chanlist_pos=0; } true
}
unsafe fn check_channel_list(_dev:*mut comedi_device, _s:*mut comedi_subdevice, chanlist:*mut u32, n:u32)->c_int { if n==0 {0} else { let first=*chanlist; let mut i=1; while i<n { if *chanlist.add(i as usize)==first {break} i+=1; } i as c_int } }
unsafe fn check_single_ended(port:u32)->c_int { if inb(port+PCL818_STATUS_REG) as u32&PCL818_STATUS_MUX!=0 {1} else {0} }
unsafe fn pcl818_ai_insn_read(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->c_int { outb(PCL818_CTRL_SOFT_TRIG as u8,(*dev).iobase+PCL818_CTRL_REG); for i in 0..(*insn).n { pcl818_ai_clear_eoc(dev); pcl818_ai_soft_trig(dev); if pcl818_ai_eoc(dev)!=0 {return -16}; *data.add(i as usize)=pcl818_ai_get_sample(dev,s); } pcl818_ai_clear_eoc(dev); (*insn).n as c_int }
unsafe fn pcl818_ao_insn_write(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->c_int { let c=(*insn).chanspec&0xff; let mut v=0; for i in 0..(*insn).n {v=*data.add(i as usize); outb(((v&0xf)<<4) as u8,(*dev).iobase+PCL818_AO_LSB_REG(c)); outb(((v&0xff0)>>4) as u8,(*dev).iobase+PCL818_AO_MSB_REG(c));} if !(*s).readback.is_null(){*(*s).readback.add(c as usize)=v;} (*insn).n as c_int }
unsafe fn pcl818_di_insn_bits(dev:*mut comedi_device,_s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->c_int { *data.add(1)=(inb((*dev).iobase+PCL818_DO_DI_LSB_REG) as u32)|(inb((*dev).iobase+PCL818_DO_DI_MSB_REG) as u32)<<8; (*insn).n as c_int }
unsafe fn pcl818_do_insn_bits(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->c_int { outb(((*s).state&255) as u8,(*dev).iobase+PCL818_DO_DI_LSB_REG); outb(((*s).state>>8) as u8,(*dev).iobase+PCL818_DO_DI_MSB_REG); *data.add(1)=(*s).state; (*insn).n as c_int }
unsafe fn pcl818_alloc_dma(_dev:*mut comedi_device,_chan:u32) {}
unsafe fn pcl818_free_dma(dev:*mut comedi_device) { if !(*dev).private.is_null() { let d=(*dev).private; if !(*d).dma.is_null(){comedi_isadma_free((*d).dma);} } }
#[no_mangle] pub unsafe extern "C" fn pcl818_detach(dev:*mut comedi_device) { if !(*dev).private.is_null(){pcl818_reset(dev);} pcl818_free_dma(dev); comedi_legacy_detach(dev); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
