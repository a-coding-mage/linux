// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OPAL hypervisor Maintenance interrupt handling support in PowerNV.
 *
 * Copyright 2014 IBM Corporation
 * Author: Mahesh Salgaonkar <mahesh@linux.vnet.ibm.com>
 */

// Dependencies supplied by the kernel and architecture-specific code are
// intentionally left as external names.

static mut OPAL_HMI_HANDLER_NB_INIT: i32 = 0;

#[repr(C)]
struct OpalHmiEvtNode {
    list: list_head,
    hmi_evt: OpalHMIEvent,
}

#[repr(C)]
struct xstop_reason {
    xstop_reason: u32,
    unit_failed: *const i8,
    description: *const i8,
}

static mut OPAL_HMI_EVT_LIST: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut OPAL_HMI_EVT_LOCK: spinlock_t = spinlock_t {};

unsafe fn print_core_checkstop_reason(level: *const i8, hmi_evt: *mut OpalHMIEvent) {
    static XSTOP_REASON: [xstop_reason; 17] = [
        xstop_reason { xstop_reason: CORE_CHECKSTOP_IFU_REGFILE, unit_failed: b"IFU\0".as_ptr() as *const i8, description: b"RegFile core check stop\0".as_ptr() as *const i8 },
        xstop_reason { xstop_reason: CORE_CHECKSTOP_IFU_LOGIC, unit_failed: b"IFU\0".as_ptr() as *const i8, description: b"Logic core check stop\0".as_ptr() as *const i8 },
        xstop_reason { xstop_reason: CORE_CHECKSTOP_PC_DURING_RECOV, unit_failed: b"PC\0".as_ptr() as *const i8, description: b"Core checkstop during recovery\0".as_ptr() as *const i8 },
        xstop_reason { xstop_reason: CORE_CHECKSTOP_ISU_REGFILE, unit_failed: b"ISU\0".as_ptr() as *const i8, description: b"RegFile core check stop (mapper error)\0".as_ptr() as *const i8 },
        xstop_reason { xstop_reason: CORE_CHECKSTOP_ISU_LOGIC, unit_failed: b"ISU\0".as_ptr() as *const i8, description: b"Logic core check stop\0".as_ptr() as *const i8 },
        xstop_reason { xstop_reason: CORE_CHECKSTOP_FXU_LOGIC, unit_failed: b"FXU\0".as_ptr() as *const i8, description: b"Logic core check stop\0".as_ptr() as *const i8 },
        xstop_reason { xstop_reason: CORE_CHECKSTOP_VSU_LOGIC, unit_failed: b"VSU\0".as_ptr() as *const i8, description: b"Logic core check stop\0".as_ptr() as *const i8 },
        xstop_reason { xstop_reason: CORE_CHECKSTOP_PC_RECOV_IN_MAINT_MODE, unit_failed: b"PC\0".as_ptr() as *const i8, description: b"Recovery in maintenance mode\0".as_ptr() as *const i8 },
        xstop_reason { xstop_reason: CORE_CHECKSTOP_LSU_REGFILE, unit_failed: b"LSU\0".as_ptr() as *const i8, description: b"RegFile core check stop\0".as_ptr() as *const i8 },
        xstop_reason { xstop_reason: CORE_CHECKSTOP_PC_FWD_PROGRESS, unit_failed: b"PC\0".as_ptr() as *const i8, description: b"Forward Progress Error\0".as_ptr() as *const i8 },
        xstop_reason { xstop_reason: CORE_CHECKSTOP_LSU_LOGIC, unit_failed: b"LSU\0".as_ptr() as *const i8, description: b"Logic core check stop\0".as_ptr() as *const i8 },
        xstop_reason { xstop_reason: CORE_CHECKSTOP_PC_LOGIC, unit_failed: b"PC\0".as_ptr() as *const i8, description: b"Logic core check stop\0".as_ptr() as *const i8 },
        xstop_reason { xstop_reason: CORE_CHECKSTOP_PC_HYP_RESOURCE, unit_failed: b"PC\0".as_ptr() as *const i8, description: b"Hypervisor Resource error - core check stop\0".as_ptr() as *const i8 },
        xstop_reason { xstop_reason: CORE_CHECKSTOP_PC_HANG_RECOV_FAILED, unit_failed: b"PC\0".as_ptr() as *const i8, description: b"Hang Recovery Failed (core check stop)\0".as_ptr() as *const i8 },
        xstop_reason { xstop_reason: CORE_CHECKSTOP_PC_AMBI_HANG_DETECTED, unit_failed: b"PC\0".as_ptr() as *const i8, description: b"Ambiguous Hang Detected (unknown source)\0".as_ptr() as *const i8 },
        xstop_reason { xstop_reason: CORE_CHECKSTOP_PC_DEBUG_TRIG_ERR_INJ, unit_failed: b"PC\0".as_ptr() as *const i8, description: b"Debug Trigger Error inject\0".as_ptr() as *const i8 },
        xstop_reason { xstop_reason: CORE_CHECKSTOP_PC_SPRD_HYP_ERR_INJ, unit_failed: b"PC\0".as_ptr() as *const i8, description: b"Hypervisor check stop via SPRC/SPRD\0".as_ptr() as *const i8 },
    ];
    if (*hmi_evt).u.xstop_error.xstop_reason == 0 { printk(b"%s\tUnknown Core check stop.\n\0".as_ptr() as *const i8, level); return; }
    printk(b"%s\tCPU PIR: %08x\n\0".as_ptr() as *const i8, level, be32_to_cpu((*hmi_evt).u.xstop_error.u.pir));
    for reason in XSTOP_REASON.iter() {
        if be32_to_cpu((*hmi_evt).u.xstop_error.xstop_reason) & reason.xstop_reason != 0 {
            printk(b"%s\t[Unit: %-3s] %s\n\0".as_ptr() as *const i8, level, reason.unit_failed, reason.description);
        }
    }
}

unsafe fn print_nx_checkstop_reason(level: *const i8, hmi_evt: *mut OpalHMIEvent) {
    static XSTOP_REASON: [xstop_reason; 14] = [
        xstop_reason { xstop_reason: NX_CHECKSTOP_SHM_INVAL_STATE_ERR, unit_failed: b"DMA & Engine\0".as_ptr() as *const i8, description: b"SHM invalid state error\0".as_ptr() as *const i8 },
        xstop_reason { xstop_reason: NX_CHECKSTOP_DMA_INVAL_STATE_ERR_1, unit_failed: b"DMA & Engine\0".as_ptr() as *const i8, description: b"DMA invalid state error bit 15\0".as_ptr() as *const i8 },
        xstop_reason { xstop_reason: NX_CHECKSTOP_DMA_INVAL_STATE_ERR_2, unit_failed: b"DMA & Engine\0".as_ptr() as *const i8, description: b"DMA invalid state error bit 16\0".as_ptr() as *const i8 },
        xstop_reason { xstop_reason: NX_CHECKSTOP_DMA_CH0_INVAL_STATE_ERR, unit_failed: b"DMA & Engine\0".as_ptr() as *const i8, description: b"Channel 0 invalid state error\0".as_ptr() as *const i8 },
        xstop_reason { xstop_reason: NX_CHECKSTOP_DMA_CH1_INVAL_STATE_ERR, unit_failed: b"DMA & Engine\0".as_ptr() as *const i8, description: b"Channel 1 invalid state error\0".as_ptr() as *const i8 },
        xstop_reason { xstop_reason: NX_CHECKSTOP_DMA_CH2_INVAL_STATE_ERR, unit_failed: b"DMA & Engine\0".as_ptr() as *const i8, description: b"Channel 2 invalid state error\0".as_ptr() as *const i8 },
        xstop_reason { xstop_reason: NX_CHECKSTOP_DMA_CH3_INVAL_STATE_ERR, unit_failed: b"DMA & Engine\0".as_ptr() as *const i8, description: b"Channel 3 invalid state error\0".as_ptr() as *const i8 },
        xstop_reason { xstop_reason: NX_CHECKSTOP_DMA_CH4_INVAL_STATE_ERR, unit_failed: b"DMA & Engine\0".as_ptr() as *const i8, description: b"Channel 4 invalid state error\0".as_ptr() as *const i8 },
        xstop_reason { xstop_reason: NX_CHECKSTOP_DMA_CH5_INVAL_STATE_ERR, unit_failed: b"DMA & Engine\0".as_ptr() as *const i8, description: b"Channel 5 invalid state error\0".as_ptr() as *const i8 },
        xstop_reason { xstop_reason: NX_CHECKSTOP_DMA_CH6_INVAL_STATE_ERR, unit_failed: b"DMA & Engine\0".as_ptr() as *const i8, description: b"Channel 6 invalid state error\0".as_ptr() as *const i8 },
        xstop_reason { xstop_reason: NX_CHECKSTOP_DMA_CH7_INVAL_STATE_ERR, unit_failed: b"DMA & Engine\0".as_ptr() as *const i8, description: b"Channel 7 invalid state error\0".as_ptr() as *const i8 },
        xstop_reason { xstop_reason: NX_CHECKSTOP_DMA_CRB_UE, unit_failed: b"DMA & Engine\0".as_ptr() as *const i8, description: b"UE error on CRB(CSB address, CCB)\0".as_ptr() as *const i8 },
        xstop_reason { xstop_reason: NX_CHECKSTOP_DMA_CRB_SUE, unit_failed: b"DMA & Engine\0".as_ptr() as *const i8, description: b"SUE error on CRB(CSB address, CCB)\0".as_ptr() as *const i8 },
        xstop_reason { xstop_reason: NX_CHECKSTOP_PBI_ISN_UE, unit_failed: b"PowerBus Interface\0".as_ptr() as *const i8, description: b"CRB Kill ISN received while holding ISN with UE error\0".as_ptr() as *const i8 },
    ];
    if (*hmi_evt).u.xstop_error.xstop_reason == 0 { printk(b"%s\tUnknown NX check stop.\n\0".as_ptr() as *const i8, level); return; }
    printk(b"%s\tNX checkstop on CHIP ID: %x\n\0".as_ptr() as *const i8, level, be32_to_cpu((*hmi_evt).u.xstop_error.u.chip_id));
    for reason in XSTOP_REASON.iter() { if be32_to_cpu((*hmi_evt).u.xstop_error.xstop_reason) & reason.xstop_reason != 0 { printk(b"%s\t[Unit: %-3s] %s\n\0".as_ptr() as *const i8, level, reason.unit_failed, reason.description); } }
}

unsafe fn print_npu_checkstop_reason(level: *const i8, hmi_evt: *mut OpalHMIEvent) {
    if (*hmi_evt).u.xstop_error.xstop_reason == 0 { printk(b"%s\tNPU checkstop on chip %x\n\0".as_ptr() as *const i8, level, be32_to_cpu((*hmi_evt).u.xstop_error.u.chip_id)); return; }
    let reason_count = core::mem::size_of_val(&(*hmi_evt).u.xstop_error.xstop_reason) / core::mem::size_of::<u8>();
    for i in 0..reason_count { let reason = ((*hmi_evt).u.xstop_error.xstop_reason >> (8 * i)) & 0xFF; if reason != 0 { printk(b"%s\tNPU checkstop on chip %x: FIR%d bit %d is set\n\0".as_ptr() as *const i8, level, be32_to_cpu((*hmi_evt).u.xstop_error.u.chip_id), reason >> 6, reason & 0x3F); } }
}

unsafe fn print_checkstop_reason(level: *const i8, hmi_evt: *mut OpalHMIEvent) {
    match (*hmi_evt).u.xstop_error.xstop_type { CHECKSTOP_TYPE_CORE => print_core_checkstop_reason(level, hmi_evt), CHECKSTOP_TYPE_NX => print_nx_checkstop_reason(level, hmi_evt), CHECKSTOP_TYPE_NPU => print_npu_checkstop_reason(level, hmi_evt), type_ => printk(b"%s\tUnknown Malfunction Alert of type %d\n\0".as_ptr() as *const i8, level, type_), }
}

unsafe fn print_hmi_event_info(hmi_evt: *mut OpalHMIEvent) {
    let (level, sevstr) = match (*hmi_evt).severity { OpalHMI_SEV_NO_ERROR => (KERN_INFO, b"Harmless\0".as_ptr()), OpalHMI_SEV_WARNING => (KERN_WARNING, b"\0".as_ptr()), OpalHMI_SEV_ERROR_SYNC => (KERN_ERR, b"Severe\0".as_ptr()), _ => (KERN_ERR, b"Fatal\0".as_ptr()) };
    static HMI_ERROR_TYPES: [&[u8]; 13] = [b"Malfunction Alert\0", b"Processor Recovery done\0", b"Processor recovery occurred again\0", b"Processor recovery occurred for masked error\0", b"Timer facility experienced an error\0", b"TFMR SPR is corrupted\0", b"UPS (Uninterrupted Power System) Overflow indication\0", b"An XSCOM operation failure\0", b"An XSCOM operation completed\0", b"SCOM has set a reserved FIR bit to cause recovery\0", b"Debug trigger has set a reserved FIR bit to cause recovery\0", b"A hypervisor resource error occurred\0", b"CAPP recovery process is in progress\0"];
    if (*hmi_evt).version < OpalHMIEvt_V1 { pr_err(b"HMI Interrupt, Unknown event version %d !\n\0".as_ptr() as *const i8, (*hmi_evt).version); return; }
    if (*hmi_evt).severity != OpalHMI_SEV_NO_ERROR || __ratelimit(core::ptr::null_mut()) { printk(b"%s%s Hypervisor Maintenance interrupt [%s]\n\0".as_ptr() as *const i8, level, sevstr.as_ptr(), if (*hmi_evt).disposition == OpalHMI_DISPOSITION_RECOVERED { b"Recovered\0".as_ptr() } else { b"Not recovered\0".as_ptr() }); let error_info = if ((*hmi_evt).type as usize) < HMI_ERROR_TYPES.len() { HMI_ERROR_TYPES[(*hmi_evt).type as usize].as_ptr() } else { b"Unknown\0".as_ptr() }; printk(b"%s Error detail: %s\n\0".as_ptr() as *const i8, level, error_info); printk(b"%s\tHMER: %016llx\n\0".as_ptr() as *const i8, level, be64_to_cpu((*hmi_evt).hmer)); if (*hmi_evt).type == OpalHMI_ERROR_TFAC || (*hmi_evt).type == OpalHMI_ERROR_TFMR_PARITY { printk(b"%s\tTFMR: %016llx\n\0".as_ptr() as *const i8, level, be64_to_cpu((*hmi_evt).tfmr)); } }
    if (*hmi_evt).version >= OpalHMIEvt_V2 && (*hmi_evt).type == OpalHMI_ERROR_MALFUNC_ALERT { print_checkstop_reason(level, hmi_evt); }
}

unsafe fn hmi_event_handler(_work: *mut work_struct) {
    let mut unrecoverable = 0;
    let mut flags = 0UL;
    spin_lock_irqsave(&mut OPAL_HMI_EVT_LOCK, &mut flags);
    while !list_empty(&OPAL_HMI_EVT_LIST) { let msg_node = list_entry(OPAL_HMI_EVT_LIST.next, OpalHmiEvtNode, list); list_del(&mut (*msg_node).list); spin_unlock_irqrestore(&mut OPAL_HMI_EVT_LOCK, flags); print_hmi_event_info(&mut (*msg_node).hmi_evt); let disposition = (*msg_node).hmi_evt.disposition; kfree(msg_node as *mut core::ffi::c_void); if disposition != OpalHMI_DISPOSITION_RECOVERED { unrecoverable = 1; } spin_lock_irqsave(&mut OPAL_HMI_EVT_LOCK, &mut flags); }
    spin_unlock_irqrestore(&mut OPAL_HMI_EVT_LOCK, flags);
    if unrecoverable != 0 { let mut msg = opal_msg {}; while opal_get_msg(__pa(&mut msg) as u64, core::mem::size_of::<opal_msg>()) == OPAL_SUCCESS { if be32_to_cpu(msg.msg_type) != OPAL_MSG_HMI_EVT { continue; } print_hmi_event_info(msg.params.as_mut_ptr() as *mut OpalHMIEvent); } pnv_platform_error_reboot(core::ptr::null_mut(), b"Unrecoverable HMI exception\0".as_ptr() as *const i8); }
}

static mut HMI_EVENT_WORK: work_struct = work_struct {};

unsafe fn opal_handle_hmi_event(_nb: *mut notifier_block, msg_type: usize, msg: *mut core::ffi::c_void) -> i32 {
    if msg_type != OPAL_MSG_HMI_EVT { return 0; }
    let hmi_evt = &(*(msg as *mut opal_msg)).params[0] as *const _ as *mut OpalHMIEvent;
    let msg_node = kzalloc(core::mem::size_of::<OpalHmiEvtNode>(), GFP_ATOMIC) as *mut OpalHmiEvtNode;
    if msg_node.is_null() { pr_err(b"HMI: out of memory, Opal message event not handled\n\0".as_ptr() as *const i8); return -12; }
    core::ptr::copy_nonoverlapping(hmi_evt, &mut (*msg_node).hmi_evt, 1);
    let mut flags = 0UL; spin_lock_irqsave(&mut OPAL_HMI_EVT_LOCK, &mut flags); list_add(&mut (*msg_node).list, &mut OPAL_HMI_EVT_LIST); spin_unlock_irqrestore(&mut OPAL_HMI_EVT_LOCK, flags); schedule_work(&mut HMI_EVENT_WORK); 0
}

static mut OPAL_HMI_HANDLER_NB: notifier_block = notifier_block { notifier_call: Some(opal_handle_hmi_event), next: core::ptr::null_mut(), priority: 0 };

unsafe fn opal_hmi_handler_init() -> i32 {
    if OPAL_HMI_HANDLER_NB_INIT == 0 { let ret = opal_message_notifier_register(OPAL_MSG_HMI_EVT, &mut OPAL_HMI_HANDLER_NB); if ret != 0 { pr_err(b"%s: Can't register OPAL event notifier (%d)\n\0".as_ptr() as *const i8, b"opal_hmi_handler_init\0".as_ptr(), ret); return ret; } OPAL_HMI_HANDLER_NB_INIT = 1; } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
