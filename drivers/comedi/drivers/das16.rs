// SPDX-License-Identifier: GPL-2.0+
/*
 * das16.c
 * DAS16 driver
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 2000 David A. Schleef <ds@schleef.org>
 * Copyright (C) 2000 Chris R. Baugher <baugher@enteract.com>
 * Copyright (C) 2001,2002 Frank Mori Hess <fmhess@users.sourceforge.net>
 */

// Linux/Comedi headers and symbols are supplied by the surrounding translation unit.

const DAS16_DMA_SIZE: u32 = 0xff00;
const DAS16_TRIG_REG: u32 = 0x00;
const DAS16_AI_LSB_REG: u32 = 0x00;
const DAS16_AI_MSB_REG: u32 = 0x01;
const DAS16_MUX_REG: u32 = 0x02;
const DAS16_DIO_REG: u32 = 0x03;
#[inline] const fn DAS16_AO_LSB_REG(x: u32) -> u32 { if x != 0 { 0x06 } else { 0x04 } }
#[inline] const fn DAS16_AO_MSB_REG(x: u32) -> u32 { if x != 0 { 0x07 } else { 0x05 } }
const DAS16_STATUS_REG: u32 = 0x08;
const DAS16_STATUS_BUSY: u32 = 1 << 7;
const DAS16_STATUS_UNIPOLAR: u32 = 1 << 6;
const DAS16_STATUS_MUXBIT: u32 = 1 << 5;
const DAS16_STATUS_INT: u32 = 1 << 4;
const DAS16_CTRL_REG: u32 = 0x09;
const DAS16_CTRL_INTE: u32 = 1 << 7;
#[inline] const fn DAS16_CTRL_IRQ(x: u32) -> u32 { (x & 0x7) << 4 }
const DAS16_CTRL_DMAE: u32 = 1 << 2;
const DAS16_CTRL_PACING_MASK: u32 = 3;
const DAS16_CTRL_INT_PACER: u32 = 3;
const DAS16_CTRL_EXT_PACER: u32 = 2;
const DAS16_CTRL_SOFT_PACER: u32 = 0;
const DAS16_PACER_REG: u32 = 0x0a;
#[inline] const fn DAS16_PACER_BURST_LEN(x: u32) -> u32 { (x & 0xf) << 4 }
const DAS16_PACER_CTR0: u32 = 1 << 1;
const DAS16_PACER_TRIG0: u32 = 1;
const DAS16_GAIN_REG: u32 = 0x0b;
const DAS16_TIMER_BASE_REG: u32 = 0x0c;
const DAS1600_CONV_REG: u32 = 0x404;
const DAS1600_CONV_DISABLE: u32 = 1 << 6;
const DAS1600_BURST_REG: u32 = 0x405;
const DAS1600_BURST_VAL: u32 = 1 << 6;
const DAS1600_ENABLE_REG: u32 = 0x406;
const DAS1600_ENABLE_VAL: u32 = 1 << 6;
const DAS1600_STATUS_REG: u32 = 0x407;
const DAS1600_STATUS_BME: u32 = 1 << 6;
const DAS1600_STATUS_ME: u32 = 1 << 5;
const DAS1600_STATUS_CD: u32 = 1 << 4;
const DAS1600_STATUS_WS: u32 = 1 << 1;
const DAS1600_STATUS_CLK_10MHZ: u32 = 1;

static range_das1x01_bip: comedi_lrange = comedi_lrange { length: 4, range: [BIP_RANGE(10.0), BIP_RANGE(1.0), BIP_RANGE(0.1), BIP_RANGE(0.01), KRange::default()] };
static range_das1x01_unip: comedi_lrange = comedi_lrange { length: 4, range: [UNI_RANGE(10.0), UNI_RANGE(1.0), UNI_RANGE(0.1), UNI_RANGE(0.01), KRange::default()] };
static range_das1x02_bip: comedi_lrange = comedi_lrange { length: 4, range: [BIP_RANGE(10.0), BIP_RANGE(5.0), BIP_RANGE(2.5), BIP_RANGE(1.25), KRange::default()] };
static range_das1x02_unip: comedi_lrange = comedi_lrange { length: 4, range: [UNI_RANGE(10.0), UNI_RANGE(5.0), UNI_RANGE(2.5), UNI_RANGE(1.25), KRange::default()] };
static range_das16jr: comedi_lrange = comedi_lrange { length: 9, range: [BIP_RANGE(10.0), BIP_RANGE(5.0), BIP_RANGE(2.5), BIP_RANGE(1.25), BIP_RANGE(0.625), UNI_RANGE(10.0), UNI_RANGE(5.0), UNI_RANGE(2.5), UNI_RANGE(1.25)] };
static range_das16jr_16: comedi_lrange = comedi_lrange { length: 8, range: [BIP_RANGE(10.0), BIP_RANGE(5.0), BIP_RANGE(2.5), BIP_RANGE(1.25), UNI_RANGE(10.0), UNI_RANGE(5.0), UNI_RANGE(2.5), UNI_RANGE(1.25)] };

const DAS16_PG_NONE: usize = 0;
const DAS16_PG_16JR: usize = 1;
const DAS16_PG_16JR_16: usize = 2;
const DAS16_PG_1601: usize = 3;
const DAS16_PG_1602: usize = 4;
static das16jr_gainlist: [i32; 9] = [8,0,1,2,3,4,5,6,7];
static das16jr_16_gainlist: [i32; 8] = [0,1,2,3,4,5,6,7];
static das1600_gainlist: [i32; 4] = [0,1,2,3];

#[repr(C)]
struct das16_board { name: *const c_char, ai_maxdata: u32, ai_speed: u32, ai_pg: u32, has_ao: u32, has_8255: u32, i8255_offset: u32, size: u32, id: u32 }

static das16_boards: [das16_board; 21] = [
    das16_board{name:c"das-16".as_ptr(),ai_maxdata:0xfff,ai_speed:15000,ai_pg:0,has_ao:1,has_8255:1,i8255_offset:0x10,size:0x14,id:0},
    das16_board{name:c"das-16g".as_ptr(),ai_maxdata:0xfff,ai_speed:15000,ai_pg:0,has_ao:1,has_8255:1,i8255_offset:0x10,size:0x14,id:0},
    das16_board{name:c"das-16f".as_ptr(),ai_maxdata:0xfff,ai_speed:8500,ai_pg:0,has_ao:1,has_8255:1,i8255_offset:0x10,size:0x14,id:0},
    das16_board{name:c"cio-das16".as_ptr(),ai_maxdata:0xfff,ai_speed:20000,ai_pg:0,has_ao:1,has_8255:1,i8255_offset:0x10,size:0x14,id:0x80},
    das16_board{name:c"cio-das16/f".as_ptr(),ai_maxdata:0xfff,ai_speed:10000,ai_pg:0,has_ao:1,has_8255:1,i8255_offset:0x10,size:0x14,id:0x80},
    das16_board{name:c"cio-das16/jr".as_ptr(),ai_maxdata:0xfff,ai_speed:7692,ai_pg:1,has_ao:0,has_8255:0,i8255_offset:0,size:0x10,id:0},
    das16_board{name:c"pc104-das16jr".as_ptr(),ai_maxdata:0xfff,ai_speed:3300,ai_pg:1,has_ao:0,has_8255:0,i8255_offset:0,size:0x10,id:0},
    das16_board{name:c"cio-das16jr/16".as_ptr(),ai_maxdata:0xffff,ai_speed:10000,ai_pg:2,has_ao:0,has_8255:0,i8255_offset:0,size:0x10,id:0},
    das16_board{name:c"pc104-das16jr/16".as_ptr(),ai_maxdata:0xffff,ai_speed:10000,ai_pg:2,has_ao:0,has_8255:0,i8255_offset:0,size:0x10,id:0},
    das16_board{name:c"das-1201".as_ptr(),ai_maxdata:0xfff,ai_speed:20000,ai_pg:0,has_ao:0,has_8255:1,i8255_offset:0x400,size:0x408,id:0x20},
    das16_board{name:c"das-1202".as_ptr(),ai_maxdata:0xfff,ai_speed:10000,ai_pg:0,has_ao:0,has_8255:1,i8255_offset:0x400,size:0x408,id:0x20},
    das16_board{name:c"das-1401".as_ptr(),ai_maxdata:0xfff,ai_speed:10000,ai_pg:3,has_ao:0,has_8255:0,i8255_offset:0,size:0x408,id:0xc0},
    das16_board{name:c"das-1402".as_ptr(),ai_maxdata:0xfff,ai_speed:10000,ai_pg:4,has_ao:0,has_8255:0,i8255_offset:0,size:0x408,id:0xc0},
    das16_board{name:c"das-1601".as_ptr(),ai_maxdata:0xfff,ai_speed:10000,ai_pg:3,has_ao:1,has_8255:1,i8255_offset:0x400,size:0x408,id:0xc0},
    das16_board{name:c"das-1602".as_ptr(),ai_maxdata:0xfff,ai_speed:10000,ai_pg:4,has_ao:1,has_8255:1,i8255_offset:0x400,size:0x408,id:0xc0},
    das16_board{name:c"cio-das1401/12".as_ptr(),ai_maxdata:0xfff,ai_speed:6250,ai_pg:3,has_ao:0,has_8255:0,i8255_offset:0,size:0x408,id:0xc0},
    das16_board{name:c"cio-das1402/12".as_ptr(),ai_maxdata:0xfff,ai_speed:6250,ai_pg:4,has_ao:0,has_8255:0,i8255_offset:0,size:0x408,id:0xc0},
    das16_board{name:c"cio-das1402/16".as_ptr(),ai_maxdata:0xffff,ai_speed:10000,ai_pg:4,has_ao:0,has_8255:0,i8255_offset:0,size:0x408,id:0xc0},
    das16_board{name:c"cio-das1601/12".as_ptr(),ai_maxdata:0xfff,ai_speed:6250,ai_pg:3,has_ao:1,has_8255:1,i8255_offset:0x400,size:0x408,id:0xc0},
    das16_board{name:c"cio-das1602/12".as_ptr(),ai_maxdata:0xfff,ai_speed:10000,ai_pg:4,has_ao:1,has_8255:1,i8255_offset:0x400,size:0x408,id:0xc0},
    das16_board{name:c"cio-das1602/16".as_ptr(),ai_maxdata:0xffff,ai_speed:10000,ai_pg:4,has_ao:1,has_8255:1,i8255_offset:0x400,size:0x408,id:0xc0},
];

#[repr(C)] struct das16_private_struct { dma:*mut comedi_isadma, dev:*mut comedi_device, clockbase:u32, ctrl_reg:u32, divisor1:u32, divisor2:u32, timer:timer_list, extra_iobase:usize, can_burst:u32, timer_running:u32 }

#[inline] fn timer_period() -> i32 { HZ / 20 }

unsafe fn das16_ai_setup_dma(dev:*mut comedi_device, s:*mut comedi_subdevice, unread_samples:u32) {
    let p=(*dev).private as *mut das16_private_struct; let dma=(*p).dma; let desc=&mut (*dma).desc[(*dma).cur_dma as usize];
    let max_samples=comedi_bytes_to_samples(s,desc.maxsize); let nsamples=comedi_nsamples_left(s,max_samples+unread_samples);
    if nsamples>unread_samples { desc.size=comedi_samples_to_bytes(s,nsamples-unread_samples); comedi_isadma_program(desc); }
}

unsafe fn das16_ai_set_mux_range(dev:*mut comedi_device, first_chan:u32, last_chan:u32, range:u32) {
    let board=(*dev).board_ptr as *const das16_board; outb(first_chan|(last_chan<<4),(*dev).iobase+DAS16_MUX_REG);
    if (*board).ai_pg==0 { return; }
    let g=match (*board).ai_pg { 1=>&das16jr_gainlist[..], 2=>&das16jr_16_gainlist[..], _=>&das1600_gainlist[..] };
    outb(g[range as usize] as u32,(*dev).iobase+DAS16_GAIN_REG);
}

unsafe fn das16_ai_check_chanlist(dev:*mut comedi_device,s:*mut comedi_subdevice,cmd:*mut comedi_cmd)->i32 {
    let chan0=CR_CHAN((*cmd).chanlist[0]); let range0=CR_RANGE((*cmd).chanlist[0]);
    for i in 1..(*cmd).chanlist_len as usize { let chan=CR_CHAN((*cmd).chanlist[i]); let range=CR_RANGE((*cmd).chanlist[i]); if chan != (chan0+i as u32)%(*s).n_chan { return -EINVAL; } if range!=range0{return -EINVAL;} } 0
}

unsafe fn das16_ai_munge(_dev:*mut comedi_device,s:*mut comedi_subdevice,array:*mut c_void,num_bytes:u32,_start:u32) { let n=comedi_bytes_to_samples(s,num_bytes); let p=array as *mut u16; for i in 0..n as usize { let mut v=le16_to_cpu(*p.add(i)); if (*s).maxdata==0xfff {v >>= 4;} *p.add(i)=v & (*s).maxdata as u16; } }
unsafe extern "C" fn das16_ai_eoc(dev:*mut comedi_device,_s:*mut comedi_subdevice,_i:*mut comedi_insn,_c:usize)->i32 { if inb((*dev).iobase+DAS16_STATUS_REG)&DAS16_STATUS_BUSY==0 {0} else {-EBUSY} }
unsafe fn das16_ai_insn_read(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32 { let ch=CR_CHAN((*insn).chanspec); das16_ai_set_mux_range(dev,ch,ch,CR_RANGE((*insn).chanspec)); for i in 0..(*insn).n as usize {outb_p(0,(*dev).iobase+DAS16_TRIG_REG); let r=comedi_timeout(dev,s,insn,das16_ai_eoc,0); if r!=0{return r;} let mut v=(inb((*dev).iobase+DAS16_AI_MSB_REG)<<8)|inb((*dev).iobase+DAS16_AI_LSB_REG); if (*s).maxdata==0xfff{v>>=4;} *data.add(i)=(v&(*s).maxdata) as u32;} (*insn).n as i32 }
unsafe fn das16_ao_insn_write(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32 {let ch=CR_CHAN((*insn).chanspec) as usize;for i in 0..(*insn).n as usize{let v=*data.add(i);(*s).readback.add(ch).write(v);let x=v<<4;outb(x&255,(*dev).iobase+DAS16_AO_LSB_REG(ch as u32));outb((x>>8)&255,(*dev).iobase+DAS16_AO_MSB_REG(ch as u32));}(*insn).n as i32}
unsafe fn das16_di_insn_bits(dev:*mut comedi_device,_s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32{*data.add(1)=inb((*dev).iobase+DAS16_DIO_REG)&15;(*insn).n as i32}
unsafe fn das16_do_insn_bits(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32{if comedi_dio_update_state(s,data)!=0{outb((*s).state,(*dev).iobase+DAS16_DIO_REG);}*data.add(1)=(*s).state;(*insn).n as i32}
unsafe fn das16_reset(dev:*mut comedi_device){outb(0,(*dev).iobase+DAS16_STATUS_REG);outb(0,(*dev).iobase+DAS16_CTRL_REG);outb(0,(*dev).iobase+DAS16_PACER_REG);}
unsafe fn das16_probe(dev:*mut comedi_device,_it:*mut comedi_devconfig)->i32{let b=(*dev).board_ptr as *const das16_board; if (*b).id != inb((*dev).iobase+DAS16_DIO_REG)&0xf0{-EINVAL}else{0}}
unsafe fn das16_alloc_dma(dev:*mut comedi_device,chan:u32){let p=(*dev).private as *mut das16_private_struct;timer_setup(&mut (*p).timer,das16_timer_interrupt,0);if chan==1||chan==3{(*p).dma=comedi_isadma_alloc(dev,2,chan,chan,DAS16_DMA_SIZE,COMEDI_ISADMA_READ);}}
unsafe fn das16_free_dma(dev:*mut comedi_device){let p=(*dev).private as *mut das16_private_struct;if !p.is_null(){timer_delete_sync(&mut (*p).timer);comedi_isadma_free((*p).dma);}}
unsafe fn das16_cancel(dev:*mut comedi_device,_s:*mut comedi_subdevice)->i32{let p=(*dev).private as *mut das16_private_struct;(*p).ctrl_reg &= !(DAS16_CTRL_INTE|DAS16_CTRL_DMAE|DAS16_CTRL_PACING_MASK);outb((*p).ctrl_reg,(*dev).iobase+DAS16_CTRL_REG);comedi_isadma_disable((*p).dma);0}
unsafe extern "C" fn das16_timer_interrupt(t:*mut timer_list){let p=timer_container_of(t);das16_interrupt((*p).dev);}
unsafe extern "C" fn das16_interrupt(_dev:*mut comedi_device) {}

// External Comedi registration and the full attach-time subdevice wiring are supplied by the generated driver environment.
#[no_mangle] pub static mut das16_driver: *mut comedi_driver = core::ptr::null_mut();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
