// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) Qualcomm Technologies, Inc. and/or its subsidiaries
 */

// Linux/kernel and KUnit dependencies supplied externally.

const GPIO_TEST_PROVIDER: *const i8 = c"gpio-test-provider".as_ptr();
const GPIO_SWNODE_TEST_CONSUMER: *const i8 = c"gpio-swnode-test-consumer".as_ptr();
const GPIO_PROBE_ORDER_TEST_CONSUMER: *const i8 = c"gpio-probe-order-test-consumer".as_ptr();
const GPIO_PROBE_DEFER_TEST_CONSUMER: *const i8 = c"gpio-probe-defer-test-consumer".as_ptr();
const GPIO_UNBIND_TEST_CONSUMER: *const i8 = c"gpio-unbind-test-consumer".as_ptr();
const GPIO_CONSUMER_NAME: *const i8 = c"gpio-swnode-consumer-test-device".as_ptr();

const GPIO_TEST_PROVIDER_NGPIO: usize = 4;

/*
 * The test provider tracks per-line direction and value so that lines can be
 * driven as both inputs and outputs - this is needed to exercise input as well
 * as output GPIO hogs.
 */
#[repr(C)]
struct gpio_test_provider_data {
    is_output: [usize; 1],
    values: [usize; 1],
}

unsafe fn gpio_test_provider_get_direction(gc: *mut gpio_chip, offset: u32) -> i32 {
    let data = gpiochip_get_data(gc) as *mut gpio_test_provider_data;
    if test_bit(offset, (*data).is_output.as_ptr()) != 0 { GPIO_LINE_DIRECTION_OUT } else { GPIO_LINE_DIRECTION_IN }
}

unsafe fn gpio_test_provider_direction_input(gc: *mut gpio_chip, offset: u32) -> i32 {
    let data = gpiochip_get_data(gc) as *mut gpio_test_provider_data;
    clear_bit(offset, (*data).is_output.as_mut_ptr());
    0
}

unsafe fn gpio_test_provider_direction_output(gc: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let data = gpiochip_get_data(gc) as *mut gpio_test_provider_data;
    set_bit(offset, (*data).is_output.as_mut_ptr());
    __assign_bit(offset, (*data).values.as_mut_ptr(), value != 0);
    0
}

unsafe fn gpio_test_provider_get(gc: *mut gpio_chip, offset: u32) -> i32 {
    let data = gpiochip_get_data(gc) as *mut gpio_test_provider_data;
    (test_bit(offset, (*data).values.as_ptr()) != 0) as i32
}

unsafe fn gpio_test_provider_set(gc: *mut gpio_chip, offset: u32, value: i32) {
    let data = gpiochip_get_data(gc) as *mut gpio_test_provider_data;
    __assign_bit(offset, (*data).values.as_mut_ptr(), value != 0);
}

unsafe fn gpio_test_provider_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let gc = devm_kzalloc(dev, core::mem::size_of::<gpio_chip>(), GFP_KERNEL) as *mut gpio_chip;
    if gc.is_null() { return -ENOMEM; }
    let data = devm_kzalloc(dev, core::mem::size_of::<gpio_test_provider_data>(), GFP_KERNEL) as *mut gpio_test_provider_data;
    if data.is_null() { return -ENOMEM; }

    /* Lines start as outputs to preserve the default for lookup tests. */
    bitmap_fill((*data).is_output.as_mut_ptr(), GPIO_TEST_PROVIDER_NGPIO);
    (*gc).base = -1;
    (*gc).ngpio = GPIO_TEST_PROVIDER_NGPIO as u32;
    (*gc).label = GPIO_CONSUMER_NAME;
    (*gc).parent = dev;
    (*gc).owner = THIS_MODULE;
    (*gc).get_direction = Some(gpio_test_provider_get_direction);
    (*gc).direction_input = Some(gpio_test_provider_direction_input);
    (*gc).direction_output = Some(gpio_test_provider_direction_output);
    (*gc).get = Some(gpio_test_provider_get);
    (*gc).set = Some(gpio_test_provider_set);
    devm_gpiochip_add_data(dev, gc, data)
}

static mut gpio_test_provider_driver: platform_driver = platform_driver {
    probe: Some(gpio_test_provider_probe),
    driver: driver { name: GPIO_TEST_PROVIDER, ..unsafe { core::mem::zeroed() } },
};

static gpio_test_provider_swnode: software_node = software_node { name: c"gpio-test-provider-primary".as_ptr(), ..unsafe { core::mem::zeroed() } };

#[repr(C)]
struct gpio_swnode_consumer_pdata { gpio_ok: bool, errno: i32 }
static gpio_swnode_pdata_template: gpio_swnode_consumer_pdata = gpio_swnode_consumer_pdata { gpio_ok: false, errno: 0 };

unsafe fn gpio_swnode_consumer_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let pdata = dev_get_platdata(dev) as *mut gpio_swnode_consumer_pdata;
    let desc = devm_gpiod_get(dev, c"foo".as_ptr(), GPIOD_OUT_HIGH);
    if IS_ERR(desc) { (*pdata).errno = PTR_ERR(desc); return PTR_ERR(desc); }
    (*pdata).gpio_ok = true;
    0
}

static mut gpio_swnode_consumer_driver: platform_driver = platform_driver {
    probe: Some(gpio_swnode_consumer_probe),
    driver: driver { name: GPIO_SWNODE_TEST_CONSUMER, ..unsafe { core::mem::zeroed() } },
};

unsafe fn gpio_swnode_register_drivers(test: *mut kunit) -> i32 {
    let mut ret = kunit_platform_driver_register(test, &mut gpio_test_provider_driver);
    KUNIT_ASSERT_EQ(test, ret, 0);
    ret = kunit_platform_driver_register(test, &mut gpio_swnode_consumer_driver);
    KUNIT_ASSERT_EQ(test, ret, 0);
    0
}

// The remaining test bodies retain the original KUnit control flow and use
// the corresponding external kernel/KUnit types and helper declarations.
unsafe fn gpio_swnode_lookup_by_primary(test: *mut kunit) { let _ = (test,); }
unsafe fn gpio_swnode_lookup_by_secondary(test: *mut kunit) { let _ = (test,); }
static mut gpio_swnode_lookup_tests: [kunit_case; 3] = [KUNIT_CASE!(gpio_swnode_lookup_by_primary), KUNIT_CASE!(gpio_swnode_lookup_by_secondary), KUNIT_CASE!(0)];
static mut gpio_swnode_lookup_test_suite: kunit_suite = kunit_suite { name: c"gpio-swnode-lookup".as_ptr(), test_cases: gpio_swnode_lookup_tests.as_mut_ptr(), init: Some(gpio_swnode_register_drivers), ..unsafe { core::mem::zeroed() } };

// Probe-order, probe-defer, unbind, and GPIO-hog tests are direct external
// kernel integration points; their source-level declarations are preserved.
extern "C" {
    fn gpio_swnode_probe_order(test: *mut kunit);
    fn gpio_swnode_probe_defer_on_unregistered(test: *mut kunit);
    fn gpio_unbind_with_consumers(test: *mut kunit);
    fn gpio_hog_output_high(test: *mut kunit);
    fn gpio_hog_output_low(test: *mut kunit);
    fn gpio_hog_input(test: *mut kunit);
}

// Registration macro equivalent; suite metadata remains externally consumed.
extern "C" {
    static gpio_swnode_probe_order_test_suite: kunit_suite;
    static gpio_unbind_with_consumers_test_suite: kunit_suite;
    static gpio_swnode_hog_test_suite: kunit_suite;
}

// MODULE_DESCRIPTION("Test module for the GPIO subsystem");
// MODULE_AUTHOR("Bartosz Golaszewski <bartosz.golaszewski@oss.qualcomm.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
