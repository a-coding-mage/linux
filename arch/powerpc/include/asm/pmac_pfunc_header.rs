/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency types supplied by the surrounding kernel translation. */
pub struct list_head;
pub struct device_node;
pub struct kref;
pub struct module;

/* Flags in command lists */
pub const PMF_FLAGS_ON_INIT: u32 = 0x80000000u32;
pub const PMF_FLGAS_ON_TERM: u32 = 0x40000000u32;
pub const PMF_FLAGS_ON_SLEEP: u32 = 0x20000000u32;
pub const PMF_FLAGS_ON_WAKE: u32 = 0x10000000u32;
pub const PMF_FLAGS_ON_DEMAND: u32 = 0x08000000u32;
pub const PMF_FLAGS_INT_GEN: u32 = 0x04000000u32;
pub const PMF_FLAGS_HIGH_SPEED: u32 = 0x02000000u32;
pub const PMF_FLAGS_LOW_SPEED: u32 = 0x01000000u32;
pub const PMF_FLAGS_SIDE_EFFECTS: u32 = 0x00800000u32;

/*
 * Arguments to a platform function call.
 *
 * NOTE: By convention, pointer arguments point to an u32
 */
#[repr(C)]
pub union pmf_args_u {
    pub v: u32,
    pub p: *mut u32,
}

#[repr(C)]
pub struct pmf_args {
    pub u: [pmf_args_u; 4],
    pub count: core::ffi::c_uint,
}

#[repr(C)]
pub struct pmf_handlers {
    pub begin: Option<unsafe extern "C" fn(*mut pmf_function, *mut pmf_args) -> *mut core::ffi::c_void>,
    pub end: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void)>,
    pub irq_enable: Option<unsafe extern "C" fn(*mut pmf_function) -> core::ffi::c_int>,
    pub irq_disable: Option<unsafe extern "C" fn(*mut pmf_function) -> core::ffi::c_int>,
    pub write_gpio: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void, *mut pmf_args, u8, u8) -> core::ffi::c_int>,
    pub read_gpio: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void, *mut pmf_args, u8, core::ffi::c_int, u8) -> core::ffi::c_int>,
    pub write_reg32: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void, *mut pmf_args, u32, u32, u32) -> core::ffi::c_int>,
    pub read_reg32: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void, *mut pmf_args, u32) -> core::ffi::c_int>,
    pub write_reg16: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void, *mut pmf_args, u32, u16, u16) -> core::ffi::c_int>,
    pub read_reg16: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void, *mut pmf_args, u32) -> core::ffi::c_int>,
    pub write_reg8: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void, *mut pmf_args, u32, u8, u8) -> core::ffi::c_int>,
    pub read_reg8: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void, *mut pmf_args, u32) -> core::ffi::c_int>,
    pub delay: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void, *mut pmf_args, u32) -> core::ffi::c_int>,
    pub wait_reg32: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void, *mut pmf_args, u32, u32, u32) -> core::ffi::c_int>,
    pub wait_reg16: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void, *mut pmf_args, u32, u16, u16) -> core::ffi::c_int>,
    pub wait_reg8: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void, *mut pmf_args, u32, u8, u8) -> core::ffi::c_int>,
    pub read_i2c: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void, *mut pmf_args, u32) -> core::ffi::c_int>,
    pub write_i2c: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void, *mut pmf_args, u32, *const u8) -> core::ffi::c_int>,
    pub rmw_i2c: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void, *mut pmf_args, u32, u32, u32, *const u8, *const u8) -> core::ffi::c_int>,
    pub read_cfg: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void, *mut pmf_args, u32, u32) -> core::ffi::c_int>,
    pub write_cfg: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void, *mut pmf_args, u32, u32, *const u8) -> core::ffi::c_int>,
    pub rmw_cfg: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void, *mut pmf_args, u32, u32, u32, u32, *const u8, *const u8) -> core::ffi::c_int>,
    pub read_i2c_sub: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void, *mut pmf_args, u8, u32) -> core::ffi::c_int>,
    pub write_i2c_sub: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void, *mut pmf_args, u8, u32, *const u8) -> core::ffi::c_int>,
    pub set_i2c_mode: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void, *mut pmf_args, core::ffi::c_int) -> core::ffi::c_int>,
    pub rmw_i2c_sub: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void, *mut pmf_args, u8, u32, u32, u32, *const u8, *const u8) -> core::ffi::c_int>,
    pub read_reg32_msrx: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void, *mut pmf_args, u32, u32, u32, u32) -> core::ffi::c_int>,
    pub read_reg16_msrx: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void, *mut pmf_args, u32, u32, u32, u32) -> core::ffi::c_int>,
    pub read_reg8_msrx: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void, *mut pmf_args, u32, u32, u32, u32) -> core::ffi::c_int>,
    pub write_reg32_slm: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void, *mut pmf_args, u32, u32, u32) -> core::ffi::c_int>,
    pub write_reg16_slm: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void, *mut pmf_args, u32, u32, u32) -> core::ffi::c_int>,
    pub write_reg8_slm: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void, *mut pmf_args, u32, u32, u32) -> core::ffi::c_int>,
    pub mask_and_compare: Option<unsafe extern "C" fn(*mut pmf_function, *mut core::ffi::c_void, *mut pmf_args, u32, *const u8, *const u8) -> core::ffi::c_int>,
    pub owner: *mut module,
}

pub struct pmf_device;

#[repr(C)]
pub struct pmf_function {
    pub link: list_head,
    pub node: *mut device_node,
    pub driver_data: *mut core::ffi::c_void,
    pub dev: *mut pmf_device,
    pub name: *const core::ffi::c_char,
    pub phandle: u32,
    pub flags: u32,
    pub data: *const core::ffi::c_void,
    pub length: core::ffi::c_uint,
    pub irq_clients: list_head,
    pub ref_: kref,
}

#[repr(C)]
pub struct pmf_irq_client {
    pub handler: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub data: *mut core::ffi::c_void,
    pub owner: *mut module,
    pub link: list_head,
    pub func: *mut pmf_function,
}

extern "C" {
    pub fn pmf_register_driver(np: *mut device_node, handlers: *mut pmf_handlers, driverdata: *mut core::ffi::c_void) -> core::ffi::c_int;
    pub fn pmf_unregister_driver(np: *mut device_node);
    pub fn pmf_register_irq_client(np: *mut device_node, name: *const core::ffi::c_char, client: *mut pmf_irq_client) -> core::ffi::c_int;
    pub fn pmf_unregister_irq_client(client: *mut pmf_irq_client);
    pub fn pmf_do_irq(func: *mut pmf_function);
    pub fn pmf_do_functions(np: *mut device_node, name: *const core::ffi::c_char, phandle: u32, flags: u32, args: *mut pmf_args) -> core::ffi::c_int;
    pub fn pmf_call_function(target: *mut device_node, name: *const core::ffi::c_char, args: *mut pmf_args) -> core::ffi::c_int;
    pub fn pmf_find_function(target: *mut device_node, name: *const core::ffi::c_char) -> *mut pmf_function;
    pub fn pmf_get_function(func: *mut pmf_function) -> *mut pmf_function;
    pub fn pmf_put_function(func: *mut pmf_function);
    pub fn pmf_call_one(func: *mut pmf_function, args: *mut pmf_args) -> core::ffi::c_int;
    pub fn pmac_pfunc_base_install() -> core::ffi::c_int;
    pub fn pmac_pfunc_base_suspend();
    pub fn pmac_pfunc_base_resume();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
