/* RNG driver for Intel RNGs; direct translation of intel-rng.c. */

const PFX: &str = "intel: ";

const INTEL_RNG_HW_STATUS: usize = 0;
const INTEL_RNG_PRESENT: u8 = 0x40;
const INTEL_RNG_ENABLED: u8 = 0x01;
const INTEL_RNG_STATUS: usize = 1;
const INTEL_RNG_DATA_PRESENT: u8 = 0x01;
const INTEL_RNG_DATA: usize = 2;

const INTEL_RNG_ADDR: usize = 0xffbc015f;
const INTEL_RNG_ADDR_LEN: usize = 3;

const FWH_DEC_EN1_REG_OLD: u8 = 0xe3;
const FWH_DEC_EN1_REG_NEW: u8 = 0xd9;
const FWH_F8_EN_MASK: u8 = 0x80;
const BIOS_CNTL_REG_OLD: u8 = 0x4e;
const BIOS_CNTL_REG_NEW: u8 = 0xdc;
const BIOS_CNTL_WRITE_ENABLE_MASK: u8 = 0x01;
const BIOS_CNTL_LOCK_ENABLE_MASK: u8 = 0x02;

const INTEL_FWH_ADDR: usize = 0xffff0000;
const INTEL_FWH_ADDR_LEN: usize = 2;
const INTEL_FWH_RESET_CMD: u8 = 0xff;
const INTEL_FWH_READ_ID_CMD: u8 = 0x90;
const INTEL_FWH_MANUFACTURER_CODE_ADDRESS: usize = 0x000000;
const INTEL_FWH_DEVICE_CODE_ADDRESS: usize = 0x000001;
const INTEL_FWH_MANUFACTURER_CODE: u8 = 0x89;
const INTEL_FWH_DEVICE_CODE_8M: u8 = 0xac;
const INTEL_FWH_DEVICE_CODE_4M: u8 = 0xad;

#[repr(C)]
pub struct PciDeviceId { pub vendor: u16, pub device: u16 }

static PCI_TBL: &[PciDeviceId] = &[
    PciDeviceId { vendor: 0x8086, device: 0x2410 },
    PciDeviceId { vendor: 0x8086, device: 0x2420 },
    PciDeviceId { vendor: 0x8086, device: 0x244c },
    PciDeviceId { vendor: 0x8086, device: 0x248c },
    PciDeviceId { vendor: 0x8086, device: 0x24cc },
    PciDeviceId { vendor: 0x8086, device: 0x2641 },
    PciDeviceId { vendor: 0x8086, device: 0x27b9 },
    PciDeviceId { vendor: 0x8086, device: 0x27bd },
    PciDeviceId { vendor: 0x8086, device: 0x2440 },
    PciDeviceId { vendor: 0x8086, device: 0x2480 },
    PciDeviceId { vendor: 0x8086, device: 0x24c0 },
    PciDeviceId { vendor: 0x8086, device: 0x24d0 },
    PciDeviceId { vendor: 0x8086, device: 0x25a1 },
    PciDeviceId { vendor: 0x8086, device: 0x2640 },
    PciDeviceId { vendor: 0x8086, device: 0x2670 }, PciDeviceId { vendor: 0x8086, device: 0x2671 },
    PciDeviceId { vendor: 0x8086, device: 0x2672 }, PciDeviceId { vendor: 0x8086, device: 0x2673 },
    PciDeviceId { vendor: 0x8086, device: 0x2674 }, PciDeviceId { vendor: 0x8086, device: 0x2675 },
    PciDeviceId { vendor: 0x8086, device: 0x2676 }, PciDeviceId { vendor: 0x8086, device: 0x2677 },
    PciDeviceId { vendor: 0x8086, device: 0x2678 }, PciDeviceId { vendor: 0x8086, device: 0x2679 },
    PciDeviceId { vendor: 0x8086, device: 0x267a }, PciDeviceId { vendor: 0x8086, device: 0x267b },
    PciDeviceId { vendor: 0x8086, device: 0x267c }, PciDeviceId { vendor: 0x8086, device: 0x267d },
    PciDeviceId { vendor: 0x8086, device: 0x267e }, PciDeviceId { vendor: 0x8086, device: 0x267f },
    PciDeviceId { vendor: 0x8086, device: 0x27b8 },
    PciDeviceId { vendor: 0x8086, device: 0x2450 },
    PciDeviceId { vendor: 0, device: 0 },
];

extern "C" {
    fn readb(addr: *mut u8) -> u8;
    fn writeb(value: u8, addr: *mut u8);
    fn udelay(usecs: u32);
    fn pci_write_config_byte(dev: *mut PciDev, where_: u8, value: u8);
    fn pci_read_config_byte(dev: *mut PciDev, where_: u8, value: *mut u8);
    fn pci_get_device(vendor: u16, device: u16, from: *mut PciDev) -> *mut PciDev;
    fn pci_dev_put(dev: *mut PciDev);
    fn ioremap(addr: usize, len: usize) -> *mut u8;
    fn iounmap(addr: *mut u8);
    fn stop_machine(fn_: unsafe extern "C" fn(*mut core::ffi::c_void) -> i32, data: *mut core::ffi::c_void, cpus: *mut core::ffi::c_void) -> i32;
    fn kmalloc(size: usize) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn pr_err(_: *const u8, ...);
    fn pr_warn(_: *const u8, ...);
    fn pr_notice(_: *const u8, ...);
    fn pr_info(_: *const u8, ...);
}

#[repr(C)] pub struct PciDev { pub device: u16 }
#[repr(C)] pub struct Hwrng { pub name: *const u8, pub init: Option<unsafe extern "C" fn(*mut Hwrng) -> i32>, pub cleanup: Option<unsafe extern "C" fn(*mut Hwrng)>, pub data_present: Option<unsafe extern "C" fn(*mut Hwrng, i32) -> i32>, pub data_read: Option<unsafe extern "C" fn(*mut Hwrng, *mut u32) -> i32>, pub priv_: usize }

unsafe fn hwstatus_get(mem: *mut u8) -> u8 { readb(mem.add(INTEL_RNG_HW_STATUS)) }
unsafe fn hwstatus_set(mem: *mut u8, status: u8) -> u8 { writeb(status, mem.add(INTEL_RNG_HW_STATUS)); hwstatus_get(mem) }

unsafe extern "C" fn intel_rng_data_present(rng: *mut Hwrng, wait: i32) -> i32 {
    let mem = (*rng).priv_ as *mut u8; let mut data = 0;
    for _ in 0..20 { data = (readb(mem.add(INTEL_RNG_STATUS)) & INTEL_RNG_DATA_PRESENT != 0) as i32; if data != 0 || wait == 0 { break; } udelay(10); } data
}
unsafe extern "C" fn intel_rng_data_read(rng: *mut Hwrng, data: *mut u32) -> i32 { *data = readb(((*rng).priv_ as *mut u8).add(INTEL_RNG_DATA)) as u32; 1 }
unsafe extern "C" fn intel_rng_init(rng: *mut Hwrng) -> i32 { let mem = (*rng).priv_ as *mut u8; let mut s = hwstatus_get(mem); if s & INTEL_RNG_ENABLED == 0 { s = hwstatus_set(mem, s | INTEL_RNG_ENABLED); } if s & INTEL_RNG_ENABLED == 0 { return -5; } 0 }
unsafe extern "C" fn intel_rng_cleanup(rng: *mut Hwrng) { let mem = (*rng).priv_ as *mut u8; let s = hwstatus_get(mem); if s & INTEL_RNG_ENABLED != 0 { hwstatus_set(mem, s & !INTEL_RNG_ENABLED); } }

#[repr(C)] struct IntelRngHw { dev: *mut PciDev, mem: *mut u8, bios_cntl_off: u8, bios_cntl_val: u8, fwh_dec_en1_off: u8, fwh_dec_en1_val: u8 }

static mut NO_FWH_DETECT: i32 = 0;
static mut INTEL_RNG: Hwrng = Hwrng { name: b"intel\0".as_ptr(), init: Some(intel_rng_init), cleanup: Some(intel_rng_cleanup), data_present: Some(intel_rng_data_present), data_read: Some(intel_rng_data_read), priv_: 0 };

unsafe extern "C" fn intel_rng_hw_init(arg: *mut core::ffi::c_void) -> i32 {
    let hw = arg as *mut IntelRngHw; let mut mfc; let mut dvc;
    if (*hw).fwh_dec_en1_val & FWH_F8_EN_MASK == 0 { pci_write_config_byte((*hw).dev, (*hw).fwh_dec_en1_off, (*hw).fwh_dec_en1_val | FWH_F8_EN_MASK); }
    if (*hw).bios_cntl_val & BIOS_CNTL_WRITE_ENABLE_MASK == 0 { pci_write_config_byte((*hw).dev, (*hw).bios_cntl_off, (*hw).bios_cntl_val | BIOS_CNTL_WRITE_ENABLE_MASK); }
    writeb(INTEL_FWH_RESET_CMD, (*hw).mem); writeb(INTEL_FWH_READ_ID_CMD, (*hw).mem);
    mfc = readb((*hw).mem.add(INTEL_FWH_MANUFACTURER_CODE_ADDRESS)); dvc = readb((*hw).mem.add(INTEL_FWH_DEVICE_CODE_ADDRESS)); writeb(INTEL_FWH_RESET_CMD, (*hw).mem);
    if (*hw).bios_cntl_val & (BIOS_CNTL_LOCK_ENABLE_MASK | BIOS_CNTL_WRITE_ENABLE_MASK) == 0 { pci_write_config_byte((*hw).dev, (*hw).bios_cntl_off, (*hw).bios_cntl_val); }
    if (*hw).fwh_dec_en1_val & FWH_F8_EN_MASK == 0 { pci_write_config_byte((*hw).dev, (*hw).fwh_dec_en1_off, (*hw).fwh_dec_en1_val); }
    if mfc != INTEL_FWH_MANUFACTURER_CODE || (dvc != INTEL_FWH_DEVICE_CODE_8M && dvc != INTEL_FWH_DEVICE_CODE_4M) { return -19; } 0
}

unsafe fn intel_init_hw_struct(hw: *mut IntelRngHw, dev: *mut PciDev) -> i32 {
    (*hw).bios_cntl_val = 0xff; (*hw).fwh_dec_en1_val = 0xff; (*hw).dev = dev;
    if (*dev).device < 0x2640 { (*hw).fwh_dec_en1_off = FWH_DEC_EN1_REG_OLD; (*hw).bios_cntl_off = BIOS_CNTL_REG_OLD; } else { (*hw).fwh_dec_en1_off = FWH_DEC_EN1_REG_NEW; (*hw).bios_cntl_off = BIOS_CNTL_REG_NEW; }
    pci_read_config_byte(dev, (*hw).fwh_dec_en1_off, &mut (*hw).fwh_dec_en1_val); pci_read_config_byte(dev, (*hw).bios_cntl_off, &mut (*hw).bios_cntl_val);
    if (*hw).bios_cntl_val & (BIOS_CNTL_LOCK_ENABLE_MASK | BIOS_CNTL_WRITE_ENABLE_MASK) == BIOS_CNTL_LOCK_ENABLE_MASK { if NO_FWH_DETECT != 0 { return -19; } return -16; }
    (*hw).mem = ioremap(INTEL_FWH_ADDR, INTEL_FWH_ADDR_LEN); if (*hw).mem.is_null() { return -16; } 0
}

#[no_mangle] pub unsafe extern "C" fn intel_rng_mod_init() -> i32 {
    let mut dev = core::ptr::null_mut(); let mut i = 0;
    while dev.is_null() && PCI_TBL[i].vendor != 0 { dev = pci_get_device(PCI_TBL[i].vendor, PCI_TBL[i].device, core::ptr::null_mut()); i += 1; }
    if dev.is_null() { return -19; }
    if NO_FWH_DETECT < 0 { pci_dev_put(dev); } else { let hw = kmalloc(core::mem::size_of::<IntelRngHw>()) as *mut IntelRngHw; if hw.is_null() { pci_dev_put(dev); return -12; } let err = intel_init_hw_struct(hw, dev); if err != 0 { pci_dev_put(dev); kfree(hw as *mut _); if err != -19 { return err; } } else { let err = stop_machine(intel_rng_hw_init, hw as *mut _, core::ptr::null_mut()); pci_dev_put(dev); iounmap((*hw).mem); kfree(hw as *mut _); if err != 0 { return err; } } }
    let mem = ioremap(INTEL_RNG_ADDR, INTEL_RNG_ADDR_LEN); if mem.is_null() { return -12; } INTEL_RNG.priv_ = mem as usize; if hwstatus_get(mem) & INTEL_RNG_PRESENT == 0 { iounmap(mem); return -19; } 0
}
#[no_mangle] pub unsafe extern "C" fn intel_rng_mod_exit() { let mem = INTEL_RNG.priv_ as *mut u8; iounmap(mem); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
