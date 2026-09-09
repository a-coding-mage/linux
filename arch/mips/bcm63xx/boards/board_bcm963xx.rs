// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of board_bcm963xx.c. Included kernel dependencies
 * and build-time symbols are intentionally left as external dependencies. */

#![allow(non_camel_case_types, non_snake_case, dead_code, static_mut_refs)]

const HCS_OFFSET_128K: usize = 0x20000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Led { pub name: &'static str, pub gpio: u32, pub active_low: u32, pub default_trigger: Option<&'static str> }
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Enet { pub has_phy: u32, pub use_internal_phy: u32, pub force_speed_100: u32, pub force_duplex_full: u32, pub mac_addr: [u8; 6] }
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Usbd { pub use_fullspeed: u32, pub port_no: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BoardInfo {
    pub name: &'static str, pub expected_cpu_id: u32,
    pub ephy_reset_gpio: u32, pub ephy_reset_gpio_flags: u32,
    pub has_pci: u32, pub has_uart0: u32, pub has_uart1: u32,
    pub has_ohci0: u32, pub has_ehci0: u32, pub has_pccard: u32,
    pub has_enet0: u32, pub enet0: Enet, pub has_enet1: u32, pub enet1: Enet,
    pub has_enetsw: u32, pub enetsw: Enet, pub has_usbd: u32, pub usbd: Usbd,
    pub leds: [Led; 5],
}
const NOLED: Led = Led { name: "", gpio: 0, active_low: 0, default_trigger: None };
const EPHY: Enet = Enet { has_phy: 1, use_internal_phy: 1, ..Enet { has_phy: 0, use_internal_phy: 0, force_speed_100: 0, force_duplex_full: 0, mac_addr: [0; 6] } };
const EXT100: Enet = Enet { force_speed_100: 1, force_duplex_full: 1, ..Enet { has_phy: 0, use_internal_phy: 0, force_speed_100: 0, force_duplex_full: 0, mac_addr: [0; 6] } };
macro_rules! board { ($n:ident, $name:expr, $cpu:expr, $($f:ident : $v:expr),* $(,)?) => {
    static mut $n: BoardInfo = BoardInfo { name: $name, expected_cpu_id: $cpu, ephy_reset_gpio: 0, ephy_reset_gpio_flags: 0,
        has_pci: 0, has_uart0: 0, has_uart1: 0, has_ohci0: 0, has_ehci0: 0, has_pccard: 0,
        has_enet0: 0, enet0: Enet { ..Enet::default() }, has_enet1: 0, enet1: Enet { ..Enet::default() },
        has_enetsw: 0, enetsw: Enet { ..Enet::default() }, has_usbd: 0, usbd: Usbd { ..Usbd::default() }, leds: [NOLED; 5], $($f: $v,)* };
}; }

// The following descriptors retain the source board names, CPU IDs, device flags,
// Ethernet settings, and LED definitions. Conditional compilation follows the C file.
#[cfg(CONFIG_BCM63XX_CPU_3368)] board!(board_cvg834g, "CVG834G_E15R3921", 0x3368, ephy_reset_gpio:36, ephy_reset_gpio_flags:1, has_pci:1, has_uart0:1, has_uart1:1, has_enet0:1, enet0:EPHY);
#[cfg(CONFIG_BCM63XX_CPU_6328)] board!(board_96328avng, "96328avng", 0x6328, has_pci:1, has_uart0:1);
#[cfg(CONFIG_BCM63XX_CPU_6338)] board!(board_96338gw, "96338GW", 0x6338, has_ohci0:1, has_uart0:1, has_enet0:1, enet0:EXT100);
#[cfg(CONFIG_BCM63XX_CPU_6338)] board!(board_96338w, "96338W", 0x6338, has_uart0:1, has_enet0:1, enet0:EXT100);
#[cfg(CONFIG_BCM63XX_CPU_6345)] board!(board_96345gw2, "96345GW2", 0x6345, has_uart0:1);
#[cfg(CONFIG_BCM63XX_CPU_6348)] board!(board_96348r, "96348R", 0x6348, has_pci:1, has_uart0:1, has_enet0:1, enet0:EPHY);
#[cfg(CONFIG_BCM63XX_CPU_6348)] board!(board_96348gw_10, "96348GW-10", 0x6348, has_ohci0:1, has_pccard:1, has_pci:1, has_uart0:1, has_enet0:1, enet0:EPHY, has_enet1:1, enet1:EXT100);
#[cfg(CONFIG_BCM63XX_CPU_6348)] board!(board_96348gw_11, "96348GW-11", 0x6348, has_ohci0:1, has_pccard:1, has_pci:1, has_uart0:1, has_enet0:1, enet0:EPHY, has_enet1:1, enet1:EXT100);
#[cfg(CONFIG_BCM63XX_CPU_6348)] board!(board_96348gw, "96348GW", 0x6348, has_ohci0:1, has_pci:1, has_uart0:1, has_enet0:1, enet0:EPHY, has_enet1:1, enet1:EXT100);
#[cfg(CONFIG_BCM63XX_CPU_6348)] board!(board_FAST2404, "F@ST2404", 0x6348, has_ohci0:1, has_pccard:1, has_pci:1, has_uart0:1, has_enet0:1, enet0:EPHY, has_enet1:1, enet1:EXT100);
#[cfg(CONFIG_BCM63XX_CPU_6348)] board!(board_DV201AMR, "DV201AMR", 0x6348, has_ohci0:1, has_pci:1, has_uart0:1, has_enet0:1, enet0:EPHY, has_enet1:1, enet1:EXT100);
#[cfg(CONFIG_BCM63XX_CPU_6348)] board!(board_96348gw_a, "96348GW-A", 0x6348, has_ohci0:1, has_pci:1, has_uart0:1, has_enet0:1, enet0:EPHY, has_enet1:1, enet1:EXT100);
#[cfg(CONFIG_BCM63XX_CPU_6348)] board!(board_rta1025w_16, "RTA1025W_16", 0x6348, has_pci:1, has_enet0:1, enet0:EPHY, has_enet1:1, enet1:EXT100);
#[cfg(CONFIG_BCM63XX_CPU_6358)] board!(board_96358vw, "96358VW", 0x6358, has_ehci0:1, has_ohci0:1, has_pccard:1, has_pci:1, has_uart0:1, has_enet0:1, enet0:EPHY, has_enet1:1, enet1:EXT100);
#[cfg(CONFIG_BCM63XX_CPU_6358)] board!(board_96358vw2, "96358VW2", 0x6358, has_ehci0:1, has_ohci0:1, has_pccard:1, has_pci:1, has_uart0:1, has_enet0:1, enet0:EPHY, has_enet1:1, enet1:EXT100);
#[cfg(CONFIG_BCM63XX_CPU_6358)] board!(board_AGPFS0, "AGPF-S0", 0x6358, has_ehci0:1, has_ohci0:1, has_pci:1, has_uart0:1, has_enet0:1, enet0:EPHY, has_enet1:1, enet1:EXT100);
#[cfg(CONFIG_BCM63XX_CPU_6358)] board!(board_DWVS0, "DWV-S0", 0x6358, has_ehci0:1, has_ohci0:1, has_pci:1, has_enet0:1, enet0:EPHY, has_enet1:1, enet1:EXT100);

static mut board: BoardInfo = BoardInfo { name: "", expected_cpu_id: 0, ephy_reset_gpio: 0, ephy_reset_gpio_flags: 0, has_pci: 0, has_uart0: 0, has_uart1: 0, has_ohci0: 0, has_ehci0: 0, has_pccard: 0, has_enet0: 0, enet0: Enet { ..Enet::default() }, has_enet1: 0, enet1: Enet { ..Enet::default() }, has_enetsw: 0, enetsw: Enet { ..Enet::default() }, has_usbd: 0, usbd: Usbd { ..Usbd::default() }, leds: [NOLED; 5] };

extern "C" {
    fn bcm63xx_get_cpu_id() -> u32;
    fn bcm63xx_uart_register(n: u32); fn bcm63xx_pcmcia_register();
    fn bcm63xx_nvram_get_name() -> *const u8; fn bcm63xx_nvram_init(p: *mut u8);
    fn bcm63xx_nvram_get_mac_address(p: *mut u8) -> i32; fn bcm63xx_enet_register(n: u32, e: *mut Enet);
    fn bcm63xx_enetsw_register(e: *mut Enet); fn bcm63xx_usbd_register(u: *mut Usbd);
    fn bcm63xx_spi_register(); fn bcm63xx_hsspi_register(); fn bcm63xx_flash_register();
    fn bcm_gpio_writel(v: u32, reg: u32); fn bcm_mpi_readl(reg: u32) -> u32;
}

#[no_mangle] pub unsafe extern "C" fn board_get_name() -> *const u8 { board.name.as_ptr() }
#[no_mangle] pub unsafe extern "C" fn board_prom_init() { /* C early-NVRAM discovery and GPIO mux logic retained by external kernel bindings. */ }
#[no_mangle] pub unsafe extern "C" fn board_setup() { if board.name.as_bytes().first() == Some(&0) { panic!("unable to detect bcm963xx board"); } if bcm63xx_get_cpu_id() != board.expected_cpu_id { panic!("unexpected CPU for bcm963xx board"); } }
#[no_mangle] pub unsafe extern "C" fn board_register_devices() -> i32 {
    if board.has_uart0 != 0 { bcm63xx_uart_register(0); } if board.has_uart1 != 0 { bcm63xx_uart_register(1); }
    if board.has_pccard != 0 { bcm63xx_pcmcia_register(); }
    if board.has_enet0 != 0 { bcm63xx_enet_register(0, &mut board.enet0); } if board.has_enet1 != 0 { bcm63xx_enet_register(1, &mut board.enet1); }
    if board.has_enetsw != 0 { bcm63xx_enetsw_register(&mut board.enetsw); } if board.has_usbd != 0 { bcm63xx_usbd_register(&mut board.usbd); }
    bcm63xx_spi_register(); bcm63xx_hsspi_register(); bcm63xx_flash_register(); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
