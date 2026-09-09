/* SPDX-License-Identifier: GPL-2.0 */

// Translated from nf_conntrack_labels.h.
// Dependencies supplied by the included kernel headers are intentionally left external.

pub const NF_CT_LABELS_MAX_SIZE: usize =
    (XT_CONNLABEL_MAXBIT + 1) / BITS_PER_BYTE;

#[repr(C)]
pub struct nf_conn_labels {
    pub bits: [core::ffi::c_ulong; NF_CT_LABELS_MAX_SIZE / core::mem::size_of::<core::ffi::c_ulong>()],
}

/* Can't use nf_ct_ext_find(), flow dissector cannot use symbols
 * exported by nf_conntrack module.
 */
#[inline]
pub unsafe fn nf_ct_labels_find(
    ct: *const crate::nf_conn,
) -> *mut crate::nf_conn_labels {
    #[cfg(feature = "CONFIG_NF_CONNTRACK_LABELS")]
    {
        let ext = (*ct).ext;

        if ext.is_null() || !crate::__nf_ct_ext_exist(ext, crate::NF_CT_EXT_LABELS) {
            return core::ptr::null_mut();
        }

        return (ext as *mut u8)
            .add((*ct).ext.offset[crate::NF_CT_EXT_LABELS as usize] as usize)
            as *mut crate::nf_conn_labels;
    }

    #[cfg(not(feature = "CONFIG_NF_CONNTRACK_LABELS"))]
    {
        core::ptr::null_mut()
    }
}

#[inline]
pub unsafe fn nf_ct_labels_ext_add(
    ct: *mut crate::nf_conn,
) -> *mut crate::nf_ct_ext {
    #[cfg(feature = "CONFIG_NF_CONNTRACK_LABELS")]
    {
        let net = crate::nf_ct_net(ct);

        if crate::atomic_read(&(*net).ct.labels_used) == 0 {
            return core::ptr::null_mut();
        }

        return crate::nf_ct_ext_add(ct, crate::NF_CT_EXT_LABELS, crate::GFP_ATOMIC);
    }

    #[cfg(not(feature = "CONFIG_NF_CONNTRACK_LABELS"))]
    {
        core::ptr::null_mut()
    }
}

extern "C" {
    pub fn nf_connlabels_replace(
        ct: *mut crate::nf_conn,
        data: *const u32,
        mask: *const u32,
        words: core::ffi::c_uint,
    ) -> core::ffi::c_int;
}

#[cfg(feature = "CONFIG_NF_CONNTRACK_LABELS")]
extern "C" {
    pub fn nf_connlabels_get(net: *mut crate::net, bit: core::ffi::c_uint)
        -> core::ffi::c_int;
    pub fn nf_connlabels_put(net: *mut crate::net);
}

#[cfg(not(feature = "CONFIG_NF_CONNTRACK_LABELS"))]
#[inline]
pub unsafe fn nf_connlabels_get(
    _net: *mut crate::net,
    _bit: core::ffi::c_uint,
) -> core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_NF_CONNTRACK_LABELS"))]
#[inline]
pub unsafe fn nf_connlabels_put(_net: *mut crate::net) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
