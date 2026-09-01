// SPDX-License-Identifier: GPL-2.0-only
//
// KUnit test for the Cirrus Logic cs35l56-shared module.
//
// Copyright (C) 2026 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{copy_nonoverlapping, null, null_mut};

type u8 = u8;
type u32 = u32;

const GPIO_LINE_DIRECTION_IN: c_int = 1;
const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const EOVERFLOW: c_int = 75;
const KUNIT_PARAM_DESC_SIZE: usize = 128;
const REGCACHE_MAPLE: c_uint = 0;
const REGMAP_ENDIAN_LITTLE: c_uint = 0;
const CONFIG_GPIOLIB_REACHABLE: bool = true;
const KUNIT_SPEED_SLOW: c_uint = 1;

const CS35L56_MAX_GPIO: usize = 13;
const CS35L63_MAX_GPIO: c_uint = 20;
const CS35L56_DSP1_PMEM_5114: c_uint = 0;
const CS35L56_SYNC_GPIO1_CFG: c_uint = 0x2c08;
const CS35L56_ASP2_DIO_GPIO13_CFG: c_uint =
    CS35L56_SYNC_GPIO1_CFG + ((CS35L56_MAX_GPIO as c_uint - 1) * size_of::<u32>() as c_uint);
const CS35L56_GPIO1_CTRL1: c_uint = 0x2c74;
const CS35L56_GPIO13_CTRL1: c_uint =
    CS35L56_GPIO1_CTRL1 + ((CS35L56_MAX_GPIO as c_uint - 1) * size_of::<u32>() as c_uint);
const CS35L56_UPDATE_REGS: c_uint = 0x2c7c;
const CS35L56_GPIO_STATUS1: c_uint = 0x2c90;
const CS35L56_GPIO_DIR_MASK: c_uint = 0x8000_0000;
const CS35L56_GPIO_FN_MASK: c_uint = 0x0000_000f;
const CS35L56_GPIO_FN_GPIO: c_uint = 0x0000_0001;
const CS35L56_PAD_GPIO_IE: c_uint = 0x0000_0001;
const CS35L56_PAD_GPIO_PULL_MASK: c_uint = 0x0000_0006;
const CS35L56_PAD_PULL_NONE: c_uint = 0;
const CS35L56_UPDT_GPIO_PRES: c_uint = 0x0000_0001;

const fn BIT(n: c_uint) -> c_ulong {
    1_c_ulong << n
}

const fn GENMASK(h: c_uint, l: c_uint) -> c_ulong {
    ((!0_c_ulong) << l) & ((!0_c_ulong) >> ((c_ulong::BITS - 1) - h))
}

const fn FIELD_PREP(mask: c_uint, val: c_uint) -> c_uint {
    (val << mask.trailing_zeros()) & mask
}

const fn FIELD_GET(mask: c_uint, val: c_uint) -> c_uint {
    (val & mask) >> mask.trailing_zeros()
}

#[repr(C)]
struct kunit {
    priv_: *mut c_void,
    param_value: *const c_void,
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct faux_device {
    dev: device,
}

#[repr(C)]
struct gpio_chip {
    label: *const c_char,
    owner: *mut c_void,
    get_direction: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    direction_input: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    get: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    base: c_int,
    ngpio: c_uint,
    parent: *mut device,
}

#[repr(C)]
struct software_node {
    name: *const c_char,
    properties: *const property_entry,
    parent: *const software_node,
}

#[repr(C)]
struct software_node_ref_args {
    node: *const software_node,
    nargs: c_uint,
    args: [u64; 4],
}

#[repr(C)]
struct property_entry {
    name: *const c_char,
    value: *const c_void,
    length: usize,
}

#[repr(C)]
struct faux_device_ops {
    probe: Option<unsafe extern "C" fn(*mut faux_device) -> c_int>,
}

#[repr(C)]
struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
struct regmap_config {
    reg_bits: c_uint,
    val_bits: c_uint,
    reg_stride: c_uint,
    max_register: c_uint,
    cache_type: c_uint,
    reg_base: c_uint,
}

#[repr(C)]
struct regmap_bus {
    reg_read: Option<unsafe extern "C" fn(*mut c_void, c_uint, *mut c_uint) -> c_int>,
    reg_write: Option<unsafe extern "C" fn(*mut c_void, c_uint, c_uint) -> c_int>,
    reg_format_endian_default: c_uint,
    val_format_endian_default: c_uint,
}

#[repr(C)]
struct cs35l56_base {
    dev: *mut device,
    regmap: *mut regmap,
    type_: u8,
    rev: u8,
    onchip_spkid_gpios: [u32; 4],
    num_onchip_spkid_gpios: c_int,
    onchip_spkid_pulls: [u32; 4],
    num_onchip_spkid_pulls: c_int,
}

#[repr(C)]
struct kunit_case {
    name: *const c_char,
    run_case: Option<unsafe extern "C" fn(*mut kunit)>,
    generate_params: *const c_void,
    attr: c_uint,
}

#[repr(C)]
struct kunit_suite {
    name: *const c_char,
    init: Option<unsafe extern "C" fn(*mut kunit) -> c_int>,
    test_cases: *mut kunit_case,
}

#[repr(C)]
struct seq_buf {
    buffer: [c_char; 64],
}

#[repr(C)]
struct cs35l56_shared_test_mock_gpio {
    pin_state: c_uint,
    chip: gpio_chip,
}

#[repr(C)]
struct cs35l56_shared_test_priv {
    test: *mut kunit,
    amp_dev: *mut faux_device,
    gpio_dev: *mut faux_device,
    gpio_priv: *mut cs35l56_shared_test_mock_gpio,
    registers: *mut regmap,
    reg_offset: c_uint,
    cs35l56_base: *mut cs35l56_base,
    applied_pad_pull_state: [u8; CS35L56_MAX_GPIO],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct cs35l56_shared_test_param {
    spkid_gpios: [c_int; 4],
    spkid_pulls: [c_int; 4],
    gpio_status: c_ulong,
    spkid: c_int,
}

extern "C" {
    static THIS_MODULE: *mut c_void;
    static cs_amp_test_hooks: *mut c_void;
    static cs35l56_regmap_sdw: regmap_config;
    static cs35l56_regmap_spi: regmap_config;
    static cs35l56_regmap_i2c: regmap_config;
    static cs35l63_regmap_sdw: regmap_config;

    fn faux_device_create(
        name: *const c_char,
        parent: *mut device,
        ops: *mut faux_device_ops,
    ) -> *mut faux_device;
    fn faux_device_destroy(fdev: *mut faux_device);
    fn regmap_exit(map: *mut regmap);
    fn device_remove_software_node(dev: *mut device);
    fn device_add_software_node(dev: *mut device, node: *const software_node) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn kunit_kzalloc(test: *mut kunit, size: usize, flags: c_uint) -> *mut c_void;
    fn kunit_kcalloc(test: *mut kunit, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_add_action_or_reset(
        dev: *mut device,
        action: unsafe extern "C" fn(*mut c_void),
        data: *mut c_void,
    ) -> c_int;
    fn kunit_add_action_or_reset(
        test: *mut kunit,
        action: unsafe extern "C" fn(*mut c_void),
        data: *mut c_void,
    ) -> c_int;
    fn devm_gpiochip_add_data(
        dev: *mut device,
        chip: *mut gpio_chip,
        data: *mut c_void,
    ) -> c_int;
    fn gpiochip_get_data(chip: *mut gpio_chip) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn regmap_init(
        dev: *mut device,
        bus: *const regmap_bus,
        context: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn kunit_fail_current_test(fmt: *const c_char, ...);
    fn kunit_skip(test: *mut kunit, fmt: *const c_char, ...);
    fn kunit_activate_static_stub(
        test: *mut kunit,
        real: *mut c_void,
        replacement: *mut c_void,
    );
    fn cs_amp_get_vendor_spkid(dev: *mut device) -> c_int;
    fn cs35l56_configure_onchip_spkid_pads(base: *mut cs35l56_base) -> c_int;
    fn cs35l56_read_onchip_spkid(base: *mut cs35l56_base) -> c_int;
    fn cs35l56_check_and_save_onchip_spkid_gpios(
        base: *mut cs35l56_base,
        gpios: *mut u32,
        num_gpios: c_int,
        pulls: *mut u32,
        num_pulls: c_int,
    ) -> c_int;
    fn cs35l56_get_speaker_id(base: *mut cs35l56_base) -> c_int;
    fn seq_buf_printf(buf: *mut seq_buf, fmt: *const c_char, ...);
    fn seq_buf_str(buf: *const seq_buf) -> *const c_char;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
}

unsafe extern "C" fn faux_device_destroy_wrapper(data: *mut c_void) {
    faux_device_destroy(data as *mut faux_device);
}

unsafe extern "C" fn regmap_exit_wrapper(data: *mut c_void) {
    regmap_exit(data as *mut regmap);
}

unsafe extern "C" fn device_remove_software_node_wrapper(data: *mut c_void) {
    device_remove_software_node(data as *mut device);
}

unsafe extern "C" fn cs35l56_shared_test_mock_gpio_get_direction(
    _chip: *mut gpio_chip,
    _offset: c_uint,
) -> c_int {
    GPIO_LINE_DIRECTION_IN
}

unsafe extern "C" fn cs35l56_shared_test_mock_gpio_direction_in(
    _chip: *mut gpio_chip,
    _offset: c_uint,
) -> c_int {
    0
}

unsafe extern "C" fn cs35l56_shared_test_mock_gpio_get(
    chip: *mut gpio_chip,
    offset: c_uint,
) -> c_int {
    let gpio_priv = gpiochip_get_data(chip) as *mut cs35l56_shared_test_mock_gpio;

    (((*gpio_priv).pin_state as c_ulong & BIT(offset)) != 0) as c_int
}

static mut cs35l56_shared_test_mock_gpio_chip: gpio_chip = gpio_chip {
    label: b"cs35l56_shared_test_mock_gpio\0".as_ptr() as *const c_char,
    owner: null_mut(),
    get_direction: Some(cs35l56_shared_test_mock_gpio_get_direction),
    direction_input: Some(cs35l56_shared_test_mock_gpio_direction_in),
    get: Some(cs35l56_shared_test_mock_gpio_get),
    base: -1,
    ngpio: 32,
    parent: null_mut(),
};

/* software_node referencing the gpio driver */
static cs35l56_shared_test_mock_gpio_swnode: software_node = software_node {
    name: b"cs35l56_shared_test_mock_gpio\0".as_ptr() as *const c_char,
    properties: null(),
    parent: null(),
};

unsafe extern "C" fn cs35l56_shared_test_mock_gpio_probe(fdev: *mut faux_device) -> c_int {
    let gpio_priv: *mut cs35l56_shared_test_mock_gpio;
    let dev = &mut (*fdev).dev as *mut device;
    let mut ret: c_int;

    gpio_priv = devm_kzalloc(dev, size_of::<cs35l56_shared_test_mock_gpio>(), GFP_KERNEL)
        as *mut cs35l56_shared_test_mock_gpio;
    if gpio_priv.is_null() {
        return -ENOMEM;
    }

    ret = device_add_software_node(dev, &cs35l56_shared_test_mock_gpio_swnode);
    if ret != 0 {
        return ret;
    }

    ret = devm_add_action_or_reset(
        dev,
        device_remove_software_node_wrapper,
        dev as *mut c_void,
    );
    if ret != 0 {
        return ret;
    }

    /* GPIO core modifies our struct gpio_chip so use a copy */
    (*gpio_priv).chip = cs35l56_shared_test_mock_gpio_chip;
    (*gpio_priv).chip.parent = dev;
    ret = devm_gpiochip_add_data(
        dev,
        &mut (*gpio_priv).chip,
        gpio_priv as *mut c_void,
    );
    if ret != 0 {
        return dev_err_probe(dev, ret, b"Failed to add gpiochip\n\0".as_ptr() as *const c_char);
    }

    dev_set_drvdata(dev, gpio_priv as *mut c_void);

    0
}

static mut cs35l56_shared_test_mock_gpio_drv: faux_device_ops = faux_device_ops {
    probe: Some(cs35l56_shared_test_mock_gpio_probe),
};

unsafe fn _cs35l56_shared_test_create_dummy_gpio(test: *mut kunit) {
    let priv_ = (*test).priv_ as *mut cs35l56_shared_test_priv;

    (*priv_).gpio_dev = faux_device_create(
        b"cs35l56_shared_test_mock_gpio\0".as_ptr() as *const c_char,
        null_mut(),
        &mut cs35l56_shared_test_mock_gpio_drv,
    );
    KUNIT_ASSERT_NOT_NULL(test, (*priv_).gpio_dev as *const c_void);
    KUNIT_ASSERT_EQ(
        test,
        0,
        kunit_add_action_or_reset(
            test,
            faux_device_destroy_wrapper,
            (*priv_).gpio_dev as *mut c_void,
        ),
    );

    (*priv_).gpio_priv = dev_get_drvdata(&mut (*(*priv_).gpio_dev).dev) as *mut _;
    KUNIT_ASSERT_NOT_NULL(test, (*priv_).gpio_priv as *const c_void);
}

static cs35l56_shared_test_mock_registers_regmap: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 32,
    reg_stride: 4,
    max_register: CS35L56_DSP1_PMEM_5114,
    cache_type: REGCACHE_MAPLE,
    reg_base: 0,
};

static cs35l56_shared_test_mock_registers_regmap_bus: regmap_bus = regmap_bus {
    /* No handlers because it is always in cache-only */
    reg_read: None,
    reg_write: None,
    reg_format_endian_default: 0,
    val_format_endian_default: 0,
};

unsafe fn cs35l56_shared_test_read_gpio_status(
    priv_: *mut cs35l56_shared_test_priv,
) -> c_uint {
    let param = (*(*priv_).test).param_value as *const cs35l56_shared_test_param;
    let mut reg_offs: c_uint;
    let mut pad_cfg: c_uint = 0;
    let mut val: c_uint = 0;
    let mut status: c_uint = 0;
    let mut mask: c_uint = 1;

    reg_offs = 0;
    while reg_offs < (CS35L56_MAX_GPIO * size_of::<u32>()) as c_uint {
        regmap_read(
            (*priv_).registers,
            CS35L56_SYNC_GPIO1_CFG + reg_offs,
            &mut pad_cfg,
        );
        regmap_read(
            (*priv_).registers,
            CS35L56_GPIO1_CTRL1 + reg_offs,
            &mut val,
        );

        /* Only read a value if set as an input pin and as a GPIO */
        val &= CS35L56_GPIO_DIR_MASK | CS35L56_GPIO_FN_MASK;
        if (pad_cfg & CS35L56_PAD_GPIO_IE) != 0
            && val == (CS35L56_GPIO_DIR_MASK | CS35L56_GPIO_FN_GPIO)
        {
            status |= ((*param).gpio_status as c_uint) & mask;
        }

        mask <<= 1;
        reg_offs += size_of::<u32>() as c_uint;
    }

    status
}

unsafe fn cs35l56_shared_test_updt_gpio_pres(
    priv_: *mut cs35l56_shared_test_priv,
    mut reg: c_uint,
    mut val: c_uint,
) -> c_int {
    let mut i: usize;
    let ret: c_int;

    ret = regmap_write((*priv_).registers, reg, val);
    if ret != 0 {
        return ret;
    }

    if (val & CS35L56_UPDT_GPIO_PRES) != 0 {
        /* Simulate transferring register state to internal latches */
        i = 0;
        while i < (*priv_).applied_pad_pull_state.len() {
            reg = CS35L56_SYNC_GPIO1_CFG + (i * size_of::<u32>()) as c_uint;
            regmap_read((*priv_).registers, reg, &mut val);
            val = FIELD_GET(CS35L56_PAD_GPIO_PULL_MASK, val);
            (*priv_).applied_pad_pull_state[i] = val as u8;
            i += 1;
        }
    }

    0
}

unsafe extern "C" fn cs35l56_shared_test_reg_read(
    context: *mut c_void,
    mut reg: c_uint,
    val: *mut c_uint,
) -> c_int {
    let priv_ = context as *mut cs35l56_shared_test_priv;

    reg = reg.wrapping_sub((*priv_).reg_offset);

    match reg {
        CS35L56_SYNC_GPIO1_CFG..=CS35L56_ASP2_DIO_GPIO13_CFG
        | CS35L56_GPIO1_CTRL1..=CS35L56_GPIO13_CTRL1 => {
            regmap_read((*priv_).registers, reg, val)
        }
        CS35L56_UPDATE_REGS => {
            *val = 0;
            0
        }
        CS35L56_GPIO_STATUS1 => {
            *val = cs35l56_shared_test_read_gpio_status(priv_);
            0
        }
        _ => {
            kunit_fail_current_test(b"Bad regmap read address %#x\n\0".as_ptr() as *const c_char, reg);
            -EINVAL
        }
    }
}

unsafe extern "C" fn cs35l56_shared_test_reg_write(
    context: *mut c_void,
    mut reg: c_uint,
    val: c_uint,
) -> c_int {
    let priv_ = context as *mut cs35l56_shared_test_priv;

    reg = reg.wrapping_sub((*priv_).reg_offset);

    match reg {
        CS35L56_UPDATE_REGS => cs35l56_shared_test_updt_gpio_pres(priv_, reg, val),
        CS35L56_SYNC_GPIO1_CFG..=CS35L56_ASP2_DIO_GPIO13_CFG
        | CS35L56_GPIO1_CTRL1..=CS35L56_GPIO13_CTRL1 => {
            regmap_write((*priv_).registers, reg, val)
        }
        _ => {
            kunit_fail_current_test(b"Bad regmap write address %#x\n\0".as_ptr() as *const c_char, reg);
            -EINVAL
        }
    }
}

static cs35l56_shared_test_regmap_bus: regmap_bus = regmap_bus {
    reg_read: Some(cs35l56_shared_test_reg_read),
    reg_write: Some(cs35l56_shared_test_reg_write),
    reg_format_endian_default: REGMAP_ENDIAN_LITTLE,
    val_format_endian_default: REGMAP_ENDIAN_LITTLE,
};

unsafe fn KUNIT_ASSERT_NOT_NULL(_test: *mut kunit, ptr: *const c_void) {
    if ptr.is_null() {
        kunit_fail_current_test(b"assertion failed: pointer is NULL\n\0".as_ptr() as *const c_char);
    }
}

unsafe fn KUNIT_ASSERT_NOT_ERR_OR_NULL(test: *mut kunit, ptr: *const c_void) {
    KUNIT_ASSERT_NOT_NULL(test, ptr);
}

unsafe fn KUNIT_ASSERT_EQ(test: *mut kunit, left: c_int, right: c_int) {
    if left != right {
        kunit_fail_current_test(
            b"assertion failed: %d != %d\n\0".as_ptr() as *const c_char,
            left,
            right,
        );
    }
}

unsafe fn KUNIT_EXPECT_EQ(test: *mut kunit, left: c_int, right: c_int) {
    KUNIT_ASSERT_EQ(test, left, right);
}

unsafe fn KUNIT_EXPECT_LE(_test: *mut kunit, left: c_int, right: c_int) {
    if left > right {
        kunit_fail_current_test(
            b"expectation failed: %d > %d\n\0".as_ptr() as *const c_char,
            left,
            right,
        );
    }
}

unsafe fn KUNIT_ASSERT_LE(_test: *mut kunit, left: c_int, right: c_int) {
    if left > right {
        kunit_fail_current_test(
            b"assertion failed: %d > %d\n\0".as_ptr() as *const c_char,
            left,
            right,
        );
    }
}

/*
 * Self-test that the mock GPIO registers obey the configuration bits.
 * Other tests rely on the mocked registers only returning a GPIO state
 * if the pin is correctly set as a GPIO input.
 */
unsafe extern "C" fn cs35l56_shared_test_mock_gpio_status_selftest(test: *mut kunit) {
    let param = (*test).param_value as *const cs35l56_shared_test_param;
    let priv_ = (*test).priv_ as *mut cs35l56_shared_test_priv;
    let cs35l56_base = (*priv_).cs35l56_base;
    let mut reg: c_uint;
    let mut val: c_uint = 0;

    KUNIT_ASSERT_NOT_NULL(test, param as *const c_void);

    /* Set all pins non-GPIO and output. Mock GPIO_STATUS should read 0 */
    reg = CS35L56_GPIO1_CTRL1;
    while reg <= CS35L56_GPIO13_CTRL1 {
        KUNIT_ASSERT_EQ(test, 0, regmap_write((*priv_).registers, reg, 0));
        reg += size_of::<u32>() as c_uint;
    }

    /* Set all pads as inputs */
    reg = CS35L56_SYNC_GPIO1_CFG;
    while reg <= CS35L56_ASP2_DIO_GPIO13_CFG {
        KUNIT_ASSERT_EQ(test, 0, regmap_write((*priv_).registers, reg, CS35L56_PAD_GPIO_IE));
        reg += size_of::<u32>() as c_uint;
    }

    KUNIT_ASSERT_EQ(test, 0, regmap_read((*cs35l56_base).regmap, CS35L56_GPIO_STATUS1, &mut val));
    KUNIT_EXPECT_EQ(test, val as c_int, 0);

    /* Set all pins as GPIO outputs. Mock GPIO_STATUS should read 0 */
    reg = CS35L56_GPIO1_CTRL1;
    while reg <= CS35L56_GPIO13_CTRL1 {
        KUNIT_ASSERT_EQ(test, 0, regmap_write((*priv_).registers, reg, CS35L56_GPIO_FN_GPIO));
        reg += size_of::<u32>() as c_uint;
    }

    KUNIT_ASSERT_EQ(test, 0, regmap_read((*cs35l56_base).regmap, CS35L56_GPIO_STATUS1, &mut val));
    KUNIT_EXPECT_EQ(test, val as c_int, 0);

    /* Set all pins as non-GPIO inputs. Mock GPIO_STATUS should read 0 */
    reg = CS35L56_GPIO1_CTRL1;
    while reg <= CS35L56_GPIO13_CTRL1 {
        KUNIT_ASSERT_EQ(test, 0, regmap_write((*priv_).registers, reg, CS35L56_GPIO_DIR_MASK));
        reg += size_of::<u32>() as c_uint;
    }

    KUNIT_ASSERT_EQ(test, 0, regmap_read((*cs35l56_base).regmap, CS35L56_GPIO_STATUS1, &mut val));
    KUNIT_EXPECT_EQ(test, val as c_int, 0);

    /* Set all pins as GPIO inputs. Mock GPIO_STATUS should match param->gpio_status */
    reg = CS35L56_GPIO1_CTRL1;
    while reg <= CS35L56_GPIO13_CTRL1 {
        KUNIT_ASSERT_EQ(
            test,
            0,
            regmap_write(
                (*priv_).registers,
                reg,
                CS35L56_GPIO_DIR_MASK | CS35L56_GPIO_FN_GPIO,
            ),
        );
        reg += size_of::<u32>() as c_uint;
    }

    KUNIT_ASSERT_EQ(test, 0, regmap_read((*cs35l56_base).regmap, CS35L56_GPIO_STATUS1, &mut val));
    KUNIT_EXPECT_EQ(test, val as c_int, (*param).gpio_status as c_int);

    /* Set all pads as outputs. Mock GPIO_STATUS should read 0 */
    reg = CS35L56_SYNC_GPIO1_CFG;
    while reg <= CS35L56_ASP2_DIO_GPIO13_CFG {
        KUNIT_ASSERT_EQ(test, 0, regmap_write((*priv_).registers, reg, 0));
        reg += size_of::<u32>() as c_uint;
    }

    KUNIT_ASSERT_EQ(test, 0, regmap_read((*cs35l56_base).regmap, CS35L56_GPIO_STATUS1, &mut val));
    KUNIT_EXPECT_EQ(test, val as c_int, 0);
}

/* Test that the listed chip pins are assembled into a speaker ID integer. */
unsafe extern "C" fn cs35l56_shared_test_get_onchip_speaker_id(test: *mut kunit) {
    let param = (*test).param_value as *const cs35l56_shared_test_param;
    let priv_ = (*test).priv_ as *mut cs35l56_shared_test_priv;
    let cs35l56_base = (*priv_).cs35l56_base;
    let mut i: usize;
    let mut reg: c_uint;

    /* Set all pins non-GPIO and output */
    reg = CS35L56_GPIO1_CTRL1;
    while reg <= CS35L56_GPIO13_CTRL1 {
        KUNIT_ASSERT_EQ(test, 0, regmap_write((*priv_).registers, reg, 0));
        reg += size_of::<u32>() as c_uint;
    }

    reg = CS35L56_SYNC_GPIO1_CFG;
    while reg <= CS35L56_ASP2_DIO_GPIO13_CFG {
        KUNIT_ASSERT_EQ(test, 0, regmap_write((*priv_).registers, reg, 0));
        reg += size_of::<u32>() as c_uint;
    }

    /* Init GPIO array */
    i = 0;
    while i < (*param).spkid_gpios.len() {
        if (*param).spkid_gpios[i] < 0 {
            break;
        }

        (*cs35l56_base).onchip_spkid_gpios[i] = ((*param).spkid_gpios[i] - 1) as u32;
        (*cs35l56_base).num_onchip_spkid_gpios += 1;
        i += 1;
    }

    (*cs35l56_base).num_onchip_spkid_pulls = 0;

    KUNIT_EXPECT_EQ(test, cs35l56_configure_onchip_spkid_pads(cs35l56_base), 0);
    KUNIT_EXPECT_EQ(test, cs35l56_read_onchip_spkid(cs35l56_base), (*param).spkid);
}

/* Test that the listed chip pins and the corresponding pads are configured correctly. */
unsafe extern "C" fn cs35l56_shared_test_onchip_speaker_id_pad_config(test: *mut kunit) {
    let param = (*test).param_value as *const cs35l56_shared_test_param;
    let priv_ = (*test).priv_ as *mut cs35l56_shared_test_priv;
    let cs35l56_base = (*priv_).cs35l56_base;
    let mut i: usize;
    let mut reg: c_uint;
    let mut val: c_uint = 0;

    /* Init values in all pin registers */
    reg = CS35L56_GPIO1_CTRL1;
    while reg <= CS35L56_GPIO13_CTRL1 {
        KUNIT_ASSERT_EQ(test, 0, regmap_write((*priv_).registers, reg, 0));
        reg += size_of::<u32>() as c_uint;
    }

    reg = CS35L56_SYNC_GPIO1_CFG;
    while reg <= CS35L56_ASP2_DIO_GPIO13_CFG {
        KUNIT_ASSERT_EQ(test, 0, regmap_write((*priv_).registers, reg, 0));
        reg += size_of::<u32>() as c_uint;
    }

    /* Init GPIO array */
    i = 0;
    while i < (*param).spkid_gpios.len() {
        if (*param).spkid_gpios[i] < 0 {
            break;
        }

        (*cs35l56_base).onchip_spkid_gpios[i] = ((*param).spkid_gpios[i] - 1) as u32;
        (*cs35l56_base).num_onchip_spkid_gpios += 1;
        i += 1;
    }

    /* Init pulls array */
    i = 0;
    while i < (*param).spkid_pulls.len() {
        if (*param).spkid_pulls[i] < 0 {
            break;
        }

        (*cs35l56_base).onchip_spkid_pulls[i] = (*param).spkid_pulls[i] as u32;
        (*cs35l56_base).num_onchip_spkid_pulls += 1;
        i += 1;
    }

    KUNIT_EXPECT_EQ(test, cs35l56_configure_onchip_spkid_pads(cs35l56_base), 0);

    i = 0;
    while i < (*param).spkid_gpios.len() {
        if (*param).spkid_gpios[i] < 0 {
            break;
        }

        /* Pad should be an input */
        reg = CS35L56_SYNC_GPIO1_CFG
            + (((*param).spkid_gpios[i] - 1) as usize * size_of::<u32>()) as c_uint;
        KUNIT_EXPECT_EQ(test, regmap_read((*priv_).registers, reg, &mut val), 0);
        KUNIT_EXPECT_EQ(
            test,
            (val & CS35L56_PAD_GPIO_IE) as c_int,
            CS35L56_PAD_GPIO_IE as c_int,
        );

        /* Specified pulls should be set, others should be none */
        if (i as c_int) < (*cs35l56_base).num_onchip_spkid_pulls {
            KUNIT_EXPECT_EQ(
                test,
                (val & CS35L56_PAD_GPIO_PULL_MASK) as c_int,
                FIELD_PREP(CS35L56_PAD_GPIO_PULL_MASK, (*param).spkid_pulls[i] as c_uint) as c_int,
            );
        } else {
            KUNIT_EXPECT_EQ(
                test,
                (val & CS35L56_PAD_GPIO_PULL_MASK) as c_int,
                CS35L56_PAD_PULL_NONE as c_int,
            );
        }

        /* Pulls for all specfied GPIOs should have been transferred to AO latch */
        if (i as c_int) < (*cs35l56_base).num_onchip_spkid_pulls {
            KUNIT_EXPECT_EQ(
                test,
                (*priv_).applied_pad_pull_state[((*param).spkid_gpios[i] - 1) as usize] as c_int,
                (*param).spkid_pulls[i],
            );
        } else {
            KUNIT_EXPECT_EQ(
                test,
                (*priv_).applied_pad_pull_state[((*param).spkid_gpios[i] - 1) as usize] as c_int,
                CS35L56_PAD_PULL_NONE as c_int,
            );
        }
        i += 1;
    }
}

/* Test that the listed chip pins are stashed correctly. */
unsafe extern "C" fn cs35l56_shared_test_stash_onchip_spkid_pins(test: *mut kunit) {
    let param = (*test).param_value as *const cs35l56_shared_test_param;
    let priv_ = (*test).priv_ as *mut cs35l56_shared_test_priv;
    let cs35l56_base = (*priv_).cs35l56_base;
    let mut gpios: [u32; 5] = [0; 5];
    let mut pulls: [u32; 5] = [0; 5];
    let mut i: c_int;
    let mut num_gpios: c_int;
    let mut num_pulls: c_int;

    const _: () = assert!(5 >= 4);
    const _: () = assert!(5 >= 4);

    num_gpios = 0;
    i = 0;
    while (i as usize) < (*param).spkid_gpios.len() {
        if (*param).spkid_gpios[i as usize] < 0 {
            break;
        }

        gpios[i as usize] = (*param).spkid_gpios[i as usize] as u32;
        num_gpios += 1;
        i += 1;
    }

    num_pulls = 0;
    i = 0;
    while (i as usize) < (*param).spkid_pulls.len() {
        if (*param).spkid_pulls[i as usize] < 0 {
            break;
        }

        pulls[i as usize] = (*param).spkid_pulls[i as usize] as u32;
        num_pulls += 1;
        i += 1;
    }

    (*cs35l56_base).num_onchip_spkid_gpios = 0;
    (*cs35l56_base).num_onchip_spkid_pulls = 0;

    KUNIT_ASSERT_LE(test, num_gpios, (*cs35l56_base).onchip_spkid_gpios.len() as c_int);
    KUNIT_ASSERT_LE(test, num_pulls, (*cs35l56_base).onchip_spkid_pulls.len() as c_int);

    KUNIT_EXPECT_EQ(
        test,
        cs35l56_check_and_save_onchip_spkid_gpios(
            cs35l56_base,
            gpios.as_mut_ptr(),
            num_gpios,
            pulls.as_mut_ptr(),
            num_pulls,
        ),
        0,
    );

    KUNIT_EXPECT_EQ(test, (*cs35l56_base).num_onchip_spkid_gpios, num_gpios);
    KUNIT_EXPECT_EQ(test, (*cs35l56_base).num_onchip_spkid_pulls, num_pulls);

    /* GPIO numbers are adjusted from 1-based to 0-based */
    i = 0;
    while i < num_gpios {
        KUNIT_EXPECT_EQ(
            test,
            (*cs35l56_base).onchip_spkid_gpios[i as usize] as c_int,
            gpios[i as usize] as c_int - 1,
        );
        i += 1;
    }

    i = 0;
    while i < num_pulls {
        KUNIT_EXPECT_EQ(
            test,
            (*cs35l56_base).onchip_spkid_pulls[i as usize] as c_int,
            pulls[i as usize] as c_int,
        );
        i += 1;
    }
}

/* Test that illegal GPIO numbers are rejected. */
unsafe extern "C" fn cs35l56_shared_test_stash_onchip_spkid_pins_reject_invalid(test: *mut kunit) {
    let priv_ = (*test).priv_ as *mut cs35l56_shared_test_priv;
    let cs35l56_base = (*priv_).cs35l56_base;
    let mut gpios: [u32; 8] = [0; 8];
    let mut pulls: [u32; 8] = [0; 8];

    KUNIT_EXPECT_LE(
        test,
        cs35l56_check_and_save_onchip_spkid_gpios(
            cs35l56_base,
            gpios.as_mut_ptr(),
            1,
            pulls.as_mut_ptr(),
            0,
        ),
        0,
    );

    match (*cs35l56_base).type_ {
        0x54 | 0x56 | 0x57 => {
            gpios[0] = CS35L56_MAX_GPIO as u32 + 1;
        }
        0x63 => {
            gpios[0] = CS35L63_MAX_GPIO + 1;
        }
        _ => {
            kunit_fail_current_test(
                b"Unsupported type:%#x\n\0".as_ptr() as *const c_char,
                (*cs35l56_base).type_ as c_uint,
            );
            return;
        }
    }
    KUNIT_EXPECT_LE(
        test,
        cs35l56_check_and_save_onchip_spkid_gpios(
            cs35l56_base,
            gpios.as_mut_ptr(),
            1,
            pulls.as_mut_ptr(),
            0,
        ),
        0,
    );

    gpios[0] = 1;
    pulls[0] = 3;
    KUNIT_EXPECT_LE(
        test,
        cs35l56_check_and_save_onchip_spkid_gpios(
            cs35l56_base,
            gpios.as_mut_ptr(),
            1,
            pulls.as_mut_ptr(),
            1,
        ),
        0,
    );

    const _: () = assert!(8 > 4);
    const _: () = assert!(8 > 4);
    KUNIT_EXPECT_EQ(
        test,
        cs35l56_check_and_save_onchip_spkid_gpios(
            cs35l56_base,
            gpios.as_mut_ptr(),
            gpios.len() as c_int,
            pulls.as_mut_ptr(),
            0,
        ),
        -EOVERFLOW,
    );
    KUNIT_EXPECT_EQ(
        test,
        cs35l56_check_and_save_onchip_spkid_gpios(
            cs35l56_base,
            gpios.as_mut_ptr(),
            1,
            pulls.as_mut_ptr(),
            pulls.len() as c_int,
        ),
        -EOVERFLOW,
    );
}

unsafe extern "C" fn cs35l56_shared_test_onchip_speaker_id_not_defined(test: *mut kunit) {
    let priv_ = (*test).priv_ as *mut cs35l56_shared_test_priv;
    let cs35l56_base = (*priv_).cs35l56_base;

    (*cs35l56_base).onchip_spkid_gpios = [0; 4];
    (*cs35l56_base).onchip_spkid_pulls = [0; 4];
    (*cs35l56_base).num_onchip_spkid_gpios = 0;
    (*cs35l56_base).num_onchip_spkid_pulls = 0;
    KUNIT_EXPECT_EQ(test, cs35l56_configure_onchip_spkid_pads(cs35l56_base), 0);
    KUNIT_EXPECT_EQ(test, cs35l56_read_onchip_spkid(cs35l56_base), -ENOENT);
}

/* simulate cs_amp_get_vendor_spkid() reading a vendor-specific ID of 1 */
unsafe extern "C" fn cs35l56_shared_test_get_vendor_spkid_1(_dev: *mut device) -> c_int {
    1
}

unsafe extern "C" fn cs35l56_shared_test_get_speaker_id_vendor(test: *mut kunit) {
    let priv_ = (*test).priv_ as *mut cs35l56_shared_test_priv;

    /* Hook cs_amp_get_vendor_spkid() to return an ID of 1 */
    kunit_activate_static_stub(
        test,
        cs_amp_get_vendor_spkid as *mut c_void,
        cs35l56_shared_test_get_vendor_spkid_1 as *mut c_void,
    );

    KUNIT_EXPECT_EQ(test, cs35l56_get_speaker_id((*priv_).cs35l56_base), 1);
}

unsafe extern "C" fn cs35l56_shared_test_get_speaker_id_property(test: *mut kunit) {
    let priv_ = (*test).priv_ as *mut cs35l56_shared_test_priv;
    let dev_props = [
        property_entry {
            name: b"cirrus,speaker-id\0".as_ptr() as *const c_char,
            value: 2usize as *const c_void,
            length: size_of::<u32>(),
        },
        property_entry {
            name: null(),
            value: null(),
            length: 0,
        },
    ];
    let dev_node = software_node {
        name: b"SPK1\0".as_ptr() as *const c_char,
        properties: dev_props.as_ptr(),
        parent: null(),
    };

    KUNIT_ASSERT_EQ(
        test,
        device_add_software_node((*(*priv_).cs35l56_base).dev, &dev_node),
        0,
    );
    KUNIT_ASSERT_EQ(
        test,
        0,
        kunit_add_action_or_reset(
            test,
            device_remove_software_node_wrapper,
            (*(*priv_).cs35l56_base).dev as *mut c_void,
        ),
    );

    KUNIT_EXPECT_EQ(test, cs35l56_get_speaker_id((*priv_).cs35l56_base), 2);
}

/*
 * Create software nodes equivalent to ACPI structure
 *
 * Device(GSPK) {
 *	Name(_DSD, ...) {
 *	Package() {
 *		cs-gpios {
 *			GPIO, n, 0,
 *			...
 *		}
 *	}
 */
unsafe fn _cs35l56_shared_test_create_spkid_swnode(
    test: *mut kunit,
    dev: *mut device,
    args: *const software_node_ref_args,
    num_args: c_int,
) {
    let priv_ = (*test).priv_ as *mut cs35l56_shared_test_priv;
    let props_template = [
        property_entry {
            name: b"spk-id-gpios\0".as_ptr() as *const c_char,
            value: args as *const c_void,
            length: num_args as usize * size_of::<software_node_ref_args>(),
        },
        property_entry {
            name: null(),
            value: null(),
            length: 0,
        },
    ];
    let props: *mut property_entry;
    let node: *mut software_node;

    props = kunit_kzalloc(test, size_of::<[property_entry; 2]>(), GFP_KERNEL) as *mut property_entry;
    KUNIT_ASSERT_NOT_NULL(test, props as *const c_void);
    copy_nonoverlapping(props_template.as_ptr(), props, props_template.len());

    node = kunit_kzalloc(test, size_of::<software_node>(), GFP_KERNEL) as *mut software_node;
    KUNIT_ASSERT_NOT_NULL(test, node as *const c_void);
    *node = software_node {
        name: b"GSPK\0".as_ptr() as *const c_char,
        properties: props,
        parent: null(),
    };

    KUNIT_ASSERT_EQ(test, device_add_software_node(dev, node), 0);
    KUNIT_ASSERT_EQ(
        test,
        0,
        kunit_add_action_or_reset(
            test,
            device_remove_software_node_wrapper,
            (*(*priv_).cs35l56_base).dev as *mut c_void,
        ),
    );
}

unsafe extern "C" fn cs35l56_shared_test_get_speaker_id_from_host_gpio(test: *mut kunit) {
    let param = (*test).param_value as *const cs35l56_shared_test_param;
    let priv_ = (*test).priv_ as *mut cs35l56_shared_test_priv;
    let cs35l56_base = (*priv_).cs35l56_base;
    let ref_: *mut software_node_ref_args;
    let mut i: c_int;

    if !CONFIG_GPIOLIB_REACHABLE {
        kunit_skip(test, b"Requires CONFIG_GPIOLIB\0".as_ptr() as *const c_char);
        return;
    }

    _cs35l56_shared_test_create_dummy_gpio(test);

    ref_ = kunit_kcalloc(
        test,
        (*param).spkid_gpios.len(),
        size_of::<software_node_ref_args>(),
        GFP_KERNEL,
    ) as *mut software_node_ref_args;
    KUNIT_ASSERT_NOT_NULL(test, ref_ as *const c_void);

    i = 0;
    while (*param).spkid_gpios[i as usize] >= 0 {
        *ref_.add(i as usize) = software_node_ref_args {
            node: &cs35l56_shared_test_mock_gpio_swnode,
            nargs: 2,
            args: [(*param).spkid_gpios[i as usize] as u64, 0, 0, 0],
        };
        i += 1;
    }
    _cs35l56_shared_test_create_spkid_swnode(test, (*cs35l56_base).dev, ref_, i);

    (*(*priv_).gpio_priv).pin_state = (*param).gpio_status as c_uint;
    KUNIT_EXPECT_EQ(
        test,
        cs35l56_get_speaker_id((*priv_).cs35l56_base),
        (*param).spkid,
    );
}

unsafe fn cs35l56_shared_test_case_regmap_init(
    test: *mut kunit,
    regmap_config: *const regmap_config,
) -> c_int {
    let priv_ = (*test).priv_ as *mut cs35l56_shared_test_priv;
    let cs35l56_base: *mut cs35l56_base;

    /*
     * Create a dummy regmap to simulate a register map by holding the
     * values of all simulated registers in the regmap cache.
     */
    (*priv_).registers = regmap_init(
        &mut (*(*priv_).amp_dev).dev,
        &cs35l56_shared_test_mock_registers_regmap_bus,
        priv_ as *mut c_void,
        &cs35l56_shared_test_mock_registers_regmap,
    );
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, (*priv_).registers as *const c_void);
    KUNIT_ASSERT_EQ(
        test,
        0,
        kunit_add_action_or_reset(test, regmap_exit_wrapper, (*priv_).registers as *mut c_void),
    );
    regcache_cache_only((*priv_).registers, true);

    /* Create dummy regmap for cs35l56 driver */
    cs35l56_base = (*priv_).cs35l56_base;
    (*cs35l56_base).regmap = regmap_init(
        (*cs35l56_base).dev,
        &cs35l56_shared_test_regmap_bus,
        priv_ as *mut c_void,
        regmap_config,
    );
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, (*cs35l56_base).regmap as *const c_void);
    KUNIT_ASSERT_EQ(
        test,
        0,
        kunit_add_action_or_reset(test, regmap_exit_wrapper, (*cs35l56_base).regmap as *mut c_void),
    );

    0
}

unsafe fn cs35l56_shared_test_case_base_init(
    test: *mut kunit,
    type_: u8,
    rev: u8,
    regmap_config: *const regmap_config,
) -> c_int {
    let priv_: *mut cs35l56_shared_test_priv;
    let ret: c_int;

    KUNIT_ASSERT_NOT_NULL(test, cs_amp_test_hooks as *const c_void);

    priv_ = kunit_kzalloc(test, size_of::<cs35l56_shared_test_priv>(), GFP_KERNEL)
        as *mut cs35l56_shared_test_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    (*test).priv_ = priv_ as *mut c_void;
    (*priv_).test = test;

    /* Create dummy amp driver dev */
    (*priv_).amp_dev =
        faux_device_create(b"cs35l56_shared_test_drv\0".as_ptr() as *const c_char, null_mut(), null_mut());
    KUNIT_ASSERT_NOT_NULL(test, (*priv_).amp_dev as *const c_void);
    KUNIT_ASSERT_EQ(
        test,
        0,
        kunit_add_action_or_reset(
            test,
            faux_device_destroy_wrapper,
            (*priv_).amp_dev as *mut c_void,
        ),
    );

    (*priv_).cs35l56_base =
        kunit_kzalloc(test, size_of::<cs35l56_base>(), GFP_KERNEL) as *mut cs35l56_base;
    KUNIT_ASSERT_NOT_NULL(test, (*priv_).cs35l56_base as *const c_void);
    (*(*priv_).cs35l56_base).dev = &mut (*(*priv_).amp_dev).dev;
    (*(*priv_).cs35l56_base).type_ = type_;
    (*(*priv_).cs35l56_base).rev = rev;

    if !regmap_config.is_null() {
        (*priv_).reg_offset = (*regmap_config).reg_base;
        ret = cs35l56_shared_test_case_regmap_init(test, regmap_config);
        if ret != 0 {
            return ret;
        }
    }

    0
}

unsafe extern "C" fn cs35l56_shared_test_case_regmap_init_L56_B0_sdw(test: *mut kunit) -> c_int {
    cs35l56_shared_test_case_base_init(test, 0x56, 0xb0, &cs35l56_regmap_sdw)
}

unsafe extern "C" fn cs35l56_shared_test_case_regmap_init_L56_B0_spi(test: *mut kunit) -> c_int {
    cs35l56_shared_test_case_base_init(test, 0x56, 0xb0, &cs35l56_regmap_spi)
}

unsafe extern "C" fn cs35l56_shared_test_case_regmap_init_L56_B0_i2c(test: *mut kunit) -> c_int {
    cs35l56_shared_test_case_base_init(test, 0x56, 0xb0, &cs35l56_regmap_i2c)
}

unsafe extern "C" fn cs35l56_shared_test_case_regmap_init_L56_B2_sdw(test: *mut kunit) -> c_int {
    cs35l56_shared_test_case_base_init(test, 0x56, 0xb2, &cs35l56_regmap_sdw)
}

unsafe extern "C" fn cs35l56_shared_test_case_regmap_init_L56_B2_spi(test: *mut kunit) -> c_int {
    cs35l56_shared_test_case_base_init(test, 0x56, 0xb2, &cs35l56_regmap_spi)
}

unsafe extern "C" fn cs35l56_shared_test_case_regmap_init_L56_B2_i2c(test: *mut kunit) -> c_int {
    cs35l56_shared_test_case_base_init(test, 0x56, 0xb2, &cs35l56_regmap_i2c)
}

unsafe extern "C" fn cs35l56_shared_test_case_regmap_init_L63_A1_sdw(test: *mut kunit) -> c_int {
    cs35l56_shared_test_case_base_init(test, 0x63, 0xa1, &cs35l63_regmap_sdw)
}

unsafe extern "C" fn cs35l56_shared_test_gpio_param_desc(
    param: *const cs35l56_shared_test_param,
    desc: *mut c_char,
) {
    let mut gpios: seq_buf = zeroed();
    let mut pulls: seq_buf = zeroed();
    let mut i: usize;

    i = 0;
    while i < (*param).spkid_gpios.len() {
        if (*param).spkid_gpios[i] < 0 {
            break;
        }

        seq_buf_printf(
            &mut gpios,
            b"%s%d\0".as_ptr() as *const c_char,
            if i == 0 {
                b"\0".as_ptr() as *const c_char
            } else {
                b",\0".as_ptr() as *const c_char
            },
            (*param).spkid_gpios[i],
        );
        i += 1;
    }

    i = 0;
    while i < (*param).spkid_pulls.len() {
        if (*param).spkid_pulls[i] < 0 {
            break;
        }

        seq_buf_printf(
            &mut pulls,
            b"%s%d\0".as_ptr() as *const c_char,
            if i == 0 {
                b"\0".as_ptr() as *const c_char
            } else {
                b",\0".as_ptr() as *const c_char
            },
            (*param).spkid_pulls[i],
        );
        i += 1;
    }

    snprintf(
        desc,
        KUNIT_PARAM_DESC_SIZE,
        b"gpios:{%s} pulls:{%s} status:%#lx spkid:%d\0".as_ptr() as *const c_char,
        seq_buf_str(&gpios),
        seq_buf_str(&pulls),
        (*param).gpio_status,
        (*param).spkid,
    );
}

static cs35l56_shared_test_gpios_selftest_cases: [cs35l56_shared_test_param; 1] = [
    cs35l56_shared_test_param {
        spkid_gpios: [-1, 0, 0, 0],
        spkid_pulls: [0; 4],
        gpio_status: GENMASK(12, 0),
        spkid: 0,
    },
];

static cs35l56_shared_test_onchip_spkid_cases: [cs35l56_shared_test_param; 24] = [
    cs35l56_shared_test_param { spkid_gpios: [1, -1, 0, 0], spkid_pulls: [0; 4], gpio_status: 0, spkid: 0 },
    cs35l56_shared_test_param { spkid_gpios: [1, -1, 0, 0], spkid_pulls: [0; 4], gpio_status: !BIT(0), spkid: 0 },
    cs35l56_shared_test_param { spkid_gpios: [1, -1, 0, 0], spkid_pulls: [0; 4], gpio_status: BIT(0), spkid: 1 },
    cs35l56_shared_test_param { spkid_gpios: [7, -1, 0, 0], spkid_pulls: [0; 4], gpio_status: 0, spkid: 0 },
    cs35l56_shared_test_param { spkid_gpios: [7, -1, 0, 0], spkid_pulls: [0; 4], gpio_status: !BIT(6), spkid: 0 },
    cs35l56_shared_test_param { spkid_gpios: [7, -1, 0, 0], spkid_pulls: [0; 4], gpio_status: BIT(6), spkid: 1 },
    cs35l56_shared_test_param { spkid_gpios: [1, 7, -1, 0], spkid_pulls: [0; 4], gpio_status: 0, spkid: 0 },
    cs35l56_shared_test_param { spkid_gpios: [1, 7, -1, 0], spkid_pulls: [0; 4], gpio_status: !(BIT(0) | BIT(6)), spkid: 0 },
    cs35l56_shared_test_param { spkid_gpios: [1, 7, -1, 0], spkid_pulls: [0; 4], gpio_status: BIT(6), spkid: 1 },
    cs35l56_shared_test_param { spkid_gpios: [1, 7, -1, 0], spkid_pulls: [0; 4], gpio_status: BIT(0), spkid: 2 },
    cs35l56_shared_test_param { spkid_gpios: [1, 7, -1, 0], spkid_pulls: [0; 4], gpio_status: BIT(6) | BIT(0), spkid: 3 },
    cs35l56_shared_test_param { spkid_gpios: [7, 1, -1, 0], spkid_pulls: [0; 4], gpio_status: 0, spkid: 0 },
    cs35l56_shared_test_param { spkid_gpios: [7, 1, -1, 0], spkid_pulls: [0; 4], gpio_status: !(BIT(6) | BIT(0)), spkid: 0 },
    cs35l56_shared_test_param { spkid_gpios: [7, 1, -1, 0], spkid_pulls: [0; 4], gpio_status: BIT(0), spkid: 1 },
    cs35l56_shared_test_param { spkid_gpios: [7, 1, -1, 0], spkid_pulls: [0; 4], gpio_status: BIT(6), spkid: 2 },
    cs35l56_shared_test_param { spkid_gpios: [7, 1, -1, 0], spkid_pulls: [0; 4], gpio_status: BIT(6) | BIT(0), spkid: 3 },
    cs35l56_shared_test_param { spkid_gpios: [3, 7, 1, -1], spkid_pulls: [0; 4], gpio_status: 0, spkid: 0 },
    cs35l56_shared_test_param { spkid_gpios: [3, 7, 1, -1], spkid_pulls: [0; 4], gpio_status: BIT(0), spkid: 1 },
    cs35l56_shared_test_param { spkid_gpios: [3, 7, 1, -1], spkid_pulls: [0; 4], gpio_status: BIT(6), spkid: 2 },
    cs35l56_shared_test_param { spkid_gpios: [3, 7, 1, -1], spkid_pulls: [0; 4], gpio_status: BIT(6) | BIT(0), spkid: 3 },
    cs35l56_shared_test_param { spkid_gpios: [3, 7, 1, -1], spkid_pulls: [0; 4], gpio_status: BIT(2), spkid: 4 },
    cs35l56_shared_test_param { spkid_gpios: [3, 7, 1, -1], spkid_pulls: [0; 4], gpio_status: BIT(2) | BIT(0), spkid: 5 },
    cs35l56_shared_test_param { spkid_gpios: [3, 7, 1, -1], spkid_pulls: [0; 4], gpio_status: BIT(2) | BIT(6), spkid: 6 },
    cs35l56_shared_test_param { spkid_gpios: [3, 7, 1, -1], spkid_pulls: [0; 4], gpio_status: BIT(2) | BIT(6) | BIT(0), spkid: 7 },
];

static cs35l56_shared_test_onchip_spkid_pull_cases: [cs35l56_shared_test_param; 10] = [
    cs35l56_shared_test_param { spkid_gpios: [1, -1, 0, 0], spkid_pulls: [1, -1, 0, 0], gpio_status: 0, spkid: 0 },
    cs35l56_shared_test_param { spkid_gpios: [1, -1, 0, 0], spkid_pulls: [2, -1, 0, 0], gpio_status: 0, spkid: 0 },
    cs35l56_shared_test_param { spkid_gpios: [7, -1, 0, 0], spkid_pulls: [1, -1, 0, 0], gpio_status: 0, spkid: 0 },
    cs35l56_shared_test_param { spkid_gpios: [7, -1, 0, 0], spkid_pulls: [2, -1, 0, 0], gpio_status: 0, spkid: 0 },
    cs35l56_shared_test_param { spkid_gpios: [1, 7, -1, 0], spkid_pulls: [1, 1, -1, 0], gpio_status: 0, spkid: 0 },
    cs35l56_shared_test_param { spkid_gpios: [1, 7, -1, 0], spkid_pulls: [2, 2, -1, 0], gpio_status: 0, spkid: 0 },
    cs35l56_shared_test_param { spkid_gpios: [7, 1, -1, 0], spkid_pulls: [1, 1, -1, 0], gpio_status: 0, spkid: 0 },
    cs35l56_shared_test_param { spkid_gpios: [7, 1, -1, 0], spkid_pulls: [2, 2, -1, 0], gpio_status: 0, spkid: 0 },
    cs35l56_shared_test_param { spkid_gpios: [3, 7, 1, -1], spkid_pulls: [1, 1, 1, -1], gpio_status: 0, spkid: 0 },
    cs35l56_shared_test_param { spkid_gpios: [3, 7, 1, -1], spkid_pulls: [2, 2, 2, -1], gpio_status: 0, spkid: 0 },
];

/* Note: spk-id-gpios property bit order is LSbit...MSbit */
static cs35l56_shared_test_host_gpio_spkid_cases: [cs35l56_shared_test_param; 24] = [
    cs35l56_shared_test_param { spkid_gpios: [0, -1, 0, 0], spkid_pulls: [0; 4], gpio_status: 0, spkid: 0 },
    cs35l56_shared_test_param { spkid_gpios: [0, -1, 0, 0], spkid_pulls: [0; 4], gpio_status: !BIT(0), spkid: 0 },
    cs35l56_shared_test_param { spkid_gpios: [0, -1, 0, 0], spkid_pulls: [0; 4], gpio_status: BIT(0), spkid: 1 },
    cs35l56_shared_test_param { spkid_gpios: [6, -1, 0, 0], spkid_pulls: [0; 4], gpio_status: 0, spkid: 0 },
    cs35l56_shared_test_param { spkid_gpios: [6, -1, 0, 0], spkid_pulls: [0; 4], gpio_status: !BIT(6), spkid: 0 },
    cs35l56_shared_test_param { spkid_gpios: [6, -1, 0, 0], spkid_pulls: [0; 4], gpio_status: BIT(6), spkid: 1 },
    cs35l56_shared_test_param { spkid_gpios: [6, 0, -1, 0], spkid_pulls: [0; 4], gpio_status: 0, spkid: 0 },
    cs35l56_shared_test_param { spkid_gpios: [6, 0, -1, 0], spkid_pulls: [0; 4], gpio_status: !(BIT(0) | BIT(6)), spkid: 0 },
    cs35l56_shared_test_param { spkid_gpios: [6, 0, -1, 0], spkid_pulls: [0; 4], gpio_status: BIT(6), spkid: 1 },
    cs35l56_shared_test_param { spkid_gpios: [6, 0, -1, 0], spkid_pulls: [0; 4], gpio_status: BIT(0), spkid: 2 },
    cs35l56_shared_test_param { spkid_gpios: [6, 0, -1, 0], spkid_pulls: [0; 4], gpio_status: BIT(6) | BIT(0), spkid: 3 },
    cs35l56_shared_test_param { spkid_gpios: [0, 6, -1, 0], spkid_pulls: [0; 4], gpio_status: 0, spkid: 0 },
    cs35l56_shared_test_param { spkid_gpios: [0, 6, -1, 0], spkid_pulls: [0; 4], gpio_status: !(BIT(6) | BIT(0)), spkid: 0 },
    cs35l56_shared_test_param { spkid_gpios: [0, 6, -1, 0], spkid_pulls: [0; 4], gpio_status: BIT(0), spkid: 1 },
    cs35l56_shared_test_param { spkid_gpios: [0, 6, -1, 0], spkid_pulls: [0; 4], gpio_status: BIT(6), spkid: 2 },
    cs35l56_shared_test_param { spkid_gpios: [0, 6, -1, 0], spkid_pulls: [0; 4], gpio_status: BIT(6) | BIT(0), spkid: 3 },
    cs35l56_shared_test_param { spkid_gpios: [0, 6, 2, -1], spkid_pulls: [0; 4], gpio_status: 0, spkid: 0 },
    cs35l56_shared_test_param { spkid_gpios: [0, 6, 2, -1], spkid_pulls: [0; 4], gpio_status: BIT(0), spkid: 1 },
    cs35l56_shared_test_param { spkid_gpios: [0, 6, 2, -1], spkid_pulls: [0; 4], gpio_status: BIT(6), spkid: 2 },
    cs35l56_shared_test_param { spkid_gpios: [0, 6, 2, -1], spkid_pulls: [0; 4], gpio_status: BIT(6) | BIT(0), spkid: 3 },
    cs35l56_shared_test_param { spkid_gpios: [0, 6, 2, -1], spkid_pulls: [0; 4], gpio_status: BIT(2), spkid: 4 },
    cs35l56_shared_test_param { spkid_gpios: [0, 6, 2, -1], spkid_pulls: [0; 4], gpio_status: BIT(2) | BIT(0), spkid: 5 },
    cs35l56_shared_test_param { spkid_gpios: [0, 6, 2, -1], spkid_pulls: [0; 4], gpio_status: BIT(2) | BIT(6), spkid: 6 },
    cs35l56_shared_test_param { spkid_gpios: [0, 6, 2, -1], spkid_pulls: [0; 4], gpio_status: BIT(2) | BIT(6) | BIT(0), spkid: 7 },
];

static mut cs35l56_shared_test_cases: [kunit_case; 11] = [
    /* Tests for speaker id */
    kunit_case { name: b"cs35l56_shared_test_mock_gpio_status_selftest\0".as_ptr() as *const c_char, run_case: Some(cs35l56_shared_test_mock_gpio_status_selftest), generate_params: cs35l56_shared_test_gpios_selftest_cases.as_ptr() as *const c_void, attr: 0 },
    kunit_case { name: b"cs35l56_shared_test_get_onchip_speaker_id\0".as_ptr() as *const c_char, run_case: Some(cs35l56_shared_test_get_onchip_speaker_id), generate_params: cs35l56_shared_test_onchip_spkid_cases.as_ptr() as *const c_void, attr: 0 },
    kunit_case { name: b"cs35l56_shared_test_onchip_speaker_id_pad_config\0".as_ptr() as *const c_char, run_case: Some(cs35l56_shared_test_onchip_speaker_id_pad_config), generate_params: cs35l56_shared_test_onchip_spkid_cases.as_ptr() as *const c_void, attr: 0 },
    kunit_case { name: b"cs35l56_shared_test_onchip_speaker_id_pad_config\0".as_ptr() as *const c_char, run_case: Some(cs35l56_shared_test_onchip_speaker_id_pad_config), generate_params: cs35l56_shared_test_onchip_spkid_pull_cases.as_ptr() as *const c_void, attr: 0 },
    kunit_case { name: b"cs35l56_shared_test_stash_onchip_spkid_pins\0".as_ptr() as *const c_char, run_case: Some(cs35l56_shared_test_stash_onchip_spkid_pins), generate_params: cs35l56_shared_test_onchip_spkid_pull_cases.as_ptr() as *const c_void, attr: 0 },
    kunit_case { name: b"cs35l56_shared_test_stash_onchip_spkid_pins_reject_invalid\0".as_ptr() as *const c_char, run_case: Some(cs35l56_shared_test_stash_onchip_spkid_pins_reject_invalid), generate_params: null(), attr: 0 },
    kunit_case { name: b"cs35l56_shared_test_onchip_speaker_id_not_defined\0".as_ptr() as *const c_char, run_case: Some(cs35l56_shared_test_onchip_speaker_id_not_defined), generate_params: null(), attr: 0 },
    kunit_case { name: b"cs35l56_shared_test_get_speaker_id_vendor\0".as_ptr() as *const c_char, run_case: Some(cs35l56_shared_test_get_speaker_id_vendor), generate_params: null(), attr: 0 },
    kunit_case { name: b"cs35l56_shared_test_get_speaker_id_property\0".as_ptr() as *const c_char, run_case: Some(cs35l56_shared_test_get_speaker_id_property), generate_params: null(), attr: 0 },
    kunit_case { name: b"cs35l56_shared_test_get_speaker_id_from_host_gpio\0".as_ptr() as *const c_char, run_case: Some(cs35l56_shared_test_get_speaker_id_from_host_gpio), generate_params: cs35l56_shared_test_host_gpio_spkid_cases.as_ptr() as *const c_void, attr: KUNIT_SPEED_SLOW },
    kunit_case { name: null(), run_case: None, generate_params: null(), attr: 0 },
];

static mut cs35l56_shared_test_suite_L56_B0_sdw: kunit_suite = kunit_suite {
    name: b"snd-soc-cs35l56-shared-test_L56_B0_sdw\0".as_ptr() as *const c_char,
    init: Some(cs35l56_shared_test_case_regmap_init_L56_B0_sdw),
    test_cases: unsafe { cs35l56_shared_test_cases.as_mut_ptr() },
};

static mut cs35l56_shared_test_suite_L56_B2_sdw: kunit_suite = kunit_suite {
    name: b"snd-soc-cs35l56-shared-test_L56_B2_sdw\0".as_ptr() as *const c_char,
    init: Some(cs35l56_shared_test_case_regmap_init_L56_B2_sdw),
    test_cases: unsafe { cs35l56_shared_test_cases.as_mut_ptr() },
};

static mut cs35l56_shared_test_suite_L63_A1_sdw: kunit_suite = kunit_suite {
    name: b"snd-soc-cs35l56-shared-test_L63_A1_sdw\0".as_ptr() as *const c_char,
    init: Some(cs35l56_shared_test_case_regmap_init_L63_A1_sdw),
    test_cases: unsafe { cs35l56_shared_test_cases.as_mut_ptr() },
};

static mut cs35l56_shared_test_suite_L56_B0_spi: kunit_suite = kunit_suite {
    name: b"snd-soc-cs35l56-shared-test_L56_B0_spi\0".as_ptr() as *const c_char,
    init: Some(cs35l56_shared_test_case_regmap_init_L56_B0_spi),
    test_cases: unsafe { cs35l56_shared_test_cases.as_mut_ptr() },
};

static mut cs35l56_shared_test_suite_L56_B2_spi: kunit_suite = kunit_suite {
    name: b"snd-soc-cs35l56-shared-test_L56_B2_spi\0".as_ptr() as *const c_char,
    init: Some(cs35l56_shared_test_case_regmap_init_L56_B2_spi),
    test_cases: unsafe { cs35l56_shared_test_cases.as_mut_ptr() },
};

static mut cs35l56_shared_test_suite_L56_B0_i2c: kunit_suite = kunit_suite {
    name: b"snd-soc-cs35l56-shared-test_L56_B0_i2c\0".as_ptr() as *const c_char,
    init: Some(cs35l56_shared_test_case_regmap_init_L56_B0_i2c),
    test_cases: unsafe { cs35l56_shared_test_cases.as_mut_ptr() },
};

static mut cs35l56_shared_test_suite_L56_B2_i2c: kunit_suite = kunit_suite {
    name: b"snd-soc-cs35l56-shared-test_L56_B2_i2c\0".as_ptr() as *const c_char,
    init: Some(cs35l56_shared_test_case_regmap_init_L56_B2_i2c),
    test_cases: unsafe { cs35l56_shared_test_cases.as_mut_ptr() },
};

/* kunit_test_suites(
 *	&cs35l56_shared_test_suite_L56_B0_sdw,
 *	&cs35l56_shared_test_suite_L56_B2_sdw,
 *	&cs35l56_shared_test_suite_L63_A1_sdw,
 *
 *	&cs35l56_shared_test_suite_L56_B0_spi,
 *	&cs35l56_shared_test_suite_L56_B2_spi,
 *
 *	&cs35l56_shared_test_suite_L56_B0_i2c,
 *	&cs35l56_shared_test_suite_L56_B2_i2c,
 * );
 */
static mut cs35l56_shared_test_suites: [*mut kunit_suite; 8] = unsafe {
    [
        &mut cs35l56_shared_test_suite_L56_B0_sdw,
        &mut cs35l56_shared_test_suite_L56_B2_sdw,
        &mut cs35l56_shared_test_suite_L63_A1_sdw,
        &mut cs35l56_shared_test_suite_L56_B0_spi,
        &mut cs35l56_shared_test_suite_L56_B2_spi,
        &mut cs35l56_shared_test_suite_L56_B0_i2c,
        &mut cs35l56_shared_test_suite_L56_B2_i2c,
        null_mut(),
    ]
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
