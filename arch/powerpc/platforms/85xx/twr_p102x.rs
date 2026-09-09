// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2010-2011, 2013 Freescale Semiconductor, Inc.
 *
 * Author: Michael Johnston <michael.johnston@freescale.com>
 *
 * Description:
 * TWR-P102x Board Setup
 */

// C includes provide the external kernel, architecture, SoC, and board symbols
// referenced below.

unsafe fn twr_p1025_pic_init() {
    let mpic: *mut mpic = mpic_alloc(
        core::ptr::null_mut(),
        0,
        MPIC_BIG_ENDIAN | MPIC_SINGLE_DEST_CPU,
        0,
        256,
        b" OpenPIC  \0".as_ptr() as *const core::ffi::c_char,
    );

    BUG_ON(mpic.is_null());
    mpic_init(mpic);
}

/* ************************************************************************
 *
 * Setup the architecture
 *
 */
unsafe fn twr_p1025_setup_arch() {
    if !ppc_md.progress.is_null() {
        ((*ppc_md.progress))(b"twr_p1025_setup_arch()\0".as_ptr() as *const core::ffi::c_char, 0);
    }

    mpc85xx_smp_init();

    fsl_pci_assign_primary();

    // CONFIG_QUICC_ENGINE
    #[cfg(CONFIG_QUICC_ENGINE)]
    {
        mpc85xx_qe_par_io_init();

        // IS_ENABLED(CONFIG_UCC_GETH) || IS_ENABLED(CONFIG_SERIAL_QE)
        #[cfg(any(CONFIG_UCC_GETH, CONFIG_SERIAL_QE))]
        {
            if machine_is(twr_p1025) {
                let mut guts: *mut ccsr_guts = core::ptr::null_mut();
                let np: *mut device_node;

                np = of_find_compatible_node(
                    core::ptr::null_mut(),
                    core::ptr::null_mut(),
                    b"fsl,p1021-guts\0".as_ptr() as *const core::ffi::c_char,
                );
                if !np.is_null() {
                    guts = of_iomap(np, 0);
                    if guts.is_null() {
                        pr_err!("twr_p1025: could not map global utilities register\n");
                    } else {
                        /* P1025 has pins muxed for QE and other functions. To
                         * enable QE UEC mode, we need to set bit QE0 for UCC1
                         * in Eth mode, QE0 and QE3 for UCC5 in Eth mode, QE9
                         * and QE12 for QE MII management signals in PMUXCR
                         * register.
                         * Set QE mux bits in PMUXCR */
                        setbits32(
                            core::ptr::addr_of_mut!((*guts).pmuxcr),
                            MPC85xx_PMUXCR_QE(0)
                                | MPC85xx_PMUXCR_QE(3)
                                | MPC85xx_PMUXCR_QE(9)
                                | MPC85xx_PMUXCR_QE(12),
                        );
                        iounmap(guts);

                        // IS_ENABLED(CONFIG_SERIAL_QE)
                        #[cfg(CONFIG_SERIAL_QE)]
                        {
                            /* On P1025TWR board, the UCC7 acted as UART port.
                             * However, The UCC7's CTS pin is low level in default,
                             * it will impact the transmission in full duplex
                             * communication. So disable the Flow control pin PA18.
                             * The UCC7 UART just can use RXD and TXD pins.
                             */
                            par_io_config_pin(0, 18, 0, 0, 0, 0);
                        }

                        /* Drive PB29 to CPLD low - CPLD will then change
                         * muxing from LBC to QE */
                        par_io_config_pin(1, 29, 1, 0, 0, 0);
                        par_io_data_set(1, 29, 0);
                    }
                    of_node_put(np);
                }
            }
        }
    }

    pr_info!("TWR-P1025 board from Freescale Semiconductor\n");
}

// machine_arch_initcall(twr_p1025, mpc85xx_common_publish_devices);
machine_arch_initcall!(twr_p1025, mpc85xx_common_publish_devices);

// define_machine(twr_p1025)
static twr_p1025: machine_desc = machine_desc {
    name: b"TWR-P1025\0".as_ptr() as *const core::ffi::c_char,
    compatible: b"fsl,TWR-P1025\0".as_ptr() as *const core::ffi::c_char,
    setup_arch: Some(twr_p1025_setup_arch),
    init_IRQ: Some(twr_p1025_pic_init),
    // CONFIG_PCI
    #[cfg(CONFIG_PCI)]
    pcibios_fixup_bus: Some(fsl_pcibios_fixup_bus),
    get_irq: Some(mpic_get_irq),
    progress: Some(udbg_progress),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
