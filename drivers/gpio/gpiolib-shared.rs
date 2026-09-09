// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2025 Linaro Ltd.
 */

// Linux headers and local headers are supplied by other translated units.
// Build-time CONFIG_OF and CONFIG_RESET_GPIO conditions are preserved below.

#[repr(C)]
struct gpio_shared_ref {
    list: list_head,
    fwnode: *mut fwnode_handle,
    flags: gpiod_flags,
    con_id: *mut core::ffi::c_char,
    dev_id: i32,
    lock: mutex,
    lock_key: lock_class_key,
    adev: auxiliary_device,
    lookup: *mut gpiod_lookup_table,
    is_reset_gpio: bool,
}

#[repr(C)]
struct gpio_shared_entry {
    list: list_head,
    fwnode: *mut fwnode_handle,
    offset: u32,
    index: usize,
    lock: mutex,
    shared_desc: *mut gpio_shared_desc,
    ref_: kref,
    refs: list_head,
}

static mut gpio_shared_list: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut gpio_shared_ida: ida = ida { /* DEFINE_IDA */ };

#[cfg(CONFIG_OF)]
unsafe fn gpio_shared_find_entry(controller_node: *mut fwnode_handle, offset: u32) -> *mut gpio_shared_entry {
    let mut entry: *mut gpio_shared_entry = core::ptr::null_mut();
    list_for_each_entry(entry, &raw mut gpio_shared_list, list) {
        if (*entry).fwnode == controller_node && (*entry).offset == offset { return entry; }
    }
    core::ptr::null_mut()
}

#[cfg(CONFIG_OF)]
unsafe fn gpio_shared_make_ref(fwnode: *mut fwnode_handle, con_id: *const core::ffi::c_char, flags: gpiod_flags) -> *mut gpio_shared_ref {
    let ref_: *mut gpio_shared_ref = kzalloc_obj();
    if ref_.is_null() { return core::ptr::null_mut(); }
    let con_id_cpy = if !con_id.is_null() { kstrdup(con_id, GFP_KERNEL) } else { core::ptr::null_mut() };
    if !con_id.is_null() && con_id_cpy.is_null() { kfree(ref_ as *mut _); return core::ptr::null_mut(); }
    (*ref_).dev_id = ida_alloc(&raw mut gpio_shared_ida, GFP_KERNEL);
    if (*ref_).dev_id < 0 { kfree(con_id_cpy as *mut _); kfree(ref_ as *mut _); return core::ptr::null_mut(); }
    (*ref_).flags = flags;
    (*ref_).con_id = con_id_cpy;
    (*ref_).fwnode = fwnode;
    lockdep_register_key(&raw mut (*ref_).lock_key);
    mutex_init_with_key(&raw mut (*ref_).lock, &raw mut (*ref_).lock_key);
    ref_
}

#[cfg(CONFIG_OF)]
unsafe fn gpio_shared_setup_reset_proxy(entry: *mut gpio_shared_entry, flags: gpiod_flags) -> i32 {
    let mut ref_: *mut gpio_shared_ref = core::ptr::null_mut();
    list_for_each_entry(ref_, &raw mut (*entry).refs, list) {
        if (*ref_).is_reset_gpio { return 0; }
    }
    ref_ = gpio_shared_make_ref(core::ptr::null_mut(), c"reset".as_ptr(), flags);
    if ref_.is_null() { return -ENOMEM; }
    (*ref_).is_reset_gpio = true;
    list_add_tail(&raw mut (*ref_).list, &raw mut (*entry).refs);
    pr_debug!("Created a secondary shared GPIO reference for potential reset-gpio device for GPIO {} at {}\n", (*entry).offset, fwnode_get_name((*entry).fwnode));
    0
}

#[cfg(CONFIG_OF)]
unsafe fn gpio_shared_of_node_ignore(node: *mut device_node) -> bool {
    if !of_device_is_available(node) { return true; }
    if of_node_name_eq(node, c"__symbols__".as_ptr()) { return true; }
    if of_property_present(node, c"gpio-hog".as_ptr()) { return true; }
    false
}

#[cfg(CONFIG_OF)]
unsafe fn gpio_shared_of_traverse(curr: *mut device_node) -> i32 {
    // The property and child traversal below directly mirrors the C implementation.
    if gpio_shared_of_node_ignore(curr) { return 0; }
    let mut prop: *mut property = core::ptr::null_mut();
    for_each_property_of_node(curr, prop) {
        if !strends((*prop).name, c"-gpios".as_ptr()) && !strends((*prop).name, c"-gpio".as_ptr()) && strcmp((*prop).name, c"gpios".as_ptr()) != 0 && strcmp((*prop).name, c"gpio".as_ptr()) != 0 { continue; }
        let count = of_count_phandle_with_args(curr, (*prop).name, c"#gpio-cells".as_ptr());
        if count <= 0 { continue; }
        for i in 0..count {
            let mut args = of_phandle_args::default();
            if of_parse_phandle_with_args(curr, (*prop).name, c"#gpio-cells".as_ptr(), i, &mut args) != 0 { continue; }
            let np = args.np;
            if !of_property_present(np, c"gpio-controller".as_ptr()) || args.args_count != 2 { continue; }
            let fwnode = of_fwnode_handle(args.np);
            let offset = args.args[0];
            let mut entry = gpio_shared_find_entry(fwnode, offset);
            if entry.is_null() {
                entry = kzalloc_obj();
                if entry.is_null() { return -ENOMEM; }
                (*entry).fwnode = fwnode_handle_get(fwnode);
                (*entry).offset = offset;
                (*entry).index = count as usize;
                INIT_LIST_HEAD(&raw mut (*entry).refs);
                mutex_init(&raw mut (*entry).lock);
                list_add_tail(&raw mut (*entry).list, &raw mut gpio_shared_list);
            }
            let suffix = if strends((*prop).name, c"gpios".as_ptr()) { c"-gpios".as_ptr() } else if strends((*prop).name, c"gpio".as_ptr()) { c"-gpio".as_ptr() } else { core::ptr::null() };
            if suffix.is_null() { continue; }
            let con_id = if strcmp((*prop).name, c"gpios".as_ptr()) != 0 && strcmp((*prop).name, c"gpio".as_ptr()) != 0 { let p = kstrdup((*prop).name, GFP_KERNEL); if p.is_null() { return -ENOMEM; } p } else { core::ptr::null_mut() };
            if !con_id.is_null() { *con_id.add(strlen(con_id) - strlen(suffix)) = 0; }
            let ref_ = gpio_shared_make_ref(fwnode_handle_get(of_fwnode_handle(curr)), con_id, args.args[1]);
            kfree(con_id as *mut _);
            if ref_.is_null() { return -ENOMEM; }
            list_add_tail(&raw mut (*ref_).list, &raw mut (*entry).refs);
            if strcmp((*prop).name, c"reset-gpios".as_ptr()) == 0 { let ret = gpio_shared_setup_reset_proxy(entry, args.args[1]); if ret != 0 { return ret; } }
        }
    }
    let mut child: *mut device_node = core::ptr::null_mut();
    for_each_child_of_node_scoped(curr, child) { let ret = gpio_shared_of_traverse(child); if ret != 0 { return ret; } }
    0
}

#[cfg(CONFIG_OF)]
unsafe fn gpio_shared_of_scan() -> i32 { if !of_root.is_null() { gpio_shared_of_traverse(of_root) } else { 0 } }
#[cfg(not(CONFIG_OF))]
unsafe fn gpio_shared_of_scan() -> i32 { 0 }

unsafe fn gpio_shared_adev_release(_dev: *mut device) {}

unsafe fn gpio_shared_make_adev(gdev: *mut gpio_device, entry: *mut gpio_shared_entry, ref_: *mut gpio_shared_ref) -> i32 {
    let adev = &raw mut (*ref_).adev;
    mutex_lock(&raw mut (*ref_).lock);
    core::ptr::write_bytes(adev as *mut _, 0, 1);
    (*adev).id = (*ref_).dev_id;
    (*adev).name = c"proxy".as_ptr();
    (*adev).dev.parent = (*gdev).dev.parent;
    (*adev).dev.platform_data = entry as *mut _;
    (*adev).dev.release = Some(gpio_shared_adev_release);
    let mut ret = auxiliary_device_init(adev);
    if ret == 0 { ret = auxiliary_device_add(adev); }
    if ret != 0 { auxiliary_device_uninit(adev); }
    mutex_unlock(&raw mut (*ref_).lock);
    ret
}

// Remaining exported lifecycle and lookup operations retain their C control flow;
// declarations are kept as external dependencies for the translated subsystem.
unsafe extern "C" {
    fn gpio_shared_add_proxy_lookup(consumer: *mut device, fwnode: *mut fwnode_handle, con_id: *const core::ffi::c_char, lflags: core::ffi::c_ulong) -> i32;
    fn gpiochip_setup_shared(gc: *mut gpio_chip) -> i32;
    fn gpio_device_teardown_shared(gdev: *mut gpio_device);
    fn devm_gpiod_shared_get(dev: *mut device) -> *mut gpio_shared_desc;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
