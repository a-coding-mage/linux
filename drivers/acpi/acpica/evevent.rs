// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: evevent - Fixed Event handling and dispatch
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 ******************************************************************************/

// Dependencies supplied by the surrounding ACPI implementation are intentionally
// referenced by name here.

// #if !ACPI_REDUCED_HARDWARE -- Entire module

/* Local prototypes */
unsafe fn acpi_ev_fixed_event_initialize() -> acpi_status;

unsafe fn acpi_ev_fixed_event_dispatch(event: u32) -> u32;

/*******************************************************************************
 *
 * FUNCTION:    acpi_ev_initialize_events
 *
 * PARAMETERS:  None
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Initialize global data structures for ACPI events (Fixed, GPE)
 *
 ******************************************************************************/

pub unsafe fn acpi_ev_initialize_events() -> acpi_status {
    let mut status: acpi_status;

    /* If Hardware Reduced flag is set, there are no fixed events */
    if acpi_gbl_reduced_hardware {
        return AE_OK;
    }

    /*
     * Initialize the Fixed and General Purpose Events. This is done prior to
     * enabling SCIs to prevent interrupts from occurring before the handlers
     * are installed.
     */
    status = acpi_ev_fixed_event_initialize();
    if ACPI_FAILURE(status) {
        return status;
    }

    status = acpi_ev_gpe_initialize();
    if ACPI_FAILURE(status) {
        return status;
    }

    status
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ev_install_xrupt_handlers
 *
 * PARAMETERS:  None
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Install interrupt handlers for the SCI and Global Lock
 *
 ******************************************************************************/

pub unsafe fn acpi_ev_install_xrupt_handlers() -> acpi_status {
    let mut status: acpi_status;

    /* If Hardware Reduced flag is set, there is no ACPI h/w */
    if acpi_gbl_reduced_hardware {
        return AE_OK;
    }

    /* Install the SCI handler */
    status = acpi_ev_install_sci_handler();
    if ACPI_FAILURE(status) {
        return status;
    }

    /* Install the handler for the Global Lock */
    status = acpi_ev_init_global_lock_handler();
    if ACPI_FAILURE(status) {
        return status;
    }

    acpi_gbl_events_initialized = TRUE;
    status
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ev_fixed_event_initialize
 *
 * PARAMETERS:  None
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Install the fixed event handlers and disable all fixed events.
 *
 ******************************************************************************/

unsafe fn acpi_ev_fixed_event_initialize() -> acpi_status {
    let mut i: u32;
    let status: acpi_status;

    /*
     * Initialize the structure that keeps track of fixed event handlers and
     * disable all of the fixed events.
     */
    i = 0;
    while i < ACPI_NUM_FIXED_EVENTS {
        acpi_gbl_fixed_event_handlers[i as usize].handler = None;
        acpi_gbl_fixed_event_handlers[i as usize].context = core::ptr::null_mut();

        /* Disable the fixed event */
        if acpi_gbl_fixed_event_info[i as usize].enable_register_id != 0xFF {
            status = acpi_write_bit_register(
                acpi_gbl_fixed_event_info[i as usize].enable_register_id,
                ACPI_DISABLE_EVENT,
            );
            if ACPI_FAILURE(status) {
                return status;
            }
        }
        i += 1;
    }

    AE_OK
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ev_fixed_event_detect
 *
 * PARAMETERS:  None
 *
 * RETURN:      INTERRUPT_HANDLED or INTERRUPT_NOT_HANDLED
 *
 * DESCRIPTION: Checks the PM status register for active fixed events
 *
 ******************************************************************************/

pub unsafe fn acpi_ev_fixed_event_detect() -> u32 {
    let mut int_status: u32 = ACPI_INTERRUPT_NOT_HANDLED;
    let mut fixed_status: u32 = 0;
    let mut fixed_enable: u32 = 0;
    let mut i: u32 = 0;
    let mut status: acpi_status;

    /*
     * Read the fixed feature status and enable registers, as all the cases
     * depend on their values. Ignore errors here.
     */
    status = acpi_hw_register_read(ACPI_REGISTER_PM1_STATUS, &mut fixed_status);
    status |= acpi_hw_register_read(ACPI_REGISTER_PM1_ENABLE, &mut fixed_enable);
    if ACPI_FAILURE(status) {
        return int_status;
    }

    /* Check for all possible Fixed Events and dispatch those that are active */
    while i < ACPI_NUM_FIXED_EVENTS {
        let info = &acpi_gbl_fixed_event_info[i as usize];
        if (fixed_status & info.status_bit_mask) != 0
            && (fixed_enable & info.enable_bit_mask) != 0
        {
            acpi_fixed_event_count[i as usize] += 1;
            if let Some(handler) = acpi_gbl_global_event_handler {
                handler(ACPI_EVENT_TYPE_FIXED, core::ptr::null_mut(), i,
                        acpi_gbl_global_event_handler_context);
            }
            int_status |= acpi_ev_fixed_event_dispatch(i);
        }
        i += 1;
    }

    int_status
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ev_fixed_event_dispatch
 *
 * PARAMETERS:  event               - Event type
 *
 * RETURN:      INTERRUPT_HANDLED or INTERRUPT_NOT_HANDLED
 *
 * DESCRIPTION: Clears the status bit for the requested event, calls the
 *              handler that previously registered for the event.
 *              NOTE: If there is no handler for the event, the event is
 *              disabled to prevent further interrupts.
 *
 ******************************************************************************/

unsafe fn acpi_ev_fixed_event_dispatch(event: u32) -> u32 {
    /* Clear the status bit */
    let _ = acpi_write_bit_register(
        acpi_gbl_fixed_event_info[event as usize].status_register_id,
        ACPI_CLEAR_STATUS,
    );

    /*
     * Make sure that a handler exists. If not, report an error
     * and disable the event to prevent further interrupts.
     */
    match acpi_gbl_fixed_event_handlers[event as usize].handler {
        None => {
            let _ = acpi_write_bit_register(
                acpi_gbl_fixed_event_info[event as usize].enable_register_id,
                ACPI_DISABLE_EVENT,
            );
            ACPI_INTERRUPT_NOT_HANDLED
        }
        Some(handler) => handler(acpi_gbl_fixed_event_handlers[event as usize].context),
    }
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_any_fixed_event_status_set
 *
 * PARAMETERS:  None
 *
 * RETURN:      TRUE or FALSE
 *
 * DESCRIPTION: Checks the PM status register for active fixed events
 *
 ******************************************************************************/

pub unsafe fn acpi_any_fixed_event_status_set() -> u32 {
    let mut status: acpi_status;
    let mut in_status: u32 = 0;
    let mut in_enable: u32 = 0;
    let mut i: u32 = 0;

    status = acpi_hw_register_read(ACPI_REGISTER_PM1_ENABLE, &mut in_enable);
    if ACPI_FAILURE(status) {
        return FALSE;
    }

    status = acpi_hw_register_read(ACPI_REGISTER_PM1_STATUS, &mut in_status);
    if ACPI_FAILURE(status) {
        return FALSE;
    }

    /* Check for all possible Fixed Events and dispatch those that are active */
    while i < ACPI_NUM_FIXED_EVENTS {
        let info = &acpi_gbl_fixed_event_info[i as usize];
        if (in_status & info.status_bit_mask) != 0
            && (in_enable & info.enable_bit_mask) != 0
        {
            return TRUE;
        }
        i += 1;
    }

    FALSE
}

// #endif /* !ACPI_REDUCED_HARDWARE */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
