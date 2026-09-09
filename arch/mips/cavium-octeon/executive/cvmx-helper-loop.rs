/***********************license start***************
 * Author: Cavium Networks
 *
 * Contact: support@caviumnetworks.com
 * This file is part of the OCTEON SDK
 *
 * Copyright (c) 2003-2008 Cavium Networks
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

/*
 * Functions for LOOP initialization, configuration,
 * and monitoring.
 *
 * C dependencies from the original implementation are supplied externally.
 */

/**
 * Probe a LOOP interface and determine the number of ports
 * connected to it. The LOOP interface should still be down
 * after this call.
 *
 * @interface: Interface to probe
 *
 * Returns Number of ports on the interface. Zero to disable.
 */
pub unsafe fn __cvmx_helper_loop_probe(interface: i32) -> i32 {
    let mut ipd_sub_port_fcs: cvmx_ipd_sub_port_fcs = core::mem::zeroed();
    let num_ports: i32 = 4;
    let mut port: i32 = 0;

    /* We need to disable length checking so packet < 64 bytes and jumbo
       frames don't get errors */
    while port < num_ports {
        let mut port_cfg: cvmx_pip_prt_cfgx = core::mem::zeroed();
        let ipd_port: i32 = cvmx_helper_get_ipd_port(interface, port);
        port_cfg.u64 = cvmx_read_csr(CVMX_PIP_PRT_CFGX(ipd_port));
        port_cfg.s.maxerr_en = 0;
        port_cfg.s.minerr_en = 0;
        cvmx_write_csr(CVMX_PIP_PRT_CFGX(ipd_port), port_cfg.u64);
        port += 1;
    }

    /* Disable FCS stripping for loopback ports */
    ipd_sub_port_fcs.u64 = cvmx_read_csr(CVMX_IPD_SUB_PORT_FCS);
    ipd_sub_port_fcs.s.port_bit2 = 0;
    cvmx_write_csr(CVMX_IPD_SUB_PORT_FCS, ipd_sub_port_fcs.u64);
    num_ports
}

/**
 * Bringup and enable a LOOP interface. After this call packet
 * I/O should be fully functional. This is called with IPD
 * enabled but PKO disabled.
 *
 * @interface: Interface to bring up
 *
 * Returns Zero on success, negative on failure
 */
pub unsafe fn __cvmx_helper_loop_enable(_interface: i32) -> i32 {
    /* Do nothing. */
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
