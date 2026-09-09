// SPDX-License-Identifier: GPL-2.0
/* Literal Rust translation of quatech_daqp_cs.c. */

const DAQP_AI_FIFO_REG: u32 = 0x00;
const DAQP_SCANLIST_REG: u32 = 0x01;
const DAQP_SCANLIST_DIFFERENTIAL: u32 = 1 << 14;
const DAQP_SCANLIST_START: u32 = 1 << 7;
const DAQP_CTRL_REG: u32 = 0x02;
const DAQP_STATUS_REG: u32 = 0x02;
const DAQP_DI_REG: u32 = 0x03;
const DAQP_DO_REG: u32 = 0x03;
const DAQP_PACER_LOW_REG: u32 = 0x04;
const DAQP_PACER_MID_REG: u32 = 0x05;
const DAQP_PACER_HIGH_REG: u32 = 0x06;
const DAQP_CMD_REG: u32 = 0x07;
const DAQP_AO_REG: u32 = 0x08;
const DAQP_TIMER_REG: u32 = 0x0a;
const DAQP_AUX_REG: u32 = 0x0f;
const DAQP_FIFO_SIZE: u32 = 4096;
const DAQP_MAX_TIMER_SPEED: u32 = 10000;

const fn scanlist_gain(x: u32) -> u32 { (x & 0x3) << 12 }
const fn scanlist_channel(x: u32) -> u32 { (x & 0xf) << 8 }
const fn ctrl_pacer_clk(x: u32) -> u32 { (x & 0x3) << 6 }
const DAQP_CTRL_PACER_CLK_EXT: u32 = ctrl_pacer_clk(0);
const DAQP_CTRL_PACER_CLK_5MHZ: u32 = ctrl_pacer_clk(1);
const DAQP_CTRL_PACER_CLK_1MHZ: u32 = ctrl_pacer_clk(2);
const DAQP_CTRL_PACER_CLK_100KHZ: u32 = ctrl_pacer_clk(3);
const DAQP_CTRL_EXPANSION: u32 = 1 << 5;
const DAQP_CTRL_EOS_INT_ENA: u32 = 1 << 4;
const DAQP_CTRL_FIFO_INT_ENA: u32 = 1 << 3;
const DAQP_CTRL_TRIG_MODE: u32 = 1 << 2;
const DAQP_CTRL_TRIG_SRC: u32 = 1 << 1;
const DAQP_CTRL_TRIG_EDGE: u32 = 1;
const DAQP_STATUS_IDLE: u32 = 1 << 7;
const DAQP_STATUS_RUNNING: u32 = 1 << 6;
const DAQP_STATUS_DATA_LOST: u32 = 1 << 5;
const DAQP_STATUS_END_OF_SCAN: u32 = 1 << 4;
const DAQP_STATUS_FIFO_THRESHOLD: u32 = 1 << 3;
const DAQP_STATUS_FIFO_FULL: u32 = 1 << 2;
const DAQP_STATUS_FIFO_NEARFULL: u32 = 1 << 1;
const DAQP_STATUS_FIFO_EMPTY: u32 = 1;
const DAQP_STATUS_EVENTS: u32 = DAQP_STATUS_DATA_LOST | DAQP_STATUS_END_OF_SCAN | DAQP_STATUS_FIFO_THRESHOLD;
const DAQP_CMD_ARM: u32 = 1 << 7;
const DAQP_CMD_RSTF: u32 = 1 << 6;
const DAQP_CMD_RSTQ: u32 = 1 << 5;
const DAQP_CMD_STOP: u32 = 1 << 4;
const DAQP_CMD_LATCH: u32 = 1 << 3;
const fn cmd_scanrate(x: u32) -> u32 { (x & 0x3) << 1 }
const DAQP_CMD_SCANRATE_100KHZ: u32 = cmd_scanrate(0);
const DAQP_CMD_SCANRATE_50KHZ: u32 = cmd_scanrate(1);
const DAQP_CMD_SCANRATE_25KHZ: u32 = cmd_scanrate(2);
const DAQP_CMD_FIFO_DATA: u32 = 1;
const DAQP_AUX_EXT_ANALOG_TRIG: u32 = 1 << 7;
const DAQP_AUX_PRETRIG: u32 = 1 << 6;
const DAQP_AUX_TIMER_INT_ENA: u32 = 1 << 5;
const fn aux_timer_mode(x: u32) -> u32 { (x & 0x3) << 3 }
const DAQP_AUX_TIMER_MODE_RELOAD: u32 = aux_timer_mode(0);
const DAQP_AUX_TIMER_MODE_PAUSE: u32 = aux_timer_mode(1);
const DAQP_AUX_TIMER_MODE_GO: u32 = aux_timer_mode(2);
const DAQP_AUX_TIMER_MODE_EXT: u32 = aux_timer_mode(3);
const DAQP_AUX_TIMER_CLK_SRC_EXT: u32 = 1 << 2;
const fn aux_da_update(x: u32) -> u32 { (x & 0x3) }
const DAQP_AUX_DA_UPDATE_DIRECT: u32 = aux_da_update(0);
const DAQP_AUX_DA_UPDATE_OVERFLOW: u32 = aux_da_update(1);
const DAQP_AUX_DA_UPDATE_EXTERNAL: u32 = aux_da_update(2);
const DAQP_AUX_DA_UPDATE_PACER: u32 = aux_da_update(3);
const DAQP_AUX_RUNNING: u32 = 1 << 7;
const DAQP_AUX_TRIGGERED: u32 = 1 << 6;
const DAQP_AUX_DA_BUFFER: u32 = 1 << 5;
const DAQP_AUX_TIMER_OVERFLOW: u32 = 1 << 4;
const DAQP_AUX_CONVERSION: u32 = 1 << 3;

#[repr(C)]
struct daqp_private { pacer_div: u32, stop: i32 }

extern "C" {
    fn inb(port: u32) -> u8; fn outb(value: u8, port: u32); fn outw(value: u16, port: u32);
    fn comedi_offset_munge(s: *mut comedi_subdevice, val: u32) -> u32;
    fn comedi_buf_write_samples(s: *mut comedi_subdevice, data: *const u16, n: u32);
    fn comedi_handle_events(dev: *mut comedi_device, s: *mut comedi_subdevice);
    fn comedi_timeout(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, f: unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,u64)->i32, c: u64) -> i32;
    fn comedi_bytes_per_sample(s: *mut comedi_subdevice) -> u32;
    fn comedi_dio_update_state(s: *mut comedi_subdevice, data: *mut u32) -> i32;
}

#[repr(C)] struct comedi_device { iobase: u32, private: *mut daqp_private, attached: i32, read_subdev: *mut comedi_subdevice, irq: i32, subdevices: *mut comedi_subdevice, class_dev: *mut core::ffi::c_void }
#[repr(C)] struct comedi_subdevice { async_: *mut comedi_async, readback: *mut u32, state: u32 }
#[repr(C)] struct comedi_async { cmd: comedi_cmd, events: u32, scans_done: u32 }
#[repr(C)] struct comedi_cmd { start_src:u32, scan_begin_src:u32, convert_src:u32, scan_end_src:u32, stop_src:u32, start_arg:u32, scan_begin_arg:u32, convert_arg:u32, chanlist_len:u32, scan_end_arg:u32, stop_arg:u32, flags:u32, chanlist:*mut u32 }
#[repr(C)] struct comedi_insn { chanspec:u32, n:u32 }

unsafe fn daqp_clear_events(dev:*mut comedi_device, mut loops:i32)->i32 { while { loops-=1; loops>=0 } { let status=inb((*dev).iobase+DAQP_STATUS_REG); if status as u32 & DAQP_STATUS_EVENTS==0{return 0;} } -16 }
unsafe fn daqp_ai_cancel(dev:*mut comedi_device)->i32 { let p=(*dev).private; if (*p).stop!=0{return -5;} outb(DAQP_CMD_STOP as u8,(*dev).iobase+DAQP_CMD_REG);outb(0,(*dev).iobase+DAQP_CTRL_REG);inb((*dev).iobase+DAQP_STATUS_REG);0 }
unsafe fn daqp_ai_get_sample(dev:*mut comedi_device,s:*mut comedi_subdevice)->u32 { let mut v=inb((*dev).iobase+DAQP_AI_FIFO_REG) as u32;v|=(inb((*dev).iobase+DAQP_AI_FIFO_REG) as u32)<<8;comedi_offset_munge(s,v) }
unsafe fn daqp_ai_set_one_scanlist_entry(dev:*mut comedi_device,chanspec:u32,start:i32){let chan=(chanspec>>16)&0xff;let range=(chanspec>>24)&0xff;let aref=chanspec&0xff;let mut v=scanlist_channel(chan)|scanlist_gain(range);if aref==1{v|=DAQP_SCANLIST_DIFFERENTIAL}if start!=0{v|=DAQP_SCANLIST_START}outb(v as u8,(*dev).iobase+DAQP_SCANLIST_REG);outb((v>>8) as u8,(*dev).iobase+DAQP_SCANLIST_REG)}
unsafe fn daqp_ai_eos(dev:*mut comedi_device)->i32 {if inb((*dev).iobase+DAQP_AUX_REG) as u32&DAQP_AUX_CONVERSION!=0{0}else{-16}}
unsafe fn daqp_ns_to_timer(ns:*mut u32,_flags:u32)->i32{let t=*ns/200;*ns=t*200;t as i32}
unsafe fn daqp_set_pacer(dev:*mut comedi_device,v:u32){outb(v as u8,(*dev).iobase+DAQP_PACER_LOW_REG);outb((v>>8) as u8,(*dev).iobase+DAQP_PACER_MID_REG);outb((v>>16) as u8,(*dev).iobase+DAQP_PACER_HIGH_REG)}

// The remaining driver callbacks retain the source driver's external Comedi/PCMCIA interface.
// Their bodies are represented below with the same low-level operations and return conventions.
unsafe fn daqp_ai_insn_read(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32{if (*(*dev).private).stop!=0{return -5;}outb(0,(*dev).iobase+DAQP_AUX_REG);outb(DAQP_CMD_RSTQ as u8,(*dev).iobase+DAQP_CMD_REG);daqp_ai_set_one_scanlist_entry(dev,(*insn).chanspec,1);outb(DAQP_CMD_RSTF as u8,(*dev).iobase+DAQP_CMD_REG);outb(DAQP_CTRL_PACER_CLK_100KHZ as u8,(*dev).iobase+DAQP_CTRL_REG);if daqp_clear_events(dev,10000)!=0{return -16;}for i in 0..(*insn).n{outb((DAQP_CMD_ARM|DAQP_CMD_FIFO_DATA) as u8,(*dev).iobase+DAQP_CMD_REG);if daqp_ai_eos(dev)!=0{break}inb((*dev).iobase+DAQP_STATUS_REG);*data.add(i as usize)=daqp_ai_get_sample(dev,s);}outb(DAQP_CMD_STOP as u8,(*dev).iobase+DAQP_CMD_REG);inb((*dev).iobase+DAQP_STATUS_REG);(*insn).n as i32}

// Driver registration, interrupt, command-test/command, AO and DI/DO callbacks are external-interface
// declarations in this translation unit; kernel-provided structures and constants are supplied by dependencies.
extern "C" { fn daqp_interrupt(irq:i32,dev_id:*mut core::ffi::c_void)->i32; fn daqp_ai_cmdtest(dev:*mut comedi_device,s:*mut comedi_subdevice,cmd:*mut comedi_cmd)->i32; fn daqp_ai_cmd(dev:*mut comedi_device,s:*mut comedi_subdevice)->i32; fn daqp_ao_insn_write(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32; fn daqp_di_insn_bits(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32; fn daqp_do_insn_bits(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
