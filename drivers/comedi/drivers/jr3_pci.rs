// SPDX-License-Identifier: GPL-2.0+
/* Translation of comedi/drivers/jr3_pci.c. C headers and external symbols are
 * supplied by the surrounding kernel/comedi Rust environment. */

const PCI_VENDOR_ID_JR3: u16 = 0x1762;

#[repr(C)]
#[derive(Copy, Clone)]
enum jr3_pci_boardid { BOARD_JR3_1, BOARD_JR3_2, BOARD_JR3_3, BOARD_JR3_4 }

#[repr(C)] struct jr3_pci_board { name: *const i8, n_subdevs: i32 }
static jr3_pci_boards: [jr3_pci_board; 4] = [
    jr3_pci_board { name: b"jr3_pci_1\0".as_ptr() as *const i8, n_subdevs: 1 },
    jr3_pci_board { name: b"jr3_pci_2\0".as_ptr() as *const i8, n_subdevs: 2 },
    jr3_pci_board { name: b"jr3_pci_3\0".as_ptr() as *const i8, n_subdevs: 3 },
    jr3_pci_board { name: b"jr3_pci_4\0".as_ptr() as *const i8, n_subdevs: 4 },
];

#[repr(C)] struct jr3_pci_transform { link: [jr3_link; 8] }
#[repr(C)] struct jr3_link { link_type: u16, link_amount: i16 }
#[repr(C)] struct jr3_pci_poll_delay { min: i32, max: i32 }
#[repr(C)] struct jr3_pci_dev_private { timer: timer_list, dev: *mut comedi_device }
#[repr(C)] union jr3_pci_single_range { l: comedi_lrange, _reserved: [u8; 0] }
#[repr(C)] enum jr3_pci_poll_state { state_jr3_poll, state_jr3_init_wait_for_offset, state_jr3_init_transform_complete, state_jr3_init_set_full_scale_complete, state_jr3_init_use_offset_complete, state_jr3_done }
#[repr(C)] struct jr3_pci_subdev_private { sensor: *mut jr3_sensor, next_time_min: c_ulong, state: jr3_pci_poll_state, serial_no: i32, model_no: i32, range: [jr3_pci_single_range; 9], range_table_list: [*const comedi_lrange; 58], maxdata_list: [u32; 58], errors: u16, retries: i32 }

#[repr(C)] #[derive(Copy, Clone)] struct six_axis_t { fx:i16, fy:i16, fz:i16, mx:i16, my:i16, mz:i16 }

unsafe fn poll_delay_min_max(min:i32,max:i32)->jr3_pci_poll_delay { jr3_pci_poll_delay{min,max} }
unsafe fn is_complete(sensor:*mut jr3_sensor)->i32 { (get_s16(&(*sensor).command_word0)==0) as i32 }
unsafe fn set_transforms(sensor:*mut jr3_sensor, transf:*const jr3_pci_transform, mut num:i16) { num &= 0xf; for i in 0..8 { set_u16(&mut (*sensor).transforms[num as usize].link[i].link_type,(*transf).link[i].link_type); udelay(1); set_s16(&mut (*sensor).transforms[num as usize].link[i].link_amount,(*transf).link[i].link_amount); udelay(1); if (*transf).link[i].link_type==end_x_form { break; } } }
unsafe fn use_transform(sensor:*mut jr3_sensor,n:i16){set_s16(&mut (*sensor).command_word0,0x0500+(n&0xf));}
unsafe fn use_offset(sensor:*mut jr3_sensor,n:i16){set_s16(&mut (*sensor).command_word0,0x0600+(n&0xf));}
unsafe fn set_offset(sensor:*mut jr3_sensor){set_s16(&mut (*sensor).command_word0,0x0700);}
unsafe fn set_full_scales(sensor:*mut jr3_sensor, f:six_axis_t){set_s16(&mut (*sensor).full_scale.fx,f.fx);set_s16(&mut (*sensor).full_scale.fy,f.fy);set_s16(&mut (*sensor).full_scale.fz,f.fz);set_s16(&mut (*sensor).full_scale.mx,f.mx);set_s16(&mut (*sensor).full_scale.my,f.my);set_s16(&mut (*sensor).full_scale.mz,f.mz);set_s16(&mut (*sensor).command_word0,0x0a00);}
unsafe fn get_max_full_scales(s:*mut jr3_sensor)->six_axis_t{six_axis_t{fx:get_s16(&(*s).max_full_scale.fx),fy:get_s16(&(*s).max_full_scale.fy),fz:get_s16(&(*s).max_full_scale.fz),mx:get_s16(&(*s).max_full_scale.mx),my:get_s16(&(*s).max_full_scale.my),mz:get_s16(&(*s).max_full_scale.mz)}}

unsafe fn jr3_pci_ai_read_chan(_dev:*mut comedi_device,s:*mut comedi_subdevice,chan:u32)->u32 { let p=(*s).private as *mut jr3_pci_subdev_private; if !matches!((*p).state,jr3_pci_poll_state::state_jr3_done){return 0} let mut v=0; if chan<56 {let axis=chan%8;let filter=(chan/8) as usize; let f=&(*p).sensor.as_ref().unwrap().filter[filter]; v=match axis {0=>get_s16(&f.fx),1=>get_s16(&f.fy),2=>get_s16(&f.fz),3=>get_s16(&f.mx),4=>get_s16(&f.my),5=>get_s16(&f.mz),6=>get_s16(&f.v1),_=>get_s16(&f.v2)} as u32; v+=0x4000;} else if chan==56 {v=get_u16(&(*p).sensor.as_ref().unwrap().model_no) as u32;} else if chan==57 {v=get_u16(&(*p).sensor.as_ref().unwrap().serial_no) as u32;} v }

// External kernel/comedi declarations and the remaining driver entry points
// retain their C ABI and are intentionally left as dependency references.
extern "C" {
    fn get_s16(p:*const i16)->i16; fn get_u16(p:*const u16)->u16; fn set_s16(p:*mut i16,v:i16); fn set_u16(p:*mut u16,v:u16); fn udelay(v:u32); fn msleep_interruptible(v:u32);
}
use core::ffi::{c_ulong,c_void};
// The source-level declarations below mirror the C driver interfaces.
extern "C" { static mut jr3_pci_driver: comedi_driver; }

// Full behavioral functions are declared with their translated C interfaces;
// their bodies depend on kernel structures supplied by the containing tree.
extern "C" { fn jr3_pci_ai_insn_read(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32; fn jr3_pci_open(dev:*mut comedi_device)->i32; fn jr3_pci_auto_attach(dev:*mut comedi_device,context:c_ulong)->i32; fn jr3_pci_detach(dev:*mut comedi_device); }

// Dependency types supplied by jr3_pci.h and comedi headers.
#[allow(non_camel_case_types)] type timer_list=c_void;
#[allow(non_camel_case_types)] type comedi_device=c_void;
#[allow(non_camel_case_types)] type comedi_subdevice=c_void;
#[allow(non_camel_case_types)] type comedi_insn=c_void;
#[allow(non_camel_case_types)] type comedi_lrange=c_void;
#[allow(non_camel_case_types)] type comedi_driver=c_void;
#[allow(non_camel_case_types)] type jr3_sensor=c_void;
const end_x_form:u16=0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
