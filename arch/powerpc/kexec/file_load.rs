// SPDX-License-Identifier: GPL-2.0-only
/*
 * powerpc code to implement the kexec_file_load syscall
 *
 * Copyright (C) 2004  Adam Litke (agl@us.ibm.com)
 * Copyright (C) 2004  IBM Corp.
 * Copyright (C) 2004,2005  Milton D Miller II, IBM Corporation
 * Copyright (C) 2005  R Sharada (sharada@in.ibm.com)
 * Copyright (C) 2006  Mohan Kumar M (mohan@in.ibm.com)
 * Copyright (C) 2016  IBM Corporation
 *
 * Based on kexec-tools' kexec-elf-ppc64.c, fs2dt.c.
 * Heavily modified for the kernel by
 * Thiago Jung Bauermann <bauerman@linux.vnet.ibm.com>.
 */

// Kernel dependencies supplied by the surrounding translation unit.

const SLAVE_CODE_SIZE: usize = 256; /* First 0x100 bytes */

/**
 * setup_kdump_cmdline - Prepend "elfcorehdr=<addr> " to command line
 *                       of kdump kernel for exporting the core.
 * @image:               Kexec image
 * @cmdline:             Command line parameters to update.
 * @cmdline_len:         Length of the cmdline parameters.
 *
 * kdump segment must be setup before calling this function.
 *
 * Returns new cmdline buffer for kdump kernel on success, NULL otherwise.
 */
pub unsafe fn setup_kdump_cmdline(
    image: *mut kimage,
    cmdline: *mut core::ffi::c_char,
    cmdline_len: core::ffi::c_ulong,
) -> *mut core::ffi::c_char {
    let cmdline_ptr: *mut core::ffi::c_char;

    cmdline_ptr = kmalloc(COMMAND_LINE_SIZE, GFP_KERNEL) as *mut core::ffi::c_char;
    if cmdline_ptr.is_null() {
        return core::ptr::null_mut();
    }

    if snprintf(
        cmdline_ptr,
        COMMAND_LINE_SIZE,
        c"elfcorehdr=0x%lx %s".as_ptr(),
        (*image).elf_load_addr,
        if cmdline_len != 0 {
            cmdline
        } else {
            c"".as_ptr()
        },
    ) >= COMMAND_LINE_SIZE as core::ffi::c_int {
        pr_err(c"Prepending elfcorehdr=<addr> exceeds cmdline size\n".as_ptr());
        kfree(cmdline_ptr as *mut core::ffi::c_void);
        return core::ptr::null_mut();
    }

    cmdline_ptr
}

/**
 * setup_purgatory - initialize the purgatory's global variables
 * @image:             kexec image.
 * @slave_code:        Slave code for the purgatory.
 * @fdt:               Flattened device tree for the next kernel.
 * @kernel_load_addr:  Address where the kernel is loaded.
 * @fdt_load_addr:     Address where the flattened device tree is loaded.
 *
 * Return: 0 on success, or negative errno on error.
 */
pub unsafe fn setup_purgatory(
    image: *mut kimage,
    slave_code: *const core::ffi::c_void,
    _fdt: *const core::ffi::c_void,
    kernel_load_addr: core::ffi::c_ulong,
    fdt_load_addr: core::ffi::c_ulong,
) -> core::ffi::c_int {
    let slave_code_buf: *mut core::ffi::c_uint;
    let mut master_entry: core::ffi::c_uint;
    let mut ret: core::ffi::c_int;

    slave_code_buf = kmalloc(SLAVE_CODE_SIZE, GFP_KERNEL) as *mut core::ffi::c_uint;
    if slave_code_buf.is_null() {
        return -ENOMEM;
    }

    // Get the slave code from the new kernel and put it in purgatory.
    ret = kexec_purgatory_get_set_symbol(
        image,
        c"purgatory_start".as_ptr(),
        slave_code_buf as *mut core::ffi::c_void,
        SLAVE_CODE_SIZE,
        true,
    );
    if ret != 0 {
        kfree(slave_code_buf as *mut core::ffi::c_void);
        return ret;
    }

    master_entry = *slave_code_buf;
    memcpy(
        slave_code_buf as *mut core::ffi::c_void,
        slave_code,
        SLAVE_CODE_SIZE,
    );
    *slave_code_buf = master_entry;
    ret = kexec_purgatory_get_set_symbol(
        image,
        c"purgatory_start".as_ptr(),
        slave_code_buf as *mut core::ffi::c_void,
        SLAVE_CODE_SIZE,
        false,
    );
    kfree(slave_code_buf as *mut core::ffi::c_void);

    ret = kexec_purgatory_get_set_symbol(
        image,
        c"kernel".as_ptr(),
        &kernel_load_addr as *const _ as *mut core::ffi::c_void,
        core::mem::size_of_val(&kernel_load_addr),
        false,
    );
    if ret != 0 {
        return ret;
    }
    ret = kexec_purgatory_get_set_symbol(
        image,
        c"dt_offset".as_ptr(),
        &fdt_load_addr as *const _ as *mut core::ffi::c_void,
        core::mem::size_of_val(&fdt_load_addr),
        false,
    );
    if ret != 0 {
        return ret;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
