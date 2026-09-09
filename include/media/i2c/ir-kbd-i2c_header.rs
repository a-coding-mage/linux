/* SPDX-License-Identifier: GPL-2.0 */

// Dependency declarations supplied by the surrounding media/i2c translation.
// #include <media/rc-core.h>

pub const DEFAULT_POLLING_INTERVAL: u32 = 100; // ms

pub struct IR_i2c;

#[repr(C)]
pub struct IR_i2c {
    pub ir_codes: *mut ::std::os::raw::c_char,
    pub c: *mut i2c_client,
    pub rc: *mut rc_dev,

    /* Used to avoid fast repeating */
    pub old: u8,

    pub polling_interval: u32, /* in ms */

    pub work: delayed_work,
    pub phys: [::std::os::raw::c_char; 32],
    pub get_key: Option<unsafe extern "C" fn(
        ir: *mut IR_i2c,
        protocol: *mut rc_proto,
        scancode: *mut u32,
        toggle: *mut u8,
    ) -> ::std::os::raw::c_int>,
    /* tx */
    pub tx_c: *mut i2c_client,
    pub lock: mutex, /* do not poll Rx during Tx */
    pub carrier: ::std::os::raw::c_uint,
    pub duty_cycle: ::std::os::raw::c_uint,
}

#[repr(C)]
pub enum ir_kbd_get_key_fn {
    IR_KBD_GET_KEY_CUSTOM = 0,
    IR_KBD_GET_KEY_PIXELVIEW,
    IR_KBD_GET_KEY_HAUP,
    IR_KBD_GET_KEY_KNC1,
    IR_KBD_GET_KEY_GENIATECH,
    IR_KBD_GET_KEY_FUSIONHDTV,
    IR_KBD_GET_KEY_HAUP_XVR,
    IR_KBD_GET_KEY_AVERMEDIA_CARDBUS,
}

/* Can be passed when instantiating an ir_video i2c device */
#[repr(C)]
pub struct IR_i2c_init_data {
    pub ir_codes: *mut ::std::os::raw::c_char,
    pub name: *const ::std::os::raw::c_char,
    pub type_: u64, /* RC_PROTO_BIT_RC5, etc */
    pub polling_interval: u32, /* 0 means DEFAULT_POLLING_INTERVAL */

    /*
     * Specify either a function pointer or a value indicating one of
     * ir_kbd_i2c's internal get_key functions
     */
    pub get_key: Option<unsafe extern "C" fn(
        ir: *mut IR_i2c,
        protocol: *mut rc_proto,
        scancode: *mut u32,
        toggle: *mut u8,
    ) -> ::std::os::raw::c_int>,
    pub internal_get_key_func: ir_kbd_get_key_fn,

    pub rc_dev: *mut rc_dev,
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
