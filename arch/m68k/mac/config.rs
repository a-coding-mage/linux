/* Rust translation of linux/arch/m68k/mac/config.c. */

// Header-provided types, constants, functions, and globals are intentionally
// left as external dependencies, as in the original implementation.

#[repr(C)]
pub struct MacBooterData {
    pub id: i32, pub videoaddr: u32, pub videodepth: u32, pub videorow: u32,
    pub dimensions: u32, pub videological: u32, pub sccbase: u32,
    pub boottime: u32, pub gmtbias: u32, pub memsize: u32, pub cpuid: u32,
    pub rombase: u32,
}

extern "C" {
    static mut mac_bi_data: MacBooterData;
    static mut macintosh_config: *mut MacModel;
    static mut mac_orig_videoaddr: u32;
    fn via_init_clock();
    fn mac_init_IRQ(); fn mac_hwclk(); fn mac_reset(); fn mac_poweroff();
    fn mac_mksound(); fn mac_identify(); fn mac_report_hardware();
    fn via_l2_flush(); fn register_platform_power_off(f: unsafe extern "C" fn());
    fn iop_init(); fn oss_init(); fn via_init(); fn psc_init(); fn baboon_init();
    fn find_via_cuda(); fn find_via_pmu();
}

#[repr(C)]
pub struct MacModel {
    pub ident: i32, pub name: *const u8, pub adb_type: i32, pub via_type: i32,
    pub scsi_type: i32, pub ide_type: i32, pub scc_type: i32, pub ether_type: i32,
    pub expansion_type: i32, pub floppy_type: i32,
}

#[repr(C)]
pub struct Resource { pub start: usize, pub end: usize, pub flags: usize }
#[repr(C)]
pub struct PlatformDevice {
    pub name: *const u8, pub id: i32, pub num_resources: usize,
    pub resource: *mut Resource,
}

#[no_mangle]
pub static mut macintosh_config_export: *mut MacModel = core::ptr::null_mut();

static mut MAC_DATA_TABLE: [MacModel; 1] = [MacModel {
    ident: -1, name: b"Unknown\0".as_ptr(), adb_type: 0, via_type: 0,
    scsi_type: 0, ide_type: 0, scc_type: 0, ether_type: 0,
    expansion_type: 0, floppy_type: 0,
}];

static mut SCC_A_RSRCS: [Resource; 2] = [Resource { start: 0, end: 0, flags: 0 }, Resource { start: 0, end: 0, flags: 0 }];
static mut SCC_B_RSRCS: [Resource; 2] = [Resource { start: 0, end: 0, flags: 0 }, Resource { start: 0, end: 0, flags: 0 }];

#[no_mangle]
pub static mut scc_a_pdev: PlatformDevice = PlatformDevice { name: b"scc\0".as_ptr(), id: 0, num_resources: 0, resource: core::ptr::null_mut() };
#[no_mangle]
pub static mut scc_b_pdev: PlatformDevice = PlatformDevice { name: b"scc\0".as_ptr(), id: 1, num_resources: 0, resource: core::ptr::null_mut() };

pub unsafe fn mac_parse_bootinfo(record: *const BiRecord) -> i32 {
    let mut unknown = 0;
    let tag = u16::from_be((*record).tag);
    let data = (*record).data as *const u32;
    match tag {
        BI_MAC_MODEL => mac_bi_data.id = u32::from_be(*data) as i32,
        BI_MAC_VADDR => mac_bi_data.videoaddr = u32::from_be(*data),
        BI_MAC_VDEPTH => mac_bi_data.videodepth = u32::from_be(*data),
        BI_MAC_VROW => mac_bi_data.videorow = u32::from_be(*data),
        BI_MAC_VDIM => mac_bi_data.dimensions = u32::from_be(*data),
        BI_MAC_VLOGICAL => { mac_orig_videoaddr = u32::from_be(*data); mac_bi_data.videological = VIDEOMEMBASE + (mac_orig_videoaddr & !VIDEOMEMMASK); }
        BI_MAC_SCCBASE => mac_bi_data.sccbase = u32::from_be(*data),
        BI_MAC_BTIME => mac_bi_data.boottime = u32::from_be(*data),
        BI_MAC_GMTBIAS => mac_bi_data.gmtbias = u32::from_be(*data),
        BI_MAC_MEMSIZE => mac_bi_data.memsize = u32::from_be(*data),
        BI_MAC_CPUID => mac_bi_data.cpuid = u32::from_be(*data),
        BI_MAC_ROMBASE => mac_bi_data.rombase = u32::from_be(*data),
        _ => unknown = 1,
    }
    unknown
}

pub unsafe fn config_mac() {
    if !MACH_IS_MAC { pr_err(b"ERROR: no Mac, but config_mac() called!!\n\0".as_ptr()); }
    mac_identify(); mac_report_hardware();
    if (*macintosh_config).ident == MAC_MODEL_IICI { mach_l2_flush = Some(via_l2_flush); }
    register_platform_power_off(mac_poweroff);
}

pub unsafe fn mac_platform_init() -> i32 {
    if !MACH_IS_MAC { return -ENODEV; }
    platform_device_register(&mut scc_a_pdev); platform_device_register(&mut scc_b_pdev);
    match (*macintosh_config).floppy_type {
        MAC_FLOPPY_QUADRA => register_swim(0x5001e000),
        MAC_FLOPPY_OLD => register_swim(0x50016000),
        MAC_FLOPPY_LC => register_swim(0x50f16000),
        _ => {}
    }
    register_mac_devices();
    0
}

// The following declarations represent the kernel interfaces used by the
// remainder of the source; their definitions are supplied by other files.
extern "C" {
    static mut mach_l2_flush: Option<unsafe extern "C" fn()>;
    fn pr_err(s: *const u8); fn platform_device_register(p: *mut PlatformDevice);
    fn register_swim(base: usize); fn register_mac_devices();
}
extern "C" { static MACH_IS_MAC: bool; static ENODEV: i32; }
extern "C" { static VIDEOMEMBASE: u32; static VIDEOMEMMASK: u32; }
extern "C" { static MAC_MODEL_IICI: i32; static MAC_FLOPPY_QUADRA: i32; static MAC_FLOPPY_OLD: i32; static MAC_FLOPPY_LC: i32; static BI_MAC_MODEL: u16; static BI_MAC_VADDR: u16; static BI_MAC_VDEPTH: u16; static BI_MAC_VROW: u16; static BI_MAC_VDIM: u16; static BI_MAC_VLOGICAL: u16; static BI_MAC_SCCBASE: u16; static BI_MAC_BTIME: u16; static BI_MAC_GMTBIAS: u16; static BI_MAC_MEMSIZE: u16; static BI_MAC_CPUID: u16; static BI_MAC_ROMBASE: u16; }
#[repr(C)] pub struct BiRecord { pub tag: u16, pub data: *const u8 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
