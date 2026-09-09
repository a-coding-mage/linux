/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from asm/ropes.h. */

/* #include <asm/parisc-device.h> */

#[cfg(target_pointer_width = "64")]
pub const ZX1_SUPPORT: bool = true;

pub const DELAYED_RESOURCE_CNT: usize = 16;
pub const MAX_IOC: usize = 2;
pub const ROPES_PER_IOC: usize = 8;

#[repr(C)]
pub struct Ioc {
    pub ioc_hpa: *mut core::ffi::c_void,
    pub res_map: *mut core::ffi::c_char,
    pub pdir_base: *mut __le64,
    pub ibase: c_ulong,
    pub imask: c_ulong,
    #[cfg(target_pointer_width = "64")]
    pub iovp_mask: c_ulong,
    pub res_hint: *mut c_ulong,
    pub res_lock: spinlock_t,
    pub res_bitshift: c_uint,
    pub res_size: c_uint,
    #[cfg(SBA_HINT_SUPPORT)]
    pub hint_mask_pdir: c_ulong,
    #[cfg(SBA_HINT_SUPPORT)]
    pub hint_shift_pdir: c_uint,
    pub saved_cnt: c_int,
    pub saved: [SbaDmaPair; DELAYED_RESOURCE_CNT],
    #[cfg(SBA_COLLECT_STATS)]
    pub avg_search: [c_ulong; SBA_SEARCH_SAMPLE],
    #[cfg(SBA_COLLECT_STATS)]
    pub avg_idx: c_ulong,
    #[cfg(SBA_COLLECT_STATS)]
    pub used_pages: c_ulong,
    #[cfg(SBA_COLLECT_STATS)]
    pub msingle_calls: c_ulong,
    #[cfg(SBA_COLLECT_STATS)]
    pub msingle_pages: c_ulong,
    #[cfg(SBA_COLLECT_STATS)]
    pub msg_calls: c_ulong,
    #[cfg(SBA_COLLECT_STATS)]
    pub msg_pages: c_ulong,
    #[cfg(SBA_COLLECT_STATS)]
    pub usingle_calls: c_ulong,
    #[cfg(SBA_COLLECT_STATS)]
    pub usingle_pages: c_ulong,
    #[cfg(SBA_COLLECT_STATS)]
    pub usg_calls: c_ulong,
    #[cfg(SBA_COLLECT_STATS)]
    pub usg_pages: c_ulong,
    pub pdir_size: c_uint,
}

#[repr(C)]
pub struct SbaDmaPair {
    pub iova: dma_addr_t,
    pub size: usize,
}

#[repr(C)]
pub struct SbaDevice {
    pub next: *mut SbaDevice,
    pub dev: *mut parisc_device,
    pub name: *const core::ffi::c_char,
    pub sba_hpa: *mut core::ffi::c_void,
    pub sba_lock: spinlock_t,
    pub flags: c_uint,
    pub hw_rev: c_uint,
    pub chip_resv: resource,
    pub iommu_resv: resource,
    pub num_ioc: c_uint,
    pub ioc: [Ioc; MAX_IOC],
}

pub static mut sba_list: *mut SbaDevice = core::ptr::null_mut();

pub const ASTRO_RUNWAY_PORT: c_uint = 0x582;
pub const IKE_MERCED_PORT: c_uint = 0x803;
pub const REO_MERCED_PORT: c_uint = 0x804;
pub const REOG_MERCED_PORT: c_uint = 0x805;
pub const PLUTO_MCKINLEY_PORT: c_uint = 0x880;

#[inline]
pub unsafe fn IS_ASTRO(d: *mut parisc_device) -> c_int { (unsafe { (*d).id.hversion == ASTRO_RUNWAY_PORT }) as c_int }
#[inline]
pub unsafe fn IS_IKE(d: *mut parisc_device) -> c_int { (unsafe { (*d).id.hversion == IKE_MERCED_PORT }) as c_int }
#[inline]
pub unsafe fn IS_PLUTO(d: *mut parisc_device) -> c_int { (unsafe { (*d).id.hversion == PLUTO_MCKINLEY_PORT }) as c_int }

pub const PLUTO_IOVA_BASE: c_ulong = 1 * 1024 * 1024 * 1024;
pub const PLUTO_IOVA_SIZE: c_ulong = 1 * 1024 * 1024 * 1024;
pub const PLUTO_GART_SIZE: c_ulong = PLUTO_IOVA_SIZE / 2;
pub const SBA_PDIR_VALID_BIT: u64 = 0x8000000000000000;
pub const SBA_AGPGART_COOKIE: __le64 = 0x0000badbadc0ffee;
pub const SBA_FUNC_ID: c_uint = 0x0000;
pub const SBA_FCLASS: c_uint = 0x0008;
pub const SBA_FUNC_SIZE: c_uint = 4096;
pub const ASTRO_IOC_OFFSET: c_uint = 32 * SBA_FUNC_SIZE;
pub const PLUTO_IOC_OFFSET: c_uint = SBA_FUNC_SIZE;
#[inline] pub const fn IKE_IOC_OFFSET(p: c_uint) -> c_uint { (p + 2) * SBA_FUNC_SIZE }

pub const IOC_CTRL: c_uint = 0x8;
pub const SBA_SEARCH_SAMPLE: usize = 0x100;
pub const IOC_CTRL_TC: c_uint = 1 << 0;
pub const IOC_CTRL_CE: c_uint = 1 << 1;
pub const IOC_CTRL_DE: c_uint = 1 << 2;
pub const IOC_CTRL_RM: c_uint = 1 << 8;
pub const IOC_CTRL_NC: c_uint = 1 << 9;
pub const IOC_CTRL_D4: c_uint = 1 << 11;
pub const IOC_CTRL_DD: c_uint = 1 << 13;

pub const LMMIO_DIRECT0_BASE: c_uint = 0x300;
pub const LMMIO_DIRECT0_MASK: c_uint = 0x308;
pub const LMMIO_DIRECT0_ROUTE: c_uint = 0x310;
pub const LMMIO_DIST_BASE: c_uint = 0x360;
pub const LMMIO_DIST_MASK: c_uint = 0x368;
pub const LMMIO_DIST_ROUTE: c_uint = 0x370;
pub const IOS_DIST_BASE: c_uint = 0x390;
pub const IOS_DIST_MASK: c_uint = 0x398;
pub const IOS_DIST_ROUTE: c_uint = 0x3A0;
pub const IOS_DIRECT_BASE: c_uint = 0x3C0;
pub const IOS_DIRECT_MASK: c_uint = 0x3C8;
pub const IOS_DIRECT_ROUTE: c_uint = 0x3D0;

pub const ROPE0_CTL: c_uint = 0x200; pub const ROPE1_CTL: c_uint = 0x208;
pub const ROPE2_CTL: c_uint = 0x210; pub const ROPE3_CTL: c_uint = 0x218;
pub const ROPE4_CTL: c_uint = 0x220; pub const ROPE5_CTL: c_uint = 0x228;
pub const ROPE6_CTL: c_uint = 0x230; pub const ROPE7_CTL: c_uint = 0x238;
pub const IOC_ROPE0_CFG: c_uint = 0x500;
pub const IOC_ROPE_AO: c_uint = 0x10;
pub const HF_ENABLE: c_uint = 0x40;
pub const IOC_IBASE: c_uint = 0x300; pub const IOC_IMASK: c_uint = 0x308;
pub const IOC_PCOM: c_uint = 0x310; pub const IOC_TCNFG: c_uint = 0x318;
pub const IOC_PDIR_BASE: c_uint = 0x320;
pub const IOVP_SIZE: usize = PAGE_SIZE;
pub const IOVP_SHIFT: usize = PAGE_SHIFT;
pub const IOVP_MASK: usize = PAGE_MASK;
pub const SBA_PERF_CFG: c_uint = 0x708; pub const SBA_PERF_MASK1: c_uint = 0x718;
pub const SBA_PERF_MASK2: c_uint = 0x730;
pub const SBA_PERF_CNT1: c_uint = 0x200; pub const SBA_PERF_CNT2: c_uint = 0x208;
pub const SBA_PERF_CNT3: c_uint = 0x210;

#[repr(C)]
pub struct LbaDevice {
    pub hba: pci_hba_data,
    pub lba_lock: spinlock_t,
    pub iosapic_obj: *mut core::ffi::c_void,
    #[cfg(target_pointer_width = "64")]
    pub iop_base: *mut core::ffi::c_void,
    pub flags: c_int,
    pub hw_rev: c_int,
}

pub const ELROY_HVERS: c_uint = 0x782;
pub const MERCURY_HVERS: c_uint = 0x783;
pub const QUICKSILVER_HVERS: c_uint = 0x784;
#[inline] pub unsafe fn IS_ELROY(d: *mut parisc_device) -> c_int { (unsafe { (*d).id.hversion == ELROY_HVERS }) as c_int }
#[inline] pub unsafe fn IS_MERCURY(d: *mut parisc_device) -> c_int { (unsafe { (*d).id.hversion == MERCURY_HVERS }) as c_int }
#[inline] pub unsafe fn IS_QUICKSILVER(d: *mut parisc_device) -> c_int { (unsafe { (*d).id.hversion == QUICKSILVER_HVERS }) as c_int }

pub unsafe fn agp_mode_mercury(hpa: *mut core::ffi::c_void) -> c_int {
    let bus_mode: u64 = readl((hpa as *mut u8).add(0x0620) as *const core::ffi::c_void);
    (bus_mode & 1) as c_int
}

unsafe extern "C" {
    pub fn iosapic_register(hpa: c_ulong, vaddr: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    pub fn iosapic_fixup_irq(obj: *mut core::ffi::c_void, pcidev: *mut pci_dev) -> c_int;
}

pub const LBA_FUNC_ID: c_uint = 0x0000; pub const LBA_FCLASS: c_uint = 0x0008;
pub const LBA_CAPABLE: c_uint = 0x0030; pub const LBA_PCI_CFG_ADDR: c_uint = 0x0040;
pub const LBA_PCI_CFG_DATA: c_uint = 0x0048; pub const LBA_PMC_MTLT: c_uint = 0x0050;
pub const LBA_FW_SCRATCH: c_uint = 0x0058; pub const LBA_ERROR_ADDR: c_uint = 0x0070;
pub const LBA_ARB_MASK: c_uint = 0x0080; pub const LBA_ARB_PRI: c_uint = 0x0088;
pub const LBA_ARB_MODE: c_uint = 0x0090; pub const LBA_ARB_MTLT: c_uint = 0x0098;
pub const LBA_MOD_ID: c_uint = 0x0100; pub const LBA_STAT_CTL: c_uint = 0x0108;
pub const LBA_BUS_RESET: c_uint = 0x01; pub const CLEAR_ERRLOG: c_uint = 0x10;
pub const CLEAR_ERRLOG_ENABLE: c_uint = 0x20;
pub const LBA_LMMIO_BASE: c_uint = 0x0200; pub const LBA_LMMIO_MASK: c_uint = 0x0208;
pub const LBA_GMMIO_BASE: c_uint = 0x0210; pub const LBA_GMMIO_MASK: c_uint = 0x0218;
pub const LBA_WLMMIO_BASE: c_uint = 0x0220; pub const LBA_WLMMIO_MASK: c_uint = 0x0228;
pub const LBA_WGMMIO_BASE: c_uint = 0x0230; pub const LBA_WGMMIO_MASK: c_uint = 0x0238;
pub const LBA_IOS_BASE: c_uint = 0x0240; pub const LBA_IOS_MASK: c_uint = 0x0248;
pub const LBA_ELMMIO_BASE: c_uint = 0x0250; pub const LBA_ELMMIO_MASK: c_uint = 0x0258;
pub const LBA_EIOS_BASE: c_uint = 0x0260; pub const LBA_EIOS_MASK: c_uint = 0x0268;
pub const LBA_GLOBAL_MASK: c_uint = 0x0270; pub const LBA_DMA_CTL: c_uint = 0x0278;
pub const LBA_IBASE: c_uint = 0x0300; pub const LBA_IMASK: c_uint = 0x0308;
pub const LBA_HINT_CFG: c_uint = 0x0310; pub const LBA_HINT_BASE: c_uint = 0x0380;
pub const LBA_BUS_MODE: c_uint = 0x0620; pub const LBA_ERROR_CONFIG: c_uint = 0x0680;
pub const LBA_SMART_MODE: c_uint = 0x20; pub const LBA_ERROR_STATUS: c_uint = 0x0688;
pub const LBA_ROPE_CTL: c_uint = 0x06A0; pub const LBA_IOSAPIC_BASE: c_uint = 0x800;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
