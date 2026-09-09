// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * arch/powerpc/platforms/83xx/asp834x.c
 *
 * Analogue & Micro ASP8347 board specific routines
 * clone of mpc834x_itx
 *
 * Copyright 2008 Codehermit
 *
 * Maintainer: Bryan O'Donoghue <bodonoghue@codhermit.ie>
 */

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    fn mpc83xx_setup_arch();
    fn mpc834x_usb_cfg();
    fn mpc83xx_declare_of_platform_devices();
    fn mpc83xx_setup_pci();
    fn mpc83xx_ipic_init_IRQ();
    fn ipic_get_irq() -> i32;
    fn mpc83xx_restart();
    fn mpc83xx_time_init();
    fn udbg_progress();
}

/* ************************************************************************
 *
 * Setup the architecture
 *
 */
unsafe fn asp834x_setup_arch() {
    mpc83xx_setup_arch();
    mpc834x_usb_cfg();
}

// Equivalent of: machine_device_initcall(asp834x, mpc83xx_declare_of_platform_devices);
// Equivalent of the kernel's define_machine(asp834x) registration:
//
// .name        = "ASP8347E"
// .compatible  = "analogue-and-micro,asp8347e"
// .setup_arch  = asp834x_setup_arch
// .discover_phbs = mpc83xx_setup_pci
// .init_IRQ     = mpc83xx_ipic_init_IRQ
// .get_irq      = ipic_get_irq
// .restart      = mpc83xx_restart
// .time_init    = mpc83xx_time_init
// .progress     = udbg_progress


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
