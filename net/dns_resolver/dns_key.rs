// SPDX-License-Identifier: LGPL-2.1-or-later
/* Key type used to cache DNS lookups made by the kernel
 *
 * See Documentation/networking/dns_resolver.rst
 */

// Kernel headers and build-time module metadata are supplied by other files.

pub static mut dns_resolver_debug: u32 = 0;
pub static mut dns_resolver_cache: *const cred = core::ptr::null();

const DNS_ERRORNO_OPTION: &[u8] = b"dnserror";

unsafe fn dns_resolver_preparse(prep: *mut key_preparsed_payload) -> i32 {
    let mut upayload: *mut user_key_payload;
    let mut derrno: libc_ulong = 0;
    let mut ret: i32;
    let mut datalen = (*prep).datalen;
    let mut result_len: isize = 0;
    let data = (*prep).data;
    let mut end: *const i8;
    let mut opt: *const i8;

    if datalen <= 1 || data.is_null() { return -EINVAL; }

    if *data == 0 {
        let v1 = data as *const dns_server_list_v1_header;
        if datalen < core::mem::size_of::<dns_server_list_v1_header>() { return -EINVAL; }
        if (*v1).hdr.content != DNS_PAYLOAD_IS_SERVER_LIST { return -EINVAL; }
        if (*v1).hdr.version != 1 { return -EINVAL; }
        if (*v1).status != DNS_LOOKUP_GOOD && (*v1).status != DNS_LOOKUP_GOOD_WITH_BAD {
            if (*prep).expiry == TIME64_MAX { (*prep).expiry = ktime_get_real_seconds() + 1; }
        }
        result_len = datalen as isize;
        goto store_result;
    }

    if *data.add(datalen - 1) != 0 { return -EINVAL; }
    datalen -= 1;
    end = data.add(datalen) as *const i8;
    opt = memchr(data as *const _, b'#' as i32, datalen);
    if opt.is_null() {
        result_len = datalen as isize;
    } else {
        let mut next_opt: *const i8;
        result_len = opt.offset_from(data as *const i8);
        opt = opt.add(1);
        loop {
            let opt_len: isize;
            let opt_nlen: isize;
            let eq: *const i8;
            let mut optval = [0i8; 128];
            next_opt = memchr(opt as *const _, b'#' as i32, end.offset_from(opt) as usize);
            if next_opt.is_null() { next_opt = end; }
            opt_len = next_opt.offset_from(opt);
            if opt_len <= 0 || opt_len > optval.len() as isize { return -EINVAL; }
            eq = memchr(opt as *const _, b'=' as i32, opt_len as usize);
            if !eq.is_null() {
                opt_nlen = eq.offset_from(opt);
                core::ptr::copy_nonoverlapping(eq.add(1), optval.as_mut_ptr(), next_opt.offset_from(eq.add(1)) as usize);
                optval[next_opt.offset_from(eq.add(1)) as usize] = 0;
            } else { opt_nlen = opt_len; optval[0] = 0; }
            if opt_nlen == (DNS_ERRORNO_OPTION.len() as isize) &&
                memcmp(opt as *const _, DNS_ERRORNO_OPTION.as_ptr() as *const _, opt_nlen as usize) == 0 {
                ret = kstrtoul(optval.as_ptr(), 10, &mut derrno);
                if ret < 0 || derrno < 1 || derrno > 511 { return -EINVAL; }
                (*prep).payload.data[dns_key_error] = ERR_PTR(-(derrno as isize));
                if next_opt >= end { break; }
                opt = next_opt.add(1);
                continue;
            }
            return -EINVAL;
        }
    }

    if !(*prep).payload.data[dns_key_error].is_null() { return 0; }

store_result:
    (*prep).quotalen = result_len as usize;
    upayload = kmalloc_flex(result_len as usize);
    if upayload.is_null() { return -ENOMEM; }
    (*upayload).datalen = result_len as usize;
    core::ptr::copy_nonoverlapping(data as *const u8, (*upayload).data.as_mut_ptr(), result_len as usize);
    (*prep).payload.data[dns_key_data] = upayload as *mut _;
    0
}

unsafe fn dns_resolver_free_preparse(prep: *mut key_preparsed_payload) {
    kfree((*prep).payload.data[dns_key_data]);
}

unsafe extern "C" fn dns_resolver_cmp(key: *const key, match_data: *const key_match_data) -> bool {
    let src = (*key).description;
    let dsp = (*match_data).raw_data;
    if src.is_null() || dsp.is_null() { return false; }
    if strcasecmp(src, dsp) == 0 { return true; }
    let mut slen = strlen(src) as isize;
    let mut dlen = strlen(dsp) as isize;
    if slen <= 0 || dlen <= 0 { return false; }
    if *src.add(slen as usize - 1) == b'.' as i8 { slen -= 1; }
    if *dsp.add(dlen as usize - 1) == b'.' as i8 { dlen -= 1; }
    slen == dlen && strncasecmp(src, dsp, slen as usize) == 0
}

unsafe fn dns_resolver_match_preparse(match_data: *mut key_match_data) -> i32 {
    (*match_data).lookup_type = KEYRING_SEARCH_LOOKUP_ITERATE;
    (*match_data).cmp = Some(dns_resolver_cmp);
    0
}

unsafe fn dns_resolver_describe(key: *const key, m: *mut seq_file) {
    seq_puts(m, (*key).description);
    if key_is_positive(key) {
        let err = PTR_ERR((*key).payload.data[dns_key_error]);
        if err != 0 { seq_printf(m, b": %d\0".as_ptr(), err); }
        else { seq_printf(m, b": %u\0".as_ptr(), (*key).datalen); }
    }
}

unsafe fn dns_resolver_read(key: *const key, buffer: *mut i8, buflen: usize) -> libc_long {
    let err = PTR_ERR((*key).payload.data[dns_key_error]);
    if err != 0 { return err as libc_long; }
    user_read(key, buffer, buflen)
}

#[no_mangle]
pub static mut key_type_dns_resolver: key_type = key_type {
    name: b"dns_resolver\0".as_ptr() as *const i8,
    flags: KEY_TYPE_NET_DOMAIN | KEY_TYPE_INSTANT_REAP,
    preparse: Some(dns_resolver_preparse),
    free_preparse: Some(dns_resolver_free_preparse),
    instantiate: Some(generic_key_instantiate),
    match_preparse: Some(dns_resolver_match_preparse),
    revoke: Some(user_revoke), destroy: Some(user_destroy),
    describe: Some(dns_resolver_describe), read: Some(dns_resolver_read),
};

unsafe fn init_dns_resolver() -> i32 {
    let cred = prepare_kernel_cred(&init_task);
    if cred.is_null() { return -ENOMEM; }
    let keyring = keyring_alloc(b".dns_resolver\0".as_ptr() as *const i8, GLOBAL_ROOT_UID, GLOBAL_ROOT_GID, cred,
        (KEY_POS_ALL & !KEY_POS_SETATTR) | KEY_USR_VIEW | KEY_USR_READ,
        KEY_ALLOC_NOT_IN_QUOTA, core::ptr::null_mut(), core::ptr::null_mut());
    if IS_ERR(keyring) { let ret = PTR_ERR(keyring); put_cred(cred); return ret; }
    let ret = register_key_type(&mut key_type_dns_resolver);
    if ret < 0 { key_put(keyring); put_cred(cred); return ret; }
    set_bit(KEY_FLAG_ROOT_CAN_CLEAR, &mut (*keyring).flags);
    (*cred).thread_keyring = keyring;
    (*cred).jit_keyring = KEY_REQKEY_DEFL_THREAD_KEYRING;
    dns_resolver_cache = cred;
    0
}

unsafe fn exit_dns_resolver() {
    key_revoke((*dns_resolver_cache).thread_keyring);
    unregister_key_type(&mut key_type_dns_resolver);
    put_cred(dns_resolver_cache);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
