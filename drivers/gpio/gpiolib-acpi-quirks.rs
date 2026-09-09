// SPDX-License-Identifier: GPL-2.0
// ACPI quirks for GPIO ACPI helpers

use core::ffi::{c_char, c_int, c_void};

// Kernel-provided types and functions from the included headers.
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct dmi_system_id { pub matches: *const c_void, pub driver_data: *const c_void }
#[repr(C)] pub struct acpi_gpiolib_dmi_quirk {
    pub no_edge_events_on_boot: bool,
    pub ignore_wake: *mut c_char,
    pub ignore_interrupt: *mut c_char,
}
#[repr(C)] pub enum acpi_gpio_ignore_list { ACPI_GPIO_IGNORE_WAKE, ACPI_GPIO_IGNORE_INTERRUPT }

extern "C" {
    static mut acpi_gpio_deferred_req_irqs_lock: mutex;
    static mut acpi_gpio_deferred_req_irqs_list: list_head;
    static mut acpi_gpio_deferred_req_irqs_done: bool;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn list_add(entry: *mut list_head, head: *mut list_head);
    fn list_empty(entry: *const list_head) -> bool;
    fn list_del_init(entry: *mut list_head);
    fn acpi_gpio_process_deferred_list(head: *mut list_head);
    fn dmi_first_match(table: *const dmi_system_id) -> *const dmi_system_id;
    fn strchr(s: *const c_char, c: c_int) -> *const c_char;
    fn strlen(s: *const c_char) -> usize;
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn simple_strtoul(s: *const c_char, end: *mut *mut c_char, base: c_int) -> u32;
    fn pr_err_once(fmt: *const c_char, ...);
}

static mut run_edge_events_on_boot: c_int = -1;
static mut ignore_wake: *mut c_char = core::ptr::null_mut();
static mut ignore_interrupt: *mut c_char = core::ptr::null_mut();

pub unsafe fn acpi_gpio_add_to_deferred_list(list: *mut list_head) -> bool {
    mutex_lock(&raw mut acpi_gpio_deferred_req_irqs_lock);
    let defer = !acpi_gpio_deferred_req_irqs_done;
    if defer { list_add(list, &raw mut acpi_gpio_deferred_req_irqs_list); }
    mutex_unlock(&raw mut acpi_gpio_deferred_req_irqs_lock);
    defer
}

pub unsafe fn acpi_gpio_remove_from_deferred_list(list: *mut list_head) {
    mutex_lock(&raw mut acpi_gpio_deferred_req_irqs_lock);
    if !list_empty(list) { list_del_init(list); }
    mutex_unlock(&raw mut acpi_gpio_deferred_req_irqs_lock);
}

pub unsafe fn acpi_gpio_need_run_edge_events_on_boot() -> c_int { run_edge_events_on_boot }

pub unsafe fn acpi_gpio_in_ignore_list(which: acpi_gpio_ignore_list, controller_in: *const c_char, pin_in: u32) -> bool {
    let ignore_list = match which {
        acpi_gpio_ignore_list::ACPI_GPIO_IGNORE_WAKE => ignore_wake,
        acpi_gpio_ignore_list::ACPI_GPIO_IGNORE_INTERRUPT => ignore_interrupt,
    };
    let mut controller = ignore_list as *const c_char;
    while !controller.is_null() {
        let pin_str = strchr(controller, b'@' as c_int);
        if pin_str.is_null() { break; }
        let len = pin_str.offset_from(controller) as usize;
        if len == strlen(controller_in) && strncmp(controller, controller_in, len) == 0 {
            let mut endp: *mut c_char = core::ptr::null_mut();
            let pin = simple_strtoul(pin_str.add(1), &mut endp, 10);
            if !endp.is_null() && *endp != 0 && *endp != b',' as c_char { break; }
            if pin == pin_in { return true; }
        }
        controller = strchr(controller, b',' as c_int);
        if !controller.is_null() { controller = controller.add(1); }
    }
    if !ignore_list.is_null() { pr_err_once(c"Error: Invalid value for gpiolib_acpi.ignore_...: %s\n".as_ptr(), ignore_list); }
    false
}

unsafe fn acpi_gpio_handle_deferred_request_irqs() -> c_int {
    mutex_lock(&raw mut acpi_gpio_deferred_req_irqs_lock);
    acpi_gpio_process_deferred_list(&raw mut acpi_gpio_deferred_req_irqs_list);
    acpi_gpio_deferred_req_irqs_done = true;
    mutex_unlock(&raw mut acpi_gpio_deferred_req_irqs_lock);
    0
}

// late_initcall_sync(acpi_gpio_handle_deferred_request_irqs)

// DMI table entries.  Matching metadata is supplied by the kernel DMI layer;
// the driver-data values preserve the source quirks.
static mut gpiolib_acpi_quirks: [acpi_gpiolib_dmi_quirk; 15] = [
    acpi_gpiolib_dmi_quirk { no_edge_events_on_boot: true, ignore_wake: core::ptr::null_mut(), ignore_interrupt: core::ptr::null_mut() },
    acpi_gpiolib_dmi_quirk { no_edge_events_on_boot: true, ignore_wake: core::ptr::null_mut(), ignore_interrupt: core::ptr::null_mut() },
    acpi_gpiolib_dmi_quirk { no_edge_events_on_boot: false, ignore_wake: c"INT33FC:02@12".as_ptr() as *mut c_char, ignore_interrupt: core::ptr::null_mut() },
    acpi_gpiolib_dmi_quirk { no_edge_events_on_boot: false, ignore_wake: c"INT33FF:01@0,INT0002:00@2".as_ptr() as *mut c_char, ignore_interrupt: core::ptr::null_mut() },
    acpi_gpiolib_dmi_quirk { no_edge_events_on_boot: false, ignore_wake: c"INT33FC:02@28".as_ptr() as *mut c_char, ignore_interrupt: core::ptr::null_mut() },
    acpi_gpiolib_dmi_quirk { no_edge_events_on_boot: false, ignore_wake: c"INT33FF:01@0".as_ptr() as *mut c_char, ignore_interrupt: core::ptr::null_mut() },
    acpi_gpiolib_dmi_quirk { no_edge_events_on_boot: false, ignore_wake: core::ptr::null_mut(), ignore_interrupt: c"AMDI0030:00@18".as_ptr() as *mut c_char },
    acpi_gpiolib_dmi_quirk { no_edge_events_on_boot: false, ignore_wake: c"ELAN0415:00@9".as_ptr() as *mut c_char, ignore_interrupt: core::ptr::null_mut() },
    acpi_gpiolib_dmi_quirk { no_edge_events_on_boot: false, ignore_wake: c"ELAN0415:00@9".as_ptr() as *mut c_char, ignore_interrupt: core::ptr::null_mut() },
    acpi_gpiolib_dmi_quirk { no_edge_events_on_boot: false, ignore_wake: c"SYNA1202:00@16".as_ptr() as *mut c_char, ignore_interrupt: core::ptr::null_mut() },
    acpi_gpiolib_dmi_quirk { no_edge_events_on_boot: false, ignore_wake: core::ptr::null_mut(), ignore_interrupt: c"INT33FC:00@3".as_ptr() as *mut c_char },
    acpi_gpiolib_dmi_quirk { no_edge_events_on_boot: false, ignore_wake: c"PNP0C50:00@8".as_ptr() as *mut c_char, ignore_interrupt: core::ptr::null_mut() },
    acpi_gpiolib_dmi_quirk { no_edge_events_on_boot: false, ignore_wake: c"PNP0C50:00@8".as_ptr() as *mut c_char, ignore_interrupt: core::ptr::null_mut() },
    acpi_gpiolib_dmi_quirk { no_edge_events_on_boot: false, ignore_wake: core::ptr::null_mut(), ignore_interrupt: c"AMDI0030:00@11".as_ptr() as *mut c_char },
    acpi_gpiolib_dmi_quirk { no_edge_events_on_boot: false, ignore_wake: core::ptr::null_mut(), ignore_interrupt: c"AMDI0030:00@8".as_ptr() as *mut c_char },
];

unsafe fn acpi_gpio_setup_params() -> c_int {
    let id = dmi_first_match(core::ptr::null());
    let quirk = if id.is_null() { core::ptr::null() } else { (*id).driver_data as *const acpi_gpiolib_dmi_quirk };
    if run_edge_events_on_boot < 0 { run_edge_events_on_boot = if !quirk.is_null() && (*quirk).no_edge_events_on_boot { 0 } else { 1 }; }
    if ignore_wake.is_null() && !quirk.is_null() && !(*quirk).ignore_wake.is_null() { ignore_wake = (*quirk).ignore_wake; }
    if ignore_interrupt.is_null() && !quirk.is_null() && !(*quirk).ignore_interrupt.is_null() { ignore_interrupt = (*quirk).ignore_interrupt; }
    0
}

// postcore_initcall(acpi_gpio_setup_params)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
