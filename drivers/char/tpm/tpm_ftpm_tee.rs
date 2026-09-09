// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) Microsoft Corporation
 *
 * Implements a firmware TPM as described here:
 * https://www.microsoft.com/en-us/research/publication/ftpm-software-implementation-tpm-chip/
 *
 * A reference implementation is available here:
 * https://github.com/microsoft/ms-tpm-20-ref/tree/master/Samples/ARM32-FirmwareTPM/optee_ta/fTPM
 */

// Dependencies are supplied by the surrounding kernel translation unit.

/*
 * TA_FTPM_UUID: BC50D971-D4C9-42C4-82CB-343FB7F37896
 *
 * Randomly generated, and must correspond to the GUID on the TA side.
 * Defined here in the reference implementation:
 * https://github.com/microsoft/ms-tpm-20-ref/blob/master/Samples/ARM32-FirmwareTPM/optee_ta/fTPM/include/fTPM.h#L42
 */
static const ftpm_ta_uuid: uuid_t = UUID_INIT!(0xBC50D971, 0xD4C9, 0x42C4,
    0x82, 0xCB, 0x34, 0x3F, 0xB7, 0xF3, 0x78, 0x96);

/**
 * ftpm_tee_tpm_op_send() - send TPM commands through the TEE shared memory
 * and retrieve the response.
 * @chip: the tpm_chip description as specified in driver/char/tpm/tpm.h
 * @buf: the buffer to send and to store the response.
 * @bufsiz: the size of the buffer.
 * @cmd_len: the number of bytes to send.
 *
 * Return:
 *     In case of success, returns the number of bytes received.
 *     On failure, -errno
 */
unsafe fn ftpm_tee_tpm_op_send(
    chip: *mut tpm_chip,
    buf: *mut u8,
    bufsiz: usize,
    cmd_len: usize,
) -> i32 {
    let pvt_data = dev_get_drvdata((*chip).dev.parent);
    let mut resp_len: usize;
    let mut rc: i32;
    let mut temp_buf: *mut u8;
    let resp_header: *mut tpm_header;
    let mut transceive_args: tee_ioctl_invoke_arg = core::mem::zeroed();
    let mut command_params: [tee_param; 4] = core::mem::zeroed();
    let shm = (*pvt_data).shm;

    if cmd_len > MAX_COMMAND_SIZE {
        dev_err!(&(*chip).dev,
            "%s: len=%zd exceeds MAX_COMMAND_SIZE supported by fTPM TA\n",
            "ftpm_tee_tpm_op_send", cmd_len);
        return -EIO;
    }

    transceive_args = tee_ioctl_invoke_arg {
        func: FTPM_OPTEE_TA_SUBMIT_COMMAND,
        session: (*pvt_data).session,
        num_params: 4,
        ..core::mem::zeroed()
    };

    command_params[0] = tee_param {
        attr: TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_INPUT,
        u: tee_param_union { memref: tee_param_memref { shm, size: cmd_len, shm_offs: 0 } },
    };

    temp_buf = tee_shm_get_va(shm, 0);
    if IS_ERR(temp_buf) {
        dev_err!(&(*chip).dev, "%s: tee_shm_get_va failed for transmit\n",
            "ftpm_tee_tpm_op_send");
        return PTR_ERR(temp_buf);
    }
    core::ptr::write_bytes(temp_buf, 0, MAX_COMMAND_SIZE + MAX_RESPONSE_SIZE);
    core::ptr::copy_nonoverlapping(buf, temp_buf, cmd_len);

    command_params[1] = tee_param {
        attr: TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_INOUT,
        u: tee_param_union { memref: tee_param_memref { shm, size: MAX_RESPONSE_SIZE, shm_offs: MAX_COMMAND_SIZE } },
    };

    rc = tee_client_invoke_func((*pvt_data).ctx, &mut transceive_args, command_params.as_mut_ptr());
    if rc < 0 || transceive_args.ret != 0 {
        dev_err!(&(*chip).dev, "%s: SUBMIT_COMMAND invoke error: 0x%x\n",
            "ftpm_tee_tpm_op_send", transceive_args.ret);
        return if rc < 0 { rc } else { transceive_args.ret };
    }

    temp_buf = tee_shm_get_va(shm, command_params[1].u.memref.shm_offs);
    if IS_ERR(temp_buf) {
        dev_err!(&(*chip).dev, "%s: tee_shm_get_va failed for receive\n",
            "ftpm_tee_tpm_op_send");
        return PTR_ERR(temp_buf);
    }

    resp_header = temp_buf as *mut tpm_header;
    resp_len = be32_to_cpu((*resp_header).length) as usize;
    if resp_len < TPM_HEADER_SIZE || resp_len > MAX_RESPONSE_SIZE || resp_len > bufsiz {
        return -EIO;
    }
    core::ptr::copy_nonoverlapping(temp_buf, buf, resp_len);
    resp_len as i32
}

static ftpm_tee_tpm_ops: tpm_class_ops = tpm_class_ops {
    flags: TPM_OPS_AUTO_STARTUP,
    send: Some(ftpm_tee_tpm_op_send),
};

/* Check whether this driver supports the fTPM TA in the TEE instance. */
unsafe fn ftpm_tee_match(ver: *mut tee_ioctl_version_data, _data: *const core::ffi::c_void) -> i32 {
    if (*ver).impl_id == TEE_IMPL_ID_OPTEE && ((*ver).gen_caps & TEE_GEN_CAP_GP) != 0 { 1 } else { 0 }
}

unsafe fn ftpm_tee_probe_generic(dev: *mut device) -> i32 {
    let mut rc: i32;
    let mut chip: *mut tpm_chip;
    let pvt_data = devm_kzalloc(dev, core::mem::size_of::<ftpm_tee_private>(), GFP_KERNEL);
    if pvt_data.is_null() { return -ENOMEM; }
    dev_set_drvdata(dev, pvt_data);
    (*pvt_data).ctx = tee_client_open_context(core::ptr::null_mut(), Some(ftpm_tee_match), core::ptr::null(), core::ptr::null());
    if IS_ERR((*pvt_data).ctx) {
        if PTR_ERR((*pvt_data).ctx) == -ENOENT { return -EPROBE_DEFER; }
        return PTR_ERR((*pvt_data).ctx);
    }
    let mut sess_arg: tee_ioctl_open_session_arg = core::mem::zeroed();
    export_uuid(sess_arg.uuid.as_mut_ptr(), &ftpm_ta_uuid);
    sess_arg.clnt_login = TEE_IOCTL_LOGIN_PUBLIC;
    rc = tee_client_open_session((*pvt_data).ctx, &mut sess_arg, core::ptr::null_mut());
    if rc < 0 || sess_arg.ret != 0 { rc = -EINVAL; goto!(out_tee_session); }
    (*pvt_data).session = sess_arg.session;
    (*pvt_data).shm = tee_shm_alloc_kernel_buf((*pvt_data).ctx, MAX_COMMAND_SIZE + MAX_RESPONSE_SIZE);
    if IS_ERR((*pvt_data).shm) { rc = -ENOMEM; goto!(out_shm_alloc); }
    chip = tpm_chip_alloc(dev, &ftpm_tee_tpm_ops);
    if IS_ERR(chip) { rc = PTR_ERR(chip); goto!(out_chip_alloc); }
    (*pvt_data).chip = chip;
    (*chip).flags |= TPM_CHIP_FLAG_TPM2 | TPM_CHIP_FLAG_SYNC;
    rc = tpm_chip_register(chip);
    if rc != 0 { goto!(out_chip); }
    return 0;
out_chip:
    put_device(&mut (*(*pvt_data).chip).dev);
out_chip_alloc:
    tee_shm_free((*pvt_data).shm);
out_shm_alloc:
    tee_client_close_session((*pvt_data).ctx, (*pvt_data).session);
out_tee_session:
    tee_client_close_context((*pvt_data).ctx);
    rc
}

unsafe fn ftpm_tee_probe(tcdev: *mut tee_client_device) -> i32 { ftpm_tee_probe_generic(&mut (*tcdev).dev) }
unsafe fn ftpm_plat_tee_probe(pdev: *mut platform_device) -> i32 { ftpm_tee_probe_generic(&mut (*pdev).dev) }

unsafe fn ftpm_tee_remove_generic(dev: *mut device) {
    let pvt_data = dev_get_drvdata(dev);
    tpm_chip_unregister((*pvt_data).chip);
    put_device(&mut (*(*pvt_data).chip).dev);
    tee_shm_free((*pvt_data).shm);
    tee_client_close_session((*pvt_data).ctx, (*pvt_data).session);
    tee_client_close_context((*pvt_data).ctx);
}
unsafe fn ftpm_tee_remove(tcdev: *mut tee_client_device) { ftpm_tee_remove_generic(&mut (*tcdev).dev); }
unsafe fn ftpm_plat_tee_remove(pdev: *mut platform_device) { ftpm_tee_remove_generic(&mut (*pdev).dev); }
unsafe fn ftpm_plat_tee_shutdown(pdev: *mut platform_device) {
    let pvt_data = dev_get_drvdata(&mut (*pdev).dev);
    tee_shm_free((*pvt_data).shm);
    tee_client_close_session((*pvt_data).ctx, (*pvt_data).session);
    tee_client_close_context((*pvt_data).ctx);
}

static of_ftpm_tee_ids: [of_device_id; 2] = [
    of_device_id { compatible: c"microsoft,ftpm", ..of_device_id::zeroed() },
    of_device_id::zeroed(),
];
MODULE_DEVICE_TABLE!(of, of_ftpm_tee_ids);

static mut ftpm_tee_plat_driver: platform_driver = platform_driver { driver: device_driver { name: c"ftpm-tee", of_match_table: of_ftpm_tee_ids.as_ptr(), ..device_driver::zeroed() }, shutdown: Some(ftpm_plat_tee_shutdown), probe: Some(ftpm_plat_tee_probe), remove: Some(ftpm_plat_tee_remove) };

/* UUID of the fTPM TA */
static optee_ftpm_id_table: [tee_client_device_id; 2] = [
    tee_client_device_id { uuid: UUID_INIT!(0xbc50d971, 0xd4c9, 0x42c4, 0x82, 0xcb, 0x34, 0x3f, 0xb7, 0xf3, 0x78, 0x96) },
    tee_client_device_id::zeroed(),
];
MODULE_DEVICE_TABLE!(tee, optee_ftpm_id_table);

static mut ftpm_tee_driver: tee_client_driver = tee_client_driver { probe: Some(ftpm_tee_probe), remove: Some(ftpm_tee_remove), id_table: optee_ftpm_id_table.as_ptr(), driver: device_driver { name: c"optee-ftpm", ..device_driver::zeroed() } };

unsafe fn ftpm_mod_init() -> i32 {
    let mut rc = platform_driver_register(&mut ftpm_tee_plat_driver);
    if rc != 0 { return rc; }
    rc = tee_client_driver_register(&mut ftpm_tee_driver);
    if rc != 0 { platform_driver_unregister(&mut ftpm_tee_plat_driver); return rc; }
    0
}
unsafe fn ftpm_mod_exit() {
    platform_driver_unregister(&mut ftpm_tee_plat_driver);
    tee_client_driver_unregister(&mut ftpm_tee_driver);
}

module_init!(ftpm_mod_init);
module_exit!(ftpm_mod_exit);
MODULE_AUTHOR!(c"Thirupathaiah Annapureddy <thiruan@microsoft.com>");
MODULE_DESCRIPTION!(c"TPM Driver for fTPM TA in TEE");
MODULE_LICENSE!(c"GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
