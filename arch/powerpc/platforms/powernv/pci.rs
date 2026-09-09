// SPDX-License-Identifier: GPL-2.0-or-later
/* Support PCI/PCIe on PowerNV platforms. */

/* External kernel/platform declarations are supplied by the surrounding tree. */

pub unsafe fn pnv_pci_get_slot_id(np: *mut device_node, id: *mut u64) -> i32 {
    let mut node = np;
    let mut bdfn: u32 = 0;
    let mut phbid: u64 = 0;
    let ret = of_property_read_u32(np, c_str!("reg"), &mut bdfn);
    if ret != 0 { return -ENXIO; }
    bdfn = (bdfn & 0x00ff_ff00) >> 8;
    while !node.is_null() {
        if PCI_DN(node).is_null() { of_node_put(node); break; }
        if !of_device_is_compatible(node, c_str!("ibm,ioda2-phb")) &&
           !of_device_is_compatible(node, c_str!("ibm,ioda3-phb")) &&
           !of_device_is_compatible(node, c_str!("ibm,ioda2-npu2-opencapi-phb")) {
            of_node_put(node); node = of_get_parent(node); continue;
        }
        let ret = of_property_read_u64(node, c_str!("ibm,opal-phbid"), &mut phbid);
        if ret != 0 { of_node_put(node); return -ENXIO; }
        if of_device_is_compatible(node, c_str!("ibm,ioda2-npu2-opencapi-phb")) {
            *id = PCI_PHB_SLOT_ID(phbid);
        } else { *id = PCI_SLOT_ID(phbid, bdfn); }
        return 0;
    }
    -ENODEV
}

pub unsafe fn pnv_pci_get_device_tree(phandle: u32, buf: *mut core::ffi::c_void, len: u64) -> i32 {
    if !opal_check_token(OPAL_GET_DEVICE_TREE) { return -ENXIO; }
    let rc = opal_get_device_tree(phandle, buf as u64, len);
    if rc < OPAL_SUCCESS { return -EIO; } rc as i32
}
pub unsafe fn pnv_pci_get_presence_state(id: u64, state: *mut u8) -> i32 {
    if !opal_check_token(OPAL_PCI_GET_PRESENCE_STATE) { return -ENXIO; }
    if opal_pci_get_presence_state(id, state as u64) != OPAL_SUCCESS { return -EIO; } 0
}
pub unsafe fn pnv_pci_get_power_state(id: u64, state: *mut u8) -> i32 {
    if !opal_check_token(OPAL_PCI_GET_POWER_STATE) { return -ENXIO; }
    if opal_pci_get_power_state(id, state as u64) != OPAL_SUCCESS { return -EIO; } 0
}
pub unsafe fn pnv_pci_set_power_state(id: u64, state: u8, msg: *mut opal_msg) -> i32 {
    let mut m = core::mem::MaybeUninit::<opal_msg>::uninit();
    if !opal_check_token(OPAL_PCI_SET_POWER_STATE) { return -ENXIO; }
    let token = opal_async_get_token_interruptible();
    if token < 0 { return token; }
    let rc = opal_pci_set_power_state(token, id, (&state as *const u8) as u64);
    let mut ret;
    if rc == OPAL_SUCCESS { ret = 0; }
    else if rc != OPAL_ASYNC_COMPLETION { ret = -EIO; }
    else { ret = opal_async_wait_response(token, m.as_mut_ptr());
        if ret >= 0 && !msg.is_null() { ret = 1; core::ptr::copy_nonoverlapping(m.as_ptr(), msg, 1); }
    }
    opal_async_release_token(token); ret
}

unsafe fn pnv_pci_dump_pest(pest_a: *mut __be64, pest_b: *mut __be64, pest_size: i32) {
    let mut prev_a = u64::MAX; let mut prev_b = u64::MAX; let mut dup = false;
    for i in 0..pest_size { let a = be64_to_cpu(*pest_a.add(i as usize)); let b = be64_to_cpu(*pest_b.add(i as usize));
        if a != prev_a || b != prev_b { if dup { pr_info(c_str!("PE[..%03x] A/B: as above\n"), i-1); dup=false; }
            prev_a=a; prev_b=b; if a & PNV_IODA_STOPPED_STATE != 0 || b & PNV_IODA_STOPPED_STATE != 0 { pr_info(c_str!("PE[%03x] A/B: %016llx %016llx\n"),i,a,b); }
        } else if !dup && (a & PNV_IODA_STOPPED_STATE != 0 || b & PNV_IODA_STOPPED_STATE != 0) { dup=true; }
    }
}

/* Diagnostic records are externally defined C-layout records; preserve the original field tests/logging. */
unsafe fn pnv_pci_dump_p7ioc_diag_data(hose: *mut pci_controller, common: *mut OpalIoPhbErrorCommon) {
    let data = common as *mut OpalIoP7IOCPhbErrorData;
    pr_info(c_str!("P7IOC PHB#%x Diag-data (Version: %d)\n"), (*hose).global_number, be32_to_cpu((*common).version));
    if (*data).brdgCtl != 0 { pr_info(c_str!("brdgCtl:     %08x\n"),be32_to_cpu((*data).brdgCtl)); }
    if (*data).portStatusReg != 0 || (*data).rootCmplxStatus != 0 || (*data).busAgentStatus != 0 { pr_info(c_str!("UtlSts:      %08x %08x %08x\n"),be32_to_cpu((*data).portStatusReg),be32_to_cpu((*data).rootCmplxStatus),be32_to_cpu((*data).busAgentStatus)); }
    pnv_pci_dump_pest((*data).pestA.as_mut_ptr(), (*data).pestB.as_mut_ptr(), OPAL_P7IOC_NUM_PEST_REGS);
}

unsafe fn pnv_pci_dump_phb3_diag_data(hose:*mut pci_controller, common:*mut OpalIoPhbErrorCommon) { let data=common as *mut OpalIoPhb3ErrorData; pr_info(c_str!("PHB3 PHB#%x Diag-data (Version: %d)\n"),(*hose).global_number,be32_to_cpu((*common).version)); pnv_pci_dump_pest((*data).pestA.as_mut_ptr(),(*data).pestB.as_mut_ptr(),OPAL_PHB3_NUM_PEST_REGS); }
unsafe fn pnv_pci_dump_phb4_diag_data(hose:*mut pci_controller, common:*mut OpalIoPhbErrorCommon) { let data=common as *mut OpalIoPhb4ErrorData; pr_info(c_str!("PHB4 PHB#%d Diag-data (Version: %d)\n"),(*hose).global_number,be32_to_cpu((*common).version)); pnv_pci_dump_pest((*data).pestA.as_mut_ptr(),(*data).pestB.as_mut_ptr(),OPAL_PHB4_NUM_PEST_REGS); }

pub unsafe fn pnv_pci_dump_phb_diag_data(hose:*mut pci_controller, log_buff:*mut u8) { if hose.is_null() || log_buff.is_null(){return;} let c=log_buff as *mut OpalIoPhbErrorCommon; match be32_to_cpu((*c).ioType) { OPAL_PHB_ERROR_DATA_TYPE_P7IOC=>pnv_pci_dump_p7ioc_diag_data(hose,c), OPAL_PHB_ERROR_DATA_TYPE_PHB3=>pnv_pci_dump_phb3_diag_data(hose,c), OPAL_PHB_ERROR_DATA_TYPE_PHB4=>pnv_pci_dump_phb4_diag_data(hose,c), _=>pr_warn(c_str!("%s: Unrecognized ioType %d\n"), c_str!("pnv_pci_dump_phb_diag_data"),be32_to_cpu((*c).ioType)) } }

pub unsafe fn pnv_pci_cfg_read(pdn:*mut pci_dn, where_:i32, size:i32, val:*mut u32)->i32 { let phb=(*(*pdn).phb).private_data; let bdfn=((*pdn).busno<<8)|(*pdn).devfn; let rc; match size { 1=>{let mut v=0;rc=opal_pci_config_read_byte((*phb).opal_id,bdfn,where_,&mut v);*val=if rc==OPAL_SUCCESS{v as u32}else{0xff}},2=>{let mut v=0;rc=opal_pci_config_read_half_word((*phb).opal_id,bdfn,where_,&mut v);*val=if rc==OPAL_SUCCESS{be16_to_cpu(v) as u32}else{0xffff}},4=>{let mut v=0;rc=opal_pci_config_read_word((*phb).opal_id,bdfn,where_,&mut v);*val=if rc==OPAL_SUCCESS{be32_to_cpu(v)}else{0xffff_ffff}},_=>return PCIBIOS_FUNC_NOT_SUPPORTED}; PCIBIOS_SUCCESSFUL }
pub unsafe fn pnv_pci_cfg_write(pdn:*mut pci_dn,where_:i32,size:i32,val:u32)->i32 { let phb=(*(*pdn).phb).private_data;let b=((*pdn).busno<<8)|(*pdn).devfn;match size{1=>{opal_pci_config_write_byte((*phb).opal_id,b,where_,val);},2=>{opal_pci_config_write_half_word((*phb).opal_id,b,where_,val);},4=>{opal_pci_config_write_word((*phb).opal_id,b,where_,val);},_=>return PCIBIOS_FUNC_NOT_SUPPORTED} PCIBIOS_SUCCESSFUL }

unsafe fn pnv_pci_config_check_eeh(pdn:*mut pci_dn){let phb=(*(*pdn).phb).private_data;let mut f=0u8;let mut e=0u16;let pe=if (*pdn).pe_number==IODA_INVALID_PE{(*phb).ioda.reserved_pe_idx}else{(*pdn).pe_number};let rc=opal_pci_eeh_freeze_status((*phb).opal_id,pe,&mut f,&mut e,core::ptr::null_mut());if rc!=0{return;}if f==OPAL_EEH_STOPPED_MMIO_FREEZE||f==OPAL_EEH_STOPPED_DMA_FREEZE||f==OPAL_EEH_STOPPED_MMIO_DMA_FREEZE{opal_pci_eeh_freeze_clear((*phb).opal_id,pe,OPAL_EEH_ACTION_CLEAR_FREEZE_ALL);}}
unsafe fn pnv_pci_read_config(bus:*mut pci_bus,devfn:u32,where_:i32,size:i32,val:*mut u32)->i32{*val=0xffff_ffff;let pdn=pci_get_pdn_by_devfn(bus,devfn);if pdn.is_null(){return PCIBIOS_DEVICE_NOT_FOUND;}let r=pnv_pci_cfg_read(pdn,where_,size,val);let phb=(*(*pdn).phb).private_data;if (*phb).flags&PNV_PHB_FLAG_EEH==0{pnv_pci_config_check_eeh(pdn);}r}
unsafe fn pnv_pci_write_config(bus:*mut pci_bus,devfn:u32,where_:i32,size:i32,val:u32)->i32{let pdn=pci_get_pdn_by_devfn(bus,devfn);if pdn.is_null(){return PCIBIOS_DEVICE_NOT_FOUND;}let r=pnv_pci_cfg_write(pdn,where_,size,val);let phb=(*(*pdn).phb).private_data;if (*phb).flags&PNV_PHB_FLAG_EEH==0{pnv_pci_config_check_eeh(pdn);}r}

unsafe fn pnv_p7ioc_rc_quirk(dev:*mut pci_dev){(*dev).class=PCI_CLASS_BRIDGE_PCI_NORMAL;}
// DECLARE_PCI_FIXUP_EARLY(PCI_VENDOR_ID_IBM, 0x3b9, pnv_p7ioc_rc_quirk)

pub static mut pnv_pci_ops: pci_ops = pci_ops { read: pnv_pci_read_config, write: pnv_pci_write_config };
pub unsafe fn pnv_pci_table_alloc(nid:i32)->*mut iommu_table { let t=kzalloc_node(core::mem::size_of::<iommu_table>(),GFP_KERNEL,nid); if t.is_null(){return core::ptr::null_mut();} INIT_LIST_HEAD_RCU(&mut (*t).it_group_list);kref_init(&mut (*t).it_kref);t }
pub unsafe fn pnv_pci_shutdown(){ list_for_each_entry!(hose,hose_list,list_node,{if let Some(f)=(*hose).controller_ops.shutdown{f(hose);}}); }
pub unsafe fn pnv_pci_init(){ pci_add_flags(PCI_CAN_SKIP_ISA_ALIGN); if !firmware_has_feature(FW_FEATURE_OPAL){return;} pcie_ports_disabled=true; for_each_compatible_node!(np,"ibm,ioda2-phb",pnv_pci_init_ioda2_phb(np)); for_each_compatible_node!(np,"ibm,ioda3-phb",pnv_pci_init_ioda2_phb(np)); for_each_compatible_node!(np,"ibm,ioda2-npu2-opencapi-phb",pnv_pci_init_npu2_opencapi_phb(np)); set_pci_dma_ops(&dma_iommu_ops); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
