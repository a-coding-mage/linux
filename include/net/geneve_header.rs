/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by <net/udp_tunnel.h> in the C source.

pub const GENEVE_UDP_PORT: u16 = 6081;

/* Geneve Header:
 *  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
 *  |Ver|  Opt Len  |O|C|    Rsvd.  |          Protocol Type        |
 *  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
 *  |        Virtual Network Identifier (VNI)       |    Reserved   |
 *  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
 *  |                    Variable Length Options                    |
 *  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
 *
 * Option Header:
 *  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
 *  |          Option Class         |      Type     |R|R|R| Length  |
 *  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
 *  |                      Variable Option Data                     |
 *  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
 */

#[repr(C)]
pub struct geneve_opt {
    pub opt_class: u16,
    pub type_: u8,
    // On little-endian bitfields: length:5, r3:1, r2:1, r1:1.
    // On big-endian bitfields: r1:1, r2:1, r3:1, length:5.
    pub length_rsvd: u8,
    pub opt_data: [u8; 0],
}

pub const GENEVE_CRIT_OPT_TYPE: u8 = 1 << 7;

#[repr(C)]
pub struct genevehdr {
    // On little-endian bitfields: opt_len:6, ver:2.
    // On big-endian bitfields: ver:2, opt_len:6.
    pub opt_len_ver: u8,
    // On little-endian bitfields: rsvd1:6, critical:1, oam:1.
    // On big-endian bitfields: oam:1, critical:1, rsvd1:6.
    pub rsvd1_critical_oam: u8,
    pub proto_type: u16,
    pub vni: [u8; 3],
    pub rsvd2: u8,
    pub options: [u8; 0],
}

pub unsafe fn netif_is_geneve(dev: *const crate::net_device) -> bool {
    (*dev).rtnl_link_ops.is_some()
        && !crate::strcmp((*dev).rtnl_link_ops.unwrap().kind, b"geneve\0".as_ptr())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
