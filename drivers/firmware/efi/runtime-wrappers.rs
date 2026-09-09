// SPDX-License-Identifier: GPL-2.0-only
/* Runtime Services function call wrappers. Direct translation of runtime-wrappers.c. */

// Dependencies supplied by the surrounding kernel EFI implementation are intentionally
// referenced here rather than reimplemented.

#[repr(C)]
pub union EfiRtsArgs {
    pub get_time: GetTime,
    pub set_time: SetTime,
    pub get_wakeup_time: GetWakeupTime,
    pub set_wakeup_time: SetWakeupTime,
    pub get_variable: GetVariable,
    pub get_next_variable: GetNextVariable,
    pub set_variable: SetVariable,
    pub query_variable_info: QueryVariableInfo,
    pub get_next_high_mono_count: GetNextHighMonoCount,
    pub update_capsule: UpdateCapsule,
    pub query_capsule_caps: QueryCapsuleCaps,
    pub acpi_prm_handler: AcpiPrmHandler,
}

#[repr(C)] pub struct GetTime { pub time: *mut efi_time_t, pub capabilities: *mut efi_time_cap_t }
#[repr(C)] pub struct SetTime { pub time: *mut efi_time_t }
#[repr(C)] pub struct GetWakeupTime { pub enabled: *mut efi_bool_t, pub pending: *mut efi_bool_t, pub time: *mut efi_time_t }
#[repr(C)] pub struct SetWakeupTime { pub enable: efi_bool_t, pub time: *mut efi_time_t }
#[repr(C)] pub struct GetVariable { pub name: *mut efi_char16_t, pub vendor: *mut efi_guid_t, pub attr: *mut u32, pub data_size: *mut libc::c_ulong, pub data: *mut libc::c_void }
#[repr(C)] pub struct GetNextVariable { pub name_size: *mut libc::c_ulong, pub name: *mut efi_char16_t, pub vendor: *mut efi_guid_t }
#[repr(C)] pub struct SetVariable { pub name: *mut efi_char16_t, pub vendor: *mut efi_guid_t, pub attr: u32, pub data_size: libc::c_ulong, pub data: *mut libc::c_void }
#[repr(C)] pub struct QueryVariableInfo { pub attr: u32, pub storage_space: *mut u64, pub remaining_space: *mut u64, pub max_variable_size: *mut u64 }
#[repr(C)] pub struct GetNextHighMonoCount { pub high_count: *mut u32 }
#[repr(C)] pub struct UpdateCapsule { pub capsules: *mut *mut efi_capsule_header_t, pub count: libc::c_ulong, pub sg_list: libc::c_ulong }
#[repr(C)] pub struct QueryCapsuleCaps { pub capsules: *mut *mut efi_capsule_header_t, pub count: libc::c_ulong, pub max_size: *mut u64, pub reset_type: *mut libc::c_int }
#[repr(C)] pub struct AcpiPrmHandler { pub acpi_prm_handler: Option<unsafe extern "efiapi" fn(u64, *mut libc::c_void) -> efi_status_t>, pub param_buffer_addr: u64, pub context: *mut libc::c_void }

extern "C" {
    static mut efi_rts_work: efi_runtime_work;
    static mut efi_runtime_lock: semaphore;
    static mut efi_runtime_lock_owner: *mut task_struct;
}

const EFI_RTS_TIMEOUT: libc::c_ulong = 120 * HZ;

pub unsafe extern "C" fn efi_call_virt_save_flags() -> libc::c_ulong {
    let mut flags = 0;
    arch_efi_save_flags(&mut flags);
    flags
}

pub unsafe extern "C" fn efi_call_virt_check_flags(flags: libc::c_ulong, caller: *const libc::c_void) {
    let cur_flags = efi_call_virt_save_flags();
    let mismatch = flags ^ cur_flags;
    if !(WARN_ON_ONCE(mismatch & ARCH_EFI_IRQ_FLAGS_MASK) != 0) { return; }
    add_taint(TAINT_FIRMWARE_WORKAROUND, LOCKDEP_NOW_UNRELIABLE);
    pr_err_ratelimited!("IRQ flags corrupted (0x%08lx=>0x%08lx) by EFI call from %pS\n", flags, cur_flags, if !caller.is_null() { caller } else { builtin_return_address(0) });
    arch_efi_restore_flags(flags);
}

pub unsafe extern "C" fn efi_rts_park_worker() -> ! {
    loop { set_current_state(TASK_IDLE); schedule(); }
}

unsafe extern "C" fn efi_call_rts(_work: *mut work_struct) {
    let args = efi_rts_work.args;
    let mut status = EFI_NOT_FOUND;
    if !efi_enabled(EFI_RUNTIME_SERVICES) { efi_rts_park_worker(); }
    efi_runtime_lock_owner = current;
    arch_efi_call_virt_setup();
    let flags = efi_call_virt_save_flags();
    match efi_rts_work.efi_rts_id {
        EFI_GET_TIME => { status = arch_efi_call_virt(efi.runtime, get_time, (*args).get_time.time, (*args).get_time.capabilities); }
        EFI_SET_TIME => { status = arch_efi_call_virt(efi.runtime, set_time, (*args).set_time.time); }
        EFI_GET_WAKEUP_TIME => { status = arch_efi_call_virt(efi.runtime, get_wakeup_time, (*args).get_wakeup_time.enabled, (*args).get_wakeup_time.pending, (*args).get_wakeup_time.time); }
        EFI_SET_WAKEUP_TIME => { status = arch_efi_call_virt(efi.runtime, set_wakeup_time, (*args).set_wakeup_time.enable, (*args).set_wakeup_time.time); }
        EFI_GET_VARIABLE => { status = arch_efi_call_virt(efi.runtime, get_variable, (*args).get_variable.name, (*args).get_variable.vendor, (*args).get_variable.attr, (*args).get_variable.data_size, (*args).get_variable.data); }
        EFI_GET_NEXT_VARIABLE => { status = arch_efi_call_virt(efi.runtime, get_next_variable, (*args).get_next_variable.name_size, (*args).get_next_variable.name, (*args).get_next_variable.vendor); }
        EFI_SET_VARIABLE => { status = arch_efi_call_virt(efi.runtime, set_variable, (*args).set_variable.name, (*args).set_variable.vendor, (*args).set_variable.attr, (*args).set_variable.data_size, (*args).set_variable.data); }
        EFI_QUERY_VARIABLE_INFO => { status = arch_efi_call_virt(efi.runtime, query_variable_info, (*args).query_variable_info.attr, (*args).query_variable_info.storage_space, (*args).query_variable_info.remaining_space, (*args).query_variable_info.max_variable_size); }
        EFI_GET_NEXT_HIGH_MONO_COUNT => { status = arch_efi_call_virt(efi.runtime, get_next_high_mono_count, (*args).get_next_high_mono_count.high_count); }
        EFI_UPDATE_CAPSULE => { status = arch_efi_call_virt(efi.runtime, update_capsule, (*args).update_capsule.capsules, (*args).update_capsule.count, (*args).update_capsule.sg_list); }
        EFI_QUERY_CAPSULE_CAPS => { status = arch_efi_call_virt(efi.runtime, query_capsule_caps, (*args).query_capsule_caps.capsules, (*args).query_capsule_caps.count, (*args).query_capsule_caps.max_size, (*args).query_capsule_caps.reset_type); }
        EFI_ACPI_PRM_HANDLER => {
            #[cfg(CONFIG_ACPI_PRMT)] { status = arch_efi_call_virt(args, (*args).acpi_prm_handler.acpi_prm_handler, (*args).acpi_prm_handler.param_buffer_addr, (*args).acpi_prm_handler.context); }
        }
        _ => { pr_err!("Requested executing invalid EFI Runtime Service.\n"); }
    }
    efi_call_virt_check_flags(flags, efi_rts_work.caller);
    arch_efi_call_virt_teardown();
    if !efi_enabled(EFI_RUNTIME_SERVICES) { efi_rts_park_worker(); }
    efi_rts_work.status = status;
    complete(&mut efi_rts_work.efi_rts_comp);
    efi_runtime_lock_owner = core::ptr::null_mut();
}

// The remaining wrappers preserve the C locking and queueing contract.
macro_rules! queued_wrapper { ($name:ident, $id:ident, ($($arg:ident : $ty:ty),*), ($($field:expr),*)) => {
    unsafe extern "C" fn $name($($arg: $ty),*) -> efi_status_t {
        if down_interruptible(&mut efi_runtime_lock) != 0 { return EFI_ABORTED; }
        let status = efi_queue_work!($id, $($field),*);
        up(&mut efi_runtime_lock); status
    }
}; }

unsafe extern "C" fn __efi_queue_work(id: efi_rts_ids, args: *mut EfiRtsArgs) -> efi_status_t {
    if !efi_enabled(EFI_RUNTIME_SERVICES) { pr_warn_once!("EFI Runtime Services are disabled!\n"); return EFI_DEVICE_ERROR; }
    efi_rts_work.efi_rts_id = id; efi_rts_work.args = args; efi_rts_work.caller = builtin_return_address(0); efi_rts_work.status = EFI_ABORTED;
    init_completion(&mut efi_rts_work.efi_rts_comp); INIT_WORK!(&mut efi_rts_work.work, efi_call_rts);
    if !queue_work(efi_rts_wq, &mut efi_rts_work.work) { pr_err!("Failed to queue work to efi_rts_wq.\n"); efi_rts_work.efi_rts_id = EFI_NONE; return efi_rts_work.status; }
    if wait_for_completion_timeout(&mut efi_rts_work.efi_rts_comp, EFI_RTS_TIMEOUT) == 0 { pr_err!("EFI runtime service %d wedged in firmware; disabling EFI runtime services\n", id); clear_bit(EFI_RUNTIME_SERVICES, &mut efi.flags); return EFI_ABORTED; }
    WARN_ON_ONCE(efi_rts_work.status == EFI_ABORTED); efi_rts_work.efi_rts_id = EFI_NONE; efi_rts_work.status
}

queued_wrapper!(virt_efi_get_time, GET_TIME, (tm: *mut efi_time_t, tc: *mut efi_time_cap_t), (tm, tc));
queued_wrapper!(virt_efi_set_time, SET_TIME, (tm: *mut efi_time_t), (tm));
queued_wrapper!(virt_efi_get_wakeup_time, GET_WAKEUP_TIME, (enabled: *mut efi_bool_t, pending: *mut efi_bool_t, tm: *mut efi_time_t), (enabled, pending, tm));
queued_wrapper!(virt_efi_set_wakeup_time, SET_WAKEUP_TIME, (enabled: efi_bool_t, tm: *mut efi_time_t), (enabled, tm));
queued_wrapper!(virt_efi_get_variable, GET_VARIABLE, (name: *mut efi_char16_t, vendor: *mut efi_guid_t, attr: *mut u32, data_size: *mut libc::c_ulong, data: *mut libc::c_void), (name, vendor, attr, data_size, data));
queued_wrapper!(virt_efi_get_next_variable, GET_NEXT_VARIABLE, (name_size: *mut libc::c_ulong, name: *mut efi_char16_t, vendor: *mut efi_guid_t), (name_size, name, vendor));
queued_wrapper!(virt_efi_set_variable, SET_VARIABLE, (name: *mut efi_char16_t, vendor: *mut efi_guid_t, attr: u32, data_size: libc::c_ulong, data: *mut libc::c_void), (name, vendor, attr, data_size, data));
queued_wrapper!(virt_efi_get_next_high_mono_count, GET_NEXT_HIGH_MONO_COUNT, (count: *mut u32), (count));

queued_wrapper!(virt_efi_query_variable_info, QUERY_VARIABLE_INFO, (attr: u32, storage_space: *mut u64, remaining_space: *mut u64, max_variable_size: *mut u64), (attr, storage_space, remaining_space, max_variable_size));
queued_wrapper!(virt_efi_update_capsule, UPDATE_CAPSULE, (capsules: *mut *mut efi_capsule_header_t, count: libc::c_ulong, sg_list: libc::c_ulong), (capsules, count, sg_list));
queued_wrapper!(virt_efi_query_capsule_caps, QUERY_CAPSULE_CAPS, (capsules: *mut *mut efi_capsule_header_t, count: libc::c_ulong, max_size: *mut u64, reset_type: *mut libc::c_int), (capsules, count, max_size, reset_type));

unsafe extern "C" fn virt_efi_set_variable_nb(name: *mut efi_char16_t, vendor: *mut efi_guid_t, attr: u32, data_size: libc::c_ulong, data: *mut libc::c_void) -> efi_status_t {
    if down_trylock(&mut efi_runtime_lock) != 0 { return EFI_NOT_READY; }
    if !efi_enabled(EFI_RUNTIME_SERVICES) { up(&mut efi_runtime_lock); return EFI_DEVICE_ERROR; }
    efi_runtime_lock_owner = current;
    let status = efi_call_virt_pointer!(efi.runtime, set_variable, name, vendor, attr, data_size, data);
    efi_runtime_lock_owner = core::ptr::null_mut(); up(&mut efi_runtime_lock); status
}

unsafe extern "C" fn virt_efi_query_variable_info_nb(attr: u32, storage_space: *mut u64, remaining_space: *mut u64, max_variable_size: *mut u64) -> efi_status_t {
    if efi.runtime_version < EFI_2_00_SYSTEM_TABLE_REVISION { return EFI_UNSUPPORTED; }
    if down_trylock(&mut efi_runtime_lock) != 0 { return EFI_NOT_READY; }
    if !efi_enabled(EFI_RUNTIME_SERVICES) { up(&mut efi_runtime_lock); return EFI_DEVICE_ERROR; }
    efi_runtime_lock_owner = current;
    let status = efi_call_virt_pointer!(efi.runtime, query_variable_info, attr, storage_space, remaining_space, max_variable_size);
    efi_runtime_lock_owner = core::ptr::null_mut(); up(&mut efi_runtime_lock); status
}

unsafe extern "C" fn virt_efi_reset_system(reset_type: libc::c_int, status: efi_status_t, data_size: libc::c_ulong, data: *mut efi_char16_t) {
    if down_trylock(&mut efi_runtime_lock) != 0 { pr_warn!("failed to invoke the reset_system() runtime service:\ncould not get exclusive access to the firmware\n"); return; }
    if !efi_enabled(EFI_RUNTIME_SERVICES) { pr_warn!("EFI Runtime Services are disabled, not invoking reset_system()\n"); up(&mut efi_runtime_lock); return; }
    efi_runtime_lock_owner = current; arch_efi_call_virt_setup(); efi_rts_work.efi_rts_id = EFI_RESET_SYSTEM;
    arch_efi_call_virt(efi.runtime, reset_system, reset_type, status, data_size, data);
    arch_efi_call_virt_teardown(); efi_runtime_lock_owner = core::ptr::null_mut(); up(&mut efi_runtime_lock);
}

#[cfg(CONFIG_ACPI_PRMT)]
pub unsafe extern "C" fn efi_call_acpi_prm_handler(handler_addr: Option<unsafe extern "efiapi" fn(u64, *mut libc::c_void) -> efi_status_t>, param_buffer_addr: u64, context: *mut libc::c_void) -> efi_status_t {
    if down_interruptible(&mut efi_runtime_lock) != 0 { return EFI_ABORTED; }
    let status = efi_queue_work!(ACPI_PRM_HANDLER, handler_addr, param_buffer_addr, context);
    up(&mut efi_runtime_lock); status
}

pub unsafe extern "C" fn efi_native_runtime_setup() {
    efi.get_time = virt_efi_get_time; efi.set_time = virt_efi_set_time; efi.get_wakeup_time = virt_efi_get_wakeup_time; efi.set_wakeup_time = virt_efi_set_wakeup_time; efi.get_variable = virt_efi_get_variable; efi.get_next_variable = virt_efi_get_next_variable; efi.set_variable = virt_efi_set_variable; efi.set_variable_nonblocking = virt_efi_set_variable_nb; efi.get_next_high_mono_count = virt_efi_get_next_high_mono_count; efi.reset_system = virt_efi_reset_system; efi.query_variable_info = virt_efi_query_variable_info; efi.query_variable_info_nonblocking = virt_efi_query_variable_info_nb; efi.update_capsule = virt_efi_update_capsule; efi.query_capsule_caps = virt_efi_query_capsule_caps;
}

pub unsafe extern "C" fn efi_runtime_assert_lock_held() { WARN_ON(efi_runtime_lock_owner != current); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
