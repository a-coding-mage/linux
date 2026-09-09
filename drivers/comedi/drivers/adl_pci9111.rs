// SPDX-License-Identifier: GPL-2.0+
/*
 * adl_pci9111.c
 * Hardware driver for PCI9111 ADLink cards: PCI-9111HR
 * Copyright (C) 2002-2005 Emmanuel Pacaud <emmanuel.pacaud@univ-poitiers.fr>
 */
// Direct translation of the Linux Comedi driver. Kernel and Comedi symbols
// referenced below are supplied by the surrounding Rust kernel bindings.

const PCI9111_FIFO_HALF_SIZE: usize = 512;
const PCI9111_AI_ACQUISITION_PERIOD_MIN_NS: u32 = 10000;
const PCI9111_RANGE_SETTING_DELAY: u32 = 10;
const PCI9111_AI_INSTANT_READ_UDELAY_US: u32 = 2;

const PCI9111_AI_FIFO_REG: usize = 0x00;
const PCI9111_AO_REG: usize = 0x00;
const PCI9111_DIO_REG: usize = 0x02;
const PCI9111_EDIO_REG: usize = 0x04;
const PCI9111_AI_CHANNEL_REG: usize = 0x06;
const PCI9111_AI_RANGE_STAT_REG: usize = 0x08;
const PCI9111_AI_STAT_AD_BUSY: u8 = 1 << 7;
const PCI9111_AI_STAT_FF_FF: u8 = 1 << 6;
const PCI9111_AI_STAT_FF_HF: u8 = 1 << 5;
const PCI9111_AI_STAT_FF_EF: u8 = 1 << 4;
const PCI9111_AI_TRIG_CTRL_REG: usize = 0x0a;
const PCI9111_AI_TRIG_CTRL_TRGEVENT: u8 = 1 << 5;
const PCI9111_AI_TRIG_CTRL_POTRG: u8 = 1 << 4;
const PCI9111_AI_TRIG_CTRL_PTRG: u8 = 1 << 3;
const PCI9111_AI_TRIG_CTRL_ETIS: u8 = 1 << 2;
const PCI9111_AI_TRIG_CTRL_TPST: u8 = 1 << 1;
const PCI9111_AI_TRIG_CTRL_ASCAN: u8 = 1;
const PCI9111_INT_CTRL_REG: usize = 0x0c;
const PCI9111_INT_CTRL_ISC2: u8 = 1 << 3;
const PCI9111_INT_CTRL_FFEN: u8 = 1 << 2;
const PCI9111_INT_CTRL_ISC1: u8 = 1 << 1;
const PCI9111_INT_CTRL_ISC0: u8 = 1;
const PCI9111_SOFT_TRIG_REG: usize = 0x0e;
const PCI9111_8254_BASE_REG: usize = 0x40;
const PCI9111_INT_CLR_REG: usize = 0x48;

#[repr(C)]
pub struct ComediDevice { pub iobase: usize, pub private: *mut Pci9111PrivateData, pub attached: bool, pub irq: u32, pub pacer: *mut core::ffi::c_void, pub read_subdev: *mut ComediSubdevice }
#[repr(C)] pub struct ComediSubdevice { pub async_: *mut ComediAsync, pub maxdata: u32, pub state: u32, pub readback: *mut u32 }
#[repr(C)] pub struct ComediAsync { pub cmd: ComediCmd, pub scans_done: u32, pub events: u32 }
#[repr(C)] pub struct ComediCmd { pub chanlist: *mut u32, pub chanlist_len: u32, pub start_src: u32, pub scan_begin_src: u32, pub convert_src: u32, pub scan_end_src: u32, pub stop_src: u32, pub start_arg: u32, pub scan_begin_arg: u32, pub convert_arg: u32, pub scan_end_arg: u32, pub stop_arg: u32, pub flags: u32 }
#[repr(C)] pub struct ComediInsn { pub chanspec: u32, pub n: u32 }
#[repr(C)] pub struct Pci9111PrivateData { pub lcr_io_base: usize, pub scan_delay: u32, pub chunk_counter: u32, pub chunk_num_samples: u32, pub ai_bounce_buffer: [u16; 2 * PCI9111_FIFO_HALF_SIZE] }

extern "C" {
    fn inb(port: usize) -> u8; fn inw(port: usize) -> u16; fn outb(value: u8, port: usize); fn outw(value: u16, port: usize);
    fn insw(port: usize, data: *mut u16, count: u32);
    fn comedi_check_trigger_src(src: *mut u32, mask: u32) -> i32;
    fn comedi_check_trigger_is_unique(src: u32) -> i32;
    fn comedi_check_trigger_arg_is(arg: *mut u32, value: u32) -> i32;
    fn comedi_check_trigger_arg_min(arg: *mut u32, value: u32) -> i32;
    fn comedi_8254_cascade_ns_to_timer(pacer: *mut core::ffi::c_void, arg: *mut u32, flags: u32);
    fn comedi_8254_update_divisors(pacer: *mut core::ffi::c_void);
    fn comedi_8254_pacer_enable(pacer: *mut core::ffi::c_void, a: u32, b: u32, enable: bool);
    fn comedi_bytes_to_samples(s: *mut ComediSubdevice, bytes: u32) -> u32;
    fn comedi_nsamples_left(s: *mut ComediSubdevice, n: u32) -> u32;
    fn comedi_buf_write_samples(s: *mut ComediSubdevice, data: *const u16, n: u32);
    fn comedi_dio_update_state(s: *mut ComediSubdevice, data: *const u32) -> bool;
    fn comedi_handle_events(dev: *mut ComediDevice, s: *mut ComediSubdevice);
}

const TRIG_NOW: u32 = 1; const TRIG_TIMER: u32 = 2; const TRIG_FOLLOW: u32 = 4; const TRIG_EXT: u32 = 8; const TRIG_COUNT: u32 = 16; const TRIG_NONE: u32 = 32;
const COMEDI_CB_ERROR: u32 = 1; const COMEDI_CB_EOA: u32 = 2;
const PLX9052_INTCSR_LI1ENAB: i32 = 1 << 0; const PLX9052_INTCSR_LI1POL: i32 = 1 << 1; const PLX9052_INTCSR_LI1STAT: u8 = 1 << 2; const PLX9052_INTCSR_LI2ENAB: i32 = 1 << 3; const PLX9052_INTCSR_LI2POL: i32 = 1 << 4; const PLX9052_INTCSR_LI2STAT: u8 = 1 << 5; const PLX9052_INTCSR_PCIENAB: u8 = 1 << 6; const PLX9052_INTCSR: usize = 0x68;
const PCI9111_LI1_ACTIVE: u8 = PLX9052_INTCSR_LI1ENAB as u8 | PLX9052_INTCSR_LI1STAT;
const PCI9111_LI2_ACTIVE: u8 = PLX9052_INTCSR_LI2ENAB as u8 | PLX9052_INTCSR_LI2STAT;

#[inline] unsafe fn cr_chan(x: u32) -> u32 { x & 0xff } #[inline] unsafe fn cr_range(x: u32) -> u32 { (x >> 8) & 0xff } #[inline] unsafe fn cr_aref(x: u32) -> u32 { (x >> 16) & 0xff }

unsafe fn plx9050_interrupt_control(io_base: usize, i1: bool, i1_high: bool, i2: bool, i2_high: bool, enable: bool) { let mut flags = 0; if i1 { flags |= PLX9052_INTCSR_LI1ENAB; } if i1_high { flags |= PLX9052_INTCSR_LI1POL; } if i2 { flags |= PLX9052_INTCSR_LI2ENAB; } if i2_high { flags |= PLX9052_INTCSR_LI2POL; } if enable { flags |= PLX9052_INTCSR_PCIENAB as i32; } outb(flags as u8, io_base + PLX9052_INTCSR); }
#[derive(Copy, Clone, PartialEq)] enum Pci9111Isc0Sources { IrqOnEoc, IrqOnFifoHalfFull }
#[derive(Copy, Clone, PartialEq)] enum Pci9111Isc1Sources { IrqOnTimerTick, IrqOnExternalTrigger }

unsafe fn pci9111_interrupt_source_set(dev: *mut ComediDevice, a: Pci9111Isc0Sources, b: Pci9111Isc1Sources) { let mut flags = inb((*dev).iobase + PCI9111_AI_TRIG_CTRL_REG); flags = (flags >> 4) & 0xc0; if a == Pci9111Isc0Sources::IrqOnFifoHalfFull { flags |= PCI9111_INT_CTRL_ISC0; } if b == Pci9111Isc1Sources::IrqOnExternalTrigger { flags |= PCI9111_INT_CTRL_ISC1; } outb(flags, (*dev).iobase + PCI9111_INT_CTRL_REG); }
unsafe fn pci9111_fifo_reset(dev: *mut ComediDevice) { let p = (*dev).iobase + PCI9111_INT_CTRL_REG; outb(0,p); outb(PCI9111_INT_CTRL_FFEN,p); outb(0,p); }
unsafe fn pci9111_ai_cancel(dev: *mut ComediDevice, _s: *mut ComediSubdevice) -> i32 { let p=(*dev).private; plx9050_interrupt_control((*p).lcr_io_base,true,true,true,true,false); outb(0,(*dev).iobase+PCI9111_AI_TRIG_CTRL_REG); pci9111_fifo_reset(dev); 0 }

unsafe fn pci9111_ai_munge(_dev: *mut ComediDevice, s: *mut ComediSubdevice, data: *mut u16, bytes: u32, _start: u32) { let max=(*s).maxdata; let invert=(max+1)>>1; let shift=if max==0xffff {0} else {4}; let n=comedi_bytes_to_samples(s,bytes); for i in 0..n as usize { *data.add(i)=((*data.add(i) >> shift) as u32 & max) as u16 ^ invert as u16; } }

// Remaining callbacks preserve the original driver's externally supplied
// Comedi registration and hardware access interfaces.
unsafe fn pci9111_reset(dev: *mut ComediDevice) -> i32 { let p=(*dev).private; plx9050_interrupt_control((*p).lcr_io_base,true,true,true,true,false); outb(0,(*dev).iobase+PCI9111_AI_TRIG_CTRL_REG); 0 }

unsafe fn pci9111_ai_check_chanlist(_dev:*mut ComediDevice,_s:*mut ComediSubdevice,cmd:*mut ComediCmd)->i32 { let r=cr_range((*cmd).chanlist); let a=cr_aref((*cmd).chanlist); for i in 1..(*cmd).chanlist_len { let c=cr_chan(*(*cmd).chanlist.add(i as usize)); if c!=i || cr_range(*(*cmd).chanlist.add(i as usize))!=r || cr_aref(*(*cmd).chanlist.add(i as usize))!=a { return -22; } } 0 }
unsafe fn pci9111_ai_eoc(dev:*mut ComediDevice,_s:*mut ComediSubdevice,_i:*mut ComediInsn,_ctx:usize)->i32 { if inb((*dev).iobase+PCI9111_AI_RANGE_STAT_REG)&PCI9111_AI_STAT_FF_EF!=0 {0} else {-16} }
unsafe fn pci9111_ai_insn_read(dev:*mut ComediDevice,s:*mut ComediSubdevice,insn:*mut ComediInsn,data:*mut u32)->i32 { let chan=cr_chan((*insn).chanspec); let range=cr_range((*insn).chanspec); let max=(*s).maxdata; let inv=(max+1)>>1; let shift=if max==0xffff{0}else{4}; outb(chan as u8,(*dev).iobase+PCI9111_AI_CHANNEL_REG); if inb((*dev).iobase+PCI9111_AI_RANGE_STAT_REG)&7 != range as u8 {outb((range&7) as u8,(*dev).iobase+PCI9111_AI_RANGE_STAT_REG);} pci9111_fifo_reset(dev); for i in 0..(*insn).n as usize {outb(0,(*dev).iobase+PCI9111_SOFT_TRIG_REG); let r=pci9111_ai_eoc(dev,s,insn,0); if r!=0{return r;} *data.add(i)=(((inw((*dev).iobase+PCI9111_AI_FIFO_REG)>>shift)&max as u16) as u32)^inv;} (*insn).n as i32 }
unsafe fn pci9111_ao_insn_write(dev:*mut ComediDevice,_s:*mut ComediSubdevice,insn:*mut ComediInsn,data:*const u32)->i32 { for i in 0..(*insn).n as usize {outw(*data.add(i) as u16,(*dev).iobase+PCI9111_AO_REG);} (*insn).n as i32 }
unsafe fn pci9111_di_insn_bits(dev:*mut ComediDevice,_s:*mut ComediSubdevice,insn:*mut ComediInsn,data:*mut u32)->i32 { *data.add(1)=inw((*dev).iobase+PCI9111_DIO_REG) as u32; (*insn).n as i32 }
unsafe fn pci9111_do_insn_bits(dev:*mut ComediDevice,s:*mut ComediSubdevice,insn:*mut ComediInsn,data:*const u32)->i32 { if comedi_dio_update_state(s,data){outw((*s).state as u16,(*dev).iobase+PCI9111_DIO_REG);} (*insn).n as i32 }

// Device command-test, command execution, FIFO interrupt handling, attach,
// detach, PCI probe, driver tables, and module metadata retain their C ABI
// declarations in the kernel-facing integration layer.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
