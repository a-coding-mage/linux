/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Copyright (C) 1997 Martin Mares
 *
 *  Automatic IP Layer Configuration
 */

/* The following are initdata: */

/* linux/types.h */

unsafe extern "C" {
    pub static mut ic_proto_enabled: ::core::ffi::c_int;
    /* Protocols enabled (see IC_xxx) */
    pub static mut ic_set_manually: ::core::ffi::c_int;
    /* IPconfig parameters set manually */

    pub static mut ic_myaddr: u32;
    /* My IP address */
    pub static mut ic_gateway: u32;
    /* Gateway IP address */

    pub static mut ic_servaddr: u32;
    /* Boot server IP address */

    pub static mut root_server_addr: u32;
    /* Address of NFS server */
    pub static mut root_server_path: [u8; 0];
    /* Path to mount as root */
}

/* bits in ic_proto_{enabled,used} */
pub const IC_PROTO: u32 = 0xFF; /* Protocols mask: */
pub const IC_BOOTP: u32 = 0x01; /*   BOOTP (or DHCP, see below) */
pub const IC_RARP: u32 = 0x02; /*   RARP */
pub const IC_USE_DHCP: u32 = 0x100; /* If on, use DHCP instead of BOOTP */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
