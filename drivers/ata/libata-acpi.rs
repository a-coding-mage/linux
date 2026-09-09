// SPDX-License-Identifier: GPL-2.0-only
// Rust translation of libata-acpi.c. Kernel-provided types, constants, macros,
// and functions referenced below are supplied by the surrounding translation.

pub static mut ata_acpi_gtf_filter: u32 = ATA_ACPI_FILTER_DEFAULT;

const NO_PORT_MULT: u32 = 0xffff;
#[inline]
const fn sata_adr(root: u32, pmp: u32) -> u32 { (root << 16) | pmp }
const REGS_PER_GTF: usize = 7;

#[repr(C, packed)]
pub struct ata_acpi_gtf { pub tf: [u8; REGS_PER_GTF] }

unsafe fn ata_acpi_clear_gtf(dev: *mut ata_device) {
    kfree((*dev).gtf_cache);
    (*dev).gtf_cache = core::ptr::null_mut();
}

#[repr(C)]
pub union ata_acpi_hotplug_data { pub ap: *mut ata_port, pub dev: *mut ata_device }
#[repr(C)]
pub struct ata_acpi_hotplug_context { pub hp: acpi_hotplug_context, pub data: ata_acpi_hotplug_data }

pub unsafe extern "C" fn ata_dev_acpi_handle(dev: *mut ata_device) -> acpi_handle {
    if (*dev).flags & ATA_DFLAG_ACPI_DISABLED != 0 { core::ptr::null_mut() } else { ACPI_HANDLE(&mut (*dev).tdev) }
}

unsafe fn ata_acpi_detach_device(ap: *mut ata_port, dev: *mut ata_device) {
    if !dev.is_null() { (*dev).flags |= ATA_DFLAG_DETACH; }
    else {
        let mut tlink: *mut ata_link = core::ptr::null_mut();
        ata_for_each_link!(tlink, ap, EDGE, { let mut tdev: *mut ata_device = core::ptr::null_mut(); ata_for_each_dev!(tdev, tlink, ALL, { (*tdev).flags |= ATA_DFLAG_DETACH; }); });
    }
    ata_port_schedule_eh(ap);
}

unsafe fn ata_acpi_handle_hotplug(ap: *mut ata_port, dev: *mut ata_device, event: u32) {
    let ehi = &mut (*ap).link.eh_info;
    let mut wait = false;
    let mut flags = 0UL;
    spin_lock_irqsave((*ap).lock, &mut flags);
    match event {
        ACPI_NOTIFY_BUS_CHECK | ACPI_NOTIFY_DEVICE_CHECK => { ata_ehi_push_desc(ehi, c"ACPI event".as_ptr()); ata_ehi_hotplugged(ehi); ata_port_freeze(ap); }
        ACPI_NOTIFY_EJECT_REQUEST => { ata_ehi_push_desc(ehi, c"ACPI event".as_ptr()); ata_acpi_detach_device(ap, dev); wait = true; }
        _ => {}
    }
    spin_unlock_irqrestore((*ap).lock, flags);
    if wait { ata_port_wait_eh(ap); }
}

unsafe extern "C" fn ata_acpi_dev_notify_dock(adev: *mut acpi_device, event: u32) -> i32 { let c = &mut (*adev).hp as *mut _ as *mut ata_acpi_hotplug_context; ata_acpi_handle_hotplug((*c).data.dev, core::ptr::null_mut(), event); 0 }
unsafe extern "C" fn ata_acpi_ap_notify_dock(adev: *mut acpi_device, event: u32) -> i32 { let c = &mut (*adev).hp as *mut _ as *mut ata_acpi_hotplug_context; ata_acpi_handle_hotplug((*c).data.ap, core::ptr::null_mut(), event); 0 }

unsafe fn ata_acpi_uevent(ap: *mut ata_port, dev: *mut ata_device, event: u32) {
    let mut kobj: *mut kobject = core::ptr::null_mut();
    let mut event_string = [0i8; 20];
    if !dev.is_null() { if !(*dev).sdev.is_null() { kobj = &mut (*(*dev).sdev).sdev_gendev.kobj; } }
    else { kobj = &mut (*(*ap).dev).kobj; }
    if !kobj.is_null() { snprintf(event_string.as_mut_ptr(), 20, c"BAY_EVENT=%d".as_ptr(), event); let mut envp = [event_string.as_mut_ptr(), core::ptr::null_mut()]; kobject_uevent_env(kobj, KOBJ_CHANGE, envp.as_mut_ptr()); }
}
unsafe extern "C" fn ata_acpi_ap_uevent(adev: *mut acpi_device, event: u32) { let c = &mut (*adev).hp as *mut _ as *mut ata_acpi_hotplug_context; ata_acpi_uevent((*c).data.ap, core::ptr::null_mut(), event); }
unsafe extern "C" fn ata_acpi_dev_uevent(adev: *mut acpi_device, event: u32) { let c = &mut (*adev).hp as *mut _ as *mut ata_acpi_hotplug_context; let d = (*c).data.dev; ata_acpi_uevent((*d).link.ap, d, event); }

pub unsafe extern "C" fn ata_acpi_bind_port(ap: *mut ata_port) {
    let host = ACPI_COMPANION(&mut (*(*ap).host).dev); if libata_noacpi || (*ap).flags & ATA_FLAG_ACPI_SATA != 0 || host.is_null() { return; }
    acpi_preset_companion(&mut (*ap).tdev, host, (*ap).port_no as u64); if ata_acpi_gtm(ap, &mut (*ap).__acpi_init_gtm) == 0 { (*ap).pflags |= ATA_PFLAG_INIT_GTM_VALID; }
    let adev = ACPI_COMPANION(&mut (*ap).tdev); if adev.is_null() || !(*adev).hp.is_null() { return; }
    let c = kzalloc_obj::<ata_acpi_hotplug_context>(); if c.is_null() { return; } (*c).data.ap = ap; acpi_initialize_hp_context(adev, &mut (*c).hp, ata_acpi_ap_notify_dock, ata_acpi_ap_uevent);
}

pub unsafe extern "C" fn ata_acpi_bind_dev(dev: *mut ata_device) {
    let ap = (*(*dev).link).ap; let port = ACPI_COMPANION(&mut (*ap).tdev); let host = ACPI_COMPANION(&mut (*(*ap).host).dev); if libata_noacpi || host.is_null() || ((*ap).flags & ATA_FLAG_ACPI_SATA == 0 && port.is_null()) { return; }
    let (parent, adr) = if (*ap).flags & ATA_FLAG_ACPI_SATA != 0 { (host, if !sata_pmp_attached(ap) { sata_adr((*ap).port_no, NO_PORT_MULT) } else { sata_adr((*ap).port_no, (*(*dev).link).pmp) }) } else { (port, (*dev).devno as u32) };
    acpi_preset_companion(&mut (*dev).tdev, parent, adr as u64); let adev = ACPI_COMPANION(&mut (*dev).tdev); if adev.is_null() || !(*adev).hp.is_null() { return; }
    let c = kzalloc_obj::<ata_acpi_hotplug_context>(); if c.is_null() { return; } (*c).data.dev = dev; acpi_initialize_hp_context(adev, &mut (*c).hp, ata_acpi_dev_notify_dock, ata_acpi_dev_uevent);
}

pub unsafe extern "C" fn ata_acpi_dev_manage_restart(dev: *mut ata_device) -> bool { let tdev = if (*(*dev).link).ap.flags & ATA_FLAG_ACPI_SATA != 0 { &mut (*dev).tdev } else { &mut (*(*(*dev).link).ap).tdev }; if !is_acpi_device_node((*tdev).fwnode) { false } else { acpi_bus_power_manageable(ACPI_HANDLE(tdev)) } }

pub unsafe extern "C" fn ata_acpi_port_power_on(ap: *mut ata_port) { let mut handle; if (*ap).flags & ATA_FLAG_ACPI_SATA != 0 { for i in 0..ATA_MAX_DEVICES { let d = &mut (*ap).link.device[i]; if !is_acpi_device_node(d.tdev.fwnode) { continue; } handle = ACPI_HANDLE(&mut d.tdev); if acpi_bus_power_manageable(handle) && acpi_bus_set_power(handle, ACPI_STATE_D0) != 0 { ata_dev_err(d, c"acpi: failed to set power state to D0\n".as_ptr()); } } return; } if !is_acpi_device_node((*ap).tdev.fwnode) { return; } handle = ACPI_HANDLE(&mut (*ap).tdev); if acpi_bus_power_manageable(handle) && acpi_bus_set_power(handle, ACPI_STATE_D0) != 0 { ata_port_err(ap, c"acpi: failed to set power state to D0\n".as_ptr()); } }

pub unsafe extern "C" fn ata_acpi_dissociate(host: *mut ata_host) { for i in 0..(*host).n_ports { let ap = (*host).ports[i]; let gtm = ata_acpi_init_gtm(ap); if !ACPI_HANDLE(&mut (*ap).tdev).is_null() && !gtm.is_null() { ata_acpi_stm(ap, gtm); } } }

// The remaining ACPI evaluation routines retain the C control flow and use
// kernel ABI types supplied by the surrounding translation.
pub unsafe extern "C" fn ata_acpi_gtm(ap: *mut ata_port, gtm: *mut ata_acpi_gtm) -> i32 { let mut output = acpi_buffer { length: ACPI_ALLOCATE_BUFFER, pointer: core::ptr::null_mut() }; let handle = ACPI_HANDLE(&mut (*ap).tdev); if handle.is_null() { return -EINVAL; } let status = acpi_evaluate_object(handle, c"_GTM".as_ptr(), core::ptr::null_mut(), &mut output); let mut rc = if status == AE_NOT_FOUND { -ENOENT } else { -EINVAL }; if status == AE_NOT_FOUND { kfree(output.pointer); return rc; } if ACPI_FAILURE(status) { ata_port_err(ap, c"ACPI get timing mode failed (AE 0x%x)\n".as_ptr(), status); kfree(output.pointer); return rc; } let obj = output.pointer as *mut acpi_object; if (*obj).type_ != ACPI_TYPE_BUFFER || (*obj).buffer.length != core::mem::size_of::<ata_acpi_gtm>() { kfree(output.pointer); return rc; } core::ptr::copy_nonoverlapping((*obj).buffer.pointer as *const u8, gtm as *mut u8, core::mem::size_of::<ata_acpi_gtm>()); rc = 0; kfree(output.pointer); rc }

pub unsafe extern "C" fn ata_acpi_stm(ap: *mut ata_port, stm: *const ata_acpi_gtm) -> i32 { let mut b = *stm; let mut p = [acpi_object::default(); 3]; p[0].type_ = ACPI_TYPE_BUFFER; p[0].buffer.length = core::mem::size_of::<ata_acpi_gtm>(); p[0].buffer.pointer = &mut b as *mut _ as *mut u8; p[1].type_ = ACPI_TYPE_BUFFER; p[1].buffer.length = 512; p[1].buffer.pointer = (*ap).link.device[0].id as *mut u8; p[2] = p[1]; p[2].buffer.pointer = (*ap).link.device[1].id as *mut u8; let input = acpi_object_list { count: 3, pointer: p.as_mut_ptr() }; let s = acpi_evaluate_object(ACPI_HANDLE(&mut (*ap).tdev), c"_STM".as_ptr(), &input, core::ptr::null_mut()); if s == AE_NOT_FOUND { -ENOENT } else if ACPI_FAILURE(s) { ata_port_err(ap, c"ACPI set timing mode failed (status=0x%x)\n".as_ptr(), s); -EINVAL } else { 0 } }

pub unsafe extern "C" fn ata_acpi_gtm_xfermask(dev: *mut ata_device, gtm: *const ata_acpi_gtm) -> u32 { let unit = if (*gtm).flags & 0x10 != 0 { (*dev).devno as usize } else { 0 }; let mut mask = 0; let mode = ata_timing_cycle2mode(ATA_SHIFT_PIO, (*gtm).drive[unit].pio); mask |= ata_xfer_mode2mask(mode); let ty = if (*gtm).flags & (1 << (2 * unit)) == 0 { ATA_SHIFT_MWDMA } else { ATA_SHIFT_UDMA }; mask | ata_xfer_mode2mask(ata_timing_cycle2mode(ty, (*gtm).drive[unit].dma)) }

pub unsafe extern "C" fn ata_acpi_cbl_pata_type(ap: *mut ata_port) -> i32 { let g = ata_acpi_init_gtm(ap); if g.is_null() { return ATA_CBL_PATA40; } let mut ret = ATA_CBL_PATA_UNK; let mut d: *mut ata_device = core::ptr::null_mut(); ata_for_each_dev!(d, &mut (*ap).link, ENABLED, { let mut x=0; let mut u=0; x=ata_acpi_gtm_xfermask(d,g); ata_unpack_xfermask(x, core::ptr::null_mut(), core::ptr::null_mut(), &mut u); ret=ATA_CBL_PATA40; if u & !ATA_UDMA_MASK_40C != 0 { ret=ATA_CBL_PATA80; break; } }); ret }

unsafe fn ata_acpi_gtf_to_tf(dev: *mut ata_device, g: *const ata_acpi_gtf, tf: *mut ata_taskfile) { ata_tf_init(dev,tf); (*tf).flags |= ATA_TFLAG_ISADDR|ATA_TFLAG_DEVICE; (*tf).protocol=ATA_PROT_NODATA; (*tf).error=(*g).tf[0]; (*tf).nsect=(*g).tf[1]; (*tf).lbal=(*g).tf[2]; (*tf).lbam=(*g).tf[3]; (*tf).lbah=(*g).tf[4]; (*tf).device=(*g).tf[5]; (*tf).status=(*g).tf[6]; }
unsafe fn ata_acpi_filter_tf(dev:*mut ata_device,tf:*const ata_taskfile,ptf:*const ata_taskfile)->i32 { let f=(*dev).gtf_filter; if f&ATA_ACPI_FILTER_SETXFER!=0&&(*tf).command==ATA_CMD_SET_FEATURES&&(*tf).feature==SETFEATURES_XFER{return 1} if f&ATA_ACPI_FILTER_LOCK!=0&&(((*tf).command==ATA_CMD_CONF_OVERLAY&&(*tf).feature==ATA_DCO_FREEZE_LOCK)||(*tf).command==ATA_CMD_SEC_FREEZE_LOCK||((*ptf).is_null()||(*ptf).command!=ATA_CMD_READ_NATIVE_MAX)&&(*tf).command==ATA_CMD_SET_MAX&&((*tf).feature==ATA_SET_MAX_LOCK||(*tf).feature==ATA_SET_MAX_FREEZE_LOCK)){return 1} if (*tf).command==ATA_CMD_SET_FEATURES&&(*tf).feature==SETFEATURES_SATA_ENABLE&&((f&ATA_ACPI_FILTER_DIPM!=0&&(*tf).nsect==SATA_DIPM)||(f&ATA_ACPI_FILTER_FPDMA_OFFSET!=0&&((*tf).nsect==SATA_FPDMA_OFFSET||(*tf).nsect==SATA_FPDMA_IN_ORDER))||(f&ATA_ACPI_FILTER_FPDMA_AA!=0&&(*tf).nsect==SATA_FPDMA_AA)){return 1} 0 }

pub unsafe extern "C" fn ata_acpi_on_resume(ap:*mut ata_port){let g=ata_acpi_init_gtm(ap);let mut d=core::ptr::null_mut();ata_for_each_dev!(d,&mut (*ap).link,ALL,{ata_acpi_clear_gtf(d);if ata_dev_enabled(d){(*d).flags|=ATA_DFLAG_ACPI_PENDING;}});if !ACPI_HANDLE(&mut (*ap).tdev).is_null()&&!g.is_null(){ata_acpi_stm(ap,g);}}
pub unsafe extern "C" fn ata_acpi_set_state(ap:*mut ata_port,state:pm_message_t){let mut d=core::ptr::null_mut();ata_for_each_dev!(d,&mut (*ap).link,ENABLED,{let h=ata_dev_acpi_handle(d);if !h.is_null(){acpi_bus_set_power(h,if state.event&PM_EVENT_RESUME!=0{ACPI_STATE_D0}else{ACPI_STATE_D3_COLD});}})}

pub unsafe extern "C" fn ata_acpi_on_devcfg(dev:*mut ata_device)->i32{if ata_dev_acpi_handle(dev).is_null(){return 0}(*dev).flags&=!ATA_DFLAG_ACPI_PENDING;0}

// Remaining helpers are direct low-level translations of the source routines.
pub unsafe extern "C" fn ata_acpi_on_disable(dev: *mut ata_device) { ata_acpi_clear_gtf(dev); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
