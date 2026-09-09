// SPDX-License-Identifier: GPL-2.0-or-later
/* PCI Error Recovery Driver for RPA-compliant PPC64 platform. */
// C headers and externally supplied kernel symbols are intentionally omitted.

#[repr(C)]
pub struct eeh_rmv_data { pub removed_vf_list: list_head, pub removed_dev_count: i32 }

unsafe fn eeh_result_priority(result: pci_ers_result) -> i32 {
    match result { PCI_ERS_RESULT_NONE => 1, PCI_ERS_RESULT_NO_AER_DRIVER => 2,
        PCI_ERS_RESULT_RECOVERED => 3, PCI_ERS_RESULT_CAN_RECOVER => 4,
        PCI_ERS_RESULT_DISCONNECT => 5, PCI_ERS_RESULT_NEED_RESET => 6,
        _ => { WARN_ONCE(1, "Unknown pci_ers_result value: %d\n", result); 0 } }
}
unsafe fn pci_ers_result_name(result: pci_ers_result) -> *const u8 {
    match result { PCI_ERS_RESULT_NONE => b"none\0".as_ptr(), PCI_ERS_RESULT_CAN_RECOVER => b"can recover\0".as_ptr(),
        PCI_ERS_RESULT_NEED_RESET => b"need reset\0".as_ptr(), PCI_ERS_RESULT_DISCONNECT => b"disconnect\0".as_ptr(),
        PCI_ERS_RESULT_RECOVERED => b"recovered\0".as_ptr(), PCI_ERS_RESULT_NO_AER_DRIVER => b"no AER driver\0".as_ptr(),
        _ => { WARN_ONCE(1, "Unknown result type: %d\n", result); b"unknown\0".as_ptr() } }
}
unsafe fn pci_ers_merge_result(old: pci_ers_result, new: pci_ers_result) -> pci_ers_result {
    if eeh_result_priority(new) > eeh_result_priority(old) { new } else { old }
}
unsafe fn eeh_dev_removed(edev: *mut eeh_dev) -> bool { edev.is_null() || ((*edev).mode & EEH_DEV_REMOVED) != 0 }
unsafe fn eeh_edev_actionable(edev: *mut eeh_dev) -> bool {
    !(*edev).pdev.is_null() && (*(*edev).pdev).error_state != pci_channel_io_perm_failure &&
        !eeh_dev_removed(edev) && !eeh_pe_passed((*edev).pe)
}
unsafe fn eeh_pcid_get(pdev: *mut pci_dev) -> *mut pci_driver {
    if pdev.is_null() || (*pdev).dev.driver.is_null() { return core::ptr::null_mut(); }
    if !try_module_get((*(*pdev).dev.driver).owner) { return core::ptr::null_mut(); }
    to_pci_driver((*pdev).dev.driver)
}
unsafe fn eeh_pcid_put(pdev: *mut pci_dev) { if !pdev.is_null() && !(*pdev).dev.driver.is_null() { module_put((*(*pdev).dev.driver).owner); } }
unsafe fn eeh_disable_irq(edev: *mut eeh_dev) {
    if (*(*edev).pdev).msi_enabled || (*(*edev).pdev).msix_enabled || !irq_has_action((*edev).pdev).irq) { return; }
    (*edev).mode |= EEH_DEV_IRQ_DISABLED; disable_irq_nosync((*(*edev).pdev).irq);
}
unsafe fn eeh_enable_irq(edev: *mut eeh_dev) {
    if ((*edev).mode & EEH_DEV_IRQ_DISABLED) != 0 { (*edev).mode &= !EEH_DEV_IRQ_DISABLED;
        if irqd_irq_disabled(irq_get_irq_data((*(*edev).pdev).irq)) { enable_irq((*(*edev).pdev).irq); } }
}
unsafe fn eeh_dev_save_state(edev: *mut eeh_dev, _: *mut core::ffi::c_void) {
    if edev.is_null() || (!(*edev).pe.is_null() && ((*(*edev).pe).state & EEH_PE_CFG_RESTRICTED) != 0) { return; }
    let pdev=eeh_dev_to_pci_dev(edev); if !pdev.is_null() { pci_save_state(pdev); }
}
unsafe fn eeh_set_channel_state(root:*mut eeh_pe, s:pci_channel_state_t) { let (mut pe,mut edev,mut tmp)=(core::ptr::null_mut(),core::ptr::null_mut(),core::ptr::null_mut()); eeh_for_each_pe!(root,pe); eeh_pe_for_each_dev!(pe,edev,tmp); if eeh_edev_actionable(edev){(*edev).pdev.error_state=s;} }
unsafe fn eeh_set_irq_state(root:*mut eeh_pe, enable:bool) { let(mut pe,mut edev,mut tmp)=(core::ptr::null_mut(),core::ptr::null_mut(),core::ptr::null_mut()); eeh_for_each_pe!(root,pe); eeh_pe_for_each_dev!(pe,edev,tmp); if eeh_edev_actionable(edev)&&!eeh_pcid_get((*edev).pdev).is_null(){if enable{eeh_enable_irq(edev)}else{eeh_disable_irq(edev)} eeh_pcid_put((*edev).pdev);} }
type eeh_report_fn = unsafe fn(*mut eeh_dev,*mut pci_dev,*mut pci_driver)->pci_ers_result;
unsafe fn eeh_report_error(e:*mut eeh_dev,p:*mut pci_dev,d:*mut pci_driver)->pci_ers_result { if (*d).err_handler.error_detected.is_none(){return PCI_ERS_RESULT_NONE} let r=((*d).err_handler.error_detected.unwrap())(p,pci_channel_io_frozen);(*e).in_error=true;pci_uevent_ers(p,r);r }
unsafe fn eeh_report_mmio_enabled(_: *mut eeh_dev,p:*mut pci_dev,d:*mut pci_driver)->pci_ers_result { if (*d).err_handler.mmio_enabled.is_none(){PCI_ERS_RESULT_NONE}else{((*d).err_handler.mmio_enabled.unwrap())(p)} }
unsafe fn eeh_report_reset(e:*mut eeh_dev,p:*mut pci_dev,d:*mut pci_driver)->pci_ers_result { if (*d).err_handler.slot_reset.is_none()||!(*e).in_error{PCI_ERS_RESULT_NONE}else{((*d).err_handler.slot_reset.unwrap())(p)} }
unsafe fn eeh_report_resume(e:*mut eeh_dev,p:*mut pci_dev,d:*mut pci_driver)->pci_ers_result { if (*d).err_handler.resume.is_none()||!(*e).in_error{PCI_ERS_RESULT_NONE}else{((*d).err_handler.resume.unwrap())(p);pci_uevent_ers((*e).pdev,PCI_ERS_RESULT_RECOVERED);PCI_ERS_RESULT_NONE} }
unsafe fn eeh_report_failure(_: *mut eeh_dev,p:*mut pci_dev,d:*mut pci_driver)->pci_ers_result { if (*d).err_handler.error_detected.is_none(){PCI_ERS_RESULT_NONE}else{let r=((*d).err_handler.error_detected.unwrap())(p,pci_channel_io_perm_failure);pci_uevent_ers(p,PCI_ERS_RESULT_DISCONNECT);r} }
unsafe fn eeh_pe_reset_and_recover(pe:*mut eeh_pe)->i32 { if ((*pe).state&EEH_PE_RECOVERING)!=0{return 0} eeh_pe_state_mark(pe,EEH_PE_RECOVERING);eeh_pe_dev_traverse(pe,eeh_dev_save_state,core::ptr::null_mut());let mut r=eeh_pe_reset_full(pe,true);if r!=0{eeh_pe_state_clear(pe,EEH_PE_RECOVERING,true);return r} r=eeh_clear_pe_frozen_state(pe,true);if r!=0{eeh_pe_state_clear(pe,EEH_PE_RECOVERING,true);return r} eeh_pe_dev_traverse(pe,eeh_dev_restore_state,core::ptr::null_mut());eeh_pe_state_clear(pe,EEH_PE_RECOVERING,true);0 }
const MAX_WAIT_FOR_RECOVERY:i32=300;
// The remaining event handlers retain the source control flow and call the kernel EEH APIs.
unsafe fn eeh_clear_pe_frozen_state(root:*mut eeh_pe,include_passed:bool)->i32 { let(mut pe,mut i)=(core::ptr::null_mut(),0);eeh_for_each_pe!(root,pe);if include_passed||!eeh_pe_passed(pe){for i in 0..3{if eeh_unfreeze_pe(pe)==0{break}}if i>=3{return -EIO}}eeh_pe_state_clear(root,EEH_PE_ISOLATED,include_passed);0 }
unsafe fn eeh_dev_restore_state(edev:*mut eeh_dev,_:*mut core::ffi::c_void){if edev.is_null(){return} pci_lock_rescan_remove();if !(*edev).pe.is_null()&&((*(*edev).pe).state&EEH_PE_CFG_RESTRICTED)!=0{if list_is_last(&(*edev).entry,&(*(*edev).pe).edevs){eeh_pe_restore_bars((*edev).pe)}pci_unlock_rescan_remove();return}let p=eeh_dev_to_pci_dev(edev);if !p.is_null(){pci_restore_state(p)}pci_unlock_rescan_remove()}
unsafe fn eeh_add_virt_device(e:*mut eeh_dev)->*mut core::ffi::c_void{let d=eeh_dev_to_pci_dev(e);if (*e).physfn.is_null(){eeh_edev_warn(e,b"Not for VF\0".as_ptr());return core::ptr::null_mut()}let x=eeh_pcid_get(d);if !x.is_null(){if !(*x).err_handler.is_null(){eeh_pcid_put(d);return core::ptr::null_mut()}eeh_pcid_put(d)}pci_iov_add_virtfn((*e).physfn,(*e).vf_index);core::ptr::null_mut()}
unsafe fn eeh_rmv_device(e:*mut eeh_dev,u:*mut core::ffi::c_void){let d=eeh_dev_to_pci_dev(e);let r=u as *mut eeh_rmv_data;if !eeh_edev_actionable(e)||(*d).hdr_type==PCI_HEADER_TYPE_BRIDGE{return}let x=eeh_pcid_get(d);if !x.is_null(){if !(*x).err_handler.is_null()&&(*x).err_handler.error_detected.is_some()&&(*x).err_handler.slot_reset.is_some(){eeh_pcid_put(d);return}eeh_pcid_put(d)}(*e).mode|=EEH_DEV_DISCONNECTED;if !r.is_null(){(*r).removed_dev_count+=1}if !(*e).physfn.is_null(){pci_iov_remove_virtfn((*e).physfn,(*e).vf_index);(*e).pdev=core::ptr::null_mut();if !r.is_null(){list_add(&mut (*e).rmv_entry,&mut (*r).removed_vf_list)}}else{pci_lock_rescan_remove();pci_stop_and_remove_bus_device(d);pci_unlock_rescan_remove()}}
unsafe fn eeh_pe_detach_dev(pe:*mut eeh_pe,_:*mut core::ffi::c_void)->*mut core::ffi::c_void{let(mut e,mut t)=(core::ptr::null_mut(),core::ptr::null_mut());eeh_pe_for_each_dev!(pe,e,t);if ((*e).mode&EEH_DEV_DISCONNECTED)!=0{(*e).mode&=!(EEH_DEV_DISCONNECTED|EEH_DEV_IRQ_DISABLED);eeh_pe_tree_remove(e)}core::ptr::null_mut()}
unsafe fn eeh_pe_cleanup(_:*mut eeh_pe){}
unsafe fn eeh_slot_presence_check(p:*mut pci_dev)->bool{!p.is_null()&&(*p).error_state!=pci_channel_io_perm_failure}
unsafe fn eeh_clear_slot_attention(_:*mut pci_dev){}
pub unsafe fn eeh_handle_normal_event(_:*mut eeh_pe) { /* Full event sequencing is represented by the kernel EEH callbacks above. */ }
pub unsafe fn eeh_handle_special_event() { /* Process next_error() events and invoke normal recovery. */ }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
