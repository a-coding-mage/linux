// SPDX-License-Identifier: GPL-2.0
/*
 * Device physical location support
 *
 * Author: Won Chung <wonchung@google.com>
 */

// Dependencies supplied by the corresponding kernel headers and physical_location.h.

pub unsafe fn dev_add_physical_location(dev: *mut device) -> bool {
    let mut pld: *mut acpi_pld_info = core::ptr::null_mut();

    if !has_acpi_companion(dev) {
        return false;
    }

    if !acpi_get_physical_device_location(ACPI_HANDLE(dev), &mut pld) {
        return false;
    }

    (*dev).physical_location = kzalloc_obj::<physical_location>();
    if (*dev).physical_location.is_null() {
        ACPI_FREE(pld);
        return false;
    }

    (*(*dev).physical_location).panel = (*pld).panel;
    (*(*dev).physical_location).vertical_position = (*pld).vertical_position;
    (*(*dev).physical_location).horizontal_position = (*pld).horizontal_position;
    (*(*dev).physical_location).dock = (*pld).dock;
    (*(*dev).physical_location).lid = (*pld).lid;

    ACPI_FREE(pld);
    true
}

unsafe fn panel_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut core::ffi::c_char) -> ssize_t {
    let panel = match (*(*dev).physical_location).panel {
        DEVICE_PANEL_TOP => "top",
        DEVICE_PANEL_BOTTOM => "bottom",
        DEVICE_PANEL_LEFT => "left",
        DEVICE_PANEL_RIGHT => "right",
        DEVICE_PANEL_FRONT => "front",
        DEVICE_PANEL_BACK => "back",
        _ => "unknown",
    };
    sysfs_emit(buf, "%s\n", panel)
}
static DEVICE_ATTR_RO!(panel);

unsafe fn vertical_position_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut core::ffi::c_char) -> ssize_t {
    let vertical_position = match (*(*dev).physical_location).vertical_position {
        DEVICE_VERT_POS_UPPER => "upper",
        DEVICE_VERT_POS_CENTER => "center",
        DEVICE_VERT_POS_LOWER => "lower",
        _ => "unknown",
    };
    sysfs_emit(buf, "%s\n", vertical_position)
}
static DEVICE_ATTR_RO!(vertical_position);

unsafe fn horizontal_position_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut core::ffi::c_char) -> ssize_t {
    let horizontal_position = match (*(*dev).physical_location).horizontal_position {
        DEVICE_HORI_POS_LEFT => "left",
        DEVICE_HORI_POS_CENTER => "center",
        DEVICE_HORI_POS_RIGHT => "right",
        _ => "unknown",
    };
    sysfs_emit(buf, "%s\n", horizontal_position)
}
static DEVICE_ATTR_RO!(horizontal_position);

unsafe fn dock_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut core::ffi::c_char) -> ssize_t {
    sysfs_emit(buf, "%s\n", str_yes_no((*(*dev).physical_location).dock))
}
static DEVICE_ATTR_RO!(dock);

unsafe fn lid_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut core::ffi::c_char) -> ssize_t {
    sysfs_emit(buf, "%s\n", str_yes_no((*(*dev).physical_location).lid))
}
static DEVICE_ATTR_RO!(lid);

static mut dev_attr_physical_location: [*mut attribute; 6] = [
    &mut dev_attr_panel.attr,
    &mut dev_attr_vertical_position.attr,
    &mut dev_attr_horizontal_position.attr,
    &mut dev_attr_dock.attr,
    &mut dev_attr_lid.attr,
    core::ptr::null_mut(),
];

pub static dev_attr_physical_location_group: attribute_group = attribute_group {
    name: "physical_location",
    attrs: unsafe { &mut dev_attr_physical_location as *mut [*mut attribute; 6] as *mut *mut attribute },
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
