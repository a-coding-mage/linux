// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-dove/cm-a510.c
 *
 * Copyright (C) 2010 CompuLab, Ltd.
 * Konstantin Sinyuk <kostyas@compulab.co.il>
 *
 * Based on Marvell DB-MV88AP510-BP Development Board Setup
 */

// Dependencies supplied by the surrounding kernel translation.

static mut cm_a510_ge00_data: mv643xx_eth_platform_data = mv643xx_eth_platform_data {
    phy_addr: MV643XX_ETH_PHY_ADDR_DEFAULT,
};

static mut cm_a510_sata_data: mv_sata_platform_data = mv_sata_platform_data {
    n_ports: 1,
};

/*
 * SPI Devices:
 * SPI0: 1M Flash Winbond w25q32bv
 */
static cm_a510_spi_flash_data: flash_platform_data = flash_platform_data {
    type_: "w25q32bv",
};

static mut cm_a510_spi_flash_info: [spi_board_info; 1] = [spi_board_info {
    modalias: "m25p80",
    platform_data: &cm_a510_spi_flash_data as *const flash_platform_data as *mut _,
    irq: -1,
    max_speed_hz: 20000000,
    bus_num: 0,
    chip_select: 0,
}];

unsafe extern "C" {
    fn machine_is_cm_a510() -> bool;
    fn dove_pcie_init(a: i32, b: i32);
    fn dove_init();
    fn dove_ge00_init(data: *mut mv643xx_eth_platform_data);
    fn dove_ehci0_init();
    fn dove_ehci1_init();
    fn dove_sata_init(data: *mut mv_sata_platform_data);
    fn dove_sdio0_init();
    fn dove_sdio1_init();
    fn dove_spi0_init();
    fn dove_spi1_init();
    fn dove_uart0_init();
    fn dove_uart1_init();
    fn dove_i2c_init();
    fn spi_register_board_info(info: *mut spi_board_info, n: usize) -> i32;
    fn dove_map_io();
    fn dove_init_early();
    fn dove_init_irq();
    fn dove_timer_init();
    fn dove_restart(mode: i32, cmd: *const u8);
}

unsafe extern "C" fn cm_a510_pci_init() -> i32 {
    if machine_is_cm_a510() {
        dove_pcie_init(1, 1);
    }

    0
}

// Equivalent of subsys_initcall(cm_a510_pci_init).

/* Board Init */
unsafe extern "C" fn cm_a510_init() {
    /*
     * Basic Dove setup. Needs to be called early.
     */
    dove_init();

    dove_ge00_init(&mut cm_a510_ge00_data);
    dove_ehci0_init();
    dove_ehci1_init();
    dove_sata_init(&mut cm_a510_sata_data);
    dove_sdio0_init();
    dove_sdio1_init();
    dove_spi0_init();
    dove_spi1_init();
    dove_uart0_init();
    dove_uart1_init();
    dove_i2c_init();
    spi_register_board_info(
        cm_a510_spi_flash_info.as_mut_ptr(),
        cm_a510_spi_flash_info.len(),
    );
}

// Equivalent of MACHINE_START(CM_A510, "Compulab CM-A510 Board") ... MACHINE_END.
// .atag_offset = 0x100, .nr_irqs = DOVE_NR_IRQS,
// .init_machine = cm_a510_init, .map_io = dove_map_io,
// .init_early = dove_init_early, .init_irq = dove_init_irq,
// .init_time = dove_timer_init, .restart = dove_restart.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
