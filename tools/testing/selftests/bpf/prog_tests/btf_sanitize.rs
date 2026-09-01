// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026, Oracle and/or its affiliates. */
// Translated from C implementation source. External test/libbpf/kernel symbols
// are expected to be supplied by the surrounding repository bindings.

use core::ffi::{c_char, c_void};
use core::mem::size_of;
use core::ptr;

type __u32 = u32;

const TYPE_LEN: usize = size_of::<btf_type>() + size_of::<__u32>();
const MAX_NR_LAYOUT: usize = 2;
const LAYOUT_LEN: usize = size_of::<btf_layout>() * MAX_NR_LAYOUT;
const STR_LEN: usize = size_of::<[u8; 5]>();

#[repr(C)]
pub struct btf_header {
    pub magic: __u32,
    pub version: __u32,
    pub flags: __u32,
    pub hdr_len: __u32,
    pub type_off: __u32,
    pub type_len: __u32,
    pub str_off: __u32,
    pub str_len: __u32,
    pub layout_off: __u32,
    pub layout_len: __u32,
}

#[repr(C)]
pub struct btf_type {
    pub name_off: __u32,
    pub info: __u32,
    pub size: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_layout {
    pub info_sz: __u32,
    pub elem_sz: __u32,
    pub flags: __u32,
}

#[repr(C)]
pub struct layout_btf {
    pub hdr: btf_header,
    pub types: [__u32; TYPE_LEN / size_of::<__u32>()],
    pub layout: [btf_layout; MAX_NR_LAYOUT],
    pub strs: [c_char; STR_LEN],
}

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kfree_skb {
    pub obj: *mut bpf_object,
}

#[repr(C)]
pub struct kern_feature_cache {
    pub res: [feature_state; __FEAT_CNT as usize],
}

type feature_state = i32;
type kernel_feature_id = i32;

extern "C" {
    static BTF_MAGIC: __u32;
    static BTF_VERSION: __u32;
    static BTF_INT_SIGNED: __u32;
    static __FEAT_CNT: kernel_feature_id;
    static FEAT_SUPPORTED: feature_state;
    static FEAT_MISSING: feature_state;
    static FEAT_BTF_LAYOUT: kernel_feature_id;
    static FEAT_BTF_FUNC: kernel_feature_id;

    fn kfree_skb__open() -> *mut kfree_skb;
    fn kfree_skb__destroy(obj: *mut kfree_skb);

    fn btf__new(data: *const c_void, size: usize) -> *mut btf;
    fn btf__raw_data(btf: *const btf, size: *mut __u32) -> *const c_void;
    fn btf__free(btf: *mut btf);

    fn calloc(nmemb: usize, size: usize) -> *mut c_void;

    fn bpf_object_set_feat_cache(obj: *mut bpf_object, cache: *mut kern_feature_cache);
    fn kernel_supports(obj: *mut bpf_object, feat: kernel_feature_id) -> bool;
    fn bpf_object__sanitize_btf(obj: *mut bpf_object, btf: *const btf) -> *mut btf;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_FALSE(value: bool, name: *const c_char) -> bool;
    fn ASSERT_TRUE(value: bool, name: *const c_char) -> bool;
    fn ASSERT_EQ<T: Copy>(actual: T, expected: T, name: *const c_char) -> bool;
}

extern "Rust" {
    // C source used BTF_TYPE_INT_ENC(1, BTF_INT_SIGNED, 0, 32, 4), whose macro
    // expands into the u32 words for one BTF int type plus its encoding word.
    fn BTF_TYPE_INT_ENC(
        name: __u32,
        encoding: __u32,
        offset: __u32,
        bits: __u32,
        size: __u32,
    ) -> [__u32; TYPE_LEN / size_of::<__u32>()];
}

static mut layout_btf: layout_btf = unsafe {
    layout_btf {
        hdr: btf_header {
            magic: BTF_MAGIC,
            version: BTF_VERSION,
            flags: 0,
            hdr_len: size_of::<btf_header>() as __u32,
            type_off: 0,
            type_len: TYPE_LEN as __u32,
            str_off: (TYPE_LEN + LAYOUT_LEN) as __u32,
            str_len: STR_LEN as __u32,
            layout_off: TYPE_LEN as __u32,
            layout_len: LAYOUT_LEN as __u32,
        },
        types: BTF_TYPE_INT_ENC(1, BTF_INT_SIGNED, 0, 32, 4),
        layout: [
            btf_layout {
                info_sz: 0,
                elem_sz: 0,
                flags: 0,
            },
            btf_layout {
                info_sz: size_of::<__u32>() as __u32,
                elem_sz: 0,
                flags: 0,
            },
        ],
        strs: [0, b'i' as c_char, b'n' as c_char, b't' as c_char, 0],
    }
};

pub unsafe extern "C" fn test_btf_sanitize_layout() {
    let mut orig: *mut btf = ptr::null_mut();
    let mut sanitized: *mut btf = ptr::null_mut();
    let mut cache: *mut kern_feature_cache = ptr::null_mut();
    let mut skel: *mut kfree_skb = ptr::null_mut();
    let mut hdr: *const btf_header;
    let mut raw: *const c_void;
    let mut raw_sz: __u32 = 0;

    skel = kfree_skb__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"kfree_skb_skel".as_ptr()) {
        return;
    }
    orig = btf__new(
        ptr::addr_of!(layout_btf) as *const c_void,
        size_of::<layout_btf>(),
    );
    if !ASSERT_OK_PTR(orig as *const c_void, c"btf_new_layout".as_ptr()) {
        goto_out(skel, sanitized, orig);
        return;
    }
    raw = btf__raw_data(orig, &mut raw_sz);
    if !ASSERT_OK_PTR(raw, c"btf__raw_data_orig".as_ptr()) {
        goto_out(skel, sanitized, orig);
        return;
    }
    hdr = raw as *const btf_header;
    ASSERT_EQ((*hdr).layout_off, TYPE_LEN as __u32, c"layout_off_nonzero".as_ptr());
    ASSERT_EQ(
        (*hdr).layout_len,
        LAYOUT_LEN as __u32,
        c"layout_len_nonzero".as_ptr(),
    );

    cache = calloc(1, size_of::<kern_feature_cache>()) as *mut kern_feature_cache;
    if !ASSERT_OK_PTR(cache as *const c_void, c"alloc_feat_cache".as_ptr()) {
        goto_out(skel, sanitized, orig);
        return;
    }
    let mut i = 0;
    while i < __FEAT_CNT {
        (*cache).res[i as usize] = FEAT_SUPPORTED;
        i += 1;
    }
    (*cache).res[FEAT_BTF_LAYOUT as usize] = FEAT_MISSING;

    bpf_object_set_feat_cache((*skel).obj, cache);

    if !ASSERT_FALSE(
        kernel_supports((*skel).obj, FEAT_BTF_LAYOUT),
        c"layout_feature_missing".as_ptr(),
    ) {
        goto_out(skel, sanitized, orig);
        return;
    }
    if !ASSERT_TRUE(
        kernel_supports((*skel).obj, FEAT_BTF_FUNC),
        c"other_feature_allowed".as_ptr(),
    ) {
        goto_out(skel, sanitized, orig);
        return;
    }

    sanitized = bpf_object__sanitize_btf((*skel).obj, orig);
    if !ASSERT_OK_PTR(sanitized as *const c_void, c"bpf_object__sanitize_btf".as_ptr()) {
        goto_out(skel, sanitized, orig);
        return;
    }

    raw = btf__raw_data(sanitized, &mut raw_sz);
    if !ASSERT_OK_PTR(raw, c"btf__raw_data_sanitized".as_ptr()) {
        goto_out(skel, sanitized, orig);
        return;
    }
    hdr = raw as *const btf_header;
    ASSERT_EQ((*hdr).layout_off, 0, c"layout_off_zero".as_ptr());
    ASSERT_EQ((*hdr).layout_len, 0, c"layout_len_zero".as_ptr());
    ASSERT_EQ((*hdr).str_off, TYPE_LEN as __u32, c"strs_after_types".as_ptr());
    ASSERT_EQ((*hdr).str_len, STR_LEN as __u32, c"strs_len_unchanged".as_ptr());
    ASSERT_EQ(
        raw_sz,
        (*hdr).hdr_len + (*hdr).type_len + (*hdr).str_len,
        c"btf_raw_sz_reduced".as_ptr(),
    );

    goto_out(skel, sanitized, orig);
}

unsafe fn goto_out(skel: *mut kfree_skb, sanitized: *mut btf, orig: *mut btf) {
    /* This will free the cache we allocated above */
    kfree_skb__destroy(skel);
    btf__free(sanitized);
    btf__free(orig);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
