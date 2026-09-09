/*
 * Common routines for the Motorola SPS MPC106/8240/107 Host bridge/Mem
 * ctlr/EPIC/etc.
 *
 * Author: Mark A. Greer
 *         mgreer@mvista.com
 *
 * 2001 (c) MontaVista, Software, Inc.  This file is licensed under
 * the terms of the GNU General Public License version 2.  This program
 * is licensed "as is" without any warranty of any kind, whether express
 * or implied.
 */

// Dependencies supplied by the surrounding translated system:
// linux/pci_ids.h and asm/pci-bridge.h

/*
 * The values here don't completely map everything but should work in most
 * cases.
 *
 * MAP A (PReP Map)
 *   Processor: 0x80000000 - 0x807fffff -> PCI I/O: 0x00000000 - 0x007fffff
 *   Processor: 0xc0000000 - 0xdfffffff -> PCI MEM: 0x00000000 - 0x1fffffff
 *   PCI MEM:   0x80000000 -> Processor System Memory: 0x00000000
 *
 * MAP B (CHRP Map)
 *   Processor: 0xfe000000 - 0xfebfffff -> PCI I/O: 0x00000000 - 0x00bfffff
 *   Processor: 0x80000000 - 0xbfffffff -> PCI MEM: 0x80000000 - 0xbfffffff
 *   PCI MEM:   0x00000000 -> Processor System Memory: 0x00000000
 */

// Define the vendor/device IDs for the various bridges.
pub const MPC10X_BRIDGE_106: u32 = (PCI_DEVICE_ID_MOTOROLA_MPC106 << 16) | PCI_VENDOR_ID_MOTOROLA;
pub const MPC10X_BRIDGE_8240: u32 = (0x0003 << 16) | PCI_VENDOR_ID_MOTOROLA;
pub const MPC10X_BRIDGE_107: u32 = (0x0004 << 16) | PCI_VENDOR_ID_MOTOROLA;
pub const MPC10X_BRIDGE_8245: u32 = (0x0006 << 16) | PCI_VENDOR_ID_MOTOROLA;

pub const MPC10X_MEM_MAP_A: u32 = 1;
pub const MPC10X_MEM_MAP_B: u32 = 2;

pub const MPC10X_MAPA_CNFG_ADDR: u32 = 0x80000cf8;
pub const MPC10X_MAPA_CNFG_DATA: u32 = 0x80000cfc;
pub const MPC10X_MAPA_ISA_IO_BASE: u32 = 0x80000000;
pub const MPC10X_MAPA_ISA_MEM_BASE: u32 = 0xc0000000;
pub const MPC10X_MAPA_DRAM_OFFSET: u32 = 0x80000000;
pub const MPC10X_MAPA_PCI_INTACK_ADDR: u32 = 0xbffffff0;
pub const MPC10X_MAPA_PCI_IO_START: u32 = 0x00000000;
pub const MPC10X_MAPA_PCI_IO_END: u32 = 0x00800000 - 1;
pub const MPC10X_MAPA_PCI_MEM_START: u32 = 0x00000000;
pub const MPC10X_MAPA_PCI_MEM_END: u32 = 0x20000000 - 1;
pub const MPC10X_MAPA_PCI_MEM_OFFSET: u32 = MPC10X_MAPA_ISA_MEM_BASE - MPC10X_MAPA_PCI_MEM_START;

pub const MPC10X_MAPB_CNFG_ADDR: u32 = 0xfec00000;
pub const MPC10X_MAPB_CNFG_DATA: u32 = 0xfee00000;
pub const MPC10X_MAPB_ISA_IO_BASE: u32 = 0xfe000000;
pub const MPC10X_MAPB_ISA_MEM_BASE: u32 = 0x80000000;
pub const MPC10X_MAPB_DRAM_OFFSET: u32 = 0x00000000;
pub const MPC10X_MAPB_PCI_INTACK_ADDR: u32 = 0xfef00000;
pub const MPC10X_MAPB_PCI_IO_START: u32 = 0x00000000;
pub const MPC10X_MAPB_PCI_IO_END: u32 = 0x00c00000 - 1;
pub const MPC10X_MAPB_PCI_MEM_START: u32 = 0x80000000;
pub const MPC10X_MAPB_PCI_MEM_END: u32 = 0xc0000000 - 1;
pub const MPC10X_MAPB_PCI_MEM_OFFSET: u32 = MPC10X_MAPB_ISA_MEM_BASE - MPC10X_MAPB_PCI_MEM_START;

pub const MPC10X_CFG_PIR_REG: u32 = 0x09;
pub const MPC10X_CFG_PIR_HOST_BRIDGE: u32 = 0x00;
pub const MPC10X_CFG_PIR_AGENT: u32 = 0x01;
pub const MPC10X_CFG_EUMBBAR: u32 = 0x78;
pub const MPC10X_CFG_PICR1_REG: u32 = 0xa8;
pub const MPC10X_CFG_PICR1_ADDR_MAP_MASK: u32 = 0x00010000;
pub const MPC10X_CFG_PICR1_ADDR_MAP_A: u32 = 0x00010000;
pub const MPC10X_CFG_PICR1_ADDR_MAP_B: u32 = 0x00000000;
pub const MPC10X_CFG_PICR1_SPEC_PCI_RD: u32 = 0x00000004;
pub const MPC10X_CFG_PICR1_ST_GATH_EN: u32 = 0x00000040;
pub const MPC10X_CFG_PICR2_REG: u32 = 0xac;
pub const MPC10X_CFG_PICR2_COPYBACK_OPT: u32 = 0x00000001;
pub const MPC10X_CFG_MAPB_OPTIONS_REG: u32 = 0xe0;
pub const MPC10X_CFG_MAPB_OPTIONS_CFAE: u32 = 0x80; // CPU_FD_ALIAS_EN
pub const MPC10X_CFG_MAPB_OPTIONS_PFAE: u32 = 0x40; // PCI_FD_ALIAS_EN
pub const MPC10X_CFG_MAPB_OPTIONS_DR: u32 = 0x20; // DLL_RESET
pub const MPC10X_CFG_MAPB_OPTIONS_PCICH: u32 = 0x08; // PCI_COMPATIBILITY_HOLE
pub const MPC10X_CFG_MAPB_OPTIONS_PROCCH: u32 = 0x04; // PROC_COMPATIBILITY_HOLE

pub const MPC10X_MCTLR_MEM_START_1: u32 = 0x80; // Banks 0-3
pub const MPC10X_MCTLR_MEM_START_2: u32 = 0x84; // Banks 4-7
pub const MPC10X_MCTLR_EXT_MEM_START_1: u32 = 0x88; // Banks 0-3
pub const MPC10X_MCTLR_EXT_MEM_START_2: u32 = 0x8c; // Banks 4-7
pub const MPC10X_MCTLR_MEM_END_1: u32 = 0x90; // Banks 0-3
pub const MPC10X_MCTLR_MEM_END_2: u32 = 0x94; // Banks 4-7
pub const MPC10X_MCTLR_EXT_MEM_END_1: u32 = 0x98; // Banks 0-3
pub const MPC10X_MCTLR_EXT_MEM_END_2: u32 = 0x9c; // Banks 4-7
pub const MPC10X_MCTLR_MEM_BANK_ENABLES: u32 = 0xa0;

pub const MPC10X_EUMB_SIZE: u32 = 0x00100000; // Total EUMB size (1MB)
pub const MPC10X_EUMB_MU_OFFSET: u32 = 0x00000000; // Msg Unit reg offset
pub const MPC10X_EUMB_MU_SIZE: u32 = 0x00001000; // Msg Unit reg size
pub const MPC10X_EUMB_DMA_OFFSET: u32 = 0x00001000; // DMA Unit reg offset
pub const MPC10X_EUMB_DMA_SIZE: u32 = 0x00001000; // DMA Unit reg size
pub const MPC10X_EUMB_ATU_OFFSET: u32 = 0x00002000; // Addr xlate reg offset
pub const MPC10X_EUMB_ATU_SIZE: u32 = 0x00001000; // Addr xlate reg size
pub const MPC10X_EUMB_I2C_OFFSET: u32 = 0x00003000; // I2C Unit reg offset
pub const MPC10X_EUMB_I2C_SIZE: u32 = 0x00001000; // I2C Unit reg size
pub const MPC10X_EUMB_DUART_OFFSET: u32 = 0x00004000; // DUART Unit reg offset (8245)
pub const MPC10X_EUMB_DUART_SIZE: u32 = 0x00001000; // DUART Unit reg size (8245)
pub const MPC10X_EUMB_EPIC_OFFSET: u32 = 0x00040000; // EPIC offset in EUMB
pub const MPC10X_EUMB_EPIC_SIZE: u32 = 0x00030000; // EPIC size
pub const MPC10X_EUMB_PM_OFFSET: u32 = 0x000fe000; // Performance Monitor reg offset (8245)
pub const MPC10X_EUMB_PM_SIZE: u32 = 0x00001000; // Performance Monitor reg size (8245)
pub const MPC10X_EUMB_WP_OFFSET: u32 = 0x000ff000; // Data path diagnostic, watchpoint reg offset
pub const MPC10X_EUMB_WP_SIZE: u32 = 0x00001000; // Data path diagnostic, watchpoint reg size

#[repr(C)]
pub enum ppc_sys_devices {
    MPC10X_IIC1,
    MPC10X_DMA0,
    MPC10X_DMA1,
    MPC10X_UART0,
    MPC10X_UART1,
    NUM_PPC_SYS_DEVS,
}

extern "C" {
    pub fn mpc10x_bridge_init(
        hose: *mut pci_controller,
        current_map: u32,
        new_map: u32,
        phys_eumb_base: u32,
    ) -> i32;
    pub fn mpc10x_get_mem_size(mem_map: u32) -> libc::c_ulong;
    pub fn mpc10x_enable_store_gathering(hose: *mut pci_controller) -> i32;
    pub fn mpc10x_disable_store_gathering(hose: *mut pci_controller) -> i32;
    pub fn mpc10x_set_openpic();
    pub fn avr_uart_configure();
    pub fn avr_uart_send(c: libc::c_char);
}

// `pci_controller`, `PCI_DEVICE_ID_MOTOROLA_MPC106`, and
// `PCI_VENDOR_ID_MOTOROLA` are supplied by the surrounding translated system.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
