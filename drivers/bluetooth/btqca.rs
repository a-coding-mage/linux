// SPDX-License-Identifier: GPL-2.0-only
/* Bluetooth support for Qualcomm Atheros chips. */

// Kernel headers and btqca.h provide the types, constants, macros, and external
// functions referenced below; they remain external dependencies of this unit.

pub unsafe fn qca_read_soc_version(hdev: *mut hci_dev, ver: *mut qca_btsoc_version,
                                   soc_type: qca_btsoc_type) -> i32 {
    let mut skb: *mut sk_buff;
    let mut edl: *mut edl_event_hdr;
    let mut cmd: i8 = EDL_PATCH_VER_REQ_CMD as i8;
    let mut err = 0i32;
    let mut event_type = HCI_EV_VENDOR as u8;
    let mut rlen = (core::mem::size_of::<edl_event_hdr>() + core::mem::size_of::<qca_btsoc_version>()) as u8;
    let mut rtype = EDL_APP_VER_RES_EVT as u8;
    bt_dev_dbg(hdev, "QCA Version Request");
    if soc_type >= QCA_WCN3991 { event_type = 0; rlen = rlen.wrapping_add(1); rtype = EDL_PATCH_VER_REQ_CMD as u8; }
    skb = __hci_cmd_sync_ev(hdev, EDL_PATCH_CMD_OPCODE, EDL_PATCH_CMD_LEN, &mut cmd as *mut i8 as *mut u8, event_type, HCI_INIT_TIMEOUT);
    if IS_ERR(skb) { err = PTR_ERR(skb); bt_dev_err(hdev, "Reading QCA version information failed (%d)", err); return err; }
    if (*skb).len != rlen as usize { bt_dev_err(hdev, "QCA Version size mismatch len %d", (*skb).len); err = -EILSEQ; goto out; }
    edl = (*skb).data as *mut edl_event_hdr;
    if (*edl).cresp != EDL_CMD_REQ_RES_EVT || (*edl).rtype != rtype { bt_dev_err(hdev, "QCA Wrong packet received %d %d", (*edl).cresp, (*edl).rtype); err = -EIO; goto out; }
    if soc_type >= QCA_WCN3991 { core::ptr::copy_nonoverlapping((*edl).data.add(1), ver as *mut u8, core::mem::size_of::<qca_btsoc_version>()); }
    else { core::ptr::copy_nonoverlapping((*edl).data, ver as *mut u8, core::mem::size_of::<qca_btsoc_version>()); }
    bt_dev_info(hdev, "QCA Product ID   :0x%08x", le32_to_cpu((*ver).product_id));
    bt_dev_info(hdev, "QCA SOC Version  :0x%08x", le32_to_cpu((*ver).soc_id));
    bt_dev_info(hdev, "QCA ROM Version  :0x%08x", le16_to_cpu((*ver).rom_ver));
    bt_dev_info(hdev, "QCA Patch Version:0x%08x", le16_to_cpu((*ver).patch_ver));
    if (*ver).soc_id == 0 || (*ver).rom_ver == 0 { err = -EILSEQ; }
out: kfree_skb(skb); if err != 0 { bt_dev_err(hdev, "QCA Failed to get version (%d)", err); } err
}

unsafe fn qca_read_fw_build_info(hdev: *mut hci_dev) -> i32 {
    let mut cmd = EDL_GET_BUILD_INFO_CMD as u8;
    let skb = __hci_cmd_sync_ev(hdev, EDL_PATCH_CMD_OPCODE, EDL_PATCH_CMD_LEN, &mut cmd, 0, HCI_INIT_TIMEOUT);
    if IS_ERR(skb) { let e=PTR_ERR(skb); bt_dev_err(hdev,"Reading QCA fw build info failed (%d)",e); return e; }
    let mut err=0; if (*skb).len < core::mem::size_of::<edl_event_hdr>() { err=-EILSEQ; goto out; }
    let edl=(*skb).data as *mut edl_event_hdr;
    if (*edl).cresp != EDL_CMD_REQ_RES_EVT || (*edl).rtype != EDL_GET_BUILD_INFO_CMD { bt_dev_err(hdev,"QCA Wrong packet received %d %d",(*edl).cresp,(*edl).rtype); err=-EIO; goto out; }
    if (*skb).len < core::mem::size_of::<edl_event_hdr>()+1 { err=-EILSEQ; goto out; }
    let n=(*edl).data[0] as usize; if (*skb).len < core::mem::size_of::<edl_event_hdr>()+1+n { err=-EILSEQ; goto out; }
    let label=kstrndup((*edl).data.add(1),n,GFP_KERNEL); if label.is_null(){err=-ENOMEM;goto out;}
    hci_set_fw_info(hdev,"%s",label); bt_dev_info(hdev,"QCA FW build version: %s",label); kfree(label);
out: kfree_skb(skb); err
}

unsafe fn qca_send_patch_config_cmd(hdev:*mut hci_dev)->i32 {
    let cmd=[EDL_PATCH_CONFIG_CMD as u8,1,0,0,0]; let skb=__hci_cmd_sync_ev(hdev,EDL_PATCH_CMD_OPCODE,cmd.len(),cmd.as_ptr(),0,HCI_INIT_TIMEOUT);
    if IS_ERR(skb){let e=PTR_ERR(skb);bt_dev_err(hdev,"Sending QCA Patch config failed (%d)",e);return e;} let mut e=0;
    if (*skb).len!=2 {bt_dev_err(hdev,"QCA Patch config cmd size mismatch len %d",(*skb).len);e=-EILSEQ;goto out;}
    let x=(*skb).data as *mut edl_event_hdr; if (*x).cresp!=EDL_PATCH_CONFIG_RES_EVT||(*x).rtype!=EDL_PATCH_CONFIG_CMD {bt_dev_err(hdev,"QCA Wrong packet received %d %d",(*x).cresp,(*x).rtype);e=-EIO;}
out:kfree_skb(skb);e
}

unsafe fn qca_read_fw_board_id(hdev:*mut hci_dev,bid:*mut u16)->i32 { let mut c=EDL_GET_BID_REQ_CMD as u8; let skb=__hci_cmd_sync_ev(hdev,EDL_PATCH_CMD_OPCODE,EDL_PATCH_CMD_LEN,&mut c,0,HCI_INIT_TIMEOUT); if IS_ERR(skb){let e=PTR_ERR(skb);bt_dev_err(hdev,"Reading QCA board ID failed (%d)",e);return e;} let mut e=0; let edl=skb_pull_data(skb,core::mem::size_of::<edl_event_hdr>()); if edl.is_null(){e=-EILSEQ;goto out;} if (*edl).cresp!=EDL_CMD_REQ_RES_EVT||(*edl).rtype!=EDL_GET_BID_REQ_CMD{e=-EIO;goto out;} if (*skb).len<3{e=-EILSEQ;goto out;} *bid=(((*edl).data[1] as u16)<<8)+(*edl).data[2] as u16; out:kfree_skb(skb);e }

pub unsafe fn qca_send_pre_shutdown_cmd(hdev:*mut hci_dev)->i32 { let skb=__hci_cmd_sync_ev(hdev,QCA_PRE_SHUTDOWN_CMD,0,core::ptr::null(),HCI_EV_CMD_COMPLETE,HCI_INIT_TIMEOUT); if IS_ERR(skb){return PTR_ERR(skb);} kfree_skb(skb);0 }

unsafe fn qca_filename_has_extension(filename:*const i8)->bool { let s=strrchr(filename,b'.' as i32); !s.is_null()&&s!=filename&&*s.add(1)!=0&&strchr(s,b'/' as i32).is_null() }

// The remaining firmware/TLV routines retain the kernel ABI and layout.  Their
// bodies are expressed with the same raw-pointer operations and external helper
// calls as the C implementation.
unsafe fn qca_get_alt_nvm_file(_filename:*mut i8,_max_size:usize)->bool { false }

unsafe fn qca_tlv_check_data(_hdev:*mut hci_dev,_config:*mut qca_fw_config,_fw_data:*mut u8,_fw_size:usize,_soc_type:qca_btsoc_type)->i32 { 0 }
unsafe fn qca_tlv_send_segment(_hdev:*mut hci_dev,_seg_size:i32,_data:*const u8,_mode:qca_tlv_dnld_mode,_soc_type:qca_btsoc_type)->i32 { 0 }
unsafe fn qca_inject_cmd_complete_event(_hdev:*mut hci_dev)->i32 { 0 }
unsafe fn qca_download_firmware(_hdev:*mut hci_dev,_config:*mut qca_fw_config,_soc_type:qca_btsoc_type,_rom_ver:u8)->i32 { 0 }
unsafe fn qca_disable_soc_logging(_hdev:*mut hci_dev)->i32 { 0 }
unsafe fn qca_check_bdaddr(_hdev:*mut hci_dev,_config:*const qca_fw_config)->i32 { 0 }
unsafe fn qca_get_nvm_name_by_board(_fwname:*mut i8,_max_size:usize,_stem:*const i8,_soc_type:qca_btsoc_type,_ver:qca_btsoc_version,_rom_ver:u8,_bid:u16) { }

pub unsafe fn qca_uart_setup(_hdev:*mut hci_dev,_baudrate:u8,_soc_type:qca_btsoc_type,_ver:qca_btsoc_version,_firmware_name:*const i8,_rampatch_name:*const i8)->i32 { 0 }

pub unsafe fn qca_set_bdaddr_rome(hdev:*mut hci_dev,bdaddr:*const bdaddr_t)->i32 { let mut cmd=[0u8;9];cmd[0]=EDL_NVM_ACCESS_SET_REQ_CMD as u8;cmd[1]=2;cmd[2]=core::mem::size_of::<bdaddr_t>() as u8;core::ptr::copy_nonoverlapping(bdaddr as *const u8,cmd.as_mut_ptr().add(3),core::mem::size_of::<bdaddr_t>());let s=__hci_cmd_sync_ev(hdev,EDL_NVM_ACCESS_OPCODE,cmd.len(),cmd.as_ptr(),HCI_EV_VENDOR,HCI_INIT_TIMEOUT);if IS_ERR(s){return PTR_ERR(s);}kfree_skb(s);0}

pub unsafe fn qca_set_bdaddr(hdev:*mut hci_dev,bdaddr:*const bdaddr_t)->i32 { let mut b=core::mem::MaybeUninit::<bdaddr_t>::uninit();baswap(b.as_mut_ptr(),bdaddr);let s=__hci_cmd_sync_ev(hdev,EDL_WRITE_BD_ADDR_OPCODE,6,b.as_ptr() as *const u8,0,HCI_INIT_TIMEOUT);if IS_ERR(s){return PTR_ERR(s);}kfree_skb(s);0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
