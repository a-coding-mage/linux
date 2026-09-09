// SPDX-License-Identifier: GPL-2.0-only
/*
 * Shared helpers to register GPIO-connected buttons and LEDs
 * on AMD Geode boards.
 */

// External kernel declarations and macros are supplied by the surrounding
// translation unit. Build-time configuration and init annotations are retained
// as comments where Rust has no direct local equivalent.

extern "C" {
    static cs5535_gpio_swnode: software_node;
}

static mut geode_gpio_keys_props: [property_entry; 2] = [
    PROPERTY_ENTRY_U32!("poll-interval", 20),
    property_entry { ..Default::default() },
];

static geode_gpio_keys_node: software_node = software_node {
    name: "geode-gpio-keys",
    properties: geode_gpio_keys_props.as_ptr(),
    ..Default::default()
};

static mut geode_restart_gpio_ref: software_node_ref_args = software_node_ref_args {
    ..Default::default()
};

static mut geode_restart_key_props: [property_entry; 5] = [
    PROPERTY_ENTRY_REF_ARRAY_LEN!("gpios", &geode_restart_gpio_ref, 1),
    PROPERTY_ENTRY_U32!("linux,code", KEY_RESTART),
    PROPERTY_ENTRY_STRING!("label", "Reset button"),
    PROPERTY_ENTRY_U32!("debounce-interval", 100),
    property_entry { ..Default::default() },
];

static geode_restart_key_node: software_node = software_node {
    parent: &geode_gpio_keys_node,
    properties: geode_restart_key_props.as_ptr(),
    ..Default::default()
};

static geode_gpio_keys_swnodes: [*const software_node; 3] = [
    &geode_gpio_keys_node,
    &geode_restart_key_node,
    core::ptr::null(),
];

/*
 * Creates gpio-keys-polled device for the restart key.
 *
 * Note that it needs to be called first, before geode_create_leds(),
 * because it registers gpiochip software node used by both gpio-keys and
 * leds-gpio devices.
 */
// __init
pub unsafe fn geode_create_restart_key(pin: c_uint) -> c_int {
    let mut keys_info = platform_device_info {
        name: "gpio-keys-polled",
        id: 1,
        ..Default::default()
    };
    let pd: *mut platform_device;
    let mut err: c_int;

    geode_restart_gpio_ref = SOFTWARE_NODE_REFERENCE!(&cs5535_gpio_swnode, pin, GPIO_ACTIVE_LOW);

    err = software_node_register_node_group(geode_gpio_keys_swnodes.as_ptr());
    if err != 0 {
        pr_err!("failed to register gpio-keys software nodes: %d\n", err);
        return err;
    }

    keys_info.fwnode = software_node_fwnode(&geode_gpio_keys_node);

    pd = platform_device_register_full(&keys_info);
    err = PTR_ERR_OR_ZERO(pd);
    if err != 0 {
        pr_err!("failed to create gpio-keys device: %d\n", err);
        software_node_unregister_node_group(geode_gpio_keys_swnodes.as_ptr());
        return err;
    }

    0
}

static geode_gpio_leds_node: software_node = software_node {
    name: "geode-leds",
    ..Default::default()
};

const MAX_LEDS: usize = 3;

// __init
pub unsafe fn geode_create_leds(
    label: *const c_char,
    leds: *const geode_led,
    n_leds: c_uint,
) -> c_int {
    let mut group: [*const software_node; MAX_LEDS + 2] = [core::ptr::null(); MAX_LEDS + 2];
    let mut swnodes: *mut software_node;
    let mut props: *mut property_entry;
    let mut gpio_refs: *mut software_node_ref_args;
    let mut led_info = platform_device_info {
        name: "leds-gpio",
        id: PLATFORM_DEVID_NONE,
        ..Default::default()
    };
    let led_dev: *mut platform_device;
    let mut node_name: *const c_char;
    let mut err: c_int;
    let mut i: c_int = 0;

    if n_leds as usize > MAX_LEDS {
        pr_err!("%s: too many LEDs\n", __func__);
        return -EINVAL;
    }

    swnodes = kzalloc_objs!(software_node, n_leds as usize);
    if swnodes.is_null() {
        return -ENOMEM;
    }

    /*
     * Each LED is represented by 3 properties: "gpios",
     * "linux,default-trigger", and am empty terminator.
     */
    props = kzalloc_objs!(property_entry, n_leds as usize * 3);
    if props.is_null() {
        err = -ENOMEM;
        goto!(err_free_swnodes);
    }

    gpio_refs = kzalloc_objs!(software_node_ref_args, n_leds as usize);
    if gpio_refs.is_null() {
        err = -ENOMEM;
        goto!(err_free_props);
    }

    group[0] = &geode_gpio_leds_node;
    i = 0;
    while i < n_leds as c_int {
        node_name = kasprintf!(GFP_KERNEL, "%s:%d", label, i);
        if node_name.is_null() {
            err = -ENOMEM;
            goto!(err_free_names);
        }

        *gpio_refs.add(i as usize) = SOFTWARE_NODE_REFERENCE!(
            &cs5535_gpio_swnode,
            (*leds.add(i as usize)).pin,
            GPIO_ACTIVE_LOW
        );
        *props.add(i as usize * 3) = PROPERTY_ENTRY_REF_ARRAY_LEN!(
            "gpios", gpio_refs.add(i as usize), 1
        );
        *props.add(i as usize * 3 + 1) = PROPERTY_ENTRY_STRING!(
            "linux,default-trigger",
            if (*leds.add(i as usize)).default_on { "default-on" } else { "default-off" }
        );
        /* props[i * 3 + 2] is an empty terminator */

        *swnodes.add(i as usize) = SOFTWARE_NODE!(
            node_name,
            props.add(i as usize * 3),
            &geode_gpio_leds_node
        );
        group[i as usize + 1] = swnodes.add(i as usize);
        i += 1;
    }

    err = software_node_register_node_group(group.as_ptr());
    if err != 0 {
        pr_err!("failed to register LED software nodes: %d\n", err);
        goto!(err_free_names);
    }

    led_info.fwnode = software_node_fwnode(&geode_gpio_leds_node);

    led_dev = platform_device_register_full(&led_info);
    err = PTR_ERR_OR_ZERO(led_dev);
    if err != 0 {
        pr_err!("failed to create LED device: %d\n", err);
        goto!(err_unregister_group);
    }

    return 0;

err_unregister_group:
    software_node_unregister_node_group(group.as_ptr());
err_free_names:
    while { i -= 1; i >= 0 } {
        kfree((*swnodes.add(i as usize)).name as *mut c_void);
    }
    kfree(gpio_refs as *mut c_void);
err_free_props:
    kfree(props as *mut c_void);
err_free_swnodes:
    kfree(swnodes as *mut c_void);
    err
}

// MODULE_IMPORT_NS("CS5535");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
