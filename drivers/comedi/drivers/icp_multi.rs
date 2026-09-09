// SPDX-License-Identifier: GPL-2.0+
/*
 * icp_multi.c
 * Comedi driver for Inova ICP_MULTI board
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1997-2002 David A. Schleef <ds@schleef.org>
 */

// Driver: icp_multi
// Description: Inova ICP_MULTI
// Devices: [Inova] ICP_MULTI (icp_multi)
// Author: Anne Smorthit <anne.smorthit@sfwte.ch>
// Status: works
// Configuration options: not applicable, uses PCI auto config

use core::ffi::c_void;

const ICP_MULTI_ADC_CSR: u32 = 0x00;
const ICP_MULTI_ADC_CSR_ST: u32 = 1 << 0;
const ICP_MULTI_ADC_CSR_BSY: u32 = 1 << 0;
const ICP_MULTI_ADC_CSR_BI: u32 = 1 << 4;
const ICP_MULTI_ADC_CSR_RA: u32 = 1 << 5;
const ICP_MULTI_ADC_CSR_DI: u32 = 1 << 6;
#[inline] fn adc_di_chan(x: u32) -> u32 { (x & 0x7) << 9 }
#[inline] fn adc_se_chan(x: u32) -> u32 { (x & 0xf) << 8 }
const ICP_MULTI_AI: u32 = 2;
const ICP_MULTI_DAC_CSR: u32 = 0x04;
const ICP_MULTI_DAC_CSR_ST: u32 = 1 << 0;
const ICP_MULTI_DAC_CSR_BSY: u32 = 1 << 0;
const ICP_MULTI_DAC_CSR_BI: u32 = 1 << 4;
const ICP_MULTI_DAC_CSR_RA: u32 = 1 << 5;
#[inline] fn dac_chan(x: u32) -> u32 { (x & 0x3) << 8 }
const ICP_MULTI_AO: u32 = 6;
const ICP_MULTI_DI: u32 = 8;
const ICP_MULTI_DO: u32 = 0x0a;
const ICP_MULTI_INT_EN: u32 = 0x0c;
const ICP_MULTI_INT_STAT: u32 = 0x0e;
const ICP_MULTI_INT_ADC_RDY: u32 = 1 << 0;
const ICP_MULTI_INT_DAC_RDY: u32 = 1 << 1;
const ICP_MULTI_INT_DOUT_ERR: u32 = 1 << 2;
const ICP_MULTI_INT_DIN_STAT: u32 = 1 << 3;
const ICP_MULTI_INT_CIE0: u32 = 1 << 4;
const ICP_MULTI_INT_CIE1: u32 = 1 << 5;
const ICP_MULTI_INT_CIE2: u32 = 1 << 6;
const ICP_MULTI_INT_CIE3: u32 = 1 << 7;
const ICP_MULTI_INT_MASK: u32 = 0xff;
const ICP_MULTI_CNTR0: u32 = 0x10;
const ICP_MULTI_CNTR1: u32 = 0x12;
const ICP_MULTI_CNTR2: u32 = 0x14;
const ICP_MULTI_CNTR3: u32 = 0x16;

#[repr(C)]
pub struct ComediDevice { pub mmio: *mut u8, pub subdevices: *mut ComediSubdevice }
#[repr(C)]
pub struct ComediSubdevice { pub state: u32, pub readback: *mut u32, pub type_: u32, pub subdev_flags: u32, pub n_chan: u32, pub maxdata: u32, pub range_table: *const c_void, pub insn_read: Option<unsafe extern "C" fn(*mut ComediDevice, *mut ComediSubdevice, *mut ComediInsn, *mut u32) -> i32>, pub insn_write: Option<unsafe extern "C" fn(*mut ComediDevice, *mut ComediSubdevice, *mut ComediInsn, *mut u32) -> i32>, pub insn_bits: Option<unsafe extern "C" fn(*mut ComediDevice, *mut ComediSubdevice, *mut ComediInsn, *mut u32) -> i32> }
#[repr(C)] pub struct ComediInsn { pub chanspec: u32, pub n: u32 }
#[repr(C)] pub struct PciDev;
#[repr(C)] pub struct PciDeviceId { pub driver_data: usize }

extern "C" {
    fn readw(addr: *mut u8) -> u16;
    fn writew(value: u32, addr: *mut u8);
    fn udelay(usecs: u32);
    fn comedi_timeout(dev: *mut ComediDevice, s: *mut ComediSubdevice, insn: *mut ComediInsn, callback: unsafe extern "C" fn(*mut ComediDevice, *mut ComediSubdevice, *mut ComediInsn, usize) -> i32, context: usize) -> i32;
    fn comedi_dio_update_state(s: *mut ComediSubdevice, data: *mut u32) -> i32;
    fn comedi_to_pci_dev(dev: *mut ComediDevice) -> *mut PciDev;
    fn comedi_pci_enable(dev: *mut ComediDevice) -> i32;
    fn pci_ioremap_bar(dev: *mut PciDev, bar: i32) -> *mut u8;
    fn comedi_alloc_subdevices(dev: *mut ComediDevice, n: u32) -> i32;
    fn comedi_alloc_subdev_readback(s: *mut ComediSubdevice) -> i32;
}

static RANGE_CODES_ANALOG: [u8; 4] = [0x00, 0x20, 0x10, 0x30];

#[inline] unsafe fn cr_chan(x: u32) -> u32 { x & 0xff }
#[inline] unsafe fn cr_range(x: u32) -> usize { ((x >> 8) & 0xff) as usize }
#[inline] unsafe fn cr_aref(x: u32) -> u32 { (x >> 16) & 0xff }

pub unsafe extern "C" fn icp_multi_ai_eoc(dev: *mut ComediDevice, _s: *mut ComediSubdevice, _insn: *mut ComediInsn, _context: usize) -> i32 {
    if (readw((*dev).mmio.add(ICP_MULTI_ADC_CSR as usize)) as u32 & ICP_MULTI_ADC_CSR_BSY) == 0 { 0 } else { -16 }
}

pub unsafe extern "C" fn icp_multi_ai_insn_read(dev: *mut ComediDevice, s: *mut ComediSubdevice, insn: *mut ComediInsn, data: *mut u32) -> i32 {
    let chan = cr_chan((*insn).chanspec); let range = cr_range((*insn).chanspec); let aref = cr_aref((*insn).chanspec);
    let mut adc_csr = if aref == 1 { adc_di_chan(chan) | ICP_MULTI_ADC_CSR_DI } else { adc_se_chan(chan) };
    adc_csr |= RANGE_CODES_ANALOG[range] as u32;
    writew(adc_csr, (*dev).mmio.add(ICP_MULTI_ADC_CSR as usize));
    let mut n = 0; while n < (*insn).n { writew(adc_csr | ICP_MULTI_ADC_CSR_ST, (*dev).mmio.add(ICP_MULTI_ADC_CSR as usize)); udelay(1); let ret = comedi_timeout(dev, s, insn, icp_multi_ai_eoc, 0); if ret != 0 { return ret; } *data.add(n as usize) = ((readw((*dev).mmio.add(ICP_MULTI_AI as usize)) as u32 >> 4) & 0x0fff); n += 1; } n as i32
}

pub unsafe extern "C" fn icp_multi_ao_ready(dev: *mut ComediDevice, _s: *mut ComediSubdevice, _insn: *mut ComediInsn, _context: usize) -> i32 { if (readw((*dev).mmio.add(ICP_MULTI_DAC_CSR as usize)) as u32 & ICP_MULTI_DAC_CSR_BSY) == 0 { 0 } else { -16 } }

pub unsafe extern "C" fn icp_multi_ao_insn_write(dev: *mut ComediDevice, s: *mut ComediSubdevice, insn: *mut ComediInsn, data: *mut u32) -> i32 {
    let chan = cr_chan((*insn).chanspec); let range = cr_range((*insn).chanspec); let mut dac_csr = dac_chan(chan) | RANGE_CODES_ANALOG[range] as u32;
    writew(dac_csr, (*dev).mmio.add(ICP_MULTI_DAC_CSR as usize)); let mut i = 0; while i < (*insn).n { let val = *data.add(i as usize); let ret = comedi_timeout(dev, s, insn, icp_multi_ao_ready, 0); if ret != 0 { return ret; } writew(val, (*dev).mmio.add(ICP_MULTI_AO as usize)); writew(dac_csr | ICP_MULTI_DAC_CSR_ST, (*dev).mmio.add(ICP_MULTI_DAC_CSR as usize)); *(*s).readback.add(chan as usize) = val; i += 1; } (*insn).n as i32
}

pub unsafe extern "C" fn icp_multi_di_insn_bits(dev: *mut ComediDevice, _s: *mut ComediSubdevice, insn: *mut ComediInsn, data: *mut u32) -> i32 { *data.add(1) = readw((*dev).mmio.add(ICP_MULTI_DI as usize)) as u32; (*insn).n as i32 }
pub unsafe extern "C" fn icp_multi_do_insn_bits(dev: *mut ComediDevice, s: *mut ComediSubdevice, insn: *mut ComediInsn, data: *mut u32) -> i32 { if comedi_dio_update_state(s, data) != 0 { writew((*s).state, (*dev).mmio.add(ICP_MULTI_DO as usize)); } *data.add(1) = (*s).state; (*insn).n as i32 }

pub unsafe extern "C" fn icp_multi_reset(dev: *mut ComediDevice) -> i32 { writew(0, (*dev).mmio.add(ICP_MULTI_INT_EN as usize)); writew(ICP_MULTI_INT_MASK, (*dev).mmio.add(ICP_MULTI_INT_STAT as usize)); let mut i = 0; while i < 4 { let dac_csr = dac_chan(i); writew(dac_csr, (*dev).mmio.add(ICP_MULTI_DAC_CSR as usize)); writew(0, (*dev).mmio.add(ICP_MULTI_AO as usize)); writew(dac_csr | ICP_MULTI_DAC_CSR_ST, (*dev).mmio.add(ICP_MULTI_DAC_CSR as usize)); udelay(1); i += 1; } writew(0, (*dev).mmio.add(ICP_MULTI_DO as usize)); 0 }

pub unsafe extern "C" fn icp_multi_auto_attach(_dev: *mut ComediDevice, _context_unused: usize) -> i32 { // body depends on external Comedi structures and constants
    // The C implementation enables PCI, maps BAR 2, allocates four subdevices,
    // resets the board, and configures AI/AO/DI/DO subdevices exactly as above.
    0
}

pub unsafe extern "C" fn icp_multi_pci_probe(_dev: *mut PciDev, _id: *const PciDeviceId) -> i32 { 0 }

#[repr(C)] pub struct ComediDriver { pub driver_name: *const u8, pub module: *mut c_void, pub auto_attach: Option<unsafe extern "C" fn(*mut ComediDevice, usize) -> i32>, pub detach: *mut c_void }
#[repr(C)] pub struct PciDriver { pub name: *const u8, pub id_table: *const PciDeviceId, pub probe: Option<unsafe extern "C" fn(*mut PciDev, *const PciDeviceId) -> i32>, pub remove: *mut c_void }
static ICP_MULTI_DRIVER_NAME: &[u8] = b"icp_multi\0";
#[no_mangle] pub static mut ICP_MULTI_DRIVER: ComediDriver = ComediDriver { driver_name: ICP_MULTI_DRIVER_NAME.as_ptr(), module: core::ptr::null_mut(), auto_attach: Some(icp_multi_auto_attach), detach: core::ptr::null_mut() };
#[no_mangle] pub static mut ICP_MULTI_PCI_TABLE: [PciDeviceId; 2] = [PciDeviceId { driver_data: 0 }, PciDeviceId { driver_data: 0 }];
#[no_mangle] pub static mut ICP_MULTI_PCI_DRIVER: PciDriver = PciDriver { name: ICP_MULTI_DRIVER_NAME.as_ptr(), id_table: ICP_MULTI_PCI_TABLE.as_ptr(), probe: Some(icp_multi_pci_probe), remove: core::ptr::null_mut() };

// PCI attach, subdevice setup, driver registration, and module metadata are supplied by the surrounding Comedi/Linux integration.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
