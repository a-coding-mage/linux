// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/arch/arm/mach-sa1100/generic.c
 *
 * Author: Nicolas Pitre
 *
 * Code common to all SA11x0 machines.
 */

// C header dependencies are supplied by the surrounding kernel translation.

const NR_FREQS: usize = 16;

/* This table is setup for a 3.6864MHz Crystal. */
#[repr(C)]
pub struct CpufreqFrequencyTable {
    pub frequency: u32,
}

#[no_mangle]
pub static mut sa11x0_freq_table: [CpufreqFrequencyTable; NR_FREQS + 1] = [
    CpufreqFrequencyTable { frequency: 59000 },
    CpufreqFrequencyTable { frequency: 73700 },
    CpufreqFrequencyTable { frequency: 88500 },
    CpufreqFrequencyTable { frequency: 103200 },
    CpufreqFrequencyTable { frequency: 118000 },
    CpufreqFrequencyTable { frequency: 132700 },
    CpufreqFrequencyTable { frequency: 147500 },
    CpufreqFrequencyTable { frequency: 162200 },
    CpufreqFrequencyTable { frequency: 176900 },
    CpufreqFrequencyTable { frequency: 191700 },
    CpufreqFrequencyTable { frequency: 206400 },
    CpufreqFrequencyTable { frequency: 221200 },
    CpufreqFrequencyTable { frequency: 235900 },
    CpufreqFrequencyTable { frequency: 250700 },
    CpufreqFrequencyTable { frequency: 265400 },
    CpufreqFrequencyTable { frequency: 280200 },
    CpufreqFrequencyTable { frequency: CPUFREQ_TABLE_END },
];

pub unsafe fn sa11x0_getspeed(cpu: u32) -> u32 {
    if cpu != 0 { return 0; }
    sa11x0_freq_table[(PPCR & 0xf) as usize].frequency
}

/* Default power-off for SA1100 */
unsafe fn sa1100_power_off() {
    mdelay(100);
    local_irq_disable();
    /* disable internal oscillator, float CS lines */
    PCFR = PCFR_OPDE | PCFR_FP | PCFR_FS;
    /* enable wake-up on GPIO0 (Assabet...) */
    PWER = 1;
    GFER = 1;
    GRER = 1;
    /* set scratchpad to zero, just in case it is used as a restart address by the bootloader. */
    PSPR = 0;
    /* enter sleep mode */
    PMCR = PMCR_SF;
}

pub unsafe fn sa11x0_restart(mode: RebootMode, _cmd: *const i8) {
    clear_reset_status(RESET_STATUS_ALL);
    if mode == REBOOT_SOFT {
        /* Jump into ROM at address 0 */
        soft_restart(0);
    } else {
        /* Use on-chip reset capability */
        RSRR = RSRR_SWR;
    }
}

unsafe fn sa11x0_register_device(dev: *mut PlatformDevice, data: *mut core::ffi::c_void) {
    (*dev).dev.platform_data = data;
    let err = platform_device_register(dev);
    if err != 0 {
        printk(KERN_ERR, b"Unable to register device %s: %d\0", (*dev).name, err);
    }
}

// Resource/device declarations retain the C layout and initialization values.
extern "C" {
    static mut sa11x0udc_device: PlatformDevice;
    static mut sa11x0uart1_device: PlatformDevice;
    static mut sa11x0uart3_device: PlatformDevice;
    static mut sa11x0ssp_device: PlatformDevice;
    static mut sa11x0rtc_device: PlatformDevice;
    static mut sa11x0dma_device: PlatformDevice;
    static mut sa11x0mcp_device: PlatformDevice;
    static mut sa11x0fb_device: PlatformDevice;
    static mut sa11x0mtd_device: PlatformDevice;
    static mut standard_io_desc: [MapDesc; 4];
    static mut irq_resource: Resource;
    static mut iomem_resource: Resource;
    static mut sa11x0_devices: [*mut PlatformDevice; 6];
}

pub unsafe fn sa11x0_ppc_configure_mcp() {
    PPDR &= !PPC_RXD4;
    PPDR |= PPC_TXD4 | PPC_SCLK | PPC_SFRM;
    PSDR |= PPC_RXD4;
    PSDR &= !(PPC_TXD4 | PPC_SCLK | PPC_SFRM);
    PPSR &= !(PPC_TXD4 | PPC_SCLK | PPC_SFRM);
}

pub unsafe fn sa11x0_register_mcp(data: *mut McpPlatData) {
    sa11x0_register_device(&mut sa11x0mcp_device, data.cast());
}

pub unsafe fn sa11x0_register_lcd(info: *mut Sa1100fbMachInfo) {
    sa11x0_register_device(&mut sa11x0fb_device, info.cast());
}

pub unsafe fn sa11x0_register_pcmcia(socket: i32, table: *mut GpiodLookupTable) {
    if !table.is_null() { gpiod_add_lookup_table(table); }
    platform_device_register_simple(b"sa11x0-pcmcia\0".as_ptr().cast(), socket, core::ptr::null_mut(), 0);
}

pub unsafe fn sa11x0_register_mtd(flash: *mut FlashPlatformData, res: *mut Resource, nr: i32) {
    (*flash).name = b"sa1100\0".as_ptr().cast();
    sa11x0mtd_device.resource = res;
    sa11x0mtd_device.num_resources = nr as u32;
    sa11x0_register_device(&mut sa11x0mtd_device, flash.cast());
}

pub unsafe fn sa1100_init() -> i32 {
    let mut wdt_res = DEFINE_RES_MEM(0x90000000, 0x20);
    register_platform_power_off(Some(sa1100_power_off));
    regulator_has_full_constraints();
    platform_device_register_simple(b"sa1100_wdt\0".as_ptr().cast(), -1, &mut wdt_res, 1);
    platform_add_devices(sa11x0_devices.as_mut_ptr(), sa11x0_devices.len())
}

pub unsafe fn sa11x0_init_late() { sa11x0_pm_init(); }

pub unsafe fn sa11x0_register_fixed_regulator(n: i32, cfg: *mut FixedVoltageConfig,
    supplies: *mut RegulatorConsumerSupply, num_supplies: u32, uses_gpio: bool) -> i32 {
    let id = kzalloc_obj((*cfg).init_data);
    (*cfg).init_data = id;
    if id.is_null() { return -ENOMEM; }
    if !uses_gpio { (*id).constraints.always_on = 1; }
    (*id).constraints.name = (*cfg).supply_name;
    (*id).constraints.min_uV = (*cfg).microvolts;
    (*id).constraints.max_uV = (*cfg).microvolts;
    (*id).constraints.valid_modes_mask = REGULATOR_MODE_NORMAL;
    (*id).constraints.valid_ops_mask = REGULATOR_CHANGE_STATUS;
    (*id).consumer_supplies = supplies;
    (*id).num_consumer_supplies = num_supplies;
    platform_device_register_resndata(core::ptr::null_mut(), b"reg-fixed-voltage\0".as_ptr().cast(), n,
        core::ptr::null_mut(), 0, cfg.cast(), core::mem::size_of::<FixedVoltageConfig>());
    0
}

pub unsafe fn sa1100_map_io() { iotable_init(standard_io_desc.as_mut_ptr(), standard_io_desc.len()); }
pub unsafe fn sa1100_timer_init() { pxa_timer_nodt_init(IRQ_OST0, io_p2v(0x90000000)); }

pub unsafe fn sa1100_init_irq() {
    request_resource(&mut iomem_resource, &mut irq_resource);
    sa11x0_init_irq_nodt(IRQ_GPIO0_SC, irq_resource.start);
    sa1100_init_gpio();
    sa11xx_clk_init();
}

/* Disable the memory bus request/grant signals on the SA1110. */
pub unsafe fn sa1110_mb_disable() {
    let mut flags = 0;
    local_irq_save(&mut flags);
    PGSR &= !GPIO_MBGNT;
    GPCR = GPIO_MBGNT;
    GPDR = (GPDR & !GPIO_MBREQ) | GPIO_MBGNT;
    GAFR &= !(GPIO_MBGNT | GPIO_MBREQ);
    local_irq_restore(flags);
}

/* If the system is going to use the SA-1111 DMA engines, set up the memory bus request/grant pins. */
pub unsafe fn sa1110_mb_enable() {
    let mut flags = 0;
    local_irq_save(&mut flags);
    PGSR &= !GPIO_MBGNT;
    GPCR = GPIO_MBGNT;
    GPDR = (GPDR & !GPIO_MBREQ) | GPIO_MBGNT;
    GAFR |= GPIO_MBGNT | GPIO_MBREQ;
    TUCR |= TUCR_MR;
    local_irq_restore(flags);
}

pub unsafe fn sa11x0_gpio_set_wake(gpio: u32, on: u32) -> i32 {
    if on != 0 { PWER |= 1u32 << gpio; } else { PWER &= !(1u32 << gpio); }
    0
}

pub unsafe fn sa11x0_sc_set_wake(irq: u32, on: u32) -> i32 {
    if (1u32 << irq) != IC_RTCAlrm { return -EINVAL; }
    if on != 0 { PWER |= PWER_RTC; } else { PWER &= !PWER_RTC; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
