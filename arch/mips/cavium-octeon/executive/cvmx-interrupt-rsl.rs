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

/*
 * Utility functions to decode Octeon's RSL_INT_BLOCKS
 * interrupts into error messages.
 */

/* Dependencies supplied by the surrounding OCTEON definitions. */

/* PRINT_ERROR is intentionally absent when not supplied by the build. */

extern "C" {
    fn __cvmx_interrupt_gmxx_rxx_int_en_enable(index: ::core::ffi::c_int, block: ::core::ffi::c_int);
}

/**
 * Enable ASX error interrupts that exist on CN3XXX, CN50XX, and
 * CN58XX.
 *
 * @block:  Interface to enable 0-1
 */
pub unsafe fn __cvmx_interrupt_asxx_enable(block: ::core::ffi::c_int) {
    let mask: ::core::ffi::c_int;
    let mut csr: cvmx_asxx_int_en;
    /*
     * CN38XX and CN58XX have two interfaces with 4 ports per
     * interface. All other chips have a max of 3 ports on
     * interface 0
     */
    if OCTEON_IS_MODEL(OCTEON_CN38XX) || OCTEON_IS_MODEL(OCTEON_CN58XX) {
        mask = 0xf; /* Set enables for 4 ports */
    } else {
        mask = 0x7; /* Set enables for 3 ports */
    }

    /* Enable interface interrupts */
    csr.u64 = cvmx_read_csr(CVMX_ASXX_INT_EN(block));
    csr.s.txpsh = mask;
    csr.s.txpop = mask;
    csr.s.ovrflw = mask;
    cvmx_write_csr(CVMX_ASXX_INT_EN(block), csr.u64);
}

/**
 * Enable GMX error reporting for the supplied interface
 *
 * @interface: Interface to enable
 */
pub unsafe fn __cvmx_interrupt_gmxx_enable(interface: ::core::ffi::c_int) {
    let mut mode: cvmx_gmxx_inf_mode;
    let mut gmx_tx_int_en: cvmx_gmxx_tx_int_en;
    let num_ports: ::core::ffi::c_int;
    let mut index: ::core::ffi::c_int;

    mode.u64 = cvmx_read_csr(CVMX_GMXX_INF_MODE(interface));

    if OCTEON_IS_MODEL(OCTEON_CN56XX) || OCTEON_IS_MODEL(OCTEON_CN52XX) {
        if mode.s.en != 0 {
            num_ports = match mode.cn52xx.mode {
                1 => 1, /* XAUI */
                2 | 3 => 4, /* SGMII, PICMG */
                _ => 0, /* Disabled */
            };
        } else {
            num_ports = 0;
        }
    } else if mode.s.en != 0 {
        if OCTEON_IS_MODEL(OCTEON_CN38XX) || OCTEON_IS_MODEL(OCTEON_CN58XX) {
            /*
             * SPI on CN38XX and CN58XX report all
             * errors through port 0.  RGMII needs
             * to check all 4 ports
             */
            if mode.s.type != 0 {
                num_ports = 1;
            } else {
                num_ports = 4;
            }
        } else {
            /*
             * CN30XX, CN31XX, and CN50XX have two
             * or three ports. GMII and MII has 2,
             * RGMII has three
             */
            if mode.s.type != 0 {
                num_ports = 2;
            } else {
                num_ports = 3;
            }
        }
    } else {
        num_ports = 0;
    }

    gmx_tx_int_en.u64 = 0;
    if num_ports != 0 {
        if OCTEON_IS_MODEL(OCTEON_CN38XX) || OCTEON_IS_MODEL(OCTEON_CN58XX) {
            gmx_tx_int_en.cn38xx.ncb_nxa = 1;
        }
        gmx_tx_int_en.s.pko_nxa = 1;
    }
    gmx_tx_int_en.s.undflw = (1 << num_ports) - 1;
    cvmx_write_csr(CVMX_GMXX_TX_INT_EN(interface), gmx_tx_int_en.u64);
    index = 0;
    while index < num_ports {
        __cvmx_interrupt_gmxx_rxx_int_en_enable(index, interface);
        index += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
