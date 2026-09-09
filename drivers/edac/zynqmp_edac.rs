// SPDX-License-Identifier: GPL-2.0
/* Xilinx ZynqMP OCM ECC Driver */

// Kernel dependencies supplied by the surrounding repository.

const ZYNQMP_OCM_EDAC_MSG_SIZE: usize = 256;
const ZYNQMP_OCM_EDAC_STRING: &[u8] = b"zynqmp_ocm\0";

const ERR_CTRL_OFST: usize = 0x0;
const OCM_ISR_OFST: usize = 0x04;
const OCM_IMR_OFST: usize = 0x08;
const OCM_IEN_OFST: usize = 0x0C;
const OCM_IDS_OFST: usize = 0x10;
const ECC_CTRL_OFST: usize = 0x14;
const CE_FFA_OFST: usize = 0x1C;
const CE_FFD0_OFST: usize = 0x20;
const CE_FFD1_OFST: usize = 0x24;
const CE_FFD2_OFST: usize = 0x28;
const CE_FFD3_OFST: usize = 0x2C;
const CE_FFE_OFST: usize = 0x30;
const UE_FFA_OFST: usize = 0x34;
const UE_FFD0_OFST: usize = 0x38;
const UE_FFD1_OFST: usize = 0x3C;
const UE_FFD2_OFST: usize = 0x40;
const UE_FFD3_OFST: usize = 0x44;
const UE_FFE_OFST: usize = 0x48;
const ECC_CTRL_CLR_CE_ERR: u32 = 0x40;
const ECC_CTRL_CLR_UE_ERR: u32 = 0x80;
const OCM_FID0_OFST: usize = 0x4C;
const OCM_FID1_OFST: usize = 0x50;
const OCM_FID2_OFST: usize = 0x54;
const OCM_FID3_OFST: usize = 0x58;
const OCM_FIC_OFST: usize = 0x74;
const UE_MAX_BITPOS_LOWER: u8 = 31;
const UE_MIN_BITPOS_UPPER: u8 = 32;
const UE_MAX_BITPOS_UPPER: u8 = 63;
const OCM_CEINTR_MASK: u32 = 1 << 6;
const OCM_UEINTR_MASK: u32 = 1 << 7;
const OCM_ECC_ENABLE_MASK: u32 = 1;
const OCM_FICOUNT_MASK: u32 = 0x00ff_ffff;
const OCM_NUM_UE_BITPOS: usize = 2;
const OCM_BASEVAL: u32 = 0xfffc_0000;
const EDAC_DEVICE: &[u8] = b"ZynqMP-OCM\0";

#[repr(C)]
pub struct EccErrorInfo {
    pub addr: u32,
    pub fault_lo: u32,
    pub fault_hi: u32,
}

#[repr(C)]
pub struct EccStatus {
    pub ce_cnt: u32,
    pub ue_cnt: u32,
    pub ceinfo: EccErrorInfo,
    pub ueinfo: EccErrorInfo,
}

#[repr(C)]
pub struct EdacPriv {
    pub baseaddr: *mut core::ffi::c_void,
    pub message: [core::ffi::c_char; ZYNQMP_OCM_EDAC_MSG_SIZE],
    pub stat: EccStatus,
    pub ce_cnt: u32,
    pub ue_cnt: u32,
    // CONFIG_EDAC_DEBUG fields are retained under the same build-time condition.
    #[cfg(CONFIG_EDAC_DEBUG)]
    pub debugfs_dir: *mut Dentry,
    #[cfg(CONFIG_EDAC_DEBUG)]
    pub ce_bitpos: u8,
    #[cfg(CONFIG_EDAC_DEBUG)]
    pub ue_bitpos: [u8; OCM_NUM_UE_BITPOS],
    #[cfg(CONFIG_EDAC_DEBUG)]
    pub fault_injection_cnt: u32,
}

#[repr(C)]
pub struct EdacDeviceCtlInfo {
    pub pvt_info: *mut EdacPriv,
    pub dev: *mut Device,
    pub mod_name: *const core::ffi::c_char,
    pub ctl_name: *const core::ffi::c_char,
    pub dev_name: *const core::ffi::c_char,
}

#[repr(C)] pub struct PlatformDevice { pub dev: Device }
#[repr(C)] pub struct Device { pub driver: *mut DeviceDriver }
#[repr(C)] pub struct DeviceDriver { pub name: *const core::ffi::c_char }
#[repr(C)] pub struct Resource;
#[repr(C)] pub struct Dentry;
#[repr(C)] pub struct File { pub private_data: *mut core::ffi::c_void }

extern "C" {
    fn readl(addr: *mut u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
    fn memset(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    fn snprintf(s: *mut core::ffi::c_char, n: usize, fmt: *const core::ffi::c_char, ...) -> i32;
    fn edac_device_handle_ce(dci: *mut EdacDeviceCtlInfo, instance: u32, block: u32, msg: *const core::ffi::c_char);
    fn edac_device_handle_ue(dci: *mut EdacDeviceCtlInfo, instance: u32, block: u32, msg: *const core::ffi::c_char);
    fn edac_device_alloc_ctl_info(size: usize, name: *const u8, n: u32, ctl: *const u8, layers: u32, nr: u32, idx: i32) -> *mut EdacDeviceCtlInfo;
    fn edac_device_free_ctl_info(dci: *mut EdacDeviceCtlInfo);
    fn edac_device_add_device(dci: *mut EdacDeviceCtlInfo) -> i32;
    fn edac_device_del_device(dev: *mut Device);
    fn devm_platform_get_and_ioremap_resource(pdev: *mut PlatformDevice, index: u32, res: *mut *mut Resource) -> *mut u8;
    fn platform_get_irq(pdev: *mut PlatformDevice, index: u32) -> i32;
    fn devm_request_irq(dev: *mut Device, irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32, flags: u32, name: *const core::ffi::c_char, data: *mut EdacDeviceCtlInfo) -> i32;
    fn platform_set_drvdata(pdev: *mut PlatformDevice, data: *mut EdacDeviceCtlInfo);
    fn platform_get_drvdata(pdev: *mut PlatformDevice) -> *mut EdacDeviceCtlInfo;
    fn dev_name(dev: *mut Device) -> *const core::ffi::c_char;
    fn edac_device_alloc_index() -> i32;
    fn ptr_err(ptr: *mut u8) -> i32;
    fn is_err(ptr: *mut u8) -> bool;
    fn warn_once(condition: bool, fmt: *const u8, ...);
    fn edac_printk(level: i32, device: *const u8, fmt: *const u8, ...);
}

unsafe fn get_error_info(base: *mut u8, p: *mut EccStatus, mask: i32) {
    if (mask as u32 & OCM_CEINTR_MASK) != 0 {
        (*p).ce_cnt += 1;
        (*p).ceinfo.fault_lo = readl(base.add(CE_FFD0_OFST));
        (*p).ceinfo.fault_hi = readl(base.add(CE_FFD1_OFST));
        (*p).ceinfo.addr = OCM_BASEVAL | readl(base.add(CE_FFA_OFST));
        writel(ECC_CTRL_CLR_CE_ERR, base.add(OCM_ISR_OFST));
    } else if (mask as u32 & OCM_UEINTR_MASK) != 0 {
        (*p).ue_cnt += 1;
        (*p).ueinfo.fault_lo = readl(base.add(UE_FFD0_OFST));
        (*p).ueinfo.fault_hi = readl(base.add(UE_FFD1_OFST));
        (*p).ueinfo.addr = OCM_BASEVAL | readl(base.add(UE_FFA_OFST));
        writel(ECC_CTRL_CLR_UE_ERR, base.add(OCM_ISR_OFST));
    }
}

unsafe fn handle_error(dci: *mut EdacDeviceCtlInfo, p: *mut EccStatus) {
    let priv_ = (*dci).pvt_info;
    let mut pinf: *mut EccErrorInfo;
    if (*p).ce_cnt != 0 {
        pinf = &mut (*p).ceinfo;
        snprintf((*priv_).message.as_mut_ptr(), ZYNQMP_OCM_EDAC_MSG_SIZE, b"\nOCM ECC error type :%s\nAddr: [0x%x]\nFault Data[0x%08x%08x]\0".as_ptr() as *const _, b"CE\0".as_ptr() as *const _, (*pinf).addr, (*pinf).fault_hi, (*pinf).fault_lo);
        edac_device_handle_ce(dci, 0, 0, (*priv_).message.as_ptr());
    }
    if (*p).ue_cnt != 0 {
        pinf = &mut (*p).ueinfo;
        snprintf((*priv_).message.as_mut_ptr(), ZYNQMP_OCM_EDAC_MSG_SIZE, b"\nOCM ECC error type :%s\nAddr: [0x%x]\nFault Data[0x%08x%08x]\0".as_ptr() as *const _, b"UE\0".as_ptr() as *const _, (*pinf).addr, (*pinf).fault_hi, (*pinf).fault_lo);
        edac_device_handle_ue(dci, 0, 0, (*priv_).message.as_ptr());
    }
    memset(p as *mut _, 0, core::mem::size_of::<EccStatus>());
}

pub unsafe extern "C" fn intr_handler(irq: i32, dev_id: *mut core::ffi::c_void) -> i32 {
    let dci = dev_id as *mut EdacDeviceCtlInfo;
    let priv_ = (*dci).pvt_info;
    let regval = readl((*priv_).baseaddr.add(OCM_ISR_OFST));
    if regval & (OCM_CEINTR_MASK | OCM_UEINTR_MASK) == 0 {
        warn_once(true, b"Unhandled IRQ%d, ISR: 0x%x\0".as_ptr(), irq, regval);
        return 0;
    }
    get_error_info((*priv_).baseaddr as *mut u8, &mut (*priv_).stat, regval as i32);
    (*priv_).ce_cnt += (*priv_).stat.ce_cnt;
    (*priv_).ue_cnt += (*priv_).stat.ue_cnt;
    handle_error(dci, &mut (*priv_).stat);
    1
}

unsafe fn get_eccstate(base: *mut u8) -> bool { readl(base.add(ECC_CTRL_OFST)) & OCM_ECC_ENABLE_MASK != 0 }

// CONFIG_EDAC_DEBUG-only fault injection entry points are represented when enabled.
#[cfg(CONFIG_EDAC_DEBUG)]
unsafe fn write_fault_count(priv_: *mut EdacPriv) {
    let mut ficount = (*priv_).fault_injection_cnt;
    if ficount & !OCM_FICOUNT_MASK != 0 { ficount &= OCM_FICOUNT_MASK; }
    writel(ficount, ((*priv_).baseaddr as *mut u8).add(OCM_FIC_OFST));
}

unsafe fn edac_probe(pdev: *mut PlatformDevice) -> i32 {
    let mut res: *mut Resource = core::ptr::null_mut();
    let baseaddr = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if is_err(baseaddr) { return ptr_err(baseaddr); }
    if !get_eccstate(baseaddr) { return -6; /* -ENXIO */ }
    let dci = edac_device_alloc_ctl_info(core::mem::size_of::<EdacPriv>(), ZYNQMP_OCM_EDAC_STRING.as_ptr(), 1, ZYNQMP_OCM_EDAC_STRING.as_ptr(), 1, 0, edac_device_alloc_index());
    if dci.is_null() { return -12; /* -ENOMEM */ }
    let priv_ = (*dci).pvt_info;
    platform_set_drvdata(pdev, dci);
    (*dci).dev = &mut (*pdev).dev;
    (*priv_).baseaddr = baseaddr as *mut core::ffi::c_void;
    (*dci).mod_name = (*(*pdev).dev.driver).name;
    (*dci).ctl_name = ZYNQMP_OCM_EDAC_STRING.as_ptr();
    (*dci).dev_name = dev_name(&mut (*pdev).dev);
    let irq = platform_get_irq(pdev, 0);
    if irq < 0 { edac_device_free_ctl_info(dci); return irq; }
    let ret = devm_request_irq(&mut (*pdev).dev, irq, intr_handler, 0, (*dci).dev_name, dci);
    if ret != 0 { edac_device_free_ctl_info(dci); return ret; }
    writel(OCM_CEINTR_MASK | OCM_UEINTR_MASK, ((*priv_).baseaddr as *mut u8).add(OCM_IEN_OFST));
    let ret = edac_device_add_device(dci);
    if ret != 0 { edac_device_free_ctl_info(dci); }
    ret
}

unsafe fn edac_remove(pdev: *mut PlatformDevice) {
    let dci = platform_get_drvdata(pdev);
    let priv_ = (*dci).pvt_info;
    writel(OCM_CEINTR_MASK | OCM_UEINTR_MASK, ((*priv_).baseaddr as *mut u8).add(OCM_IDS_OFST));
    edac_device_del_device(&mut (*pdev).dev);
    edac_device_free_ctl_info(dci);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
