// SPDX-License-Identifier: GPL-2.0-or-later
/* Structure dynamic extension infrastructure
 * Copyright (C) 2004 Rusty Russell IBM Corporation
 * Copyright (C) 2007 Netfilter Core Team <coreteam@netfilter.org>
 * Copyright (C) 2007 USAGI/WIDE Project <http://www.linux-ipv6.org>
 */

// Kernel headers and configuration-provided declarations are supplied by the
// surrounding translation unit.

const NF_CT_EXT_PREALLOC: u32 = 128; /* conntrack events are on by default */

// The following types, constants, and functions are external kernel symbols.
// Their definitions are supplied by the corresponding translated headers.
extern "C" {
    static nf_ct_ext_type_len: [u8; NF_CT_EXT_NUM];
}

// Configuration-dependent entries retain the conditions of the C source.
#[allow(dead_code)]
static NF_CT_EXT_TYPE_LEN: [u8; NF_CT_EXT_NUM] = [
    /* [NF_CT_EXT_HELPER] = size_of::<nf_conn_help>() */
];

#[inline(always)]
unsafe fn total_extension_size() -> usize {
    /* remember to add new extensions below */
    // BUILD_BUG_ON(NF_CT_EXT_NUM > 10);

    core::mem::size_of::<nf_ct_ext>()
        + core::mem::size_of::<nf_conn_help>()
        // + core::mem::size_of::<nf_conn_nat>() when CONFIG_NF_NAT is enabled
        + core::mem::size_of::<nf_conn_seqadj>()
        + core::mem::size_of::<nf_conn_acct>()
        // + core::mem::size_of::<nf_conntrack_ecache>() when CONFIG_NF_CONNTRACK_EVENTS is enabled
        // + core::mem::size_of::<nf_conn_tstamp>() when CONFIG_NF_CONNTRACK_TIMESTAMP is enabled
        // + core::mem::size_of::<nf_conn_timeout>() when CONFIG_NF_CONNTRACK_TIMEOUT is enabled
        // + core::mem::size_of::<nf_conn_labels>() when CONFIG_NF_CONNTRACK_LABELS is enabled
        // + core::mem::size_of::<nf_conn_synproxy>() when CONFIG_NETFILTER_SYNPROXY is enabled
        // + core::mem::size_of::<nf_conn_act_ct_ext>() when CONFIG_NET_ACT_CT is enabled
}

pub unsafe fn nf_ct_ext_add(
    ct: *mut nf_conn,
    id: nf_ct_ext_id,
    gfp: gfp_t,
) -> *mut core::ffi::c_void {
    let mut newlen: usize;
    let mut newoff: usize;
    let oldlen: usize;
    let alloc: usize;
    let new: *mut nf_ct_ext;

    /* Conntrack must not be confirmed to avoid races on reallocation. */
    // WARN_ON(nf_ct_is_confirmed(ct));

    /* struct nf_ct_ext uses u8 to store offsets/size */
    // BUILD_BUG_ON(total_extension_size() > 255u);

    if !(*ct).ext.is_null() {
        let old: *const nf_ct_ext = (*ct).ext;

        if __nf_ct_ext_exist(old, id) {
            return core::ptr::null_mut();
        }
        oldlen = (*old).len as usize;
    } else {
        oldlen = core::mem::size_of::<*mut nf_ct_ext>();
    }

    newoff = (oldlen + core::mem::align_of::<nf_ct_ext>() - 1)
        & !(core::mem::align_of::<nf_ct_ext>() - 1);
    newlen = newoff + nf_ct_ext_type_len[id as usize] as usize;

    alloc = core::cmp::max(newlen, NF_CT_EXT_PREALLOC as usize);
    new = krealloc((*ct).ext, alloc, gfp);
    if new.is_null() {
        return core::ptr::null_mut();
    }

    if (*ct).ext.is_null() {
        core::ptr::write_bytes((*new).offset.as_mut_ptr(), 0, (*new).offset.len());
    }

    (*new).offset[id as usize] = newoff as u8;
    (*new).len = newlen as u8;
    core::ptr::write_bytes(
        (new as *mut u8).add(newoff),
        0,
        newlen - newoff,
    );

    (*ct).ext = new;
    (new as *mut u8).add(newoff) as *mut core::ffi::c_void
}

// EXPORT_SYMBOL(nf_ct_ext_add);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
