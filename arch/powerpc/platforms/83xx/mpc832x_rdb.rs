// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * arch/powerpc/platforms/83xx/mpc832x_rdb.c
 *
 * Copyright (C) Freescale Semiconductor, Inc. 2007. All rights reserved.
 *
 * Description:
 * MPC832x RDB board specific routines.
 * This file is based on mpc832x_mds.c and mpc8313_rdb.c
 * Author: Michael Barkowski <michael.barkowski@freescale.com>
 */

// Linux and platform dependencies are supplied by the surrounding translation.

#[cfg(feature = "CONFIG_QUICC_ENGINE")]
unsafe fn of_fsl_spi_probe(
    type_: *mut core::ffi::c_char,
    compatible: *mut core::ffi::c_char,
    sysclk: u32,
    board_infos: *mut spi_board_info,
    num_board_infos: u32,
    cs_control: Option<unsafe extern "C" fn(*mut spi_device, bool)>,
) -> i32 {
    let mut np: *mut device_node;
    let mut i: u32 = 0;

    // for_each_compatible_node(np, type, compatible)
    for_each_compatible_node!(np, type_, compatible) {
        let mut ret: i32;
        let mut j: u32;
        let mut prop: *const core::ffi::c_void;
        let mut res: [resource; 2] = [core::mem::zeroed(), core::mem::zeroed()];
        let mut pdev: *mut platform_device;
        let mut pdata = fsl_spi_platform_data { cs_control, ..core::mem::zeroed() };

        pdata.sysclk = sysclk;

        prop = of_get_property(np, c"reg".as_ptr(), core::ptr::null_mut());
        if prop.is_null() { goto_err!(err); }
        pdata.bus_num = *(prop as *const u32);

        prop = of_get_property(np, c"cell-index".as_ptr(), core::ptr::null_mut());
        if !prop.is_null() { i = *(prop as *const u32); }

        prop = of_get_property(np, c"mode".as_ptr(), core::ptr::null_mut());
        if !prop.is_null() && strcmp(prop as *const i8, c"cpu-qe".as_ptr()) == 0 {
            pdata.flags = SPI_QE_CPU_MODE;
        }

        j = 0;
        while j < num_board_infos {
            if (*board_infos.add(j as usize)).bus_num == pdata.bus_num {
                pdata.max_chipselect += 1;
            }
            j += 1;
        }
        if pdata.max_chipselect == 0 { continue; }

        ret = of_address_to_resource(np, 0, res.as_mut_ptr());
        if ret != 0 { goto_err!(err); }
        ret = of_irq_to_resource(np, 0, res.as_mut_ptr().add(1));
        if ret <= 0 { goto_err!(err); }

        pdev = platform_device_alloc(c"mpc83xx_spi".as_ptr(), i);
        if pdev.is_null() { goto_err!(err); }
        ret = platform_device_add_data(pdev, &pdata as *const _ as *const core::ffi::c_void,
                                       core::mem::size_of_val(&pdata));
        if ret != 0 { platform_device_put(pdev); goto_err!(err); }
        ret = platform_device_add_resources(pdev, res.as_ptr(), 2);
        if ret != 0 { platform_device_put(pdev); goto_err!(err); }
        ret = platform_device_add(pdev);
        if ret != 0 { platform_device_put(pdev); goto_err!(err); }
        i += 1;
        continue;
        err: pr_err!(c"%pOF: registration failed\n", np);
        i += 1;
    }
    i as i32
}

#[cfg(feature = "CONFIG_QUICC_ENGINE")]
unsafe fn fsl_spi_init(board_infos: *mut spi_board_info, num_board_infos: u32,
                       cs_control: Option<unsafe extern "C" fn(*mut spi_device, bool)>) -> i32 {
    let mut sysclk: u32 = u32::MAX;
    let ret: i32;
    sysclk = get_brgfreq();
    if sysclk == u32::MAX {
        sysclk = fsl_get_sys_freq();
        if sysclk == u32::MAX { return -19; }
    }
    ret = of_fsl_spi_probe(core::ptr::null_mut(), c"fsl,spi".as_ptr() as *mut _, sysclk,
                           board_infos, num_board_infos, cs_control);
    if ret == 0 {
        of_fsl_spi_probe(c"spi".as_ptr() as *mut _, c"fsl_spi".as_ptr() as *mut _, sysclk,
                         board_infos, num_board_infos, cs_control);
    }
    spi_register_board_info(board_infos, num_board_infos)
}

#[cfg(feature = "CONFIG_QUICC_ENGINE")]
unsafe extern "C" fn mpc83xx_spi_cs_control(spi: *mut spi_device, on: bool) {
    pr_debug!(c"%s %d %d\n", c"mpc83xx_spi_cs_control", spi_get_chipselect(spi, 0), on);
    par_io_data_set(3, 13, on);
}

#[cfg(feature = "CONFIG_QUICC_ENGINE")]
static mut mpc832x_mmc_pdata: mmc_spi_platform_data = mmc_spi_platform_data { ocr_mask: MMC_VDD_33_34 };
#[cfg(feature = "CONFIG_QUICC_ENGINE")]
static mut mpc832x_spi_boardinfo: spi_board_info = spi_board_info {
    bus_num: 0x4c0, chip_select: 0, max_speed_hz: 50000000,
    modalias: *b"mmc_spi\0", platform_data: core::ptr::addr_of_mut!(mpc832x_mmc_pdata) as *mut _,
};

#[cfg(feature = "CONFIG_QUICC_ENGINE")]
unsafe fn mpc832x_spi_init() -> i32 {
    par_io_config_pin(3, 0, 3, 0, 1, 0); // SPI1 MOSI, I/O
    par_io_config_pin(3, 1, 3, 0, 1, 0); // SPI1 MISO, I/O
    par_io_config_pin(3, 2, 3, 0, 1, 0); // SPI1 CLK, I/O
    par_io_config_pin(3, 3, 2, 0, 1, 0); // SPI1 SEL, I
    par_io_config_pin(3, 13, 1, 0, 0, 0); // !SD_CS, O
    par_io_config_pin(3, 14, 2, 0, 0, 0); // SD_INSERT, I
    par_io_config_pin(3, 15, 2, 0, 0, 0); // SD_PROTECT,I
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), c"mmc-spi-slot".as_ptr());
    of_node_put(np);
    if !np.is_null() { return 0; }
    fsl_spi_init(core::ptr::addr_of_mut!(mpc832x_spi_boardinfo), 1, Some(mpc83xx_spi_cs_control))
}

unsafe fn mpc832x_rdb_setup_arch() {
    mpc83xx_setup_arch();
    #[cfg(feature = "CONFIG_QUICC_ENGINE")]
    {
        let mut np = of_find_node_by_name(core::ptr::null_mut(), c"par_io".as_ptr());
        if !np.is_null() {
            par_io_init(np); of_node_put(np);
            for_each_node_by_name!(np, c"ucc".as_ptr(), { par_io_of_config(np); });
        }
    }
}

machine_device_initcall!(mpc832x_rdb, mpc83xx_declare_of_platform_devices);
define_machine!(mpc832x_rdb {
    name: c"MPC832x RDB", compatible: c"MPC832xRDB",
    setup_arch: mpc832x_rdb_setup_arch, discover_phbs: mpc83xx_setup_pci,
    init_IRQ: mpc83xx_ipic_init_IRQ, get_irq: ipic_get_irq, restart: mpc83xx_restart,
    time_init: mpc83xx_time_init, progress: udbg_progress,
});

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
