// SPDX-License-Identifier: GPL-2.0
/*
 * comedi/drivers/dt2801.c
 * Device Driver for DataTranslation DT2801
 */

// Linux/Comedi dependencies supplied by the surrounding translation unit.
use core::ffi::c_char;

extern "C" {
    fn inb_p(port: u16) -> u8;
    fn outb_p(value: u8, port: u16);
    fn usleep_range(min: u32, max: u32);
}

const DT2801_TIMEOUT: i32 = 1000;
const DT2801_MAX_DMA_SIZE: usize = 64 * 1024;

const DT_C_RESET: i32 = 0x0;
const DT_C_CLEAR_ERR: i32 = 0x1;
const DT_C_READ_ERRREG: i32 = 0x2;
const DT_C_SET_CLOCK: i32 = 0x3;
const DT_C_TEST: i32 = 0xb;
const DT_C_STOP: i32 = 0xf;
const DT_C_SET_DIGIN: i32 = 0x4;
const DT_C_SET_DIGOUT: i32 = 0x5;
const DT_C_READ_DIG: i32 = 0x6;
const DT_C_WRITE_DIG: i32 = 0x7;
const DT_C_WRITE_DAIM: i32 = 0x8;
const DT_C_SET_DA: i32 = 0x9;
const DT_C_WRITE_DA: i32 = 0xa;
const DT_C_READ_ADIM: i32 = 0xc;
const DT_C_SET_AD: i32 = 0xd;
const DT_C_READ_AD: i32 = 0xe;

const DT_MOD_DMA: i32 = 1 << 4;
const DT_MOD_CONT: i32 = 1 << 5;
const DT_MOD_EXTCLK: i32 = 1 << 6;
const DT_MOD_EXTTRIG: i32 = 1 << 7;
const DT_S_DATA_OUT_READY: i32 = 1 << 0;
const DT_S_DATA_IN_FULL: i32 = 1 << 1;
const DT_S_READY: i32 = 1 << 2;
const DT_S_COMMAND: i32 = 1 << 3;
const DT_S_COMPOSITE_ERROR: i32 = 1 << 7;
const DT2801_DATA: u16 = 0;
const DT2801_STATUS: u16 = 1;
const DT2801_CMD: u16 = 1;
const ETIME: i32 = 62;
const EIO: i32 = 5;
const ENOMEM: i32 = 12;

#[repr(C)]
pub struct ComediLrange { pub length: u32, pub range: [u8; 1] }
#[repr(C)]
pub struct ComediDevice {
    pub iobase: u16,
    pub class_dev: *mut core::ffi::c_void,
    pub subdevices: *mut ComediSubdevice,
    pub board_ptr: *const core::ffi::c_void,
    pub board_name: *const c_char,
}
#[repr(C)]
pub struct ComediSubdevice {
    pub type_: u32, pub subdev_flags: u32, pub n_chan: u32, pub maxdata: u32,
    pub range_table: *const ComediLrange, pub range_table_list: *mut *const ComediLrange,
    pub state: u32, pub io_bits: u32, pub readback: *mut u32,
    pub insn_read: Option<unsafe extern "C" fn(*mut ComediDevice, *mut ComediSubdevice, *mut ComediInsn, *mut u32) -> i32>,
    pub insn_write: Option<unsafe extern "C" fn(*mut ComediDevice, *mut ComediSubdevice, *mut ComediInsn, *mut u32) -> i32>,
    pub insn_bits: Option<unsafe extern "C" fn(*mut ComediDevice, *mut ComediSubdevice, *mut ComediInsn, *mut u32) -> i32>,
    pub insn_config: Option<unsafe extern "C" fn(*mut ComediDevice, *mut ComediSubdevice, *mut ComediInsn, *mut u32) -> i32>,
}
#[repr(C)] pub struct ComediInsn { pub n: u32, pub chanspec: u32 }
#[repr(C)] pub struct ComediDevconfig { pub options: *mut i32 }
#[repr(C)] pub struct Dt2801Board { pub name: *const c_char, pub boardcode: i32, pub ad_diff: i32, pub ad_chan: i32, pub adbits: i32, pub adrangetype: i32, pub dabits: i32 }
#[repr(C)] pub struct Dt2801Private { pub dac_range_types: [*const ComediLrange; 2] }

extern "C" {
    static range_bipolar10: ComediLrange; static range_bipolar5: ComediLrange;
    static range_bipolar2_5: ComediLrange; static range_unipolar10: ComediLrange;
    static range_unipolar5: ComediLrange; static range_unknown: ComediLrange;
    static range_digital: ComediLrange;
    fn comedi_dio_update_state(s: *mut ComediSubdevice, data: *mut u32) -> i32;
    fn comedi_dio_insn_config(dev: *mut ComediDevice, s: *mut ComediSubdevice, insn: *mut ComediInsn, data: *mut u32, mask: u32) -> i32;
    fn comedi_check_request_region(dev: *mut ComediDevice, base: i32, len: i32, min: i32, max: i32, io_size: i32) -> i32;
    fn comedi_alloc_subdevices(dev: *mut ComediDevice, n: i32) -> i32;
    fn comedi_alloc_devpriv(dev: *mut ComediDevice, size: usize) -> *mut core::ffi::c_void;
    fn comedi_alloc_subdev_readback(s: *mut ComediSubdevice) -> i32;
    fn dev_dbg(dev: *mut core::ffi::c_void, fmt: *const c_char, ...);
}

static RANGE_DT2801_AI_PGL_BIPOLAR: ComediLrange = ComediLrange { length: 4, range: [0; 1] };
static RANGE_DT2801_AI_PGL_UNIPOLAR: ComediLrange = ComediLrange { length: 4, range: [0; 1] };

static BOARDTYPES: [Dt2801Board; 8] = [
    Dt2801Board { name: b"dt2801\0".as_ptr() as *const c_char, boardcode: 0x09, ad_diff: 2, ad_chan: 16, adbits: 12, adrangetype: 0, dabits: 12 },
    Dt2801Board { name: b"dt2801-a\0".as_ptr() as *const c_char, boardcode: 0x52, ad_diff: 2, ad_chan: 16, adbits: 12, adrangetype: 0, dabits: 12 },
    Dt2801Board { name: b"dt2801/5716a\0".as_ptr() as *const c_char, boardcode: 0x82, ad_diff: 1, ad_chan: 16, adbits: 16, adrangetype: 1, dabits: 12 },
    Dt2801Board { name: b"dt2805\0".as_ptr() as *const c_char, boardcode: 0x12, ad_diff: 1, ad_chan: 16, adbits: 12, adrangetype: 0, dabits: 12 },
    Dt2801Board { name: b"dt2805/5716a\0".as_ptr() as *const c_char, boardcode: 0x92, ad_diff: 1, ad_chan: 16, adbits: 16, adrangetype: 1, dabits: 12 },
    Dt2801Board { name: b"dt2808\0".as_ptr() as *const c_char, boardcode: 0x20, ad_diff: 0, ad_chan: 16, adbits: 12, adrangetype: 2, dabits: 8 },
    Dt2801Board { name: b"dt2818\0".as_ptr() as *const c_char, boardcode: 0xa2, ad_diff: 0, ad_chan: 4, adbits: 12, adrangetype: 0, dabits: 12 },
    Dt2801Board { name: b"dt2809\0".as_ptr() as *const c_char, boardcode: 0xb0, ad_diff: 0, ad_chan: 8, adbits: 12, adrangetype: 1, dabits: 12 },
];

unsafe fn dt2801_readdata(dev: *mut ComediDevice, data: *mut i32) -> i32 {
    let mut timeout = DT2801_TIMEOUT;
    loop { let stat = inb_p((*dev).iobase + DT2801_STATUS) as i32;
        if stat & (DT_S_COMPOSITE_ERROR | DT_S_READY) != 0 { return stat; }
        if stat & DT_S_DATA_OUT_READY != 0 { *data = inb_p((*dev).iobase + DT2801_DATA) as i32; return 0; }
        timeout -= 1; if timeout <= 0 { break; }
    } -ETIME
}
unsafe fn dt2801_readdata2(dev: *mut ComediDevice, data: *mut i32) -> i32 { let mut lb=0; let mut hb=0; let r=dt2801_readdata(dev,&mut lb); if r!=0{return r;} let r=dt2801_readdata(dev,&mut hb); if r!=0{return r;} *data=(hb<<8)+lb; 0 }
unsafe fn dt2801_writedata(dev: *mut ComediDevice, data: u32) -> i32 { let mut timeout=DT2801_TIMEOUT; loop { let stat=inb_p((*dev).iobase+DT2801_STATUS) as i32; if stat&DT_S_COMPOSITE_ERROR!=0{return stat;} if stat&DT_S_DATA_IN_FULL==0 {outb_p((data&0xff) as u8,(*dev).iobase+DT2801_DATA);return 0;} timeout-=1;if timeout<=0{break;} } -ETIME }
unsafe fn dt2801_writedata2(dev: *mut ComediDevice, data: u32) -> i32 { let r=dt2801_writedata(dev,data&0xff);if r<0{return r;}let r=dt2801_writedata(dev,data>>8);if r<0{return r;}0 }
unsafe fn dt2801_wait_for_ready(dev: *mut ComediDevice) -> i32 { let mut timeout=DT2801_TIMEOUT; let mut stat=inb_p((*dev).iobase+DT2801_STATUS) as i32;if stat&DT_S_READY!=0{return 0;}loop{stat=inb_p((*dev).iobase+DT2801_STATUS) as i32;if stat&DT_S_COMPOSITE_ERROR!=0{return stat;}if stat&DT_S_READY!=0{return 0;}timeout-=1;if timeout<=0{break;}}-ETIME }
unsafe fn dt2801_writecmd(dev: *mut ComediDevice, command: i32) { let _=dt2801_wait_for_ready(dev); let stat=inb_p((*dev).iobase+DT2801_STATUS) as i32; if stat&DT_S_READY==0 {} outb_p(command as u8,(*dev).iobase+DT2801_CMD); }
unsafe fn dt2801_reset(dev: *mut ComediDevice) -> i32 { for _ in 0..4{inb_p((*dev).iobase+DT2801_DATA);}outb_p(DT_C_STOP as u8,(*dev).iobase+DT2801_CMD);usleep_range(100,200);let mut stat=0;let mut timeout=10000;loop{stat=inb_p((*dev).iobase+DT2801_STATUS) as i32;if stat&DT_S_READY!=0{break;}timeout-=1;if timeout<0{break;}}outb_p(DT_C_RESET as u8,(*dev).iobase+DT2801_CMD);usleep_range(100,200);timeout=10000;loop{stat=inb_p((*dev).iobase+DT2801_STATUS) as i32;if stat&DT_S_READY!=0{break;}timeout-=1;if timeout<0{break;}}let mut board_code=0;let _=dt2801_readdata(dev,&mut board_code);board_code }

unsafe fn probe_number_of_ai_chans(dev: *mut ComediDevice) -> i32 { let mut n=0; while n<16 {dt2801_writecmd(dev,DT_C_READ_ADIM);dt2801_writedata(dev,0);dt2801_writedata(dev,n as u32);let mut data=0;if dt2801_readdata2(dev,&mut data)!=0{break;}n+=1;}dt2801_reset(dev);dt2801_reset(dev);n }
unsafe fn dac_range_lkup(opt: i32) -> *const ComediLrange { match opt {0=>&range_bipolar10,1=>&range_bipolar5,2=>&range_bipolar2_5,3=>&range_unipolar10,4=>&range_unipolar5,_=>&range_unknown} }
unsafe fn ai_range_lkup(typ:i32,opt:i32)->*const ComediLrange {match typ{0=>if opt!=0{&RANGE_DT2801_AI_PGL_UNIPOLAR}else{&RANGE_DT2801_AI_PGL_BIPOLAR},1=>if opt!=0{&range_unipolar10}else{&range_bipolar10},2=>&range_unipolar5,_=>&range_unknown}}
unsafe fn dt2801_error(dev:*mut ComediDevice,stat:i32)->i32{if stat<0{return stat;}dt2801_reset(dev);dt2801_reset(dev);-EIO}
unsafe extern "C" fn dt2801_ai_insn_read(dev:*mut ComediDevice,_s:*mut ComediSubdevice,insn:*mut ComediInsn,data:*mut u32)->i32{for i in 0..(*insn).n{dt2801_writecmd(dev,DT_C_READ_ADIM);dt2801_writedata(dev,((*insn).chanspec>>24)&0xff);dt2801_writedata(dev,(*insn).chanspec&0xff);let mut d=0;let r=dt2801_readdata2(dev,&mut d);if r!=0{return dt2801_error(dev,r);}*data.add(i as usize)=d as u32;}(*insn).n as i32}
unsafe extern "C" fn dt2801_ao_insn_write(dev:*mut ComediDevice,s:*mut ComediSubdevice,insn:*mut ComediInsn,data:*mut u32)->i32{let chan=(*insn).chanspec&0xff;dt2801_writecmd(dev,DT_C_WRITE_DAIM);dt2801_writedata(dev,chan);dt2801_writedata2(dev,*data);if !(*s).readback.is_null(){*(*s).readback.add(chan as usize)=*data;}1}
unsafe extern "C" fn dt2801_dio_insn_bits(dev:*mut ComediDevice,s:*mut ComediSubdevice,insn:*mut ComediInsn,data:*mut u32)->i32{let which=if s==(*dev).subdevices.add(3){1}else{0};if comedi_dio_update_state(s,data)!=0{dt2801_writecmd(dev,DT_C_WRITE_DIG);dt2801_writedata(dev,which);dt2801_writedata(dev,(*s).state);}dt2801_writecmd(dev,DT_C_READ_DIG);dt2801_writedata(dev,which);let mut val=0;dt2801_readdata(dev,&mut val);*data.add(1)=val as u32;(*insn).n as i32}
unsafe extern "C" fn dt2801_dio_insn_config(dev:*mut ComediDevice,s:*mut ComediSubdevice,insn:*mut ComediInsn,data:*mut u32)->i32{let r=comedi_dio_insn_config(dev,s,insn,data,0xff);if r!=0{return r;}dt2801_writecmd(dev,if (*s).io_bits!=0{DT_C_SET_DIGOUT}else{DT_C_SET_DIGIN});dt2801_writedata(dev,if s==(*dev).subdevices.add(3){1}else{0});(*insn).n as i32}

#[no_mangle]
pub unsafe extern "C" fn dt2801_attach(dev:*mut ComediDevice,it:*mut ComediDevconfig)->i32{let r=comedi_check_request_region(dev,*(*it).options,2,0x200,0x3ff,2);if r!=0{return r;}let code=dt2801_reset(dev);let mut typ=0;for i in 0..BOARDTYPES.len(){if BOARDTYPES[i].boardcode==code{typ=i;break;}}(*dev).board_ptr=&BOARDTYPES[typ] as *const _ as *const _;let _=probe_number_of_ai_chans(dev);if comedi_alloc_subdevices(dev,4)!=0{return -ENOMEM;}let p=comedi_alloc_devpriv(dev,core::mem::size_of::<Dt2801Private>()) as *mut Dt2801Private;if p.is_null(){return -ENOMEM;}(*dev).board_name=BOARDTYPES[typ].name;0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
