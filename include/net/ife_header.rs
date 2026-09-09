/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are referenced
// here but are not defined by this header.

#[cfg(feature = "CONFIG_NET_IFE")]
extern "C" {
    pub fn ife_encode(skb: *mut sk_buff, metalen: u16) -> *mut core::ffi::c_void;
    pub fn ife_decode(skb: *mut sk_buff, metalen: *mut u16) -> *mut core::ffi::c_void;

    pub fn ife_tlv_meta_decode(
        skbdata: *mut core::ffi::c_void,
        ifehdr_end: *const core::ffi::c_void,
        attrtype: *mut u16,
        dlen: *mut u16,
        totlen: *mut u16,
    ) -> *mut core::ffi::c_void;
    pub fn ife_tlv_meta_encode(
        skbdata: *mut core::ffi::c_void,
        attrtype: u16,
        dlen: u16,
        dval: *const core::ffi::c_void,
    ) -> i32;

    pub fn ife_tlv_meta_next(skbdata: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
}

#[cfg(not(feature = "CONFIG_NET_IFE"))]
#[inline]
pub unsafe fn ife_encode(
    _skb: *mut sk_buff,
    _metalen: u16,
) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_NET_IFE"))]
#[inline]
pub unsafe fn ife_decode(
    _skb: *mut sk_buff,
    _metalen: *mut u16,
) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

// The disabled configuration's declaration intentionally follows the source
// header, which omits the ifehdr_end parameter.
#[cfg(not(feature = "CONFIG_NET_IFE"))]
#[inline]
pub unsafe fn ife_tlv_meta_decode(
    _skbdata: *mut core::ffi::c_void,
    _attrtype: *mut u16,
    _dlen: *mut u16,
    _totlen: *mut u16,
) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_NET_IFE"))]
#[inline]
pub unsafe fn ife_tlv_meta_encode(
    _skbdata: *mut core::ffi::c_void,
    _attrtype: u16,
    _dlen: u16,
    _dval: *const core::ffi::c_void,
) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_NET_IFE"))]
#[inline]
pub unsafe fn ife_tlv_meta_next(
    _skbdata: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
