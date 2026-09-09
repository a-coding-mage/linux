// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Error mapping routines from Samba libsmb/errormap.c
 * Copyright (C) Andrew Tridgell 2001
 * Copyright (C) Luke Kenneth Casson Leighton 1997-2001.
 */

// C dependencies supplied by the surrounding kernel CIFS implementation.

#[inline(always)]
unsafe fn smb1_posix_error_cmp(key: *const core::ffi::c_void, pivot: *const core::ffi::c_void) -> i32 {
    let key = *(key as *const u16);
    let pivot = &*(pivot as *const smb_to_posix_error);
    if key < pivot.smb_err { -1 } else if key > pivot.smb_err { 1 } else { 0 }
}

// Automatically generated mapping tables are supplied by the corresponding C translation units.
extern "C" {
    static mapping_table_ERRDOS: [smb_to_posix_error; 0];
    static mapping_table_ERRSRV: [smb_to_posix_error; 0];
    static ntstatus_to_dos_map: [ntstatus_to_dos_err; 0];
}

#[inline(always)]
unsafe fn ntstatus_to_dos_cmp(key: *const core::ffi::c_void, pivot: *const core::ffi::c_void) -> i32 {
    let key = *(key as *const u32);
    let pivot = &*(pivot as *const ntstatus_to_dos_err);
    if key < pivot.ntstatus { -1 } else if key > pivot.ntstatus { 1 } else { 0 }
}

unsafe fn search_ntstatus_to_dos_map(ntstatus: u32) -> *const ntstatus_to_dos_err {
    __inline_bsearch(&ntstatus as *const u32 as *const core::ffi::c_void,
        ntstatus_to_dos_map.as_ptr() as *const core::ffi::c_void,
        ARRAY_SIZE(ntstatus_to_dos_map), core::mem::size_of::<ntstatus_to_dos_err>(), ntstatus_to_dos_cmp)
}

unsafe fn search_mapping_table_ERRDOS(smb_err: u16) -> *const smb_to_posix_error {
    __inline_bsearch(&smb_err as *const u16 as *const core::ffi::c_void,
        mapping_table_ERRDOS.as_ptr() as *const core::ffi::c_void,
        ARRAY_SIZE(mapping_table_ERRDOS), core::mem::size_of::<smb_to_posix_error>(), smb1_posix_error_cmp)
}

unsafe fn search_mapping_table_ERRSRV(smb_err: u16) -> *const smb_to_posix_error {
    __inline_bsearch(&smb_err as *const u16 as *const core::ffi::c_void,
        mapping_table_ERRSRV.as_ptr() as *const core::ffi::c_void,
        ARRAY_SIZE(mapping_table_ERRSRV), core::mem::size_of::<smb_to_posix_error>(), smb1_posix_error_cmp)
}

pub unsafe fn map_smb_to_linux_error(buf: *mut i8, log_err: bool) -> i32 {
    let smb = buf as *mut smb_hdr;
    let mut rc = -EIO;
    let (smberrclass, smberrcode);
    let mut err_map: *const smb_to_posix_error = core::ptr::null();
    if (*smb).Status.CifsError == 0 { return 0; }
    if (*smb).Flags2 & SMBFLG2_ERR_STATUS != 0 {
        let err = le32_to_cpu((*smb).Status.CifsError);
        let map = search_ntstatus_to_dos_map(err);
        if !map.is_null() {
            if (log_err && err != NT_STATUS_MORE_PROCESSING_REQUIRED) || (cifsFYI & CIFS_RC != 0) {
                pr_notice("Status code returned 0x%08x %s\n", (*map).ntstatus, (*map).nt_errstr);
            }
            smberrclass = (*map).dos_class; smberrcode = (*map).dos_code;
        } else { smberrclass = ERRHRD; smberrcode = ERRgeneral; }
    } else {
        smberrclass = (*smb).Status.DosError.ErrorClass;
        smberrcode = le16_to_cpu((*smb).Status.DosError.Error);
    }
    if smberrclass == ERRDOS { err_map = search_mapping_table_ERRDOS(smberrcode); }
    else if smberrclass == ERRSRV { err_map = search_mapping_table_ERRSRV(smberrcode); }
    if !err_map.is_null() { rc = (*err_map).posix_code; }
    if (*smb).Flags2 & SMBFLG2_ERR_STATUS != 0 {
        let err = le32_to_cpu((*smb).Status.CifsError);
        if err == NT_STATUS_NOT_A_REPARSE_POINT { rc = -ENODATA; }
        else if err == NT_STATUS_PRIVILEGE_NOT_HELD { rc = -EPERM; }
    }
    cifs_dbg(FYI, "Mapping smb error code 0x%x to POSIX err %d\n", le32_to_cpu((*smb).Status.CifsError), rc);
    if rc == -EIO { smb_EIO2(smb_eio_trace_smb1_received_error, le32_to_cpu((*smb).Status.CifsError), le16_to_cpu((*smb).Flags2)); }
    rc
}

pub unsafe fn map_and_check_smb_error(server: *mut TCP_Server_Info, mid: *mut mid_q_entry, log_err: bool) -> i32 {
    let smb = (*mid).resp_buf as *mut smb_hdr;
    let rc = map_smb_to_linux_error(smb as *mut i8, log_err);
    if rc == -EACCES && (*smb).Flags2 & SMBFLG2_ERR_STATUS == 0 {
        let class = (*smb).Status.DosError.ErrorClass;
        let code = le16_to_cpu((*smb).Status.DosError.Error);
        if class == ERRSRV && code == ERRbaduid {
            cifs_dbg(FYI, "Server returned 0x%x, reconnecting session...\n", code);
            cifs_signal_cifsd_for_reconnect(server, false);
        }
    }
    rc
}

unsafe fn check_sorted<T, F: Fn(&T, &T) -> bool>(array: *const T, count: usize, field_ordered: F) -> i32 {
    let mut i = 1;
    while i < count {
        if !field_ordered(&*array.add(i), &*array.add(i - 1)) { return -EINVAL; }
        i += 1;
    }
    0
}

pub unsafe fn smb1_init_maperror() -> i32 {
    let mut rc = check_sorted(ntstatus_to_dos_map.as_ptr(), ARRAY_SIZE(ntstatus_to_dos_map), |a, b| a.ntstatus >= b.ntstatus);
    if rc != 0 { return rc; }
    rc = check_sorted(mapping_table_ERRDOS.as_ptr(), ARRAY_SIZE(mapping_table_ERRDOS), |a, b| a.smb_err >= b.smb_err);
    if rc != 0 { return rc; }
    check_sorted(mapping_table_ERRSRV.as_ptr(), ARRAY_SIZE(mapping_table_ERRSRV), |a, b| a.smb_err >= b.smb_err)
}

#[cfg(CONFIG_SMB1_KUNIT_TESTS)]
pub unsafe fn search_ntstatus_to_dos_map_test(ntstatus: u32) -> *const ntstatus_to_dos_err {
    search_ntstatus_to_dos_map(ntstatus)
}

#[cfg(CONFIG_SMB1_KUNIT_TESTS)]
pub static ntstatus_to_dos_map_test: *const ntstatus_to_dos_err = unsafe { ntstatus_to_dos_map.as_ptr() };
#[cfg(CONFIG_SMB1_KUNIT_TESTS)]
pub static ntstatus_to_dos_num: usize = ARRAY_SIZE(ntstatus_to_dos_map);
#[cfg(CONFIG_SMB1_KUNIT_TESTS)]
pub unsafe fn search_mapping_table_ERRDOS_test(smb_err: u16) -> *const smb_to_posix_error { search_mapping_table_ERRDOS(smb_err) }
#[cfg(CONFIG_SMB1_KUNIT_TESTS)]
pub static mapping_table_ERRDOS_test: *const smb_to_posix_error = unsafe { mapping_table_ERRDOS.as_ptr() };
#[cfg(CONFIG_SMB1_KUNIT_TESTS)]
pub static mapping_table_ERRDOS_num: usize = ARRAY_SIZE(mapping_table_ERRDOS);
#[cfg(CONFIG_SMB1_KUNIT_TESTS)]
pub unsafe fn search_mapping_table_ERRSRV_test(smb_err: u16) -> *const smb_to_posix_error { search_mapping_table_ERRSRV(smb_err) }
#[cfg(CONFIG_SMB1_KUNIT_TESTS)]
pub static mapping_table_ERRSRV_test: *const smb_to_posix_error = unsafe { mapping_table_ERRSRV.as_ptr() };
#[cfg(CONFIG_SMB1_KUNIT_TESTS)]
pub static mapping_table_ERRSRV_num: usize = ARRAY_SIZE(mapping_table_ERRSRV);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
