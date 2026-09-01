// Translated from testing/selftests/bpf/bpf_kfuncs.h.
// C header guards and ksym/weak annotations are C/BPF build metadata.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct bpf_sock_addr_kern {
    _private: [u8; 0],
}

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_dynptr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xdp_md {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_tcp_req_attrs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_key {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

unsafe extern "C" {
    /* Description
     *  Initializes an skb-type dynptr
     * Returns
     *  Error code
     */
    pub fn bpf_dynptr_from_skb(
        skb: *mut __sk_buff,
        flags: __u64,
        ptr__uninit: *mut bpf_dynptr,
    ) -> c_int;

    /* Description
     *  Initializes an xdp-type dynptr
     * Returns
     *  Error code
     */
    pub fn bpf_dynptr_from_xdp(
        xdp: *mut xdp_md,
        flags: __u64,
        ptr__uninit: *mut bpf_dynptr,
    ) -> c_int;

    pub fn bpf_dynptr_from_skb_meta(
        skb: *mut __sk_buff,
        flags: __u64,
        ptr__uninit: *mut bpf_dynptr,
    ) -> c_int;

    /* Description
     *  Obtain a read-only pointer to the dynptr's data
     * Returns
     *  Either a direct pointer to the dynptr data or a pointer to the user-provided
     *  buffer if unable to obtain a direct pointer
     */
    pub fn bpf_dynptr_slice(
        ptr: *const bpf_dynptr,
        offset: __u64,
        buffer: *mut c_void,
        buffer__szk: __u64,
    ) -> *mut c_void;

    /* Description
     *  Obtain a read-write pointer to the dynptr's data
     * Returns
     *  Either a direct pointer to the dynptr data or a pointer to the user-provided
     *  buffer if unable to obtain a direct pointer
     */
    pub fn bpf_dynptr_slice_rdwr(
        ptr: *const bpf_dynptr,
        offset: __u64,
        buffer: *mut c_void,
        buffer__szk: __u64,
    ) -> *mut c_void;

    pub fn bpf_dynptr_adjust(ptr: *mut bpf_dynptr, start: __u64, end: __u64) -> c_int;
    pub fn bpf_dynptr_is_null(ptr: *const bpf_dynptr) -> bool;
    pub fn bpf_dynptr_is_rdonly(ptr: *const bpf_dynptr) -> bool;
    pub fn bpf_dynptr_size(ptr: *const bpf_dynptr) -> __u64;
    pub fn bpf_dynptr_clone(ptr: *const bpf_dynptr, clone__init: *mut bpf_dynptr) -> c_int;

    /* Description
     *  Modify the address of a AF_UNIX sockaddr.
     * Returns
     *  -EINVAL if the address size is too big or, 0 if the sockaddr was successfully modified.
     */
    pub fn bpf_sock_addr_set_sun_path(
        sa_kern: *mut bpf_sock_addr_kern,
        sun_path: *const __u8,
        sun_path__sz: __u32,
    ) -> c_int;

    /* Description
     *  Allocate and configure a reqsk and link it with a listener and skb.
     * Returns
     *  Error code
     */
    pub fn bpf_sk_assign_tcp_reqsk(
        skb: *mut __sk_buff,
        sk: *mut sock,
        attrs: *mut bpf_tcp_req_attrs,
        attrs__sz: c_int,
    ) -> c_int;

    pub fn bpf_cast_to_kern_ctx(arg1: *mut c_void) -> *mut c_void;

    pub fn bpf_rdonly_cast(obj: *const c_void, btf_id: __u32) -> *mut c_void;

    pub fn bpf_get_file_xattr(
        file: *mut file,
        name: *const c_char,
        value_ptr: *mut bpf_dynptr,
    ) -> c_int;
    pub fn bpf_get_fsverity_digest(
        file: *mut file,
        digest_ptr: *const bpf_dynptr,
    ) -> c_int;

    pub fn bpf_lookup_user_key(serial: __s32, flags: __u64) -> *mut bpf_key;
    pub fn bpf_lookup_system_key(id: __u64) -> *mut bpf_key;
    pub fn bpf_key_put(key: *mut bpf_key);
    pub fn bpf_verify_pkcs7_signature(
        data_ptr: *const bpf_dynptr,
        sig_ptr: *const bpf_dynptr,
        trusted_keyring: *mut bpf_key,
    ) -> c_int;

    /* Description
     *  Returns xattr of a dentry
     * Returns
     *  Error code
     */
    pub fn bpf_get_dentry_xattr(
        dentry: *mut dentry,
        name: *const c_char,
        value_ptr: *mut bpf_dynptr,
    ) -> c_int;

    pub fn bpf_set_dentry_xattr(
        dentry: *mut dentry,
        name__str: *const c_char,
        value_p: *const bpf_dynptr,
        flags: c_int,
    ) -> c_int;
    pub fn bpf_remove_dentry_xattr(dentry: *mut dentry, name__str: *const c_char) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
