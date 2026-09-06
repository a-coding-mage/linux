// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2013 Politecnico di Torino, Italy
 *                    TORSEC group -- https://security.polito.it
 *
 * Author: Roberto Sassu <roberto.sassu@polito.it>
 *
 * File: ima_template.c
 *      Helpers to manage template descriptors.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u8 = core::ffi::c_uchar;
type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;
type loff_t = i64;
type gfp_t = core::ffi::c_uint;

const GFP_KERNEL: gfp_t = 0;
const GFP_NOFS: gfp_t = 0;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const HASH_ALGO_SHA1: c_int = 0;
const HASH_ALGO_MD5: c_int = 1;
const IMA_TEMPLATE_NUM_FIELDS_MAX: usize = 16;
const IMA_TEMPLATE_FIELD_ID_MAX_LEN: usize = 16;
const TPM_DIGEST_SIZE: usize = 20;
const HDR__LAST: usize = 4;
const HDR_PCR: usize = 0;
const HDR_DIGEST: usize = 1;
const HDR_TEMPLATE_NAME: usize = 2;
const HDR_TEMPLATE_DATA: usize = 3;
const ENFORCE_FIELDS: c_int = 1;
const ENFORCE_BUFEND: c_int = 2;
const ULONG_MAX: c_ulong = c_ulong::MAX;

/*
 * Used when restoring measurements carried over from a kexec. 'd' and 'n' don't
 * need to be accounted for since they shouldn't be defined in the same template
 * description as 'd-ng' and 'n-ng' respectively.
 */
const MAX_TEMPLATE_NAME_LEN: usize =
    b"d-ng|n-ng|evmsig|xattrnames|xattrlengths|xattrvalues|iuid|igid|imode\0".len();

const IMA_TEMPLATE_IMA_NAME: *const c_char = c"ima".as_ptr();
const IMA_TEMPLATE_IMA_FMT: *const c_char = c"d|n".as_ptr();
const CONFIG_IMA_DEFAULT_TEMPLATE: *const c_char = c"ima-ng".as_ptr();

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ima_template_field {
    pub field_id: *const c_char,
    pub field_init: Option<unsafe extern "C" fn()>,
    pub field_show: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct ima_template_desc {
    pub name: *const c_char,
    pub fmt: *const c_char,
    pub fields: *mut *const ima_template_field,
    pub num_fields: c_int,
    pub list: list_head,
}

#[repr(C)]
pub struct ima_field_data {
    pub len: u32,
    pub data: *mut u8,
}

#[repr(C)]
pub struct tpm_digest {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ima_template_entry {
    pub digests: *mut tpm_digest,
    pub template_desc: *mut ima_template_desc,
    pub template_data_len: c_int,
    pub pcr: u32,
    pub template_data: [ima_field_data; 0],
}

#[repr(C)]
pub struct ima_kexec_hdr {
    pub version: u16,
    pub buffer_size: u64,
    pub count: u64,
}

unsafe extern "C" {
    static mut ima_hash_algo: c_int;
    static mut ima_canonical_fmt: bool_;
    static mut ima_tpm_chip: *mut c_void;
    static mut ima_extra_slots: c_int;

    fn ima_eventdigest_init();
    fn ima_eventname_init();
    fn ima_eventdigest_ng_init();
    fn ima_eventdigest_ngv2_init();
    fn ima_eventname_ng_init();
    fn ima_eventsig_init();
    fn ima_eventbuf_init();
    fn ima_eventdigest_modsig_init();
    fn ima_eventmodsig_init();
    fn ima_eventevmsig_init();
    fn ima_eventinodeuid_init();
    fn ima_eventinodegid_init();
    fn ima_eventinodemode_init();
    fn ima_eventinodexattrnames_init();
    fn ima_eventinodexattrlengths_init();
    fn ima_eventinodexattrvalues_init();

    fn ima_show_template_digest();
    fn ima_show_template_string();
    fn ima_show_template_digest_ng();
    fn ima_show_template_digest_ngv2();
    fn ima_show_template_sig();
    fn ima_show_template_buf();
    fn ima_show_template_uint();

    fn strlen(s: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strchrnul(s: *const c_char, c: c_int) -> *const c_char;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn kzalloc(size: usize, flags: gfp_t) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn kstrdup(s: *const c_char, flags: gfp_t) -> *mut c_char;
    fn pr_err(fmt: *const c_char, ...);

    fn list_empty(head: *const list_head) -> c_int;
    fn list_add_tail_rcu(new: *mut list_head, head: *mut list_head);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn bitmap_zero(dst: *mut c_ulong, nbits: c_int);
    fn bitmap_set(map: *mut c_ulong, start: c_int, nr: c_int);
    fn le16_to_cpu(x: u16) -> u16;
    fn le32_to_cpu(x: u32) -> u32;
    fn le64_to_cpu(x: u64) -> u64;
    fn NR_BANKS(chip: *mut c_void) -> c_int;

    fn ima_parse_buf(
        bufstartp: *mut c_void,
        bufendp: *mut c_void,
        bufcurp: *mut *mut c_void,
        maxfields: c_int,
        fields: *mut ima_field_data,
        curfields: *mut c_int,
        fields_mask: *mut c_ulong,
        enforce_mask: c_int,
        bufname: *const c_char,
    ) -> c_int;
    fn ima_free_template_entry(entry: *mut ima_template_entry);
    fn ima_calc_field_array_hash(
        template_data: *mut ima_field_data,
        entry: *mut ima_template_entry,
    ) -> c_int;
    fn ima_restore_measurement_entry(entry: *mut ima_template_entry) -> c_int;
}

static mut builtin_templates: [ima_template_desc; 9] = [
    ima_template_desc { name: IMA_TEMPLATE_IMA_NAME, fmt: IMA_TEMPLATE_IMA_FMT, fields: ptr::null_mut(), num_fields: 0, list: list_head { next: ptr::null_mut(), prev: ptr::null_mut() } },
    ima_template_desc { name: c"ima-ng".as_ptr(), fmt: c"d-ng|n-ng".as_ptr(), fields: ptr::null_mut(), num_fields: 0, list: list_head { next: ptr::null_mut(), prev: ptr::null_mut() } },
    ima_template_desc { name: c"ima-sig".as_ptr(), fmt: c"d-ng|n-ng|sig".as_ptr(), fields: ptr::null_mut(), num_fields: 0, list: list_head { next: ptr::null_mut(), prev: ptr::null_mut() } },
    ima_template_desc { name: c"ima-ngv2".as_ptr(), fmt: c"d-ngv2|n-ng".as_ptr(), fields: ptr::null_mut(), num_fields: 0, list: list_head { next: ptr::null_mut(), prev: ptr::null_mut() } },
    ima_template_desc { name: c"ima-sigv2".as_ptr(), fmt: c"d-ngv2|n-ng|sig".as_ptr(), fields: ptr::null_mut(), num_fields: 0, list: list_head { next: ptr::null_mut(), prev: ptr::null_mut() } },
    ima_template_desc { name: c"ima-buf".as_ptr(), fmt: c"d-ng|n-ng|buf".as_ptr(), fields: ptr::null_mut(), num_fields: 0, list: list_head { next: ptr::null_mut(), prev: ptr::null_mut() } },
    ima_template_desc { name: c"ima-modsig".as_ptr(), fmt: c"d-ng|n-ng|sig|d-modsig|modsig".as_ptr(), fields: ptr::null_mut(), num_fields: 0, list: list_head { next: ptr::null_mut(), prev: ptr::null_mut() } },
    ima_template_desc { name: c"evm-sig".as_ptr(), fmt: c"d-ng|n-ng|evmsig|xattrnames|xattrlengths|xattrvalues|iuid|igid|imode".as_ptr(), fields: ptr::null_mut(), num_fields: 0, list: list_head { next: ptr::null_mut(), prev: ptr::null_mut() } },
    ima_template_desc { name: c"".as_ptr(), fmt: c"".as_ptr(), fields: ptr::null_mut(), num_fields: 0, list: list_head { next: ptr::null_mut(), prev: ptr::null_mut() } }, /* placeholder for a custom format */
];

static mut defined_templates: list_head = list_head {
    next: unsafe { &raw mut defined_templates },
    prev: unsafe { &raw mut defined_templates },
};
static mut template_list: spinlock_t = spinlock_t { _private: [] };
static mut template_setup_done: c_int = 0;

static supported_fields: [ima_template_field; 16] = [
    ima_template_field { field_id: c"d".as_ptr(), field_init: Some(ima_eventdigest_init), field_show: Some(ima_show_template_digest) },
    ima_template_field { field_id: c"n".as_ptr(), field_init: Some(ima_eventname_init), field_show: Some(ima_show_template_string) },
    ima_template_field { field_id: c"d-ng".as_ptr(), field_init: Some(ima_eventdigest_ng_init), field_show: Some(ima_show_template_digest_ng) },
    ima_template_field { field_id: c"d-ngv2".as_ptr(), field_init: Some(ima_eventdigest_ngv2_init), field_show: Some(ima_show_template_digest_ngv2) },
    ima_template_field { field_id: c"n-ng".as_ptr(), field_init: Some(ima_eventname_ng_init), field_show: Some(ima_show_template_string) },
    ima_template_field { field_id: c"sig".as_ptr(), field_init: Some(ima_eventsig_init), field_show: Some(ima_show_template_sig) },
    ima_template_field { field_id: c"buf".as_ptr(), field_init: Some(ima_eventbuf_init), field_show: Some(ima_show_template_buf) },
    ima_template_field { field_id: c"d-modsig".as_ptr(), field_init: Some(ima_eventdigest_modsig_init), field_show: Some(ima_show_template_digest_ng) },
    ima_template_field { field_id: c"modsig".as_ptr(), field_init: Some(ima_eventmodsig_init), field_show: Some(ima_show_template_sig) },
    ima_template_field { field_id: c"evmsig".as_ptr(), field_init: Some(ima_eventevmsig_init), field_show: Some(ima_show_template_sig) },
    ima_template_field { field_id: c"iuid".as_ptr(), field_init: Some(ima_eventinodeuid_init), field_show: Some(ima_show_template_uint) },
    ima_template_field { field_id: c"igid".as_ptr(), field_init: Some(ima_eventinodegid_init), field_show: Some(ima_show_template_uint) },
    ima_template_field { field_id: c"imode".as_ptr(), field_init: Some(ima_eventinodemode_init), field_show: Some(ima_show_template_uint) },
    ima_template_field { field_id: c"xattrnames".as_ptr(), field_init: Some(ima_eventinodexattrnames_init), field_show: Some(ima_show_template_string) },
    ima_template_field { field_id: c"xattrlengths".as_ptr(), field_init: Some(ima_eventinodexattrlengths_init), field_show: Some(ima_show_template_sig) },
    ima_template_field { field_id: c"xattrvalues".as_ptr(), field_init: Some(ima_eventinodexattrvalues_init), field_show: Some(ima_show_template_sig) },
];

static mut ima_template: *mut ima_template_desc = ptr::null_mut();
static mut ima_buf_template: *mut ima_template_desc = ptr::null_mut();

unsafe fn kmalloc_objs<T>(count: usize) -> *mut T {
    kzalloc(size_of::<T>().wrapping_mul(count), GFP_KERNEL) as *mut T
}

unsafe fn kzalloc_obj<T>() -> *mut T {
    kzalloc(size_of::<T>(), GFP_KERNEL) as *mut T
}

unsafe fn kzalloc_objs<T>(count: usize, flags: gfp_t) -> *mut T {
    kzalloc(size_of::<T>().wrapping_mul(count), flags) as *mut T
}

unsafe fn kzalloc_flex_entry(num_fields: c_int, flags: gfp_t) -> *mut ima_template_entry {
    let base = size_of::<ima_template_entry>();
    let flex = size_of::<ima_field_data>().wrapping_mul(num_fields as usize);
    kzalloc(base.wrapping_add(flex), flags) as *mut ima_template_entry
}

/**
 * ima_template_has_modsig - Check whether template has modsig-related fields.
 * @ima_template: IMA template to check.
 *
 * Tells whether the given template has fields referencing a file's appended
 * signature.
 */
#[no_mangle]
pub unsafe extern "C" fn ima_template_has_modsig(
    ima_template: *const ima_template_desc,
) -> bool_ {
    let mut i: c_int = 0;

    while i < (*ima_template).num_fields {
        let field = *(*ima_template).fields.add(i as usize);
        if strcmp((*field).field_id, c"modsig".as_ptr()) == 0
            || strcmp((*field).field_id, c"d-modsig".as_ptr()) == 0
        {
            return true;
        }
        i += 1;
    }

    false
}

unsafe extern "C" fn ima_template_setup(str_: *mut c_char) -> c_int {
    let mut template_desc: *mut ima_template_desc;
    let template_len: c_int = strlen(str_) as c_int;

    if template_setup_done != 0 {
        return 1;
    }

    if ima_template.is_null() {
        ima_init_template_list();
    }

    /*
     * Verify that a template with the supplied name exists.
     * If not, use CONFIG_IMA_DEFAULT_TEMPLATE.
     */
    template_desc = lookup_template_desc(str_);
    if template_desc.is_null() {
        pr_err(
            c"template %s not found, using %s\n".as_ptr(),
            str_,
            CONFIG_IMA_DEFAULT_TEMPLATE,
        );
        return 1;
    }

    /*
     * Verify whether the current hash algorithm is supported
     * by the 'ima' template.
     */
    if template_len == 3
        && strcmp(str_, IMA_TEMPLATE_IMA_NAME) == 0
        && ima_hash_algo != HASH_ALGO_SHA1
        && ima_hash_algo != HASH_ALGO_MD5
    {
        pr_err(c"template does not support hash alg\n".as_ptr());
        return 1;
    }

    ima_template = template_desc;
    template_setup_done = 1;
    1
}
/* __setup("ima_template=", ima_template_setup); */

unsafe extern "C" fn ima_template_fmt_setup(str_: *mut c_char) -> c_int {
    let num_templates: c_int = builtin_templates.len() as c_int;

    if template_setup_done != 0 {
        return 1;
    }

    if template_desc_init_fields(str_, ptr::null_mut(), ptr::null_mut()) < 0 {
        pr_err(
            c"format string '%s' not valid, using template %s\n".as_ptr(),
            str_,
            CONFIG_IMA_DEFAULT_TEMPLATE,
        );
        return 1;
    }

    builtin_templates[(num_templates - 1) as usize].fmt = str_;
    ima_template = builtin_templates.as_mut_ptr().add((num_templates - 1) as usize);
    template_setup_done = 1;

    1
}
/* __setup("ima_template_fmt=", ima_template_fmt_setup); */

#[no_mangle]
pub unsafe extern "C" fn lookup_template_desc(name: *const c_char) -> *mut ima_template_desc {
    let mut template_desc: *mut ima_template_desc;
    let mut found: c_int = 0;

    rcu_read_lock();
    template_desc = builtin_templates.as_mut_ptr();
    while template_desc < builtin_templates.as_mut_ptr().add(builtin_templates.len()) {
        if strcmp((*template_desc).name, name) == 0 || strcmp((*template_desc).fmt, name) == 0 {
            found = 1;
            break;
        }
        template_desc = template_desc.add(1);
    }
    rcu_read_unlock();
    if found != 0 { template_desc } else { ptr::null_mut() }
}

unsafe fn lookup_template_field(field_id: *const c_char) -> *const ima_template_field {
    let mut i: usize = 0;

    while i < supported_fields.len() {
        if strncmp(
            supported_fields[i].field_id,
            field_id,
            IMA_TEMPLATE_FIELD_ID_MAX_LEN,
        ) == 0
        {
            return &supported_fields[i];
        }
        i += 1;
    }
    ptr::null()
}

unsafe fn template_fmt_size(template_fmt: *const c_char) -> c_int {
    let mut c: c_char;
    let template_fmt_len: c_int = strlen(template_fmt) as c_int;
    let mut i: c_int = 0;
    let mut j: c_int = 0;

    while i < template_fmt_len {
        c = *template_fmt.add(i as usize);
        if c == b'|' as c_char {
            j += 1;
        }
        i += 1;
    }

    j + 1
}

#[no_mangle]
pub unsafe extern "C" fn template_desc_init_fields(
    template_fmt: *const c_char,
    fields: *mut *mut *const ima_template_field,
    num_fields: *mut c_int,
) -> c_int {
    let mut template_fmt_ptr: *const c_char;
    let mut found_fields: [*const ima_template_field; IMA_TEMPLATE_NUM_FIELDS_MAX] =
        [ptr::null(); IMA_TEMPLATE_NUM_FIELDS_MAX];
    let template_num_fields: c_int;
    let mut i: c_int;
    let mut len: isize;

    if !num_fields.is_null() && *num_fields > 0 {
        /* already initialized? */
        return 0;
    }

    template_num_fields = template_fmt_size(template_fmt);

    if template_num_fields > IMA_TEMPLATE_NUM_FIELDS_MAX as c_int {
        pr_err(
            c"format string '%s' contains too many fields\n".as_ptr(),
            template_fmt,
        );
        return -EINVAL;
    }

    i = 0;
    template_fmt_ptr = template_fmt;
    while i < template_num_fields {
        let mut tmp_field_id: [c_char; IMA_TEMPLATE_FIELD_ID_MAX_LEN + 1] =
            [0; IMA_TEMPLATE_FIELD_ID_MAX_LEN + 1];

        len = strchrnul(template_fmt_ptr, b'|' as c_int).offset_from(template_fmt_ptr);
        if len == 0 || len > IMA_TEMPLATE_FIELD_ID_MAX_LEN as isize {
            pr_err(c"Invalid field with length %d\n".as_ptr(), len as c_int);
            return -EINVAL;
        }

        memcpy(
            tmp_field_id.as_mut_ptr() as *mut c_void,
            template_fmt_ptr as *const c_void,
            len as usize,
        );
        tmp_field_id[len as usize] = 0;
        found_fields[i as usize] = lookup_template_field(tmp_field_id.as_ptr());
        if found_fields[i as usize].is_null() {
            pr_err(c"field '%s' not found\n".as_ptr(), tmp_field_id.as_ptr());
            return -ENOENT;
        }

        i += 1;
        template_fmt_ptr = template_fmt_ptr.add(len as usize + 1);
    }

    if !fields.is_null() && !num_fields.is_null() {
        *fields = kmalloc_objs::<*const ima_template_field>(i as usize);
        if (*fields).is_null() {
            return -ENOMEM;
        }

        memcpy(
            *fields as *mut c_void,
            found_fields.as_ptr() as *const c_void,
            i as usize * size_of::<*const ima_template_field>(),
        );
        *num_fields = i;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn ima_init_template_list() {
    let mut i: c_int;

    if list_empty(&raw const defined_templates) == 0 {
        return;
    }

    spin_lock(&raw mut template_list);
    i = 0;
    while i < builtin_templates.len() as c_int {
        list_add_tail_rcu(
            &mut builtin_templates[i as usize].list,
            &raw mut defined_templates,
        );
        i += 1;
    }
    spin_unlock(&raw mut template_list);
}

#[no_mangle]
pub unsafe extern "C" fn ima_template_desc_current() -> *mut ima_template_desc {
    if ima_template.is_null() {
        ima_init_template_list();
        ima_template = lookup_template_desc(CONFIG_IMA_DEFAULT_TEMPLATE);
    }
    ima_template
}

#[no_mangle]
pub unsafe extern "C" fn ima_template_desc_buf() -> *mut ima_template_desc {
    if ima_buf_template.is_null() {
        ima_init_template_list();
        ima_buf_template = lookup_template_desc(c"ima-buf".as_ptr());
    }
    ima_buf_template
}

#[no_mangle]
pub unsafe extern "C" fn ima_init_template() -> c_int {
    let mut template: *mut ima_template_desc = ima_template_desc_current();
    let mut result: c_int;

    result = template_desc_init_fields(
        (*template).fmt,
        &mut (*template).fields,
        &mut (*template).num_fields,
    );
    if result < 0 {
        pr_err(
            c"template %s init failed, result: %d\n".as_ptr(),
            if strlen((*template).name) != 0 {
                (*template).name
            } else {
                (*template).fmt
            },
            result,
        );
        return result;
    }

    template = ima_template_desc_buf();
    if template.is_null() {
        pr_err(c"Failed to get ima-buf template\n".as_ptr());
        return -EINVAL;
    }

    result = template_desc_init_fields(
        (*template).fmt,
        &mut (*template).fields,
        &mut (*template).num_fields,
    );
    if result < 0 {
        pr_err(
            c"template %s init failed, result: %d\n".as_ptr(),
            if strlen((*template).name) != 0 {
                (*template).name
            } else {
                (*template).fmt
            },
            result,
        );
    }

    result
}

unsafe fn restore_template_fmt(template_name: *mut c_char) -> *mut ima_template_desc {
    let mut template_desc: *mut ima_template_desc = ptr::null_mut();
    let ret: c_int;

    ret = template_desc_init_fields(template_name, ptr::null_mut(), ptr::null_mut());
    if ret < 0 {
        pr_err(
            c"attempting to initialize the template \"%s\" failed\n".as_ptr(),
            template_name,
        );
        return template_desc;
    }

    template_desc = kzalloc_obj::<ima_template_desc>();
    if template_desc.is_null() {
        return template_desc;
    }

    (*template_desc).name = c"".as_ptr();
    (*template_desc).fmt = kstrdup(template_name, GFP_KERNEL);
    if (*template_desc).fmt.is_null() {
        kfree(template_desc as *mut c_void);
        template_desc = ptr::null_mut();
        return template_desc;
    }

    spin_lock(&raw mut template_list);
    list_add_tail_rcu(&mut (*template_desc).list, &raw mut defined_templates);
    spin_unlock(&raw mut template_list);

    template_desc
}

unsafe fn ima_restore_template_data(
    template_desc: *mut ima_template_desc,
    template_data: *mut c_void,
    template_data_size: c_int,
    entry: *mut *mut ima_template_entry,
) -> c_int {
    let mut digests: *mut tpm_digest;
    let mut ret: c_int = 0;
    let mut i: c_int;

    *entry = kzalloc_flex_entry((*template_desc).num_fields, GFP_NOFS);
    if (*entry).is_null() {
        return -ENOMEM;
    }

    digests = kzalloc_objs::<tpm_digest>(
        (NR_BANKS(ima_tpm_chip) + ima_extra_slots) as usize,
        GFP_NOFS,
    );
    if digests.is_null() {
        kfree(*entry as *mut c_void);
        return -ENOMEM;
    }

    (**entry).digests = digests;

    ret = ima_parse_buf(
        template_data,
        (template_data as *mut u8).add(template_data_size as usize) as *mut c_void,
        ptr::null_mut(),
        (*template_desc).num_fields,
        (**entry).template_data.as_mut_ptr(),
        ptr::null_mut(),
        ptr::null_mut(),
        ENFORCE_FIELDS | ENFORCE_BUFEND,
        c"template data".as_ptr(),
    );
    if ret < 0 {
        kfree((**entry).digests as *mut c_void);
        kfree(*entry as *mut c_void);
        return ret;
    }

    (**entry).template_desc = template_desc;
    i = 0;
    while i < (*template_desc).num_fields {
        let field_data: *mut ima_field_data = (**entry).template_data.as_mut_ptr().add(i as usize);
        let data: *mut u8 = (*field_data).data;

        (**entry).template_data[i as usize].data =
            kzalloc((*field_data).len as usize + 1, GFP_KERNEL) as *mut u8;
        if (**entry).template_data[i as usize].data.is_null() {
            ret = -ENOMEM;
            break;
        }
        memcpy(
            (**entry).template_data[i as usize].data as *mut c_void,
            data as *const c_void,
            (*field_data).len as usize,
        );
        (**entry).template_data_len += size_of::<u32>() as c_int;
        (**entry).template_data_len += (*field_data).len as c_int;
        i += 1;
    }

    if ret < 0 {
        ima_free_template_entry(*entry);
        *entry = ptr::null_mut();
    }

    ret
}

/* Restore the serialized binary measurement list without extending PCRs. */
#[no_mangle]
pub unsafe extern "C" fn ima_restore_measurement_list(size: loff_t, buf: *mut c_void) -> c_int {
    let mut template_name: [c_char; MAX_TEMPLATE_NAME_LEN] = [0; MAX_TEMPLATE_NAME_LEN];
    let zero: [u8; TPM_DIGEST_SIZE] = [0; TPM_DIGEST_SIZE];

    let khdr: *mut ima_kexec_hdr = buf as *mut ima_kexec_hdr;
    let mut hdr: [ima_field_data; HDR__LAST] = [
        ima_field_data { len: size_of::<u32>() as u32, data: ptr::null_mut() },
        ima_field_data { len: TPM_DIGEST_SIZE as u32, data: ptr::null_mut() },
        ima_field_data { len: 0, data: ptr::null_mut() },
        ima_field_data { len: 0, data: ptr::null_mut() },
    ];

    let mut bufp: *mut c_void = (buf as *mut u8).add(size_of::<ima_kexec_hdr>()) as *mut c_void;
    let bufendp: *mut c_void;
    let mut entry: *mut ima_template_entry = ptr::null_mut();
    let mut template_desc: *mut ima_template_desc;
    let mut hdr_mask: [c_ulong; 1] = [0; 1];
    let mut count: c_ulong = 0;
    let mut ret: c_int = 0;

    if buf.is_null() || size < size_of::<ima_kexec_hdr>() as loff_t {
        return 0;
    }

    if ima_canonical_fmt {
        (*khdr).version = le16_to_cpu((*khdr).version);
        (*khdr).count = le64_to_cpu((*khdr).count);
        (*khdr).buffer_size = le64_to_cpu((*khdr).buffer_size);
    }

    if (*khdr).version != 1 {
        pr_err(c"attempting to restore a incompatible measurement list".as_ptr());
        return -EINVAL;
    }

    if (*khdr).count > (ULONG_MAX - 1) as u64 {
        pr_err(c"attempting to restore too many measurements".as_ptr());
        return -EINVAL;
    }

    bitmap_zero(hdr_mask.as_mut_ptr(), HDR__LAST as c_int);
    bitmap_set(hdr_mask.as_mut_ptr(), HDR_PCR as c_int, 1);
    bitmap_set(hdr_mask.as_mut_ptr(), HDR_DIGEST as c_int, 1);

    /*
     * ima kexec buffer prefix: version, buffer size, count
     * v1 format: pcr, digest, template-name-len, template-name,
     *	      template-data-size, template-data
     */
    bufendp = (buf as *mut u8).add((*khdr).buffer_size as usize) as *mut c_void;
    while (bufp as usize) < (bufendp as usize) && {
        count = count.wrapping_add(1);
        count < (*khdr).count as c_ulong
    } {
        let mut enforce_mask: c_int = ENFORCE_FIELDS;

        enforce_mask |= if count == (*khdr).count as c_ulong {
            ENFORCE_BUFEND
        } else {
            0
        };
        ret = ima_parse_buf(
            bufp,
            bufendp,
            &mut bufp,
            HDR__LAST as c_int,
            hdr.as_mut_ptr(),
            ptr::null_mut(),
            hdr_mask.as_mut_ptr(),
            enforce_mask,
            c"entry header".as_ptr(),
        );
        if ret < 0 {
            break;
        }

        if hdr[HDR_TEMPLATE_NAME].len >= MAX_TEMPLATE_NAME_LEN as u32 {
            pr_err(c"attempting to restore a template name that is too long\n".as_ptr());
            ret = -EINVAL;
            break;
        }

        /* template name is not null terminated */
        memcpy(
            template_name.as_mut_ptr() as *mut c_void,
            hdr[HDR_TEMPLATE_NAME].data as *const c_void,
            hdr[HDR_TEMPLATE_NAME].len as usize,
        );
        template_name[hdr[HDR_TEMPLATE_NAME].len as usize] = 0;

        if strcmp(template_name.as_ptr(), c"ima".as_ptr()) == 0 {
            pr_err(
                c"attempting to restore an unsupported template \"%s\" failed\n".as_ptr(),
                template_name.as_ptr(),
            );
            ret = -EINVAL;
            break;
        }

        template_desc = lookup_template_desc(template_name.as_ptr());
        if template_desc.is_null() {
            template_desc = restore_template_fmt(template_name.as_mut_ptr());
            if template_desc.is_null() {
                break;
            }
        }

        /*
         * Only the running system's template format is initialized
         * on boot.  As needed, initialize the other template formats.
         */
        ret = template_desc_init_fields(
            (*template_desc).fmt,
            &mut (*template_desc).fields,
            &mut (*template_desc).num_fields,
        );
        if ret < 0 {
            pr_err(
                c"attempting to restore the template fmt \"%s\" failed\n".as_ptr(),
                (*template_desc).fmt,
            );
            ret = -EINVAL;
            break;
        }

        ret = ima_restore_template_data(
            template_desc,
            hdr[HDR_TEMPLATE_DATA].data as *mut c_void,
            hdr[HDR_TEMPLATE_DATA].len as c_int,
            &mut entry,
        );
        if ret < 0 {
            break;
        }

        if memcmp(
            hdr[HDR_DIGEST].data as *const c_void,
            zero.as_ptr() as *const c_void,
            size_of_val(&zero),
        ) != 0
        {
            ret = ima_calc_field_array_hash((**(&mut entry)).template_data.as_mut_ptr(), entry);
            if ret < 0 {
                pr_err(c"cannot calculate template digest\n".as_ptr());
                ret = -EINVAL;
                break;
            }
        }

        (*entry).pcr = if !ima_canonical_fmt {
            *(hdr[HDR_PCR].data as *mut u32)
        } else {
            le32_to_cpu(*(hdr[HDR_PCR].data as *mut u32))
        };
        ret = ima_restore_measurement_entry(entry);
        if ret < 0 {
            break;
        }
    }
    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
