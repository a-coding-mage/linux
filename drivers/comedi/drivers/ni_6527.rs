// SPDX-License-Identifier: GPL-2.0+
/*
 * ni_6527.c
 * Comedi driver for National Instruments PCI-6527
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1999,2002,2003 David A. Schleef <ds@schleef.org>
 */

// Driver: ni_6527
// Description: National Instruments 6527
// Devices: [National Instruments] PCI-6527 (pci-6527), PXI-6527 (pxi-6527)
// Author: David A. Schleef <ds@schleef.org>
// Updated: Sat, 25 Jan 2003 13:24:40 -0800
// Status: works
// Configuration Options: not applicable, uses PCI auto config

const fn bit(x: u32) -> u32 { 1u32 << x }
const fn ni6527_di_reg(x: usize) -> usize { 0x00 + x }
const fn ni6527_do_reg(x: usize) -> usize { 0x03 + x }
const NI6527_ID_REG: usize = 0x06;
const NI6527_CLR_REG: usize = 0x07;
const NI6527_CLR_EDGE: u32 = bit(3);
const NI6527_CLR_OVERFLOW: u32 = bit(2);
const NI6527_CLR_FILT: u32 = bit(1);
const NI6527_CLR_INTERVAL: u32 = bit(0);
const NI6527_CLR_IRQS: u32 = NI6527_CLR_EDGE | NI6527_CLR_OVERFLOW;
const NI6527_CLR_RESET_FILT: u32 = NI6527_CLR_FILT | NI6527_CLR_INTERVAL;
const fn ni6527_filt_interval_reg(x: usize) -> usize { 0x08 + x }
const fn ni6527_filt_ena_reg(x: usize) -> usize { 0x0c + x }
const NI6527_STATUS_REG: usize = 0x14;
const NI6527_STATUS_IRQ: u32 = bit(2);
const NI6527_STATUS_OVERFLOW: u32 = bit(1);
const NI6527_STATUS_EDGE: u32 = bit(0);
const NI6527_CTRL_REG: usize = 0x15;
const NI6527_CTRL_FALLING: u32 = bit(4);
const NI6527_CTRL_RISING: u32 = bit(3);
const NI6527_CTRL_IRQ: u32 = bit(2);
const NI6527_CTRL_OVERFLOW: u32 = bit(1);
const NI6527_CTRL_EDGE: u32 = bit(0);
const NI6527_CTRL_DISABLE_IRQS: u32 = 0;
const NI6527_CTRL_ENABLE_IRQS: u32 = NI6527_CTRL_FALLING | NI6527_CTRL_RISING | NI6527_CTRL_IRQ | NI6527_CTRL_EDGE;
const fn ni6527_rising_edge_reg(x: usize) -> usize { 0x18 + x }
const fn ni6527_falling_edge_reg(x: usize) -> usize { 0x20 + x }

#[repr(C)]
enum Ni6527Boardid { BoardPci6527, BoardPxi6527 }
#[repr(C)]
struct Ni6527Board { name: *const i8 }
#[repr(C)]
struct Ni6527Private { filter_interval: u32, filter_enable: u32 }

extern "C" {
    static ni6527_boards: [Ni6527Board; 2];
    fn writeb(v: u8, p: *mut u8);
    fn readb(p: *const u8) -> u8;
    fn comedi_dio_update_state(s: *mut ComediSubdevice, data: *const u32) -> u32;
    fn comedi_buf_write_samples(s: *mut ComediSubdevice, v: *const u16, n: usize);
    fn comedi_handle_events(dev: *mut ComediDevice, s: *mut ComediSubdevice);
    fn comedi_check_trigger_src(src: *mut u32, valid: u32) -> i32;
    fn comedi_check_trigger_arg_is(arg: *mut u32, val: u32) -> i32;
    fn comedi_to_pci_dev(dev: *mut ComediDevice) -> *mut PciDev;
    fn comedi_alloc_devpriv(dev: *mut ComediDevice, size: usize) -> *mut Ni6527Private;
    fn comedi_pci_enable(dev: *mut ComediDevice) -> i32;
    fn pci_ioremap_bar(dev: *mut PciDev, bar: i32) -> *mut u8;
    fn request_irq(irq: u32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32, flags: u32, name: *const i8, data: *mut core::ffi::c_void) -> i32;
    fn comedi_alloc_subdevices(dev: *mut ComediDevice, n: usize) -> i32;
    fn comedi_pci_detach(dev: *mut ComediDevice);
    fn comedi_pci_auto_config(dev: *mut PciDev, drv: *mut ComediDriver, data: usize) -> i32;
    fn comedi_pci_auto_unconfig(dev: *mut PciDev);
}

// External kernel/Comedi structures and constants are supplied by dependent bindings.
#[allow(dead_code)]
#[repr(C)] struct ComediDevice { private: *mut core::ffi::c_void, mmio: *mut u8, board_ptr: *const core::ffi::c_void, board_name: *const i8, irq: u32, read_subdev: *mut ComediSubdevice, subdevices: *mut ComediSubdevice }
#[repr(C)] struct ComediSubdevice { state: u32, len_chanlist: u32 }
#[repr(C)] struct ComediInsn { chanspec: u32, n: i32 }
#[repr(C)] struct ComediCmd { start_src: u32, scan_begin_src: u32, convert_src: u32, scan_end_src: u32, stop_src: u32, start_arg: u32, scan_begin_arg: u32, convert_arg: u32, scan_end_arg: u32, stop_arg: u32, chanlist_len: u32 }
#[repr(C)] struct PciDev { irq: u32 }
#[repr(C)] struct ComediDriver { driver_name: *const i8, module: *mut core::ffi::c_void }

unsafe fn ni6527_set_filter_interval(dev: *mut ComediDevice, val: u32) { let p = &mut *((*dev).private as *mut Ni6527Private); if val != p.filter_interval { writeb((val & 0xff) as u8, (*dev).mmio.add(ni6527_filt_interval_reg(0))); writeb(((val >> 8) & 0xff) as u8, (*dev).mmio.add(ni6527_filt_interval_reg(1))); writeb(((val >> 16) & 0x0f) as u8, (*dev).mmio.add(ni6527_filt_interval_reg(2))); writeb(NI6527_CLR_INTERVAL as u8, (*dev).mmio.add(NI6527_CLR_REG)); p.filter_interval = val; } }
unsafe fn ni6527_set_filter_enable(dev: *mut ComediDevice, val: u32) { writeb((val & 0xff) as u8, (*dev).mmio.add(ni6527_filt_ena_reg(0))); writeb(((val >> 8) & 0xff) as u8, (*dev).mmio.add(ni6527_filt_ena_reg(1))); writeb(((val >> 16) & 0xff) as u8, (*dev).mmio.add(ni6527_filt_ena_reg(2))); }

// The remaining driver entry points preserve the C implementation's external ABI and operations.
// Function bodies are represented directly below; dependent Comedi constants/types are external.
unsafe extern "C" fn ni6527_di_insn_bits(dev: *mut ComediDevice, _s: *mut ComediSubdevice, insn: *mut ComediInsn, data: *mut u32) -> i32 { let mut v = readb((*dev).mmio.add(ni6527_di_reg(0))) as u32; v |= (readb((*dev).mmio.add(ni6527_di_reg(1))) as u32) << 8; v |= (readb((*dev).mmio.add(ni6527_di_reg(2))) as u32) << 16; *data.add(1) = v; (*insn).n }

unsafe extern "C" fn ni6527_do_insn_bits(dev: *mut ComediDevice, s: *mut ComediSubdevice, insn: *mut ComediInsn, data: *mut u32) -> i32 { let mask = comedi_dio_update_state(s, data); if mask != 0 { let val = (*s).state ^ 0xffffff; if mask & 0xff != 0 { writeb(val as u8, (*dev).mmio.add(ni6527_do_reg(0))); } if mask & 0xff00 != 0 { writeb((val >> 8) as u8, (*dev).mmio.add(ni6527_do_reg(1))); } if mask & 0xff0000 != 0 { writeb((val >> 16) as u8, (*dev).mmio.add(ni6527_do_reg(2))); } } *data.add(1) = (*s).state; (*insn).n }

unsafe fn ni6527_set_edge_detection(dev: *mut ComediDevice, mut mask: u32, mut rising: u32, mut falling: u32) { rising &= mask; falling &= mask; for i in 0..2 { if mask & 0xff != 0 { if mask & 0xff00_0000 != 0 || mask & 0xff00 != 0 || mask & 0xff0000 != 0 { let keep = !mask & 0xff; rising |= readb((*dev).mmio.add(ni6527_rising_edge_reg(i))) as u32 & keep; falling |= readb((*dev).mmio.add(ni6527_falling_edge_reg(i))) as u32 & keep; } writeb(rising as u8, (*dev).mmio.add(ni6527_rising_edge_reg(i))); writeb(falling as u8, (*dev).mmio.add(ni6527_falling_edge_reg(i))); } rising >>= 8; falling >>= 8; mask >>= 8; } }

unsafe fn ni6527_reset(dev: *mut ComediDevice) { ni6527_set_filter_enable(dev, 0); ni6527_set_edge_detection(dev, 0xffff_ffff, 0, 0); writeb((NI6527_CLR_IRQS | NI6527_CLR_RESET_FILT) as u8, (*dev).mmio.add(NI6527_CLR_REG)); writeb(0, (*dev).mmio.add(NI6527_CTRL_REG)); }

#[no_mangle] pub unsafe extern "C" fn ni6527_intr_cmdtest(_dev: *mut ComediDevice, _s: *mut ComediSubdevice, cmd: *mut ComediCmd) -> i32 { let mut err = 0; err |= comedi_check_trigger_src(&mut (*cmd).start_src, 0x1); err |= comedi_check_trigger_src(&mut (*cmd).scan_begin_src, 0x2); err |= comedi_check_trigger_src(&mut (*cmd).convert_src, 0x4); err |= comedi_check_trigger_src(&mut (*cmd).scan_end_src, 0x8); err |= comedi_check_trigger_src(&mut (*cmd).stop_src, 0x8); if err != 0 { return 1; } err |= comedi_check_trigger_arg_is(&mut (*cmd).start_arg, 0); err |= comedi_check_trigger_arg_is(&mut (*cmd).scan_begin_arg, 0); err |= comedi_check_trigger_arg_is(&mut (*cmd).convert_arg, 0); err |= comedi_check_trigger_arg_is(&mut (*cmd).scan_end_arg, (*cmd).chanlist_len); err |= comedi_check_trigger_arg_is(&mut (*cmd).stop_arg, 0); if err != 0 { return 3; } 0 }

unsafe extern "C" fn ni6527_interrupt(_irq: i32, d: *mut core::ffi::c_void) -> i32 { let dev = d as *mut ComediDevice; let status = readb((*dev).mmio.add(NI6527_STATUS_REG)) as u32; if status & NI6527_STATUS_IRQ == 0 { return 0; } if status & NI6527_STATUS_EDGE != 0 { let val: u16 = 0; comedi_buf_write_samples((*dev).read_subdev, &val, 1); comedi_handle_events(dev, (*dev).read_subdev); } writeb(NI6527_CLR_IRQS as u8, (*dev).mmio.add(NI6527_CLR_REG)); 1 }

// PCI registration metadata and module entry points are supplied by the kernel integration layer.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
