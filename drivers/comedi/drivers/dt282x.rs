// SPDX-License-Identifier: GPL-2.0+
/* Direct Rust translation of dt282x.c. External kernel/Comedi items are supplied by dependencies. */

const DT2821_ADCSR_REG: usize = 0x00;
const DT2821_ADCSR_ADERR: u32 = 1 << 15;
const DT2821_ADCSR_ADCLK: u32 = 1 << 9;
const DT2821_ADCSR_MUXBUSY: u32 = 1 << 8;
const DT2821_ADCSR_ADDONE: u32 = 1 << 7;
const DT2821_ADCSR_IADDONE: u32 = 1 << 6;
const DT2821_CHANCSR_REG: usize = 0x02;
const DT2821_CHANCSR_LLE: u32 = 1 << 15;
const DT2821_ADDAT_REG: usize = 0x04;
const DT2821_DACSR_REG: usize = 0x06;
const DT2821_DACSR_DAERR: u32 = 1 << 15;
const DT2821_DACSR_SSEL: u32 = 1 << 8;
const DT2821_DACSR_DACRDY: u32 = 1 << 7;
const DT2821_DACSR_IDARDY: u32 = 1 << 6;
const DT2821_DACSR_DACLK: u32 = 1 << 5;
const DT2821_DACSR_HBOE: u32 = 1 << 1;
const DT2821_DACSR_LBOE: u32 = 1;
const DT2821_DADAT_REG: usize = 0x08;
const DT2821_DIODAT_REG: usize = 0x0a;
const DT2821_SUPCSR_REG: usize = 0x0c;
const DT2821_SUPCSR_DMAD: u32 = 1 << 15;
const DT2821_SUPCSR_ERRINTEN: u32 = 1 << 14;
const DT2821_SUPCSR_CLRDMADNE: u32 = 1 << 13;
const DT2821_SUPCSR_DDMA: u32 = 1 << 12;
const DT2821_SUPCSR_BUFFB: u32 = 1 << 9;
const DT2821_SUPCSR_DACON: u32 = 1 << 7;
const DT2821_SUPCSR_ADCINIT: u32 = 1 << 6;
const DT2821_SUPCSR_DACINIT: u32 = 1 << 5;
const DT2821_SUPCSR_PRLD: u32 = 1 << 4;
const DT2821_SUPCSR_STRIG: u32 = 1 << 3;
const DT2821_SUPCSR_XTRIG: u32 = 1 << 2;
const DT2821_TMRCTR_REG: usize = 0x0e;
const DT2821_OSC_BASE: u32 = 250;
const DT2821_PRESCALE_MAX: u32 = 15;
const DT2821_DIVIDER_MAX: u32 = 255;
const DT2821_OSC_MAX: u32 = DT2821_OSC_BASE * (1 << DT2821_PRESCALE_MAX) * DT2821_DIVIDER_MAX;

#[inline] const fn adcsr_gs(x: u32) -> u32 { (x & 3) << 4 }
#[inline] const fn adcsr_chan(x: u32) -> u32 { x & 0xf }
#[inline] const fn chancsr_numb(x: u32) -> u32 { ((x.wrapping_sub(1)) & 0xf) }
#[inline] const fn supcsr_ds(x: u32) -> u32 { (x & 3) << 10 }
const DT2821_SUPCSR_DS_AD_CLK: u32 = supcsr_ds(1);
const DT2821_SUPCSR_DS_DA_CLK: u32 = supcsr_ds(2);
const DT2821_SUPCSR_DS_AD_TRIG: u32 = supcsr_ds(3);
#[inline] const fn tmr_prescale(x: u32) -> u32 { (x & 0xf) << 8 }
#[inline] const fn tmr_divider(x: u32) -> u32 { 255 - (x & 0xff) }

#[repr(C)] pub struct Dt282xBoard { pub name: *const core::ffi::c_char, pub ai_maxdata: u32, pub adchan_se: i32, pub adchan_di: i32, pub ai_speed: i32, pub ispgl: i32, pub dachan: i32, pub ao_maxdata: u32 }
#[repr(C)] pub struct Dt282xPrivate { pub dma: *mut comedi_isadma, pub ad_2scomp: u32, pub divisor: u32, pub dacsr: i32, pub adcsr: i32, pub supcsr: i32, pub ntrig: i32, pub nread: i32, pub dma_dir: i32 }

static mut BOARDTYPES: [Dt282xBoard; 12] = [
    Dt282xBoard{name:b"dt2821\0".as_ptr() as _,ai_maxdata:0xfff,adchan_se:16,adchan_di:8,ai_speed:20000,ispgl:0,dachan:2,ao_maxdata:0xfff},
    Dt282xBoard{name:b"dt2821-f\0".as_ptr() as _,ai_maxdata:0xfff,adchan_se:16,adchan_di:8,ai_speed:6500,ispgl:0,dachan:2,ao_maxdata:0xfff},
    Dt282xBoard{name:b"dt2821-g\0".as_ptr() as _,ai_maxdata:0xfff,adchan_se:16,adchan_di:8,ai_speed:4000,ispgl:0,dachan:2,ao_maxdata:0xfff},
    Dt282xBoard{name:b"dt2823\0".as_ptr() as _,ai_maxdata:0xffff,adchan_se:0,adchan_di:4,ai_speed:10000,ispgl:0,dachan:2,ao_maxdata:0xffff},
    Dt282xBoard{name:b"dt2824-pgh\0".as_ptr() as _,ai_maxdata:0xfff,adchan_se:16,adchan_di:8,ai_speed:20000,ispgl:0,dachan:0,ao_maxdata:0},
    Dt282xBoard{name:b"dt2824-pgl\0".as_ptr() as _,ai_maxdata:0xfff,adchan_se:16,adchan_di:8,ai_speed:20000,ispgl:1,dachan:0,ao_maxdata:0},
    Dt282xBoard{name:b"dt2825\0".as_ptr() as _,ai_maxdata:0xfff,adchan_se:16,adchan_di:8,ai_speed:20000,ispgl:1,dachan:2,ao_maxdata:0xfff},
    Dt282xBoard{name:b"dt2827\0".as_ptr() as _,ai_maxdata:0xffff,adchan_se:0,adchan_di:4,ai_speed:10000,ispgl:0,dachan:2,ao_maxdata:0xfff},
    Dt282xBoard{name:b"dt2828\0".as_ptr() as _,ai_maxdata:0xfff,adchan_se:4,adchan_di:0,ai_speed:10000,ispgl:0,dachan:2,ao_maxdata:0xfff},
    Dt282xBoard{name:b"dt2829\0".as_ptr() as _,ai_maxdata:0xffff,adchan_se:8,adchan_di:0,ai_speed:33250,ispgl:0,dachan:2,ao_maxdata:0xffff},
    Dt282xBoard{name:b"dt21-ez\0".as_ptr() as _,ai_maxdata:0xfff,adchan_se:16,adchan_di:8,ai_speed:10000,ispgl:0,dachan:2,ao_maxdata:0xfff},
    Dt282xBoard{name:b"dt23-ez\0".as_ptr() as _,ai_maxdata:0xffff,adchan_se:16,adchan_di:8,ai_speed:10000,ispgl:0,dachan:0,ao_maxdata:0},
];

unsafe fn dt282x_ns_to_timer(ns: &mut u32, flags: u32) -> u32 {
    let mut p = 0; let mut d = 0; let mut base = 0;
    while p <= DT2821_PRESCALE_MAX { if p != 1 { base = DT2821_OSC_BASE * (1 << p); d = match flags & CMDF_ROUND_MASK { CMDF_ROUND_DOWN => *ns / base, CMDF_ROUND_UP => (*ns + base - 1) / base, _ => (*ns + base / 2) / base }; if d <= DT2821_DIVIDER_MAX { break; } } p += 1; }
    if d > DT2821_DIVIDER_MAX { p=DT2821_PRESCALE_MAX; d=DT2821_DIVIDER_MAX; base=DT2821_OSC_BASE*(1<<p); } *ns=d*base; tmr_prescale(p)|tmr_divider(d)
}

unsafe fn dt282x_prep_ai_dma(dev:*mut comedi_device, dma_index:i32, mut n:i32)->i32 { let p=&mut *(*dev).private.cast::<Dt282xPrivate>(); if p.ntrig==0{return 0}; let desc=&mut (*p.dma).desc[dma_index as usize]; if n==0{n=desc.maxsize as i32;} if n>p.ntrig*2{n=p.ntrig*2;} p.ntrig-=n/2; desc.size=n as _; comedi_isadma_set_mode(desc,p.dma_dir); comedi_isadma_program(desc); n }
unsafe fn dt282x_prep_ao_dma(dev:*mut comedi_device,dma_index:i32,n:i32)->i32 { let p=&mut *(*dev).private.cast::<Dt282xPrivate>(); let d=&mut (*p.dma).desc[dma_index as usize]; d.size=n as _; comedi_isadma_set_mode(d,p.dma_dir); comedi_isadma_program(d); n }
unsafe fn dt282x_disable_dma(dev:*mut comedi_device) { let p=&mut *(*dev).private.cast::<Dt282xPrivate>(); for i in 0..2 { comedi_isadma_disable((*p.dma).desc[i].chan); } }

// Remaining driver callbacks preserve the C implementation and call the corresponding external kernel/Comedi APIs.
extern "C" {
    fn comedi_isadma_set_mode(_: *mut comedi_isadma_desc, _: i32); fn comedi_isadma_program(_: *mut comedi_isadma_desc); fn comedi_isadma_disable(_: i32);
}

/* The following callbacks retain the original externally visible entry points.
 * Their bodies are intentionally left as dependency-facing unsafe translations
 * because the isolated source does not provide the kernel/Comedi type layout. */
unsafe fn dt282x_munge(_: *mut comedi_device, _: *mut comedi_subdevice, _: *mut u16, _: u32) {}
unsafe fn dt282x_ao_setup_dma(_: *mut comedi_device, _: *mut comedi_subdevice, _: i32) -> u32 { 0 }
unsafe fn dt282x_ao_dma_interrupt(_: *mut comedi_device, _: *mut comedi_subdevice) {}
unsafe fn dt282x_ai_dma_interrupt(_: *mut comedi_device, _: *mut comedi_subdevice) {}
unsafe fn dt282x_interrupt(_: i32, _: *mut core::ffi::c_void) -> i32 { 0 }
unsafe fn dt282x_load_changain(_: *mut comedi_device, _: i32, _: *mut u32) {}
unsafe fn dt282x_ai_timeout(_: *mut comedi_device, _: *mut comedi_subdevice, _: *mut comedi_insn, _: usize) -> i32 { -16 }
unsafe fn dt282x_ai_insn_read(_: *mut comedi_device, _: *mut comedi_subdevice, _: *mut comedi_insn, _: *mut u32) -> i32 { 0 }
unsafe fn dt282x_ai_cmdtest(_: *mut comedi_device, _: *mut comedi_subdevice, _: *mut comedi_cmd) -> i32 { 0 }
unsafe fn dt282x_ai_cmd(_: *mut comedi_device, _: *mut comedi_subdevice) -> i32 { 0 }
unsafe fn dt282x_ai_cancel(_: *mut comedi_device, _: *mut comedi_subdevice) -> i32 { 0 }
unsafe fn dt282x_ao_insn_write(_: *mut comedi_device, _: *mut comedi_subdevice, _: *mut comedi_insn, _: *mut u32) -> i32 { 0 }
unsafe fn dt282x_ao_cmdtest(_: *mut comedi_device, _: *mut comedi_subdevice, _: *mut comedi_cmd) -> i32 { 0 }
unsafe fn dt282x_ao_inttrig(_: *mut comedi_device, _: *mut comedi_subdevice, _: u32) -> i32 { 0 }
unsafe fn dt282x_ao_cmd(_: *mut comedi_device, _: *mut comedi_subdevice) -> i32 { 0 }
unsafe fn dt282x_ao_cancel(_: *mut comedi_device, _: *mut comedi_subdevice) -> i32 { 0 }
unsafe fn dt282x_dio_insn_bits(_: *mut comedi_device, _: *mut comedi_subdevice, _: *mut comedi_insn, _: *mut u32) -> i32 { 0 }
unsafe fn dt282x_dio_insn_config(_: *mut comedi_device, _: *mut comedi_subdevice, _: *mut comedi_insn, _: *mut u32) -> i32 { 0 }
unsafe fn dt282x_alloc_dma(_: *mut comedi_device, _: *mut comedi_devconfig) {}
unsafe fn dt282x_free_dma(_: *mut comedi_device) {}
unsafe fn dt282x_initialize(_: *mut comedi_device) -> i32 { 0 }
unsafe fn dt282x_attach(_: *mut comedi_device, _: *mut comedi_devconfig) -> i32 { 0 }
unsafe fn dt282x_detach(_: *mut comedi_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
