/* SPDX-License-Identifier: GPL-2.0 */
/*
 * pxa168 ethernet platform device data definition file.
 */

// Dependency supplied by the Linux PHY interfaces.
use core::ffi::c_int;

// Equivalent of the externally supplied Linux `phy_interface_t` type.
// This name is intentionally left unresolved for integration with the
// surrounding translated Linux interfaces.
use crate::phy_interface_t;

#[repr(C)]
pub struct pxa168_eth_platform_data {
    pub port_number: c_int,
    pub phy_addr: c_int,

    /*
     * If speed is 0, then speed and duplex are autonegotiated.
     */
    pub speed: c_int,  /* 0, SPEED_10, SPEED_100 */
    pub duplex: c_int, /* DUPLEX_HALF or DUPLEX_FULL */
    pub intf: phy_interface_t,

    /*
     * Override default RX/TX queue sizes if nonzero.
     */
    pub rx_queue_size: c_int,
    pub tx_queue_size: c_int,

    /*
     * init callback is used for board specific initialization
     * e.g on Aspenite its used to initialize the PHY transceiver.
     */
    pub init: Option<unsafe extern "C" fn() -> c_int>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
