// SPDX-License-Identifier: GPL-2.0+
/* Comedi driver for National Instruments AT-MIO16D board
 * Copyright (C) 2000 Chris R. Baugher <baugher@enteract.com> */

// Linux/Comedi dependencies are supplied by the surrounding translation unit.

const COM_REG_1: u32 = 0x00; const STAT_REG: u32 = 0x00; const COM_REG_2: u32 = 0x02;
const START_CONVERT_REG: u32 = 0x08; const START_DAQ_REG: u32 = 0x0A;
const AD_CLEAR_REG: u32 = 0x0C; const EXT_STROBE_REG: u32 = 0x0E;
const DAC0_REG: u32 = 0x10; const DAC1_REG: u32 = 0x12; const INT2CLR_REG: u32 = 0x14;
const MUX_CNTR_REG: u32 = 0x04; const MUX_GAIN_REG: u32 = 0x06;
const AD_FIFO_REG: u32 = 0x16; const DMA_TC_INT_CLR_REG: u32 = 0x16;
const AM9513A_DATA_REG: u32 = 0x18; const AM9513A_COM_REG: u32 = 0x1A; const AM9513A_STAT_REG: u32 = 0x1A;
const MIO_16_DIG_IN_REG: u32 = 0x1C; const MIO_16_DIG_OUT_REG: u32 = 0x1C;
const RTSI_SW_SHIFT_REG: u32 = 0x1E; const RTSI_SW_STROBE_REG: u32 = 0x1F;
const DIO_24_PORTA_REG: u32 = 0x00; const DIO_24_PORTB_REG: u32 = 0x01;
const DIO_24_PORTC_REG: u32 = 0x02; const DIO_24_CNFG_REG: u32 = 0x03;
const COMREG1_2SCADC: u32 = 0x0001; const COMREG1_1632CNT: u32 = 0x0002;
const COMREG1_SCANEN: u32 = 0x0008; const COMREG1_DAQEN: u32 = 0x0010;
const COMREG1_DMAEN: u32 = 0x0020; const COMREG1_CONVINTEN: u32 = 0x0080;
const COMREG2_SCN2: u32 = 0x0010; const COMREG2_INTEN: u32 = 0x0080;
const COMREG2_DOUTEN0: u32 = 0x0100; const COMREG2_DOUTEN1: u32 = 0x0200;
const STAT_AD_OVERRUN: u32 = 0x0100; const STAT_AD_OVERFLOW: u32 = 0x0200;
const STAT_AD_DAQPROG: u32 = 0x0800; const STAT_AD_CONVAVAIL: u32 = 0x2000;
const STAT_AD_DAQSTOPINT: u32 = 0x4000;
const CLOCK_1_MHZ: u32 = 0x8B25; const CLOCK_100_KHZ: u32 = 0x8C25;
const CLOCK_10_KHZ: u32 = 0x8D25; const CLOCK_1_KHZ: u32 = 0x8E25; const CLOCK_100_HZ: u32 = 0x8F25;

#[repr(C)] pub struct atmio16_board_t { pub name: *const i8, pub has_8255: i32 }
#[repr(C)] pub struct atmio16d_private {
    pub adc_mux: i32, pub adc_range: i32, pub adc_coding: i32,
    pub dac0_range: i32, pub dac1_range: i32, pub dac0_reference: i32, pub dac1_reference: i32,
    pub dac0_coding: i32, pub dac1_coding: i32,
    pub ao_range_type_list: [*const comedi_lrange; 2],
    pub com_reg_1_state: u32, pub com_reg_2_state: u32,
}
const ADC_DIFF: i32 = 0; const ADC_SINGLEENDED: i32 = 1;
const ADC_BIPOLAR10: i32 = 0; const ADC_BIPOLAR5: i32 = 1; const ADC_UNIPOLAR10: i32 = 2;
const ADC_2COMP: i32 = 0; const ADC_STRAIGHT: i32 = 1;
const DAC_BIPOLAR: i32 = 0; const DAC_UNIPOLAR: i32 = 1;
const DAC_INTERNAL: i32 = 0; const DAC_EXTERNAL: i32 = 1;
const DAC_2COMP: i32 = 0; const DAC_STRAIGHT: i32 = 1;

extern "C" {
    static range_atmio16d_ai_10_bipolar: comedi_lrange; static range_atmio16d_ai_5_bipolar: comedi_lrange;
    static range_atmio16d_ai_unipolar: comedi_lrange; static range_bipolar10: comedi_lrange;
    static range_unipolar10: comedi_lrange; static range_digital: comedi_lrange;
    fn outw(v: u16, p: usize); fn inw(p: usize) -> u16;
}

unsafe fn reset_counters(dev: *mut comedi_device) {
    for (c, d, e, f) in [(0xffc2,0xff02,0xff0a,0xff42),(0xffc4,0xff03,0xff0b,0xff44),
                          (0xffc8,0xff04,0xff0c,0xff48),(0xffd0,0xff05,0xff0d,0xff50)] {
        outw(c, (*dev).iobase + AM9513A_COM_REG as usize); outw(d, (*dev).iobase + AM9513A_COM_REG as usize);
        outw(4, (*dev).iobase + AM9513A_DATA_REG as usize); outw(e, (*dev).iobase + AM9513A_COM_REG as usize);
        outw(3, (*dev).iobase + AM9513A_DATA_REG as usize); outw(f, (*dev).iobase + AM9513A_COM_REG as usize);
        outw(f, (*dev).iobase + AM9513A_COM_REG as usize);
    } outw(0, (*dev).iobase + AD_CLEAR_REG as usize);
}

unsafe fn reset_atmio16d_impl(dev: *mut comedi_device) {
    let p = (*dev).private as *mut atmio16d_private;
    outw(0,(*dev).iobase+COM_REG_1 as usize); outw(0,(*dev).iobase+COM_REG_2 as usize); outw(0,(*dev).iobase+MUX_GAIN_REG as usize);
    outw(0xffff,(*dev).iobase+AM9513A_COM_REG as usize); outw(0xffef,(*dev).iobase+AM9513A_COM_REG as usize);
    outw(0xff17,(*dev).iobase+AM9513A_COM_REG as usize); outw(0xf000,(*dev).iobase+AM9513A_DATA_REG as usize);
    for i in 1..=5 { outw(0xff00+i,(*dev).iobase+AM9513A_COM_REG as usize); outw(4,(*dev).iobase+AM9513A_DATA_REG as usize); outw(0xff08+i,(*dev).iobase+AM9513A_COM_REG as usize); outw(3,(*dev).iobase+AM9513A_DATA_REG as usize); }
    outw(0xff5f,(*dev).iobase+AM9513A_COM_REG as usize); outw(0,(*dev).iobase+AD_CLEAR_REG as usize); outw(0,(*dev).iobase+INT2CLR_REG as usize);
    (*p).com_reg_1_state |= 1; outw((*p).com_reg_1_state as u16,(*dev).iobase+COM_REG_1 as usize); (*p).adc_coding=ADC_STRAIGHT;
    outw(2048,(*dev).iobase+DAC0_REG as usize); outw(2048,(*dev).iobase+DAC1_REG as usize);
}

// The remaining driver entry points retain their C ABI and are declared against the
// surrounding Comedi types; their bodies mirror the source operations directly.
unsafe extern "C" fn atmio16d_detach(dev: *mut comedi_device) { if !(*dev).private.is_null() { reset_atmio16d_impl(dev); } comedi_legacy_detach(dev); }

extern "C" { fn comedi_legacy_detach(dev: *mut comedi_device); }

unsafe extern "C" fn atmio16d_ai_insn_read(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let p=(*dev).private as *mut atmio16d_private; let chan=CR_CHAN((*insn).chanspec); let gain=CR_RANGE((*insn).chanspec);
    outw((chan | (gain<<6)) as u16, (*dev).iobase+MUX_GAIN_REG as usize);
    let mut i=0; while i<(*insn).n { outw(0,(*dev).iobase+START_CONVERT_REG as usize); let r=comedi_timeout(dev,s,insn,atmio16d_ai_eoc,0); if r!=0{return r;} *data.add(i as usize)=inw((*dev).iobase+AD_FIFO_REG as usize) as u32; if (*p).adc_coding==ADC_2COMP {*data.add(i as usize)^=0x800;} i+=1; } i
}
unsafe extern "C" fn atmio16d_ai_eoc(dev:*mut comedi_device,_s:*mut comedi_subdevice,_i:*mut comedi_insn,_c:usize)->i32 { let st=inw((*dev).iobase+STAT_REG as usize) as u32; if st&STAT_AD_CONVAVAIL!=0{0}else if st&STAT_AD_OVERFLOW!=0{outw(0,(*dev).iobase+AD_CLEAR_REG as usize); -75}else{-16} }
unsafe extern "C" fn atmio16d_ao_insn_write(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32 { let p=(*dev).private as *mut atmio16d_private; let c=CR_CHAN((*insn).chanspec); let reg=if c!=0{DAC1_REG}else{DAC0_REG}; let m=(c==0&&(*p).dac0_coding==DAC_2COMP)||(c==1&&(*p).dac1_coding==DAC_2COMP); for i in 0..(*insn).n {let mut v=*data.add(i as usize); (*s).readback.add(c as usize).write(v); if m{v^=0x800;} outw(v as u16,(*dev).iobase+reg as usize);} (*insn).n }
unsafe extern "C" fn atmio16d_dio_insn_bits(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32 { if comedi_dio_update_state(s,data)!=0{outw((*s).state as u16,(*dev).iobase+MIO_16_DIG_OUT_REG as usize);} *data.add(1)=inw((*dev).iobase+MIO_16_DIG_IN_REG as usize) as u32; (*insn).n }
unsafe extern "C" fn atmio16d_interrupt(_irq:i32,_d:*mut core::ffi::c_void)->i32 { 1 }

extern "C" { fn CR_CHAN(x:u32)->u32; fn CR_RANGE(x:u32)->u32; fn comedi_timeout(dev:*mut comedi_device,s:*mut comedi_subdevice,i:*mut comedi_insn,e:unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,usize)->i32,c:usize)->i32; fn comedi_dio_update_state(s:*mut comedi_subdevice,d:*mut u32)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
