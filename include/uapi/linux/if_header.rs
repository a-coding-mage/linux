/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* Translated from linux/if.h. Included C headers and build-time compatibility
 * conditions are external dependencies and are intentionally not expanded. */

#[cfg(any())] // __UAPI_DEF_IF_IFNAMSIZ
pub const IFNAMSIZ: usize = 16;
pub const IFALIASZ: usize = 256;
pub const ALTIFNAMSIZ: usize = 128;

/* For glibc compatibility. The surrounding C condition is preserved here. */
#[cfg(any())] // __UAPI_DEF_IF_NET_DEVICE_FLAGS_LOWER_UP_DORMANT_ECHO != 0 || __UAPI_DEF_IF_NET_DEVICE_FLAGS != 0
#[repr(C)]
pub enum net_device_flags {
    IFF_UP = 1 << 0,
    IFF_BROADCAST = 1 << 1,
    IFF_DEBUG = 1 << 2,
    IFF_LOOPBACK = 1 << 3,
    IFF_POINTOPOINT = 1 << 4,
    IFF_NOTRAILERS = 1 << 5,
    IFF_RUNNING = 1 << 6,
    IFF_NOARP = 1 << 7,
    IFF_PROMISC = 1 << 8,
    IFF_ALLMULTI = 1 << 9,
    IFF_MASTER = 1 << 10,
    IFF_SLAVE = 1 << 11,
    IFF_MULTICAST = 1 << 12,
    IFF_PORTSEL = 1 << 13,
    IFF_AUTOMEDIA = 1 << 14,
    IFF_DYNAMIC = 1 << 15,
    IFF_LOWER_UP = 1 << 16,
    IFF_DORMANT = 1 << 17,
    IFF_ECHO = 1 << 18,
}

pub const IFF_VOLATILE: u32 = IFF_LOOPBACK | IFF_POINTOPOINT | IFF_BROADCAST |
    IFF_ECHO | IFF_MASTER | IFF_SLAVE | IFF_RUNNING | IFF_LOWER_UP | IFF_DORMANT;

pub const IF_GET_IFACE: u32 = 0x0001;
pub const IF_GET_PROTO: u32 = 0x0002;
pub const IF_IFACE_V35: u32 = 0x1000;
pub const IF_IFACE_V24: u32 = 0x1001;
pub const IF_IFACE_X21: u32 = 0x1002;
pub const IF_IFACE_T1: u32 = 0x1003;
pub const IF_IFACE_E1: u32 = 0x1004;
pub const IF_IFACE_SYNC_SERIAL: u32 = 0x1005;
pub const IF_IFACE_X21D: u32 = 0x1006;
pub const IF_PROTO_HDLC: u32 = 0x2000;
pub const IF_PROTO_PPP: u32 = 0x2001;
pub const IF_PROTO_CISCO: u32 = 0x2002;
pub const IF_PROTO_FR: u32 = 0x2003;
pub const IF_PROTO_FR_ADD_PVC: u32 = 0x2004;
pub const IF_PROTO_FR_DEL_PVC: u32 = 0x2005;
pub const IF_PROTO_X25: u32 = 0x2006;
pub const IF_PROTO_HDLC_ETH: u32 = 0x2007;
pub const IF_PROTO_FR_ADD_ETH_PVC: u32 = 0x2008;
pub const IF_PROTO_FR_DEL_ETH_PVC: u32 = 0x2009;
pub const IF_PROTO_FR_PVC: u32 = 0x200A;
pub const IF_PROTO_FR_ETH_PVC: u32 = 0x200B;
pub const IF_PROTO_RAW: u32 = 0x200C;

pub const IF_OPER_UNKNOWN: u32 = 0;
pub const IF_OPER_NOTPRESENT: u32 = 1;
pub const IF_OPER_DOWN: u32 = 2;
pub const IF_OPER_LOWERLAYERDOWN: u32 = 3;
pub const IF_OPER_TESTING: u32 = 4;
pub const IF_OPER_DORMANT: u32 = 5;
pub const IF_OPER_UP: u32 = 6;
pub const IF_LINK_MODE_DEFAULT: u32 = 0;
pub const IF_LINK_MODE_DORMANT: u32 = 1;
pub const IF_LINK_MODE_TESTING: u32 = 2;

#[repr(C)]
pub struct ifmap {
    pub mem_start: ::core::ffi::c_ulong,
    pub mem_end: ::core::ffi::c_ulong,
    pub base_addr: u16,
    pub irq: u8,
    pub dma: u8,
    pub port: u8,
}

#[repr(C)]
pub union if_settings_ifs_ifsu {
    pub raw_hdlc: *mut raw_hdlc_proto,
    pub cisco: *mut cisco_proto,
    pub fr: *mut fr_proto,
    pub fr_pvc: *mut fr_proto_pvc,
    pub fr_pvc_info: *mut fr_proto_pvc_info,
    pub x25: *mut x25_hdlc_proto,
    pub sync: *mut sync_serial_settings,
    pub te1: *mut te1_settings,
}

#[repr(C)]
pub struct if_settings {
    pub type_: u32,
    pub size: u32,
    pub ifs_ifsu: if_settings_ifs_ifsu,
}

#[repr(C)]
pub union ifreq_ifrn {
    pub ifrn_name: [::core::ffi::c_char; IFNAMSIZ],
}

#[repr(C)]
pub union ifreq_ifru {
    pub ifru_addr: sockaddr,
    pub ifru_dstaddr: sockaddr,
    pub ifru_broadaddr: sockaddr,
    pub ifru_netmask: sockaddr,
    pub ifru_hwaddr: sockaddr,
    pub ifru_flags: i16,
    pub ifru_ivalue: i32,
    pub ifru_mtu: i32,
    pub ifru_map: ifmap,
    pub ifru_slave: [::core::ffi::c_char; IFNAMSIZ],
    pub ifru_newname: [::core::ffi::c_char; IFNAMSIZ],
    pub ifru_data: *mut ::core::ffi::c_void,
    pub ifru_settings: if_settings,
}

pub const IFHWADDRLEN: usize = 6;
#[repr(C)]
pub struct ifreq {
    pub ifr_ifrn: ifreq_ifrn,
    pub ifr_ifru: ifreq_ifru,
}

#[repr(C)]
pub union ifconf_ifc_ifcu {
    pub ifcu_buf: *mut ::core::ffi::c_char,
    pub ifcu_req: *mut ifreq,
}

#[repr(C)]
pub struct ifconf {
    pub ifc_len: i32,
    pub ifc_ifcu: ifconf_ifc_ifcu,
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
