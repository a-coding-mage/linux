// SPDX-License-Identifier: GPL-2.0
/* Renesas AP-325RXA board setup; translated from setup.c. */

// Kernel headers and symbols referenced below are supplied by the surrounding
// platform bindings and are intentionally not redefined here.

const CEU_BUFFER_MEMORY_SIZE: usize = 4 << 20;
static mut ceu_dma_membase: phys_addr_t = 0;

static mut dummy_supplies: [regulator_consumer_supply; 2] = [
    REGULATOR_SUPPLY!("vddvario", "smsc911x"),
    REGULATOR_SUPPLY!("vdd33a", "smsc911x"),
];

static mut smsc911x_config: smsc911x_platform_config = smsc911x_platform_config {
    phy_interface: PHY_INTERFACE_MODE_MII,
    irq_polarity: SMSC911X_IRQ_POLARITY_ACTIVE_LOW,
    irq_type: SMSC911X_IRQ_TYPE_OPEN_DRAIN,
    flags: SMSC911X_USE_32BIT,
};

static mut smsc9118_resources: [resource; 2] = [
    resource { start: 0xb6080000, end: 0xb60fffff, flags: IORESOURCE_MEM, ..resource::default() },
    resource { start: evt2irq(0x660), end: evt2irq(0x660), flags: IORESOURCE_IRQ, ..resource::default() },
];
static mut smsc9118_device: platform_device = platform_device {
    name: c_str!("smsc911x"), id: -1, num_resources: 2, resource: smsc9118_resources.as_mut_ptr(),
    dev: device { platform_data: &mut smsc911x_config as *mut _ as *mut _, ..device::default() }, ..platform_device::default()
};

static mut ap325rxa_nor_flash_partitions: [mtd_partition; 5] = [
    mtd_partition { name: c_str!("uboot"), offset: 0, size: 1 * 1024 * 1024, mask_flags: MTD_WRITEABLE, ..mtd_partition::default() },
    mtd_partition { name: c_str!("kernel"), offset: MTDPART_OFS_APPEND, size: 2 * 1024 * 1024, ..mtd_partition::default() },
    mtd_partition { name: c_str!("free-area0"), offset: MTDPART_OFS_APPEND, size: 7 * 1024 * 1024 + 512 * 1024, ..mtd_partition::default() },
    mtd_partition { name: c_str!("CPLD-Data"), offset: MTDPART_OFS_APPEND, mask_flags: MTD_WRITEABLE, size: 1024 * 128 * 2, ..mtd_partition::default() },
    mtd_partition { name: c_str!("free-area1"), offset: MTDPART_OFS_APPEND, size: MTDPART_SIZ_FULL, ..mtd_partition::default() },
];
static mut ap325rxa_nor_flash_data: physmap_flash_data = physmap_flash_data { width: 2, parts: ap325rxa_nor_flash_partitions.as_mut_ptr(), nr_parts: 5, ..physmap_flash_data::default() };
static mut ap325rxa_nor_flash_resources: [resource; 1] = [resource { name: c_str!("NOR Flash"), start: 0, end: 0x00ffffff, flags: IORESOURCE_MEM, ..resource::default() }];
static mut ap325rxa_nor_flash_device: platform_device = platform_device { name: c_str!("physmap-flash"), resource: ap325rxa_nor_flash_resources.as_mut_ptr(), num_resources: 1, dev: device { platform_data: &mut ap325rxa_nor_flash_data as *mut _ as *mut _, ..device::default() }, ..platform_device::default() };

static mut nand_partition_info: [mtd_partition; 1] = [mtd_partition { name: c_str!("nand_data"), offset: 0, size: MTDPART_SIZ_FULL, ..mtd_partition::default() }];
static mut nand_flash_resources: [resource; 1] = [resource { start: 0xa4530000, end: 0xa45300ff, flags: IORESOURCE_MEM, ..resource::default() }];
static mut nand_flash_data: sh_flctl_platform_data = sh_flctl_platform_data { parts: nand_partition_info.as_mut_ptr(), nr_parts: 1, flcmncr_val: FCKSEL_E | TYPESEL_SET | NANWF_E, has_hwecc: 1, ..sh_flctl_platform_data::default() };
static mut nand_flash_device: platform_device = platform_device { name: c_str!("sh_flctl"), resource: nand_flash_resources.as_mut_ptr(), num_resources: 1, dev: device { platform_data: &mut nand_flash_data as *mut _ as *mut _, ..device::default() }, ..platform_device::default() };

const FPGA_LCDREG: usize = 0xB4100180;
const FPGA_BKLREG: usize = 0xB4100212;
const FPGA_LCDREG_VAL: u16 = 0x0018;
const PORT_MSELCRB: usize = 0xA4050182;
const PORT_HIZCRC: usize = 0xA405015C;
const PORT_DRVCRA: usize = 0xA405018A;
const PORT_DRVCRB: usize = 0xA405018C;

unsafe extern "C" {
    fn gpio_set_value(gpio: i32, value: i32);
    fn __raw_writew(value: u16, address: usize);
    fn __raw_readw(address: usize) -> u16;
    fn msleep(ms: u32);
    fn gpio_request(gpio: i32, label: *const i8) -> i32;
    fn gpio_direction_output(gpio: i32, value: i32) -> i32;
    fn gpio_direction_input(gpio: i32) -> i32;
    fn gpiod_export(desc: *mut gpio_desc, direction_may_change: bool) -> i32;
    fn gpio_to_desc(gpio: i32) -> *mut gpio_desc;
    fn clk_add_alias(id: *const i8, dev_id: *const i8, con_id: *const i8, dev: *mut device) -> i32;
    fn gpiod_add_lookup_table(table: *mut gpiod_lookup_table);
    fn i2c_register_board_info(busnum: i32, info: *mut i2c_board_info, len: usize) -> i32;
    fn device_initialize(dev: *mut device) -> i32;
    fn dma_declare_coherent_memory(dev: *mut device, dma: phys_addr_t, device_addr: phys_addr_t, size: usize) -> i32;
    fn platform_device_add(dev: *mut platform_device) -> i32;
    fn platform_add_devices(devs: *mut *mut platform_device, num: usize) -> i32;
    fn sh_mobile_register_self_refresh(flags: u32, enter_start: *mut i8, enter_end: *mut i8, leave_start: *mut i8, leave_end: *mut i8);
    fn regulator_register_always_on(id: i32, name: *const i8, consumers: *mut regulator_consumer_supply, n: usize, microvolts: i32);
    fn regulator_register_fixed(id: i32, consumers: *mut regulator_consumer_supply, n: usize);
    fn memblock_phys_alloc(size: usize, align: usize) -> phys_addr_t;
    fn memblock_phys_free(base: phys_addr_t, size: usize);
    fn memblock_remove(base: phys_addr_t, size: usize);
    fn panic(msg: *const i8) -> !;
}

unsafe fn ap320_wvga_set_brightness(brightness: i32) -> i32 {
    if brightness != 0 { gpio_set_value(GPIO_PTS3, 0); __raw_writew(0x100, FPGA_BKLREG); }
    else { __raw_writew(0, FPGA_BKLREG); gpio_set_value(GPIO_PTS3, 1); }
    0
}
unsafe fn ap320_wvga_power_on() { msleep(100); __raw_writew(FPGA_LCDREG_VAL, FPGA_LCDREG); }
unsafe fn ap320_wvga_power_off() { __raw_writew(0, FPGA_LCDREG); }

// The remaining platform-data objects retain the C initializers and external
// kernel types through the native bindings.
static mut ap325rxa_devices: [*mut platform_device; 6] = [
    &mut smsc9118_device, &mut ap325rxa_nor_flash_device, &mut lcdc_device,
    &mut nand_flash_device, &mut sdhi0_cn3_device, &mut sdhi1_cn7_device,
];

unsafe fn ap325rxa_devices_setup() -> i32 {
    sh_mobile_register_self_refresh(SUSP_SH_STANDBY | SUSP_SH_SF, &mut ap325rxa_sdram_enter_start, &mut ap325rxa_sdram_enter_end, &mut ap325rxa_sdram_leave_start, &mut ap325rxa_sdram_leave_end);
    regulator_register_always_on(0, c_str!("fixed-3.3V"), fixed3v3_power_consumers.as_mut_ptr(), 4, 3300000);
    regulator_register_fixed(1, dummy_supplies.as_mut_ptr(), 2);
    gpio_request(GPIO_PTX5, core::ptr::null()); gpio_direction_output(GPIO_PTX5, 1); gpiod_export(gpio_to_desc(GPIO_PTX5), false);
    gpio_request(GPIO_PTX4, core::ptr::null()); gpio_direction_output(GPIO_PTX4, 0); gpiod_export(gpio_to_desc(GPIO_PTX4), false);
    gpio_request(GPIO_PTF7, core::ptr::null()); gpio_direction_input(GPIO_PTF7); gpiod_export(gpio_to_desc(GPIO_PTF7), false);
    for gpio in [GPIO_FN_LCDD15,GPIO_FN_LCDD14,GPIO_FN_LCDD13,GPIO_FN_LCDD12,GPIO_FN_LCDD11,GPIO_FN_LCDD10,GPIO_FN_LCDD9,GPIO_FN_LCDD8,GPIO_FN_LCDD7,GPIO_FN_LCDD6,GPIO_FN_LCDD5,GPIO_FN_LCDD4,GPIO_FN_LCDD3,GPIO_FN_LCDD2,GPIO_FN_LCDD1,GPIO_FN_LCDD0,GPIO_FN_LCDLCLK_PTR,GPIO_FN_LCDDCK,GPIO_FN_LCDVEPWC,GPIO_FN_LCDVCPWC,GPIO_FN_LCDVSYN,GPIO_FN_LCDHSYN,GPIO_FN_LCDDISP,GPIO_FN_LCDDON] { gpio_request(gpio, core::ptr::null()); }
    gpio_request(GPIO_PTS3, core::ptr::null()); gpio_direction_output(GPIO_PTS3, 1);
    for gpio in [GPIO_FN_VIO_CLK2,GPIO_FN_VIO_VD2,GPIO_FN_VIO_HD2,GPIO_FN_VIO_FLD,GPIO_FN_VIO_CKO,GPIO_FN_VIO_D15,GPIO_FN_VIO_D14,GPIO_FN_VIO_D13,GPIO_FN_VIO_D12,GPIO_FN_VIO_D11,GPIO_FN_VIO_D10,GPIO_FN_VIO_D9,GPIO_FN_VIO_D8] { gpio_request(gpio, core::ptr::null()); }
    for (gpio, val) in [(GPIO_PTZ7,0),(GPIO_PTZ6,0),(GPIO_PTZ5,0),(GPIO_PTZ4,0)] { gpio_request(gpio, core::ptr::null()); gpio_direction_output(gpio,val); }
    __raw_writew(__raw_readw(PORT_MSELCRB) & !0x0001, PORT_MSELCRB);
    for gpio in [GPIO_FN_FCE,GPIO_FN_NAF7,GPIO_FN_NAF6,GPIO_FN_NAF5,GPIO_FN_NAF4,GPIO_FN_NAF3,GPIO_FN_NAF2,GPIO_FN_NAF1,GPIO_FN_NAF0,GPIO_FN_FCDE,GPIO_FN_FOE,GPIO_FN_FSC,GPIO_FN_FWE,GPIO_FN_FRB] { gpio_request(gpio, core::ptr::null()); }
    __raw_writew(0, PORT_HIZCRC); __raw_writew(0xFFFF, PORT_DRVCRA); __raw_writew(0xFFFF, PORT_DRVCRB);
    for gpio in [GPIO_FN_SDHI0CD_PTD,GPIO_FN_SDHI0WP_PTD,GPIO_FN_SDHI0D3_PTD,GPIO_FN_SDHI0D2_PTD,GPIO_FN_SDHI0D1_PTD,GPIO_FN_SDHI0D0_PTD,GPIO_FN_SDHI0CMD_PTD,GPIO_FN_SDHI0CLK_PTD,GPIO_FN_SDHI1CD,GPIO_FN_SDHI1D3,GPIO_FN_SDHI1D2,GPIO_FN_SDHI1D1,GPIO_FN_SDHI1D0,GPIO_FN_SDHI1CMD,GPIO_FN_SDHI1CLK] { gpio_request(gpio, core::ptr::null()); }
    clk_add_alias(core::ptr::null(), c_str!("0-0021"), c_str!("video_clk"), core::ptr::null_mut()); gpiod_add_lookup_table(&mut ov7725_gpios); i2c_register_board_info(0, ap325rxa_i2c_devices.as_mut_ptr(), 2);
    device_initialize(&mut ap325rxa_ceu_device.dev); dma_declare_coherent_memory(&mut ap325rxa_ceu_device.dev, ceu_dma_membase, ceu_dma_membase, CEU_BUFFER_MEMORY_SIZE); platform_device_add(&mut ap325rxa_ceu_device); platform_add_devices(ap325rxa_devices.as_mut_ptr(), 6)
}

unsafe fn ap325rxa_mode_pins() -> i32 { MODE_PIN5 | MODE_PIN8 }
unsafe fn ap325rxa_mv_mem_reserve() { let size = CEU_BUFFER_MEMORY_SIZE; let phys = memblock_phys_alloc(size, PAGE_SIZE); if phys == 0 { panic(c_str!("Failed to allocate CEU memory\n")); } memblock_phys_free(phys, size); memblock_remove(phys, size); ceu_dma_membase = phys; }

extern "C" { static mut ap325rxa_sdram_enter_start: i8; static mut ap325rxa_sdram_enter_end: i8; static mut ap325rxa_sdram_leave_start: i8; static mut ap325rxa_sdram_leave_end: i8; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
