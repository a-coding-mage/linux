/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Translated from <linux/atmbr2684.h>. */

/* Type of media we're bridging (ethernet, token ring, etc)  Currently only
 * ethernet is supported. */
pub const BR2684_MEDIA_ETHERNET: i32 = 0; /* 802.3 */
pub const BR2684_MEDIA_802_4: i32 = 1; /* 802.4 */
pub const BR2684_MEDIA_TR: i32 = 2; /* 802.5 - token ring */
pub const BR2684_MEDIA_FDDI: i32 = 3;
pub const BR2684_MEDIA_802_6: i32 = 4; /* 802.6 */

/* Used only at device creation: payload is routed, not bridged. */
pub const BR2684_FLAG_ROUTED: i32 = 1 << 16;

/* Is there FCS inbound on this VC? */
pub const BR2684_FCSIN_NO: i32 = 0;
pub const BR2684_FCSIN_IGNORE: i32 = 1;
pub const BR2684_FCSIN_VERIFY: i32 = 2;

/* Is there FCS outbound on this VC? */
pub const BR2684_FCSOUT_NO: i32 = 0;
pub const BR2684_FCSOUT_SENDZERO: i32 = 1;
pub const BR2684_FCSOUT_GENERATE: i32 = 2;

/* Does this VC include LLC encapsulation? */
pub const BR2684_ENCAPS_VC: i32 = 0; /* VC-mux */
pub const BR2684_ENCAPS_LLC: i32 = 1;
pub const BR2684_ENCAPS_AUTODETECT: i32 = 2; /* Unsupported */

/* Is this VC bridged or routed? */
pub const BR2684_PAYLOAD_ROUTED: i32 = 0;
pub const BR2684_PAYLOAD_BRIDGED: i32 = 1;

/* IFNAMSIZ is supplied by the Linux interface header dependency. */
#[repr(C)]
pub struct atm_newif_br2684 {
    pub backend_num: atm_backend_t, /* ATM_BACKEND_BR2684 */
    pub media: ::core::ffi::c_int, /* BR2684_MEDIA_*, flags in upper bits */
    pub ifname: [::core::ffi::c_char; IFNAMSIZ],
    pub mtu: ::core::ffi::c_int,
}

/* Interface selection method. */
pub const BR2684_FIND_BYNOTHING: i32 = 0;
pub const BR2684_FIND_BYNUM: i32 = 1;
pub const BR2684_FIND_BYIFNAME: i32 = 2;

#[repr(C)]
pub union br2684_if_spec_spec {
    pub ifname: [::core::ffi::c_char; IFNAMSIZ],
    pub devnum: ::core::ffi::c_int,
}

#[repr(C)]
pub struct br2684_if_spec {
    pub method: ::core::ffi::c_int, /* BR2684_FIND_* */
    pub spec: br2684_if_spec_spec,
}

#[repr(C)]
pub struct atm_backend_br2684 {
    pub backend_num: atm_backend_t, /* ATM_BACKEND_BR2684 */
    pub ifspec: br2684_if_spec,
    pub fcs_in: ::core::ffi::c_int, /* BR2684_FCSIN_* */
    pub fcs_out: ::core::ffi::c_int, /* BR2684_FCSOUT_* */
    pub fcs_auto: ::core::ffi::c_int, /* 1: fcs_{in,out} disabled if no FCS rx'ed */
    pub encaps: ::core::ffi::c_int, /* BR2684_ENCAPS_* */
    pub has_vpiid: ::core::ffi::c_int, /* 1: use vpn_id - Unsupported */
    pub vpn_id: [__u8; 7],
    pub send_padding: ::core::ffi::c_int, /* unsupported */
    pub min_size: ::core::ffi::c_int, /* we will pad smaller packets than this */
}

#[repr(C)]
pub struct br2684_filter {
    pub prefix: __be32, /* network byte order */
    pub netmask: __be32, /* 0 = disable filter */
}

#[repr(C)]
pub struct br2684_filter_set {
    pub ifspec: br2684_if_spec,
    pub filter: br2684_filter,
}

#[repr(i32)]
pub enum br2684_payload {
    p_routed = BR2684_PAYLOAD_ROUTED,
    p_bridged = BR2684_PAYLOAD_BRIDGED,
}

/* The value uses the external Linux _IOW ioctl encoding macro. */
pub const BR2684_SETFILT: u32 = _IOW(b'a', ATMIOC_BACKEND + 0, br2684_filter_set);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
