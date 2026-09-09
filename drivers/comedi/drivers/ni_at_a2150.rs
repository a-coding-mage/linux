// SPDX-License-Identifier: GPL-2.0+
/*
 * Comedi driver for National Instruments AT-A2150 boards
 * Copyright (C) 2001, 2002 Frank Mori Hess <fmhess@users.sourceforge.net>
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 2000 David A. Schleef <ds@schleef.org>
 */

/* Driver: ni_at_a2150; Description: National Instruments AT-A2150 */

const A2150_DMA_BUFFER_SIZE: u32 = 0xff00;
const CONFIG_REG: usize = 0x0;
const CHANNEL_MASK: u32 = 0x7;
const CLOCK_MASK: u32 = 0xf << 3;
const ENABLE0_BIT: u32 = 0x80;
const ENABLE1_BIT: u32 = 0x100;
const AC0_BIT: u32 = 0x200;
const AC1_BIT: u32 = 0x400;
const APD_BIT: u32 = 0x800;
const DPD_BIT: u32 = 0x1000;
const TRIGGER_REG: usize = 0x2;
const POST_TRIGGER_BITS: u32 = 0x2;
const DELAY_TRIGGER_BITS: u32 = 0x3;
const HW_TRIG_EN: u32 = 0x10;
const FIFO_START_REG: usize = 0x6;
const FIFO_RESET_REG: usize = 0x8;
const FIFO_DATA_REG: usize = 0xa;
const DMA_TC_CLEAR_REG: usize = 0xe;
const STATUS_REG: usize = 0x12;
const FNE_BIT: u32 = 0x1;
const OVFL_BIT: u32 = 0x8;
const EDAQ_BIT: u32 = 0x10;
const DCAL_BIT: u32 = 0x20;
const INTR_BIT: u32 = 0x40;
const DMA_TC_BIT: u32 = 0x80;
const IRQ_DMA_CNTRL_REG: usize = 0x12;
const DMA_EN_BIT: u32 = 0x8;
const FIFO_INTR_EN_BIT: u32 = 0x100;
const FIFO_INTR_FHF_BIT: u32 = 0x200;
const DMA_INTR_EN_BIT: u32 = 0x800;
const DMA_DEM_EN_BIT: u32 = 0x1000;
const I8253_BASE_REG: usize = 0x14;

#[inline] fn channel_bits(x: u32) -> u32 { x & 0x7 }
#[inline] fn clock_select_bits(x: u32) -> u32 { (x & 0x3) << 3 }
#[inline] fn clock_divisor_bits(x: u32) -> u32 { (x & 0x3) << 5 }
#[inline] fn id_bits(x: u32) -> u32 { (x >> 8) & 0x3 }
#[inline] fn dma_chan_bits(x: u32) -> u32 { x & 0x7 }
#[inline] fn irq_lvl_bits(x: u32) -> u32 { (x & 0xf) << 4 }

#[repr(C)]
pub struct a2150_board { pub name: *const i8, pub clock: [i32; 4], pub num_clocks: i32, pub ai_speed: i32 }

static range_a2150: comedi_lrange = comedi_lrange { length: 1, range: [BIP_RANGE(2.828)] };
enum { a2150_c, a2150_s }
static a2150_boards: [a2150_board; 2] = [
    a2150_board { name: b"at-a2150c\0" as *const u8 as *const i8, clock: [31250, 22676, 20833, 19531], num_clocks: 4, ai_speed: 19531 },
    a2150_board { name: b"at-a2150s\0" as *const u8 as *const i8, clock: [62500, 50000, 41667, 0], num_clocks: 3, ai_speed: 41667 },
];

#[repr(C)] pub struct a2150_private { pub dma: *mut comedi_isadma, pub count: u32, pub irq_dma_bits: i32, pub config_bits: i32 }

unsafe fn a2150_interrupt(_irq: i32, d: *mut core::ffi::c_void) -> irqreturn_t {
    let dev = d as *mut comedi_device;
    if !(*dev).attached { return IRQ_HANDLED; }
    let p = (*dev).private as *mut a2150_private;
    let dma = (*p).dma;
    let desc = &mut (*dma).desc[0];
    let s = (*dev).read_subdev;
    let async_ = (*s).async_;
    let cmd = &mut (*async_).cmd;
    let buf = desc.virt_addr as *mut u16;
    let status = inw((*dev).iobase + STATUS_REG);
    if status & INTR_BIT == 0 { return IRQ_NONE; }
    if status & OVFL_BIT != 0 { (*async_).events |= COMEDI_CB_ERROR; comedi_handle_events(dev, s); }
    if status & DMA_TC_BIT == 0 { (*async_).events |= COMEDI_CB_ERROR; comedi_handle_events(dev, s); return IRQ_HANDLED; }
    let residue = comedi_isadma_disable(desc.chan);
    let max_points = comedi_bytes_to_samples(s, desc.size);
    let mut num_points = max_points - comedi_bytes_to_samples(s, residue);
    if (*p).count < num_points && cmd.stop_src == TRIG_COUNT { num_points = (*p).count; }
    let mut leftover = if cmd.stop_src == TRIG_NONE { comedi_bytes_to_samples(s, desc.size) } else if (*p).count > max_points { ((*p).count - max_points).min(max_points) } else { 0 };
    if residue != 0 { leftover = 0; }
    for i in 0..num_points { let mut dpnt = *buf.add(i as usize); dpnt ^= 0x8000; comedi_buf_write_samples(s, &mut dpnt, 1); if cmd.stop_src == TRIG_COUNT { (*p).count -= 1; if (*p).count == 0 { (*async_).events |= COMEDI_CB_EOA; break; } } }
    if leftover != 0 { desc.size = comedi_samples_to_bytes(s, leftover); comedi_isadma_program(desc); }
    comedi_handle_events(dev, s); outw(0, (*dev).iobase + DMA_TC_CLEAR_REG); IRQ_HANDLED
}

unsafe fn a2150_cancel(dev: *mut comedi_device, _s: *mut comedi_subdevice) -> i32 { let p = (*dev).private as *mut a2150_private; (*p).irq_dma_bits &= !(DMA_INTR_EN_BIT as i32) & !(DMA_EN_BIT as i32); outw((*p).irq_dma_bits as u16, (*dev).iobase + IRQ_DMA_CNTRL_REG); comedi_isadma_disable((*(*p).dma).desc[0].chan); outw(0, (*dev).iobase + FIFO_RESET_REG); 0 }

unsafe fn a2150_get_timing(dev: *mut comedi_device, period: *mut u32, flags: u32) -> i32 {
    let board = (*dev).board_ptr as *const a2150_board; let p = (*dev).private as *mut a2150_private;
    let mut lub_shift = 3; let mut lub_index = 0; let mut lub = (*board).clock[0] * (1 << lub_shift);
    let mut glb_shift = 0; let mut glb_index = (*board).num_clocks - 1; let mut glb = (*board).clock[glb_index as usize];
    if *period < glb as u32 { *period = glb as u32; } if *period > lub as u32 { *period = lub as u32; }
    for i in 0..4 { for j in 0..(*board).num_clocks { let temp = (*board).clock[j as usize] * (1 << i); if temp < lub && temp >= *period as i32 { lub_shift=i; lub_index=j; lub=temp; } if temp > glb && temp <= *period as i32 { glb_shift=i; glb_index=j; glb=temp; } } }
    match flags & CMDF_ROUND_MASK { CMDF_ROUND_UP => *period=lub as u32, CMDF_ROUND_DOWN => *period=glb as u32, _ => { *period=if lub as u32-*period < *period-glb as u32 { lub as u32 } else { glb as u32 }; } }
    (*p).config_bits &= !(CLOCK_MASK as i32); if *period == lub as u32 { (*p).config_bits |= (clock_select_bits(lub_index as u32)|clock_divisor_bits(lub_shift as u32)) as i32; } else { (*p).config_bits |= (clock_select_bits(glb_index as u32)|clock_divisor_bits(glb_shift as u32)) as i32; } 0
}

unsafe fn a2150_set_chanlist(dev:*mut comedi_device,start:u32,num:u32)->i32 { let p=(*dev).private as *mut a2150_private; if start+num>4{return -1}; (*p).config_bits &= !(CHANNEL_MASK as i32); match num {1=>(*p).config_bits|=channel_bits(0x4+start) as i32,2=>{if start==0{(*p).config_bits|=2}else if start==2{(*p).config_bits|=3}else{return -1}},4=>(*p).config_bits|=1,_=>return -1};0 }

unsafe fn a2150_ai_cmdtest(dev:*mut comedi_device,s:*mut comedi_subdevice,cmd:*mut comedi_cmd)->i32 { let board=(*dev).board_ptr as *const a2150_board;let mut err=0;err|=comedi_check_trigger_src(&mut (*cmd).start_src,TRIG_NOW|TRIG_EXT);err|=comedi_check_trigger_src(&mut (*cmd).scan_begin_src,TRIG_TIMER);err|=comedi_check_trigger_src(&mut (*cmd).convert_src,TRIG_NOW);err|=comedi_check_trigger_src(&mut (*cmd).scan_end_src,TRIG_COUNT);err|=comedi_check_trigger_src(&mut (*cmd).stop_src,TRIG_COUNT|TRIG_NONE);if err!=0{return 1;}err|=comedi_check_trigger_is_unique((*cmd).start_src);err|=comedi_check_trigger_is_unique((*cmd).stop_src);if err!=0{return 2;}err|=comedi_check_trigger_arg_is(&mut (*cmd).start_arg,0);if (*cmd).convert_src==TRIG_TIMER{err|=comedi_check_trigger_arg_min(&mut (*cmd).convert_arg,(*board).ai_speed as u32);}err|=comedi_check_trigger_arg_min(&mut (*cmd).chanlist_len,1);err|=comedi_check_trigger_arg_is(&mut (*cmd).scan_end_arg,(*cmd).chanlist_len);if (*cmd).stop_src==TRIG_COUNT{err|=comedi_check_trigger_arg_min(&mut (*cmd).stop_arg,1);}else{err|=comedi_check_trigger_arg_is(&mut (*cmd).stop_arg,0);}if err!=0{return 3;}if (*cmd).scan_begin_src==TRIG_TIMER{let mut arg=(*cmd).scan_begin_arg;a2150_get_timing(dev,&mut arg,(*cmd).flags);err|=comedi_check_trigger_arg_is(&mut (*cmd).scan_begin_arg,arg);}if err!=0{return 4;}if !(*cmd).chanlist.is_null()&&(*cmd).chanlist_len>0{err|=a2150_ai_check_chanlist(dev,s,cmd);}if err!=0{return 5;}0 }

unsafe fn a2150_ai_cmd(dev:*mut comedi_device,s:*mut comedi_subdevice)->i32 {let p=(*dev).private as *mut a2150_private;let desc=&mut (*(*p).dma).desc[0];let async_=(*s).async_;let cmd=&mut (*async_).cmd;let old=(*p).config_bits;if cmd.flags&CMDF_PRIORITY!=0{return -1;}outw(0,(*dev).iobase+FIFO_RESET_REG);if a2150_set_chanlist(dev,CR_CHAN(cmd.chanlist[0]),cmd.chanlist_len)<0{return -1;}if CR_AREF(cmd.chanlist[0])==AREF_OTHER{(*p).config_bits|=AC0_BIT as i32}else{(*p).config_bits&=!(AC0_BIT as i32)}if CR_AREF(cmd.chanlist[2])==AREF_OTHER{(*p).config_bits|=AC1_BIT as i32}else{(*p).config_bits&=!(AC1_BIT as i32)}a2150_get_timing(dev,&mut cmd.scan_begin_arg,cmd.flags);outw((*p).config_bits as u16,(*dev).iobase+CONFIG_REG);(*p).count=cmd.stop_arg*cmd.chanlist_len;comedi_isadma_disable(desc.chan);desc.size=comedi_bytes_per_sample(s)*cmd.chanlist_len*333333333/cmd.scan_begin_arg;if desc.size>desc.maxsize{desc.size=desc.maxsize;}if desc.size<comedi_bytes_per_sample(s){desc.size=comedi_bytes_per_sample(s);}desc.size-=desc.size%comedi_bytes_per_sample(s);comedi_isadma_program(desc);outw(0,(*dev).iobase+DMA_TC_CLEAR_REG);(*p).irq_dma_bits|=(DMA_INTR_EN_BIT|DMA_EN_BIT) as i32;outw((*p).irq_dma_bits as u16,(*dev).iobase+IRQ_DMA_CNTRL_REG);comedi_8254_load((*dev).pacer,2,72,I8254_MODE0|I8254_BINARY);let mut trig=if cmd.start_src==TRIG_NOW&&(old as u32&CLOCK_MASK)!=((*p).config_bits as u32&CLOCK_MASK){DELAY_TRIGGER_BITS}else{POST_TRIGGER_BITS};if cmd.start_src==TRIG_EXT{trig|=HW_TRIG_EN;}outw(trig as u16,(*dev).iobase+TRIGGER_REG);if cmd.start_src==TRIG_NOW{outw(0,(*dev).iobase+FIFO_START_REG);}0}

unsafe fn a2150_ai_eoc(dev:*mut comedi_device,_s:*mut comedi_subdevice,_i:*mut comedi_insn,_c:usize)->i32{if inw((*dev).iobase+STATUS_REG)&FNE_BIT!=0{0}else{-EBUSY}}
unsafe fn a2150_ai_rinsn(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32{let p=(*dev).private as *mut a2150_private;outw(0,(*dev).iobase+FIFO_RESET_REG);if a2150_set_chanlist(dev,CR_CHAN((*insn).chanspec),1)<0{return -1;}(*p).config_bits&=!((AC0_BIT|AC1_BIT) as i32);outw((*p).config_bits as u16,(*dev).iobase+CONFIG_REG);(*p).irq_dma_bits&=!(DMA_INTR_EN_BIT as i32)&!(DMA_EN_BIT as i32);outw((*p).irq_dma_bits as u16,(*dev).iobase+IRQ_DMA_CNTRL_REG);outw(0,(*dev).iobase+TRIGGER_REG);outw(0,(*dev).iobase+FIFO_START_REG);for _ in 0..36{let r=comedi_timeout(dev,s,insn,a2150_ai_eoc,0);if r!=0{return r;}inw((*dev).iobase+FIFO_DATA_REG);}for n in 0..(*insn).n{let r=comedi_timeout(dev,s,insn,a2150_ai_eoc,0);if r!=0{return r;}*data.add(n as usize)=(inw((*dev).iobase+FIFO_DATA_REG)^0x8000) as u32;}outw(0,(*dev).iobase+FIFO_RESET_REG);(*insn).n as i32}

unsafe fn a2150_free_dma(dev:*mut comedi_device){let p=(*dev).private as *mut a2150_private;if !p.is_null(){comedi_isadma_free((*p).dma);}}
unsafe fn a2150_probe(dev:*mut comedi_device)->*const a2150_board{let id=id_bits(inw((*dev).iobase+STATUS_REG) as u32) as usize;if id>=a2150_boards.len(){core::ptr::null()}else{&a2150_boards[id]}}

unsafe fn a2150_attach(dev:*mut comedi_device,it:*mut comedi_devconfig)->i32{let p=comedi_alloc_devpriv(dev,core::mem::size_of::<a2150_private>()) as *mut a2150_private;if p.is_null(){return -ENOMEM;}let ret=comedi_check_request_region(dev,(*it).options[0],0x1c,0,0x3ff,32);if ret!=0{return ret;}let board=a2150_probe(dev);if board.is_null(){return -ENODEV;}(*dev).board_ptr=board as *mut _;(*dev).board_name=(*board).name;(*p).config_bits=0;outw((*p).irq_dma_bits as u16,(*dev).iobase+IRQ_DMA_CNTRL_REG);outw_p((DPD_BIT|APD_BIT) as u16,(*dev).iobase+CONFIG_REG);outw_p(DPD_BIT as u16,(*dev).iobase+CONFIG_REG);outw(0,(*dev).iobase+CONFIG_REG);for i in 0..2000{if inw((*dev).iobase+STATUS_REG)&DCAL_BIT==0{(*p).config_bits|=(ENABLE0_BIT|ENABLE1_BIT) as i32;outw((*p).config_bits as u16,(*dev).iobase+CONFIG_REG);return 0;}usleep_range(1000,3000);if i==1999{return -ETIME;}}0}
unsafe fn a2150_detach(dev:*mut comedi_device){if (*dev).iobase!=0{outw((APD_BIT|DPD_BIT) as u16,(*dev).iobase+CONFIG_REG);}a2150_free_dma(dev);comedi_legacy_detach(dev);}

// External declarations are supplied by the Linux kernel/Comedi translation.
unsafe fn a2150_ai_check_chanlist(dev:*mut comedi_device,s:*mut comedi_subdevice,cmd:*mut comedi_cmd)->i32 { let chan0=CR_CHAN((*cmd).chanlist[0]); let mut aref0=CR_AREF((*cmd).chanlist[0]); if (*cmd).chanlist_len==2&&(chan0==1||chan0==3){dev_dbg((*dev).class_dev,b"length 2 chanlist must be channels 0,1 or channels 2,3\0");return -EINVAL;} if (*cmd).chanlist_len==3{return -EINVAL;} for i in 1..(*cmd).chanlist_len {let chan=CR_CHAN((*cmd).chanlist[i]);let aref=CR_AREF((*cmd).chanlist[i]);if chan!=chan0+i{return -EINVAL;}if chan==2{aref0=aref;}if aref!=aref0{return -EINVAL;}} let _=s;0 }

/* External kernel and Comedi types/functions are supplied by the translated dependency set. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
