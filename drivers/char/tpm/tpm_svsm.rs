// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2025 Red Hat, Inc. All Rights Reserved.
 *
 * Driver for the vTPM defined by the AMD SVSM spec [1].
 *
 * The specification defines a protocol that a SEV-SNP guest OS can use to
 * discover and talk to a vTPM emulated by the Secure VM Service Module (SVSM)
 * in the guest context, but at a more privileged level (usually VMPL0).
 *
 * [1] "Secure VM Service Module for SEV-SNP Guests"
 *     Publication # 58019 Revision: 1.00
 */

// Linux kernel dependencies supplied by other translation units.

#[repr(C)]
struct tpm_svsm_priv {
    buffer: *mut core::ffi::c_void,
}

unsafe fn tpm_svsm_send(
    chip: *mut tpm_chip,
    buf: *mut u8,
    bufsiz: usize,
    cmd_len: usize,
) -> i32 {
    let priv_ = dev_get_drvdata(&mut (*(*chip).dev) as *mut device);
    let mut ret: i32;

    ret = svsm_vtpm_cmd_request_fill((*priv_).buffer, 0, buf, cmd_len);
    if ret != 0 {
        return ret;
    }

    /*
     * The SVSM call uses the same buffer for the command and for the
     * response, so after this call, the buffer will contain the response.
     *
     * Note: we have to use an internal buffer because the device in SVSM
     * expects the svsm_vtpm header + data to be physically contiguous.
     */
    ret = snp_svsm_vtpm_send_command((*priv_).buffer);
    if ret != 0 {
        return ret;
    }

    svsm_vtpm_cmd_response_parse((*priv_).buffer, buf, bufsiz)
}

static tpm_class_ops tpm_chip_ops: tpm_class_ops = tpm_class_ops {
    flags: TPM_OPS_AUTO_STARTUP,
    send: Some(tpm_svsm_send),
};

unsafe fn tpm_svsm_probe(pdev: *mut platform_device) -> i32 {
    let dev: *mut device = &mut (*pdev).dev;
    let priv_: *mut tpm_svsm_priv;
    let chip: *mut tpm_chip;
    let mut err: i32;

    priv_ = devm_kmalloc(dev, core::mem::size_of::<tpm_svsm_priv>(), GFP_KERNEL);
    if priv_.is_null() {
        return -ENOMEM;
    }

    /*
     * The maximum buffer supported is one page (see SVSM_VTPM_MAX_BUFFER
     * in tpm_svsm.h).
     */
    (*priv_).buffer = devm_get_free_pages(dev, GFP_KERNEL, 0) as *mut core::ffi::c_void;
    if (*priv_).buffer.is_null() {
        return -ENOMEM;
    }

    chip = tpmm_chip_alloc(dev, &tpm_chip_ops);
    if IS_ERR(chip) {
        return PTR_ERR(chip);
    }

    dev_set_drvdata(&mut (*chip).dev, priv_ as *mut core::ffi::c_void);

    (*chip).flags |= TPM_CHIP_FLAG_SYNC;
    err = tpm2_probe(chip);
    if err != 0 {
        return err;
    }

    err = tpm_chip_register(chip);
    if err != 0 {
        return err;
    }

    dev_info(
        dev,
        "SNP SVSM vTPM %s device\n",
        if ((*chip).flags & TPM_CHIP_FLAG_TPM2) != 0 { "2.0" } else { "1.2" },
    );

    0
}

unsafe fn tpm_svsm_remove(pdev: *mut platform_device) {
    let chip: *mut tpm_chip = platform_get_drvdata(pdev);

    tpm_chip_unregister(chip);
}

/*
 * tpm_svsm_remove() lives in .exit.text. For drivers registered via
 * module_platform_driver_probe() this is ok because they cannot get unbound
 * at runtime. So mark the driver struct with __refdata to prevent modpost
 * triggering a section mismatch warning.
 */
static mut tpm_svsm_driver: platform_driver = platform_driver {
    remove: Some(tpm_svsm_remove),
    driver: driver {
        name: "tpm-svsm",
    },
};

module_platform_driver_probe!(tpm_svsm_driver, tpm_svsm_probe);

module_description!("SNP SVSM vTPM Driver");
module_license!("GPL");
module_alias!("platform:tpm-svsm");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
