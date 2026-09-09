/*
 * stmark2.c -- Support for Kernelspace AMCORE open board
 *
 * (C) Copyright 2026, Angelo Dureghello <angelo@kernel-space.org>
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by the kernel and architecture-specific headers.

/*
 * Partitioning of parallel NOR flash (39VF3201B)
 */
static mut stmark2_partitions: [mtd_partition; 3] = [
    mtd_partition {
        name: "U-Boot (1024K)",
        size: 0x100000,
        offset: 0x0,
    },
    mtd_partition {
        name: "Kernel+initramfs (7168K)",
        size: 0x700000,
        offset: MTDPART_OFS_APPEND,
    },
    mtd_partition {
        name: "Flash Free Space (8192K)",
        size: MTDPART_SIZ_FULL,
        offset: MTDPART_OFS_APPEND,
    },
];

static mut stmark2_spi_flash_data: flash_platform_data = flash_platform_data {
    name: "is25lp128",
    parts: unsafe { stmark2_partitions.as_ptr() },
    nr_parts: ARRAY_SIZE!(stmark2_partitions),
    type_: "is25lp128",
};

static mut stmark2_board_info: [spi_board_info; 1] = [spi_board_info {
    modalias: "m25p80",
    max_speed_hz: 5000000,
    bus_num: 0,
    chip_select: 1,
    platform_data: unsafe {
        &mut stmark2_spi_flash_data as *mut flash_platform_data as *mut core::ffi::c_void
    },
    mode: SPI_MODE_3,
}];

/* SPI controller data, SPI (0) */
static mut dspi_spi0_info: fsl_dspi_platform_data = fsl_dspi_platform_data {
    cs_num: 4,
    bus_num: 0,
    sck_cs_delay: 100,
    cs_sck_delay: 100,
};

static mut dspi_spi0_resource: [resource; 4] = [
    DEFINE_RES_MEM!(MCFDSPI_BASE0, 0x100),
    DEFINE_RES_IRQ!(MCF_IRQ_DSPI0),
    DEFINE_RES_DMA!(12),
    DEFINE_RES_DMA!(13),
];

static mut stmark2_dspi_mask: u64 = DMA_BIT_MASK!(32);

/* SPI controller, id = bus number */
static mut dspi_spi0_device: platform_device = platform_device {
    name: "fsl-dspi",
    id: 0,
    num_resources: ARRAY_SIZE!(dspi_spi0_resource),
    resource: unsafe { dspi_spi0_resource.as_mut_ptr() },
    dev: device {
        platform_data: unsafe {
            &mut dspi_spi0_info as *mut fsl_dspi_platform_data as *mut core::ffi::c_void
        },
        dma_mask: unsafe { &mut stmark2_dspi_mask },
        coherent_dma_mask: DMA_BIT_MASK!(32),
    },
};

static mut dac0_resource: resource = DEFINE_RES_MEM!(MCFDAC_BASE0, 0x100);

static mut dac0_device: platform_device = platform_device {
    name: "mcfdac",
    id: 0,
    num_resources: 1,
    resource: unsafe { &mut dac0_resource },
};

static mut dac1_resource: resource = DEFINE_RES_MEM!(MCFDAC_BASE1, 0x100);

static mut dac1_device: platform_device = platform_device {
    name: "mcfdac",
    id: 1,
    num_resources: 1,
    resource: unsafe { &mut dac1_resource },
};

static mut stmark2_devices: [*mut platform_device; 3] = unsafe {
    [&mut dspi_spi0_device, &mut dac0_device, &mut dac1_device]
};

/*
 * Note: proper pin-mux setup is mandatory for proper SPI functionality.
 */
unsafe fn init_stmark2() -> i32 {
    let mut val: u16;

    /* DSPI0, all pins as DSPI, and using CS1 */
    mcf_write8(0x80, MCFGPIO_PAR_DSPIOWL);
    mcf_write8(0xfc, MCFGPIO_PAR_DSPIOWH);

    /* Board gpio setup */
    mcf_write8(0x00, MCFGPIO_PAR_BE);
    mcf_write8(0x00, MCFGPIO_PAR_FBCTL);
    mcf_write8(0x00, MCFGPIO_PAR_CS);

    /* CAN pads */
    mcf_write8(0x50, MCFGPIO_PAR_CANI2C);

    val = mcf_read16(MCF_CCM_MISCCR2);
    val &= !(MCF_CCM_MISCCR2_ADC3_EN | MCF_CCM_MISCCR2_ADC7_EN);
    val |= MCF_CCM_MISCCR2_DAC0_SEL | MCF_CCM_MISCCR2_DAC1_SEL;
    mcf_write16(val, MCF_CCM_MISCCR2);

    platform_add_devices(stmark2_devices.as_mut_ptr(), ARRAY_SIZE!(stmark2_devices));

    spi_register_board_info(stmark2_board_info.as_mut_ptr(),
                            ARRAY_SIZE!(stmark2_board_info));

    0
}

device_initcall!(init_stmark2);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
