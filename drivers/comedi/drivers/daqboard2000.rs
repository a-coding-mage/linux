// SPDX-License-Identifier: GPL-2.0+
/* Rust translation of comedi/drivers/daqboard2000.c. */

// Linux/Comedi dependencies supplied by the surrounding repository.
use core::ffi::{c_char, c_int, c_ulong, c_void};

extern "C" {
    fn writew(value: u16, addr: *mut c_void);
    fn readw(addr: *mut c_void) -> u16;
    fn writel(value: u32, addr: *mut c_void);
    fn readl(addr: *mut c_void) -> u32;
    fn mdelay(ms: u32);
    fn udelay(us: u32);
    fn usleep_range(min: u32, max: u32);
}

#[repr(C)]
pub struct comedi_lrange { pub length: u32, pub range: [c_void; 13] }
extern "C" {
    static range_bipolar10: comedi_lrange;
    fn BIP_RANGE(v: f64) -> c_void;
    fn UNI_RANGE(v: f64) -> c_void;
}

const DB2K_FIRMWARE: *const c_char = b"daqboard2000_firmware.bin\0".as_ptr() as *const c_char;

macro_rules! db2k_range { ($($x:expr),*) => { [$($x),*] }; }
static db2k_ai_range: comedi_lrange = comedi_lrange { length: 13, range: db2k_range!(
    unsafe { BIP_RANGE(10.0) }, unsafe { BIP_RANGE(5.0) }, unsafe { BIP_RANGE(2.5) },
    unsafe { BIP_RANGE(1.25) }, unsafe { BIP_RANGE(0.625) }, unsafe { BIP_RANGE(0.3125) },
    unsafe { BIP_RANGE(0.156) }, unsafe { UNI_RANGE(10.0) }, unsafe { UNI_RANGE(5.0) },
    unsafe { UNI_RANGE(2.5) }, unsafe { UNI_RANGE(1.25) }, unsafe { UNI_RANGE(0.625) },
    unsafe { UNI_RANGE(0.3125) }
) };

macro_rules! consts { ($($n:ident = $v:expr),* $(,)?) => { $(pub const $n: u32 = $v;)* }; }
consts!
    DB2K_REG_ACQ_CONTROL = 0x00, DB2K_REG_ACQ_STATUS = 0x00,
    DB2K_REG_ACQ_SCAN_LIST_FIFO = 0x02, DB2K_REG_ACQ_PACER_CLOCK_DIV_LOW = 0x04,
    DB2K_REG_ACQ_SCAN_COUNTER = 0x08, DB2K_REG_ACQ_PACER_CLOCK_DIV_HIGH = 0x0a,
    DB2K_REG_ACQ_TRIGGER_COUNT = 0x0c, DB2K_REG_ACQ_RESULTS_FIFO = 0x10,
    DB2K_REG_ACQ_RESULTS_SHADOW = 0x14, DB2K_REG_ACQ_ADC_RESULT = 0x18,
    DB2K_REG_DAC_SCAN_COUNTER = 0x1c, DB2K_REG_DAC_CONTROL = 0x20,
    DB2K_REG_DAC_STATUS = 0x20, DB2K_REG_DAC_FIFO = 0x24,
    DB2K_REG_DAC_PACER_CLOCK_DIV = 0x2a, DB2K_REG_REF_DACS = 0x2c,
    DB2K_REG_DIO_CONTROL = 0x30, DB2K_REG_P3_HSIO_DATA = 0x32,
    DB2K_REG_P3_CONTROL = 0x34, DB2K_REG_CAL_EEPROM_CONTROL = 0x36,
    DB2K_REG_DIO_P2_EXP_IO_8_BIT = 0x40, DB2K_REG_COUNTER_TIMER_CONTROL = 0x80,
    DB2K_REG_DMA_CONTROL = 0xb0, DB2K_REG_TRIG_CONTROL = 0xb2,
    DB2K_REG_CAL_EEPROM = 0xb8, DB2K_REG_ACQ_DIGITAL_MARK = 0xba,
    DB2K_REG_TRIG_DACS = 0xbc, DB2K_REG_CPLD_STATUS = 0x1000,
    DB2K_REG_CPLD_WDATA = 0x1000, DB2K_ACQ_CONTROL_SEQ_START_SCAN_LIST = 0x0011,
    DB2K_ACQ_CONTROL_SEQ_STOP_SCAN_LIST = 0x0010,
    DB2K_ACQ_CONTROL_RESET_SCAN_LIST_FIFO = 0x0004,
    DB2K_ACQ_CONTROL_RESET_RESULTS_FIFO = 0x0002,
    DB2K_ACQ_CONTROL_RESET_CONFIG_PIPE = 0x0001,
    DB2K_ACQ_CONTROL_ADC_PACER_INTERNAL = 0x0030,
    DB2K_ACQ_CONTROL_ADC_PACER_EXTERNAL = 0x0032,
    DB2K_ACQ_CONTROL_ADC_PACER_ENABLE = 0x0031,
    DB2K_ACQ_CONTROL_ADC_PACER_ENABLE_DAC_PACER = 0x0034,
    DB2K_ACQ_CONTROL_ADC_PACER_DISABLE = 0x0030,
    DB2K_ACQ_CONTROL_ADC_PACER_NORMAL_MODE = 0x0060,
    DB2K_ACQ_CONTROL_ADC_PACER_COMPATIBILITY_MODE = 0x0061,
    DB2K_ACQ_CONTROL_ADC_PACER_INTERNAL_OUT_ENABLE = 0x0008,
    DB2K_ACQ_CONTROL_ADC_PACER_EXTERNAL_RISING = 0x0100,
    DB2K_ACQ_STATUS_RESULTS_FIFO_MORE_1_SAMPLE = 0x0001,
    DB2K_ACQ_STATUS_RESULTS_FIFO_HAS_DATA = 0x0002,
    DB2K_ACQ_STATUS_RESULTS_FIFO_OVERRUN = 0x0004,
    DB2K_ACQ_STATUS_LOGIC_SCANNING = 0x0008,
    DB2K_ACQ_STATUS_CONFIG_PIPE_FULL = 0x0010,
    DB2K_ACQ_STATUS_SCAN_LIST_FIFO_EMPTY = 0x0020,
    DB2K_ACQ_STATUS_ADC_NOT_READY = 0x0040,
    DB2K_ACQ_STATUS_ARBITRATION_FAILURE = 0x0080,
    DB2K_ACQ_STATUS_ADC_PACER_OVERRUN = 0x0100,
    DB2K_ACQ_STATUS_DAC_PACER_OVERRUN = 0x0200,
    DB2K_DAC_STATUS_DAC_FULL = 0x0001, DB2K_DAC_STATUS_REF_BUSY = 0x0002,
    DB2K_DAC_STATUS_TRIG_BUSY = 0x0004, DB2K_DAC_STATUS_CAL_BUSY = 0x0008,
    DB2K_DAC_CONTROL_ENABLE_BIT = 0x0001, DB2K_DAC_CONTROL_DATA_IS_SIGNED = 0x0002,
    DB2K_DAC_CONTROL_RESET_FIFO = 0x0004, DB2K_DAC_CONTROL_PATTERN_DISABLE = 0x0060,
    DB2K_DAC_CONTROL_PATTERN_ENABLE = 0x0061, DB2K_TRIG_CONTROL_TYPE_ANALOG = 0x0000,
    DB2K_TRIG_CONTROL_TYPE_TTL = 0x0010, DB2K_TRIG_CONTROL_EDGE_HI_LO = 0x0004,
    DB2K_TRIG_CONTROL_EDGE_LO_HI = 0x0000, DB2K_TRIG_CONTROL_LEVEL_ABOVE = 0x0000,
    DB2K_TRIG_CONTROL_LEVEL_BELOW = 0x0004, DB2K_TRIG_CONTROL_SENSE_LEVEL = 0x0002,
    DB2K_TRIG_CONTROL_SENSE_EDGE = 0x0000, DB2K_TRIG_CONTROL_ENABLE = 0x0001,
    DB2K_TRIG_CONTROL_DISABLE = 0x0000, DB2K_REF_DACS_SET = 0x0080,
    DB2K_REF_DACS_SELECT_POS_REF = 0x0100, DB2K_REF_DACS_SELECT_NEG_REF = 0x0000,
    DB2K_CPLD_STATUS_INIT = 0x0002, DB2K_CPLD_STATUS_TXREADY = 0x0004,
    DB2K_CPLD_VERSION_MASK = 0xf000, DB2K_CPLD_VERSION_NEW = 0x5000;

#[repr(C)] pub struct comedi_device { pub mmio: *mut u8, pub private: *mut db2k_private, pub subdevices: *mut comedi_subdevice, pub board_ptr: *const db2k_boardtype, pub board_name: *const c_char, pub class_dev: *mut c_void }
#[repr(C)] pub struct comedi_subdevice { pub readback: *mut u32, pub n_chan: u32, pub maxdata: u32 }
#[repr(C)] pub struct comedi_insn { pub chanspec: u32, pub n: u32 }
#[repr(C)] pub struct pci_dev { pub dev: c_void }
#[repr(C)] pub struct pci_device_id { pub driver_data: c_ulong }
#[repr(C)] pub struct db2k_private { pub plx: *mut u8 }
#[repr(C)] pub struct db2k_boardtype { pub name: *const c_char, pub has_2_ao: bool }

extern "C" {
    fn comedi_timeout(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn, f:unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,c_ulong)->c_int, context:c_ulong)->c_int;
    fn CR_RANGE(x:u32)->c_int; fn CR_CHAN(x:u32)->c_int;
    fn comedi_to_pci_dev(dev:*mut comedi_device)->*mut pci_dev;
    fn comedi_alloc_devpriv(dev:*mut comedi_device,size:usize)->*mut db2k_private;
    fn comedi_pci_enable(dev:*mut comedi_device)->c_int; fn pci_ioremap_bar(dev:*mut pci_dev,bar:c_int)->*mut u8;
    fn comedi_alloc_subdevices(dev:*mut comedi_device,n:c_int)->c_int;
    fn comedi_load_firmware(dev:*mut comedi_device,pdev:*mut c_void,name:*const c_char, f:unsafe extern "C" fn(*mut comedi_device,*const u8,usize,c_ulong)->c_int, context:c_ulong)->c_int;
    fn comedi_alloc_subdev_readback(s:*mut comedi_subdevice)->c_int; fn subdev_8255_cb_init(dev:*mut comedi_device,s:*mut comedi_subdevice,cb:unsafe extern "C" fn(*mut comedi_device,c_int,c_int,c_int,c_ulong)->c_int, iobase:c_ulong)->c_int;
    fn iounmap(addr:*mut u8); fn comedi_pci_detach(dev:*mut comedi_device); fn comedi_pci_auto_config(dev:*mut pci_dev,driver:*mut c_void, data:c_ulong)->c_int; fn comedi_pci_auto_unconfig(dev:*mut pci_dev);
}

const PLX_REG_CNTRL:u32=0x50; const PLX_CNTRL_RESET:u32=1<<30; const PLX_CNTRL_EERELOAD:u32=1<<29; const PLX_CNTRL_USERO:u32=1<<24; const PLX_CNTRL_USERI:u32=1<<25; const PLX_CNTRL_EEPRESENT:u32=1<<28;

#[inline] unsafe fn reg(dev:*mut comedi_device, off:u32)->*mut c_void { (*dev).mmio.add(off as usize) as *mut c_void }
unsafe extern "C" fn db2k_write_acq_scan_list_entry(dev:*mut comedi_device,entry:u16){writew(entry&0xff,reg(dev,2));writew((entry>>8)&0xff,reg(dev,2));}
unsafe extern "C" fn db2k_setup_sampling(dev:*mut comedi_device,chan:c_int,_gain:c_int){let mut w3:u16=match chan/4{0=>1,1=>2,2=>5,3=>6,4=>0x41,5=>0x42,_=>0};let w0=0;let w1=4;let w2=((chan<<6)&0xc0) as u16|0x0800;w3|=0xc000;db2k_write_acq_scan_list_entry(dev,w0);db2k_write_acq_scan_list_entry(dev,w1);db2k_write_acq_scan_list_entry(dev,w2);db2k_write_acq_scan_list_entry(dev,w3);}
unsafe extern "C" fn db2k_ai_status(dev:*mut comedi_device,_s:*mut comedi_subdevice,_i:*mut comedi_insn,context:c_ulong)->c_int{if (readw(reg(dev,0)) as c_ulong&context)!=0{0}else{-16}}
unsafe extern "C" fn db2k_ai_insn_read(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->c_int{writew(7,reg(dev,0));writel(1_000_000,reg(dev,4));writew(0,reg(dev,0xa));let chan=CR_CHAN((*insn).chanspec);let gain=CR_RANGE((*insn).chanspec);for i in 0..(*insn).n{db2k_setup_sampling(dev,chan,gain);writew(0x11,reg(dev,0));let r=comedi_timeout(dev,s,insn,db2k_ai_status,0x10);if r!=0{return r}writew(0x31,reg(dev,0));let r=comedi_timeout(dev,s,insn,db2k_ai_status,8);if r!=0{return r}let r=comedi_timeout(dev,s,insn,db2k_ai_status,2);if r!=0{return r}*data.add(i as usize)=readw(reg(dev,0x10)) as u32;writew(0x30,reg(dev,0));writew(0x10,reg(dev,0));}(*insn).n as c_int}
unsafe extern "C" fn db2k_ao_eoc(dev:*mut comedi_device,_s:*mut comedi_subdevice,insn:*mut comedi_insn,_context:c_ulong)->c_int{let chan=CR_CHAN((*insn).chanspec);if readw(reg(dev,0x20))&(0x10<<chan)==0{0}else{-16}}
unsafe extern "C" fn db2k_ao_insn_write(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->c_int{let chan=CR_CHAN((*insn).chanspec);for i in 0..(*insn).n{let v=*data.add(i as usize);writew(v as u16,reg(dev,0x38+(chan as u32)*2));let r=comedi_timeout(dev,s,insn,db2k_ao_eoc,0);if r!=0{return r}(*s).readback.add(chan as usize).write(v);}(*insn).n as c_int}

// The remaining driver lifecycle and firmware routines preserve the C call structure.
unsafe extern "C" fn db2k_reset_local_bus(dev:*mut comedi_device){let p=(*(*dev).private).plx;let mut c=readl(p.add(PLX_REG_CNTRL as usize) as *mut c_void)|PLX_CNTRL_RESET;writel(c,p.add(PLX_REG_CNTRL as usize) as *mut c_void);mdelay(10);c&=!PLX_CNTRL_RESET;writel(c,p.add(PLX_REG_CNTRL as usize) as *mut c_void);mdelay(10);}
unsafe extern "C" fn db2k_reload_plx(dev:*mut comedi_device){let p=(*(*dev).private).plx;let a=p.add(PLX_REG_CNTRL as usize) as *mut c_void;let mut c=readl(a)&!PLX_CNTRL_EERELOAD;writel(c,a);mdelay(10);c|=PLX_CNTRL_EERELOAD;writel(c,a);mdelay(10);c&=!PLX_CNTRL_EERELOAD;writel(c,a);mdelay(10);}
unsafe extern "C" fn db2k_pulse_prog_pin(dev:*mut comedi_device){let p=(*(*dev).private).plx;let a=p.add(PLX_REG_CNTRL as usize) as *mut c_void;let mut c=readl(a)|PLX_CNTRL_USERO;writel(c,a);mdelay(10);c&=!PLX_CNTRL_USERO;writel(c,a);mdelay(10);}
unsafe extern "C" fn db2k_wait_cpld_init(dev:*mut comedi_device)->c_int{for _ in 0..50{if readw(reg(dev,0x1000))&2!=0{return 0}usleep_range(100,1000)}udelay(5);-110}
unsafe extern "C" fn db2k_wait_cpld_txready(dev:*mut comedi_device)->c_int{for _ in 0..100{if readw(reg(dev,0x1000))&4!=0{return 0}udelay(1)}-110}
unsafe extern "C" fn db2k_write_cpld(dev:*mut comedi_device,data:u16,new_cpld:bool)->c_int{if new_cpld{let r=db2k_wait_cpld_txready(dev);if r!=0{return r}}else{usleep_range(10,20)}writew(data,reg(dev,0x1000));if readw(reg(dev,0x1000))&2==0{-5}else{0}}
unsafe extern "C" fn db2k_wait_fpga_programmed(dev:*mut comedi_device)->c_int{let p=(*(*dev).private).plx;for _ in 0..200{if readl(p.add(PLX_REG_CNTRL as usize) as *mut c_void)&PLX_CNTRL_USERI!=0{return 0}usleep_range(100,1000)}-110}
unsafe extern "C" fn db2k_load_firmware(dev:*mut comedi_device,arr:*const u8,len:usize,_context:c_ulong)->c_int{let mut i=0;while i+1<len&& !(*arr.add(i)==0xff&&*arr.add(i+1)==0x20){i+=1}if i+1>=len{return -22}if (len-i)&1!=0{return -22}let mut result=-5;for _ in 0..3{db2k_reset_local_bus(dev);db2k_reload_plx(dev);db2k_pulse_prog_pin(dev);if db2k_wait_cpld_init(dev)!=0{continue}let newc=readw(reg(dev,0x1000))&0xf000==0x5000;let mut j=i;while j<len{result=db2k_write_cpld(dev,((*arr.add(j) as u16)<<8)+*arr.add(j+1) as u16,newc);if result!=0{break}j+=2}if result==0{result=db2k_wait_fpga_programmed(dev)}if result==0{db2k_reset_local_bus(dev);db2k_reload_plx(dev);break}}result}
unsafe extern "C" fn db2k_adc_stop_dma_transfer(_:*mut comedi_device){}
unsafe extern "C" fn db2k_adc_disarm(dev:*mut comedi_device){udelay(2);writew(0,reg(dev,0xb2));udelay(2);writew(0x10,reg(dev,0xb2));udelay(2);writew(0x10,reg(dev,0));udelay(2);writew(0x30,reg(dev,0));db2k_adc_stop_dma_transfer(dev)}
unsafe extern "C" fn db2k_activate_reference_dacs(dev:*mut comedi_device){for v in [0x180u16,0x80u16]{writew(v,reg(dev,0x2c));for _ in 0..20{if readw(reg(dev,0x20))&2==0{break}udelay(2)}}}
unsafe extern "C" fn db2k_initialize_ctrs(_: *mut comedi_device){} unsafe extern "C" fn db2k_initialize_tmrs(_: *mut comedi_device){} unsafe extern "C" fn db2k_dac_disarm(_: *mut comedi_device){}
unsafe extern "C" fn db2k_initialize_adc(dev:*mut comedi_device){db2k_adc_disarm(dev);db2k_activate_reference_dacs(dev);db2k_initialize_ctrs(dev);db2k_initialize_tmrs(dev)}
unsafe extern "C" fn db2k_8255_cb(dev:*mut comedi_device,dir:c_int,port:c_int,data:c_int,iobase:c_ulong)->c_int{let a=reg(dev,iobase as u32+(port*2) as u32);if dir!=0{writew(data as u16,a);0}else{readw(a) as c_int}}

// Driver registration metadata is supplied by the kernel/Comedi integration layer.
#[repr(C)] pub struct db2k_driver { pub driver_name:*const c_char }
pub static mut db2k_driver_instance: db2k_driver = db2k_driver { driver_name: b"daqboard2000\0".as_ptr() as *const c_char };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
