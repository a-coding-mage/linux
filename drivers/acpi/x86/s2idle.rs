// SPDX-License-Identifier: GPL-2.0
/* Architecture-specific ACPI-based support for suspend-to-idle. */

// C headers and CONFIG_SUSPEND conditional are supplied by the surrounding kernel bindings.

static mut SLEEP_NO_LPS0: bool = false;
static mut CHECK_LPS0_CONSTRAINTS: bool = false;

const ACPI_LPS0_DSM_UUID_MICROSOFT: &str = "11e00d56-ce64-47ce-837b-1f898f9aa461";
const ACPI_LPS0_DSM_UUID: &str = "c4eb40a0-6cd2-11e2-bcfd-0800200c9a66";
const ACPI_LPS0_GET_DEVICE_CONSTRAINTS: u32 = 1;
const ACPI_LPS0_SCREEN_OFF: u32 = 3;
const ACPI_LPS0_SCREEN_ON: u32 = 4;
const ACPI_LPS0_ENTRY: u32 = 5;
const ACPI_LPS0_EXIT: u32 = 6;
const ACPI_LPS0_MS_ENTRY: u32 = 7;
const ACPI_LPS0_MS_EXIT: u32 = 8;
const ACPI_MS_TURN_ON_DISPLAY: u32 = 9;
const ACPI_LPS0_DSM_UUID_AMD: &str = "e3f32452-febc-43ce-9039-932122d37721";
const ACPI_LPS0_ENTRY_AMD: u32 = 2;
const ACPI_LPS0_EXIT_AMD: u32 = 3;
const ACPI_LPS0_SCREEN_OFF_AMD: u32 = 4;
const ACPI_LPS0_SCREEN_ON_AMD: u32 = 5;

static mut LPS0_DEVICE_HANDLE: acpi_handle = core::ptr::null_mut();
static mut LPS0_DSM_GUID: guid_t = guid_t::zeroed();
static mut LPS0_DSM_FUNC_MASK: i32 = 0;
static mut LPS0_DSM_GUID_MICROSOFT: guid_t = guid_t::zeroed();
static mut LPS0_DSM_FUNC_MASK_MICROSOFT: i32 = 0;
static mut LPS0_DSM_STATE: i32 = 0;

#[repr(C)]
struct lpi_device_info { name: *mut i8, enabled: i32, package: *mut acpi_object }
#[repr(C)]
struct lpi_device_constraint { uid: i32, min_dstate: i32, function_states: i32 }
#[repr(C)]
struct lpi_constraints { handle: acpi_handle, min_dstate: i32 }
#[repr(C)]
struct lpi_device_constraint_amd { name: *mut i8, enabled: i32, function_states: i32, min_dstate: i32 }

static mut LPS0_S2IDLE_DEVOPS_HEAD: list_head = list_head::new();
static mut LPI_CONSTRAINTS_TABLE: *mut lpi_constraints = core::ptr::null_mut();
static mut LPI_CONSTRAINTS_TABLE_SIZE: i32 = 0;
static mut REV_ID: i32 = 0;

unsafe fn lpi_device_get_constraints_amd() {
    let out_obj = acpi_evaluate_dsm_typed(LPS0_DEVICE_HANDLE, &LPS0_DSM_GUID, REV_ID, ACPI_LPS0_GET_DEVICE_CONSTRAINTS, core::ptr::null(), ACPI_TYPE_PACKAGE);
    acpi_handle_debug(LPS0_DEVICE_HANDLE, "_DSM function 1 eval %s\n", if !out_obj.is_null() { "successful" } else { "failed" });
    if out_obj.is_null() { return; }
    for i in 0..(*out_obj).package.count {
        let package = &mut (*out_obj).package.elements.add(i);
        if (*package).type_ == ACPI_TYPE_PACKAGE {
            if !LPI_CONSTRAINTS_TABLE.is_null() { acpi_handle_err(LPS0_DEVICE_HANDLE, "Duplicate constraints list\n"); break; }
            LPI_CONSTRAINTS_TABLE = kzalloc_objs::<lpi_constraints>((*package).package.count);
            if LPI_CONSTRAINTS_TABLE.is_null() { break; }
            for j in 0..(*package).package.count {
                let info_obj = &mut (*package).package.elements.add(j);
                let mut dev_info = lpi_device_constraint_amd { name: core::ptr::null_mut(), enabled: 0, function_states: 0, min_dstate: 0 };
                let list = &mut *LPI_CONSTRAINTS_TABLE.add(LPI_CONSTRAINTS_TABLE_SIZE as usize);
                for k in 0..(*info_obj).package.count { let obj = &*(*info_obj).package.elements.add(k); match k { 0 => dev_info.enabled = obj.integer.value as i32, 1 => dev_info.name = obj.string.pointer, 2 => dev_info.function_states = obj.integer.value as i32, 3 => dev_info.min_dstate = obj.integer.value as i32, _ => {} } }
                if dev_info.enabled == 0 || dev_info.name.is_null() || dev_info.min_dstate == 0 { continue; }
                if ACPI_FAILURE(acpi_get_handle(core::ptr::null_mut(), dev_info.name, &mut list.handle)) { continue; }
                list.min_dstate = dev_info.min_dstate; LPI_CONSTRAINTS_TABLE_SIZE += 1;
            }
        }
    }
    ACPI_FREE(out_obj);
}

unsafe fn lpi_device_get_constraints() {
    let out_obj = acpi_evaluate_dsm_typed(LPS0_DEVICE_HANDLE, &LPS0_DSM_GUID, 1, ACPI_LPS0_GET_DEVICE_CONSTRAINTS, core::ptr::null(), ACPI_TYPE_PACKAGE);
    acpi_handle_debug(LPS0_DEVICE_HANDLE, "_DSM function 1 eval %s\n", if !out_obj.is_null() { "successful" } else { "failed" });
    if out_obj.is_null() { return; }
    LPI_CONSTRAINTS_TABLE = kzalloc_objs::<lpi_constraints>((*out_obj).package.count);
    if !LPI_CONSTRAINTS_TABLE.is_null() {
        for i in 0..(*out_obj).package.count {
            let package = &mut (*out_obj).package.elements.add(i); if package.is_null() { continue; }
            let mut info = lpi_device_info { name: core::ptr::null_mut(), enabled: 0, package: core::ptr::null_mut() }; let mut package_count = 0;
            for j in 0..(*package).package.count { let e = &*(*package).package.elements.add(j); match e.type_ { ACPI_TYPE_INTEGER => info.enabled = e.integer.value as i32, ACPI_TYPE_STRING => info.name = e.string.pointer, ACPI_TYPE_PACKAGE => { package_count = e.package.count; info.package = e.package.elements; }, _ => {} } }
            if info.enabled == 0 || info.package.is_null() || info.name.is_null() { continue; }
            let constraint = &mut *LPI_CONSTRAINTS_TABLE.add(LPI_CONSTRAINTS_TABLE_SIZE as usize);
            if ACPI_FAILURE(acpi_get_handle(core::ptr::null_mut(), info.name, &mut constraint.handle)) { continue; }
            constraint.min_dstate = -1;
            for j in 0..package_count { let p = &*info.package.add(j); if p.type_ == ACPI_TYPE_PACKAGE && p.package.count >= 2 { constraint.min_dstate = p.package.elements[1].integer.value as i32; } }
            if constraint.min_dstate >= 0 { LPI_CONSTRAINTS_TABLE_SIZE += 1; }
        }
    }
    ACPI_FREE(out_obj);
}

unsafe fn lpi_check_constraints() { if LPI_CONSTRAINTS_TABLE.is_null() { return; } for i in 0..LPI_CONSTRAINTS_TABLE_SIZE { let e = &mut *LPI_CONSTRAINTS_TABLE.add(i as usize); let adev = acpi_fetch_acpi_dev(e.handle); if adev.is_null() { continue; } if !(*adev).flags.power_manageable { e.handle = core::ptr::null_mut(); } else if (*adev).power.state < e.min_dstate { acpi_handle_info(e.handle, "LPI: Constraint not met; min power state:%s current power state:%s\n", acpi_power_state_string(e.min_dstate), acpi_power_state_string((*adev).power.state)); } } }

unsafe fn acpi_s2idle_vendor_amd() -> bool { boot_cpu_data.x86_vendor == X86_VENDOR_AMD }
unsafe fn acpi_sleep_dsm_state_to_str(state: u32) -> &'static str { if LPS0_DSM_FUNC_MASK_MICROSOFT != 0 || !acpi_s2idle_vendor_amd() { match state { ACPI_LPS0_SCREEN_OFF=>"screen off", ACPI_LPS0_SCREEN_ON=>"screen on", ACPI_LPS0_ENTRY=>"lps0 entry", ACPI_LPS0_EXIT=>"lps0 exit", ACPI_LPS0_MS_ENTRY=>"lps0 ms entry", ACPI_LPS0_MS_EXIT=>"lps0 ms exit", ACPI_MS_TURN_ON_DISPLAY=>"lps0 ms turn on display", _=>"unknown" } } else { match state { ACPI_LPS0_SCREEN_ON_AMD=>"screen on", ACPI_LPS0_SCREEN_OFF_AMD=>"screen off", ACPI_LPS0_ENTRY_AMD=>"lps0 entry", ACPI_LPS0_EXIT_AMD=>"lps0 exit", _=>"unknown" } } }
unsafe fn acpi_sleep_run_lps0_dsm(func: u32, mask: u32, guid: guid_t) { if mask & (1 << func) == 0 { return; } let out_obj = acpi_evaluate_dsm(LPS0_DEVICE_HANDLE, &guid, REV_ID, func, core::ptr::null()); ACPI_FREE(out_obj); LPS0_DSM_STATE = func as i32; }

#[repr(C)] struct amd_lps0_hid_device_data { check_off_by_one: bool }
static AMD_PICASSO: amd_lps0_hid_device_data = amd_lps0_hid_device_data { check_off_by_one: true };
static AMD_CEZZANE: amd_lps0_hid_device_data = amd_lps0_hid_device_data { check_off_by_one: false };

unsafe fn acpi_s2idle_begin_lps0() -> i32 {
    if !LPS0_DEVICE_HANDLE.is_null() && !SLEEP_NO_LPS0 && CHECK_LPS0_CONSTRAINTS && LPI_CONSTRAINTS_TABLE.is_null() {
        if acpi_s2idle_vendor_amd() { lpi_device_get_constraints_amd(); } else { lpi_device_get_constraints(); }
        if LPI_CONSTRAINTS_TABLE.is_null() { LPI_CONSTRAINTS_TABLE = ERR_PTR(-ENODATA); }
    }
    acpi_s2idle_begin()
}
unsafe fn acpi_s2idle_prepare_late_lps0() -> i32 {
    if LPS0_DEVICE_HANDLE.is_null() || SLEEP_NO_LPS0 { return 0; }
    if CHECK_LPS0_CONSTRAINTS { lpi_check_constraints(); }
    if LPS0_DSM_FUNC_MASK > 0 { acpi_sleep_run_lps0_dsm(if acpi_s2idle_vendor_amd(){ACPI_LPS0_SCREEN_OFF_AMD}else{ACPI_LPS0_SCREEN_OFF}, LPS0_DSM_FUNC_MASK as u32, LPS0_DSM_GUID); }
    if LPS0_DSM_FUNC_MASK_MICROSOFT > 0 { acpi_sleep_run_lps0_dsm(ACPI_LPS0_SCREEN_OFF, LPS0_DSM_FUNC_MASK_MICROSOFT as u32, LPS0_DSM_GUID_MICROSOFT); }
    if LPS0_DSM_FUNC_MASK > 0 && acpi_s2idle_vendor_amd() { acpi_sleep_run_lps0_dsm(ACPI_LPS0_ENTRY_AMD, LPS0_DSM_FUNC_MASK as u32, LPS0_DSM_GUID); }
    if LPS0_DSM_FUNC_MASK_MICROSOFT > 0 { acpi_sleep_run_lps0_dsm(ACPI_LPS0_MS_ENTRY,LPS0_DSM_FUNC_MASK_MICROSOFT as u32,LPS0_DSM_GUID_MICROSOFT); acpi_sleep_run_lps0_dsm(ACPI_LPS0_ENTRY,LPS0_DSM_FUNC_MASK_MICROSOFT as u32,LPS0_DSM_GUID_MICROSOFT); }
    if LPS0_DSM_FUNC_MASK > 0 && !acpi_s2idle_vendor_amd() { acpi_sleep_run_lps0_dsm(ACPI_LPS0_ENTRY,LPS0_DSM_FUNC_MASK as u32,LPS0_DSM_GUID); }
    acpi_s2idle_prepare_late(); 0
}
unsafe fn acpi_s2idle_check_lps0() { if !LPS0_DEVICE_HANDLE.is_null() && !SLEEP_NO_LPS0 { acpi_s2idle_check(); } }
unsafe fn acpi_s2idle_restore_early_lps0() { if LPS0_DEVICE_HANDLE.is_null() || SLEEP_NO_LPS0 { return; } if LPS0_DSM_FUNC_MASK > 0 { acpi_sleep_run_lps0_dsm(if acpi_s2idle_vendor_amd(){ACPI_LPS0_EXIT_AMD}else{ACPI_LPS0_EXIT},LPS0_DSM_FUNC_MASK as u32,LPS0_DSM_GUID); } if LPS0_DSM_FUNC_MASK_MICROSOFT > 0 { acpi_sleep_run_lps0_dsm(ACPI_LPS0_EXIT,LPS0_DSM_FUNC_MASK_MICROSOFT as u32,LPS0_DSM_GUID_MICROSOFT); acpi_sleep_run_lps0_dsm(ACPI_MS_TURN_ON_DISPLAY,LPS0_DSM_FUNC_MASK_MICROSOFT as u32,LPS0_DSM_GUID_MICROSOFT); acpi_sleep_run_lps0_dsm(ACPI_LPS0_MS_EXIT,LPS0_DSM_FUNC_MASK_MICROSOFT as u32,LPS0_DSM_GUID_MICROSOFT); } if LPS0_DSM_FUNC_MASK > 0 { acpi_sleep_run_lps0_dsm(if acpi_s2idle_vendor_amd(){ACPI_LPS0_SCREEN_ON_AMD}else{ACPI_LPS0_SCREEN_ON},LPS0_DSM_FUNC_MASK as u32,LPS0_DSM_GUID); } }

// The remaining ACPI registration and suspend callbacks retain the kernel's external interfaces.
unsafe fn validate_dsm(handle: acpi_handle, uuid: *const i8, rev: i32, dsm_guid: *mut guid_t) -> i32 { guid_parse(uuid, dsm_guid); let obj = acpi_evaluate_dsm_typed(handle, dsm_guid, rev, 0, core::ptr::null(), ACPI_TYPE_BUFFER); if obj.is_null() || (*obj).buffer.length == 0 || (*obj).buffer.length > core::mem::size_of::<u32>() { ACPI_FREE(obj); return -EINVAL; } let ret = *( (*obj).buffer.pointer as *const i32); ACPI_FREE(obj); ret }

// Direct translations of the platform callbacks and exported registration functions.
pub unsafe fn acpi_s2idle_setup() { acpi_scan_add_handler(&mut lps0_handler); s2idle_set_ops(&acpi_s2idle_ops_lps0); }
pub unsafe fn acpi_register_lps0_dev(arg: *mut acpi_s2idle_dev_ops) -> i32 { if LPS0_DEVICE_HANDLE.is_null() || SLEEP_NO_LPS0 { return -ENODEV; } let flags = lock_system_sleep(); list_add(&mut (*arg).list_node, &mut LPS0_S2IDLE_DEVOPS_HEAD); unlock_system_sleep(flags); 0 }
pub unsafe fn acpi_unregister_lps0_dev(arg: *mut acpi_s2idle_dev_ops) { if LPS0_DEVICE_HANDLE.is_null() || SLEEP_NO_LPS0 { return; } let flags = lock_system_sleep(); list_del(&mut (*arg).list_node); unlock_system_sleep(flags); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
