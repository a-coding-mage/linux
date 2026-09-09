// SPDX-License-Identifier: GPL-2.0+
/* Direct Rust translation of comedi/drivers/cb_pcidda.c. */

const EEPROM_SIZE: usize = 128;
const MAX_AO_CHANNELS: usize = 8;
const CB_DDA_DIO0_8255_BASE: u32 = 0x00;
const CB_DDA_DIO1_8255_BASE: u32 = 0x04;
const CB_DDA_DA_CTRL_REG: u32 = 0x00;
const CB_DDA_DA_CTRL_SU: u32 = 1 << 0;
const CB_DDA_DA_CTRL_EN: u32 = 1 << 1;
const CB_DDA_DA_CTRL_RANGE2V5: u32 = 0 << 6;
const CB_DDA_DA_CTRL_RANGE5V: u32 = 2 << 6;
const CB_DDA_DA_CTRL_RANGE10V: u32 = 3 << 6;
const CB_DDA_DA_CTRL_UNIP: u32 = 1 << 8;
const DACALIBRATION1: usize = 4;
const SERIAL_IN_BIT: u32 = 0x1;
const CAL_CHANNEL_MASK: u32 = 0x7 << 1;
const CAL_COUNTER_MASK: u32 = 0x1f;
const CAL_COUNTER_OVERFLOW_BIT: u32 = 0x20;
const AO_BELOW_REF_BIT: u32 = 0x40;
const SERIAL_OUT_BIT: u32 = 0x80;
const DACALIBRATION2: usize = 6;
const SELECT_EEPROM_BIT: u32 = 0x1;
const DESELECT_REF_DAC_BIT: u32 = 0x2;
const DUMMY_BIT: u32 = 0x40;
const CB_DDA_CALDAC_FINE_GAIN: u32 = 0;
const CB_DDA_CALDAC_COURSE_GAIN: u32 = 1;
const CB_DDA_CALDAC_COURSE_OFFSET: u32 = 2;
const CB_DDA_CALDAC_FINE_OFFSET: u32 = 3;

const fn cb_dda_da_ctrl_dac(x: u32) -> u32 { x << 2 }
const fn cb_dda_caldac_deselect_bit(n: u32) -> u32 { 0x4 << n }
const fn cb_dda_da_data_reg(x: u32) -> usize { (0x08 + x * 2) as usize }
const fn cal_channel_bits(channel: u32) -> u32 { (channel << 1) & CAL_CHANNEL_MASK }

#[repr(C)]
pub struct ComediLRange { pub length: u32, pub range: [u32; 6] }
static CB_PCIDDA_RANGES: ComediLRange = ComediLRange { length: 6, range: [0; 6] };

#[repr(C)]
#[derive(Copy, Clone)]
enum CbPciddaBoardid { BoardDda02_12, BoardDda04_12, BoardDda08_12, BoardDda02_16, BoardDda04_16, BoardDda08_16 }

#[repr(C)]
struct CbPciddaBoard { name: *const u8, ao_chans: i32, ao_bits: i32 }

static CB_PCIDDA_BOARDS: [CbPciddaBoard; 6] = [
    CbPciddaBoard { name: b"pci-dda02/12\0".as_ptr(), ao_chans: 2, ao_bits: 12 },
    CbPciddaBoard { name: b"pci-dda04/12\0".as_ptr(), ao_chans: 4, ao_bits: 12 },
    CbPciddaBoard { name: b"pci-dda08/12\0".as_ptr(), ao_chans: 8, ao_bits: 12 },
    CbPciddaBoard { name: b"pci-dda02/16\0".as_ptr(), ao_chans: 2, ao_bits: 16 },
    CbPciddaBoard { name: b"pci-dda04/16\0".as_ptr(), ao_chans: 4, ao_bits: 16 },
    CbPciddaBoard { name: b"pci-dda08/16\0".as_ptr(), ao_chans: 8, ao_bits: 16 },
];

#[repr(C)]
struct CbPciddaPrivate { daqio: usize, dac_cal1_bits: u32, ao_range: [u32; MAX_AO_CHANNELS], eeprom_data: [u16; EEPROM_SIZE] }

extern "C" {
    fn inw_p(addr: usize) -> u16;
    fn outw_p(value: u32, addr: usize);
    fn outw(value: u32, addr: usize);
    fn CR_CHAN(chanspec: u32) -> u32;
    fn CR_RANGE(chanspec: u32) -> u32;
}

unsafe fn cb_pcidda_serial_in(dev: *mut ComediDevice) -> u32 {
    let p = (*dev).private as *mut CbPciddaPrivate; let mut value = 0; for i in 1..=16 { if inw_p((*p).daqio + DACALIBRATION1) as u32 & SERIAL_OUT_BIT != 0 { value |= 1 << (16 - i); } } value
}
unsafe fn cb_pcidda_serial_out(dev: *mut ComediDevice, value: u32, num_bits: u32) { let p = (*dev).private as *mut CbPciddaPrivate; for i in 1..=num_bits { if value & (1 << (num_bits-i)) != 0 { (*p).dac_cal1_bits |= SERIAL_IN_BIT; } else { (*p).dac_cal1_bits &= !SERIAL_IN_BIT; } outw_p((*p).dac_cal1_bits, (*p).daqio + DACALIBRATION1); } }
unsafe fn cb_pcidda_read_eeprom(dev: *mut ComediDevice, address: u32) -> u32 { let p=(*dev).private as *mut CbPciddaPrivate; let mut c=SELECT_EEPROM_BIT|DESELECT_REF_DAC_BIT|DUMMY_BIT; for i in 0..4 { c |= cb_dda_caldac_deselect_bit(i); } outw_p(c,(*p).daqio+DACALIBRATION2); cb_pcidda_serial_out(dev,6,3); cb_pcidda_serial_out(dev,address,8); let v=cb_pcidda_serial_in(dev); outw_p(c & !SELECT_EEPROM_BIT,(*p).daqio+DACALIBRATION2); v }
unsafe fn cb_pcidda_write_caldac(dev:*mut ComediDevice, caldac:u32, channel:u32, value:u32) { let p=(*dev).private as *mut CbPciddaPrivate; cb_pcidda_serial_out(dev,channel,3); cb_pcidda_serial_out(dev,value,8); let mut c=DESELECT_REF_DAC_BIT|DUMMY_BIT; for i in 0..4 { c|=cb_dda_caldac_deselect_bit(i); } c &= !cb_dda_caldac_deselect_bit(caldac); outw_p(c,(*p).daqio+DACALIBRATION2); outw_p(c|cb_dda_caldac_deselect_bit(caldac),(*p).daqio+DACALIBRATION2); }
unsafe fn cb_pcidda_calibrate(dev:*mut ComediDevice, channel:u32, range:u32) { let p=(*dev).private as *mut CbPciddaPrivate; let caldac=channel/2; let chan=4*(channel%2); let index=2*range+12*channel; (*p).ao_range[channel as usize]=range; let offset=(*p).eeprom_data[(7+index) as usize] as u32; let gain=(*p).eeprom_data[(8+index) as usize] as u32; cb_pcidda_write_caldac(dev,caldac,chan+2,(offset>>8)&0xff); cb_pcidda_write_caldac(dev,caldac,chan+3,offset&0xff); cb_pcidda_write_caldac(dev,caldac,chan+1,(gain>>8)&0xff); cb_pcidda_write_caldac(dev,caldac,chan,gain&0xff); }

#[repr(C)] pub struct ComediDevice { pub private:*mut core::ffi::c_void, pub board_ptr:*const CbPciddaBoard, pub board_name:*const u8, pub iobase:usize, pub subdevices:*mut ComediSubdevice }
#[repr(C)] pub struct ComediSubdevice { pub type_:u32, pub subdev_flags:u32, pub n_chan:u32, pub maxdata:u32, pub range_table:*const ComediLRange, pub insn_write:Option<unsafe extern "C" fn(*mut ComediDevice,*mut ComediSubdevice,*mut ComediInsn,*mut u32)->i32> }
#[repr(C)] pub struct ComediInsn { pub chanspec:u32, pub n:u32 }

unsafe extern "C" fn cb_pcidda_ao_insn_write(dev:*mut ComediDevice,_s:*mut ComediSubdevice,insn:*mut ComediInsn,data:*mut u32)->i32 { let p=(*dev).private as *mut CbPciddaPrivate; let ch=CR_CHAN((*insn).chanspec); let r=CR_RANGE((*insn).chanspec); if r!=(*p).ao_range[ch as usize] { cb_pcidda_calibrate(dev,ch,r); } let mut ctrl=CB_DDA_DA_CTRL_EN|cb_dda_da_ctrl_dac(ch); ctrl|=match r {0|3=>CB_DDA_DA_CTRL_RANGE10V,1|4=>CB_DDA_DA_CTRL_RANGE5V,_=>CB_DDA_DA_CTRL_RANGE2V5}; if r>2 {ctrl|=CB_DDA_DA_CTRL_UNIP;} outw(ctrl,(*p).daqio+CB_DDA_DA_CTRL_REG as usize); for i in 0..(*insn).n {outw(*data.add(i as usize),(*p).daqio+cb_dda_da_data_reg(ch));} (*insn).n as i32 }

extern "C" { fn comedi_to_pci_dev(dev:*mut ComediDevice)->*mut core::ffi::c_void; fn comedi_alloc_devpriv(dev:*mut ComediDevice,size:usize)->*mut core::ffi::c_void; fn comedi_pci_enable(dev:*mut ComediDevice)->i32; fn pci_resource_start(dev:*mut core::ffi::c_void,bar:u32)->usize; fn comedi_alloc_subdevices(dev:*mut ComediDevice,n:u32)->i32; fn subdev_8255_io_init(dev:*mut ComediDevice,s:*mut ComediSubdevice,offset:u32)->i32; fn comedi_pci_detach(dev:*mut ComediDevice)->i32; fn comedi_pci_auto_config(dev:*mut core::ffi::c_void,driver:*mut ComediDriver,data:usize)->i32; fn comedi_pci_auto_unconfig(dev:*mut core::ffi::c_void)->i32; }
#[repr(C)] pub struct ComediDriver { pub driver_name:*const u8, pub module:*mut core::ffi::c_void, pub auto_attach:Option<unsafe extern "C" fn(*mut ComediDevice,usize)->i32>, pub detach:Option<unsafe extern "C" fn(*mut ComediDevice)->i32> }

#[no_mangle] pub unsafe extern "C" fn cb_pcidda_auto_attach(dev:*mut ComediDevice,context:usize)->i32 { if context>=CB_PCIDDA_BOARDS.len(){return -19;} let board=&CB_PCIDDA_BOARDS[context]; (*dev).board_ptr=board; (*dev).board_name=board.name; let p=comedi_alloc_devpriv(dev,core::mem::size_of::<CbPciddaPrivate>()) as *mut CbPciddaPrivate; if p.is_null(){return -12;} (*dev).private=p; let ret=comedi_pci_enable(dev); if ret!=0{return ret;} let pci=comedi_to_pci_dev(dev); (*dev).iobase=pci_resource_start(pci,2); (*p).daqio=pci_resource_start(pci,3); let ret=comedi_alloc_subdevices(dev,3); if ret!=0{return ret;} let s=(*dev).subdevices; (*s).type_=2; (*s).subdev_flags=1; (*s).n_chan=board.ao_chans as u32; (*s).maxdata=(1u32<<board.ao_bits)-1; (*s).range_table=&CB_PCIDDA_RANGES; (*s).insn_write=Some(cb_pcidda_ao_insn_write); for i in 0..2 {let ret=subdev_8255_io_init(dev,s.add(1+i),i as u32*4); if ret!=0{return ret;}} for i in 0..EEPROM_SIZE {(*p).eeprom_data[i]=cb_pcidda_read_eeprom(dev,i as u32) as u16;} for i in 0..board.ao_chans as usize {cb_pcidda_calibrate(dev,i as u32,(*p).ao_range[i]);} 0 }

#[no_mangle] pub static mut cb_pcidda_driver:ComediDriver=ComediDriver{driver_name:b"cb_pcidda\0".as_ptr(),module:core::ptr::null_mut(),auto_attach:Some(cb_pcidda_auto_attach),detach:Some(comedi_pci_detach)};
#[repr(C)] pub struct PciDeviceId { pub vendor:u16, pub device:u16, pub driver_data:usize }
#[repr(C)] pub struct PciDriver { pub name:*const u8, pub id_table:*const PciDeviceId, pub probe:Option<unsafe extern "C" fn(*mut core::ffi::c_void,*const PciDeviceId)->i32>, pub remove:Option<unsafe extern "C" fn(*mut core::ffi::c_void)->i32> }
#[no_mangle] pub unsafe extern "C" fn cb_pcidda_pci_probe(dev:*mut core::ffi::c_void,id:*const PciDeviceId)->i32 { comedi_pci_auto_config(dev,&mut cb_pcidda_driver,(*id).driver_data) }
#[no_mangle] pub static cb_pcidda_pci_table:[PciDeviceId;7]=[PciDeviceId{vendor:0x1307,device:0x0020,driver_data:0},PciDeviceId{vendor:0x1307,device:0x0021,driver_data:1},PciDeviceId{vendor:0x1307,device:0x0022,driver_data:2},PciDeviceId{vendor:0x1307,device:0x0023,driver_data:3},PciDeviceId{vendor:0x1307,device:0x0024,driver_data:4},PciDeviceId{vendor:0x1307,device:0x0025,driver_data:5},PciDeviceId{vendor:0,device:0,driver_data:0}];
#[no_mangle] pub static mut cb_pcidda_pci_driver:PciDriver=PciDriver{name:b"cb_pcidda\0".as_ptr(),id_table:cb_pcidda_pci_table.as_ptr(),probe:Some(cb_pcidda_pci_probe),remove:Some(comedi_pci_auto_unconfig)};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
