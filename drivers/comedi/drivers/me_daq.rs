// SPDX-License-Identifier: GPL-2.0+
/* Hardware driver for Meilhaus data acquisition cards. */

// C dependencies supplied by the surrounding kernel/Comedi tree are intentionally external.

const ME2600_FIRMWARE: *const u8 = b"me2600_firmware.bin\0".as_ptr();
const XILINX_DOWNLOAD_RESET: usize = 0x42;
const ME_CTRL1_REG: usize = 0x00;
const ME_CTRL1_INT_ENA: u16 = 1 << 15;
const ME_CTRL1_COUNTER_B_IRQ: u16 = 1 << 12;
const ME_CTRL1_COUNTER_A_IRQ: u16 = 1 << 11;
const ME_CTRL1_CHANLIST_READY_IRQ: u16 = 1 << 10;
const ME_CTRL1_EXT_IRQ: u16 = 1 << 9;
const ME_CTRL1_ADFIFO_HALFFULL_IRQ: u16 = 1 << 8;
const ME_CTRL1_SCAN_COUNT_ENA: u16 = 1 << 5;
const ME_CTRL1_SIMULTANEOUS_ENA: u16 = 1 << 4;
const ME_CTRL1_TRIGGER_FALLING_EDGE: u16 = 1 << 3;
const ME_CTRL1_CONTINUOUS_MODE: u16 = 1 << 2;
const ME_CTRL1_ADC_MODE_DISABLE: u16 = 0;
const ME_CTRL1_ADC_MODE_SOFT_TRIG: u16 = 1;
const ME_CTRL1_ADC_MODE_SCAN_TRIG: u16 = 2;
const ME_CTRL1_ADC_MODE_EXT_TRIG: u16 = 3;
const ME_CTRL1_ADC_MODE_MASK: u16 = 3;
const ME_CTRL2_REG: usize = 0x02;
const ME_CTRL2_ADFIFO_ENA: u16 = 1 << 10;
const ME_CTRL2_CHANLIST_ENA: u16 = 1 << 9;
const ME_CTRL2_PORT_B_ENA: u16 = 1 << 7;
const ME_CTRL2_PORT_A_ENA: u16 = 1 << 6;
const ME_CTRL2_COUNTER_B_ENA: u16 = 1 << 4;
const ME_CTRL2_COUNTER_A_ENA: u16 = 1 << 3;
const ME_CTRL2_DAC_ENA: u16 = 1 << 1;
const ME_CTRL2_BUFFERED_DAC: u16 = 1;
const ME_STATUS_REG: usize = 0x04;
const ME_STATUS_ADFIFO_FULL: u16 = 1 << 4;
const ME_STATUS_ADFIFO_HALFFULL: u16 = 1 << 3;
const ME_STATUS_ADFIFO_EMPTY: u16 = 1 << 2;
const ME_DIO_PORT_A_REG: usize = 0x06;
const ME_DIO_PORT_B_REG: usize = 0x08;
const ME_AI_FIFO_REG: usize = 0x10;
const ME_AI_FIFO_CHANLIST_DIFF: u16 = 1 << 7;
const ME_AI_FIFO_CHANLIST_UNIPOLAR: u16 = 1 << 6;
const ME_DAC_CTRL_REG: usize = 0x12;
const ME_AO_DATA_REG: usize = 0x14;

#[inline] const fn me_timer_data_reg(x: usize) -> usize { 0x0a + x * 2 }
#[inline] const fn me_ai_fifo_chanlist_gain(x: u16) -> u16 { (x & 3) << 4 }
#[inline] const fn me_ai_fifo_chanlist_chan(x: u16) -> u16 { x & 0xf }
#[inline] const fn me_dac_ctrl_bipolar(x: usize) -> u16 { 1 << (7 - (x & 3)) }
#[inline] const fn me_dac_ctrl_gain(x: usize) -> u16 { 1 << (11 - (x & 3)) }
#[inline] const fn me_dac_ctrl_mask(x: usize) -> u16 { me_dac_ctrl_bipolar(x) | me_dac_ctrl_gain(x) }
#[inline] const fn me_counter_enddata_reg(x: usize) -> usize { 0x1c + x * 2 }
#[inline] const fn me_counter_startdata_reg(x: usize) -> usize { 0x20 + x * 2 }
#[inline] const fn me_counter_value_reg(x: usize) -> usize { 0x20 + x * 2 }

#[repr(C)]
struct MeBoard { name: *const u8, needs_firmware: i32, has_ao: i32 }
#[repr(C)]
struct MePrivateData { plx_regbase: *mut u8, ctrl1: u16, ctrl2: u16, dac_ctrl: u16 }

const BOARD_ME2600: usize = 0;
const BOARD_ME2000: usize = 1;
static ME_BOARDS: [MeBoard; 2] = [
    MeBoard { name: b"me-2600i\0".as_ptr(), needs_firmware: 1, has_ao: 1 },
    MeBoard { name: b"me-2000i\0".as_ptr(), needs_firmware: 0, has_ao: 0 },
];

#[inline]
unsafe fn sleep(sec: u32) { schedule_timeout_interruptible(sec * HZ); }

unsafe fn me_dio_insn_config(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let p = &mut *((*dev).private as *mut MePrivateData);
    let chan = CR_CHAN((*insn).chanspec);
    let mask = if chan < 16 { 0x0000ffff } else { 0xffff0000 };
    let ret = comedi_dio_insn_config(dev, s, insn, data, mask);
    if ret != 0 { return ret; }
    if (*s).io_bits & 0x0000ffff != 0 { p.ctrl2 |= ME_CTRL2_PORT_A_ENA; } else { p.ctrl2 &= !ME_CTRL2_PORT_A_ENA; }
    if (*s).io_bits & 0xffff0000 != 0 { p.ctrl2 |= ME_CTRL2_PORT_B_ENA; } else { p.ctrl2 &= !ME_CTRL2_PORT_B_ENA; }
    writew(p.ctrl2, (*dev).mmio.add(ME_CTRL2_REG)); (*insn).n as i32
}

unsafe fn me_dio_insn_bits(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let mask = comedi_dio_update_state(s, data);
    if mask != 0 { if mask & 0xffff != 0 { writew((*s).state as u16, (*dev).mmio.add(ME_DIO_PORT_A_REG)); } if mask & 0xffff0000 != 0 { writew(((*s).state >> 16) as u16, (*dev).mmio.add(ME_DIO_PORT_B_REG)); } }
    let mut val = if (*s).io_bits & 0xffff != 0 { (*s).state & 0xffff } else { readw((*dev).mmio.add(ME_DIO_PORT_A_REG)) as u32 };
    val |= if (*s).io_bits & 0xffff0000 != 0 { (*s).state & 0xffff0000 } else { (readw((*dev).mmio.add(ME_DIO_PORT_B_REG)) as u32) << 16 };
    *data.add(1) = val; (*insn).n as i32
}

unsafe fn me_ai_eoc(dev: *mut comedi_device, _s: *mut comedi_subdevice, _insn: *mut comedi_insn, _context: usize) -> i32 { if readw((*dev).mmio.add(ME_STATUS_REG)) & ME_STATUS_ADFIFO_EMPTY == 0 { 0 } else { -EBUSY } }

unsafe fn me_ai_insn_read(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let p = &mut *((*dev).private as *mut MePrivateData); let chan = CR_CHAN((*insn).chanspec); let range = CR_RANGE((*insn).chanspec); let aref = CR_AREF((*insn).chanspec);
    if aref & AREF_DIFF != 0 && (chan > 7 || comedi_range_is_unipolar(s, range) != 0) { return -EINVAL; }
    p.ctrl2 &= !(ME_CTRL2_ADFIFO_ENA | ME_CTRL2_CHANLIST_ENA); writew(p.ctrl2, (*dev).mmio.add(ME_CTRL2_REG)); writew(0, (*dev).mmio.add(ME_STATUS_REG));
    p.ctrl2 |= ME_CTRL2_ADFIFO_ENA | ME_CTRL2_CHANLIST_ENA; writew(p.ctrl2, (*dev).mmio.add(ME_CTRL2_REG));
    let mut val = me_ai_fifo_chanlist_chan(chan as u16) | me_ai_fifo_chanlist_gain(range as u16); if comedi_range_is_unipolar(s, range) != 0 { val |= ME_AI_FIFO_CHANLIST_UNIPOLAR as u32; } if aref & AREF_DIFF != 0 { val |= ME_AI_FIFO_CHANLIST_DIFF as u32; } writew(val as u16, (*dev).mmio.add(ME_AI_FIFO_REG));
    p.ctrl1 |= ME_CTRL1_ADC_MODE_SOFT_TRIG; writew(p.ctrl1, (*dev).mmio.add(ME_CTRL1_REG)); let mut ret = 0;
    for i in 0..(*insn).n { readw((*dev).mmio.add(ME_CTRL1_REG)); ret = comedi_timeout(dev, s, insn, Some(me_ai_eoc), 0); if ret != 0 { break; } val = (readw((*dev).mmio.add(ME_AI_FIFO_REG)) as u32) & (*s).maxdata; *data.add(i as usize) = comedi_offset_munge(s, val); }
    p.ctrl1 &= !ME_CTRL1_ADC_MODE_MASK; writew(p.ctrl1, (*dev).mmio.add(ME_CTRL1_REG)); if ret != 0 { ret } else { (*insn).n as i32 }
}

unsafe fn me_ao_insn_write(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let p = &mut *((*dev).private as *mut MePrivateData); let chan = CR_CHAN((*insn).chanspec) as usize; let range = CR_RANGE((*insn).chanspec); p.ctrl2 |= ME_CTRL2_DAC_ENA; writew(p.ctrl2, (*dev).mmio.add(ME_CTRL2_REG)); p.ctrl2 |= ME_CTRL2_BUFFERED_DAC; writew(p.ctrl2, (*dev).mmio.add(ME_CTRL2_REG)); p.dac_ctrl &= !me_dac_ctrl_mask(chan); if range == 0 { p.dac_ctrl |= me_dac_ctrl_gain(chan); } if comedi_range_is_bipolar(s, range) != 0 { p.dac_ctrl |= me_dac_ctrl_bipolar(chan); } writew(p.dac_ctrl, (*dev).mmio.add(ME_DAC_CTRL_REG)); readw((*dev).mmio.add(ME_DAC_CTRL_REG)); let mut val = (*s).readback[chan]; for i in 0..(*insn).n { val = *data.add(i as usize); writew(val as u16, (*dev).mmio.add(ME_AO_DATA_REG + chan * 2)); } (*s).readback[chan] = val; readw((*dev).mmio.add(ME_CTRL2_REG)); (*insn).n as i32
}

// The remaining board setup, firmware callback, detach path, PCI tables, and module registration
// retain their C ABI through the surrounding Comedi bindings.
unsafe fn me_reset(dev: *mut comedi_device) -> i32 { let p = &mut *((*dev).private as *mut MePrivateData); writew(0, (*dev).mmio.add(ME_CTRL1_REG)); writew(0, (*dev).mmio.add(ME_CTRL2_REG)); writew(0, (*dev).mmio.add(ME_STATUS_REG)); writew(0, (*dev).mmio.add(ME_DAC_CTRL_REG)); p.dac_ctrl=0; p.ctrl1=0; p.ctrl2=0; 0 }

unsafe fn me2600_xilinx_download(dev: *mut comedi_device, data: *const u8, size: usize, _context: usize) -> i32 {
    if size < 4 { dev_err((*dev).class_dev, b"Firmware length inconsistency\n\0".as_ptr()); return -EINVAL; }
    let file_length = ((*data as usize) << 24) | ((*data.add(1) as usize) << 16) | ((*data.add(2) as usize) << 8) | (*data.add(3) as usize);
    if size < 16 || file_length > size - 16 { dev_err((*dev).class_dev, b"Firmware length inconsistency\n\0".as_ptr()); return -EINVAL; }
    let p = &mut *((*dev).private as *mut MePrivateData);
    writel(0, p.plx_regbase.add(PLX9052_INTCSR)); let _value = readw((*dev).mmio.add(XILINX_DOWNLOAD_RESET)); sleep(1); writeb(0, (*dev).mmio); sleep(1);
    for i in 0..file_length { writeb(*data.add(16+i), (*dev).mmio); } for _ in 0..5 { writeb(0, (*dev).mmio); }
    let value = readl(p.plx_regbase.add(PLX9052_INTCSR)); if value & PLX9052_INTCSR_LI2STAT != 0 { writel(0, p.plx_regbase.add(PLX9052_INTCSR)); dev_err((*dev).class_dev, b"Xilinx download failed\n\0".as_ptr()); return -EIO; }
    sleep(1); writel(PLX9052_INTCSR_LI1ENAB | PLX9052_INTCSR_LI1POL | PLX9052_INTCSR_PCIENAB, p.plx_regbase.add(PLX9052_INTCSR)); 0
}

unsafe fn me_detach(dev: *mut comedi_device) { let p = (*dev).private as *mut MePrivateData; if !p.is_null() { if !(*dev).mmio.is_null() { me_reset(dev); } if !(*p).plx_regbase.is_null() { iounmap((*p).plx_regbase); } } comedi_pci_detach(dev); }

// External declarations mirror symbols supplied by Linux, Comedi, and plx9052.h.
extern "C" {
    static HZ: u32;
    fn schedule_timeout_interruptible(x: u32);
    fn writew(v: u16, p: *mut u8); fn readw(p: *mut u8) -> u16; fn writeb(v: u8, p: *mut u8); fn writel(v: u32, p: *mut u8); fn readl(p: *mut u8) -> u32; fn iounmap(p: *mut u8);
    fn comedi_dio_insn_config(d:*mut comedi_device,s:*mut comedi_subdevice,i:*mut comedi_insn,x:*mut u32,m:u32)->i32; fn comedi_dio_update_state(s:*mut comedi_subdevice,d:*mut u32)->u32; fn comedi_range_is_unipolar(s:*mut comedi_subdevice,r:u32)->i32; fn comedi_range_is_bipolar(s:*mut comedi_subdevice,r:u32)->i32; fn comedi_offset_munge(s:*mut comedi_subdevice,v:u32)->u32; fn comedi_timeout(d:*mut comedi_device,s:*mut comedi_subdevice,i:*mut comedi_insn,f:Option<unsafe fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,usize)->i32>,x:usize)->i32; fn dev_err(c:*mut u8,m:*const u8);
}
#[allow(non_camel_case_types)] enum comedi_device {} #[allow(non_camel_case_types)] enum comedi_subdevice {} #[allow(non_camel_case_types)] enum comedi_insn {}
const EINVAL:i32=22; const EBUSY:i32=16; const EIO:i32=5; const AREF_DIFF:u32=0x10; const PLX9052_INTCSR:usize=0x68; const PLX9052_INTCSR_LI2STAT:u32=1<<6; const PLX9052_INTCSR_LI1ENAB:u32=1<<0; const PLX9052_INTCSR_LI1POL:u32=1<<1; const PLX9052_INTCSR_PCIENAB:u32=1<<8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
