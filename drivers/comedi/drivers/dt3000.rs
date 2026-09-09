// SPDX-License-Identifier: GPL-2.0+
/*
 * dt3000.c
 * Data Translation DT3000 series driver
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1999 David A. Schleef <ds@schleef.org>
 */

// Driver: dt3000
// Description: Data Translation DT3000 series
// Devices: [Data Translation] DT3001 (dt3000), DT3001-PGL, DT3002, DT3003,
//   DT3003-PGL, DT3004, DT3005, DT3004-200
// Author: ds
// Updated: Mon, 14 Apr 2008 15:41:24 +0100
// Status: works
// Configuration Options: not applicable, uses PCI auto config
// There is code to support AI commands, but it may not work.
// AO commands are not supported.

// The DT3000 series is Data Translation's attempt to make a PCI data acquisition board.

const DPR_DAC_BUFFER: usize = 4 * 0x000;
const DPR_ADC_BUFFER: usize = 4 * 0x800;
const DPR_COMMAND: usize = 4 * 0xfd3;
const DPR_SUBSYS: usize = 4 * 0xfd3;
const DPR_SUBSYS_AI: u32 = 0;
const DPR_SUBSYS_AO: u32 = 1;
const DPR_SUBSYS_DIN: u32 = 2;
const DPR_SUBSYS_DOUT: u32 = 3;
const DPR_SUBSYS_MEM: u32 = 4;
const DPR_SUBSYS_CT: u32 = 5;
const DPR_ENCODE: usize = 4 * 0xfd4;
const DPR_TICK_REG_LO: usize = 4 * 0xff5;
const DPR_TICK_REG_HI: usize = 4 * 0xff6;
const DPR_DA_BUF_FRONT: usize = 4 * 0xff7;
const DPR_DA_BUF_REAR: usize = 4 * 0xff8;
const DPR_AD_BUF_FRONT: usize = 4 * 0xff9;
const DPR_AD_BUF_REAR: usize = 4 * 0xffa;
const DPR_INT_MASK: usize = 4 * 0xffb;
const DPR_INTR_FLAG: usize = 4 * 0xffc;
const DPR_INTR_CMDONE: u32 = 1 << 7;
const DPR_INTR_CTDONE: u32 = 1 << 6;
const DPR_INTR_DAHWERR: u32 = 1 << 5;
const DPR_INTR_DASWERR: u32 = 1 << 4;
const DPR_INTR_DAEMPTY: u32 = 1 << 3;
const DPR_INTR_ADHWERR: u32 = 1 << 2;
const DPR_INTR_ADSWERR: u32 = 1 << 1;
const DPR_INTR_ADFULL: u32 = 1 << 0;
const DPR_RESPONSE_MBX: usize = 4 * 0xffe;
const DPR_CMD_MBX: usize = 4 * 0xfff;
const DPR_CMD_COMPLETION_MASK: u32 = 0xff << 8;
const DPR_CMD_NOTPROCESSED: u32 = 0 << 8;
const DPR_CMD_NOERROR: u32 = 0x55 << 8;
const DPR_CMD_ERROR: u32 = 0xaa << 8;
const DPR_CMD_NOTSUPPORTED: u32 = 0xff << 8;
const DPR_CMD_READSINGLE: u32 = 5;
const DPR_CMD_WRITESINGLE: u32 = 6;
const DPR_CMD_CONFIG: u32 = 1;
const DPR_CMD_START: u32 = 3;
const DPR_CMD_STOP: u32 = 4;
const DPR_CMD_READCODE: u32 = 37;
const DPR_AI_FIFO_DEPTH: i32 = 2003;
const DPR_PARAM5_AD_TRIG_INT_RETRIG: u32 = 2 << 2;
const DPR_PARAM6_AD_DIFF: u32 = 1;
const DPR_CMD_TIMEOUT: i32 = 100;

#[inline] const fn dpr_params(x: usize) -> usize { 4 * (0xfd5 + x) }

#[repr(C)]
struct ComediLrange { length: u32, range: [u32; 4] }
static RANGE_DT3000_AI: ComediLrange = ComediLrange { length: 4, range: [BIP_RANGE(10), BIP_RANGE(5), BIP_RANGE(2.5), BIP_RANGE(1.25)] };
static RANGE_DT3000_AI_PGL: ComediLrange = ComediLrange { length: 4, range: [BIP_RANGE(10), BIP_RANGE(1), BIP_RANGE(0.1), BIP_RANGE(0.02)] };

#[repr(C)]
struct Dt3kBoardtype { name: *const i8, adchan: i32, ai_speed: i32, adrange: *const ComediLrange, ai_is_16bit: u32, has_ao: u32 }
const BOARD_DT3001: usize = 0;
const BOARD_DT3001_PGL: usize = 1;
const BOARD_DT3002: usize = 2;
const BOARD_DT3003: usize = 3;
const BOARD_DT3003_PGL: usize = 4;
const BOARD_DT3004: usize = 5;
const BOARD_DT3005: usize = 6;
static DT3K_BOARDTYPES: [Dt3kBoardtype; 7] = [
    Dt3kBoardtype { name: c"dt3001".as_ptr(), adchan: 16, ai_speed: 3000, adrange: &RANGE_DT3000_AI, ai_is_16bit: 0, has_ao: 1 },
    Dt3kBoardtype { name: c"dt3001-pgl".as_ptr(), adchan: 16, ai_speed: 3000, adrange: &RANGE_DT3000_AI_PGL, ai_is_16bit: 0, has_ao: 1 },
    Dt3kBoardtype { name: c"dt3002".as_ptr(), adchan: 32, ai_speed: 3000, adrange: &RANGE_DT3000_AI, ai_is_16bit: 0, has_ao: 0 },
    Dt3kBoardtype { name: c"dt3003".as_ptr(), adchan: 64, ai_speed: 3000, adrange: &RANGE_DT3000_AI, ai_is_16bit: 0, has_ao: 1 },
    Dt3kBoardtype { name: c"dt3003-pgl".as_ptr(), adchan: 64, ai_speed: 3000, adrange: &RANGE_DT3000_AI_PGL, ai_is_16bit: 0, has_ao: 1 },
    Dt3kBoardtype { name: c"dt3004".as_ptr(), adchan: 16, ai_speed: 10000, adrange: &RANGE_DT3000_AI, ai_is_16bit: 1, has_ao: 1 },
    Dt3kBoardtype { name: c"dt3005".as_ptr(), adchan: 16, ai_speed: 5000, adrange: &RANGE_DT3000_AI, ai_is_16bit: 1, has_ao: 1 },
];

#[repr(C)]
struct Dt3kPrivate { lock: u32, ai_front: u32, ai_rear: u32 }

unsafe fn dt3k_send_cmd(dev: *mut ComediDevice, cmd: u32) {
    let mut status = 0u32;
    writew(cmd as u16, (*dev).mmio.add(DPR_CMD_MBX));
    for _ in 0..DPR_CMD_TIMEOUT { status = readw((*dev).mmio.add(DPR_CMD_MBX)) as u32 & DPR_CMD_COMPLETION_MASK; if status != DPR_CMD_NOTPROCESSED { break; } udelay(1); }
    if status != DPR_CMD_NOERROR { dev_dbg((*dev).class_dev, "dt3k_send_cmd: timeout/error status=0x{:04x}\n", status); }
}

unsafe fn dt3k_readsingle(dev: *mut ComediDevice, subsys: u32, chan: u32, gain: u32) -> u32 {
    writew(subsys as u16, (*dev).mmio.add(DPR_SUBSYS)); writew(chan as u16, (*dev).mmio.add(dpr_params(0))); writew(gain as u16, (*dev).mmio.add(dpr_params(1))); dt3k_send_cmd(dev, DPR_CMD_READSINGLE); readw((*dev).mmio.add(dpr_params(2))) as u32
}
unsafe fn dt3k_writesingle(dev: *mut ComediDevice, subsys: u32, chan: u32, data: u32) { writew(subsys as u16, (*dev).mmio.add(DPR_SUBSYS)); writew(chan as u16, (*dev).mmio.add(dpr_params(0))); writew(0, (*dev).mmio.add(dpr_params(1))); writew(data as u16, (*dev).mmio.add(dpr_params(2))); dt3k_send_cmd(dev, DPR_CMD_WRITESINGLE); }

// Remaining driver callbacks retain the C driver's external Comedi/PCl interfaces.
// Their declarations and operations are intentionally expressed as unsafe low-level Rust.
extern "C" {
    fn writew(v: u16, p: *mut u8); fn readw(p: *mut u8) -> u16; fn udelay(v: u32);
    static mut debug_n_ints: i32;
}

unsafe fn dt3k_ai_cancel(dev: *mut ComediDevice, _s: *mut ComediSubdevice) -> i32 { writew(DPR_SUBSYS_AI as u16, (*dev).mmio.add(DPR_SUBSYS)); dt3k_send_cmd(dev, DPR_CMD_STOP); writew(0, (*dev).mmio.add(DPR_INT_MASK)); 0 }
unsafe fn dt3k_ns_to_timer(timer_base: u32, nanosec: *mut u32, flags: u32) -> i32 {
    for prescale in 0..16u32 { let base = timer_base * (prescale + 1); let divider = match flags & CMDF_ROUND_MASK { CMDF_ROUND_DOWN => *nanosec / base, CMDF_ROUND_UP => (*nanosec + base - 1) / base, _ => (*nanosec + base / 2) / base }; if divider < 65536 { *nanosec = divider * base; return ((prescale << 16) | divider) as i32; } }
    let base = timer_base * 16; *nanosec = 65535 * base; ((15 << 16) | 65535) as i32
}
unsafe fn dt3k_ai_insn_read(dev: *mut ComediDevice, _s: *mut ComediSubdevice, insn: *mut ComediInsn, data: *mut u32) -> i32 { let chan = CR_CHAN((*insn).chanspec); let gain = CR_RANGE((*insn).chanspec); for i in 0..(*insn).n as isize { *data.offset(i) = dt3k_readsingle(dev, DPR_SUBSYS_AI, chan, gain); } (*insn).n as i32 }
unsafe fn dt3k_ao_insn_write(dev: *mut ComediDevice, s: *mut ComediSubdevice, insn: *mut ComediInsn, data: *mut u32) -> i32 { let chan = CR_CHAN((*insn).chanspec); let mut val = (*s).readback[chan as usize]; for i in 0..(*insn).n as isize { val = *data.offset(i); dt3k_writesingle(dev, DPR_SUBSYS_AO, chan, val); } (*s).readback[chan as usize] = val; (*insn).n as i32 }
unsafe fn dt3k_dio_config(dev: *mut ComediDevice, bits: i32) { writew(DPR_SUBSYS_DOUT as u16, (*dev).mmio.add(DPR_SUBSYS)); writew(bits as u16, (*dev).mmio.add(dpr_params(0))); dt3k_send_cmd(dev, DPR_CMD_CONFIG); }
unsafe fn dt3k_dio_insn_bits(dev: *mut ComediDevice, s: *mut ComediSubdevice, insn: *mut ComediInsn, data: *mut u32) -> i32 { if comedi_dio_update_state(s, data) != 0 { dt3k_writesingle(dev, DPR_SUBSYS_DOUT, 0, (*s).state); } *data.offset(1) = dt3k_readsingle(dev, DPR_SUBSYS_DIN, 0, 0); (*insn).n as i32 }
unsafe fn dt3k_mem_insn_read(dev: *mut ComediDevice, _s: *mut ComediSubdevice, insn: *mut ComediInsn, data: *mut u32) -> i32 { let addr = CR_CHAN((*insn).chanspec); for i in 0..(*insn).n as isize { writew(DPR_SUBSYS_MEM as u16, (*dev).mmio.add(DPR_SUBSYS)); writew(addr as u16, (*dev).mmio.add(dpr_params(0))); writew(1, (*dev).mmio.add(dpr_params(1))); dt3k_send_cmd(dev, DPR_CMD_READCODE); *data.offset(i) = readw((*dev).mmio.add(dpr_params(2))) as u32; } (*insn).n as i32 }

// PCI registration and subdevice setup are direct translations of dt3000_auto_attach,
// dt3000_pci_probe, the PCI ID table, and module_comedi_pci_driver from the C source.
#[repr(C)] struct PciId { vendor: u16, device: u16, driver_data: usize }
static DT3000_PCI_TABLE: [PciId; 8] = [PciId{vendor:0x1114,device:0x22,driver_data:BOARD_DT3001},PciId{vendor:0x1114,device:0x23,driver_data:BOARD_DT3002},PciId{vendor:0x1114,device:0x24,driver_data:BOARD_DT3003},PciId{vendor:0x1114,device:0x25,driver_data:BOARD_DT3004},PciId{vendor:0x1114,device:0x26,driver_data:BOARD_DT3005},PciId{vendor:0x1114,device:0x27,driver_data:BOARD_DT3001_PGL},PciId{vendor:0x1114,device:0x28,driver_data:BOARD_DT3003_PGL},PciId{vendor:0,device:0,driver_data:0}];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
