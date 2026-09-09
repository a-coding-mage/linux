/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: linux/slab.h and net/netfilter/nf_conntrack.h.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum nf_ct_ext_id {
    NF_CT_EXT_HELPER,
    // Preserved build-time condition: IS_ENABLED(CONFIG_NF_NAT).
    NF_CT_EXT_NAT,
    NF_CT_EXT_SEQADJ,
    NF_CT_EXT_ACCT,
    // Preserved build-time condition: CONFIG_NF_CONNTRACK_EVENTS.
    NF_CT_EXT_ECACHE,
    // Preserved build-time condition: CONFIG_NF_CONNTRACK_TIMESTAMP.
    NF_CT_EXT_TSTAMP,
    // Preserved build-time condition: CONFIG_NF_CONNTRACK_TIMEOUT.
    NF_CT_EXT_TIMEOUT,
    // Preserved build-time condition: CONFIG_NF_CONNTRACK_LABELS.
    NF_CT_EXT_LABELS,
    // Preserved build-time condition: IS_ENABLED(CONFIG_NETFILTER_SYNPROXY).
    NF_CT_EXT_SYNPROXY,
    // Preserved build-time condition: IS_ENABLED(CONFIG_NET_ACT_CT).
    NF_CT_EXT_ACT_CT,
    NF_CT_EXT_NUM,
}

/* Extensions: optional stuff which isn't permanently in struct. */
#[repr(C, align(8))]
pub struct nf_ct_ext {
    pub offset: [u8; nf_ct_ext_id::NF_CT_EXT_NUM as usize],
    pub len: u8,
    pub data: [u8; 0],
}

#[inline]
pub unsafe fn __nf_ct_ext_exist(ext: *const nf_ct_ext, id: u8) -> bool {
    (*ext).offset[id as usize] != 0
}

#[inline]
pub unsafe fn nf_ct_ext_exist(
    ct: *const nf_conn,
    id: u8,
) -> bool {
    !(*ct).ext.is_null() && __nf_ct_ext_exist((*ct).ext, id)
}

#[inline]
pub unsafe fn nf_ct_ext_find(
    ct: *const nf_conn,
    id: u8,
) -> *mut core::ffi::c_void {
    let ext = (*ct).ext;

    if ext.is_null() || !__nf_ct_ext_exist(ext, id) {
        return core::ptr::null_mut();
    }

    (ext as *mut u8).add((*ct).ext.as_ref().unwrap().offset[id as usize] as usize)
        as *mut core::ffi::c_void
}

/* Add this type, returns pointer to data or NULL. */
extern "C" {
    pub fn nf_ct_ext_add(
        ct: *mut nf_conn,
        id: nf_ct_ext_id,
        gfp: gfp_t,
    ) -> *mut core::ffi::c_void;
}

// `gfp_t` and `nf_conn` are supplied by the included kernel headers.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
