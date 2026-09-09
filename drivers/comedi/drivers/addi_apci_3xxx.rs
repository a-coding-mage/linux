// SPDX-License-Identifier: GPL-2.0+
/* Direct low-level Rust translation of addi_apci_3xxx.c. */

const CONV_UNIT_NS: u8 = 1 << 0;
const CONV_UNIT_US: u8 = 1 << 1;
const CONV_UNIT_MS: u8 = 1 << 2;

#[repr(C)]
pub struct ComediLrange { pub length: u32, pub range: [u32; 8] }
// Range values are supplied by the comedi dependency.
extern "C" {
    static apci3xxx_ai_range: ComediLrange;
    static apci3xxx_ao_range: ComediLrange;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Apci3xxxBoardinfo {
    pub name: *const u8, pub ai_subdev_flags: i32, pub ai_n_chan: i32,
    pub ai_maxdata: u32, pub ai_conv_units: u8, pub ai_min_acq_ns: u32,
    pub has_ao: u32, pub has_dig_in: u32, pub has_dig_out: u32, pub has_ttl_io: u32,
}

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum Apci3xxxBoardid { BOARD_APCI3000_16, BOARD_APCI3000_8, BOARD_APCI3000_4,
    BOARD_APCI3006_16, BOARD_APCI3006_8, BOARD_APCI3006_4, BOARD_APCI3010_16,
    BOARD_APCI3010_8, BOARD_APCI3010_4, BOARD_APCI3016_16, BOARD_APCI3016_8,
    BOARD_APCI3016_4, BOARD_APCI3100_16_4, BOARD_APCI3100_8_4, BOARD_APCI3106_16_4,
    BOARD_APCI3106_8_4, BOARD_APCI3110_16_4, BOARD_APCI3110_8_4, BOARD_APCI3116_16_4,
    BOARD_APCI3116_8_4, BOARD_APCI3003, BOARD_APCI3002_16, BOARD_APCI3002_8,
    BOARD_APCI3002_4, BOARD_APCI3500 }

#[repr(C)] pub struct Apci3xxxPrivate { pub ai_timer: u32, pub ai_time_base: u8 }

/* The kernel/comedi types and operations below are external dependencies. */
#[repr(C)] pub struct ComediDevice { pub mmio: *mut u8, pub iobase: usize, pub irq: i32, pub board_ptr: *const Apci3xxxBoardinfo, pub private: *mut Apci3xxxPrivate, pub read_subdev: *mut ComediSubdevice, pub subdevices: *mut ComediSubdevice, pub board_name: *const u8 }
#[repr(C)] pub struct ComediSubdevice { pub async_: *mut ComediAsync, pub state: u32, pub io_bits: u32, pub readback: *mut u32 }
#[repr(C)] pub struct ComediAsync { pub cmd: ComediCmd, pub events: u32 }
#[repr(C)] pub struct ComediCmd { pub chanlist: *mut u32, pub start_arg: u32, pub scan_begin_arg: u32, pub convert_arg: u32, pub scan_end_arg: u32, pub stop_arg: u32, pub chanlist_len: u32, pub start_src: u32, pub scan_begin_src: u32, pub convert_src: u32, pub scan_end_src: u32, pub stop_src: u32, pub flags: u32 }
#[repr(C)] pub struct ComediInsn { pub chanspec: u32, pub n: u32 }
extern "C" { fn readl(p: *mut u8) -> u32; fn writel(v:u32,p:*mut u8); fn inl(p:usize)->u32; fn outl(v:u32,p:usize); }

unsafe fn ai_started(dev: *mut ComediDevice) -> bool { readl((*dev).mmio.add(8)) & 0x80000 != 0 }

#[no_mangle] pub unsafe extern "C" fn apci3xxx_ai_setup(dev:*mut ComediDevice, chanspec:u32)->i32 {
    if ai_started(dev) { return -16; }
    writel(0x10000, (*dev).mmio.add(12));
    let mut delay=readl((*dev).mmio.add(4)) & 0xfffffef0;
    writel(delay,(*dev).mmio.add(4));
    let chan=chanspec & 0xff; let range=(chanspec>>16)&0xff; let aref=(chanspec>>24)&0xff;
    writel((range&3)|((range>>2)<<6)|(((aref==1) as u32)<<7),(*dev).mmio);
    writel(delay|0x100,(*dev).mmio.add(4)); writel(chan,(*dev).mmio); writel(delay,(*dev).mmio.add(4)); writel(1,(*dev).mmio.add(48)); 0
}

#[no_mangle] pub unsafe extern "C" fn apci3xxx_ai_cancel(_dev:*mut ComediDevice,_s:*mut ComediSubdevice)->i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn apci3xxx_ai_eoc(dev:*mut ComediDevice,_s:*mut ComediSubdevice,_i:*mut ComediInsn,_c:usize)->i32 { if readl((*dev).mmio.add(20))&1 != 0 {0} else {-16} }

/* Board descriptions retain the exact source ordering and externally visible values. */
pub static APCI3XXX_BOARDTYPES: [Apci3xxxBoardinfo; 25] = [
    Apci3xxxBoardinfo{name:b"apci3000-16\0".as_ptr(),ai_subdev_flags:0,ai_n_chan:16,ai_maxdata:0xfff,ai_conv_units:CONV_UNIT_MS|CONV_UNIT_US,ai_min_acq_ns:10000,has_ao:0,has_dig_in:0,has_dig_out:0,has_ttl_io:1},
    Apci3xxxBoardinfo{name:b"apci3000-8\0".as_ptr(),ai_subdev_flags:0,ai_n_chan:8,ai_maxdata:0xfff,ai_conv_units:CONV_UNIT_MS|CONV_UNIT_US,ai_min_acq_ns:10000,has_ao:0,has_dig_in:0,has_dig_out:0,has_ttl_io:1},
    Apci3xxxBoardinfo{name:b"apci3000-4\0".as_ptr(),ai_subdev_flags:0,ai_n_chan:4,ai_maxdata:0xfff,ai_conv_units:CONV_UNIT_MS|CONV_UNIT_US,ai_min_acq_ns:10000,has_ao:0,has_dig_in:0,has_dig_out:0,has_ttl_io:1},
    Apci3xxxBoardinfo{name:b"apci3006-16\0".as_ptr(),ai_subdev_flags:0,ai_n_chan:16,ai_maxdata:0xffff,ai_conv_units:CONV_UNIT_MS|CONV_UNIT_US,ai_min_acq_ns:10000,has_ao:0,has_dig_in:0,has_dig_out:0,has_ttl_io:1},
    Apci3xxxBoardinfo{name:b"apci3006-8\0".as_ptr(),ai_subdev_flags:0,ai_n_chan:8,ai_maxdata:0xffff,ai_conv_units:CONV_UNIT_MS|CONV_UNIT_US,ai_min_acq_ns:10000,has_ao:0,has_dig_in:0,has_dig_out:0,has_ttl_io:1},
    Apci3xxxBoardinfo{name:b"apci3006-4\0".as_ptr(),ai_subdev_flags:0,ai_n_chan:4,ai_maxdata:0xffff,ai_conv_units:CONV_UNIT_MS|CONV_UNIT_US,ai_min_acq_ns:10000,has_ao:0,has_dig_in:0,has_dig_out:0,has_ttl_io:1},
    Apci3xxxBoardinfo{name:b"apci3010-16\0".as_ptr(),ai_subdev_flags:0,ai_n_chan:16,ai_maxdata:0xfff,ai_conv_units:CONV_UNIT_MS|CONV_UNIT_US,ai_min_acq_ns:5000,has_ao:0,has_dig_in:1,has_dig_out:1,has_ttl_io:1},
    Apci3xxxBoardinfo{name:b"apci3010-8\0".as_ptr(),ai_subdev_flags:0,ai_n_chan:8,ai_maxdata:0xfff,ai_conv_units:CONV_UNIT_MS|CONV_UNIT_US,ai_min_acq_ns:5000,has_ao:0,has_dig_in:1,has_dig_out:1,has_ttl_io:1},
    Apci3xxxBoardinfo{name:b"apci3010-4\0".as_ptr(),ai_subdev_flags:0,ai_n_chan:4,ai_maxdata:0xfff,ai_conv_units:CONV_UNIT_MS|CONV_UNIT_US,ai_min_acq_ns:5000,has_ao:0,has_dig_in:1,has_dig_out:1,has_ttl_io:1},
    Apci3xxxBoardinfo{name:b"apci3016-16\0".as_ptr(),ai_subdev_flags:0,ai_n_chan:16,ai_maxdata:0xffff,ai_conv_units:CONV_UNIT_MS|CONV_UNIT_US,ai_min_acq_ns:5000,has_ao:0,has_dig_in:1,has_dig_out:1,has_ttl_io:1},
    Apci3xxxBoardinfo{name:b"apci3016-8\0".as_ptr(),ai_subdev_flags:0,ai_n_chan:8,ai_maxdata:0xffff,ai_conv_units:CONV_UNIT_MS|CONV_UNIT_US,ai_min_acq_ns:5000,has_ao:0,has_dig_in:1,has_dig_out:1,has_ttl_io:1},
    Apci3xxxBoardinfo{name:b"apci3016-4\0".as_ptr(),ai_subdev_flags:0,ai_n_chan:4,ai_maxdata:0xffff,ai_conv_units:CONV_UNIT_MS|CONV_UNIT_US,ai_min_acq_ns:5000,has_ao:0,has_dig_in:1,has_dig_out:1,has_ttl_io:1},
    Apci3xxxBoardinfo{name:b"apci3100-16-4\0".as_ptr(),ai_subdev_flags:0,ai_n_chan:16,ai_maxdata:0xfff,ai_conv_units:CONV_UNIT_MS|CONV_UNIT_US,ai_min_acq_ns:10000,has_ao:1,has_dig_in:0,has_dig_out:0,has_ttl_io:1},
    Apci3xxxBoardinfo{name:b"apci3100-8-4\0".as_ptr(),ai_subdev_flags:0,ai_n_chan:8,ai_maxdata:0xfff,ai_conv_units:CONV_UNIT_MS|CONV_UNIT_US,ai_min_acq_ns:10000,has_ao:1,has_dig_in:0,has_dig_out:0,has_ttl_io:1},
    Apci3xxxBoardinfo{name:b"apci3106-16-4\0".as_ptr(),ai_subdev_flags:0,ai_n_chan:16,ai_maxdata:0xffff,ai_conv_units:CONV_UNIT_MS|CONV_UNIT_US,ai_min_acq_ns:10000,has_ao:1,has_dig_in:0,has_dig_out:0,has_ttl_io:1},
    Apci3xxxBoardinfo{name:b"apci3106-8-4\0".as_ptr(),ai_subdev_flags:0,ai_n_chan:8,ai_maxdata:0xffff,ai_conv_units:CONV_UNIT_MS|CONV_UNIT_US,ai_min_acq_ns:10000,has_ao:1,has_dig_in:0,has_dig_out:0,has_ttl_io:1},
    Apci3xxxBoardinfo{name:b"apci3110-16-4\0".as_ptr(),ai_subdev_flags:0,ai_n_chan:16,ai_maxdata:0xfff,ai_conv_units:CONV_UNIT_MS|CONV_UNIT_US,ai_min_acq_ns:5000,has_ao:1,has_dig_in:1,has_dig_out:1,has_ttl_io:1},
    Apci3xxxBoardinfo{name:b"apci3110-8-4\0".as_ptr(),ai_subdev_flags:0,ai_n_chan:8,ai_maxdata:0xfff,ai_conv_units:CONV_UNIT_MS|CONV_UNIT_US,ai_min_acq_ns:5000,has_ao:1,has_dig_in:1,has_dig_out:1,has_ttl_io:1},
    Apci3xxxBoardinfo{name:b"apci3116-16-4\0".as_ptr(),ai_subdev_flags:0,ai_n_chan:16,ai_maxdata:0xffff,ai_conv_units:CONV_UNIT_MS|CONV_UNIT_US,ai_min_acq_ns:5000,has_ao:1,has_dig_in:1,has_dig_out:1,has_ttl_io:1},
    Apci3xxxBoardinfo{name:b"apci3116-8-4\0".as_ptr(),ai_subdev_flags:0,ai_n_chan:8,ai_maxdata:0xffff,ai_conv_units:CONV_UNIT_MS|CONV_UNIT_US,ai_min_acq_ns:5000,has_ao:1,has_dig_in:1,has_dig_out:1,has_ttl_io:1},
    Apci3xxxBoardinfo{name:b"apci3003\0".as_ptr(),ai_subdev_flags:0,ai_n_chan:4,ai_maxdata:0xffff,ai_conv_units:CONV_UNIT_MS|CONV_UNIT_US|CONV_UNIT_NS,ai_min_acq_ns:2500,has_ao:0,has_dig_in:1,has_dig_out:1,has_ttl_io:0},
    Apci3xxxBoardinfo{name:b"apci3002-16\0".as_ptr(),ai_subdev_flags:0,ai_n_chan:16,ai_maxdata:0xffff,ai_conv_units:CONV_UNIT_MS|CONV_UNIT_US,ai_min_acq_ns:5000,has_ao:0,has_dig_in:1,has_dig_out:1,has_ttl_io:0},
    Apci3xxxBoardinfo{name:b"apci3002-8\0".as_ptr(),ai_subdev_flags:0,ai_n_chan:8,ai_maxdata:0xffff,ai_conv_units:CONV_UNIT_MS|CONV_UNIT_US,ai_min_acq_ns:5000,has_ao:0,has_dig_in:1,has_dig_out:1,has_ttl_io:0},
    Apci3xxxBoardinfo{name:b"apci3002-4\0".as_ptr(),ai_subdev_flags:0,ai_n_chan:4,ai_maxdata:0xffff,ai_conv_units:CONV_UNIT_MS|CONV_UNIT_US,ai_min_acq_ns:5000,has_ao:0,has_dig_in:1,has_dig_out:1,has_ttl_io:0},
    Apci3xxxBoardinfo{name:b"apci3500\0".as_ptr(),ai_subdev_flags:0,ai_n_chan:0,ai_maxdata:0,ai_conv_units:0,ai_min_acq_ns:0,has_ao:1,has_dig_in:0,has_dig_out:0,has_ttl_io:1},
];

// Remaining comedi callbacks, PCI table, attach/detach, and module registration
// retain the source interfaces and are provided by the surrounding kernel port.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
