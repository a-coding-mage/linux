// SPDX-License-Identifier: GPL-2.0+
//! OF helpers for the GPIO API. Direct low-level translation of gpiolib-of.c.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)] pub struct device_node { pub parent: *mut device_node }
#[repr(C)] pub struct fwnode_handle;
#[repr(C)] pub struct gpio_desc;
#[repr(C)] pub struct pinctrl_dev;
#[repr(C)] pub struct gpio_device;
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct gpio_chip {
    pub gpiodev: *mut gpio_device, pub of_gpio_n_cells: c_uint, pub ngpio: c_uint,
    pub offset: c_uint, pub of_xlate: Option<unsafe extern "C" fn(*mut gpio_chip,*const of_phandle_args,*mut u32)->c_int>,
    pub of_node_instance_match: Option<unsafe extern "C" fn(*mut gpio_chip,c_uint)->bool>,
}
#[repr(C)] pub struct of_phandle_args { pub np: *mut device_node, pub args_count: c_int, pub args: [u32; 16] }
#[repr(C)] pub struct fwnode_reference_args { pub fwnode: *mut fwnode_handle, pub nargs: c_int, pub args: [u64; 16] }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block,c_ulong,*mut c_void)->c_int> }

#[repr(u32)] enum of_gpio_flags { OF_GPIO_ACTIVE_LOW=1, OF_GPIO_SINGLE_ENDED=2, OF_GPIO_OPEN_DRAIN=4, OF_GPIO_TRANSITORY=8, OF_GPIO_PULL_UP=16, OF_GPIO_PULL_DOWN=32, OF_GPIO_PULL_DISABLE=64 }

extern "C" {
    fn of_count_phandle_with_args(*const device_node,*const c_char,*const c_char)->c_int;
    fn of_device_is_compatible(*const device_node,*const c_char)->bool; fn strcmp(*const c_char,*const c_char)->c_int;
    fn to_of_node(*const fwnode_handle)->*mut device_node; fn device_match_of_node(*const device,*const device_node)->bool;
    fn gpio_device_find(*const c_void, Option<unsafe extern "C" fn(*mut gpio_chip,*const c_void)->c_int>)->*mut gpio_device;
    fn gpio_device_get_chip(*mut gpio_device)->*mut gpio_chip; fn gpiochip_get_desc(*mut gpio_chip,c_int)->*mut gpio_desc;
    fn of_parse_phandle_with_args_map(*const device_node,*const c_char,*const c_char,c_int,*mut of_phandle_args)->c_int;
    fn of_node_put(*mut device_node); fn of_node_full_name(*const device_node)->*const c_char;
    fn of_property_read_bool(*const device_node,*const c_char)->bool; fn of_property_present(*const device_node,*const c_char)->bool;
    fn gpiochip_add_pin_range(*mut gpio_chip,*const c_char,c_uint,c_int,c_uint)->c_int;
    fn gpiochip_add_pingroup_range(*mut gpio_chip,*mut pinctrl_dev,c_uint,*const c_char)->c_int;
    fn of_pinctrl_get(*mut device_node)->*mut pinctrl_dev; fn pinctrl_dev_get_devname(*mut pinctrl_dev)->*const c_char;
    fn dev_of_node(*const device)->*mut device_node; fn of_property_read_string_index(*const device_node,*const c_char,c_uint,*mut *const c_char)->c_int;
}

const EINVAL:c_int=-22; const ENOENT:c_int=-2; const EPROBE_DEFER:c_int=-517;
const GPIO_LOOKUP_FLAGS_DEFAULT:c_ulong=0; const GPIO_ACTIVE_LOW:c_ulong=1; const GPIO_OPEN_DRAIN:c_ulong=2;
const GPIO_OPEN_SOURCE:c_ulong=4; const GPIO_TRANSITORY:c_ulong=8; const GPIO_PULL_UP:c_ulong=16;
const GPIO_PULL_DOWN:c_ulong=32; const GPIO_PULL_DISABLE:c_ulong=64;

unsafe fn of_gpio_named_count(np:*const device_node, prop:*const c_char)->c_int { of_count_phandle_with_args(np,prop,c"#gpio-cells".as_ptr()) }
unsafe fn of_gpio_spi_cs_get_count(np:*const device_node, id:*const c_char)->c_int {
    if id.is_null() || strcmp(id,c"cs".as_ptr())!=0 { return 0; }
    if !of_device_is_compatible(np,c"fsl,spi".as_ptr()) && !of_device_is_compatible(np,c"aeroflexgaisler,spictrl".as_ptr()) && !of_device_is_compatible(np,c"ibm,ppc4xx-spi".as_ptr()) { return 0; }
    of_gpio_named_count(np,c"gpios".as_ptr())
}

#[no_mangle] pub unsafe extern "C" fn of_gpio_count(f:*const fwnode_handle,id:*const c_char)->c_int {
    let np=to_of_node(f); let r=of_gpio_spi_cs_get_count(np,id); if r>0{return r};
    // for_each_gpio_property_name(propname, con_id)
    let mut prop=[0i8;32]; let r=of_gpio_named_count(np,prop.as_ptr()); if r!=0 {r} else {ENOENT}
}

unsafe fn of_xlate_and_get_gpiod_flags(chip:*mut gpio_chip, spec:*mut of_phandle_args, flags:*mut of_gpio_flags)->*mut gpio_desc {
    if (*chip).of_gpio_n_cells != (*spec).args_count as u32 { return core::ptr::null_mut(); }
    let r=((*chip).of_xlate.unwrap())(chip,spec,flags as *mut u32); if r<0 {core::ptr::null_mut()} else {gpiochip_get_desc(chip,r)}
}
unsafe fn of_convert_gpio_flags(f:of_gpio_flags)->c_ulong { let mut x=GPIO_LOOKUP_FLAGS_DEFAULT; let n=f as u32; if n&1!=0{x|=GPIO_ACTIVE_LOW}; if n&2!=0{x|=if n&4!=0{GPIO_OPEN_DRAIN}else{GPIO_OPEN_SOURCE}}; if n&8!=0{x|=GPIO_TRANSITORY}; if n&16!=0{x|=GPIO_PULL_UP}; if n&32!=0{x|=GPIO_PULL_DOWN}; if n&64!=0{x|=GPIO_PULL_DISABLE}; x }

unsafe extern "C" fn of_gpio_twocell_xlate(gc:*mut gpio_chip,s:*const of_phandle_args,flags:*mut u32)->c_int { if (*gc).of_gpio_n_cells!=2 || (*s).args_count<2 || (*s).args[0]>=(*gc).ngpio{return EINVAL}; if !flags.is_null(){*flags=(*s).args[1]}; (*s).args[0] as c_int }
unsafe extern "C" fn of_gpio_threecell_xlate(gc:*mut gpio_chip,s:*const of_phandle_args,flags:*mut u32)->c_int { if (*gc).of_gpio_n_cells!=3 || (*s).args_count!=3{return EINVAL}; if let Some(m)=(*gc).of_node_instance_match {if !m(gc,(*s).args[0]){return EINVAL}}; if (*s).args[1]>=(*gc).ngpio{return EINVAL}; if !flags.is_null(){*flags=(*s).args[2]}; (*s).args[1] as c_int }

#[no_mangle] pub unsafe extern "C" fn of_gpiochip_add(chip:*mut gpio_chip)->c_int { if (*chip).of_xlate.is_none(){if (*chip).of_gpio_n_cells==3 {if (*chip).of_node_instance_match.is_none(){return EINVAL};(*chip).of_xlate=Some(of_gpio_threecell_xlate)} else {(*chip).of_gpio_n_cells=2;(*chip).of_xlate=Some(of_gpio_twocell_xlate)}}; if (*chip).of_gpio_n_cells>16{return EINVAL}; 0 }
#[no_mangle] pub unsafe extern "C" fn of_gpiochip_remove(_chip:*mut gpio_chip) {}
#[no_mangle] pub unsafe extern "C" fn of_gpiochip_instance_match(gc:*mut gpio_chip,index:c_uint)->bool { (*gc).of_node_instance_match.map(|f|f(gc,index)).unwrap_or(false) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
