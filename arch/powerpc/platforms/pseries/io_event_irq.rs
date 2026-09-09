// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2010 2011 Mark Nelson and Tseng-Hui (Frank) Lin, IBM Corporation
 */

// The C source includes Linux, PowerPC, and pseries headers supplying the
// declarations referenced below.

/*
 * IO event interrupt is a mechanism provided by RTAS to return
 * information about hardware error and non-error events. Device
 * drivers can register their event handlers to receive events.
 * Device drivers are expected to use atomic_notifier_chain_register()
 * and atomic_notifier_chain_unregister() to register and unregister
 * their event handlers. Since multiple IO event types and scopes
 * share an IO event interrupt, the event handlers are called one
 * by one until the IO event is claimed by one of the handlers.
 * The event handlers are expected to return NOTIFY_OK if the
 * event is handled by the event handler or NOTIFY_DONE if the
 * event does not belong to the handler.
 */

// ATOMIC_NOTIFIER_HEAD(pseries_ioei_notifier_list);
// The notifier-head definition is provided by the notifier subsystem.
extern "C" {
    pub static mut pseries_ioei_notifier_list: atomic_notifier_head;
}

extern "C" {
    fn rtas_error_type(elog: *mut rtas_error_log) -> i32;
    fn get_pseries_errorlog(
        elog: *mut rtas_error_log,
        section_id: i32,
    ) -> *mut pseries_errorlog;
    fn printk_once(format: *const u8, ...);
    fn rtas_call(token: i32, nargs: i32, nret: i32, ret_buf: *mut core::ffi::c_void, ...) -> i32;
    fn virq_to_hw(irq: i32) -> u32;
    fn atomic_notifier_call_chain(
        head: *mut atomic_notifier_head,
        val: u64,
        v: *mut core::ffi::c_void,
    ) -> i32;
    fn rtas_function_token(token: i32) -> i32;
    fn of_find_node_by_path(path: *const u8) -> *mut device_node;
    fn request_event_sources_irqs(
        np: *mut device_node,
        handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t,
        name: *const u8,
    ) -> i32;
    fn of_node_put(np: *mut device_node);
    fn pr_info(format: *const u8, ...);
}

#[no_mangle]
pub static mut ioei_check_exception_token: i32 = 0;

#[repr(align(64))]
pub struct IoEiRtasBuf(pub [u8; RTAS_DATA_BUF_SIZE as usize]);

#[no_mangle]
pub static mut ioei_rtas_buf: IoEiRtasBuf = IoEiRtasBuf([0; RTAS_DATA_BUF_SIZE as usize]);

/**
 * Find the data portion of an IO Event section from event log.
 * @elog: RTAS error/event log.
 *
 * Return:
 *     pointer to a valid IO event section data. NULL if not found.
 */
unsafe fn ioei_find_event(elog: *mut rtas_error_log) -> *mut pseries_io_event {
    let sect: *mut pseries_errorlog;

    if rtas_error_type(elog) != RTAS_TYPE_IO {
        printk_once(b"io_event_irq: Unexpected event type %d\0".as_ptr(),
                    rtas_error_type(elog));
        return core::ptr::null_mut();
    }

    sect = get_pseries_errorlog(elog, PSERIES_ELOG_SECT_ID_IO_EVENT);
    if sect.is_null() {
        printk_once(b"io_event_irq: RTAS extended event log does not contain an IO Event section. Could be a bug in system firmware!\n\0".as_ptr());
        return core::ptr::null_mut();
    }
    // C returns the address of sect->data, represented here by the
    // corresponding field address supplied by the pseries error-log type.
    &mut (*sect).data as *mut _ as *mut pseries_io_event
}

unsafe extern "C" fn ioei_interrupt(irq: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let mut event: *mut pseries_io_event;
    let mut rtas_rc: i32;

    loop {
        rtas_rc = rtas_call(
            ioei_check_exception_token,
            6,
            1,
            core::ptr::null_mut(),
            RTAS_VECTOR_EXTERNAL_INTERRUPT,
            virq_to_hw(irq),
            RTAS_IO_EVENTS,
            1, // Time Critical
            __pa(ioei_rtas_buf.0.as_mut_ptr()),
            RTAS_DATA_BUF_SIZE,
        );
        if rtas_rc != 0 {
            break;
        }

        event = ioei_find_event(ioei_rtas_buf.0.as_mut_ptr() as *mut rtas_error_log);
        if event.is_null() {
            continue;
        }

        atomic_notifier_call_chain(
            &raw mut pseries_ioei_notifier_list,
            0,
            event as *mut core::ffi::c_void,
        );
    }
    IRQ_HANDLED
}

unsafe extern "C" fn ioei_init() -> i32 {
    let np: *mut device_node;

    ioei_check_exception_token = rtas_function_token(RTAS_FN_CHECK_EXCEPTION);
    if ioei_check_exception_token == RTAS_UNKNOWN_SERVICE {
        return -ENODEV;
    }

    np = of_find_node_by_path(b"/event-sources/ibm,io-events\0".as_ptr());
    if !np.is_null() {
        request_event_sources_irqs(np, ioei_interrupt, b"IO_EVENT\0".as_ptr());
        pr_info(b"IBM I/O event interrupts enabled\n\0".as_ptr());
        of_node_put(np);
    } else {
        return -ENODEV;
    }
    0
}

// machine_subsys_initcall(pseries, ioei_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
