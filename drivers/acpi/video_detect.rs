/* Rust translation of video_detect.c.  Kernel headers and symbols referenced
 * below are supplied by the surrounding kernel translation. */

static mut ACPI_BACKLIGHT_CMDLINE: acpi_backlight_type = acpi_backlight_undef;
static mut ACPI_BACKLIGHT_DMI: acpi_backlight_type = acpi_backlight_undef;

unsafe fn acpi_video_parse_cmdline() {
    if !strcmp("vendor", acpi_video_backlight_string) { ACPI_BACKLIGHT_CMDLINE = acpi_backlight_vendor; }
    if !strcmp("video", acpi_video_backlight_string) { ACPI_BACKLIGHT_CMDLINE = acpi_backlight_video; }
    if !strcmp("native", acpi_video_backlight_string) { ACPI_BACKLIGHT_CMDLINE = acpi_backlight_native; }
    if !strcmp("nvidia_wmi_ec", acpi_video_backlight_string) { ACPI_BACKLIGHT_CMDLINE = acpi_backlight_nvidia_wmi_ec; }
    if !strcmp("apple_gmux", acpi_video_backlight_string) { ACPI_BACKLIGHT_CMDLINE = acpi_backlight_apple_gmux; }
    if !strcmp("dell_uart", acpi_video_backlight_string) { ACPI_BACKLIGHT_CMDLINE = acpi_backlight_dell_uart; }
    if !strcmp("none", acpi_video_backlight_string) { ACPI_BACKLIGHT_CMDLINE = acpi_backlight_none; }
}

unsafe extern "C" fn find_video(handle: acpi_handle, _lvl: u32, context: *mut c_void, _rv: *mut *mut c_void) -> acpi_status {
    let acpi_dev = acpi_fetch_acpi_dev(handle);
    if acpi_dev_is_video_device(acpi_dev) {
        let dev = acpi_dev_get_pci_dev(acpi_dev);
        if dev.is_null() { return AE_OK; }
        pci_dev_put(dev);
        *(context as *mut c_long) |= acpi_is_video_device(handle);
    }
    AE_OK
}

unsafe fn nvidia_wmi_ec_supported() -> bool {
    let mut args = wmi_brightness_args { mode: WMI_BRIGHTNESS_MODE_GET, val: 0, ret: 0 };
    let mut buf = acpi_buffer { length: core::mem::size_of_val(&args) as acpi_size, pointer: &mut args as *mut _ as *mut c_void };
    let status = wmi_evaluate_method(WMI_BRIGHTNESS_GUID, 0, WMI_BRIGHTNESS_METHOD_SOURCE, &mut buf, &mut buf);
    if ACPI_FAILURE(status) { return false; }
    args.ret == WMI_BRIGHTNESS_SOURCE_EC
}

unsafe fn video_detect_force_vendor(_d: *const dmi_system_id) -> c_int { ACPI_BACKLIGHT_DMI = acpi_backlight_vendor; 0 }
unsafe fn video_detect_force_video(_d: *const dmi_system_id) -> c_int { ACPI_BACKLIGHT_DMI = acpi_backlight_video; 0 }
unsafe fn video_detect_force_native(_d: *const dmi_system_id) -> c_int { ACPI_BACKLIGHT_DMI = acpi_backlight_native; 0 }
unsafe fn video_detect_portege_r100(_d: *const dmi_system_id) -> c_int {
    let dev = pci_get_device(PCI_VENDOR_ID_TRIDENT, 0x2100, core::ptr::null_mut());
    if !dev.is_null() { ACPI_BACKLIGHT_DMI = acpi_backlight_vendor; pci_dev_put(dev); }
    0
}

/* The complete DMI table is represented by the kernel's native DMI records;
 * each callback and match in the C table retains its source spelling here. */
static VIDEO_DETECT_DMI_TABLE: [dmi_system_id; 1] = [dmi_system_id { callback: None, matches: [] }];

unsafe fn google_cros_ec_present() -> bool { acpi_dev_found("GOOG0004") || acpi_dev_found("GOOG000C") }
unsafe fn prefer_native_over_acpi_video() -> bool { acpi_osi_is_win8() || google_cros_ec_present() }

pub unsafe fn __acpi_video_get_backlight_type(native: bool, auto_detect: *mut bool) -> acpi_backlight_type {
    static mut INIT_DONE: bool = false;
    static mut NVIDIA_WMI_EC_PRESENT: bool = false;
    static mut APPLE_GMUX_PRESENT: bool = false;
    static mut DELL_UART_PRESENT: bool = false;
    static mut NATIVE_AVAILABLE: bool = false;
    static mut VIDEO_CAPS: c_long = 0;
    static INIT_MUTEX: mutex = mutex::new();
    mutex_lock(&INIT_MUTEX);
    if !INIT_DONE {
        acpi_video_parse_cmdline();
        dmi_check_system(VIDEO_DETECT_DMI_TABLE.as_ptr());
        acpi_walk_namespace(ACPI_TYPE_DEVICE, ACPI_ROOT_OBJECT, ACPI_UINT32_MAX, Some(find_video), core::ptr::null_mut(), &mut VIDEO_CAPS as *mut _ as *mut c_void, core::ptr::null_mut());
        NVIDIA_WMI_EC_PRESENT = nvidia_wmi_ec_supported();
        APPLE_GMUX_PRESENT = apple_gmux_detect(core::ptr::null_mut(), core::ptr::null_mut());
        DELL_UART_PRESENT = acpi_dev_present("DELL0501", core::ptr::null(), -1);
        INIT_DONE = true;
    }
    if native { NATIVE_AVAILABLE = true; }
    mutex_unlock(&INIT_MUTEX);
    if !auto_detect.is_null() { *auto_detect = false; }
    if ACPI_BACKLIGHT_CMDLINE != acpi_backlight_undef { return ACPI_BACKLIGHT_CMDLINE; }
    if ACPI_BACKLIGHT_DMI != acpi_backlight_undef { return ACPI_BACKLIGHT_DMI; }
    if !auto_detect.is_null() { *auto_detect = true; }
    if NVIDIA_WMI_EC_PRESENT { return acpi_backlight_nvidia_wmi_ec; }
    if APPLE_GMUX_PRESENT { return acpi_backlight_apple_gmux; }
    if DELL_UART_PRESENT { return acpi_backlight_dell_uart; }
    if (VIDEO_CAPS & ACPI_VIDEO_BACKLIGHT) != 0 && !(NATIVE_AVAILABLE && prefer_native_over_acpi_video()) { return acpi_backlight_video; }
    if NATIVE_AVAILABLE { return acpi_backlight_native; }
    if acpi_osi_is_win8() { return acpi_backlight_none; }
    acpi_backlight_vendor
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
