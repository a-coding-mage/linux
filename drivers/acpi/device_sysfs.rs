// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of drivers/acpi/device_sysfs.c. */

// Kernel headers and macros are supplied by the surrounding translation unit.

unsafe fn acpi_object_path(handle: acpi_handle, buf: *mut c_char) -> ssize_t {
    let mut path = acpi_buffer { length: ACPI_ALLOCATE_BUFFER, pointer: core::ptr::null_mut() };
    let mut result = acpi_get_name(handle, ACPI_FULL_PATHNAME, &mut path);
    if result != 0 { return result as ssize_t; }
    result = sysfs_emit(buf, c_str!("%s\n"), path.pointer as *mut c_char);
    kfree(path.pointer);
    result as ssize_t
}

#[repr(C)]
struct acpi_data_node_attr {
    attr: attribute,
    show: Option<unsafe extern "C" fn(*mut acpi_data_node, *mut c_char) -> ssize_t>,
    store: Option<unsafe extern "C" fn(*mut acpi_data_node, *const c_char, usize) -> ssize_t>,
}

static mut data_node_path: acpi_data_node_attr = acpi_data_node_attr {
    attr: __ATTR!(path, 0o444, data_node_show_path, None), show: Some(data_node_show_path), store: None,
};

unsafe extern "C" fn data_node_show_path(dn: *mut acpi_data_node, buf: *mut c_char) -> ssize_t {
    if !(*dn).handle.is_null() { acpi_object_path((*dn).handle, buf) } else { 0 }
}

static mut acpi_data_node_default_attrs: [*mut attribute; 2] = [
    unsafe { &mut data_node_path.attr }, core::ptr::null_mut(),
];
// ATTRIBUTE_GROUPS(acpi_data_node_default)

unsafe extern "C" fn acpi_data_node_attr_show(kobj: *mut kobject, attr: *mut attribute, buf: *mut c_char) -> ssize_t {
    let dn = container_of!(kobj, acpi_data_node, kobj);
    let dn_attr = container_of!(attr, acpi_data_node_attr, attr);
    match (*dn_attr).show { Some(show) => show(dn, buf), None => -ENXIO as ssize_t }
}

static acpi_data_node_sysfs_ops: sysfs_ops = sysfs_ops { show: Some(acpi_data_node_attr_show), store: None };

unsafe extern "C" fn acpi_data_node_release(kobj: *mut kobject) {
    let dn = container_of!(kobj, acpi_data_node, kobj);
    complete(&mut (*dn).kobj_done);
}

static acpi_data_node_ktype: kobj_type = kobj_type {
    sysfs_ops: &acpi_data_node_sysfs_ops,
    default_groups: acpi_data_node_default_groups,
    release: Some(acpi_data_node_release),
};

unsafe fn acpi_expose_nondev_subnodes(kobj: *mut kobject, data: *mut acpi_device_data) {
    let list = &mut (*data).subnodes;
    if list_empty(list) { return; }
    list_for_each_entry!(dn, list, sibling, {
        init_completion(&mut (*dn).kobj_done);
        let ret = kobject_init_and_add(&mut (*dn).kobj, &acpi_data_node_ktype, kobj, c_str!("%s"), (*dn).name);
        if ret == 0 { acpi_expose_nondev_subnodes(&mut (*dn).kobj, &mut (*dn).data); }
        else if !(*dn).handle.is_null() { acpi_handle_err((*dn).handle, c_str!("Failed to expose (%d)\n"), ret); }
    });
}

unsafe fn acpi_hide_nondev_subnodes(data: *mut acpi_device_data) {
    let list = &mut (*data).subnodes;
    if list_empty(list) { return; }
    list_for_each_entry_reverse!(dn, list, sibling, {
        acpi_hide_nondev_subnodes(&mut (*dn).data);
        kobject_put(&mut (*dn).kobj);
    });
}

unsafe fn create_pnp_modalias(dev: *const acpi_device, modalias: *mut c_char, mut size: i32) -> i32 {
    if !acpi_device_is_present(dev) { return 0; }
    let mut count = 0;
    list_for_each_entry!(id, &(*dev).pnp.ids, list, { if strcmp((*id).id, ACPI_DT_NAMESPACE_HID) != 0 { count += 1; } });
    if count == 0 { return 0; }
    let mut len = snprintf(modalias, size, c_str!("acpi:"));
    if len >= size { return -ENOMEM; }
    size -= len;
    list_for_each_entry!(id, &(*dev).pnp.ids, list, {
        if strcmp((*id).id, ACPI_DT_NAMESPACE_HID) != 0 {
            count = snprintf(modalias.add(len as usize), size, c_str!("%s:"), (*id).id);
            if count >= size { return -ENOMEM; }
            len += count; size -= count;
        }
    });
    len
}

unsafe fn create_of_modalias(dev: *const acpi_device, modalias: *mut c_char, mut size: i32) -> i32 {
    let mut buf = acpi_buffer { length: ACPI_ALLOCATE_BUFFER, pointer: core::ptr::null_mut() };
    let status = acpi_get_name((*dev).handle, ACPI_SINGLE_NAME, &mut buf);
    if ACPI_FAILURE(status) { return -ENODEV; }
    let mut c = buf.pointer as *mut u8;
    while *c != 0 { *c = (*c as char).to_ascii_lowercase() as u8; c = c.add(1); }
    let mut len = snprintf(modalias, size, c_str!("of:N%sT"), buf.pointer);
    ACPI_FREE(buf.pointer);
    if len >= size { return -ENOMEM; }
    size -= len;
    let compat = (*dev).data.of_compatible;
    let (nval, mut obj) = if (*compat).type_ == ACPI_TYPE_PACKAGE { ((*compat).package.count, (*compat).package.elements) } else { (1, compat) };
    for _ in 0..nval {
        let count = snprintf(modalias.add(len as usize), size, c_str!("C%s"), (*obj).string.pointer);
        if count >= size { return -ENOMEM; }
        len += count; size -= count; obj = obj.add(1);
    }
    len
}

pub unsafe fn __acpi_device_uevent_modalias(adev: *const acpi_device, env: *mut kobj_uevent_env) -> i32 {
    if adev.is_null() { return -ENODEV; }
    if list_empty(&(*adev).pnp.ids) { return 0; }
    if add_uevent_var(env, c_str!("MODALIAS=")) != 0 { return -ENOMEM; }
    let len = if !(*adev).data.of_compatible.is_null() {
        create_of_modalias(adev, (*env).buf.as_mut_ptr().add((*env).buflen - 1), (core::mem::size_of_val(&(*env).buf) - (*env).buflen) as i32)
    } else { create_pnp_modalias(adev, (*env).buf.as_mut_ptr().add((*env).buflen - 1), (core::mem::size_of_val(&(*env).buf) - (*env).buflen) as i32) };
    if len < 0 { return len; } (*env).buflen += len as usize; 0
}

pub unsafe fn acpi_device_uevent_modalias(dev: *const device, env: *mut kobj_uevent_env) -> i32 { __acpi_device_uevent_modalias(acpi_companion_match(dev), env) }

unsafe fn __acpi_device_modalias(adev: *const acpi_device, buf: *mut c_char, mut size: i32) -> i32 {
    if adev.is_null() { return -ENODEV; } if list_empty(&(*adev).pnp.ids) { return 0; }
    let mut len = create_pnp_modalias(adev, buf, size - 1); if len < 0 { return len; } else if len > 0 { *buf.add(len as usize) = b'\n' as c_char; len += 1; size -= len; }
    if (*adev).data.of_compatible.is_null() { return len; }
    let count = create_of_modalias(adev, buf.add(len as usize), size - 1); if count < 0 { return count; } else if count > 0 { len += count; *buf.add(len as usize) = b'\n' as c_char; len += 1; } len
}

pub unsafe fn acpi_device_modalias(dev: *mut device, buf: *mut c_char, size: i32) -> i32 { __acpi_device_modalias(acpi_companion_match(dev), buf, size) }

unsafe extern "C" fn modalias_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { __acpi_device_modalias(to_acpi_device(dev), buf, 1024) as ssize_t }
unsafe extern "C" fn real_power_state_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { let adev = to_acpi_device(dev); let mut state = 0; let ret = acpi_device_get_power(adev, &mut state); if ret != 0 { ret as ssize_t } else { sysfs_emit(buf, c_str!("%s\n"), acpi_power_state_string(state)) as ssize_t } }
unsafe extern "C" fn power_state_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { sysfs_emit(buf, c_str!("%s\n"), acpi_power_state_string((*to_acpi_device(dev)).power.state)) as ssize_t }
unsafe extern "C" fn eject_store(d: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: usize) -> ssize_t { let adev = to_acpi_device(d); if count == 0 || *buf != b'1' as c_char { return -EINVAL as ssize_t; } if ((*adev).handler.is_null() || !(*(*adev).handler).hotplug.enabled) && (*d).driver.is_null() { return -ENODEV as ssize_t; } let mut unused = 0; let mut status = acpi_get_type((*adev).handle, &mut unused); if ACPI_FAILURE(status) || !(*adev).flags.ejectable { return -ENODEV as ssize_t; } acpi_dev_get(adev); status = acpi_hotplug_schedule(adev, ACPI_OST_EC_OSPM_EJECT); if ACPI_SUCCESS(status) { return count as ssize_t; } acpi_dev_put(adev); acpi_evaluate_ost((*adev).handle, ACPI_OST_EC_OSPM_EJECT, ACPI_OST_SC_NON_SPECIFIC_FAILURE, core::ptr::null_mut()); if status == AE_NO_MEMORY { -ENOMEM as ssize_t } else { -EAGAIN as ssize_t } }
unsafe extern "C" fn hid_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { sysfs_emit(buf, c_str!("%s\n"), acpi_device_hid(to_acpi_device(dev))) as ssize_t }
unsafe extern "C" fn cid_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { let adev = to_acpi_device(dev); let mut info = core::ptr::null_mut(); let mut len = 0; acpi_get_object_info((*adev).handle, &mut info); if info.is_null() { return 0; } if (*info).valid & ACPI_VALID_CID != 0 { let list = &(*info).compatible_id_list; for i in 0..(list.count - 1) { len += sysfs_emit_at(buf, len, c_str!("%s,"), list.ids[i].string); } len += sysfs_emit_at(buf, len, c_str!("%s\n"), list.ids[list.count - 1].string); } kfree(info); len as ssize_t }
unsafe extern "C" fn uid_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { sysfs_emit(buf, c_str!("%s\n"), acpi_device_uid(to_acpi_device(dev))) as ssize_t }
unsafe extern "C" fn adr_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { let adr = (*to_acpi_device(dev)).pnp.bus_address; if adr > U32_MAX as u64 { sysfs_emit(buf, c_str!("0x%016llx\n"), adr) as ssize_t } else { sysfs_emit(buf, c_str!("0x%08llx\n"), adr) as ssize_t } }
unsafe extern "C" fn path_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { acpi_object_path((*to_acpi_device(dev)).handle, buf) }
unsafe extern "C" fn description_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { let adev = to_acpi_device(dev); let mut buffer = acpi_buffer { length: ACPI_ALLOCATE_BUFFER, pointer: core::ptr::null_mut() }; let status = acpi_evaluate_object_typed((*adev).handle, c_str!("_STR"), core::ptr::null_mut(), &mut buffer, ACPI_TYPE_BUFFER); if ACPI_FAILURE(status) { return -EIO as ssize_t; } let obj = buffer.pointer as *mut acpi_object; let result = utf16s_to_utf8s((*obj).buffer.pointer as *mut wchar_t, (*obj).buffer.length, UTF16_LITTLE_ENDIAN, buf, PAGE_SIZE - 1); *buf.add(result as usize) = b'\n' as c_char; ACPI_FREE(obj); (result + 1) as ssize_t }
unsafe extern "C" fn sun_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { let mut sun = 0; if ACPI_FAILURE(acpi_evaluate_integer((*to_acpi_device(dev)).handle, c_str!("_SUN"), core::ptr::null_mut(), &mut sun)) { -EIO as ssize_t } else { sysfs_emit(buf, c_str!("%llu\n"), sun) as ssize_t } }
unsafe extern "C" fn hrv_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { let mut hrv = 0; if ACPI_FAILURE(acpi_evaluate_integer((*to_acpi_device(dev)).handle, c_str!("_HRV"), core::ptr::null_mut(), &mut hrv)) { -EIO as ssize_t } else { sysfs_emit(buf, c_str!("%llu\n"), hrv) as ssize_t } }
unsafe extern "C" fn status_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { let mut sta = 0; if ACPI_FAILURE(acpi_evaluate_integer((*to_acpi_device(dev)).handle, c_str!("_STA"), core::ptr::null_mut(), &mut sta)) { -EIO as ssize_t } else { sysfs_emit(buf, c_str!("%llu\n"), sta) as ssize_t } }

unsafe fn acpi_show_attr(dev: *mut acpi_device, attr: *const device_attribute) -> bool {
    if attr == &dev_attr_path { return !(*dev).handle.is_null(); }
    if attr == &dev_attr_hid || attr == &dev_attr_modalias { return !list_empty(&(*dev).pnp.ids); }
    if attr == &dev_attr_description { return acpi_has_method((*dev).handle, c_str!("_STR")); }
    if attr == &dev_attr_adr { return (*dev).pnp.type_.bus_address != 0; }
    if attr == &dev_attr_uid { return !acpi_device_uid(dev).is_null(); }
    if attr == &dev_attr_sun { return acpi_has_method((*dev).handle, c_str!("_SUN")); }
    if attr == &dev_attr_hrv { return acpi_has_method((*dev).handle, c_str!("_HRV")); }
    if attr == &dev_attr_status { return acpi_has_method((*dev).handle, c_str!("_STA")); }
    if attr == &dev_attr_cid { return acpi_has_method((*dev).handle, c_str!("_CID")); }
    if attr == &dev_attr_eject { return acpi_has_method((*dev).handle, c_str!("_EJ0")); }
    if attr == &dev_attr_power_state { return (*dev).flags.power_manageable; }
    if attr == &dev_attr_real_power_state { return (*dev).flags.power_manageable && (*dev).power.flags.power_resources; }
    dev_warn_once!(&(*dev).dev, c_str!("Unexpected attribute: %s\n"), (*attr).attr.name); false
}

unsafe extern "C" fn acpi_attr_is_visible(kobj: *mut kobject, attr: *mut attribute, _attrno: i32) -> umode_t {
    let dev = to_acpi_device(kobj_to_dev(kobj));
    let da = container_of!(attr, device_attribute, attr);
    if acpi_show_attr(dev, da) { (*attr).mode } else { 0 }
}

static mut acpi_group: attribute_group = attribute_group { attrs: acpi_attrs, is_visible: Some(acpi_attr_is_visible) };
#[no_mangle]
pub static mut acpi_groups: [*const attribute_group; 2] = [&acpi_group, core::ptr::null()];

pub unsafe fn acpi_device_setup_files(dev: *mut acpi_device) { acpi_expose_nondev_subnodes(&mut (*dev).dev.kobj, &mut (*dev).data); }
pub unsafe fn acpi_device_remove_files(dev: *mut acpi_device) { acpi_hide_nondev_subnodes(&mut (*dev).data); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
