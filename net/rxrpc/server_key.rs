// SPDX-License-Identifier: GPL-2.0-or-later
/* RxRPC key management
 *
 * Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 *
 * RxRPC keys should have a description of describing their purpose:
 *	"afs@CAMBRIDGE.REDHAT.COM>
 */

// pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// Kernel includes and symbols supplied by other files are external dependencies.

unsafe extern "C" {
    fn rxrpc_security_lookup(sec_class: ::core::ffi::c_uint) -> *const rxrpc_security;
    fn generic_key_instantiate(
        prep: *mut key_preparsed_payload,
        key: *mut key,
    ) -> ::core::ffi::c_int;
    fn memdup_sockptr_nul(optval: sockptr_t, optlen: usize) -> *mut ::core::ffi::c_char;
    fn request_key(
        key_type: *const key_type,
        description: *const ::core::ffi::c_char,
        callout_info: *const ::core::ffi::c_char,
    ) -> *mut key;
    fn kfree(ptr: *mut ::core::ffi::c_void);
    fn key_get(key: *mut key) -> *mut key;
    fn lock_sock(sk: *mut sock);
    fn release_sock(sk: *mut sock);
    fn test_bit(nr: ::core::ffi::c_ulong, addr: *const ::core::ffi::c_ulong) -> bool;
    fn set_bit(nr: ::core::ffi::c_ulong, addr: *mut ::core::ffi::c_ulong);
    fn clear_bit(nr: ::core::ffi::c_ulong, addr: *mut ::core::ffi::c_ulong);
    fn rxrpc_sk(sk: *mut sock) -> *mut rxrpc_sock;
}

#[repr(C)]
pub struct key_type {
    pub name: *const ::core::ffi::c_char,
    pub flags: ::core::ffi::c_uint,
    pub vet_description: Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int>,
    pub preparse: Option<unsafe extern "C" fn(*mut key_preparsed_payload) -> ::core::ffi::c_int>,
    pub free_preparse: Option<unsafe extern "C" fn(*mut key_preparsed_payload)>,
    pub instantiate: Option<unsafe extern "C" fn(*mut key_preparsed_payload, *mut key) -> ::core::ffi::c_int>,
    pub destroy: Option<unsafe extern "C" fn(*mut key)>,
    pub describe: Option<unsafe extern "C" fn(*const key, *mut seq_file)>,
}

#[repr(C)]
pub struct key_preparsed_payload {
    pub datalen: usize,
    pub orig_description: *const ::core::ffi::c_char,
    pub payload: [*mut ::core::ffi::c_void; 4],
}

#[repr(C)]
pub struct key { pub payload: [*mut ::core::ffi::c_void; 4], pub description: *const ::core::ffi::c_char, pub serial: ::core::ffi::c_ulong }
#[repr(C)] pub struct rxrpc_security {
    pub preparse_server_key: Option<unsafe extern "C" fn(*mut key_preparsed_payload) -> ::core::ffi::c_int>,
    pub free_preparse_server_key: Option<unsafe extern "C" fn(*mut key_preparsed_payload)>,
    pub destroy_server_key: Option<unsafe extern "C" fn(*mut key)>,
    pub describe_server_key: Option<unsafe extern "C" fn(*const key, *mut seq_file)>,
}
#[repr(C)] pub struct rxrpc_sock { pub securities: *mut key, pub sk: sock, pub flags: ::core::ffi::c_ulong }
#[repr(C)] pub struct sock { pub sk_state: ::core::ffi::c_int }
#[repr(C)] pub struct seq_file;
pub type sockptr_t = *mut ::core::ffi::c_void;

extern "C" { static key_type_keyring: key_type; }

static mut rxrpc_vet_description_s: Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int> = Some(rxrpc_vet_description_s);

// rxrpc server keys take "<serviceId>:<securityIndex>[:<sec-specific>]" as the
// description and the key material as the payload.
#[no_mangle]
pub static mut key_type_rxrpc_s: key_type = key_type {
    name: b"rxrpc_s\0".as_ptr() as *const _,
    flags: KEY_TYPE_NET_DOMAIN,
    vet_description: Some(rxrpc_vet_description_s),
    preparse: Some(rxrpc_preparse_s),
    free_preparse: Some(rxrpc_free_preparse_s),
    instantiate: Some(generic_key_instantiate),
    destroy: Some(rxrpc_destroy_s),
    describe: Some(rxrpc_describe_s),
};

const KEY_TYPE_NET_DOMAIN: ::core::ffi::c_uint = 1;
const EINVAL: ::core::ffi::c_int = 22;
const ENOPKG: ::core::ffi::c_int = 65;
const EISCONN: ::core::ffi::c_int = 106;
const PAGE_SIZE: usize = 4096;
const RXRPC_UNBOUND: ::core::ffi::c_int = 0;
const RXRPC_SOCK_MANAGE_RESPONSE: ::core::ffi::c_ulong = 0;

// Vet the description for an RxRPC server key.
unsafe extern "C" fn rxrpc_vet_description_s(desc: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut end: *mut ::core::ffi::c_char = core::ptr::null_mut();
    let service = libc_strtoul(desc, &mut end, 10);
    if *end != b':' as _ || service > 65535 { return -EINVAL; }
    let sec_class = libc_strtoul(end.add(1), &mut end, 10);
    if (*end != 0 && *end != b':' as _) || sec_class < 1 || sec_class > 255 { return -EINVAL; }
    0
}

unsafe extern "C" fn libc_strtoul(s: *const ::core::ffi::c_char, end: *mut *mut ::core::ffi::c_char, base: ::core::ffi::c_int) -> ::core::ffi::c_ulong;

// Preparse a server secret key.
unsafe extern "C" fn rxrpc_preparse_s(prep: *mut key_preparsed_payload) -> ::core::ffi::c_int {
    if (*prep).orig_description.is_null() { return -EINVAL; }
    let sec_class = 0u32;
    let sec = rxrpc_security_lookup(sec_class);
    if sec.is_null() { return -ENOPKG; }
    (*prep).payload[1] = sec as *mut _;
    match (*sec).preparse_server_key { Some(f) => f(prep), None => -EINVAL }
}

unsafe extern "C" fn rxrpc_free_preparse_s(prep: *mut key_preparsed_payload) {
    let sec = (*prep).payload[1] as *const rxrpc_security;
    if !sec.is_null() { if let Some(f) = (*sec).free_preparse_server_key { f(prep); } }
}

unsafe extern "C" fn rxrpc_destroy_s(key: *mut key) {
    let sec = (*key).payload[1] as *const rxrpc_security;
    if !sec.is_null() { if let Some(f) = (*sec).destroy_server_key { f(key); } }
}

unsafe extern "C" fn rxrpc_describe_s(key: *const key, _m: *mut seq_file) {
    let sec = (*key).payload[1] as *const rxrpc_security;
    if !sec.is_null() { if let Some(f) = (*sec).describe_server_key { f(key, _m); } }
}

pub unsafe extern "C" fn rxrpc_server_keyring(rx: *mut rxrpc_sock, optval: sockptr_t, optlen: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if !(*rx).securities.is_null() || optlen <= 0 || optlen as usize > PAGE_SIZE - 1 { return -EINVAL; }
    let description = memdup_sockptr_nul(optval, optlen as usize);
    if description.is_null() { return -EINVAL; }
    let key = request_key(&key_type_keyring, description, core::ptr::null());
    if key.is_null() { kfree(description as *mut _); return -EINVAL; }
    (*rx).securities = key; kfree(description as *mut _); 0
}

pub unsafe extern "C" fn rxrpc_sock_set_security_keyring(sk: *mut sock, keyring: *mut key) -> ::core::ffi::c_int {
    let rx = rxrpc_sk(sk); let mut ret = 0;
    lock_sock(sk);
    if !(*rx).securities.is_null() { ret = -EINVAL; }
    else if (*rx).sk.sk_state != RXRPC_UNBOUND { ret = -EISCONN; }
    else { (*rx).securities = key_get(keyring); }
    release_sock(sk); ret
}

pub unsafe extern "C" fn rxrpc_sock_set_manage_response(sk: *mut sock, set: bool) -> ::core::ffi::c_int {
    let rx = rxrpc_sk(sk); lock_sock(sk);
    let ret = test_bit(RXRPC_SOCK_MANAGE_RESPONSE, &(*rx).flags);
    if set { set_bit(RXRPC_SOCK_MANAGE_RESPONSE, &mut (*rx).flags); }
    else { clear_bit(RXRPC_SOCK_MANAGE_RESPONSE, &mut (*rx).flags); }
    release_sock(sk); ret as _
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
