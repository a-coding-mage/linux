// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*******************************************************************************
 *
 * Module Name: evsci - System Control Interrupt configuration and
 *                      legacy to ACPI mode state transition functions
 *
 ******************************************************************************/

// Dependencies are supplied by the surrounding ACPICA translation unit.
// Entire module is excluded when ACPI_REDUCED_HARDWARE is enabled.

#[allow(non_camel_case_types)]
type u32_acpi = u32;

#[cfg(not(feature = "acpi_reduced_hardware"))]
unsafe fn acpi_ev_sci_xrupt_handler(context: *mut core::ffi::c_void) -> u32 {
    let gpe_xrupt_list = context as *mut acpi_gpe_xrupt_info;
    let mut interrupt_handled: u32 = ACPI_INTERRUPT_NOT_HANDLED;

    // We are guaranteed by the ACPICA initialization/shutdown code that
    // if this interrupt handler is installed, ACPI is enabled.

    // Fixed Events: Check for and dispatch any Fixed Events that have occurred
    interrupt_handled |= acpi_ev_fixed_event_detect();

    // General Purpose Events: Check for and dispatch any GPEs that have occurred
    interrupt_handled |= acpi_ev_gpe_detect(gpe_xrupt_list);

    // Invoke all host-installed SCI handlers
    interrupt_handled |= acpi_ev_sci_dispatch();

    acpi_sci_count += 1;
    interrupt_handled
}

#[cfg(not(feature = "acpi_reduced_hardware"))]
pub unsafe fn acpi_ev_sci_dispatch() -> u32 {
    let mut sci_handler: *mut acpi_sci_handler_info;
    let mut flags: acpi_cpu_flags;
    let mut int_status: u32 = ACPI_INTERRUPT_NOT_HANDLED;

    // Are there any host-installed SCI handlers?
    if acpi_gbl_sci_handler_list.is_null() {
        return int_status;
    }

    flags = acpi_os_acquire_lock(acpi_gbl_gpe_lock);

    // Invoke all host-installed SCI handlers
    sci_handler = acpi_gbl_sci_handler_list;
    while !sci_handler.is_null() {
        // Invoke the installed handler (at interrupt level)
        int_status |= ((*sci_handler).address)((*sci_handler).context);
        sci_handler = (*sci_handler).next;
    }

    acpi_os_release_lock(acpi_gbl_gpe_lock, flags);
    int_status
}

#[cfg(not(feature = "acpi_reduced_hardware"))]
pub unsafe fn acpi_ev_gpe_xrupt_handler(context: *mut core::ffi::c_void) -> u32 {
    let gpe_xrupt_list = context as *mut acpi_gpe_xrupt_info;
    let mut interrupt_handled: u32 = ACPI_INTERRUPT_NOT_HANDLED;

    // We are guaranteed by the ACPICA initialization/shutdown code that
    // if this interrupt handler is installed, ACPI is enabled.

    // GPEs: Check for and dispatch any GPEs that have occurred
    interrupt_handled |= acpi_ev_gpe_detect(gpe_xrupt_list);
    interrupt_handled
}

#[cfg(not(feature = "acpi_reduced_hardware"))]
pub unsafe fn acpi_ev_install_sci_handler() -> u32 {
    let status: u32;

    status = acpi_os_install_interrupt_handler(
        acpi_gbl_FADT.sci_interrupt as u32,
        acpi_ev_sci_xrupt_handler,
        acpi_gbl_gpe_xrupt_list_head,
    );
    status
}

#[cfg(not(feature = "acpi_reduced_hardware"))]
pub unsafe fn acpi_ev_remove_all_sci_handlers() -> acpi_status {
    let sci_handler: *mut acpi_sci_handler_info;
    let flags: acpi_cpu_flags;
    let status: acpi_status;

    // Just let the OS remove the handler and disable the level
    status = acpi_os_remove_interrupt_handler(
        acpi_gbl_FADT.sci_interrupt as u32,
        acpi_ev_sci_xrupt_handler,
    );

    if acpi_gbl_sci_handler_list.is_null() {
        return status;
    }

    flags = acpi_os_acquire_lock(acpi_gbl_gpe_lock);

    // Free all host-installed SCI handlers
    while !acpi_gbl_sci_handler_list.is_null() {
        sci_handler = acpi_gbl_sci_handler_list;
        acpi_gbl_sci_handler_list = (*sci_handler).next;
        ACPI_FREE(sci_handler);
    }

    acpi_os_release_lock(acpi_gbl_gpe_lock, flags);
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
