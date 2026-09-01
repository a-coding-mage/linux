// SPDX-License-Identifier: GPL-2.0-only
//
// KUnit test for the Cirrus side-codec library.
//
// Copyright (C) 2023 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const GPIO_LINE_DIRECTION_IN: c_int = 1;
const PIN_CONFIG_LEVEL: c_int = 1;
const PIN_CONFIG_OUTPUT_ENABLE: c_int = 2;
const EOPNOTSUPP: c_int = 95;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const GFP_KERNEL: c_ulong = 0;
const KUNIT_PARAM_DESC_SIZE: usize = 128;

const fn BIT(nr: c_uint) -> c_uint {
    1u32.wrapping_shl(nr)
}

type c_uint = u32;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct faux_device {
    pub dev: device,
}

#[repr(C)]
pub struct gpio_chip {
    pub label: *const c_char,
    pub owner: *mut c_void,
    pub request: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub free: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint)>,
    pub get_direction: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub direction_input: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub direction_output: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint, c_int) -> c_int>,
    pub set: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint, c_int) -> c_int>,
    pub set_config: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint, c_ulong) -> c_int>,
    pub base: c_int,
    pub ngpio: u16,
    pub parent: *mut device,
}

#[repr(C)]
pub struct software_node {
    pub name: *const c_char,
    pub properties: *const property_entry,
}

#[repr(C)]
pub struct software_node_ref_args {
    pub node: *const software_node,
    pub nargs: c_uint,
    pub args: [u64; 2],
}

#[repr(C)]
pub struct property_entry {
    pub name: *const c_char,
    pub length: usize,
    pub is_array: bool,
    pub pointer: *const c_void,
}

#[repr(C)]
pub struct faux_device_ops {
    pub probe: Option<unsafe extern "C" fn(*mut faux_device) -> c_int>,
}

#[repr(C)]
pub struct kunit {
    pub priv_: *mut c_void,
    pub param_value: *const c_void,
}

#[repr(C)]
pub struct kunit_case {
    pub run_case: Option<unsafe extern "C" fn(*mut kunit)>,
    pub generate_params: *const c_void,
}

#[repr(C)]
pub struct kunit_suite {
    pub name: *const c_char,
    pub init: Option<unsafe extern "C" fn(*mut kunit) -> c_int>,
    pub test_cases: *mut kunit_case,
}

#[repr(C)]
struct cirrus_scodec_test_gpio {
    pin_state: c_uint,
    chip: gpio_chip,
}

#[repr(C)]
struct cirrus_scodec_test_priv {
    amp_dev: *mut faux_device,
    gpio_dev: *mut faux_device,
    gpio_priv: *mut cirrus_scodec_test_gpio,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;

    fn gpiochip_generic_request(chip: *mut gpio_chip, offset: c_uint) -> c_int;
    fn gpiochip_generic_free(chip: *mut gpio_chip, offset: c_uint);
    fn gpiochip_get_data(chip: *mut gpio_chip) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_ulong) -> *mut c_void;
    fn device_add_software_node(dev: *mut device, node: *const software_node) -> c_int;
    fn device_remove_software_node(dev: *mut device);
    fn devm_add_action_or_reset(
        dev: *mut device,
        action: Option<unsafe extern "C" fn(*mut c_void)>,
        data: *mut c_void,
    ) -> c_int;
    fn devm_gpiochip_add_data(
        dev: *mut device,
        chip: *mut gpio_chip,
        data: *mut c_void,
    ) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn faux_device_create(
        name: *const c_char,
        parent: *mut device,
        ops: *const faux_device_ops,
    ) -> *mut faux_device;
    fn faux_device_destroy(fdev: *mut faux_device);
    fn kunit_add_action_or_reset(
        test: *mut kunit,
        action: Option<unsafe extern "C" fn(*mut c_void)>,
        data: *mut c_void,
    ) -> c_int;
    fn kunit_kzalloc(test: *mut kunit, size: usize, gfp: c_ulong) -> *mut c_void;
    fn kunit_kcalloc(test: *mut kunit, n: usize, size: usize, gfp: c_ulong) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn pinconf_to_config_param(config: c_ulong) -> c_int;
    fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn cirrus_scodec_get_speaker_id(
        dev: *mut device,
        amp_index: c_int,
        num_amps: c_int,
        fixed_gpio_id: c_int,
    ) -> c_int;
}

unsafe extern "C" fn faux_device_destroy_wrapper(data: *mut c_void) {
    unsafe { faux_device_destroy(data as *mut faux_device) };
}

unsafe extern "C" fn device_remove_software_node_wrapper(data: *mut c_void) {
    unsafe { device_remove_software_node(data as *mut device) };
}

unsafe extern "C" fn cirrus_scodec_test_gpio_get_direction(
    _chip: *mut gpio_chip,
    _offset: c_uint,
) -> c_int {
    GPIO_LINE_DIRECTION_IN
}

unsafe extern "C" fn cirrus_scodec_test_gpio_direction_in(
    _chip: *mut gpio_chip,
    _offset: c_uint,
) -> c_int {
    0
}

unsafe extern "C" fn cirrus_scodec_test_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    let gpio_priv = unsafe { gpiochip_get_data(chip) as *mut cirrus_scodec_test_gpio };

    unsafe { (((*gpio_priv).pin_state & BIT(offset)) != 0) as c_int }
}

unsafe extern "C" fn cirrus_scodec_test_gpio_direction_out(
    _chip: *mut gpio_chip,
    _offset: c_uint,
    _value: c_int,
) -> c_int {
    -EOPNOTSUPP
}

unsafe extern "C" fn cirrus_scodec_test_gpio_set(
    _chip: *mut gpio_chip,
    _offset: c_uint,
    _value: c_int,
) -> c_int {
    -EOPNOTSUPP
}

unsafe extern "C" fn cirrus_scodec_test_gpio_set_config(
    _gc: *mut gpio_chip,
    _offset: c_uint,
    config: c_ulong,
) -> c_int {
    match unsafe { pinconf_to_config_param(config) } {
        PIN_CONFIG_LEVEL | PIN_CONFIG_OUTPUT_ENABLE => -EOPNOTSUPP,
        _ => 0,
    }
}

static mut cirrus_scodec_test_gpio_chip: gpio_chip = gpio_chip {
    label: c"cirrus_scodec_test_gpio".as_ptr(),
    owner: ptr::null_mut(),
    request: Some(gpiochip_generic_request),
    free: Some(gpiochip_generic_free),
    get_direction: Some(cirrus_scodec_test_gpio_get_direction),
    direction_input: Some(cirrus_scodec_test_gpio_direction_in),
    get: Some(cirrus_scodec_test_gpio_get),
    direction_output: Some(cirrus_scodec_test_gpio_direction_out),
    set: Some(cirrus_scodec_test_gpio_set),
    set_config: Some(cirrus_scodec_test_gpio_set_config),
    base: -1,
    ngpio: 32,
    parent: ptr::null_mut(),
};

/* software_node referencing the gpio driver */
static cirrus_scodec_test_gpio_swnode: software_node = software_node {
    name: c"cirrus_scodec_test_gpio".as_ptr(),
    properties: ptr::null(),
};

unsafe extern "C" fn cirrus_scodec_test_gpio_probe(fdev: *mut faux_device) -> c_int {
    let gpio_priv: *mut cirrus_scodec_test_gpio;
    let mut ret: c_int;

    gpio_priv = unsafe {
        devm_kzalloc(
            &mut (*fdev).dev,
            size_of::<cirrus_scodec_test_gpio>(),
            GFP_KERNEL,
        ) as *mut cirrus_scodec_test_gpio
    };
    if gpio_priv.is_null() {
        return -ENOMEM;
    }

    ret = unsafe { device_add_software_node(&mut (*fdev).dev, &cirrus_scodec_test_gpio_swnode) };
    if ret != 0 {
        return ret;
    }

    ret = unsafe {
        devm_add_action_or_reset(
            &mut (*fdev).dev,
            Some(device_remove_software_node_wrapper),
            &mut (*fdev).dev as *mut device as *mut c_void,
        )
    };
    if ret != 0 {
        return ret;
    }

    /* GPIO core modifies our struct gpio_chip so use a copy */
    unsafe {
        (*gpio_priv).chip = cirrus_scodec_test_gpio_chip;
        (*gpio_priv).chip.owner = THIS_MODULE;
        (*gpio_priv).chip.parent = &mut (*fdev).dev;
        ret = devm_gpiochip_add_data(
            &mut (*fdev).dev,
            &mut (*gpio_priv).chip,
            gpio_priv as *mut c_void,
        );
    }
    if ret != 0 {
        return unsafe {
            dev_err_probe(
                &mut (*fdev).dev,
                ret,
                c"Failed to add gpiochip\n".as_ptr(),
            )
        };
    }

    unsafe { dev_set_drvdata(&mut (*fdev).dev, gpio_priv as *mut c_void) };

    0
}

static cirrus_scodec_test_gpio_driver_ops: faux_device_ops = faux_device_ops {
    probe: Some(cirrus_scodec_test_gpio_probe),
};

unsafe extern "C" fn cirrus_scodec_test_create_gpio(test: *mut kunit) {
    let priv_ = unsafe { (*test).priv_ as *mut cirrus_scodec_test_priv };

    unsafe {
        (*priv_).gpio_dev = faux_device_create(
            c"cirrus_scodec_test_gpio_drv".as_ptr(),
            ptr::null_mut(),
            &cirrus_scodec_test_gpio_driver_ops,
        );
        KUNIT_ASSERT_NOT_NULL(test, (*priv_).gpio_dev as *const c_void);
        KUNIT_ASSERT_EQ(
            test,
            0,
            kunit_add_action_or_reset(
                test,
                Some(faux_device_destroy_wrapper),
                (*priv_).gpio_dev as *mut c_void,
            ),
        );

        (*priv_).gpio_priv = dev_get_drvdata(&mut (*(*priv_).gpio_dev).dev)
            as *mut cirrus_scodec_test_gpio;
        KUNIT_ASSERT_NOT_NULL(test, (*priv_).gpio_priv as *const c_void);
    }
}

unsafe extern "C" fn cirrus_scodec_test_set_gpio_ref_arg(
    arg: *mut software_node_ref_args,
    gpio_num: c_int,
) {
    let template = software_node_ref_args {
        node: &cirrus_scodec_test_gpio_swnode,
        nargs: 2,
        args: [gpio_num as u64, 0],
    };

    unsafe { *arg = template };
}

unsafe extern "C" fn cirrus_scodec_test_set_spkid_swnode(
    test: *mut kunit,
    dev: *mut device,
    args: *mut software_node_ref_args,
    num_args: c_int,
) -> c_int {
    let props_template = [
        property_entry {
            name: c"spk-id-gpios".as_ptr(),
            length: num_args as usize,
            is_array: true,
            pointer: args as *const c_void,
        },
        property_entry {
            name: ptr::null(),
            length: 0,
            is_array: false,
            pointer: ptr::null(),
        },
    ];
    let props: *mut property_entry;
    let node: *mut software_node;

    node = unsafe { kunit_kzalloc(test, size_of::<software_node>(), GFP_KERNEL) as *mut software_node };
    if node.is_null() {
        return -ENOMEM;
    }

    props = unsafe {
        kunit_kzalloc(test, size_of_val(&props_template), GFP_KERNEL) as *mut property_entry
    };
    if props.is_null() {
        return -ENOMEM;
    }

    unsafe {
        memcpy(
            props as *mut c_void,
            props_template.as_ptr() as *const c_void,
            size_of_val(&props_template),
        );
        (*node).properties = props;
        device_add_software_node(dev, node)
    }
}

#[repr(C)]
struct cirrus_scodec_test_spkid_param {
    num_amps: c_int,
    gpios_per_amp: c_int,
    num_amps_sharing: c_int,
}

unsafe extern "C" fn cirrus_scodec_test_spkid_parse(test: *mut kunit) {
    let priv_ = unsafe { (*test).priv_ as *mut cirrus_scodec_test_priv };
    let param =
        unsafe { (*test).param_value as *const cirrus_scodec_test_spkid_param };
    let num_spk_id_refs = unsafe { (*param).num_amps * (*param).gpios_per_amp };
    let refs: *mut software_node_ref_args;
    let dev = unsafe { &mut (*(*priv_).amp_dev).dev as *mut device };
    let mut v: c_uint;
    let mut i: c_int;
    let mut ret: c_int;

    refs = unsafe {
        kunit_kcalloc(
            test,
            num_spk_id_refs as usize,
            size_of::<software_node_ref_args>(),
            GFP_KERNEL,
        ) as *mut software_node_ref_args
    };
    unsafe { KUNIT_ASSERT_NOT_NULL(test, refs as *const c_void) };

    i = 0;
    v = 0;
    while i < num_spk_id_refs {
        unsafe { cirrus_scodec_test_set_gpio_ref_arg(refs.add(i as usize), v as c_int) };
        i += 1;
        v = v.wrapping_add(1);

        /*
         * If amps are sharing GPIOs repeat the last set of
         * GPIOs until we've done that number of amps.
         * We have done all GPIOs for an amp when i is a multiple
         * of gpios_per_amp.
         * We have done all amps sharing the same GPIOs when i is
         * a multiple of (gpios_per_amp * num_amps_sharing).
         */
        unsafe {
            if i % (*param).gpios_per_amp == 0
                && i % ((*param).gpios_per_amp * (*param).num_amps_sharing) != 0
            {
                v = v.wrapping_sub((*param).gpios_per_amp as c_uint);
            }
        }
    }

    ret = unsafe { cirrus_scodec_test_set_spkid_swnode(test, dev, refs, num_spk_id_refs) };
    unsafe { KUNIT_EXPECT_EQ_MSG(test, ret, 0, c"Failed to add swnode\n".as_ptr()) };

    i = 0;
    while unsafe { i < (*param).num_amps } {
        v = 0;
        while unsafe { v < (1u32 << (*param).gpios_per_amp) } {
            /* Set only the GPIO bits used by this amp */
            unsafe {
                (*(*priv_).gpio_priv).pin_state =
                    v << ((*param).gpios_per_amp * (i / (*param).num_amps_sharing));

                ret = cirrus_scodec_get_speaker_id(dev, i, (*param).num_amps, -1);
                KUNIT_EXPECT_EQ_MSG(
                    test,
                    ret,
                    v as c_int,
                    c"get_speaker_id failed amp:%d pin_state:%#x\n".as_ptr(),
                    i,
                    (*(*priv_).gpio_priv).pin_state,
                );
            }
            v = v.wrapping_add(1);
        }
        i += 1;
    }
}

unsafe extern "C" fn cirrus_scodec_test_no_spkid(test: *mut kunit) {
    let priv_ = unsafe { (*test).priv_ as *mut cirrus_scodec_test_priv };
    let dev = unsafe { &mut (*(*priv_).amp_dev).dev as *mut device };
    let ret: c_int;

    ret = unsafe { cirrus_scodec_get_speaker_id(dev, 0, 4, -1) };
    unsafe { KUNIT_EXPECT_EQ(test, ret, -ENOENT) };
}

unsafe extern "C" fn cirrus_scodec_test_case_init(test: *mut kunit) -> c_int {
    let priv_: *mut cirrus_scodec_test_priv;

    priv_ = unsafe {
        kunit_kzalloc(
            test,
            size_of::<cirrus_scodec_test_priv>(),
            GFP_KERNEL,
        ) as *mut cirrus_scodec_test_priv
    };
    if priv_.is_null() {
        return -ENOMEM;
    }

    unsafe { (*test).priv_ = priv_ as *mut c_void };

    /* Create dummy GPIO */
    unsafe { cirrus_scodec_test_create_gpio(test) };

    /* Create dummy amp driver dev */
    unsafe {
        (*priv_).amp_dev = faux_device_create(
            c"cirrus_scodec_test_amp_drv".as_ptr(),
            ptr::null_mut(),
            ptr::null(),
        );
        KUNIT_ASSERT_NOT_NULL(test, (*priv_).amp_dev as *const c_void);
        KUNIT_ASSERT_EQ(
            test,
            0,
            kunit_add_action_or_reset(
                test,
                Some(faux_device_destroy_wrapper),
                (*priv_).amp_dev as *mut c_void,
            ),
        );
    }

    0
}

static cirrus_scodec_test_spkid_param_cases: [cirrus_scodec_test_spkid_param; 28] = [
    cirrus_scodec_test_spkid_param { num_amps: 2, gpios_per_amp: 1, num_amps_sharing: 1 },
    cirrus_scodec_test_spkid_param { num_amps: 2, gpios_per_amp: 2, num_amps_sharing: 1 },
    cirrus_scodec_test_spkid_param { num_amps: 2, gpios_per_amp: 3, num_amps_sharing: 1 },
    cirrus_scodec_test_spkid_param { num_amps: 2, gpios_per_amp: 4, num_amps_sharing: 1 },
    cirrus_scodec_test_spkid_param { num_amps: 3, gpios_per_amp: 1, num_amps_sharing: 1 },
    cirrus_scodec_test_spkid_param { num_amps: 3, gpios_per_amp: 2, num_amps_sharing: 1 },
    cirrus_scodec_test_spkid_param { num_amps: 3, gpios_per_amp: 3, num_amps_sharing: 1 },
    cirrus_scodec_test_spkid_param { num_amps: 3, gpios_per_amp: 4, num_amps_sharing: 1 },
    cirrus_scodec_test_spkid_param { num_amps: 4, gpios_per_amp: 1, num_amps_sharing: 1 },
    cirrus_scodec_test_spkid_param { num_amps: 4, gpios_per_amp: 2, num_amps_sharing: 1 },
    cirrus_scodec_test_spkid_param { num_amps: 4, gpios_per_amp: 3, num_amps_sharing: 1 },
    cirrus_scodec_test_spkid_param { num_amps: 4, gpios_per_amp: 4, num_amps_sharing: 1 },

    /* Same GPIO shared by all amps */
    cirrus_scodec_test_spkid_param { num_amps: 2, gpios_per_amp: 1, num_amps_sharing: 2 },
    cirrus_scodec_test_spkid_param { num_amps: 2, gpios_per_amp: 2, num_amps_sharing: 2 },
    cirrus_scodec_test_spkid_param { num_amps: 2, gpios_per_amp: 3, num_amps_sharing: 2 },
    cirrus_scodec_test_spkid_param { num_amps: 2, gpios_per_amp: 4, num_amps_sharing: 2 },
    cirrus_scodec_test_spkid_param { num_amps: 3, gpios_per_amp: 1, num_amps_sharing: 3 },
    cirrus_scodec_test_spkid_param { num_amps: 3, gpios_per_amp: 2, num_amps_sharing: 3 },
    cirrus_scodec_test_spkid_param { num_amps: 3, gpios_per_amp: 3, num_amps_sharing: 3 },
    cirrus_scodec_test_spkid_param { num_amps: 3, gpios_per_amp: 4, num_amps_sharing: 3 },
    cirrus_scodec_test_spkid_param { num_amps: 4, gpios_per_amp: 1, num_amps_sharing: 4 },
    cirrus_scodec_test_spkid_param { num_amps: 4, gpios_per_amp: 2, num_amps_sharing: 4 },
    cirrus_scodec_test_spkid_param { num_amps: 4, gpios_per_amp: 3, num_amps_sharing: 4 },
    cirrus_scodec_test_spkid_param { num_amps: 4, gpios_per_amp: 4, num_amps_sharing: 4 },

    /* Two sets of shared GPIOs */
    cirrus_scodec_test_spkid_param { num_amps: 4, gpios_per_amp: 1, num_amps_sharing: 2 },
    cirrus_scodec_test_spkid_param { num_amps: 4, gpios_per_amp: 2, num_amps_sharing: 2 },
    cirrus_scodec_test_spkid_param { num_amps: 4, gpios_per_amp: 3, num_amps_sharing: 2 },
    cirrus_scodec_test_spkid_param { num_amps: 4, gpios_per_amp: 4, num_amps_sharing: 2 },
];

unsafe extern "C" fn cirrus_scodec_test_spkid_param_desc(
    param: *const cirrus_scodec_test_spkid_param,
    desc: *mut c_char,
) {
    unsafe {
        snprintf(
            desc,
            KUNIT_PARAM_DESC_SIZE,
            c"amps:%d gpios_per_amp:%d num_amps_sharing:%d".as_ptr(),
            (*param).num_amps,
            (*param).gpios_per_amp,
            (*param).num_amps_sharing,
        );
    }
}

// KUNIT_ARRAY_PARAM(cirrus_scodec_test_spkid, cirrus_scodec_test_spkid_param_cases,
//                   cirrus_scodec_test_spkid_param_desc);
unsafe extern "C" {
    static cirrus_scodec_test_spkid_gen_params: c_void;
}

static mut cirrus_scodec_test_cases: [kunit_case; 3] = [
    kunit_case {
        run_case: Some(cirrus_scodec_test_spkid_parse),
        generate_params: unsafe { &cirrus_scodec_test_spkid_gen_params as *const c_void },
    },
    kunit_case {
        run_case: Some(cirrus_scodec_test_no_spkid),
        generate_params: ptr::null(),
    },
    kunit_case {
        run_case: None,
        generate_params: ptr::null(),
    }, /* terminator */
];

static mut cirrus_scodec_test_suite: kunit_suite = kunit_suite {
    name: c"snd-hda-cirrus-scodec-test".as_ptr(),
    init: Some(cirrus_scodec_test_case_init),
    test_cases: unsafe { cirrus_scodec_test_cases.as_mut_ptr() },
};

// kunit_test_suite(cirrus_scodec_test_suite);
// MODULE_IMPORT_NS("SND_HDA_CIRRUS_SCODEC");
// MODULE_DESCRIPTION("KUnit test for the Cirrus side-codec library");
// MODULE_AUTHOR("Richard Fitzgerald <rf@opensource.cirrus.com>");
// MODULE_LICENSE("GPL");

unsafe extern "C" {
    fn KUNIT_ASSERT_NOT_NULL(test: *mut kunit, ptr: *const c_void);
    fn KUNIT_ASSERT_EQ(test: *mut kunit, left: c_int, right: c_int);
    fn KUNIT_EXPECT_EQ(test: *mut kunit, left: c_int, right: c_int);
    fn KUNIT_EXPECT_EQ_MSG(
        test: *mut kunit,
        left: c_int,
        right: c_int,
        fmt: *const c_char,
        ...
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
