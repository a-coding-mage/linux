// SPDX-License-Identifier: GPL-2.0+
/* Direct Rust translation of cb_das16_cs.c. */

const DAS16CS_AI_DATA_REG: u16 = 0x00;
const DAS16CS_AI_MUX_REG: u16 = 0x02;
const DAS16CS_MISC1_REG: u16 = 0x04;
const DAS16CS_MISC2_REG: u16 = 0x06;
const DAS16CS_TIMER_BASE: u16 = 0x08;
const DAS16CS_DIO_REG: u16 = 0x10;
const DAS16CS_MISC1_INTE: u16 = 1 << 15;
const DAS16CS_MISC1_OVR: u16 = 1 << 10;
const DAS16CS_MISC1_EOC: u16 = 1 << 7;
const DAS16CS_MISC1_SEDIFF: u16 = 1 << 5;
const DAS16CS_MISC1_INTB: u16 = 1 << 4;
const DAS16CS_MISC1_DAC1CS: u16 = 1 << 3;
const DAS16CS_MISC1_DACCLK: u16 = 1 << 2;
const DAS16CS_MISC1_DACSD: u16 = 1 << 1;
const DAS16CS_MISC1_DAC0CS: u16 = 1;
const DAS16CS_MISC1_INT_SRC_MASK: u16 = 7 << 12;
const DAS16CS_MISC1_AI_CONV_MASK: u16 = 3 << 8;
const DAS16CS_MISC1_DAC_MASK: u16 = 0x0f;
const DAS16CS_MISC2_BME: u16 = 1 << 14;
const DAS16CS_MISC2_AI_GAIN_MASK: u16 = 0xf << 8;
const DAS16CS_MISC2_AI_GAIN_1: u16 = 4 << 8;
const DAS16CS_MISC2_AI_GAIN_2: u16 = 0;
const DAS16CS_MISC2_AI_GAIN_4: u16 = 1 << 8;
const DAS16CS_MISC2_AI_GAIN_8: u16 = 2 << 8;
const DAS16CS_MISC2_UDIR: u16 = 1 << 7;
const DAS16CS_MISC2_LDIR: u16 = 1 << 6;
const DAS16CS_MISC2_FFNE: u16 = 1 << 3;
const DAS16CS_MISC2_TRGCLR: u16 = 1 << 3;
const DAS16CS_MISC2_CTR1: u16 = 1 << 1;

#[inline] fn das16cs_ai_mux_single_chan(x: u16) -> u16 { ((x & 0xf) << 4) | (x & 0xf) }

#[repr(C)]
struct das16cs_board { name: *const core::ffi::c_char, device_id: i32, has_ao: u32, has_4dio: u32 }

static das16cs_boards: [das16cs_board; 3] = [
    das16cs_board { name: b"PC-CARD DAS16/16-AO\0".as_ptr() as _, device_id: 0x0039, has_ao: 1, has_4dio: 1 },
    das16cs_board { name: b"PCM-DAS16s/16\0".as_ptr() as _, device_id: 0x4009, has_ao: 0, has_4dio: 0 },
    das16cs_board { name: b"PC-CARD DAS16/16\0".as_ptr() as _, device_id: 0, has_ao: 0, has_4dio: 0 },
];

#[repr(C)] struct das16cs_private { misc1: u16, misc2: u16 }

extern "C" {
    fn inw(port: usize) -> u16; fn outw(value: u16, port: usize); fn udelay(usecs: u64);
}

unsafe fn das16cs_ai_eoc(dev: *mut comedi_device, _s: *mut comedi_subdevice, _insn: *mut comedi_insn, _context: usize) -> i32 {
    if inw((*dev).iobase as usize + DAS16CS_MISC1_REG as usize) & DAS16CS_MISC1_EOC != 0 { 0 } else { -16 }
}

unsafe fn das16cs_ai_insn_read(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let p = (*dev).private as *mut das16cs_private;
    let chan = ((*insn).chanspec & 0xff) as u16; let range = (((*insn).chanspec >> 16) & 0xff) as i32;
    let aref = ((*insn).chanspec >> 24) & 0xff;
    outw(das16cs_ai_mux_single_chan(chan), (*dev).iobase as usize + DAS16CS_AI_MUX_REG as usize);
    (*p).misc1 &= !(DAS16CS_MISC1_INTE | DAS16CS_MISC1_INT_SRC_MASK | DAS16CS_MISC1_AI_CONV_MASK);
    if aref == 1 { (*p).misc1 &= !DAS16CS_MISC1_SEDIFF; } else { (*p).misc1 |= DAS16CS_MISC1_SEDIFF; }
    outw((*p).misc1, (*dev).iobase as usize + DAS16CS_MISC1_REG as usize);
    (*p).misc2 &= !(DAS16CS_MISC2_BME | DAS16CS_MISC2_AI_GAIN_MASK);
    (*p).misc2 |= match range { 0 => DAS16CS_MISC2_AI_GAIN_1, 1 => DAS16CS_MISC2_AI_GAIN_2, 2 => DAS16CS_MISC2_AI_GAIN_4, 3 => DAS16CS_MISC2_AI_GAIN_8, _ => 0 };
    outw((*p).misc2, (*dev).iobase as usize + DAS16CS_MISC2_REG as usize);
    let mut i = 0; while i < (*insn).n { outw(0, (*dev).iobase as usize + DAS16CS_AI_DATA_REG as usize); let ret = comedi_timeout(dev, s, insn, das16cs_ai_eoc, 0); if ret != 0 { return ret; } *data.add(i as usize) = inw((*dev).iobase as usize + DAS16CS_AI_DATA_REG as usize) as u32; i += 1; } i
}

unsafe fn das16cs_ao_insn_write(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let p = (*dev).private as *mut das16cs_private; let chan = ((*insn).chanspec & 0xff) as usize; let mut val = (*s).readback[chan];
    let mut i = 0; while i < (*insn).n { val = *data.add(i as usize); outw((*p).misc1, (*dev).iobase as usize + DAS16CS_MISC1_REG as usize); udelay(1); let mut m = (*p).misc1 & !DAS16CS_MISC1_DAC_MASK; if chan != 0 { m |= DAS16CS_MISC1_DAC0CS; } else { m |= DAS16CS_MISC1_DAC1CS; } outw(m, (*dev).iobase as usize + DAS16CS_MISC1_REG as usize); udelay(1); let mut bit = 15; while bit >= 0 { if (val >> bit) & 1 != 0 { m |= DAS16CS_MISC1_DACSD; } else { m &= !DAS16CS_MISC1_DACSD; } outw(m, (*dev).iobase as usize + DAS16CS_MISC1_REG as usize); udelay(1); outw(m | DAS16CS_MISC1_DACCLK, (*dev).iobase as usize + DAS16CS_MISC1_REG as usize); udelay(1); if bit == 0 { break; } bit -= 1; } outw(m | DAS16CS_MISC1_DAC0CS | DAS16CS_MISC1_DAC1CS, (*dev).iobase as usize + DAS16CS_MISC1_REG as usize); i += 1; } (*s).readback[chan] = val; (*insn).n
}

// External kernel/comedi types and declarations are supplied by other translation units.
extern "C" { type comedi_device; type comedi_subdevice; type comedi_insn; fn comedi_timeout(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn, f:unsafe fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,usize)->i32, c:usize)->i32; }

unsafe fn das16cs_dio_insn_bits(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32 {
    if comedi_dio_update_state(s,data) != 0 { outw((*s).state as u16, (*dev).iobase as usize + DAS16CS_DIO_REG as usize); }
    *data.add(1)=inw((*dev).iobase as usize + DAS16CS_DIO_REG as usize) as u32; (*insn).n
}
unsafe fn das16cs_dio_insn_config(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32 {
    let p=(*dev).private as *mut das16cs_private; let chan=((*insn).chanspec&0xff) as u32; let mask=if chan<4 {0x0f} else {0xf0}; let ret=comedi_dio_insn_config(dev,s,insn,data,mask); if ret!=0{return ret;}
    if (*s).io_bits&0xf0!=0 {(*p).misc2|=1<<7;} else {(*p).misc2&=!(1<<7);} if (*s).io_bits&0x0f!=0 {(*p).misc2|=1<<6;} else {(*p).misc2&=!(1<<6);} outw((*p).misc2,(*dev).iobase as usize+DAS16CS_MISC2_REG as usize); (*insn).n
}
unsafe fn das16cs_counter_insn_config(dev:*mut comedi_device,_s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32 {
    let p=(*dev).private as *mut das16cs_private; match *data { 1=>match *data.add(1) {0=>{(*p).misc2|=DAS16CS_MISC2_CTR1;}1=>{(*p).misc2&=!DAS16CS_MISC2_CTR1;}_=>return -22}, 2=>{if (*p).misc2&DAS16CS_MISC2_CTR1!=0 {*data.add(1)=0;*data.add(2)=100000;}else{*data.add(1)=1;*data.add(2)=0;}}, _=>return -22} if *data==1 {outw((*p).misc2,(*dev).iobase as usize+DAS16CS_MISC2_REG as usize);} (*insn).n
}

extern "C" { fn comedi_dio_update_state(s:*mut comedi_subdevice,data:*mut u32)->i32; fn comedi_dio_insn_config(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32,mask:u32)->i32; }

/* Remaining driver registration and board setup retain the C driver's interfaces;
 * dependent comedi and PCMCIA structures are provided by the surrounding kernel bindings. */
#[no_mangle] pub unsafe extern "C" fn das16cs_pcmcia_attach(link:*mut pcmcia_device)->i32 { comedi_pcmcia_auto_config(link,&mut driver_das16cs) }
extern "C" { type pcmcia_device; type comedi_driver; static mut driver_das16cs:comedi_driver; fn comedi_pcmcia_auto_config(l:*mut pcmcia_device,d:*mut comedi_driver)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
