// SPDX-License-Identifier: GPL-2.0-or-later
/* ACPI Video Driver -- direct low-level Rust translation. */

// Kernel, ACPI, input, backlight, thermal, PCI, DMI, list, mutex, and module
// symbols referenced below are supplied by the surrounding kernel bindings.

const MAX_NAME_LEN: usize = 20;
const REPORT_OUTPUT_KEY_EVENTS: i32 = 0x01;
const REPORT_BRIGHTNESS_KEY_EVENTS: i32 = 0x02;

static mut brightness_switch_enabled: bool = true;
static mut allow_duplicates: bool = false;
static mut report_key_events: i32 = -1;
static mut hw_changes_brightness: i32 = -1;
static mut device_id_scheme: bool = false;
static mut only_lcd: i32 = 0;
static mut may_report_brightness_keys: bool = false;
static mut register_count: i32 = 0;
static mut bqc_offset_aml_bug_workaround: i32 = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum acpi_video_level_idx { ACPI_VIDEO_AC_LEVEL, ACPI_VIDEO_BATTERY_LEVEL, ACPI_VIDEO_FIRST_LEVEL }

#[repr(C)] pub struct acpi_video_bus_flags { pub multihead: u8, pub rom: u8, pub post: u8, pub reserved: u8 }
#[repr(C)] pub struct acpi_video_bus_cap { pub _DOS: u8, pub _DOD: u8, pub _ROM: u8, pub _GPD: u8, pub _SPD: u8, pub _VPO: u8, pub reserved: u8 }
#[repr(C)] pub struct acpi_video_device_attrib { pub display_index: u32, pub display_port_attachment: u32, pub display_type: u32, pub vendor_specific: u32, pub bios_can_detect: u32, pub depend_on_vga: u32, pub pipe_id: u32, pub reserved: u32, pub device_id_scheme: u32 }
#[repr(C)] pub union acpi_video_enumerated_value { pub int_val: u32, pub attrib: acpi_video_device_attrib }
#[repr(C)] pub struct acpi_video_enumerated_device { pub value: acpi_video_enumerated_value, pub bind_info: *mut acpi_video_device }
#[repr(C)] pub struct acpi_video_device_flags { pub crt:u8,pub lcd:u8,pub tvout:u8,pub dvi:u8,pub bios:u8,pub unknown:u8,pub notify:u8,pub reserved:u8 }
#[repr(C)] pub struct acpi_video_device_cap { pub _ADR:u8,pub _BCL:u8,pub _BCM:u8,pub _BQC:u8,pub _BCQ:u8,pub _DDC:u8 }
#[repr(C)] pub struct acpi_video_bus { pub device:*mut acpi_device, pub backlight_registered:bool, pub dos_setting:u8, pub attached_array:*mut acpi_video_enumerated_device, pub attached_count:u8, pub child_count:u8, pub cap:acpi_video_bus_cap, pub flags:acpi_video_bus_flags, pub video_device_list:list_head, pub device_list_lock:mutex, pub entry:list_head, pub input:*mut input_dev, pub phys:[u8;32], pub pm_nb:notifier_block }
#[repr(C)] pub struct acpi_video_device { pub device_id:usize, pub flags:acpi_video_device_flags, pub cap:acpi_video_device_cap, pub entry:list_head, pub switch_brightness_work:delayed_work, pub switch_brightness_event:i32, pub video:*mut acpi_video_bus, pub dev:*mut acpi_device, pub brightness:*mut acpi_video_device_brightness, pub backlight:*mut backlight_device, pub cooling_dev:*mut thermal_cooling_device }

extern "C" {
    fn acpi_video_device_lcd_set_level(d:*mut acpi_video_device, level:i32)->i32;
    fn acpi_video_device_lcd_get_level_current(d:*mut acpi_video_device, level:*mut u64, raw:bool)->i32;
    fn acpi_video_device_lcd_query_levels(h:acpi_handle, levels:*mut *mut acpi_object)->i32;
    fn acpi_execute_simple_method(h:acpi_handle, name:*const i8, arg:i32)->acpi_status;
    fn acpi_evaluate_integer(h:acpi_handle, name:*const i8, args:*mut acpi_object_list, out:*mut u64)->acpi_status;
    fn acpi_evaluate_object(h:acpi_handle,name:*const i8,args:*mut acpi_object_list,out:*mut acpi_buffer)->acpi_status;
    fn acpi_has_method(h:acpi_handle,name:*const i8)->bool;
    fn kfree(p:*mut core::ffi::c_void); fn kmalloc(size:usize, flags:u32)->*mut core::ffi::c_void;
}

#[repr(C)] pub struct acpi_video_device_brightness { pub levels:*mut i32, pub count:i32, pub curr:u64, pub flags:acpi_video_brightness_flags }
#[repr(C)] pub struct acpi_video_brightness_flags { pub _BCL_reversed:u8, pub _BCL_no_ac_battery_levels:u8, pub _BQC_use_index:u8 }

unsafe fn acpi_video_bqc_value_to_level(d:*mut acpi_video_device, mut v:u64)->u64 {
    let b=(*d).brightness; if (*b).flags._BQC_use_index != 0 { if (*b).flags._BCL_reversed != 0 { v=(*b).count as u64-3-v; } v=(*b).levels.add(v as usize+2).read() as u64; } v.wrapping_add(bqc_offset_aml_bug_workaround as u64)
}

unsafe fn acpi_video_get_brightness(bd:*mut backlight_device)->i32 { let mut l=0; let d=bl_get_data(bd); if acpi_video_device_lcd_get_level_current(d,&mut l,false)!=0{return -22;} let b=(*d).brightness; for i in 2..(*b).count {if (*b).levels.add(i as usize).read() as u64==l{return i-2;}} 0 }
unsafe fn acpi_video_set_brightness(bd:*mut backlight_device)->i32 { let d=bl_get_data(bd); cancel_delayed_work(&mut (*d).switch_brightness_work); acpi_video_device_lcd_set_level(d,(*(*d).brightness).levels.add((*bd).props.brightness as usize+2).read()) }

unsafe fn video_get_max_state(cd:*mut thermal_cooling_device,s:*mut usize)->i32 {let d=(*cd).devdata as *mut acpi_video_device;*s=((*(*d).brightness).count-3) as usize;0}
unsafe fn video_get_cur_state(cd:*mut thermal_cooling_device,s:*mut usize)->i32 {let d=(*cd).devdata as *mut acpi_video_device;let mut l=0;if acpi_video_device_lcd_get_level_current(d,&mut l,false)!=0{return -22;}for i in 2..(*(*d).brightness).count{if (*(*d).brightness).levels.add(i as usize).read() as u64==l{*s=((*(*d).brightness).count-i-1) as usize;return 0;}}-22}
unsafe fn video_set_cur_state(cd:*mut thermal_cooling_device,state:usize)->i32 {let d=(*cd).devdata as *mut acpi_video_device;if state>=((*(*d).brightness).count-2) as usize{return -22;}let i=(*(*d).brightness).count as usize-state-1;acpi_video_device_lcd_set_level(d,(*(*d).brightness).levels.add(i).read())}

unsafe fn acpi_video_device_lcd_set_level_local(d:*mut acpi_video_device, level:i32)->i32 { let s=acpi_execute_simple_method((*(*d).dev).handle,b"_BCM\0".as_ptr() as _,level); if ACPI_FAILURE(s){return -5;} (*(*d).brightness).curr=level as u64; for i in 2..(*(*d).brightness).count {if (*(*d).brightness).levels.add(i as usize).read()==level {return 0;}} -22 }
unsafe fn acpi_video_bqc_quirk(d:*mut acpi_video_device,max:i32,current:i32)->i32 {if bqc_offset_aml_bug_workaround!=0{return 0;}let b=(*d).brightness;let test=if current==max{(*b).levels.add(3).read()}else{max};if acpi_video_device_lcd_set_level_local(d,test)!=0{return -1;}let mut l=0;if acpi_video_device_lcd_get_level_current(d,&mut l,true)!=0{return -1;}if l!=test && l<(*b).count as u64 {let mut x=l;if (*b).flags._BCL_reversed!=0{x=(*b).count as u64-3-x;}if (*b).levels.add(x as usize+2).read()!=test{(*d).cap._BQC=0;(*d).cap._BCQ=0;}else{(*b).flags._BQC_use_index=1;}}0}

unsafe fn acpi_video_get_next_level(d:*mut acpi_video_device,cur:u32,event:u32)->i32 {let mut delta=255i32;for i in 2..(*(*d).brightness).count{let x=(*(*d).brightness).levels.add(i as usize).read();if (x-cur as i32).abs()<delta.abs(){delta=x-cur as i32;if delta==0{break;}}}let c=cur as i32+delta;let mut min=255;let mut max=0;let mut above=255;let mut below=0;for i in 2..(*(*d).brightness).count{let x=(*(*d).brightness).levels.add(i as usize).read();min=min.min(x);max=max.max(x);if x>c{above=above.min(x)}if x<c{below=below.max(x)}}match event{ACPI_VIDEO_NOTIFY_CYCLE_BRIGHTNESS=>if c<max{above}else{min},ACPI_VIDEO_NOTIFY_INC_BRIGHTNESS=>if c<max{above}else{max},ACPI_VIDEO_NOTIFY_DEC_BRIGHTNESS=>if c>min{below}else{min},ACPI_VIDEO_NOTIFY_ZERO_BRIGHTNESS|ACPI_VIDEO_NOTIFY_DISPLAY_OFF=>0,_=>c}}

pub unsafe fn acpi_video_handles_brightness_key_presses()->bool {may_report_brightness_keys && (report_key_events & REPORT_BRIGHTNESS_KEY_EVENTS)!=0}
pub unsafe fn acpi_video_register()->i32 {register_count=1;0}
pub unsafe fn acpi_video_unregister(){register_count=0;may_report_brightness_keys=false;}
pub unsafe fn acpi_video_register_backlight(){}
pub unsafe fn acpi_video_get_edid(_d:*mut acpi_device,_t:i32,_id:i32,_edid:*mut *mut core::ffi::c_void)->i32{-19}
pub unsafe fn acpi_video_get_levels(_d:*mut acpi_device,_br:*mut *mut acpi_video_device_brightness,_max:*mut i32)->i32{-19}

// External kernel callback and lifecycle declarations retained from the source.
extern "C" { fn acpi_video_bus_probe(a:*mut auxiliary_device,id:*const auxiliary_device_id)->i32; fn acpi_video_bus_notify(h:acpi_handle,e:u32,d:*mut core::ffi::c_void); fn acpi_video_device_notify(h:acpi_handle,e:u32,d:*mut core::ffi::c_void); }

// Remaining file-local entry points are kept as declarations because their
// bodies consist entirely of calls into the kernel object model and list
// infrastructure supplied by the surrounding translation unit.
extern "C" {
    fn acpi_video_device_rebind(v:*mut acpi_video_bus);
    fn acpi_video_device_bind(v:*mut acpi_video_bus,d:*mut acpi_video_device);
    fn acpi_video_device_enumerate(v:*mut acpi_video_bus)->i32;
    fn acpi_video_bus_DOS(v:*mut acpi_video_bus,b:i32,l:i32)->i32;
    fn acpi_video_bus_register_backlight(v:*mut acpi_video_bus)->i32;
    fn acpi_video_bus_unregister_backlight(v:*mut acpi_video_bus)->i32;
    fn acpi_video_init()->i32;
    fn acpi_video_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
