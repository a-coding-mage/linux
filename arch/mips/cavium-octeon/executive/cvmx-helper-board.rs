/***********************license start***************
 * Author: Cavium Networks
 *
 * Contact: support@caviumnetworks.com
 * This file is part of the OCTEON SDK
 *
 * Copyright (c) 2003-2008 Cavium Networks
 *
 * This file is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License, version 2, as
 * published by the Free Software Foundation.
 ***********************license end**************************************/

/* Helper functions to abstract board specific data about network ports. */

/* External declarations and constants are supplied by the translated headers. */

pub unsafe fn cvmx_helper_board_get_mii_address(ipd_port: i32) -> i32 {
    match cvmx_sysinfo_get().board_type {
        CVMX_BOARD_TYPE_SIM => -1,
        CVMX_BOARD_TYPE_EBT3000 | CVMX_BOARD_TYPE_EBT5800 |
        CVMX_BOARD_TYPE_THUNDER | CVMX_BOARD_TYPE_NICPRO2 => {
            if ipd_port >= 16 && ipd_port < 20 { ipd_port - 16 } else { -1 }
        }
        CVMX_BOARD_TYPE_KODAMA | CVMX_BOARD_TYPE_EBH3100 |
        CVMX_BOARD_TYPE_HIKARI | CVMX_BOARD_TYPE_CN3010_EVB_HS5 |
        CVMX_BOARD_TYPE_CN3005_EVB_HS5 | CVMX_BOARD_TYPE_CN3020_EVB_HS5 => {
            if ipd_port == 0 { 4 } else if ipd_port == 1 { 9 } else { -1 }
        }
        CVMX_BOARD_TYPE_NAC38 => {
            if ipd_port >= 0 && ipd_port < 4 { ipd_port }
            else if ipd_port >= 16 && ipd_port < 20 { ipd_port - 16 + 4 }
            else { -1 }
        }
        CVMX_BOARD_TYPE_EBH3000 => -1,
        CVMX_BOARD_TYPE_EBH5200 | CVMX_BOARD_TYPE_EBH5201 | CVMX_BOARD_TYPE_EBT5200 => {
            if ipd_port >= CVMX_HELPER_BOARD_MGMT_IPD_PORT &&
               ipd_port < CVMX_HELPER_BOARD_MGMT_IPD_PORT + 2 {
                ipd_port - CVMX_HELPER_BOARD_MGMT_IPD_PORT
            } else if ipd_port >= 0 && ipd_port < 4 { ipd_port + 2 } else { -1 }
        }
        CVMX_BOARD_TYPE_EBH5600 | CVMX_BOARD_TYPE_EBH5601 | CVMX_BOARD_TYPE_EBH5610 => {
            if ipd_port == CVMX_HELPER_BOARD_MGMT_IPD_PORT { 0 }
            else if ipd_port >= 0 && ipd_port < 4 { ipd_port + 1 } else { -1 }
        }
        CVMX_BOARD_TYPE_CUST_NB5 => if ipd_port == 2 { 4 } else { -1 },
        CVMX_BOARD_TYPE_NIC_XLE_4G => if ipd_port >= 16 && ipd_port < 20 { ipd_port - 16 + 1 } else { -1 },
        CVMX_BOARD_TYPE_NIC_XLE_10G | CVMX_BOARD_TYPE_NIC10E => -1,
        CVMX_BOARD_TYPE_NIC4E => if ipd_port >= 0 && ipd_port <= 3 { (ipd_port + 0x1f) & 0x1f } else { -1 },
        CVMX_BOARD_TYPE_NIC2E => if ipd_port >= 0 && ipd_port <= 1 { ipd_port + 1 } else { -1 },
        CVMX_BOARD_TYPE_BBGW_REF => -1,
        CVMX_BOARD_TYPE_CUST_WSX16 => {
            if ipd_port >= 0 && ipd_port <= 3 { ipd_port }
            else if ipd_port >= 16 && ipd_port <= 19 { ipd_port - 16 + 4 } else { -1 }
        }
        CVMX_BOARD_TYPE_UBNT_E100 => if ipd_port >= 0 && ipd_port <= 2 { 7 - ipd_port } else { -1 },
        CVMX_BOARD_TYPE_KONTRON_S1901 => if ipd_port == CVMX_HELPER_BOARD_MGMT_IPD_PORT { 1 } else { -1 },
        _ => {
            cvmx_dprintf("cvmx_helper_board_get_mii_address: Unknown board type %d\n", cvmx_sysinfo_get().board_type);
            -1
        }
    }
}

pub unsafe fn __cvmx_helper_board_link_get(ipd_port: i32) -> cvmx_helper_link_info {
    let mut result: cvmx_helper_link_info;
    WARN_ONCE(!octeon_is_simulation(), "Using deprecated link status - please update your DT");
    result.u64 = 0;
    if octeon_is_simulation() {
        result.s.link_up = 1; result.s.full_duplex = 1; result.s.speed = 1000; return result;
    }
    if OCTEON_IS_MODEL(OCTEON_CN3XXX) || OCTEON_IS_MODEL(OCTEON_CN58XX) || OCTEON_IS_MODEL(OCTEON_CN50XX) {
        let mut inband_status: cvmx_gmxx_rxx_rx_inbnd;
        let interface = cvmx_helper_get_interface_num(ipd_port);
        let index = cvmx_helper_get_interface_index_num(ipd_port);
        inband_status.u64 = cvmx_read_csr(CVMX_GMXX_RXX_RX_INBND(index, interface));
        result.s.link_up = inband_status.s.status;
        result.s.full_duplex = inband_status.s.duplex;
        match inband_status.s.speed {
            0 => result.s.speed = 10,
            1 => result.s.speed = 100,
            2 => result.s.speed = 1000,
            3 => result.u64 = 0,
            _ => {}
        }
    } else { result.u64 = 0; }
    if !result.s.link_up { result.u64 = 0; }
    result
}

pub unsafe fn __cvmx_helper_board_interface_probe(interface: i32, supported_ports: i32) -> i32 {
    match cvmx_sysinfo_get().board_type {
        CVMX_BOARD_TYPE_CN3005_EVB_HS5 | CVMX_BOARD_TYPE_BBGW_REF => if interface == 0 { 2 } else { supported_ports },
        CVMX_BOARD_TYPE_NIC_XLE_4G => if interface == 0 { 0 } else { supported_ports },
        CVMX_BOARD_TYPE_EBH5600 => if interface == 1 { 0 } else { supported_ports },
        _ => supported_ports
    }
}

pub unsafe fn __cvmx_helper_board_usb_get_clock_type() -> cvmx_helper_board_usb_clock_types {
    match cvmx_sysinfo_get().board_type {
        CVMX_BOARD_TYPE_BBGW_REF | CVMX_BOARD_TYPE_LANAI2_A | CVMX_BOARD_TYPE_LANAI2_U |
        CVMX_BOARD_TYPE_LANAI2_G | CVMX_BOARD_TYPE_NIC10E_66 | CVMX_BOARD_TYPE_UBNT_E100 => USB_CLOCK_TYPE_CRYSTAL_12,
        CVMX_BOARD_TYPE_NIC10E => USB_CLOCK_TYPE_REF_12,
        _ => if OCTEON_IS_OCTEON2() { USB_CLOCK_TYPE_CRYSTAL_12 } else { USB_CLOCK_TYPE_REF_48 }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
