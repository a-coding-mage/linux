/***********************license start***************
 * Author: Cavium Networks
 *
 * Contact: support@caviumnetworks.com
 * This file is part of the OCTEON SDK
 *
 * Copyright (C) 2003-2018 Cavium, Inc.
 *
 * This file is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License, Version 2, as
 * published by the Free Software Foundation.
 *
 * This file is distributed in the hope that it will be useful, but
 * AS-IS and WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, TITLE, or
 * NONINFRINGEMENT.  See the GNU General Public License for more
 * details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this file; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA
 * or visit http://www.gnu.org/licenses/.
 *
 * This file may also be available under a different license from Cavium.
 * Contact Cavium Networks for more information
 ***********************license end**************************************/

/* Functions for SPI initialization, configuration, and monitoring. */

// C includes and build-time configuration are supplied by the surrounding
// OCTEON bindings. CVMX_HELPER_SPI_TIMEOUT defaults to 10 when unspecified.
pub const CVMX_HELPER_SPI_TIMEOUT: i32 = 10;

extern "C" {
    fn cvmx_sysinfo_get() -> *const cvmx_sysinfo;
    fn cvmx_spi4000_is_present(interface: i32) -> i32;
    fn cvmx_read_csr(address: u64) -> u64;
    fn cvmx_write_csr(address: u64, value: u64);
    fn __cvmx_helper_setup_gmx(interface: i32, num_ports: i32);
    fn cvmx_helper_ports_on_interface(interface: i32) -> i32;
    fn cvmx_spi_start_interface(interface: i32, mode: i32, timeout: i32, num_ports: i32);
    fn cvmx_spi4000_initialize(interface: i32);
    fn __cvmx_interrupt_spxx_int_msk_enable(interface: i32);
    fn __cvmx_interrupt_stxx_int_msk_enable(interface: i32);
    fn __cvmx_interrupt_gmxx_enable(interface: i32);
    fn cvmx_helper_get_interface_num(ipd_port: i32) -> i32;
    fn cvmx_helper_get_interface_index_num(ipd_port: i32) -> i32;
    fn cvmx_spi4000_check_speed(interface: i32, index: i32) -> cvmx_gmxx_rxx_rx_inbnd;
    fn CVMX_PIP_PRT_CFGX(ipd_port: i32) -> u64;
}

#[repr(C)]
pub struct cvmx_sysinfo { pub board_type: i32 }

#[repr(C)]
pub union cvmx_pko_reg_crc_enable { pub u64_: u64, pub s: cvmx_pko_reg_crc_enable_s }
#[repr(C)]
pub struct cvmx_pko_reg_crc_enable_s { pub enable: u64 }

#[repr(C)]
pub union cvmx_pip_prt_cfgx { pub u64_: u64, pub s: cvmx_pip_prt_cfgx_s }
#[repr(C)]
pub struct cvmx_pip_prt_cfgx_s { pub crc_en: u64 }

#[repr(C)]
pub union cvmx_gmxx_rxx_rx_inbnd { pub u64_: u64, pub s: cvmx_gmxx_rxx_rx_inbnd_s }
#[repr(C)]
pub struct cvmx_gmxx_rxx_rx_inbnd_s { pub status: u64, pub duplex: u64, pub speed: u64 }

#[repr(C)]
pub union cvmx_helper_link_info { pub u64_: u64, pub s: cvmx_helper_link_info_s }
#[repr(C)]
pub struct cvmx_helper_link_info_s { pub link_up: u64, pub full_duplex: u64, pub speed: u64 }

pub const CVMX_BOARD_TYPE_SIM: i32 = 0;
pub const CVMX_SPI_MODE_DUPLEX: i32 = 0;

extern "C" {
    static CVMX_PKO_REG_CRC_ENABLE: u64;
}

#[allow(non_snake_case)]
pub unsafe fn __cvmx_helper_spi_enumerate(interface: i32) -> i32 {
    if ((*cvmx_sysinfo_get()).board_type != CVMX_BOARD_TYPE_SIM) && cvmx_spi4000_is_present(interface) != 0 { 10 } else { 16 }
}

#[allow(non_snake_case)]
pub unsafe fn __cvmx_helper_spi_probe(interface: i32) -> i32 {
    let num_ports: i32;
    if ((*cvmx_sysinfo_get()).board_type != CVMX_BOARD_TYPE_SIM) && cvmx_spi4000_is_present(interface) != 0 {
        num_ports = 10;
    } else {
        let mut enable = cvmx_pko_reg_crc_enable { u64_: cvmx_read_csr(CVMX_PKO_REG_CRC_ENABLE) };
        num_ports = 16;
        enable.s.enable |= 0xffffu64 << ((interface * 16) as u32);
        cvmx_write_csr(CVMX_PKO_REG_CRC_ENABLE, enable.u64_);
    }
    __cvmx_helper_setup_gmx(interface, num_ports);
    num_ports
}

#[allow(non_snake_case)]
pub unsafe fn __cvmx_helper_spi_enable(interface: i32) -> i32 {
    let num_ports = cvmx_helper_ports_on_interface(interface);
    let mut ipd_port = interface * 16;
    while ipd_port < interface * 16 + num_ports {
        let mut port_config = cvmx_pip_prt_cfgx { u64_: cvmx_read_csr(CVMX_PIP_PRT_CFGX(ipd_port)) };
        port_config.s.crc_en = 1;
        cvmx_write_csr(CVMX_PIP_PRT_CFGX(ipd_port), port_config.u64_);
        ipd_port += 1;
    }
    if (*cvmx_sysinfo_get()).board_type != CVMX_BOARD_TYPE_SIM {
        cvmx_spi_start_interface(interface, CVMX_SPI_MODE_DUPLEX, CVMX_HELPER_SPI_TIMEOUT, num_ports);
        if cvmx_spi4000_is_present(interface) != 0 { cvmx_spi4000_initialize(interface); }
    }
    __cvmx_interrupt_spxx_int_msk_enable(interface);
    __cvmx_interrupt_stxx_int_msk_enable(interface);
    __cvmx_interrupt_gmxx_enable(interface);
    0
}

#[allow(non_snake_case)]
pub unsafe fn __cvmx_helper_spi_link_get(ipd_port: i32) -> cvmx_helper_link_info {
    let interface = cvmx_helper_get_interface_num(ipd_port);
    let index = cvmx_helper_get_interface_index_num(ipd_port);
    let mut result = cvmx_helper_link_info { u64_: 0 };
    if (*cvmx_sysinfo_get()).board_type == CVMX_BOARD_TYPE_SIM {
        result.s.link_up = 1; result.s.full_duplex = 1; result.s.speed = 10000;
    } else if cvmx_spi4000_is_present(interface) != 0 {
        let inband = cvmx_spi4000_check_speed(interface, index);
        result.s.link_up = inband.s.status;
        result.s.full_duplex = inband.s.duplex;
        match inband.s.speed {
            0 => result.s.speed = 10,
            1 => result.s.speed = 100,
            2 => result.s.speed = 1000,
            3 => { result.s.speed = 0; result.s.link_up = 0; },
            _ => {}
        }
    } else {
        result.s.link_up = 1; result.s.full_duplex = 1; result.s.speed = 10000;
    }
    result
}

#[allow(non_snake_case)]
pub unsafe fn __cvmx_helper_spi_link_set(_ipd_port: i32, _link_info: cvmx_helper_link_info) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
