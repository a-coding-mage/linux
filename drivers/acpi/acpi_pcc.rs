// SPDX-License-Identifier: GPL-2.0-only
/*
 * Author: Sudeep Holla <sudeep.holla@arm.com>
 * Copyright 2021 Arm Limited
 *
 * The PCC Address Space also referred as PCC Operation Region pertains to the
 * region of PCC subspace that succeeds the PCC signature. The PCC Operation
 * Region works in conjunction with the PCC Table(Platform Communications
 * Channel Table). PCC subspaces that are marked for use as PCC Operation
 * Regions must not be used as PCC subspaces for the standard ACPI features
 * such as CPPC, RASF, PDTT and MPST. These standard features must always use
 * the PCC Table instead.
 *
 * This driver sets up the PCC Address Space and installs an handler to enable
 * handling of PCC OpRegion in the firmware.
 *
 */

/* Arbitrary retries in case the remote processor is slow to respond to PCC commands. */
const PCC_CMD_WAIT_RETRIES_NUM: u64 = 500u64;

#[repr(C)]
struct pcc_data {
    pcc_chan: *mut pcc_mbox_chan,
    done: completion,
    cl: mbox_client,
    ctx: acpi_pcc_info,
}

static mut pcc_ctx: acpi_pcc_info = unsafe { core::mem::zeroed() };

unsafe extern "C" fn pcc_rx_callback(cl: *mut mbox_client, _m: *mut core::ffi::c_void) {
    let data: *mut pcc_data = container_of!(cl, pcc_data, cl);

    complete(core::ptr::addr_of_mut!((*data).done));
}

unsafe extern "C" fn acpi_pcc_address_space_setup(
    region_handle: acpi_handle,
    function: u32,
    handler_context: *mut core::ffi::c_void,
    region_context: *mut *mut core::ffi::c_void,
) -> acpi_status {
    let mut data: *mut pcc_data;
    let ctx: *mut acpi_pcc_info = handler_context as *mut acpi_pcc_info;
    let mut pcc_chan: *mut pcc_mbox_chan;
    let ret: acpi_status;

    data = kzalloc_obj!(*data);
    if data.is_null() {
        return AE_NO_MEMORY;
    }

    (*data).cl.rx_callback = Some(pcc_rx_callback);
    (*data).cl.knows_txdone = true;
    (*data).ctx.length = (*ctx).length;
    (*data).ctx.subspace_id = (*ctx).subspace_id;
    (*data).ctx.internal_buffer = (*ctx).internal_buffer;

    init_completion(core::ptr::addr_of_mut!((*data).done));
    (*data).pcc_chan = pcc_mbox_request_channel(
        core::ptr::addr_of_mut!((*data).cl),
        (*ctx).subspace_id,
    );
    if IS_ERR((*data).pcc_chan) {
        pr_err!("Failed to find PCC channel for subspace %d\n", (*ctx).subspace_id);
        ret = AE_NOT_FOUND;
        goto!(err_free_data);
    }

    pcc_chan = (*data).pcc_chan;
    if !(*(*(*pcc_chan).mchan).mbox).txdone_irq {
        pr_err!("This channel-%d does not support interrupt.\n", (*ctx).subspace_id);
        ret = AE_SUPPORT;
        goto!(err_free_channel);
    }

    *region_context = data as *mut core::ffi::c_void;
    return AE_OK;

    err_free_channel:
    pcc_mbox_free_channel((*data).pcc_chan);
    err_free_data:
    kfree(data);

    ret
}

unsafe extern "C" fn acpi_pcc_address_space_handler(
    function: u32,
    addr: acpi_physical_address,
    bits: u32,
    value: *mut acpi_integer,
    handler_context: *mut core::ffi::c_void,
    region_context: *mut core::ffi::c_void,
) -> acpi_status {
    let mut ret: i32;
    let data: *mut pcc_data = region_context as *mut pcc_data;
    let usecs_lat: u64;

    reinit_completion(core::ptr::addr_of_mut!((*data).done));

    /* Write to Shared Memory */
    memcpy_toio(
        (*data).pcc_chan.as_ref().unwrap().shmem,
        value as *const core::ffi::c_void,
        (*data).ctx.length,
    );

    ret = mbox_send_message((*data).pcc_chan.as_ref().unwrap().mchan, core::ptr::null_mut());
    if ret < 0 {
        return AE_ERROR;
    }

    /*
     * pcc_chan->latency is just a Nominal value. In reality the remote
     * processor could be much slower to reply. So add an arbitrary
     * amount of wait on top of Nominal.
     */
    usecs_lat = PCC_CMD_WAIT_RETRIES_NUM
        .wrapping_mul((*data).pcc_chan.as_ref().unwrap().latency);
    ret = wait_for_completion_timeout(
        core::ptr::addr_of_mut!((*data).done),
        usecs_to_jiffies(usecs_lat),
    );
    if ret == 0 {
        pr_err!("PCC command executed timeout!\n");
        return AE_TIME;
    }

    mbox_chan_txdone((*data).pcc_chan.as_ref().unwrap().mchan, ret);

    memcpy_fromio(
        value as *mut core::ffi::c_void,
        (*data).pcc_chan.as_ref().unwrap().shmem,
        (*data).ctx.length,
    );

    AE_OK
}

unsafe extern "C" fn acpi_init_pcc() {
    let status: acpi_status;

    status = acpi_install_address_space_handler(
        ACPI_ROOT_OBJECT,
        ACPI_ADR_SPACE_PLATFORM_COMM,
        Some(acpi_pcc_address_space_handler),
        Some(acpi_pcc_address_space_setup),
        core::ptr::addr_of_mut!(pcc_ctx) as *mut core::ffi::c_void,
    );
    if ACPI_FAILURE(status) {
        pr_alert!("OperationRegion handler could not be installed\n");
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
