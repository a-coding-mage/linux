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
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE, TITLE, or
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
 * Functions for NPI initialization, configuration,
 * and monitoring.
 */
// Dependencies supplied by the surrounding OCTEON environment are intentionally external.

/**
 * Probe a NPI interface and determine the number of ports
 * connected to it. The NPI interface should still be down
 * after this call.
 *
 * @interface: Interface to probe
 *
 * Returns Number of ports on the interface. Zero to disable.
 */
pub unsafe fn __cvmx_helper_npi_probe(interface: i32) -> i32 {
    let _ = interface;
    // Corresponds to: #if CVMX_PKO_QUEUES_PER_PORT_PCI > 0
    #[cfg(feature = "CVMX_PKO_QUEUES_PER_PORT_PCI")]
    {
        if OCTEON_IS_MODEL(OCTEON_CN38XX) || OCTEON_IS_MODEL(OCTEON_CN58XX) {
            return 4;
        } else if OCTEON_IS_MODEL(OCTEON_CN56XX)
            && !OCTEON_IS_MODEL(OCTEON_CN56XX_PASS1_X)
        {
            /* The packet engines didn't exist before pass 2 */
            return 4;
        } else if OCTEON_IS_MODEL(OCTEON_CN52XX)
            && !OCTEON_IS_MODEL(OCTEON_CN52XX_PASS1_X)
        {
            /* The packet engines didn't exist before pass 2 */
            return 4;
        }
    }
    0
}

/**
 * Bringup and enable a NPI interface. After this call packet
 * I/O should be fully functional. This is called with IPD
 * enabled but PKO disabled.
 *
 * @interface: Interface to bring up
 *
 * Returns Zero on success, negative on failure
 */
pub unsafe fn __cvmx_helper_npi_enable(interface: i32) -> i32 {
    /*
     * On CN50XX, CN52XX, and CN56XX we need to disable length
     * checking so packet < 64 bytes and jumbo frames don't get
     * errors.
     */
    if !OCTEON_IS_MODEL(OCTEON_CN3XXX) && !OCTEON_IS_MODEL(OCTEON_CN58XX) {
        let num_ports = cvmx_helper_ports_on_interface(interface);
        let mut port = 0;
        while port < num_ports {
            let mut port_cfg: cvmx_pip_prt_cfgx = core::mem::zeroed();
            let ipd_port = cvmx_helper_get_ipd_port(interface, port);
            port_cfg.u64 = cvmx_read_csr(CVMX_PIP_PRT_CFGX(ipd_port));
            port_cfg.s.maxerr_en = 0;
            port_cfg.s.minerr_en = 0;
            cvmx_write_csr(CVMX_PIP_PRT_CFGX(ipd_port), port_cfg.u64);
            port += 1;
        }
    }

    /* Enables are controlled by the remote host, so nothing to do here */
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
