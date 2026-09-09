/* SPDX-License-Identifier: GPL-2.0 */

/* This header is kernel-only; the user-space portion contains no declarations. */

use core::ffi::c_void;

pub type dma_addr_t = u64;

#[repr(C)]
pub struct task_struct;
#[repr(C)]
pub struct mm_struct;
#[repr(C)]
pub struct vm_area_struct;
#[repr(C)]
pub struct linux_hose_info;
#[repr(C)]
pub struct pci_dev;
#[repr(C)]
pub struct pci_ops;
#[repr(C)]
pub struct pci_controller;
#[repr(C)]
pub struct _alpha_agp_info;
#[repr(C)]
pub struct rtc_time;

#[repr(C)]
pub struct alpha_machine_vector {
    /* These two slots are kept at the beginning for entry.S. */
    pub hae_cache: c_ulong,
    pub hae_register: *mut c_ulong,

    pub nr_irqs: i32,
    pub rtc_port: i32,
    pub rtc_boot_cpu_only: i32,
    pub max_asn: u32,
    pub max_isa_dma_address: c_ulong,
    pub irq_probe_mask: c_ulong,
    pub iack_sc: c_ulong,
    pub min_io_address: c_ulong,
    pub min_mem_address: c_ulong,
    pub pci_dac_offset: c_ulong,

    pub mv_pci_tbi: Option<unsafe extern "C" fn(*mut pci_controller, dma_addr_t, dma_addr_t)>,

    pub mv_ioread8: Option<unsafe extern "C" fn(*const c_void) -> u8>,
    pub mv_ioread16: Option<unsafe extern "C" fn(*const c_void) -> u16>,
    pub mv_ioread32: Option<unsafe extern "C" fn(*const c_void) -> u32>,
    pub mv_ioread64: Option<unsafe extern "C" fn(*const c_void) -> u64>,

    pub mv_iowrite8: Option<unsafe extern "C" fn(u8, *mut c_void)>,
    pub mv_iowrite16: Option<unsafe extern "C" fn(u16, *mut c_void)>,
    pub mv_iowrite32: Option<unsafe extern "C" fn(u32, *mut c_void)>,
    pub mv_iowrite64: Option<unsafe extern "C" fn(u64, *mut c_void)>,

    pub mv_readb: Option<unsafe extern "C" fn(*const c_void) -> u8>,
    pub mv_readw: Option<unsafe extern "C" fn(*const c_void) -> u16>,
    pub mv_readl: Option<unsafe extern "C" fn(*const c_void) -> u32>,
    pub mv_readq: Option<unsafe extern "C" fn(*const c_void) -> u64>,

    pub mv_writeb: Option<unsafe extern "C" fn(u8, *mut c_void)>,
    pub mv_writew: Option<unsafe extern "C" fn(u16, *mut c_void)>,
    pub mv_writel: Option<unsafe extern "C" fn(u32, *mut c_void)>,
    pub mv_writeq: Option<unsafe extern "C" fn(u64, *mut c_void)>,

    pub mv_ioportmap: Option<unsafe extern "C" fn(c_ulong) -> *mut c_void>,
    pub mv_ioremap: Option<unsafe extern "C" fn(c_ulong, c_ulong) -> *mut c_void>,
    pub mv_iounmap: Option<unsafe extern "C" fn(*mut c_void)>,
    pub mv_is_ioaddr: Option<unsafe extern "C" fn(c_ulong) -> i32>,
    pub mv_is_mmio: Option<unsafe extern "C" fn(*const c_void) -> i32>,

    pub update_irq_hw: Option<unsafe extern "C" fn(c_ulong, c_ulong, i32)>,
    pub ack_irq: Option<unsafe extern "C" fn(c_ulong)>,
    pub device_interrupt: Option<unsafe extern "C" fn(c_ulong)>,
    pub machine_check: Option<unsafe extern "C" fn(c_ulong, c_ulong)>,

    pub smp_callin: Option<unsafe extern "C" fn()>,
    pub init_arch: Option<unsafe extern "C" fn()>,
    pub init_irq: Option<unsafe extern "C" fn()>,
    pub init_rtc: Option<unsafe extern "C" fn()>,
    pub init_pci: Option<unsafe extern "C" fn()>,
    pub kill_arch: Option<unsafe extern "C" fn(i32)>,

    pub pci_swizzle: Option<unsafe extern "C" fn(*mut pci_dev, *mut u8) -> u8>,
    pub pci_map_irq: Option<unsafe extern "C" fn(*const pci_dev, u8, u8) -> i32>,
    pub pci_ops: *mut pci_ops,

    pub agp_info: Option<unsafe extern "C" fn() -> *mut _alpha_agp_info>,
    pub vector_name: *const u8,
    pub sys: alpha_machine_vector_sys,
}

pub type c_ulong = usize;

#[repr(C)]
pub union alpha_machine_vector_sys {
    pub cia: alpha_machine_vector_sys_cia,
    pub t2: alpha_machine_vector_sys_t2,
    pub sio: alpha_machine_vector_sys_sio,
}

#[repr(C)]
pub struct alpha_machine_vector_sys_cia { pub gru_int_req_bits: c_ulong }
#[repr(C)]
pub struct alpha_machine_vector_sys_t2 { pub gamma_bias: c_ulong }
#[repr(C)]
pub struct alpha_machine_vector_sys_sio { pub route_tab: u32 }

unsafe extern "C" {
    pub static mut alpha_mv: alpha_machine_vector;
}

/* CONFIG_ALPHA_GENERIC supplies variables; otherwise these are build-time constants. */
#[cfg(CONFIG_ALPHA_GENERIC)]
unsafe extern "C" {
    pub static mut alpha_using_srm: i32;
    pub static mut alpha_using_qemu: i32;
}

#[cfg(not(CONFIG_ALPHA_GENERIC))]
pub const alpha_using_srm: i32 = if cfg!(CONFIG_ALPHA_SRM) { 1 } else { 0 };
#[cfg(not(CONFIG_ALPHA_GENERIC))]
pub const alpha_using_qemu: i32 = if cfg!(CONFIG_ALPHA_QEMU) { 1 } else { 0 };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
