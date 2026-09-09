/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of net/iw_handler.h. */

// Dependencies supplied by the surrounding kernel translation.

pub const IW_HANDLER_VERSION: u32 = 8;

pub const EIWCOMMIT: i32 = EINPROGRESS;
pub const IW_REQUEST_FLAG_COMPAT: u16 = 0x0001;

pub const IW_HEADER_TYPE_NULL: u8 = 0;
pub const IW_HEADER_TYPE_CHAR: u8 = 2;
pub const IW_HEADER_TYPE_UINT: u8 = 4;
pub const IW_HEADER_TYPE_FREQ: u8 = 5;
pub const IW_HEADER_TYPE_ADDR: u8 = 6;
pub const IW_HEADER_TYPE_POINT: u8 = 8;
pub const IW_HEADER_TYPE_PARAM: u8 = 9;
pub const IW_HEADER_TYPE_QUAL: u8 = 10;

pub const IW_DESCR_FLAG_NONE: u16 = 0x0000;
pub const IW_DESCR_FLAG_DUMP: u16 = 0x0001;
pub const IW_DESCR_FLAG_EVENT: u16 = 0x0002;
pub const IW_DESCR_FLAG_RESTRICT: u16 = 0x0004;
pub const IW_DESCR_FLAG_NOMAX: u16 = 0x0008;

#[repr(C)]
pub struct iw_request_info {
    pub cmd: u16,
    pub flags: u16,
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

pub type iw_handler = unsafe extern "C" fn(
    dev: *mut net_device,
    info: *mut iw_request_info,
    wrqu: *mut iwreq_data,
    extra: *mut i8,
) -> i32;

#[repr(C)]
pub struct iw_handler_def {
    pub standard: *const iw_handler,
    pub num_standard: u16,
    #[cfg(CONFIG_WEXT_PRIV)]
    pub num_private: u16,
    #[cfg(CONFIG_WEXT_PRIV)]
    pub num_private_args: u16,
    #[cfg(CONFIG_WEXT_PRIV)]
    pub private: *const iw_handler,
    #[cfg(CONFIG_WEXT_PRIV)]
    pub private_args: *const iw_priv_args,
    pub get_wireless_stats:
        Option<unsafe extern "C" fn(dev: *mut net_device) -> *mut iw_statistics>,
}

#[repr(C)]
pub struct iw_ioctl_description {
    pub header_type: u8,
    pub flags: u8,
    pub token_size: u16,
    pub min_tokens: u16,
    pub max_tokens: u16,
}

#[repr(C)]
pub struct iw_spy_data {
    pub spy_number: i32,
    pub spy_address: [[u8; ETH_ALEN]; IW_MAX_SPY],
    pub spy_stat: [iw_quality; IW_MAX_SPY],
    pub spy_thr_low: iw_quality,
    pub spy_thr_high: iw_quality,
    pub spy_thr_under: [u8; IW_MAX_SPY],
}

extern "C" {
    pub fn wireless_send_event(
        dev: *mut net_device,
        cmd: u32,
        wrqu: *mut iwreq_data,
        extra: *const i8,
    );

    #[cfg(CONFIG_WEXT_CORE)]
    pub fn wireless_nlevent_flush();

    pub fn iwe_stream_add_event(
        info: *mut iw_request_info,
        stream: *mut i8,
        ends: *mut i8,
        iwe: *mut iw_event,
        event_len: i32,
    ) -> *mut i8;

    pub fn iwe_stream_add_point(
        info: *mut iw_request_info,
        stream: *mut i8,
        ends: *mut i8,
        iwe: *mut iw_event,
        extra: *mut i8,
    ) -> *mut i8;
}

#[inline]
pub unsafe fn wireless_nlevent_flush() {
    #[cfg(not(CONFIG_WEXT_CORE))]
    {}
}

#[inline]
pub unsafe fn iwe_stream_lcp_len(info: *mut iw_request_info) -> i32 {
    #[cfg(CONFIG_COMPAT)]
    {
        if ((*info).flags & IW_REQUEST_FLAG_COMPAT) != 0 {
            return IW_EV_COMPAT_LCP_LEN;
        }
    }
    IW_EV_LCP_LEN
}

#[inline]
pub unsafe fn iwe_stream_point_len(info: *mut iw_request_info) -> i32 {
    #[cfg(CONFIG_COMPAT)]
    {
        if ((*info).flags & IW_REQUEST_FLAG_COMPAT) != 0 {
            return IW_EV_COMPAT_POINT_LEN;
        }
    }
    IW_EV_POINT_LEN
}

#[inline]
pub unsafe fn iwe_stream_event_len_adjust(info: *mut iw_request_info, mut event_len: i32) -> i32 {
    #[cfg(CONFIG_COMPAT)]
    {
        if ((*info).flags & IW_REQUEST_FLAG_COMPAT) != 0 {
            event_len -= IW_EV_LCP_LEN;
            event_len += IW_EV_COMPAT_LCP_LEN;
        }
    }
    event_len
}

#[inline]
pub unsafe fn iwe_stream_add_event_check(
    info: *mut iw_request_info,
    stream: *mut i8,
    ends: *mut i8,
    iwe: *mut iw_event,
    event_len: i32,
) -> *mut i8 {
    let res = iwe_stream_add_event(info, stream, ends, iwe, event_len);
    if res == stream { ERR_PTR(-E2BIG) } else { res }
}

#[inline]
pub unsafe fn iwe_stream_add_point_check(
    info: *mut iw_request_info,
    stream: *mut i8,
    ends: *mut i8,
    iwe: *mut iw_event,
    extra: *mut i8,
) -> *mut i8 {
    let res = iwe_stream_add_point(info, stream, ends, iwe, extra);
    if res == stream { ERR_PTR(-E2BIG) } else { res }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
