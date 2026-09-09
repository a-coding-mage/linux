/*
 * device.c  -- common ColdFire SoC device support
 *
 * (C) Copyright 2011, Greg Ungerer <gerg@uclinux.org>
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

/* Linux header dependencies are supplied by the surrounding translation unit. */

#[repr(C)]
struct mcf_platform_uart { mapbase: usize, irq: i32 }
#[repr(C)]
struct platform_device {
    name: *const u8,
    id: i32,
    num_resources: usize,
    resource: *mut resource,
    dev: platform_device_dev,
}
#[repr(C)]
struct platform_device_dev {
    dma_mask: *mut u64,
    coherent_dma_mask: u64,
    platform_data: *mut core::ffi::c_void,
}
#[repr(C)] struct resource { start: usize, end: usize, flags: u64, name: *const u8 }
#[repr(C)] struct fec_platform_data { phy: i32 }
#[repr(C)] struct mcfqspi_cs_control {
    setup: Option<unsafe extern "C" fn(*mut mcfqspi_cs_control) -> i32>,
    teardown: Option<unsafe extern "C" fn(*mut mcfqspi_cs_control)>,
    select: Option<unsafe extern "C" fn(*mut mcfqspi_cs_control, u8, bool)>,
    deselect: Option<unsafe extern "C" fn(*mut mcfqspi_cs_control, u8, bool)>,
}
#[repr(C)] struct mcfqspi_platform_data { bus_num: i32, num_chipselect: i32, cs_control: *mut mcfqspi_cs_control }
#[repr(C)] struct dma_slave_map { slave: *const u8, parameter: *const u8, filter_param: usize }
#[repr(C)] struct mcf_edma_platform_data { dma_channels: i32, slave_map: *const dma_slave_map, slavecnt: usize }
#[repr(C)] struct mcf_esdhc_platform_data { max_bus_width: i32, cd_type: i32 }
#[repr(C)] struct flexcan_platform_data { clk_src: i32, clock_frequency: u32 }

extern "C" {
    fn gpio_request(gpio: usize, label: *const u8) -> i32;
    fn gpio_direction_output(gpio: usize, value: i32) -> i32;
    fn gpio_free(gpio: usize);
    fn gpio_set_value(gpio: usize, value: bool);
    fn pr_debug(format: *const u8, ...);
    fn mcf_write8(value: u8, address: usize);
    fn mcf_mapirq2imr(irq: usize, mask: usize);
    fn platform_add_devices(devices: *mut *mut platform_device, count: usize) -> i32;
}

static mut mcf_uart_platform_data: [mcf_platform_uart; 3] = [
    mcf_platform_uart { mapbase: MCFUART_BASE0, irq: MCF_IRQ_UART0 },
    mcf_platform_uart { mapbase: MCFUART_BASE1, irq: MCF_IRQ_UART1 },
    mcf_platform_uart { mapbase: 0, irq: 0 },
];
static mut mcf_uart: platform_device = platform_device {
    name: b"mcfuart\0".as_ptr(), id: 0, num_resources: 0, resource: core::ptr::null_mut(),
    dev: platform_device_dev { dma_mask: core::ptr::null_mut(), coherent_dma_mask: 0, platform_data: core::ptr::null_mut() },
};

#[cfg(MCFFEC_BASE0)]
static mut mcf_fec0_resources: [resource; 4] = [
    resource { start: MCFFEC_BASE0, end: MCFFEC_BASE0 + MCFFEC_SIZE0 - 1, flags: IORESOURCE_MEM, name: core::ptr::null() },
    resource { start: MCF_IRQ_FECRX0, end: MCF_IRQ_FECRX0, flags: IORESOURCE_IRQ, name: core::ptr::null() },
    resource { start: MCF_IRQ_FECTX0, end: MCF_IRQ_FECTX0, flags: IORESOURCE_IRQ, name: core::ptr::null() },
    resource { start: MCF_IRQ_FECENTC0, end: MCF_IRQ_FECENTC0, flags: IORESOURCE_IRQ, name: core::ptr::null() },
];
#[cfg(MCFFEC_BASE0)]
static mut mcf_fec0: platform_device = platform_device {
    name: b"fec\0".as_ptr(), id: 0, num_resources: 4, resource: unsafe { mcf_fec0_resources.as_mut_ptr() },
    dev: platform_device_dev { dma_mask: core::ptr::null_mut(), coherent_dma_mask: 0x1_0000_0000, platform_data: core::ptr::null_mut() },
};

#[cfg(any(MCFQSPI_BASE, CONFIG_SPI_COLDFIRE_QSPI))]
static mut mcf_qspi_resources: [resource; 2] = [
    resource { start: MCFQSPI_BASE, end: MCFQSPI_BASE + MCFQSPI_SIZE - 1, flags: IORESOURCE_MEM, name: core::ptr::null() },
    resource { start: MCF_IRQ_QSPI, end: MCF_IRQ_QSPI, flags: IORESOURCE_IRQ, name: core::ptr::null() },
];

#[cfg(any(MCFQSPI_BASE, CONFIG_SPI_COLDFIRE_QSPI))]
unsafe extern "C" fn mcf_cs_setup(_: *mut mcfqspi_cs_control) -> i32 {
    let mut status = gpio_request(MCFQSPI_CS0, b"MCFQSPI_CS0\0".as_ptr());
    if status != 0 { pr_debug(b"gpio_request for MCFQSPI_CS0 failed\n\0".as_ptr()); return status; }
    status = gpio_direction_output(MCFQSPI_CS0, 1);
    if status != 0 { pr_debug(b"gpio_direction_output for MCFQSPI_CS0 failed\n\0".as_ptr()); gpio_free(MCFQSPI_CS0); return status; }
    status = gpio_request(MCFQSPI_CS1, b"MCFQSPI_CS1\0".as_ptr());
    if status != 0 { gpio_free(MCFQSPI_CS0); return status; }
    status = gpio_direction_output(MCFQSPI_CS1, 1);
    if status != 0 { gpio_free(MCFQSPI_CS1); gpio_free(MCFQSPI_CS0); return status; }
    status = gpio_request(MCFQSPI_CS2, b"MCFQSPI_CS2\0".as_ptr());
    if status != 0 { gpio_free(MCFQSPI_CS1); gpio_free(MCFQSPI_CS0); return status; }
    status = gpio_direction_output(MCFQSPI_CS2, 1);
    if status != 0 { gpio_free(MCFQSPI_CS2); gpio_free(MCFQSPI_CS1); gpio_free(MCFQSPI_CS0); return status; }
    status
}
#[cfg(any(MCFQSPI_BASE, CONFIG_SPI_COLDFIRE_QSPI))]
unsafe extern "C" fn mcf_cs_teardown(_: *mut mcfqspi_cs_control) { gpio_free(MCFQSPI_CS2); gpio_free(MCFQSPI_CS1); gpio_free(MCFQSPI_CS0); }
#[cfg(any(MCFQSPI_BASE, CONFIG_SPI_COLDFIRE_QSPI))]
unsafe extern "C" fn mcf_cs_select(_: *mut mcfqspi_cs_control, chip_select: u8, cs_high: bool) {
    match chip_select { 0 => gpio_set_value(MCFQSPI_CS0, cs_high), 1 => gpio_set_value(MCFQSPI_CS1, cs_high), 2 => gpio_set_value(MCFQSPI_CS2, cs_high), _ => {} }
}
#[cfg(any(MCFQSPI_BASE, CONFIG_SPI_COLDFIRE_QSPI))]
unsafe extern "C" fn mcf_cs_deselect(_: *mut mcfqspi_cs_control, chip_select: u8, cs_high: bool) {
    match chip_select { 0 => gpio_set_value(MCFQSPI_CS0, !cs_high), 1 => gpio_set_value(MCFQSPI_CS1, !cs_high), 2 => gpio_set_value(MCFQSPI_CS2, !cs_high), _ => {} }
}
#[cfg(any(MCFQSPI_BASE, CONFIG_SPI_COLDFIRE_QSPI))]
static mut mcf_cs_control: mcfqspi_cs_control = mcfqspi_cs_control { setup: Some(mcf_cs_setup), teardown: Some(mcf_cs_teardown), select: Some(mcf_cs_select), deselect: Some(mcf_cs_deselect) };
#[cfg(any(MCFQSPI_BASE, CONFIG_SPI_COLDFIRE_QSPI))]
static mut mcf_qspi_data: mcfqspi_platform_data = mcfqspi_platform_data { bus_num: 0, num_chipselect: 4, cs_control: unsafe { &mut mcf_cs_control } };
#[cfg(any(MCFQSPI_BASE, CONFIG_SPI_COLDFIRE_QSPI))]
static mut mcf_qspi: platform_device = platform_device { name: b"mcfqspi\0".as_ptr(), id: 0, num_resources: 2, resource: unsafe { mcf_qspi_resources.as_mut_ptr() }, dev: platform_device_dev { dma_mask: core::ptr::null_mut(), coherent_dma_mask: 0, platform_data: unsafe { &mut mcf_qspi_data as *mut _ as *mut core::ffi::c_void } } };

static mut mcf_devices: [*mut platform_device; 2] = [unsafe { &mut mcf_uart }, core::ptr::null_mut()];

unsafe extern "C" fn mcf_uart_set_irq() {
    #[cfg(MCFUART_UIVR)] {
        mcf_write8(MCFSIM_ICR_LEVEL6 | MCFSIM_ICR_PRI1, MCFSIM_UART1ICR);
        mcf_write8(MCF_IRQ_UART0 as u8, MCFUART_BASE0 + MCFUART_UIVR);
        mcf_mapirq2imr(MCF_IRQ_UART0, MCFINTC_UART0);
        mcf_write8(MCFSIM_ICR_LEVEL6 | MCFSIM_ICR_PRI2, MCFSIM_UART2ICR);
        mcf_write8(MCF_IRQ_UART1 as u8, MCFUART_BASE1 + MCFUART_UIVR);
        mcf_mapirq2imr(MCF_IRQ_UART1, MCFINTC_UART1);
    }
}

unsafe extern "C" fn mcf_init_devices() -> i32 {
    mcf_uart_set_irq();
    platform_add_devices(mcf_devices.as_mut_ptr(), mcf_devices.len());
    0
}

/* Additional conditionally compiled platform data from the original source. */
#[cfg(CONFIG_I2C_IMX)]
static mut mcf_i2c_resources: [resource; 2] = [
    resource { start: MCFI2C_BASE0, end: MCFI2C_BASE0 + MCFI2C_SIZE0 - 1, flags: IORESOURCE_MEM, name: core::ptr::null() },
    resource { start: MCF_IRQ_I2C0, end: MCF_IRQ_I2C0, flags: IORESOURCE_IRQ, name: core::ptr::null() },
];
#[cfg(CONFIG_I2C_IMX)]
static mut mcf_i2c0: platform_device = platform_device { name: b"imx1-i2c\0".as_ptr(), id: 0, num_resources: 2, resource: unsafe { mcf_i2c_resources.as_mut_ptr() }, dev: platform_device_dev { dma_mask: core::ptr::null_mut(), coherent_dma_mask: 0, platform_data: core::ptr::null_mut() } };

#[cfg(MCFEDMA_BASE)]
static mcf_edma_map: [dma_slave_map; 16] = [
    dma_slave_map { slave: b"dreq0\0".as_ptr(), parameter: b"rx-tx\0".as_ptr(), filter_param: MCF_EDMA_FILTER_PARAM(0) },
    dma_slave_map { slave: b"dreq1\0".as_ptr(), parameter: b"rx-tx\0".as_ptr(), filter_param: MCF_EDMA_FILTER_PARAM(1) },
    dma_slave_map { slave: b"uart.0\0".as_ptr(), parameter: b"rx\0".as_ptr(), filter_param: MCF_EDMA_FILTER_PARAM(2) },
    dma_slave_map { slave: b"uart.0\0".as_ptr(), parameter: b"tx\0".as_ptr(), filter_param: MCF_EDMA_FILTER_PARAM(3) },
    dma_slave_map { slave: b"uart.1\0".as_ptr(), parameter: b"rx\0".as_ptr(), filter_param: MCF_EDMA_FILTER_PARAM(4) },
    dma_slave_map { slave: b"uart.1\0".as_ptr(), parameter: b"tx\0".as_ptr(), filter_param: MCF_EDMA_FILTER_PARAM(5) },
    dma_slave_map { slave: b"uart.2\0".as_ptr(), parameter: b"rx\0".as_ptr(), filter_param: MCF_EDMA_FILTER_PARAM(6) },
    dma_slave_map { slave: b"uart.2\0".as_ptr(), parameter: b"tx\0".as_ptr(), filter_param: MCF_EDMA_FILTER_PARAM(7) },
    dma_slave_map { slave: b"timer0\0".as_ptr(), parameter: b"rx-tx\0".as_ptr(), filter_param: MCF_EDMA_FILTER_PARAM(8) },
    dma_slave_map { slave: b"timer1\0".as_ptr(), parameter: b"rx-tx\0".as_ptr(), filter_param: MCF_EDMA_FILTER_PARAM(9) },
    dma_slave_map { slave: b"timer2\0".as_ptr(), parameter: b"rx-tx\0".as_ptr(), filter_param: MCF_EDMA_FILTER_PARAM(10) },
    dma_slave_map { slave: b"timer3\0".as_ptr(), parameter: b"rx-tx\0".as_ptr(), filter_param: MCF_EDMA_FILTER_PARAM(11) },
    dma_slave_map { slave: b"fsl-dspi.0\0".as_ptr(), parameter: b"rx\0".as_ptr(), filter_param: MCF_EDMA_FILTER_PARAM(12) },
    dma_slave_map { slave: b"fsl-dspi.0\0".as_ptr(), parameter: b"tx\0".as_ptr(), filter_param: MCF_EDMA_FILTER_PARAM(13) },
    dma_slave_map { slave: b"fsl-dspi.1\0".as_ptr(), parameter: b"rx\0".as_ptr(), filter_param: MCF_EDMA_FILTER_PARAM(14) },
    dma_slave_map { slave: b"fsl-dspi.1\0".as_ptr(), parameter: b"tx\0".as_ptr(), filter_param: MCF_EDMA_FILTER_PARAM(15) },
];
#[cfg(MCFEDMA_BASE)]
static mut mcf_edma_data: mcf_edma_platform_data = mcf_edma_platform_data { dma_channels: 64, slave_map: mcf_edma_map.as_ptr(), slavecnt: 16 };
#[cfg(MCFSDHC_BASE)]
static mut mcf_esdhc_data: mcf_esdhc_platform_data = mcf_esdhc_platform_data { max_bus_width: 4, cd_type: ESDHC_CD_NONE };
#[cfg(MCFFLEXCAN_SIZE)]
static mut mcf5441x_flexcan_info: flexcan_platform_data = flexcan_platform_data { clk_src: 1, clock_frequency: 120000000 };

/* arch_initcall(mcf_init_devices); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
