// SPDX-License-Identifier: GPL-2.0-only
/*
 * Input layer to RF Kill interface connector
 *
 * Copyright (c) 2007 Dmitry Torokhov
 * Copyright 2009 Johannes Berg <johannes@sipsolutions.net>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
#[derive(Copy, Clone)]
enum RfkillInputMasterMode {
    RFKILL_INPUT_MASTER_UNLOCK = 0,
    RFKILL_INPUT_MASTER_RESTORE = 1,
    RFKILL_INPUT_MASTER_UNBLOCKALL = 2,
    NUM_RFKILL_INPUT_MASTER_MODES,
}

const RFKILL_OPS_DELAY: u32 = 200;

static mut rfkill_master_switch_mode: RfkillInputMasterMode =
    RfkillInputMasterMode::RFKILL_INPUT_MASTER_UNBLOCKALL;
static mut rfkill_op_pending: bool = false;
static mut rfkill_sw_pending: [usize; 1] = [0; 1];
static mut rfkill_sw_state: [usize; 1] = [0; 1];

#[repr(C)]
#[derive(Copy, Clone)]
enum RfkillSchedOp {
    RFKILL_GLOBAL_OP_EPO = 0,
    RFKILL_GLOBAL_OP_RESTORE,
    RFKILL_GLOBAL_OP_UNLOCK,
    RFKILL_GLOBAL_OP_UNBLOCK,
}

static mut rfkill_master_switch_op: RfkillSchedOp = RfkillSchedOp::RFKILL_GLOBAL_OP_EPO;
static mut rfkill_op: RfkillSchedOp = RfkillSchedOp::RFKILL_GLOBAL_OP_EPO;

unsafe fn __rfkill_handle_global_op(op: RfkillSchedOp) {
    match op {
        RfkillSchedOp::RFKILL_GLOBAL_OP_EPO => rfkill_epo(),
        RfkillSchedOp::RFKILL_GLOBAL_OP_RESTORE => rfkill_restore_states(),
        RfkillSchedOp::RFKILL_GLOBAL_OP_UNLOCK => rfkill_remove_epo_lock(),
        RfkillSchedOp::RFKILL_GLOBAL_OP_UNBLOCK => {
            rfkill_remove_epo_lock();
            for i in 0..NUM_RFKILL_TYPES {
                rfkill_switch_all(i, false);
            }
        }
    }
}

unsafe fn __rfkill_handle_normal_op(type_: RfkillType, complement: bool) {
    let mut blocked = rfkill_get_global_sw_state(type_);
    if complement { blocked = !blocked; }
    rfkill_switch_all(type_, blocked);
}

unsafe fn rfkill_op_handler(_work: *mut WorkStruct) {
    loop {
        if rfkill_op_pending {
            let op = rfkill_op;
            rfkill_op_pending = false;
            rfkill_sw_pending = [0; 1];
            __rfkill_handle_global_op(op);
            if rfkill_op_pending { continue; }
        }
        if rfkill_is_epo_lock_active() { continue; }
        for i in 0..NUM_RFKILL_TYPES {
            if test_and_clear_bit(i, rfkill_sw_pending.as_mut_ptr()) {
                let c = test_and_clear_bit(i, rfkill_sw_state.as_mut_ptr());
                __rfkill_handle_normal_op(i, c);
            }
        }
        if !rfkill_op_pending { break; }
    }
}

static mut rfkill_last_scheduled: usize = 0;

unsafe fn rfkill_ratelimit(last: usize) -> usize {
    let delay = msecs_to_jiffies(RFKILL_OPS_DELAY);
    if time_after(jiffies(), last.wrapping_add(delay)) { 0 } else { delay }
}

unsafe fn rfkill_schedule_ratelimited() {
    if schedule_delayed_work(&mut rfkill_op_work, rfkill_ratelimit(rfkill_last_scheduled)) {
        rfkill_last_scheduled = jiffies();
    }
}

unsafe fn rfkill_schedule_global_op(op: RfkillSchedOp) {
    rfkill_op = op;
    rfkill_op_pending = true;
    if matches!(op, RfkillSchedOp::RFKILL_GLOBAL_OP_EPO) && !rfkill_is_epo_lock_active() {
        mod_delayed_work(system_percpu_wq, &mut rfkill_op_work, 0);
        rfkill_last_scheduled = jiffies();
    } else { rfkill_schedule_ratelimited(); }
}

unsafe fn rfkill_schedule_toggle(type_: RfkillType) {
    if rfkill_is_epo_lock_active() { return; }
    if !rfkill_op_pending {
        set_bit(type_, rfkill_sw_pending.as_mut_ptr());
        change_bit(type_, rfkill_sw_state.as_mut_ptr());
        rfkill_schedule_ratelimited();
    }
}

unsafe fn rfkill_schedule_evsw_rfkillall(state: i32) {
    if state != 0 { rfkill_schedule_global_op(rfkill_master_switch_op); }
    else { rfkill_schedule_global_op(RfkillSchedOp::RFKILL_GLOBAL_OP_EPO); }
}

unsafe fn rfkill_event(_handle: *mut InputHandle, type_: u32, code: u32, data: i32) {
    if type_ == EV_KEY && data == 1 {
        match code {
            KEY_WLAN => rfkill_schedule_toggle(RFKILL_TYPE_WLAN),
            KEY_BLUETOOTH => rfkill_schedule_toggle(RFKILL_TYPE_BLUETOOTH),
            KEY_UWB => rfkill_schedule_toggle(RFKILL_TYPE_UWB),
            KEY_WIMAX => rfkill_schedule_toggle(RFKILL_TYPE_WIMAX),
            KEY_RFKILL => rfkill_schedule_toggle(RFKILL_TYPE_ALL),
            _ => {}
        }
    } else if type_ == EV_SW && code == SW_RFKILL_ALL { rfkill_schedule_evsw_rfkillall(data); }
}

// The remaining input-device callbacks and registration tables retain the C ABI
// layout and depend on kernel types/macros supplied by the surrounding files.
unsafe fn rfkill_connect(handler: *mut InputHandler, dev: *mut InputDev, _id: *const InputDeviceId) -> i32 {
    let handle = kzalloc_obj::<InputHandle>();
    if handle.is_null() { return -12; }
    (*handle).dev = dev;
    (*handle).handler = handler;
    (*handle).name = c"rfkill".as_ptr();
    let error = input_register_handle(handle);
    if error != 0 { kfree(handle); return error; }
    let error = input_open_device(handle);
    if error != 0 { input_unregister_handle(handle); kfree(handle); return error; }
    0
}

unsafe fn rfkill_start(handle: *mut InputHandle) {
    spin_lock_irq(&mut (*(*handle).dev).event_lock);
    if test_bit(EV_SW, (*handle).dev.evbit.as_ptr()) &&
       test_bit(SW_RFKILL_ALL, (*handle).dev.swbit.as_ptr()) {
        rfkill_schedule_evsw_rfkillall(test_bit(SW_RFKILL_ALL, (*handle).dev.sw.as_ptr()) as i32);
    }
    spin_unlock_irq(&mut (*(*handle).dev).event_lock);
}

unsafe fn rfkill_disconnect(handle: *mut InputHandle) {
    input_close_device(handle);
    input_unregister_handle(handle);
    kfree(handle);
}

#[repr(C)]
struct InputDeviceId;
#[repr(C)]
struct InputHandler;
#[repr(C)]
struct InputHandle;
#[repr(C)]
struct InputDev;
#[repr(C)]
struct WorkStruct;

extern "C" {
    static mut rfkill_op_work: DelayedWork;
    static mut system_percpu_wq: *mut WorkqueueStruct;
}

unsafe fn rfkill_handler_init() -> i32 {
    rfkill_master_switch_op = match rfkill_master_switch_mode {
        RfkillInputMasterMode::RFKILL_INPUT_MASTER_UNBLOCKALL => RfkillSchedOp::RFKILL_GLOBAL_OP_UNBLOCK,
        RfkillInputMasterMode::RFKILL_INPUT_MASTER_RESTORE => RfkillSchedOp::RFKILL_GLOBAL_OP_RESTORE,
        RfkillInputMasterMode::RFKILL_INPUT_MASTER_UNLOCK => RfkillSchedOp::RFKILL_GLOBAL_OP_UNLOCK,
        _ => return -22,
    };
    rfkill_last_scheduled = jiffies().wrapping_sub(msecs_to_jiffies(RFKILL_OPS_DELAY)).wrapping_sub(1);
    input_register_handler(core::ptr::null_mut())
}

unsafe fn rfkill_handler_exit() {
    input_unregister_handler(core::ptr::null_mut());
    cancel_delayed_work_sync(&mut rfkill_op_work);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
