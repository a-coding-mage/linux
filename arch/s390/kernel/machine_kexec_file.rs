// SPDX-License-Identifier: GPL-2.0
/*
 * s390 code for kexec_file_load system call
 *
 * Copyright IBM Corp. 2018
 *
 * Author(s): Philipp Rudo <prudo@linux.vnet.ibm.com>
 */

// #define pr_fmt(fmt) "kexec: " fmt
// C dependencies are supplied by the surrounding kernel translation unit.

pub static KEXEC_FILE_LOADERS: [*const kexec_file_ops; 3] = [
    &s390_kexec_elf_ops,
    &s390_kexec_image_ops,
    core::ptr::null(),
];

#[cfg(CONFIG_KEXEC_SIG)]
pub unsafe fn s390_verify_sig(kernel: *const c_char, mut kernel_len: c_ulong) -> c_int {
    let marker_len: c_ulong = (core::mem::size_of::<MODULE_SIGNATURE_MARKER>() - 1) as c_ulong;
    let mut ms: *const module_signature;
    let mut sig_len: c_ulong;
    let mut ret: c_int;

    /* Skip signature verification when not secure IPLed. */
    if !ipl_secure_flag {
        return 0;
    }

    if marker_len > kernel_len {
        return -EKEYREJECTED;
    }

    if libc::memcmp(
        kernel.add((kernel_len - marker_len) as usize) as *const c_void,
        MODULE_SIGNATURE_MARKER.as_ptr() as *const c_void,
        marker_len as usize,
    ) != 0
    {
        return -EKEYREJECTED;
    }
    kernel_len -= marker_len;

    ms = (kernel as *const u8)
        .add((kernel_len - core::mem::size_of::<module_signature>() as c_ulong) as usize)
        as *const module_signature;
    kernel_len -= core::mem::size_of::<module_signature>() as c_ulong;

    sig_len = be32_to_cpu((*ms).sig_len) as c_ulong;
    if sig_len >= kernel_len {
        return -EKEYREJECTED;
    }
    kernel_len -= sig_len;

    if (*ms).id_type != MODULE_SIGNATURE_TYPE_PKCS7 {
        return -EKEYREJECTED;
    }

    if (*ms).algo != 0
        || (*ms).hash != 0
        || (*ms).signer_len != 0
        || (*ms).key_id_len != 0
        || (*ms).__pad[0] != 0
        || (*ms).__pad[1] != 0
        || (*ms).__pad[2] != 0
    {
        return -EBADMSG;
    }

    ret = verify_pkcs7_signature(
        kernel,
        kernel_len as usize,
        kernel.add(kernel_len as usize),
        sig_len as usize,
        VERIFY_USE_SECONDARY_KEYRING,
        VERIFYING_MODULE_SIGNATURE,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
    );
    if ret == -ENOKEY && IS_ENABLED(CONFIG_INTEGRITY_PLATFORM_KEYRING) {
        ret = verify_pkcs7_signature(
            kernel,
            kernel_len as usize,
            kernel.add(kernel_len as usize),
            sig_len as usize,
            VERIFY_USE_PLATFORM_KEYRING,
            VERIFYING_MODULE_SIGNATURE,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        );
    }
    ret
}

unsafe fn kexec_file_update_purgatory(
    image: *mut kimage,
    _data: *mut s390_load_data,
) -> c_int {
    let (entry, kind) = if (*image).type_ == KEXEC_TYPE_CRASH {
        (STARTUP_KDUMP_OFFSET, KEXEC_TYPE_CRASH)
    } else {
        (STARTUP_NORMAL_OFFSET, KEXEC_TYPE_DEFAULT)
    };
    let mut ret = kexec_purgatory_get_set_symbol(
        image,
        b"kernel_entry\0".as_ptr() as *const c_char,
        &entry as *const _ as *mut c_void,
        core::mem::size_of_val(&entry),
        false,
    );
    if ret != 0 {
        return ret;
    }
    ret = kexec_purgatory_get_set_symbol(
        image,
        b"kernel_type\0".as_ptr() as *const c_char,
        &kind as *const _ as *mut c_void,
        core::mem::size_of_val(&kind),
        false,
    );
    if ret != 0 {
        return ret;
    }

    #[cfg(CONFIG_CRASH_DUMP)]
    if (*image).type_ == KEXEC_TYPE_CRASH {
        let crash_size = crashk_res.end - crashk_res.start + 1;
        ret = kexec_purgatory_get_set_symbol(
            image, b"crash_start\0".as_ptr() as *const c_char,
            &mut crashk_res.start as *mut _ as *mut c_void,
            core::mem::size_of_val(&crashk_res.start), false);
        if ret != 0 { return ret; }
        ret = kexec_purgatory_get_set_symbol(
            image, b"crash_size\0".as_ptr() as *const c_char,
            &crash_size as *const _ as *mut c_void,
            core::mem::size_of_val(&crash_size), false);
    }
    ret
}

unsafe fn kexec_file_add_purgatory(image: *mut kimage, data: *mut s390_load_data) -> c_int {
    let mut buf: kexec_buf = core::mem::zeroed();
    (*data).memsz = ALIGN((*data).memsz, PAGE_SIZE);
    buf.image = image;
    buf.mem = (*data).memsz;
    #[cfg(CONFIG_CRASH_DUMP)]
    if (*image).type_ == KEXEC_TYPE_CRASH { buf.mem += crashk_res.start; }
    let ret = kexec_load_purgatory(image, &mut buf);
    if ret != 0 { return ret; }
    (*data).memsz += buf.memsz;
    kexec_file_update_purgatory(image, data)
}

unsafe fn kexec_file_add_initrd(image: *mut kimage, data: *mut s390_load_data) -> c_int {
    let mut buf: kexec_buf = core::mem::zeroed();
    buf.image = image;
    buf.buffer = (*image).initrd_buf;
    buf.bufsz = (*image).initrd_buf_len;
    (*data).memsz = ALIGN((*data).memsz, PAGE_SIZE);
    buf.mem = (*data).memsz;
    #[cfg(CONFIG_CRASH_DUMP)]
    if (*image).type_ == KEXEC_TYPE_CRASH { buf.mem += crashk_res.start; }
    buf.memsz = buf.bufsz;
    (*(*data).parm).initrd_start = (*data).memsz;
    (*(*data).parm).initrd_size = buf.memsz;
    (*data).memsz += buf.memsz;
    let ret = kexec_add_buffer(&mut buf);
    if ret != 0 { return ret; }
    ipl_report_add_component((*data).report, &buf, 0, 0)
}

unsafe fn kexec_file_add_ipl_report(image: *mut kimage, data: *mut s390_load_data) -> c_int {
    let mut buf: kexec_buf = core::mem::zeroed();
    (*data).memsz = ALIGN((*data).memsz, PAGE_SIZE);
    buf.image = image;
    buf.mem = (*data).memsz;
    let mut ptr = __va(ipl_cert_list_addr) as *mut u8;
    let end = ptr.add(ipl_cert_list_size as usize);
    let mut ncerts = 0;
    while ptr < end {
        ncerts += 1;
        let len = *(ptr as *const c_uint);
        ptr = ptr.add(core::mem::size_of::<c_uint>()).add(len as usize);
    }
    let mut addr = (*data).memsz + (*(*data).report).size;
    addr += ncerts * core::mem::size_of::<ipl_rb_certificate_entry>();
    ptr = __va(ipl_cert_list_addr) as *mut u8;
    while ptr < end {
        let len = *(ptr as *const c_uint);
        ptr = ptr.add(core::mem::size_of::<c_uint>());
        ipl_report_add_certificate((*data).report, ptr as *const c_void, addr, len as usize);
        addr += len as usize;
        ptr = ptr.add(len as usize);
    }
    let mut ret = -ENOMEM;
    buf.buffer = ipl_report_finish((*data).report);
    if buf.buffer.is_null() { return ret; }
    buf.bufsz = (*(*data).report).size;
    buf.memsz = buf.bufsz;
    (*image).arch.ipl_buf = buf.buffer;
    (*data).memsz += buf.memsz;
    let lc = (*data).kernel_buf.add(core::mem::offset_of!(lowcore, ipl_parmblock_ptr));
    *(lc as *mut __u32) = buf.mem as __u32;
    #[cfg(CONFIG_CRASH_DUMP)]
    if (*image).type_ == KEXEC_TYPE_CRASH { buf.mem += crashk_res.start; }
    ret = kexec_add_buffer(&mut buf);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
