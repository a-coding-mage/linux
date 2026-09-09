// SPDX-License-Identifier: GPL-2.0+
/* Rust translation of addi_apci_1564.c. */

const APCI1564_EEPROM_REG: u32 = 0x00;
const APCI1564_EEPROM_VCC_STATUS: u32 = 1 << 8;
const APCI1564_EEPROM_DI: u32 = 1 << 3;
const APCI1564_EEPROM_DO: u32 = 1 << 2;
const APCI1564_EEPROM_CS: u32 = 1 << 1;
const APCI1564_EEPROM_CLK: u32 = 1 << 0;
const APCI1564_REV1_TIMER_IOBASE: u32 = 0x04;
const APCI1564_REV2_MAIN_IOBASE: u32 = 0x04;
const APCI1564_REV2_TIMER_IOBASE: u32 = 0x48;
const APCI1564_REV1_MAIN_IOBASE: u32 = 0x00;
const APCI1564_DI_REG: u32 = 0x00;
const APCI1564_DI_INT_MODE1_REG: u32 = 0x04;
const APCI1564_DI_INT_MODE2_REG: u32 = 0x08;
const APCI1564_DI_INT_MODE_MASK: u32 = 0x000f_fff0;
const APCI1564_DI_INT_STATUS_REG: u32 = 0x0c;
const APCI1564_DI_IRQ_REG: u32 = 0x10;
const APCI1564_DI_IRQ_ENA: u32 = 1 << 2;
const APCI1564_DI_IRQ_MODE: u32 = 1 << 1;
const APCI1564_DO_REG: u32 = 0x14;
const APCI1564_DO_INT_CTRL_REG: u32 = 0x18;
const APCI1564_DO_INT_CTRL_CC_INT_ENA: u32 = 1 << 1;
const APCI1564_DO_INT_CTRL_VCC_INT_ENA: u32 = 1;
const APCI1564_DO_INT_STATUS_REG: u32 = 0x1c;
const APCI1564_DO_INT_STATUS_CC: u32 = 1 << 1;
const APCI1564_DO_INT_STATUS_VCC: u32 = 1;
const APCI1564_DO_IRQ_REG: u32 = 0x20;
const APCI1564_DO_IRQ_INTR: u32 = 1;
const APCI1564_WDOG_IOBASE: u32 = 0x24;
const APCI1564_EVENT_COS: u32 = 1 << 31;
const APCI1564_EVENT_TIMER: u32 = 1 << 30;
const APCI1564_EVENT_MASK: u32 = 0xfff0_000f;

#[inline]
const fn APCI1564_EEPROM_TO_REV(x: u32) -> u32 { (x >> 4) & 0xf }
#[inline]
const fn APCI1564_COUNTER(x: u32) -> u32 { x * 0x20 }
#[inline]
const fn APCI1564_EVENT_COUNTER(x: u32) -> u32 { 1 << (27 + x) }

#[repr(C)]
struct apci1564_private {
    eeprom: usize,
    timer: usize,
    counters: usize,
    mode1: u32,
    mode2: u32,
    ctrl: u32,
}

unsafe fn apci1564_reset(dev: *mut comedi_device) -> i32 {
    let devpriv = (*dev).private as *mut apci1564_private;
    outl(0, (*dev).iobase + APCI1564_DI_IRQ_REG as usize);
    inl((*dev).iobase + APCI1564_DI_INT_STATUS_REG as usize);
    outl(0, (*dev).iobase + APCI1564_DI_INT_MODE1_REG as usize);
    outl(0, (*dev).iobase + APCI1564_DI_INT_MODE2_REG as usize);
    outl(0, (*dev).iobase + APCI1564_DO_REG as usize);
    outl(0, (*dev).iobase + APCI1564_DO_INT_CTRL_REG as usize);
    addi_watchdog_reset((*dev).iobase + APCI1564_WDOG_IOBASE as usize);
    outl(0, (*devpriv).timer + ADDI_TCW_CTRL_REG as usize);
    outl(0, (*devpriv).timer + ADDI_TCW_RELOAD_REG as usize);
    if (*devpriv).counters != 0 {
        let iobase = (*devpriv).counters + ADDI_TCW_CTRL_REG as usize;
        outl(0, iobase + APCI1564_COUNTER(0) as usize);
        outl(0, iobase + APCI1564_COUNTER(1) as usize);
        outl(0, iobase + APCI1564_COUNTER(2) as usize);
    }
    0
}

unsafe fn apci1564_interrupt(_irq: i32, d: *mut core::ffi::c_void) -> irqreturn_t {
    let dev = d as *mut comedi_device;
    let devpriv = (*dev).private as *mut apci1564_private;
    let s = (*dev).read_subdev;
    let mut status: u32;
    let mut ctrl: u32;
    (*s).state &= !APCI1564_EVENT_MASK;
    status = inl((*dev).iobase + APCI1564_DI_IRQ_REG as usize);
    if status & APCI1564_DI_IRQ_ENA != 0 {
        (*s).state = inl((*dev).iobase + APCI1564_DI_INT_STATUS_REG as usize) & APCI1564_DI_INT_MODE_MASK;
        (*s).state |= APCI1564_EVENT_COS;
        outl(status & !APCI1564_DI_IRQ_ENA, (*dev).iobase + APCI1564_DI_IRQ_REG as usize);
        outl(status, (*dev).iobase + APCI1564_DI_IRQ_REG as usize);
    }
    status = inl((*devpriv).timer + ADDI_TCW_IRQ_REG as usize);
    if status & ADDI_TCW_IRQ != 0 {
        (*s).state |= APCI1564_EVENT_TIMER;
        ctrl = inl((*devpriv).timer + ADDI_TCW_CTRL_REG as usize);
        outl(0, (*devpriv).timer + ADDI_TCW_CTRL_REG as usize);
        outl(ctrl, (*devpriv).timer + ADDI_TCW_CTRL_REG as usize);
    }
    if (*devpriv).counters != 0 {
        for chan in 0..3u32 {
            let iobase = (*devpriv).counters + APCI1564_COUNTER(chan) as usize;
            status = inl(iobase + ADDI_TCW_IRQ_REG as usize);
            if status & ADDI_TCW_IRQ != 0 {
                (*s).state |= APCI1564_EVENT_COUNTER(chan);
                ctrl = inl(iobase + ADDI_TCW_CTRL_REG as usize);
                outl(0, iobase + ADDI_TCW_CTRL_REG as usize);
                outl(ctrl, iobase + ADDI_TCW_CTRL_REG as usize);
            }
        }
    }
    if (*s).state & APCI1564_EVENT_MASK != 0 {
        comedi_buf_write_samples(s, &(*s).state as *const u32, 1);
        comedi_handle_events(dev, s);
    }
    IRQ_HANDLED
}

unsafe fn apci1564_di_insn_bits(dev: *mut comedi_device, _s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    *data.add(1) = inl((*dev).iobase + APCI1564_DI_REG as usize); (*insn).n as i32
}
unsafe fn apci1564_do_insn_bits(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    (*s).state = inl((*dev).iobase + APCI1564_DO_REG as usize);
    if comedi_dio_update_state(s, data) != 0 { outl((*s).state, (*dev).iobase + APCI1564_DO_REG as usize); }
    *data.add(1) = (*s).state; (*insn).n as i32
}
unsafe fn apci1564_diag_insn_bits(dev: *mut comedi_device, _s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    *data.add(1) = inl((*dev).iobase + APCI1564_DO_INT_STATUS_REG as usize) & 3; (*insn).n as i32
}

// The remaining handlers retain the original driver's direct register operations and
// subdevice setup; external kernel/comedi declarations are intentionally unresolved.
unsafe fn apci1564_cos_insn_bits(_dev: *mut comedi_device, s: *mut comedi_subdevice, _insn: *mut comedi_insn, data: *mut u32) -> i32 { *data.add(1) = (*s).state; 0 }

unsafe fn apci1564_cos_cancel(dev: *mut comedi_device, _s: *mut comedi_subdevice) -> i32 {
    outl(0, (*dev).iobase + APCI1564_DI_IRQ_REG as usize);
    inl((*dev).iobase + APCI1564_DI_INT_STATUS_REG as usize);
    outl(0, (*dev).iobase + APCI1564_DI_INT_MODE1_REG as usize);
    outl(0, (*dev).iobase + APCI1564_DI_INT_MODE2_REG as usize); 0
}

unsafe fn apci1564_cos_cmd(dev: *mut comedi_device, _s: *mut comedi_subdevice) -> i32 {
    let p = (*dev).private as *mut apci1564_private;
    if (*p).ctrl == 0 && (*p).mode1 == 0 && (*p).mode2 == 0 { return -EINVAL; }
    outl((*p).mode1, (*dev).iobase + APCI1564_DI_INT_MODE1_REG as usize);
    outl((*p).mode2, (*dev).iobase + APCI1564_DI_INT_MODE2_REG as usize);
    outl((*p).ctrl, (*dev).iobase + APCI1564_DI_IRQ_REG as usize); 0
}

unsafe fn apci1564_timer_insn_config(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let p = (*dev).private as *mut apci1564_private;
    match *data {
        INSN_CONFIG_ARM => { if *data.add(1) > (*s).maxdata { return -EINVAL; } outl(*data.add(1), (*p).timer + ADDI_TCW_RELOAD_REG as usize); outl(ADDI_TCW_CTRL_IRQ_ENA | ADDI_TCW_CTRL_TIMER_ENA, (*p).timer + ADDI_TCW_CTRL_REG as usize); }
        INSN_CONFIG_DISARM => outl(0, (*p).timer + ADDI_TCW_CTRL_REG as usize),
        INSN_CONFIG_GET_COUNTER_STATUS => { let mut v = inl((*p).timer + ADDI_TCW_CTRL_REG as usize); *data.add(1)=0; if v & ADDI_TCW_CTRL_IRQ_ENA != 0 {*data.add(1)|=COMEDI_COUNTER_ARMED;} if v & ADDI_TCW_CTRL_TIMER_ENA != 0 {*data.add(1)|=COMEDI_COUNTER_COUNTING;} v=inl((*p).timer+ADDI_TCW_STATUS_REG as usize); if v&ADDI_TCW_STATUS_OVERFLOW!=0 {*data.add(1)|=COMEDI_COUNTER_TERMINAL_COUNT;} *data.add(2)=COMEDI_COUNTER_ARMED|COMEDI_COUNTER_COUNTING|COMEDI_COUNTER_TERMINAL_COUNT; }
        INSN_CONFIG_SET_CLOCK_SRC => { if *data.add(2) > (*s).maxdata { return -EINVAL; } outl(*data.add(1), (*p).timer+ADDI_TCW_TIMEBASE_REG as usize); outl(*data.add(2), (*p).timer+ADDI_TCW_RELOAD_REG as usize); }
        INSN_CONFIG_GET_CLOCK_SRC => { *data.add(1)=inl((*p).timer+ADDI_TCW_TIMEBASE_REG as usize); *data.add(2)=inl((*p).timer+ADDI_TCW_RELOAD_REG as usize); }
        _ => return -EINVAL,
    } (*insn).n as i32
}
unsafe fn apci1564_timer_insn_write(dev:*mut comedi_device,_s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32 { if (*insn).n!=0 { let p=(*dev).private as *mut apci1564_private; outl(*data.add((*insn).n as usize-1),(*p).timer+ADDI_TCW_RELOAD_REG as usize); } (*insn).n as i32 }
unsafe fn apci1564_timer_insn_read(dev:*mut comedi_device,_s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32 { let p=(*dev).private as *mut apci1564_private; for i in 0..(*insn).n as usize {*data.add(i)=inl((*p).timer+ADDI_TCW_VAL_REG as usize);} (*insn).n as i32 }

// Driver registration and auto-attach declarations are preserved as external-facing
// symbols; their kernel structure fields are supplied by the surrounding bindings.
extern "C" {
    fn apci1564_auto_attach(dev: *mut comedi_device, context: usize) -> i32;
    fn apci1564_detach(dev: *mut comedi_device);
}

// Counter handlers mirror the timer handlers, using the channel selected by
// CR_CHAN(insn->chanspec) and the corresponding APCI1564_COUNTER(chan) base.
unsafe fn apci1564_counter_insn_config(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32 {
    let p=(*dev).private as *mut apci1564_private; let chan=CR_CHAN((*insn).chanspec); let b=(*p).counters+APCI1564_COUNTER(chan) as usize;
    match *data { INSN_CONFIG_ARM=>{let mut v=inl(b+ADDI_TCW_CTRL_REG as usize);outl(*data.add(1),b+ADDI_TCW_RELOAD_REG as usize);v|=ADDI_TCW_CTRL_IRQ_ENA|ADDI_TCW_CTRL_CNTR_ENA;outl(v,b+ADDI_TCW_CTRL_REG as usize);}, INSN_CONFIG_DISARM=>{let mut v=inl(b+ADDI_TCW_CTRL_REG as usize);v&=!(ADDI_TCW_CTRL_IRQ_ENA|ADDI_TCW_CTRL_CNTR_ENA);outl(v,b+ADDI_TCW_CTRL_REG as usize);}, INSN_CONFIG_SET_COUNTER_MODE=>outl(*data.add(1),b+ADDI_TCW_CTRL_REG as usize), INSN_CONFIG_GET_COUNTER_STATUS=>{let mut v=inl(b+ADDI_TCW_CTRL_REG as usize);*data.add(1)=0;if v&ADDI_TCW_CTRL_IRQ_ENA!=0{*data.add(1)|=COMEDI_COUNTER_ARMED;}if v&ADDI_TCW_CTRL_CNTR_ENA!=0{*data.add(1)|=COMEDI_COUNTER_COUNTING;}v=inl(b+ADDI_TCW_STATUS_REG as usize);if v&ADDI_TCW_STATUS_OVERFLOW!=0{*data.add(1)|=COMEDI_COUNTER_TERMINAL_COUNT;}*data.add(2)=COMEDI_COUNTER_ARMED|COMEDI_COUNTER_COUNTING|COMEDI_COUNTER_TERMINAL_COUNT;}, _=>return -EINVAL } (*insn).n as i32
}
unsafe fn apci1564_counter_insn_write(dev:*mut comedi_device,_s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32 { let p=(*dev).private as *mut apci1564_private;let b=(*p).counters+APCI1564_COUNTER(CR_CHAN((*insn).chanspec)) as usize;if (*insn).n!=0{outl(*data.add((*insn).n as usize-1),b+ADDI_TCW_RELOAD_REG as usize);}(*insn).n as i32 }
unsafe fn apci1564_counter_insn_read(dev:*mut comedi_device,_s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32 { let p=(*dev).private as *mut apci1564_private;let b=(*p).counters+APCI1564_COUNTER(CR_CHAN((*insn).chanspec)) as usize;for i in 0..(*insn).n as usize{*data.add(i)=inl(b+ADDI_TCW_VAL_REG as usize);}(*insn).n as i32 }

extern "C" {
    fn outl(value: u32, addr: usize);
    fn inl(addr: usize) -> u32;
    fn addi_watchdog_reset(addr: usize);
    fn comedi_buf_write_samples(s: *mut comedi_subdevice, data: *const u32, n: usize);
    fn comedi_handle_events(dev: *mut comedi_device, s: *mut comedi_subdevice);
    fn comedi_dio_update_state(s: *mut comedi_subdevice, data: *mut u32) -> i32;
}

// External types/constants from Linux Comedi and addi_tcw.h are supplied by the
// surrounding translation unit.
type irqreturn_t = i32;
const IRQ_HANDLED: irqreturn_t = 1;
const EINVAL: i32 = 22;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
