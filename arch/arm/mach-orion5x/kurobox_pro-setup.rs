// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of arch/arm/mach-orion5x/kurobox_pro-setup.c */

// Kernel and architecture dependencies supplied by the surrounding tree.
use core::ffi::{c_char, c_void};

const KUROBOX_PRO_NOR_BOOT_BASE: usize = 0xf4000000;
const KUROBOX_PRO_NOR_BOOT_SIZE: usize = 256 * 1024;
const KUROBOX_PRO_NAND_BASE: usize = 0xfc000000;
const KUROBOX_PRO_NAND_SIZE: usize = 2 * 1024 * 1024;

#[repr(C)]
struct MtdPartition { name: *const c_char, offset: usize, size: usize }
#[repr(C)]
struct Resource { flags: u64, start: usize, end: usize }
#[repr(C)]
struct OrionNandData { parts: *mut MtdPartition, nr_parts: usize, cle: u32, ale: u32, width: u32 }
#[repr(C)]
struct PhysmapFlashData { width: u32 }
#[repr(C)]
struct Device { platform_data: *mut c_void }
#[repr(C)]
struct PlatformDevice { name: *const c_char, id: i32, dev: Device, resource: *mut Resource, num_resources: u32 }
#[repr(C)]
struct PciDev { _private: [u8; 0] }
#[repr(C)]
struct HwPci { nr_controllers: u32, setup: Option<unsafe extern "C" fn()>, scan: Option<unsafe extern "C" fn()>, map_irq: Option<unsafe extern "C" fn(*const PciDev, u8, u8) -> i32> }
#[repr(C)]
struct EthPlatformData { phy_addr: u32 }
#[repr(C)]
struct I2cBoardInfo { kind: [u8; 20], addr: u16 }
#[repr(C)]
struct SataPlatformData { n_ports: u32 }

extern "C" {
    static mut orion5x_tclk: u32;
    fn orion5x_pci_map_irq(dev: *const PciDev, slot: u8, pin: u8) -> i32;
    fn orion5x_pci_sys_setup(); fn orion5x_pci_sys_scan_bus();
    fn orion5x_pci_disable(); fn pci_common_init(pci: *mut HwPci);
    fn machine_is_kurobox_pro() -> bool;
    fn readl(addr: usize) -> u32; fn writel(value: u32, addr: usize);
    fn udelay(usecs: u32); fn mdelay(msecs: u32); fn barrier();
    fn printk(fmt: *const c_char, ...); fn pr_info(fmt: *const c_char, ...);
    fn memset(dst: *mut c_void, value: i32, size: usize) -> *mut c_void;
    fn orion5x_init(); fn orion5x_mpp_conf(modes: *mut u32);
    fn orion5x_ehci0_init(); fn orion5x_ehci1_init();
    fn orion5x_eth_init(data: *mut EthPlatformData); fn orion5x_i2c_init();
    fn orion5x_sata_init(data: *mut SataPlatformData); fn orion5x_uart0_init();
    fn orion5x_uart1_init(); fn orion5x_xor_init();
    fn mvebu_mbus_add_window_by_id(target: u32, attr: u32, base: usize, size: usize);
    fn platform_device_register(dev: *mut PlatformDevice);
    fn i2c_register_board_info(bus: i32, info: *mut I2cBoardInfo, count: u32);
    fn register_platform_power_off(handler: unsafe extern "C" fn());
}

static mut kurobox_pro_nand_parts: [MtdPartition; 3] = [
    MtdPartition { name: b"uImage\0".as_ptr() as *const c_char, offset: 0, size: 4 * 1024 * 1024 },
    MtdPartition { name: b"rootfs\0".as_ptr() as *const c_char, offset: 4 * 1024 * 1024, size: 64 * 1024 * 1024 },
    MtdPartition { name: b"extra\0".as_ptr() as *const c_char, offset: 68 * 1024 * 1024, size: 256 * 1024 * 1024 - 68 * 1024 * 1024 },
];
static mut kurobox_pro_nand_resource: Resource = Resource { flags: 0x200, start: KUROBOX_PRO_NAND_BASE, end: KUROBOX_PRO_NAND_BASE + KUROBOX_PRO_NAND_SIZE - 1 };
static mut kurobox_pro_nand_data: OrionNandData = OrionNandData { parts: core::ptr::null_mut(), nr_parts: 3, cle: 0, ale: 1, width: 8 };
static mut kurobox_pro_nand_flash: PlatformDevice = PlatformDevice { name: b"orion_nand\0".as_ptr() as *const c_char, id: -1, dev: Device { platform_data: core::ptr::null_mut() }, resource: core::ptr::null_mut(), num_resources: 1 };
static mut kurobox_pro_nor_flash_data: PhysmapFlashData = PhysmapFlashData { width: 1 };
static mut kurobox_pro_nor_flash_resource: Resource = Resource { flags: 0x200, start: KUROBOX_PRO_NOR_BOOT_BASE, end: KUROBOX_PRO_NOR_BOOT_BASE + KUROBOX_PRO_NOR_BOOT_SIZE - 1 };
static mut kurobox_pro_nor_flash: PlatformDevice = PlatformDevice { name: b"physmap-flash\0".as_ptr() as *const c_char, id: 0, dev: Device { platform_data: core::ptr::null_mut() }, resource: core::ptr::null_mut(), num_resources: 1 };

const UART1_VIRT_BASE: usize = 0;
const UART_LSR: usize = 5; const UART_RX: usize = 0; const UART_TX: usize = 0;
const UART_IER: usize = 1; const UART_FCR: usize = 2; const UART_MCR: usize = 4; const UART_LCR: usize = 3;
const UART_LSR_DR: u32 = 1; const UART_LSR_THRE: u32 = 0x20;
#[inline] unsafe fn uart1_reg(x: usize) -> usize { UART1_VIRT_BASE + (x << 2) }

unsafe extern "C" fn kurobox_pro_pci_map_irq(dev: *const PciDev, slot: u8, pin: u8) -> i32 {
    let irq = orion5x_pci_map_irq(dev, slot, pin); if irq != -1 { irq } else { -1 }
}
static mut kurobox_pro_pci: HwPci = HwPci { nr_controllers: 2, setup: Some(orion5x_pci_sys_setup), scan: Some(orion5x_pci_sys_scan_bus), map_irq: Some(kurobox_pro_pci_map_irq) };
unsafe extern "C" fn kurobox_pro_pci_init() -> i32 { if machine_is_kurobox_pro() { orion5x_pci_disable(); pci_common_init(&mut kurobox_pro_pci); } 0 }

static mut kurobox_pro_eth_data: EthPlatformData = EthPlatformData { phy_addr: 8 };
static mut kurobox_pro_i2c_rtc: I2cBoardInfo = I2cBoardInfo { kind: *b"rs5c372a\0\0\0\0\0\0\0\0\0\0\0\0", addr: 0x32 };
static mut kurobox_pro_sata_data: SataPlatformData = SataPlatformData { n_ports: 2 };

unsafe fn kurobox_pro_miconread(buf: *mut u8, count: i32) -> i32 { let mut i = 0; while i < count { let mut timeout = 10; while readl(uart1_reg(UART_LSR)) & UART_LSR_DR == 0 { timeout -= 1; if timeout == 0 { break; } udelay(1000); } if timeout == 0 { break; } *buf.add(i as usize) = readl(uart1_reg(UART_RX)) as u8; i += 1; } i }
unsafe fn kurobox_pro_miconwrite(buf: *const u8, mut count: i32) -> i32 { let mut i = 0; while count > 0 { while readl(uart1_reg(UART_LSR)) & UART_LSR_THRE == 0 { barrier(); } writel(*buf.add(i), uart1_reg(UART_TX)); i += 1; count -= 1; } 0 }

unsafe fn kurobox_pro_miconsend(data: *const u8, count: i32) -> i32 { let mut checksum = 0u8; for i in 0..count { checksum = checksum.wrapping_sub(*data.add(i as usize)); } let mut retry = 2; let mut recv = [0u8; 40]; let mut send = [0u8; 40]; loop { kurobox_pro_miconwrite(data, count); kurobox_pro_miconwrite(&checksum, 1); if kurobox_pro_miconread(recv.as_mut_ptr(), 40) > 3 { if (recv[0] as u32 + recv[1] as u32 + recv[2] as u32 + recv[3] as u32) & 0xff == 0 && recv[0] == 1 && recv[1] == *data.add(1) && recv[2] == 0 { mdelay(10); return 0; } } else { memset(send.as_mut_ptr() as *mut c_void, 0xff, 40); kurobox_pro_miconwrite(send.as_ptr(), 40); mdelay(100); kurobox_pro_miconread(recv.as_mut_ptr(), 40); } if retry == 0 { break; } retry -= 1; } mdelay(10); -1 }

unsafe extern "C" fn kurobox_pro_power_off() { let watchdogkill = [1u8, 0x35, 0]; let shutdownwait = [0u8, 0x0c]; let poweroff = [0u8, 6]; let divisor = (orion5x_tclk + 8 * 38400) / (16 * 38400); writel(0x83, uart1_reg(UART_LCR)); writel(divisor & 0xff, uart1_reg(0)); writel((divisor >> 8) & 0xff, uart1_reg(0)); writel(0x1b, uart1_reg(UART_LCR)); writel(0, uart1_reg(UART_IER)); writel(7, uart1_reg(UART_FCR)); writel(0, uart1_reg(UART_MCR)); kurobox_pro_miconsend(watchdogkill.as_ptr(), 3); kurobox_pro_miconsend(shutdownwait.as_ptr(), 2); kurobox_pro_miconsend(poweroff.as_ptr(), 2); }

static mut kurobox_pro_mpp_modes: [u32; 21] = [0,0,2,2,0,0,3,3,0,0,0,0,4,4,4,4,5,5,5,5,0];
unsafe extern "C" fn kurobox_pro_init() { kurobox_pro_nand_data.parts = kurobox_pro_nand_parts.as_mut_ptr(); kurobox_pro_nand_flash.dev.platform_data = &mut kurobox_pro_nand_data as *mut _ as *mut c_void; kurobox_pro_nand_flash.resource = &mut kurobox_pro_nand_resource; kurobox_pro_nor_flash.dev.platform_data = &mut kurobox_pro_nor_flash_data as *mut _ as *mut c_void; kurobox_pro_nor_flash.resource = &mut kurobox_pro_nor_flash_resource; orion5x_init(); orion5x_mpp_conf(kurobox_pro_mpp_modes.as_mut_ptr()); orion5x_ehci0_init(); orion5x_ehci1_init(); orion5x_eth_init(&mut kurobox_pro_eth_data); orion5x_i2c_init(); orion5x_sata_init(&mut kurobox_pro_sata_data); orion5x_uart0_init(); orion5x_uart1_init(); orion5x_xor_init(); mvebu_mbus_add_window_by_id(0,0, KUROBOX_PRO_NOR_BOOT_BASE, KUROBOX_PRO_NOR_BOOT_SIZE); platform_device_register(&mut kurobox_pro_nor_flash); if machine_is_kurobox_pro() { mvebu_mbus_add_window_by_id(0,0, KUROBOX_PRO_NAND_BASE, KUROBOX_PRO_NAND_SIZE); platform_device_register(&mut kurobox_pro_nand_flash); } i2c_register_board_info(0, &mut kurobox_pro_i2c_rtc, 1); register_platform_power_off(kurobox_pro_power_off); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
