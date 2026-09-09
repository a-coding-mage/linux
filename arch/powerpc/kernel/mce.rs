// SPDX-License-Identifier: GPL-2.0-or-later
/* Machine check exception handling. */

// Dependencies are supplied by the surrounding kernel translation unit.

extern "C" {
    fn blocking_notifier_chain_register(list: *mut (), nb: *mut ()) -> i32;
    fn blocking_notifier_chain_unregister(list: *mut (), nb: *mut ()) -> i32;
    fn arch_irq_work_raise();
    fn schedule_work(work: *mut WorkStruct);
    fn blocking_notifier_call_chain(list: *mut (), val: u64, data: *mut MachineCheckEvent);
    fn search_kernel_exception_table(ip: u64) -> *const ExceptionTableEntry;
    fn extable_fixup(entry: *const ExceptionTableEntry) -> u64;
    fn regs_set_return_ip(regs: *mut PtRegs, ip: u64);
    fn memory_failure(pfn: u64, flags: u64);
    fn add_taint(taint: i32, lockdep: i32);
    fn machine_check_print_event_info(evt: *mut MachineCheckEvent, user_mode: bool, in_guest: bool);
    fn hv_nmi_check_nonrecoverable(regs: *mut PtRegs);
    fn user_mode(regs: *mut PtRegs) -> bool;
    fn wait_for_subcore_guest_exit();
    fn wait_for_tb_resync();
    fn smp_processor_id() -> i32;
    fn mfspr(spr: i32) -> u64;
    fn mtspr(spr: i32, value: u64);
    fn of_get_cpu_node(cpu: i32, thread: *mut ()) -> *mut DeviceNode;
    fn of_node_put(node: *mut DeviceNode);
    fn preempt_disable();
    fn preempt_enable();
    fn ppc64_bolted_size() -> u64;
    fn early_cpu_to_node(cpu: i32) -> i32;
    fn memblock_alloc_try_nid(size: usize, align: usize, min: u64, max: u64, nid: i32) -> *mut MceInfo;
    fn panic(msg: *const u8) -> !;
}

#[repr(C)] pub struct WorkStruct { _private: [u8; 0] }
#[repr(C)] pub struct PtRegs { pub msr: u64, pub gpr: [u64; 32], pub nip: u64 }
#[repr(C)] pub struct MachineCheckEvent { pub version: u8, pub srr0: u64, pub srr1: u64, pub gpr3: u64, pub in_use: u8, pub cpu: i32, pub disposition: i32, pub initiator: i32, pub severity: i32, pub sync_error: i32, pub error_class: usize, pub error_type: i32, pub u: MachineCheckUnion }
#[repr(C)] pub union MachineCheckUnion { pub ue_error: UeError, pub slb_error: SlbError, pub erat_error: EratError, pub tlb_error: TlbError, pub user_error: UserError, pub ra_error: RaError, pub link_error: LinkError }
#[repr(C)] pub struct UeError { pub ue_error_type: usize, pub ignore_event: bool, pub effective_address_provided: bool, pub effective_address: u64, pub physical_address_provided: bool, pub physical_address: u64 }
#[repr(C)] pub struct SlbError { pub slb_error_type: usize, pub effective_address_provided: bool, pub effective_address: u64 }
#[repr(C)] pub struct EratError { pub erat_error_type: usize, pub effective_address_provided: bool, pub effective_address: u64 }
#[repr(C)] pub struct TlbError { pub tlb_error_type: usize, pub effective_address_provided: bool, pub effective_address: u64 }
#[repr(C)] pub struct UserError { pub user_error_type: usize, pub effective_address_provided: bool, pub effective_address: u64 }
#[repr(C)] pub struct RaError { pub ra_error_type: usize, pub effective_address_provided: bool, pub effective_address: u64 }
#[repr(C)] pub struct LinkError { pub link_error_type: usize, pub effective_address_provided: bool, pub effective_address: u64 }
#[repr(C)] pub struct MceErrorInfo { pub error_type: i32, pub initiator: i32, pub severity: i32, pub sync_error: i32, pub error_class: usize, pub ignore_event: bool, pub ue_error_type: usize, pub slb_error_type: usize, pub erat_error_type: usize, pub tlb_error_type: usize, pub user_error_type: usize, pub ra_error_type: usize, pub link_error_type: usize }
#[repr(C)] pub struct MceInfo { pub mce_nest_count: i32, pub mce_ue_count: i32, pub mce_queue_count: i32, pub mce_event: [MachineCheckEvent; 32], pub mce_ue_event_queue: [MachineCheckEvent; 32], pub mce_event_queue: [MachineCheckEvent; 32] }
#[repr(C)] pub struct ExceptionTableEntry { _private: [u8; 0] }
#[repr(C)] pub struct DeviceNode { _private: [u8; 0] }

const MAX_MC_EVT: i32 = 32;
const MCE_EVENT_RELEASE: bool = true;
const ULONG_MAX: u64 = u64::MAX;
const MCE_V1: u8 = 1;
const MSR_RI: u64 = 1 << 1;

static mut local_paca: *mut Paca = core::ptr::null_mut();
#[repr(C)] pub struct Paca { pub mce_info: *mut MceInfo, pub paca_index: i32, pub mce_pending_irq_work: i32, pub hmi_p9_special_emu: i32, pub hmi_irqs: i32 }

static mut mce_notifier_list: *mut () = core::ptr::null_mut();
static mut mce_ue_event_work: WorkStruct = WorkStruct { _private: [] };

pub unsafe fn mce_register_notifier(nb: *mut ()) -> i32 { blocking_notifier_chain_register(mce_notifier_list, nb) }
pub unsafe fn mce_unregister_notifier(nb: *mut ()) -> i32 { blocking_notifier_chain_unregister(mce_notifier_list, nb) }

unsafe fn mce_set_error_info(mce: *mut MachineCheckEvent, err: *mut MceErrorInfo) {
    (*mce).error_type = (*err).error_type;
    match (*err).error_type {
        0 => (*mce).u.ue_error.ue_error_type = (*err).ue_error_type,
        1 => (*mce).u.slb_error.slb_error_type = (*err).slb_error_type,
        2 => (*mce).u.erat_error.erat_error_type = (*err).erat_error_type,
        3 => (*mce).u.tlb_error.tlb_error_type = (*err).tlb_error_type,
        4 => (*mce).u.user_error.user_error_type = (*err).user_error_type,
        5 => (*mce).u.ra_error.ra_error_type = (*err).ra_error_type,
        6 => (*mce).u.link_error.link_error_type = (*err).link_error_type,
        _ => {}
    }
}

pub unsafe fn mce_irq_work_queue() { arch_irq_work_raise(); set_mce_pending_irq_work(); }

pub unsafe fn save_mce_event(regs: *mut PtRegs, handled: i64, err: *mut MceErrorInfo, nip: u64, addr: u64, phys_addr: u64) {
    let info = (*local_paca).mce_info; let index = (*info).mce_nest_count; (*info).mce_nest_count += 1;
    if index >= MAX_MC_EVT { return; }
    let mce = &mut (*info).mce_event[index as usize];
    mce.version=MCE_V1; mce.srr0=nip; mce.srr1=(*regs).msr; mce.gpr3=(*regs).gpr[3]; mce.in_use=1; mce.cpu=(*local_paca).paca_index;
    mce.disposition=if handled != 0 && (*regs).msr & MSR_RI != 0 { 0 } else { 1 };
    mce.initiator=(*err).initiator; mce.severity=(*err).severity; mce.sync_error=(*err).sync_error; mce.error_class=(*err).error_class; mce_set_error_info(mce,err);
    if mce.error_type == 0 { mce.u.ue_error.ignore_event=(*err).ignore_event; }
    if mce.disposition != 0 { mce_irq_work_queue(); } if addr == 0 { return; }
    match mce.error_type { 0 => {mce.u.ue_error.effective_address_provided=true;mce.u.ue_error.effective_address=addr;if phys_addr != ULONG_MAX {mce.u.ue_error.physical_address_provided=true;mce.u.ue_error.physical_address=phys_addr;machine_check_ue_event(mce);}}, _ => {} }
}

pub unsafe fn get_mce_event(mce: *mut MachineCheckEvent, release: bool) -> i32 { let info=(*local_paca).mce_info; let index=(*info).mce_nest_count-1;if index<0{return 0;}if index<MAX_MC_EVT {if !mce.is_null(){*mce=(*info).mce_event[index as usize];}if release{(*info).mce_event[index as usize].in_use=0;}if release{(*info).mce_nest_count-=1;}return 1}if release{(*info).mce_nest_count-=1;}0 }
pub unsafe fn release_mce_event(){get_mce_event(core::ptr::null_mut(),true);}
unsafe fn machine_check_ue_work(){schedule_work(&mut mce_ue_event_work);}
unsafe fn machine_check_ue_event(evt:*mut MachineCheckEvent){let info=(*local_paca).mce_info;let index=(*info).mce_ue_count;(*info).mce_ue_count+=1;if index<MAX_MC_EVT{(*info).mce_ue_event_queue[index as usize]=*evt;}else{(*info).mce_ue_count-=1;}}
pub unsafe fn machine_check_queue_event(){let mut evt=core::mem::zeroed();if get_mce_event(&mut evt,true)==0{return;}let info=(*local_paca).mce_info;let index=(*info).mce_queue_count;(*info).mce_queue_count+=1;if index>=MAX_MC_EVT{(*info).mce_queue_count-=1;return;}(*info).mce_event_queue[index as usize]=evt;mce_irq_work_queue();}
pub unsafe fn set_mce_pending_irq_work(){(*local_paca).mce_pending_irq_work=1;} pub unsafe fn clear_mce_pending_irq_work(){(*local_paca).mce_pending_irq_work=0;}

pub unsafe fn mce_common_process_ue(regs: *mut PtRegs, err: *mut MceErrorInfo) {
    let entry=search_kernel_exception_table((*regs).nip); if !entry.is_null(){(*err).ignore_event=true;regs_set_return_ip(regs,extable_fixup(entry));}
}
unsafe fn machine_process_ue_event(_work:*mut WorkStruct){let info=(*local_paca).mce_info;while (*info).mce_ue_count>0{let i=(*info).mce_ue_count-1;let evt=&mut (*info).mce_ue_event_queue[i as usize];blocking_notifier_call_chain(mce_notifier_list,0,evt);(*info).mce_ue_count-=1;}}
unsafe fn machine_check_process_queued_event(){let info=(*local_paca).mce_info;while (*info).mce_queue_count>0{let i=(*info).mce_queue_count-1;let evt=&mut (*info).mce_event_queue[i as usize];machine_check_print_event_info(evt,false,false);(*info).mce_queue_count-=1;}}
pub unsafe fn mce_run_irq_context_handlers(){if (*local_paca).mce_pending_irq_work!=0{machine_check_process_queued_event();machine_check_ue_work();clear_mce_pending_irq_work();}}

pub unsafe fn machine_check_early(regs:*mut PtRegs)->i64{hv_nmi_check_nonrecoverable(regs);0}
#[repr(C)] pub enum HmerDebugTrigFunction{DtrigUnknown,DtrigVectorCi,DtrigSuspendEscape}
static mut hmer_debug_trig_function:HmerDebugTrigFunction=HmerDebugTrigFunction::DtrigUnknown;
pub unsafe fn hmi_handle_debugtrig(regs:*mut PtRegs)->i64{let hmer=mfspr(0);if hmer==0{return -1;}if matches!(hmer_debug_trig_function,HmerDebugTrigFunction::DtrigVectorCi)&&!regs.is_null()&&user_mode(regs){(*local_paca).hmi_p9_special_emu=1;return 1;}-1}
pub unsafe fn hmi_exception_realmode(regs:*mut PtRegs)->i32{(*local_paca).hmi_irqs+=1;let ret=hmi_handle_debugtrig(regs);if ret>=0{return ret as i32;}wait_for_subcore_guest_exit();wait_for_tb_resync();1}
pub unsafe fn mce_init(){let _=ppc64_bolted_size();for i in 0..1{let info=memblock_alloc_try_nid(core::mem::size_of::<MceInfo>(),core::mem::align_of::<MceInfo>(),0,u64::MAX,early_cpu_to_node(i));if info.is_null(){panic(b"Failed to allocate memory for MCE event data\0".as_ptr());}(*local_paca).mce_info=info;}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
