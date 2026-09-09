/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding kernel translation.

pub const SYSTEM_RAM_LOW: u32 = 1;
pub const SYSTEM_RAM_HIGH: u32 = 2;
pub const SYSTEM_RAM_RESERVED: u32 = 3;
pub const PCI_IO: u32 = 4;
pub const PCI_MEM: u32 = 5;
pub const LOONGSON_CFG_REG: u32 = 6;
pub const VIDEO_ROM: u32 = 7;
pub const ADAPTER_ROM: u32 = 8;
pub const ACPI_TABLE: u32 = 9;
pub const SMBIOS_TABLE: u32 = 10;
pub const UMA_VIDEO_RAM: u32 = 11;
pub const VUMA_VIDEO_RAM: u32 = 12;
pub const MAX_MEMORY_TYPE: u32 = 13;

pub const MEM_SIZE_IS_IN_BYTES: u32 = 1u32 << 31;

pub const LOONGSON3_BOOT_MEM_MAP_MAX: usize = 128;

#[repr(C, packed)]
pub struct efi_memory_map_loongson {
    pub vers: u16, /* version of efi_memory_map */
    pub nr_map: u32, /* number of memory_maps */
    pub mem_freq: u32, /* memory frequence */
    pub map: [mem_map; LOONGSON3_BOOT_MEM_MAP_MAX],
}

#[repr(C, packed)]
pub struct mem_map {
    pub node_id: u32, /* node_id which memory attached to */
    pub mem_type: u32, /* system memory, pci memory, pci io, etc. */
    pub mem_start: u64, /* memory map start address */
    pub mem_size: u32, /* each memory_map size, not the total size */
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum loongson_cpu_type {
    Legacy_2E = 0x0,
    Legacy_2F = 0x1,
    Legacy_3A = 0x2,
    Legacy_3B = 0x3,
    Legacy_1A = 0x4,
    Legacy_1B = 0x5,
    Legacy_2G = 0x6,
    Legacy_2H = 0x7,
    Legacy_2K = 0x8,
    Loongson_1A = 0x100,
    Loongson_1B = 0x101,
    Loongson_2E = 0x200,
    Loongson_2F = 0x201,
    Loongson_2G = 0x202,
    Loongson_2H = 0x203,
    Loongson_2K = 0x204,
    Loongson_3A = 0x300,
    Loongson_3B = 0x301,
}

/* Capability and feature descriptor structure for MIPS CPU */
#[repr(C, packed)]
pub struct efi_cpuinfo_loongson {
    pub vers: u16, /* version of efi_cpuinfo_loongson */
    pub processor_id: u32, /* PRID, e.g. 6305, 6306 */
    pub cputype: u32, /* Loongson_3A/3B, etc. */
    pub total_node: u32, /* num of total numa nodes */
    pub cpu_startup_core_id: u16, /* Boot core id */
    pub reserved_cores_mask: u16,
    pub cpu_clock_freq: u32, /* cpu_clock */
    pub nr_cpus: u32,
}

pub const MAX_UARTS: usize = 64;
#[repr(C, packed)]
pub struct uart_device {
    pub iotype: u32, /* see include/linux/serial_core.h */
    pub uartclk: u32,
    pub int_offset: u32,
    pub uart_base: u64,
}

pub const MAX_SENSORS: usize = 64;
pub const SENSOR_TEMPER: u32 = 0x00000001;
pub const SENSOR_VOLTAGE: u32 = 0x00000002;
pub const SENSOR_FAN: u32 = 0x00000004;
#[repr(C, packed)]
pub struct sensor_device {
    pub name: [i8; 32], /* a formal name */
    pub label: [i8; 64], /* a flexible description */
    pub r#type: u32, /* SENSOR_* */
    pub id: u32, /* instance id of a sensor-class */
    pub fan_policy: u32, /* see loongson_hwmon.h */
    pub fan_percent: u32, /* only for constant speed policy */
    pub base_addr: u64, /* base address of device registers */
}

#[repr(C, packed)]
pub struct system_loongson {
    pub vers: u16, /* version of system_loongson */
    pub ccnuma_smp: u32, /* 0: no numa; 1: has numa */
    pub sing_double_channel: u32, /* 1:single; 2:double */
    pub nr_uarts: u32,
    pub uarts: [uart_device; MAX_UARTS],
    pub nr_sensors: u32,
    pub sensors: [sensor_device; MAX_SENSORS],
    pub has_ec: i8,
    pub ec_name: [i8; 32],
    pub ec_base_addr: u64,
    pub has_tcm: i8,
    pub tcm_name: [i8; 32],
    pub tcm_base_addr: u64,
    pub workarounds: u64, /* see workarounds.h */
}

#[repr(C, packed)]
pub struct irq_source_routing_table {
    pub vers: u16, pub size: u16, pub rtr_bus: u16, pub rtr_devfn: u16,
    pub vendor: u32, pub device: u32, pub PIC_type: u32, /* conform use HT or PCI to route to CPU-PIC */
    pub ht_int_bit: u64, /* 3A: 1<<24; 3B: 1<<16 */
    pub ht_enable: u64, /* irqs used in this PIC */
    pub node_id: u32, /* node id: 0x0-0; 0x1-1; 0x10-2; 0x11-3 */
    pub pci_mem_start_addr: u64, pub pci_mem_end_addr: u64,
    pub pci_io_start_addr: u64, pub pci_io_end_addr: u64,
    pub pci_config_addr: u64, pub dma_mask_bits: u16, pub dma_noncoherent: u16,
}

#[repr(C, packed)]
pub struct interface_info {
    pub vers: u16, pub size: u16, pub flag: u8, /* used or unused */
    pub description: [i8; 64], /* description for each change */
}

pub const MAX_RESOURCE_NUMBER: usize = 128;
#[repr(C)]
pub struct resource_loongson { pub start: u64, pub end: u64, pub name: [i8; 64], pub flags: u32 }
#[repr(C)]
pub struct archdev_data {} /* arch specific additions */
#[repr(C)]
pub struct board_devices {
    pub name: [i8; 64], pub num_resources: u32,
    pub resource: [resource_loongson; MAX_RESOURCE_NUMBER], pub archdata: archdev_data,
}
#[repr(C)]
pub struct loongson_special_attribute {
    pub vers: u16, pub special_name: [i8; 64], pub loongson_special_type: u32,
    pub resource: [resource_loongson; MAX_RESOURCE_NUMBER],
}
#[repr(C)]
pub struct loongson_params {
    pub memory_offset: u64, pub cpu_offset: u64, pub system_offset: u64, pub irq_offset: u64,
    pub interface_offset: u64, pub special_offset: u64, pub boarddev_table_offset: u64,
}
#[repr(C)]
pub struct smbios_tables { pub vers: u16, pub vga_bios: u64, pub lp: loongson_params }
#[repr(C)]
pub struct efi_reset_system_t {
    pub ResetCold: u64, pub ResetWarm: u64, pub ResetType: u64, pub Shutdown: u64,
    pub DoSuspend: u64, /* NULL if not support */
}
#[repr(C)]
pub struct efi_loongson {
    pub mps: u64, pub acpi: u64, pub acpi20: u64, pub smbios: smbios_tables,
    pub sal_systab: u64, pub boot_info: u64,
}
#[repr(C)]
pub struct boot_params { pub efi: efi_loongson, pub reset_system: efi_reset_system_t }

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum loongson_bridge_type { LS7A = 1, RS780E = 2, VIRTUAL = 3 }

extern "C" {
    pub static mut loongson_memmap: *mut efi_memory_map_loongson;
    pub static mut loongson_sysconf: loongson_system_configuration;
    pub static mut eboard: *mut board_devices;
    pub static mut einter: *mut interface_info;
    pub static mut especial: *mut loongson_special_attribute;
    pub static mut node_id_offset: u32;
    pub fn ls7a_early_config();
    pub fn rs780e_early_config();
    pub fn virtual_early_config();
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
