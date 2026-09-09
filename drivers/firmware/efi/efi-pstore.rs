// SPDX-License-Identifier: GPL-2.0+

// Dependencies supplied by the surrounding kernel Rust environment:
// linux::efi, linux::module, linux::pstore, linux::slab, linux::ucs2_string

const DUMP_NAME_LEN: usize = 66;

static mut record_size: c_uint = 1024;

const PSTORE_EFI_ATTRIBUTES: u32 =
    EFI_VARIABLE_NON_VOLATILE | EFI_VARIABLE_BOOTSERVICE_ACCESS | EFI_VARIABLE_RUNTIME_ACCESS;

static mut pstore_disable: bool = IS_ENABLED_CONFIG_EFI_VARS_PSTORE_DEFAULT_DISABLE;

unsafe extern "C" {
    fn efivars_pstore_init() -> c_int;
    fn efivars_pstore_exit();
}

unsafe extern "C" fn efi_pstore_disable_set(
    val: *const c_char,
    kp: *const kernel_param,
) -> c_int {
    let old_pstore_disable = pstore_disable;
    let err = param_set_bool(val, kp);
    if err != 0 {
        return err;
    }

    if old_pstore_disable != pstore_disable {
        if pstore_disable {
            efivars_pstore_exit();
        } else {
            efivars_pstore_init();
        }
    }

    0
}

static pstore_disable_ops: kernel_param_ops = kernel_param_ops {
    set: Some(efi_pstore_disable_set),
    get: Some(param_get_bool),
};

unsafe extern "C" fn efi_pstore_open(psi: *mut pstore_info) -> c_int {
    let err = efivar_lock();
    if err != 0 {
        return err;
    }

    (*psi).data = kzalloc(record_size as usize, GFP_KERNEL);
    if (*psi).data.is_null() {
        efivar_unlock();
        return -ENOMEM;
    }

    0
}

unsafe extern "C" fn efi_pstore_close(psi: *mut pstore_info) -> c_int {
    efivar_unlock();
    kfree((*psi).data);
    0
}

#[inline]
fn generic_id(timestamp: u64, part: c_uint, count: c_int) -> u64 {
    (timestamp.wrapping_mul(100).wrapping_add(part as u64))
        .wrapping_mul(1000)
        .wrapping_add(count as u64)
}

unsafe extern "C" fn efi_pstore_read_func(
    record: *mut pstore_record,
    varname: *mut efi_char16_t,
) -> c_int {
    let mut wlen: c_ulong;
    let mut size = record_size as c_ulong;
    let mut name = [0i8; DUMP_NAME_LEN];
    let mut data_type = 0i8;
    let mut status: efi_status_t;
    let mut cnt = 0i32;
    let mut part = 0u32;
    let mut time = 0u64;

    ucs2_as_utf8(name.as_mut_ptr(), varname, DUMP_NAME_LEN);

    if sscanf(name.as_ptr(), c"dump-type%u-%u-%d-%llu-%c".as_ptr(), &mut (*record).type_, &mut part, &mut cnt, &mut time, &mut data_type) == 5 {
        (*record).id = generic_id(time, part, cnt);
        (*record).part = part;
        (*record).count = cnt;
        (*record).time.tv_sec = time;
        (*record).time.tv_nsec = 0;
        (*record).compressed = data_type == b'C' as i8;
        (*record).ecc_notice_size = 0;
    } else if sscanf(name.as_ptr(), c"dump-type%u-%u-%d-%llu".as_ptr(), &mut (*record).type_, &mut part, &mut cnt, &mut time) == 4 {
        (*record).id = generic_id(time, part, cnt);
        (*record).part = part;
        (*record).count = cnt;
        (*record).time.tv_sec = time;
        (*record).time.tv_nsec = 0;
        (*record).compressed = false;
        (*record).ecc_notice_size = 0;
    } else if sscanf(name.as_ptr(), c"dump-type%u-%u-%llu".as_ptr(), &mut (*record).type_, &mut part, &mut time) == 3 {
        /* Check if an old format, which doesn't support holding multiple logs, remains. */
        (*record).id = generic_id(time, part, 0);
        (*record).part = part;
        (*record).count = 0;
        (*record).time.tv_sec = time;
        (*record).time.tv_nsec = 0;
        (*record).compressed = false;
        (*record).ecc_notice_size = 0;
    } else {
        return 0;
    }

    (*record).buf = kmalloc(size as usize, GFP_KERNEL);
    if (*record).buf.is_null() {
        return -ENOMEM;
    }

    status = efivar_get_variable(varname, &LINUX_EFI_CRASH_GUID, core::ptr::null_mut(), &mut size, (*record).buf);
    if status != EFI_SUCCESS {
        kfree((*record).buf);
        return efi_status_to_err(status);
    }

    wlen = (ucs2_strnlen(varname, DUMP_NAME_LEN) + 1) * core::mem::size_of::<efi_char16_t>() as c_ulong;
    (*record).priv_ = kmemdup(varname as *const c_void, wlen as usize, GFP_KERNEL);
    if (*record).priv_.is_null() {
        kfree((*record).buf);
        return -ENOMEM;
    }

    size as c_int
}

unsafe extern "C" fn efi_pstore_read(record: *mut pstore_record) -> ssize_t {
    let varname = (*record).psi.data as *mut efi_char16_t;
    let mut guid = LINUX_EFI_CRASH_GUID;
    let mut varname_size: c_ulong;

    loop {
        varname_size = 512;
        let status = efivar_get_next_variable(&mut varname_size, varname, &mut guid);
        if status == EFI_NOT_FOUND { return 0; }
        if status != EFI_SUCCESS { return efi_status_to_err(status) as ssize_t; }
        if efi_guidcmp(guid, LINUX_EFI_CRASH_GUID) != 0 { continue; }
        return efi_pstore_read_func(record, varname) as ssize_t;
    }
}

unsafe extern "C" fn efi_pstore_write(record: *mut pstore_record) -> c_int {
    let mut name = [0i8; DUMP_NAME_LEN];
    let mut efi_name = [0 as efi_char16_t; DUMP_NAME_LEN];
    (*record).id = generic_id((*record).time.tv_sec, (*record).part, (*record).count);
    snprintf(name.as_mut_ptr(), name.len(), c"dump-type%u-%u-%d-%lld-%c".as_ptr(), (*record).type_, (*record).part, (*record).count, (*record).time.tv_sec as c_longlong, if (*record).compressed { b'C' } else { b'D' });
    for i in 0..DUMP_NAME_LEN { efi_name[i] = name[i] as efi_char16_t; }
    if efivar_trylock() != 0 { return -EBUSY; }
    let status = efivar_set_variable_locked(efi_name.as_ptr(), &LINUX_EFI_CRASH_GUID, PSTORE_EFI_ATTRIBUTES, (*record).size, (*record).psi.buf, true);
    efivar_unlock();
    efi_status_to_err(status)
}

unsafe extern "C" fn efi_pstore_erase(record: *mut pstore_record) -> c_int {
    let status = efivar_set_variable((*record).priv_ as *mut efi_char16_t, &LINUX_EFI_CRASH_GUID, PSTORE_EFI_ATTRIBUTES, 0, core::ptr::null_mut());
    if status != EFI_SUCCESS && status != EFI_NOT_FOUND { return efi_status_to_err(status); }
    0
}

static mut efi_pstore_info: pstore_info = pstore_info {
    owner: THIS_MODULE,
    name: KBUILD_MODNAME,
    flags: PSTORE_FLAGS_DMESG,
    open: Some(efi_pstore_open),
    close: Some(efi_pstore_close),
    read: Some(efi_pstore_read),
    write: Some(efi_pstore_write),
    erase: Some(efi_pstore_erase),
    ..pstore_info::ZERO
};

unsafe extern "C" fn efivars_pstore_init_impl() -> c_int {
    if !efivar_supports_writes() || pstore_disable { return 0; }
    if record_size < 1024 { record_size = 1024; }
    efi_pstore_info.buf = kmalloc(record_size as usize, GFP_KERNEL);
    if efi_pstore_info.buf.is_null() { return -ENOMEM; }
    efi_pstore_info.bufsize = record_size as usize;
    if pstore_register(&mut efi_pstore_info) != 0 {
        kfree(efi_pstore_info.buf);
        efi_pstore_info.buf = core::ptr::null_mut();
        efi_pstore_info.bufsize = 0;
    }
    0
}

unsafe extern "C" fn efivars_pstore_exit_impl() {
    if efi_pstore_info.bufsize == 0 { return; }
    pstore_unregister(&mut efi_pstore_info);
    kfree(efi_pstore_info.buf);
    efi_pstore_info.buf = core::ptr::null_mut();
    efi_pstore_info.bufsize = 0;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
