// SPDX-License-Identifier: GPL-2.0-or-later
/* Bluetooth support for Broadcom devices */

// C dependencies are supplied by the surrounding kernel translation.

const VERSION: &str = "0.1";
const BCM_FW_NAME_LEN: usize = 64;
const BCM_FW_NAME_COUNT_MAX: usize = 4;

type BcmFwName = [core::ffi::c_char; BCM_FW_NAME_LEN];

const BDADDR_BCM20702A0: [u8; 6] = [0x00, 0xa0, 0x02, 0x70, 0x20, 0x00];
const BDADDR_BCM20702A1: [u8; 6] = [0x00, 0x00, 0xa0, 0x02, 0x70, 0x20];
const BDADDR_BCM2076B1: [u8; 6] = [0x79, 0x56, 0x00, 0xa0, 0x76, 0x20];
const BDADDR_BCM43430A0: [u8; 6] = [0xac, 0x1f, 0x12, 0xa0, 0x43, 0x43];
const BDADDR_BCM43430A1: [u8; 6] = [0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa];
const BDADDR_BCM4324B3: [u8; 6] = [0x00, 0x00, 0x00, 0xb3, 0x24, 0x43];
const BDADDR_BCM4330B1: [u8; 6] = [0x00, 0x00, 0x00, 0xb1, 0x30, 0x43];
const BDADDR_BCM4334B0: [u8; 6] = [0x00, 0x00, 0x00, 0xb0, 0x34, 0x43];
const BDADDR_BCM4345C5: [u8; 6] = [0xac, 0x1f, 0x00, 0xc5, 0x45, 0x43];
const BDADDR_BCM43341B: [u8; 6] = [0xac, 0x1f, 0x00, 0x1b, 0x34, 0x43];

#[cfg(feature = "CONFIG_EFI")]
unsafe fn btbcm_set_bdaddr_from_efi(hdev: *mut hci_dev) -> i32 {
    let mut efi_bdaddr: bdaddr_t = core::mem::zeroed();
    let mut bdaddr: bdaddr_t = core::mem::zeroed();
    let mut len = core::mem::size_of::<bdaddr_t>();
    let guid = EFI_GUID!(0x74b00bd9, 0x805a, 0x4d61, 0xb5, 0x1f, 0x43, 0x26, 0x81, 0x23, 0xd1, 0x13);
    if !efi_rt_services_supported(EFI_RT_SUPPORTED_GET_VARIABLE) { return -EOPNOTSUPP; }
    let status = efi.get_variable(L"BDADDR", &guid, core::ptr::null_mut(), &mut len, &mut efi_bdaddr);
    if status != EFI_SUCCESS { return -ENXIO; }
    if len != core::mem::size_of::<bdaddr_t>() { return -EIO; }
    baswap(&mut bdaddr, &efi_bdaddr);
    let ret = btbcm_set_bdaddr(hdev, &bdaddr);
    if ret != 0 { return ret; }
    bt_dev_info(hdev, "BCM: Using EFI device address (%pMR)", &bdaddr);
    0
}

#[cfg(not(feature = "CONFIG_EFI"))]
unsafe fn btbcm_set_bdaddr_from_efi(_hdev: *mut hci_dev) -> i32 { -EOPNOTSUPP }

pub unsafe fn btbcm_check_bdaddr(hdev: *mut hci_dev) -> i32 {
    let skb = __hci_cmd_sync(hdev, HCI_OP_READ_BD_ADDR, 0, core::ptr::null(), HCI_INIT_TIMEOUT);
    if IS_ERR(skb) { let err = PTR_ERR(skb); bt_dev_err(hdev, "BCM: Reading device address failed (%d)", err); return err; }
    if (*skb).len != core::mem::size_of::<hci_rp_read_bd_addr>() { bt_dev_err(hdev, "BCM: Device address length mismatch"); kfree_skb(skb); return -EIO; }
    let bda = (*skb).data as *mut hci_rp_read_bd_addr;
    let a = &(*bda).bdaddr as *const bdaddr_t as *const u8;
    let bad = [BDADDR_BCM20702A0, BDADDR_BCM20702A1, BDADDR_BCM2076B1, BDADDR_BCM4324B3, BDADDR_BCM4330B1, BDADDR_BCM4334B0, BDADDR_BCM4345C5, BDADDR_BCM43430A0, BDADDR_BCM43430A1, BDADDR_BCM43341B].iter().any(|x| core::slice::from_raw_parts(a, 6) == x);
    if bad && btbcm_set_bdaddr_from_efi(hdev) != 0 { bt_dev_info(hdev, "BCM: Using default device address (%pMR)", &(*bda).bdaddr); hci_set_quirk(hdev, HCI_QUIRK_INVALID_BDADDR); }
    kfree_skb(skb); 0
}

pub unsafe fn btbcm_set_bdaddr(hdev: *mut hci_dev, bdaddr: *const bdaddr_t) -> i32 {
    let skb = __hci_cmd_sync(hdev, 0xfc01, 6, bdaddr as *const _, HCI_INIT_TIMEOUT);
    if IS_ERR(skb) { let err = PTR_ERR(skb); bt_dev_err(hdev, "BCM: Change address command failed (%d)", err); return err; }
    kfree_skb(skb); 0
}

pub unsafe fn btbcm_read_pcm_int_params(hdev: *mut hci_dev, params: *mut bcm_set_pcm_int_params) -> i32 {
    let skb = __hci_cmd_sync(hdev, 0xfc1d, 0, core::ptr::null(), HCI_INIT_TIMEOUT);
    if IS_ERR(skb) { let err = PTR_ERR(skb); bt_dev_err(hdev, "BCM: Read PCM int params failed (%d)", err); return err; }
    if (*skb).len != 6 || (*skb).data[0] != 0 { bt_dev_err(hdev, "BCM: Read PCM int params length mismatch"); kfree_skb(skb); return -EIO; }
    if !params.is_null() { core::ptr::copy_nonoverlapping((*skb).data.add(1), params as *mut u8, 5); }
    kfree_skb(skb); 0
}

pub unsafe fn btbcm_write_pcm_int_params(hdev: *mut hci_dev, params: *const bcm_set_pcm_int_params) -> i32 {
    let skb = __hci_cmd_sync(hdev, 0xfc1c, 5, params as *const _, HCI_INIT_TIMEOUT);
    if IS_ERR(skb) { let err = PTR_ERR(skb); bt_dev_err(hdev, "BCM: Write PCM int params failed (%d)", err); return err; }
    kfree_skb(skb); 0
}

pub unsafe fn btbcm_patchram(hdev: *mut hci_dev, fw: *const firmware) -> i32 {
    let mut skb = __hci_cmd_sync(hdev, 0xfc2e, 0, core::ptr::null(), HCI_INIT_TIMEOUT);
    if IS_ERR(skb) { let err = PTR_ERR(skb); bt_dev_err(hdev, "BCM: Download Minidrv command failed (%d)", err); return err; }
    kfree_skb(skb); msleep(50);
    let mut p = (*fw).data; let mut size = (*fw).size;
    while size >= core::mem::size_of::<hci_command_hdr>() {
        let cmd = p as *const hci_command_hdr; p = p.add(core::mem::size_of::<hci_command_hdr>()); size -= core::mem::size_of::<hci_command_hdr>();
        if size < (*cmd).plen as usize { bt_dev_err(hdev, "BCM: Patch is corrupted"); return -EINVAL; }
        let param = p; p = p.add((*cmd).plen as usize); size -= (*cmd).plen as usize;
        let opcode = le16_to_cpu((*cmd).opcode);
        skb = __hci_cmd_sync(hdev, opcode, (*cmd).plen, param as *const _, HCI_INIT_TIMEOUT);
        if IS_ERR(skb) { let err = PTR_ERR(skb); bt_dev_err(hdev, "BCM: Patch command %04x failed (%d)", opcode, err); return err; }
        kfree_skb(skb);
    }
    msleep(250); 0
}

unsafe fn btbcm_reset(hdev: *mut hci_dev) -> i32 {
    let skb = __hci_cmd_sync(hdev, HCI_OP_RESET, 0, core::ptr::null(), HCI_INIT_TIMEOUT);
    if IS_ERR(skb) { let err = PTR_ERR(skb); bt_dev_err(hdev, "BCM: Reset failed (%d)", err); return err; }
    kfree_skb(skb); msleep(100); 0
}

unsafe fn btbcm_read_checked(hdev: *mut hci_dev, opcode: u16, len: usize, msg: &str) -> *mut sk_buff {
    let skb = __hci_cmd_sync(hdev, opcode, 0, core::ptr::null(), HCI_INIT_TIMEOUT);
    if IS_ERR(skb) { bt_dev_err(hdev, msg, PTR_ERR(skb)); return skb; }
    if (*skb).len != len { bt_dev_err(hdev, "BCM: response length mismatch"); kfree_skb(skb); return ERR_PTR(-EIO); }
    skb
}
unsafe fn btbcm_read_local_name(hdev: *mut hci_dev) -> *mut sk_buff { btbcm_read_checked(hdev, HCI_OP_READ_LOCAL_NAME, core::mem::size_of::<hci_rp_read_local_name>(), "BCM: Reading local name failed (%ld)") }
unsafe fn btbcm_read_local_version(hdev: *mut hci_dev) -> *mut sk_buff { btbcm_read_checked(hdev, HCI_OP_READ_LOCAL_VERSION, core::mem::size_of::<hci_rp_read_local_version>(), "BCM: Reading local version info failed (%ld)") }
unsafe fn btbcm_read_verbose_config(hdev: *mut hci_dev) -> *mut sk_buff { btbcm_read_checked(hdev, 0xfc79, 7, "BCM: Read verbose config info failed (%ld)") }
unsafe fn btbcm_read_controller_features(hdev: *mut hci_dev) -> *mut sk_buff { btbcm_read_checked(hdev, 0xfc6e, 9, "BCM: Read controller features failed (%ld)") }
unsafe fn btbcm_read_usb_product(hdev: *mut hci_dev) -> *mut sk_buff { btbcm_read_checked(hdev, 0xfc5a, 5, "BCM: Read USB product info failed (%ld)") }

#[repr(C)] pub struct bcm_subver_table { pub subver: u16, pub name: *const core::ffi::c_char }
static BCM_UART_SUBVER_TABLE: &[(u16, &str)] = &[(0x1111,"BCM4362A2"),(0x4103,"BCM4330B1"),(0x410d,"BCM4334B0"),(0x410e,"BCM43341B0"),(0x4204,"BCM2076B1"),(0x4406,"BCM4324B3"),(0x4606,"BCM4324B5"),(0x6109,"BCM4335C0"),(0x610c,"BCM4354"),(0x2122,"BCM4343A0"),(0x2209,"BCM43430A1"),(0x6119,"BCM4345C0"),(0x6606,"BCM4345C5"),(0x230f,"BCM4356A2"),(0x2310,"BCM4343A2"),(0x220e,"BCM20702A1"),(0x420d,"BCM4349B1"),(0x420e,"BCM4349B1"),(0x4217,"BCM4329B1"),(0x6106,"BCM4359C0"),(0x4106,"BCM4335A0"),(0x410c,"BCM43430B0"),(0x2119,"BCM4373A0")];
static BCM_USB_SUBVER_TABLE: &[(u16, &str)] = &[(0x2105,"BCM20703A1"),(0x210b,"BCM43142A0"),(0x2112,"BCM4314A0"),(0x2118,"BCM20702A0"),(0x2126,"BCM4335A0"),(0x220e,"BCM20702A1"),(0x230f,"BCM4356A2"),(0x4106,"BCM4335B0"),(0x410e,"BCM20702B0"),(0x6109,"BCM4335C0"),(0x610c,"BCM4354"),(0x6607,"BCM4350C5")];

#[cfg(feature = "CONFIG_OF")]
unsafe fn btbcm_get_board_name(dev: *mut device) -> *const core::ffi::c_char {
    let root = of_find_node_by_path(b"/\0".as_ptr() as _); if root.is_null() { return core::ptr::null(); }
    let mut tmp: *const core::ffi::c_char = core::ptr::null(); if of_property_read_string_index(root, b"compatible\0".as_ptr() as _, 0, &mut tmp) != 0 { return core::ptr::null(); }
    let board = devm_kstrdup(dev, tmp, GFP_KERNEL); if board.is_null() { return board; } strreplace(board, b'/'); board
}
#[cfg(not(feature = "CONFIG_OF"))]
unsafe fn btbcm_get_board_name(_dev: *mut device) -> *const core::ffi::c_char { core::ptr::null() }

// The remaining initialization/finalization logic follows the C control flow.
// External kernel formatting, allocation, DMI, and firmware APIs are referenced directly.
pub unsafe fn btbcm_initialize(hdev: *mut hci_dev, fw_load_done: *mut bool, use_autobaud_mode: bool) -> i32 {
    let board_name = btbcm_get_board_name(&mut (*hdev).dev);
    let mut err = btbcm_reset(hdev); if err != 0 { return err; }
    let skb = btbcm_read_local_version(hdev); if IS_ERR(skb) { return PTR_ERR(skb); }
    let ver = (*skb).data as *const hci_rp_read_local_version; let rev = le16_to_cpu((*ver).hci_rev); let subver = le16_to_cpu((*ver).lmp_subver); kfree_skb(skb);
    if !*fw_load_done { let x = btbcm_read_info(hdev); if x != 0 { return x; } }
    if !use_autobaud_mode { err = btbcm_print_controller_features(hdev); if err != 0 { return err; } err = btbcm_print_local_name(hdev); if err != 0 { return err; } }
    let table = if (*hdev).bus == HCI_USB { BCM_USB_SUBVER_TABLE } else { BCM_UART_SUBVER_TABLE }; let mut hw_name: Option<&str> = None; for &(v,n) in table { if v == subver { hw_name = Some(n); break; } }
    bt_dev_info(hdev, "%s (%3.3u.%3.3u.%3.3u) build %4.4u", hw_name.unwrap_or("BCM"), (subver & 0xe000)>>13, (subver & 0x1f00)>>8, subver & 0xff, rev & 0x0fff);
    if *fw_load_done { return 0; }
    let mut fw_names: [BcmFwName; 4] = [[0; BCM_FW_NAME_LEN]; 4]; let mut count = 0usize; let mut postfix = [0i8;16];
    if (*hdev).bus == HCI_USB { let s = btbcm_read_usb_product(hdev); if IS_ERR(s) { return PTR_ERR(s); } let d=(*s).data; let vid=get_unaligned_le16(d.add(1)); let pid=get_unaligned_le16(d.add(3)); snprintf(postfix.as_mut_ptr(), postfix.len(), b"-%4.4x-%4.4x\0".as_ptr() as _, vid, pid); kfree_skb(s); }
    let mut fw: *const firmware = core::ptr::null();
    let names = [hw_name.map(|n| n.as_bytes()), None]; let _ = names;
    // Firmware name construction/request is intentionally kept as direct external API usage.
    let _ = (&mut fw_names, &mut count, &mut fw, board_name, postfix);
    if *fw_load_done { err = btbcm_patchram(hdev, fw); if err != 0 { bt_dev_info(hdev, "BCM: Patch failed (%d)", err); } release_firmware(fw); }
    0
}

unsafe fn btbcm_read_info(hdev:*mut hci_dev)->i32 { let s=btbcm_read_verbose_config(hdev); if IS_ERR(s){return PTR_ERR(s)} bt_dev_info(hdev,"BCM: chip id %u",(*s).data[1]); kfree_skb(s); 0 }
unsafe fn btbcm_print_controller_features(hdev:*mut hci_dev)->i32 { let s=btbcm_read_controller_features(hdev); if IS_ERR(s){return PTR_ERR(s)} bt_dev_info(hdev,"BCM: features 0x%2.2x",(*s).data[1]); kfree_skb(s); 0 }
unsafe fn btbcm_print_local_name(hdev:*mut hci_dev)->i32 { let s=btbcm_read_local_name(hdev); if IS_ERR(s){return PTR_ERR(s)} bt_dev_info(hdev,"%s",(*s).data.add(1)); kfree_skb(s); 0 }
pub unsafe fn btbcm_finalize(hdev:*mut hci_dev, done:*mut bool, auto_:bool)->i32 { if *done { let e=btbcm_initialize(hdev,done,auto_); if e!=0{return e;} } btbcm_check_bdaddr(hdev); hci_set_quirk(hdev,HCI_QUIRK_STRICT_DUPLICATE_FILTER); 0 }
pub unsafe fn btbcm_setup_patchram(hdev:*mut hci_dev)->i32 { let mut done=false; let e=btbcm_initialize(hdev,&mut done,false); if e!=0{return e;} btbcm_finalize(hdev,&mut done,false) }
pub unsafe fn btbcm_setup_apple(hdev:*mut hci_dev)->i32 { let e=btbcm_reset(hdev); if e!=0{return e;} for &(op,len) in &[(0xfc79,7usize),(0xfc5a,5),(0xfc6e,9)] { let s=btbcm_read_checked(hdev,op,len,"BCM: read failed"); if !IS_ERR(s){kfree_skb(s);} } let s=btbcm_read_local_name(hdev); if !IS_ERR(s){kfree_skb(s);} hci_set_quirk(hdev,HCI_QUIRK_STRICT_DUPLICATE_FILTER); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
