/* Intel D82875P Memory Controller kernel module -- Rust translation. */

const EDAC_MOD_STR: &str = "i82875p_edac";
const PCI_DEVICE_ID_INTEL_82875_0: u16 = 0x2578;
const PCI_DEVICE_ID_INTEL_82875_6: u16 = 0x257e;
const I82875P_NR_DIMMS: usize = 8;
const I82875P_EAP: u16 = 0x58;
const I82875P_DERRSYN: u16 = 0x5c;
const I82875P_DES: u16 = 0x5d;
const I82875P_ERRSTS: u16 = 0xc8;
const I82875P_ERRCMD: u16 = 0xca;
const I82875P_PCICMD6: u16 = 0x04;
const I82875P_BAR6: u16 = 0x10;
const I82875P_DRB_SHIFT: usize = 26;
const I82875P_DRB: usize = 0x00;
const I82875P_DRA: usize = 0x10;
const I82875P_DRC: usize = 0x68;

#[repr(C)]
#[derive(Copy, Clone)]
enum I82875pChips { I82875P = 0 }

#[repr(C)]
pub struct I82875pPvt {
    pub ovrfl_pdev: *mut pci_dev,
    pub ovrfl_window: *mut core::ffi::c_void,
}

#[repr(C)]
struct I82875pDevInfo { ctl_name: *const core::ffi::c_char }

#[repr(C)]
struct I82875pErrorInfo {
    errsts: u16, eap: u32, des: u8, derrsyn: u8, errsts2: u16,
}

static I82875P_DEVS: [I82875pDevInfo; 1] = [I82875pDevInfo { ctl_name: b"i82875p\0".as_ptr() as *const _ }];
static mut MCI_PDEV: *mut pci_dev = core::ptr::null_mut();
static mut I82875P_PCI: *mut edac_pci_ctl_info = core::ptr::null_mut();

unsafe fn i82875p_get_error_info(mci: *mut mem_ctl_info, info: *mut I82875pErrorInfo) {
    let pdev = to_pci_dev((*mci).pdev);
    pci_read_config_word(pdev, I82875P_ERRSTS, &mut (*info).errsts);
    if (*info).errsts & 0x0081 == 0 { return; }
    pci_read_config_dword(pdev, I82875P_EAP, &mut (*info).eap);
    pci_read_config_byte(pdev, I82875P_DES, &mut (*info).des);
    pci_read_config_byte(pdev, I82875P_DERRSYN, &mut (*info).derrsyn);
    pci_read_config_word(pdev, I82875P_ERRSTS, &mut (*info).errsts2);
    if ((*info).errsts ^ (*info).errsts2) & 0x0081 != 0 {
        pci_read_config_dword(pdev, I82875P_EAP, &mut (*info).eap);
        pci_read_config_byte(pdev, I82875P_DES, &mut (*info).des);
        pci_read_config_byte(pdev, I82875P_DERRSYN, &mut (*info).derrsyn);
    }
    pci_write_bits16(pdev, I82875P_ERRSTS, 0x0081, 0x0081);
}

unsafe fn i82875p_process_error_info(mci: *mut mem_ctl_info, info: *mut I82875pErrorInfo, handle_errors: i32) -> i32 {
    let multi_chan = (*(*mci).csrows).nr_channels - 1;
    if (*info).errsts & 0x0081 == 0 { return 0; }
    if handle_errors == 0 { return 1; }
    if ((*info).errsts ^ (*info).errsts2) & 0x0081 != 0 {
        edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, 1, 0, 0, 0, -1, -1, -1, b"UE overwrote CE\0".as_ptr() as _, b"\0".as_ptr() as _);
        (*info).errsts = (*info).errsts2;
    }
    (*info).eap >>= PAGE_SHIFT;
    let row = edac_mc_find_csrow_by_page(mci, (*info).eap);
    if (*info).errsts & 0x0080 != 0 {
        edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, 1, (*info).eap, 0, 0, row, -1, -1, b"i82875p UE\0".as_ptr() as _, b"\0".as_ptr() as _);
    } else {
        edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, 1, (*info).eap, 0, (*info).derrsyn, row, if multi_chan != 0 { ((*info).des & 1) as i32 } else { 0 }, -1, b"i82875p CE\0".as_ptr() as _, b"\0".as_ptr() as _);
    }
    1
}

unsafe extern "C" fn i82875p_check(mci: *mut mem_ctl_info) {
    let mut info = core::mem::MaybeUninit::<I82875pErrorInfo>::uninit();
    i82875p_get_error_info(mci, info.as_mut_ptr());
    i82875p_process_error_info(mci, info.as_mut_ptr(), 1);
}

unsafe fn dual_channel_active(drc: u32) -> u32 { (drc >> 21) & 1 }

/* External kernel/EDAC declarations are supplied by the surrounding build. */
extern "C" {
    fn to_pci_dev(dev: *mut device) -> *mut pci_dev;
    fn pci_read_config_word(_: *mut pci_dev, _: u16, _: *mut u16); fn pci_read_config_dword(_: *mut pci_dev, _: u16, _: *mut u32); fn pci_read_config_byte(_: *mut pci_dev, _: u16, _: *mut u8);
    fn pci_write_bits16(_: *mut pci_dev, _: u16, _: u16, _: u16);
    fn edac_mc_handle_error(_: i32, _: *mut mem_ctl_info, _: u32, _: u32, _: u32, _: u32, _: i32, _: i32, _: i32, _: *const core::ffi::c_char, _: *const core::ffi::c_char);
    fn edac_mc_find_csrow_by_page(_: *mut mem_ctl_info, _: u32) -> i32;
}

/* Remaining kernel interfaces and types are supplied by the including kernel. */
#[allow(non_camel_case_types)] pub enum pci_dev {}
#[allow(non_camel_case_types)] pub enum device {}
#[allow(non_camel_case_types)] pub enum mem_ctl_info {}
#[allow(non_camel_case_types)] pub enum edac_pci_ctl_info {}
extern "C" {
    fn opstate_init();
    fn pci_get_device(_: u16, _: u16, _: *mut pci_dev) -> *mut pci_dev;
    fn pci_dev_get(_: *mut pci_dev) -> *mut pci_dev;
    fn pci_dev_put(_: *mut pci_dev);
    fn pci_enable_device(_: *mut pci_dev) -> i32;
    fn pci_disable_device(_: *mut pci_dev);
    fn pci_release_regions(_: *mut pci_dev); fn pci_request_regions(_: *mut pci_dev, _: *const core::ffi::c_char) -> i32;
    fn pci_ioremap_bar(_: *mut pci_dev, _: i32) -> *mut core::ffi::c_void; fn iounmap(_: *mut core::ffi::c_void);
    fn pci_name(_: *mut pci_dev) -> *const core::ffi::c_char;
    fn pci_register_driver(_: *mut pci_driver) -> i32; fn pci_unregister_driver(_: *mut pci_driver);
    fn pci_scan_single_device(_: *mut pci_bus, _: u8) -> *mut pci_dev; fn pci_bus_assign_resources(_: *mut pci_bus); fn pci_bus_add_device(_: *mut pci_dev);
    fn edac_pci_create_generic_ctl(_: *mut device, _: *const core::ffi::c_char) -> *mut edac_pci_ctl_info;
    fn edac_pci_release_generic_ctl(_: *mut edac_pci_ctl_info);
    fn edac_mc_del_mc(_: *mut device) -> *mut mem_ctl_info; fn edac_mc_free(_: *mut mem_ctl_info);
}
#[allow(non_camel_case_types)] pub enum pci_bus {}
#[repr(C)] pub struct pci_driver { pub name: *const core::ffi::c_char, pub probe: Option<unsafe extern "C" fn(*mut pci_dev,*const pci_device_id)->i32>, pub remove: Option<unsafe extern "C" fn(*mut pci_dev)>, pub id_table: *const pci_device_id }
#[repr(C)] pub struct pci_device_id { pub driver_data: usize }

static mut I82875P_DRIVER: pci_driver = pci_driver { name: b"i82875p_edac\0".as_ptr() as _, probe: Some(i82875p_init_one), remove: Some(i82875p_remove_one), id_table: core::ptr::null() };

unsafe extern "C" fn i82875p_init_one(pdev: *mut pci_dev, ent: *const pci_device_id) -> i32 {
    if pci_enable_device(pdev) < 0 { return -5; }
    if MCI_PDEV.is_null() { MCI_PDEV = pci_dev_get(pdev); }
    i82875p_probe1(pdev, (*ent).driver_data as i32)
}
unsafe fn i82875p_probe1(_: *mut pci_dev, _: i32) -> i32 { -19 }
unsafe extern "C" fn i82875p_remove_one(pdev: *mut pci_dev) {
    if !I82875P_PCI.is_null() { edac_pci_release_generic_ctl(I82875P_PCI); }
    let mci = edac_mc_del_mc(pdev as *mut device); if !mci.is_null() { edac_mc_free(mci); }
}
unsafe extern "C" fn i82875p_init() -> i32 {
    opstate_init(); let rc = pci_register_driver(&mut I82875P_DRIVER); if rc < 0 { return rc; } 0
}
unsafe extern "C" fn i82875p_exit() { if !MCI_PDEV.is_null() { i82875p_remove_one(MCI_PDEV); pci_dev_put(MCI_PDEV); } pci_unregister_driver(&mut I82875P_DRIVER); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
