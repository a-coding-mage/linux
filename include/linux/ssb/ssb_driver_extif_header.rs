/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Hardware-specific External Interface I/O core definitions
 * for the BCM47xx family of SiliconBackplane-based chips.
 *
 * Translated from the C header. External dependencies are intentionally
 * referenced but not defined here.
 */

/* external interface address space */
pub const fn ssb_extif_pcmcia_membase(x: u32) -> u32 { x }
pub const fn ssb_extif_pcmcia_iobase(x: u32) -> u32 { x + 0x100000 }
pub const fn ssb_extif_pcmcia_cfgbase(x: u32) -> u32 { x + 0x200000 }
pub const fn ssb_extif_cfgif_base(x: u32) -> u32 { x + 0x800000 }
pub const fn ssb_extif_flash_base(x: u32) -> u32 { x + 0xc00000 }

pub const SSB_EXTIF_NR_GPIOOUT: u32 = 5;
/* Multiple instances of output and output-enable registers allow multiple
 * cores to control GPIO outputs without sharing one register pair. */
pub const fn ssb_extif_gpio_out(index: u32) -> u32 {
    assert!(index < SSB_EXTIF_NR_GPIOOUT);
    SSB_EXTIF_GPIO_OUT_BASE + index * 8
}
pub const fn ssb_extif_gpio_outen(index: u32) -> u32 {
    assert!(index < SSB_EXTIF_NR_GPIOOUT);
    SSB_EXTIF_GPIO_OUTEN_BASE + index * 8
}

/* EXTIF core registers */
pub const SSB_EXTIF_CTL: u32 = 0x0000;
pub const SSB_EXTIF_CTL_UARTEN: u32 = 1 << 0;
pub const SSB_EXTIF_EXTSTAT: u32 = 0x0004;
pub const SSB_EXTIF_EXTSTAT_EMODE: u32 = 1 << 0;
pub const SSB_EXTIF_EXTSTAT_EIRQPIN: u32 = 1 << 1;
pub const SSB_EXTIF_EXTSTAT_GPIOIRQPIN: u32 = 1 << 2;
pub const SSB_EXTIF_PCMCIA_CFG: u32 = 0x0010;
pub const SSB_EXTIF_PCMCIA_MEMWAIT: u32 = 0x0014;
pub const SSB_EXTIF_PCMCIA_ATTRWAIT: u32 = 0x0018;
pub const SSB_EXTIF_PCMCIA_IOWAIT: u32 = 0x001c;
pub const SSB_EXTIF_PROG_CFG: u32 = 0x0020;
pub const SSB_EXTIF_PROG_WAITCNT: u32 = 0x0024;
pub const SSB_EXTIF_FLASH_CFG: u32 = 0x0028;
pub const SSB_EXTIF_FLASH_WAITCNT: u32 = 0x002c;
pub const SSB_EXTIF_WATCHDOG: u32 = 0x0040;
pub const SSB_EXTIF_CLOCK_N: u32 = 0x0044;
pub const SSB_EXTIF_CLOCK_SB: u32 = 0x0048;
pub const SSB_EXTIF_CLOCK_PCI: u32 = 0x004c;
pub const SSB_EXTIF_CLOCK_MII: u32 = 0x0050;
pub const SSB_EXTIF_GPIO_IN: u32 = 0x0060;
pub const SSB_EXTIF_GPIO_OUT_BASE: u32 = 0x0064;
pub const SSB_EXTIF_GPIO_OUTEN_BASE: u32 = 0x0068;
pub const SSB_EXTIF_EJTAG_OUTEN: u32 = 0x0090;
pub const SSB_EXTIF_GPIO_INTPOL: u32 = 0x0094;
pub const SSB_EXTIF_GPIO_INTMASK: u32 = 0x0098;
pub const SSB_EXTIF_UART_DATA: u32 = 0x0300;
pub const SSB_EXTIF_UART_TIMER: u32 = 0x0310;
pub const SSB_EXTIF_UART_FCR: u32 = 0x0320;
pub const SSB_EXTIF_UART_LCR: u32 = 0x0330;
pub const SSB_EXTIF_UART_MCR: u32 = 0x0340;
pub const SSB_EXTIF_UART_LSR: u32 = 0x0350;
pub const SSB_EXTIF_UART_MSR: u32 = 0x0360;
pub const SSB_EXTIF_UART_SCRATCH: u32 = 0x0370;

/* pcmcia/prog/flash_config */
pub const SSB_EXTCFG_EN: u32 = 1 << 0;
pub const SSB_EXTCFG_MODE: u32 = 0xE;
pub const SSB_EXTCFG_MODE_SHIFT: u32 = 1;
pub const SSB_EXTCFG_MODE_FLASH: u32 = 0x0;
pub const SSB_EXTCFG_MODE_SYNC: u32 = 0x2;
pub const SSB_EXTCFG_MODE_PCMCIA: u32 = 0x4;
pub const SSB_EXTCFG_DS16: u32 = 1 << 4;
pub const SSB_EXTCFG_BSWAP: u32 = 1 << 5;
pub const SSB_EXTCFG_CLKDIV: u32 = 0xC0;
pub const SSB_EXTCFG_CLKDIV_SHIFT: u32 = 6;
pub const SSB_EXTCFG_CLKDIV_2: u32 = 0x0;
pub const SSB_EXTCFG_CLKDIV_3: u32 = 0x40;
pub const SSB_EXTCFG_CLKDIV_4: u32 = 0x80;
pub const SSB_EXTCFG_CLKEN: u32 = 1 << 8;
pub const SSB_EXTCFG_STROBE: u32 = 1 << 9;

/* The C names are kept explicit because token pasting is not available in
 * stable Rust macro_rules!; these values remain part of the public ABI. */
pub const SSB_PCMCIA_MEMW_0: u32 = 0x0000003F;
pub const SSB_PCMCIA_MEMW_1: u32 = 0x00001F00;
pub const SSB_PCMCIA_MEMW_1_SHIFT: u32 = 8;
pub const SSB_PCMCIA_MEMW_2: u32 = 0x001F0000;
pub const SSB_PCMCIA_MEMW_2_SHIFT: u32 = 16;
pub const SSB_PCMCIA_MEMW_3: u32 = 0x1F000000;
pub const SSB_PCMCIA_MEMW_3_SHIFT: u32 = 24;
pub const SSB_PCMCIA_ATTW_0: u32 = 0x0000003F;
pub const SSB_PCMCIA_ATTW_1: u32 = 0x00001F00;
pub const SSB_PCMCIA_ATTW_1_SHIFT: u32 = 8;
pub const SSB_PCMCIA_ATTW_2: u32 = 0x001F0000;
pub const SSB_PCMCIA_ATTW_2_SHIFT: u32 = 16;
pub const SSB_PCMCIA_ATTW_3: u32 = 0x1F000000;
pub const SSB_PCMCIA_ATTW_3_SHIFT: u32 = 24;
pub const SSB_PCMCIA_IOW_0: u32 = 0x0000003F;
pub const SSB_PCMCIA_IOW_1: u32 = 0x00001F00;
pub const SSB_PCMCIA_IOW_1_SHIFT: u32 = 8;
pub const SSB_PCMCIA_IOW_2: u32 = 0x001F0000;
pub const SSB_PCMCIA_IOW_2_SHIFT: u32 = 16;
pub const SSB_PCMCIA_IOW_3: u32 = 0x1F000000;
pub const SSB_PCMCIA_IOW_3_SHIFT: u32 = 24;
pub const SSB_PROG_WCNT_0: u32 = 0x0000001F;
pub const SSB_PROG_WCNT_1: u32 = 0x00001F00;
pub const SSB_PROG_WCNT_1_SHIFT: u32 = 8;
pub const SSB_PROG_WCNT_2: u32 = 0x001F0000;
pub const SSB_PROG_WCNT_2_SHIFT: u32 = 16;
pub const SSB_PROG_WCNT_3: u32 = 0x1F000000;
pub const SSB_PROG_WCNT_3_SHIFT: u32 = 24;
pub const SSB_PROG_W0: u32 = 0x0000000C;
pub const SSB_PROG_W1: u32 = 0x00000A00;
pub const SSB_PROG_W2: u32 = 0x00020000;
pub const SSB_PROG_W3: u32 = 0x01000000;
pub const SSB_FLASH_WCNT_0: u32 = 0x0000001F;
pub const SSB_FLASH_WCNT_1: u32 = 0x00001F00;
pub const SSB_FLASH_WCNT_1_SHIFT: u32 = 8;
pub const SSB_FLASH_WCNT_2: u32 = 0x001F0000;
pub const SSB_FLASH_WCNT_2_SHIFT: u32 = 16;
pub const SSB_FLASH_WCNT_3: u32 = 0x1F000000;
pub const SSB_FLASH_WCNT_3_SHIFT: u32 = 24;

pub const SSB_EXTIF_WATCHDOG_CLK: u32 = 48_000_000;
pub const SSB_EXTIF_WATCHDOG_MAX_TIMER: u32 = (1 << 28) - 1;
pub const SSB_EXTIF_WATCHDOG_MAX_TIMER_MS: u32 =
    SSB_EXTIF_WATCHDOG_MAX_TIMER / (SSB_EXTIF_WATCHDOG_CLK / 1000);

/* CONFIG_SSB_DRIVER_EXTIF selects the enabled declarations in the C build. */
#[cfg(feature = "CONFIG_SSB_DRIVER_EXTIF")]
#[repr(C)]
pub struct ssb_extif {
    pub dev: *mut ssb_device,
    pub gpio_lock: spinlock_t,
}

#[cfg(feature = "CONFIG_SSB_DRIVER_EXTIF")]
pub unsafe fn ssb_extif_available(extif: *mut ssb_extif) -> bool {
    !(*extif).dev.is_null()
}

#[cfg(feature = "CONFIG_SSB_DRIVER_EXTIF")]
extern "C" {
    pub fn ssb_extif_get_clockcontrol(extif: *mut ssb_extif, plltype: *mut u32, n: *mut u32, m: *mut u32);
    pub fn ssb_extif_timing_init(extif: *mut ssb_extif, ns: libc::c_ulong);
    pub fn ssb_extif_watchdog_timer_set(extif: *mut ssb_extif, ticks: u32) -> u32;
    pub fn ssb_extif_gpio_in(extif: *mut ssb_extif, mask: u32) -> u32;
    pub fn ssb_extif_gpio_out(extif: *mut ssb_extif, mask: u32, value: u32) -> u32;
    pub fn ssb_extif_gpio_outen(extif: *mut ssb_extif, mask: u32, value: u32) -> u32;
    pub fn ssb_extif_gpio_polarity(extif: *mut ssb_extif, mask: u32, value: u32) -> u32;
    pub fn ssb_extif_gpio_intmask(extif: *mut ssb_extif, mask: u32, value: u32) -> u32;
}

#[cfg(all(feature = "CONFIG_SSB_DRIVER_EXTIF", feature = "CONFIG_SSB_SERIAL"))]
extern "C" {
    pub fn ssb_extif_serial_init(extif: *mut ssb_extif, ports: *mut ssb_serial_port) -> libc::c_int;
}

/* External types supplied by other translated headers. */
#[cfg(feature = "CONFIG_SSB_DRIVER_EXTIF")]
extern "C" {
    type ssb_device;
    type spinlock_t;
}
#[cfg(all(feature = "CONFIG_SSB_DRIVER_EXTIF", feature = "CONFIG_SSB_SERIAL"))]
extern "C" { type ssb_serial_port; }

/* Disabled-driver inline stubs, corresponding to the #else branch. */
#[cfg(not(feature = "CONFIG_SSB_DRIVER_EXTIF"))]
#[repr(C)]
pub struct ssb_extif {}
#[cfg(not(feature = "CONFIG_SSB_DRIVER_EXTIF"))]
pub const fn ssb_extif_available(_extif: *mut ssb_extif) -> bool { false }
#[cfg(not(feature = "CONFIG_SSB_DRIVER_EXTIF"))]
pub fn ssb_extif_get_clockcontrol(_extif: *mut ssb_extif, _plltype: *mut u32, _n: *mut u32, _m: *mut u32) {}
#[cfg(not(feature = "CONFIG_SSB_DRIVER_EXTIF"))]
pub fn ssb_extif_timing_init(_extif: *mut ssb_extif, _ns: libc::c_ulong) {}
#[cfg(not(feature = "CONFIG_SSB_DRIVER_EXTIF"))]
pub const fn ssb_extif_watchdog_timer_set(_extif: *mut ssb_extif, _ticks: u32) -> u32 { 0 }
#[cfg(not(feature = "CONFIG_SSB_DRIVER_EXTIF"))]
pub const fn ssb_extif_gpio_in(_extif: *mut ssb_extif, _mask: u32) -> u32 { 0 }
#[cfg(not(feature = "CONFIG_SSB_DRIVER_EXTIF"))]
pub const fn ssb_extif_gpio_out(_extif: *mut ssb_extif, _mask: u32, _value: u32) -> u32 { 0 }
#[cfg(not(feature = "CONFIG_SSB_DRIVER_EXTIF"))]
pub const fn ssb_extif_gpio_outen(_extif: *mut ssb_extif, _mask: u32, _value: u32) -> u32 { 0 }
#[cfg(not(feature = "CONFIG_SSB_DRIVER_EXTIF"))]
pub const fn ssb_extif_gpio_polarity(_extif: *mut ssb_extif, _mask: u32, _value: u32) -> u32 { 0 }
#[cfg(not(feature = "CONFIG_SSB_DRIVER_EXTIF"))]
pub const fn ssb_extif_gpio_intmask(_extif: *mut ssb_extif, _mask: u32, _value: u32) -> u32 { 0 }
#[cfg(all(not(feature = "CONFIG_SSB_DRIVER_EXTIF"), feature = "CONFIG_SSB_SERIAL"))]
pub const fn ssb_extif_serial_init(_extif: *mut ssb_extif, _ports: *mut ssb_serial_port) -> libc::c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
