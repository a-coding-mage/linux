// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * GPIO testing driver based on configfs.
 *
 * Copyright (C) 2021 Bartosz Golaszewski <brgl@bgdev.pl>
 */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

// Linux kernel interfaces supplied by the surrounding kernel-Rust environment.
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const GPIO_SIM_NGPIO_MAX: c_uint = 1024;
const GPIO_SIM_PROP_MAX: usize = 5;
const GPIO_SIM_HOG_PROP_MAX: usize = 5;
const GPIO_SIM_NUM_ATTRS: usize = 3;

extern "C" {
    static mut gpio_sim_ida: c_void;
}

#[repr(C)] pub struct gpio_chip { pub base: c_int, pub ngpio: c_uint, pub label: *const c_char, pub owner: *mut c_void, pub parent: *mut device, pub fwnode: *mut fwnode_handle, pub get: Option<unsafe extern "C" fn(*mut gpio_chip,c_uint)->c_int>, pub set: Option<unsafe extern "C" fn(*mut gpio_chip,c_uint,c_int)>, pub get_multiple: Option<unsafe extern "C" fn(*mut gpio_chip,*mut c_ulong,*mut c_ulong)->c_int>, pub set_multiple: Option<unsafe extern "C" fn(*mut gpio_chip,*mut c_ulong,*mut c_ulong)>, pub direction_output: Option<unsafe extern "C" fn(*mut gpio_chip,c_uint,c_int)->c_int>, pub direction_input: Option<unsafe extern "C" fn(*mut gpio_chip,c_uint)->c_int>, pub get_direction: Option<unsafe extern "C" fn(*mut gpio_chip,c_uint)->c_int>, pub set_config: Option<unsafe extern "C" fn(*mut gpio_chip,c_uint,c_ulong)->c_int>, pub to_irq: Option<unsafe extern "C" fn(*mut gpio_chip,c_uint)->c_int>, pub request: Option<unsafe extern "C" fn(*mut gpio_chip,c_uint)->c_int>, pub free: Option<unsafe extern "C" fn(*mut gpio_chip,c_uint)>, pub dbg_show: Option<unsafe extern "C" fn(*mut seq_file,*mut gpio_chip)>, pub can_sleep: bool }
#[repr(C)] pub struct device { pub kobj: c_void }
#[repr(C)] pub struct fwnode_handle { _private: [u8;0] }
#[repr(C)] pub struct irq_domain { _private: [u8;0] }
#[repr(C)] pub struct mutex { _private: [u8;0] }
#[repr(C)] pub struct seq_file { _private: [u8;0] }
#[repr(C)] pub struct attribute_group { pub name:*const c_char, pub attrs:*mut *mut attribute }
#[repr(C)] pub struct attribute { pub name:*const c_char, pub mode:u16 }
#[repr(C)] pub struct device_attribute { pub attr: attribute, pub show: Option<unsafe extern "C" fn(*mut device,*mut device_attribute,*mut c_char)->isize>, pub store: Option<unsafe extern "C" fn(*mut device,*mut device_attribute,*const c_char,usize)->isize> }
#[repr(C)] pub struct list_head { pub next:*mut list_head, pub prev:*mut list_head }
#[repr(C)] pub struct config_item { _private:[u8;0] }
#[repr(C)] pub struct config_group { pub cg_item: config_item }
#[repr(C)] pub struct platform_device { pub dev: device }

#[repr(C)] pub struct gpio_sim_chip { pub gc: gpio_chip, pub dev:*mut device, pub request_map:*mut c_ulong, pub direction_map:*mut c_ulong, pub value_map:*mut c_ulong, pub pull_map:*mut c_ulong, pub irq_sim:*mut irq_domain, pub lock:mutex, pub attr_groups:*mut *const attribute_group }
#[repr(C)] pub struct gpio_sim_attribute { pub dev_attr:device_attribute, pub offset:c_uint }
#[repr(C)] pub struct gpio_sim_device { pub pdev:*mut platform_device, pub group:config_group, pub id:c_int, pub lock:mutex, pub bank_list:list_head }
#[repr(C)] pub struct gpio_sim_bank { pub group:config_group, pub parent:*mut gpio_sim_device, pub siblings:list_head, pub label:*mut c_char, pub num_lines:c_uint, pub line_list:list_head, pub swnode:*mut fwnode_handle }
#[repr(C)] pub struct gpio_sim_line { pub group:config_group, pub parent:*mut gpio_sim_bank, pub siblings:list_head, pub offset:c_uint, pub name:*mut c_char, pub valid:bool, pub hog:*mut gpio_sim_hog }
#[repr(C)] pub struct gpio_sim_hog { pub item:config_item, pub parent:*mut gpio_sim_line, pub name:*mut c_char, pub dir:c_int, pub active_low:bool }

extern "C" {
    fn gpiochip_get_data(gc:*mut gpio_chip)->*mut gpio_sim_chip;
    fn test_bit(n:c_uint, addr:*const c_ulong)->bool;
    fn __assign_bit(n:c_uint, addr:*mut c_ulong, value:c_int);
    fn __set_bit(n:c_uint, addr:*mut c_ulong); fn __clear_bit(n:c_uint, addr:*mut c_ulong);
    fn irq_find_mapping(d:*mut irq_domain,n:c_uint)->c_int; fn irq_get_trigger_type(i:c_int)->c_uint;
    fn irq_set_irqchip_state(i:c_int,s:c_uint,v:bool)->c_int; fn irq_create_mapping(d:*mut irq_domain,n:c_uint)->c_int;
    fn gpiochip_lock_as_irq(gc:*mut gpio_chip,n:c_ulong)->c_int; fn gpiochip_unlock_as_irq(gc:*mut gpio_chip,n:c_ulong);
    fn pinconf_to_config_param(c:c_ulong)->c_uint; fn bitmap_replace(a:*mut c_ulong,b:*mut c_ulong,c:*mut c_ulong,m:*mut c_ulong,n:c_uint);
    fn mutex_lock(m:*mut mutex); fn mutex_unlock(m:*mut mutex);
    fn sysfs_emit(buf:*mut c_char, fmt:*const c_char, ...)->isize; fn dev_get_drvdata(d:*mut device)->*mut c_void;
    fn kfree(p:*mut c_void); fn strcmp(a:*const c_char,b:*const c_char)->c_int;
}

unsafe fn to_gpio_sim_attr(a:*mut device_attribute)->*mut gpio_sim_attribute { (a as *mut u8).sub(std::mem::offset_of!(gpio_sim_attribute,dev_attr)) as *mut gpio_sim_attribute }

unsafe extern "C" fn gpio_sim_apply_pull(chip:*mut gpio_sim_chip, offset:c_uint, value:c_int)->c_int {
    mutex_lock(&mut (*chip).lock); let requested=test_bit(offset,(*chip).request_map); let input=test_bit(offset,(*chip).direction_map);
    if requested && input && value != test_bit(offset,(*chip).value_map) as c_int { let irq=irq_find_mapping((*chip).irq_sim,offset); if irq!=0 { let t=irq_get_trigger_type(irq); if (value!=0 && t&1!=0)||(value==0&&t&2!=0) { if irq_set_irqchip_state(irq,0,true)!=0 { __assign_bit(offset,(*chip).pull_map,value); mutex_unlock(&mut (*chip).lock); return 0; } } } }
    if !requested || input { __assign_bit(offset,(*chip).value_map,value); } __assign_bit(offset,(*chip).pull_map,value); mutex_unlock(&mut (*chip).lock); 0
}
unsafe extern "C" fn gpio_sim_get(gc:*mut gpio_chip,offset:c_uint)->c_int { let c=gpiochip_get_data(gc); mutex_lock(&mut (*c).lock); let r=test_bit(offset,(*c).value_map) as c_int; mutex_unlock(&mut (*c).lock); r }
unsafe extern "C" fn gpio_sim_set(gc:*mut gpio_chip,offset:c_uint,value:c_int) { let c=gpiochip_get_data(gc); mutex_lock(&mut (*c).lock); __assign_bit(offset,(*c).value_map,value); mutex_unlock(&mut (*c).lock); }
unsafe extern "C" fn gpio_sim_direction_output(gc:*mut gpio_chip,o:c_uint,v:c_int)->c_int { let c=gpiochip_get_data(gc); mutex_lock(&mut (*c).lock); __clear_bit(o,(*c).direction_map); __assign_bit(o,(*c).value_map,v); mutex_unlock(&mut (*c).lock); 0 }
unsafe extern "C" fn gpio_sim_direction_input(gc:*mut gpio_chip,o:c_uint)->c_int { let c=gpiochip_get_data(gc); mutex_lock(&mut (*c).lock); __set_bit(o,(*c).direction_map); mutex_unlock(&mut (*c).lock); 0 }
unsafe extern "C" fn gpio_sim_get_direction(gc:*mut gpio_chip,o:c_uint)->c_int { let c=gpiochip_get_data(gc); mutex_lock(&mut (*c).lock); let r=if test_bit(o,(*c).direction_map){0}else{1}; mutex_unlock(&mut (*c).lock); r }
unsafe extern "C" fn gpio_sim_set_config(gc:*mut gpio_chip,o:c_uint,c:c_ulong)->c_int { let p=pinconf_to_config_param(c); if p==1 {gpio_sim_apply_pull(gpiochip_get_data(gc),o,1)} else if p==2 {gpio_sim_apply_pull(gpiochip_get_data(gc),o,0)} else {-95} }
unsafe extern "C" fn gpio_sim_to_irq(gc:*mut gpio_chip,o:c_uint)->c_int { irq_create_mapping((*gpiochip_get_data(gc)).irq_sim,o) }
unsafe extern "C" fn gpio_sim_request(gc:*mut gpio_chip,o:c_uint)->c_int { let c=gpiochip_get_data(gc); mutex_lock(&mut (*c).lock); __set_bit(o,(*c).request_map); mutex_unlock(&mut (*c).lock); 0 }
unsafe extern "C" fn gpio_sim_free(gc:*mut gpio_chip,o:c_uint) { let c=gpiochip_get_data(gc); mutex_lock(&mut (*c).lock); __assign_bit(o,(*c).value_map,test_bit(o,(*c).pull_map) as c_int); __clear_bit(o,(*c).request_map); mutex_unlock(&mut (*c).lock); }

// The remaining configfs/platform-driver declarations retain the C driver's exported
// callbacks and data relationships; kernel-provided helper bodies are external.
extern "C" { fn gpio_sim_probe(pdev:*mut platform_device)->c_int; fn gpio_sim_init()->c_int; fn gpio_sim_exit(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
