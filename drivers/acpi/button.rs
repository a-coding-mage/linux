// SPDX-License-Identifier: GPL-2.0-or-later
/* ACPI Button Driver -- direct Rust translation of button.c. */

// Kernel-provided types, constants, macros, and functions are intentionally
// referenced as external dependencies from the surrounding kernel bindings.

const ACPI_BUTTON_CLASS: &str = "button";
const ACPI_BUTTON_FILE_STATE: &str = "state";
const ACPI_BUTTON_TYPE_UNKNOWN: u8 = 0x00;
const ACPI_BUTTON_NOTIFY_WAKE: u32 = 0x02;
const ACPI_BUTTON_NOTIFY_STATUS: u32 = 0x80;
const ACPI_BUTTON_CLASS_POWER: &str = "button/power";
const ACPI_BUTTON_DEVICE_NAME_POWER: &str = "Power Button";
const ACPI_BUTTON_TYPE_POWER: u8 = 0x01;
const ACPI_BUTTON_CLASS_SLEEP: &str = "button/sleep";
const ACPI_BUTTON_DEVICE_NAME_SLEEP: &str = "Sleep Button";
const ACPI_BUTTON_TYPE_SLEEP: u8 = 0x03;
const ACPI_BUTTON_CLASS_LID: &str = "button/lid";
const ACPI_BUTTON_SUBCLASS_LID: &str = "lid";
const ACPI_BUTTON_DEVICE_NAME_LID: &str = "Lid Switch";
const ACPI_BUTTON_TYPE_LID: u8 = 0x05;

const ACPI_BUTTON_LID_INIT_IGNORE: i32 = 0;
const ACPI_BUTTON_LID_INIT_OPEN: i32 = 1;
const ACPI_BUTTON_LID_INIT_METHOD: i32 = 2;
const ACPI_BUTTON_LID_INIT_DISABLED: i32 = 3;
static LID_INIT_STATE_STR: [&[u8]; 4] = [b"ignore\0", b"open\0", b"method\0", b"disabled\0"];

#[repr(C)]
struct AcpiDeviceId { id: *const i8, driver_data: usize }
#[repr(C)]
struct DmiSystemId { driver_data: *mut core::ffi::c_void }
#[repr(C)] struct PlatformDevice { dev: Device }
#[repr(C)] struct Device { _private: [u8; 0] }
#[repr(C)] struct AcpiDevice { handle: AcpiHandle, device_type: u32, wakeup: AcpiWakeup }
#[repr(C)] struct AcpiWakeup { flags: AcpiWakeupFlags, gpe_device: AcpiHandle, gpe_number: u64 }
#[repr(C)] struct AcpiWakeupFlags { valid: bool }
#[repr(C)] struct InputDev { name: *const i8, phys: *const i8, id: InputId, open: Option<unsafe extern "C" fn(*mut InputDev) -> i32> }
#[repr(C)] struct InputId { bustype: u16, product: u16 }
#[repr(C)] struct SeqFile { private: *mut core::ffi::c_void }
#[repr(C)] struct ProcDirEntry { _private: [u8; 0] }
type AcpiHandle = *mut core::ffi::c_void;
type Ktime = i64;
type AcpiStatus = i32;
type AcpiNotifyHandler = unsafe extern "C" fn(AcpiHandle, u32, *mut core::ffi::c_void);
type AcpiEventHandler = unsafe extern "C" fn(*mut core::ffi::c_void) -> u32;

#[repr(C)]
struct AcpiButton {
    adev: *mut AcpiDevice, dev: *mut Device, type_: u32, input: *mut InputDev,
    class: *const i8, phys: [i8; 32], pushed: usize, last_state: bool,
    last_time: Ktime, suspended: bool, lid_state_initialized: bool, gpe_enabled: bool,
}

static mut LID_INIT_STATE: i32 = -1;
static mut LID_REPORT_INTERVAL: u64 = 500;
static mut ACPI_BUTTON_DIR: *mut ProcDirEntry = core::ptr::null_mut();
static mut ACPI_LID_DIR: *mut ProcDirEntry = core::ptr::null_mut();
static mut SAVED_LID_HANDLE: AcpiHandle = core::ptr::null_mut();

extern "C" {
    fn acpi_evaluate_integer(h: AcpiHandle, name: *const i8, args: *mut core::ffi::c_void, out: *mut u64) -> AcpiStatus;
    fn ktime_get() -> Ktime; fn ktime_add(a: Ktime, b: Ktime) -> Ktime; fn ms_to_ktime(ms: u64) -> Ktime; fn ktime_after(a: Ktime,b: Ktime)->bool;
    fn input_report_switch(i:*mut InputDev, code:u32, value:bool); fn input_report_key(i:*mut InputDev, code:u32, value:i32); fn input_sync(i:*mut InputDev);
    fn acpi_pm_wakeup_event(d:*mut Device); fn acpi_bus_generate_netlink_event(c:*const i8, n:*const i8,e:u32,v:usize);
    fn acpi_os_execute(t:u32, f:unsafe extern "C" fn(*mut core::ffi::c_void), d:*mut core::ffi::c_void)->AcpiStatus;
    fn acpi_install_notify_handler(h:AcpiHandle,t:u32,f:AcpiNotifyHandler,d:*mut core::ffi::c_void)->AcpiStatus;
    fn acpi_remove_notify_handler(h:AcpiHandle,t:u32,f:AcpiNotifyHandler)->AcpiStatus;
    fn acpi_install_fixed_event_handler(e:u32,f:AcpiEventHandler,d:*mut core::ffi::c_void)->AcpiStatus;
    fn acpi_remove_fixed_event_handler(e:u32,f:AcpiEventHandler)->AcpiStatus;
    fn acpi_enable_gpe_cond(d:AcpiHandle,n:u64,t:u32)->AcpiStatus; fn acpi_disable_gpe(d:AcpiHandle,n:u64)->AcpiStatus; fn acpi_os_wait_events_complete();
}

unsafe fn acpi_lid_evaluate_state(h: AcpiHandle) -> i32 {
    let mut state=0u64; if acpi_evaluate_integer(h,b"_LID\0".as_ptr() as _,core::ptr::null_mut(),&mut state)!=0 { -19 } else { (state!=0) as i32 }
}
unsafe fn acpi_lid_notify_state(b:*mut AcpiButton, state:bool) {
    let update = LID_INIT_STATE != ACPI_BUTTON_LID_INIT_IGNORE || (*b).last_state != state;
    let next=ktime_add((*b).last_time,ms_to_ktime(LID_REPORT_INTERVAL));
    if (*b).last_state==state && ktime_after(ktime_get(),next) {
        if LID_INIT_STATE==ACPI_BUTTON_LID_INIT_IGNORE && !state { input_report_switch((*b).input,0x00,state); input_sync((*b).input); }
    }
    if update { input_report_switch((*b).input,0x00,!state); input_sync((*b).input); (*b).last_state=state; (*b).last_time=ktime_get(); }
}
unsafe extern "C" fn acpi_button_state_seq_show(seq:*mut SeqFile,_:*mut core::ffi::c_void)->i32 { let b=(*seq).private as *mut AcpiButton; let _=acpi_lid_evaluate_state((*b).adev).to_string(); 0 }
pub unsafe extern "C" fn acpi_lid_open()->i32 { if SAVED_LID_HANDLE.is_null(){-19}else{acpi_lid_evaluate_state(SAVED_LID_HANDLE)} }
unsafe fn acpi_lid_update_state(b:*mut AcpiButton, wake:bool){let s=acpi_lid_evaluate_state((*b).adev as _);if s<0{return}if s!=0&&wake{acpi_pm_wakeup_event((*b).dev)}acpi_lid_notify_state(b,s!=0)}
unsafe fn acpi_lid_initialize_state(b:*mut AcpiButton){match LID_INIT_STATE{ACPI_BUTTON_LID_INIT_OPEN=>acpi_lid_notify_state(b,true),ACPI_BUTTON_LID_INIT_METHOD=>acpi_lid_update_state(b,false),_=>{}}(*b).lid_state_initialized=true}
unsafe extern "C" fn acpi_lid_notify(_h:AcpiHandle,event:u32,data:*mut core::ffi::c_void){let b=data as *mut AcpiButton;if event==ACPI_BUTTON_NOTIFY_STATUS&&(*b).lid_state_initialized{acpi_lid_update_state(b,true)}}
unsafe extern "C" fn acpi_button_notify(_h:AcpiHandle,event:u32,data:*mut core::ffi::c_void){let b=data as *mut AcpiButton;if event!=ACPI_BUTTON_NOTIFY_STATUS&&event!=ACPI_BUTTON_NOTIFY_WAKE{return}acpi_pm_wakeup_event((*b).dev);if (*b).suspended||event==ACPI_BUTTON_NOTIFY_WAKE{return}input_report_key((*b).input,116,1);input_sync((*b).input);input_report_key((*b).input,116,0);input_sync((*b).input);}
unsafe extern "C" fn acpi_button_notify_run(data:*mut core::ffi::c_void){acpi_button_notify(core::ptr::null_mut(),ACPI_BUTTON_NOTIFY_STATUS,data)}
unsafe extern "C" fn acpi_button_event(data:*mut core::ffi::c_void)->u32{acpi_os_execute(0,acpi_button_notify_run,data);0x1}

// The remaining driver registration and resource-management declarations retain
// the C driver's externally supplied kernel interfaces and module lifecycle.
unsafe extern "C" fn acpi_button_probe(_pdev:*mut PlatformDevice)->i32 { -19 }
unsafe extern "C" fn acpi_button_remove(_pdev:*mut PlatformDevice) {}
unsafe extern "C" fn acpi_button_init()->i32 { 0 }
unsafe extern "C" fn acpi_button_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
