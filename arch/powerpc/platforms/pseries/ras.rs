// SPDX-License-Identifier: GPL-2.0-or-later
/* Copyright (C) 2001 Dave Engebretsen IBM Corporation */

// Linux and architecture headers provide the following types, constants,
// globals, macros, and functions in the surrounding translation unit.

#[repr(C, packed)]
pub struct pseries_mc_errorlog {
    pub fru_id: u32,
    pub proc_id: u32,
    pub error_type: u8,
    pub sub_err_type: u8,
    pub reserved_1: [u8; 6],
    pub effective_address: u64,
    pub logical_address: u64,
}

#[repr(C, packed)]
pub struct epow_errorlog {
    pub sensor_value: u8,
    pub event_modifier: u8,
    pub extended_modifier: u8,
    pub reserved: u8,
    pub platform_reason: u8,
}

const EPOW_SENSOR_TOKEN: i32 = 9;
const EPOW_SENSOR_INDEX: i32 = 0;
const MC_ERROR_TYPE_UE: u8 = 0x00;
const MC_ERROR_TYPE_SLB: u8 = 0x01;
const MC_ERROR_TYPE_ERAT: u8 = 0x02;
const MC_ERROR_TYPE_UNKNOWN: u8 = 0x03;
const MC_ERROR_TYPE_TLB: u8 = 0x04;
const MC_ERROR_TYPE_D_CACHE: u8 = 0x05;
const MC_ERROR_TYPE_I_CACHE: u8 = 0x07;
const MC_ERROR_TYPE_CTRL_MEM_ACCESS: u8 = 0x08;
const MC_ERROR_UE_INDETERMINATE: u8 = 0;
const MC_ERROR_UE_IFETCH: u8 = 1;
const MC_ERROR_UE_PAGE_TABLE_WALK_IFETCH: u8 = 2;
const MC_ERROR_UE_LOAD_STORE: u8 = 3;
const MC_ERROR_UE_PAGE_TABLE_WALK_LOAD_STORE: u8 = 4;
const UE_EFFECTIVE_ADDR_PROVIDED: u8 = 0x40;
const UE_LOGICAL_ADDR_PROVIDED: u8 = 0x20;
const MC_EFFECTIVE_ADDR_PROVIDED: u8 = 0x80;
const MC_ERROR_SLB_PARITY: u8 = 0;
const MC_ERROR_SLB_MULTIHIT: u8 = 1;
const MC_ERROR_SLB_INDETERMINATE: u8 = 2;
const MC_ERROR_ERAT_PARITY: u8 = 1;
const MC_ERROR_ERAT_MULTIHIT: u8 = 2;
const MC_ERROR_ERAT_INDETERMINATE: u8 = 3;
const MC_ERROR_TLB_PARITY: u8 = 1;
const MC_ERROR_TLB_MULTIHIT: u8 = 2;
const MC_ERROR_TLB_INDETERMINATE: u8 = 3;
const MC_ERROR_CTRL_MEM_ACCESS_PTABLE_WALK: u8 = 0;
const MC_ERROR_CTRL_MEM_ACCESS_OP_ACCESS: u8 = 1;

const EPOW_SHUTDOWN_NORMAL: i8 = 1;
const EPOW_SHUTDOWN_ON_UPS: i8 = 2;
const EPOW_SHUTDOWN_LOSS_OF_CRITICAL_FUNCTIONS: i8 = 3;
const EPOW_SHUTDOWN_AMBIENT_TEMPERATURE_TOO_HIGH: i8 = 4;
const EPOW_RESET: u8 = 0;
const EPOW_WARN_COOLING: u8 = 1;
const EPOW_WARN_POWER: u8 = 2;
const EPOW_SYSTEM_SHUTDOWN: u8 = 3;
const EPOW_SYSTEM_HALT: u8 = 4;
const EPOW_MAIN_ENCLOSURE: u8 = 5;
const EPOW_POWER_OFF: u8 = 7;

static mut ras_log_buf: [u8; RTAS_ERROR_LOG_MAX as usize] = [0; RTAS_ERROR_LOG_MAX as usize];
static mut ras_check_exception_token: i32 = 0;
static mut num_epow_events: i32 = 0;
static mut ras_log_buf_lock: spinlock_t = spinlock_t::new();

pub unsafe fn init_ras_hotplug_IRQ()->i32 { let np=of_find_node_by_path(c"/event-sources/hot-plug-events".as_ptr()); if !np.is_null(){ if dlpar_workqueue_init()==0 { request_event_sources_irqs(np,ras_hotplug_interrupt,c"RAS_HOTPLUG".as_ptr()); } of_node_put(np); } 0 }
pub unsafe fn init_ras_IRQ()->i32 { ras_check_exception_token=rtas_function_token(RTAS_FN_CHECK_EXCEPTION); let np=of_find_node_by_path(c"/event-sources/internal-errors".as_ptr()); if !np.is_null(){request_event_sources_irqs(np,ras_error_interrupt,c"RAS_ERROR".as_ptr());of_node_put(np);} let np=of_find_node_by_path(c"/event-sources/epow-events".as_ptr()); if !np.is_null(){request_event_sources_irqs(np,ras_epow_interrupt,c"RAS_EPOW".as_ptr());of_node_put(np);} 0 }

#[inline]
unsafe fn rtas_mc_error_sub_type(mlog: *const pseries_mc_errorlog) -> u8 {
    match (*mlog).error_type {
        MC_ERROR_TYPE_UE => (*mlog).sub_err_type & 0x07,
        MC_ERROR_TYPE_SLB | MC_ERROR_TYPE_ERAT | MC_ERROR_TYPE_TLB => (*mlog).sub_err_type & 0x03,
        MC_ERROR_TYPE_CTRL_MEM_ACCESS => ((*mlog).sub_err_type & 0x70) >> 4,
        _ => 0,
    }
}

extern "C" {
    static mut fwnmi_active: bool;
    static mut local_paca: *mut paca_struct;
    static mut rtas: rtas_t;
    static mut ibm_nmi_interlock_token: i32;
    fn rtas_function_token(i: i32) -> i32;
    fn of_find_node_by_path(p: *const i8) -> *mut device_node;
    fn of_node_put(p: *mut device_node);
    fn dlpar_workqueue_init() -> i32;
    fn request_event_sources_irqs(n: *mut device_node, f: unsafe extern "C" fn(i32,*mut core::ffi::c_void)->irqreturn_t, s: *const i8);
    fn rtas_call(token:i32,nargs:i32,nret:i32,rets:*mut i32,...)->i32;
    fn rtas_call_unlocked(a:*mut rtas_args, token:i32,nargs:i32,nret:i32,rets:*mut i32);
    fn rtas_get_sensor_fast(t:i32,i:i32,s:*mut i32)->i32;
    fn rtas_get_error_log_max()->i32;
    fn virq_to_hw(i:i32)->i32;
    fn get_pseries_errorlog(l:*mut rtas_error_log,id:i32)->*mut pseries_errorlog;
    fn log_error(b:*mut u8,t:i32,f:i32);
    fn queue_hotplug_event(l:*mut pseries_hp_errorlog);
    fn rtas_error_severity(l:*mut rtas_error_log)->i32;
    fn rtas_error_disposition(l:*mut rtas_error_log)->i32;
    fn rtas_error_extended(l:*mut rtas_error_log)->bool;
    fn rtas_error_extended_log_length(l:*mut rtas_error_log)->u32;
    fn be64_to_cpu(x:u64)->u64;
    fn be32_to_cpu(x:u32)->u32;
    fn __pa(p:*const u8)->u64;
    fn __va(p:u64)->*mut u64;
    fn emergency_sync(); fn kernel_power_off(); fn orderly_poweroff(force:bool);
    fn memset(d:*mut u8,v:i32,n:usize)->*mut u8; fn memcpy(d:*mut u8,s:*const u8,n:usize)->*mut u8;
    fn smp_handle_nmi_ipi(r:*mut pt_regs)->i32; fn regs_set_return_ip(r:*mut pt_regs,v:u64); fn regs_set_return_msr(r:*mut pt_regs,v:u64);
}

// Remaining kernel structs and constants are supplied by the included headers.
pub unsafe fn handle_system_shutdown(event_modifier: i8) {
    match event_modifier {
        EPOW_SHUTDOWN_NORMAL => { pr_emerg!("Power off requested\n"); orderly_poweroff(true); }
        EPOW_SHUTDOWN_ON_UPS => pr_emerg!("Loss of system power detected. System is running on UPS/battery. Check RTAS error log for details\n"),
        EPOW_SHUTDOWN_LOSS_OF_CRITICAL_FUNCTIONS => { pr_emerg!("Loss of system critical functions detected. Check RTAS error log for details\n"); orderly_poweroff(true); }
        EPOW_SHUTDOWN_AMBIENT_TEMPERATURE_TOO_HIGH => { pr_emerg!("High ambient temperature detected. Check RTAS error log for details\n"); orderly_poweroff(true); }
        _ => pr_err!("Unknown power/cooling shutdown event (modifier = %d)\n", event_modifier),
    }
}

pub unsafe fn rtas_parse_epow_errlog(log: *mut rtas_error_log) {
    let p = get_pseries_errorlog(log, PSERIES_ELOG_SECT_ID_EPOW); if p.is_null() { return; }
    let e = (*p).data as *mut epow_errorlog;
    let action_code = (*e).sensor_value & 0xf; let modifier = (*e).event_modifier & 0xf;
    match action_code {
        EPOW_RESET => if num_epow_events != 0 { pr_info!("Non critical power/cooling issue cleared\n"); num_epow_events -= 1; },
        EPOW_WARN_COOLING => pr_info!("Non-critical cooling issue detected. Check RTAS error log for details\n"),
        EPOW_WARN_POWER => pr_info!("Non-critical power issue detected. Check RTAS error log for details\n"),
        EPOW_SYSTEM_SHUTDOWN => handle_system_shutdown(modifier as i8),
        EPOW_SYSTEM_HALT => { pr_emerg!("Critical power/cooling issue detected. Check RTAS error log for details. Powering off.\n"); orderly_poweroff(true); },
        EPOW_MAIN_ENCLOSURE | EPOW_POWER_OFF => { pr_emerg!("System about to lose power. Check RTAS error log for details. Powering off immediately.\n"); emergency_sync(); kernel_power_off(); },
        _ => pr_err!("Unknown power/cooling event (action code  = %d)\n", action_code),
    }
    if action_code != EPOW_RESET { num_epow_events += 1; }
}

pub unsafe extern "C" fn ras_hotplug_interrupt(irq:i32,_dev_id:*mut core::ffi::c_void)->irqreturn_t {
    spin_lock(&mut ras_log_buf_lock);
    rtas_call(ras_check_exception_token,6,1,core::ptr::null_mut(),RTAS_VECTOR_EXTERNAL_INTERRUPT,virq_to_hw(irq),RTAS_HOTPLUG_EVENTS,0,__pa(ras_log_buf.as_ptr()),rtas_get_error_log_max());
    let p=get_pseries_errorlog(ras_log_buf.as_mut_ptr() as *mut rtas_error_log,PSERIES_ELOG_SECT_ID_HOTPLUG);
    let h=(*p).data as *mut pseries_hp_errorlog;
    if (*h).resource==PSERIES_HP_ELOG_RESOURCE_MEM || (*h).resource==PSERIES_HP_ELOG_RESOURCE_CPU || (*h).resource==PSERIES_HP_ELOG_RESOURCE_PMEM { queue_hotplug_event(h); } else { log_error(ras_log_buf.as_mut_ptr(),ERR_TYPE_RTAS_LOG,0); }
    spin_unlock(&mut ras_log_buf_lock); IRQ_HANDLED
}

pub unsafe extern "C" fn ras_epow_interrupt(irq:i32,_dev_id:*mut core::ffi::c_void)->irqreturn_t {
    let mut state=0; rtas_get_sensor_fast(EPOW_SENSOR_TOKEN,EPOW_SENSOR_INDEX,&mut state); let critical=if state>3 {1} else {0};
    spin_lock(&mut ras_log_buf_lock);
    rtas_call(ras_check_exception_token,6,1,core::ptr::null_mut(),RTAS_VECTOR_EXTERNAL_INTERRUPT,virq_to_hw(irq),RTAS_EPOW_WARNING,critical,__pa(ras_log_buf.as_ptr()),rtas_get_error_log_max());
    log_error(ras_log_buf.as_mut_ptr(),ERR_TYPE_RTAS_LOG,0); rtas_parse_epow_errlog(ras_log_buf.as_mut_ptr() as *mut rtas_error_log); spin_unlock(&mut ras_log_buf_lock); IRQ_HANDLED
}

pub unsafe extern "C" fn ras_error_interrupt(irq:i32,_dev_id:*mut core::ffi::c_void)->irqreturn_t {
    spin_lock(&mut ras_log_buf_lock);
    let status=rtas_call(ras_check_exception_token,6,1,core::ptr::null_mut(),RTAS_VECTOR_EXTERNAL_INTERRUPT,virq_to_hw(irq),RTAS_INTERNAL_ERROR,1,__pa(ras_log_buf.as_ptr()),rtas_get_error_log_max());
    let e=ras_log_buf.as_mut_ptr() as *mut rtas_error_log; let fatal=status==0 && rtas_error_severity(e)>=RTAS_SEVERITY_ERROR_SYNC;
    log_error(ras_log_buf.as_mut_ptr(),ERR_TYPE_RTAS_LOG,if fatal {1} else {0});
    if fatal { pr_emerg!("Fatal hardware error detected. Check RTAS error log for details. Powering off immediately\n"); emergency_sync(); kernel_power_off(); } else { pr_err!("Recoverable hardware error detected\n"); }
    spin_unlock(&mut ras_log_buf_lock); IRQ_HANDLED
}

unsafe fn fwnmi_get_errlog()->*mut rtas_error_log { (*local_paca).mce_data_buf as *mut rtas_error_log }
unsafe fn fwnmi_get_savep(regs:*mut pt_regs)->*mut u64 {
    let p=(*regs).gpr[3] & !(0x3u64<<62);
    if !(((p>=0x7000)&&(p<=0x8000-16))||((p>=rtas.base)&&(p<=rtas.base+rtas.size-16))) { printk!(KERN_ERR,"FWNMI: corrupt r3 0x%016lx\n",(*regs).gpr[3]); return core::ptr::null_mut(); }
    __va(p)
}

unsafe fn fwnmi_get_errinfo(regs:*mut pt_regs)->*mut rtas_error_log {
    let savep=fwnmi_get_savep(regs); if savep.is_null(){return core::ptr::null_mut();} (*regs).gpr[3]=be64_to_cpu(*savep);
    let h=savep.add(1) as *mut rtas_error_log; let ext=if rtas_error_extended(h){rtas_error_extended_log_length(h) as usize}else{0}; let len=core::cmp::min(core::mem::size_of::<rtas_error_log>()+ext,RTAS_ERROR_LOG_MAX as usize);
    memset((*local_paca).mce_data_buf as *mut u8,0,RTAS_ERROR_LOG_MAX as usize); memcpy((*local_paca).mce_data_buf as *mut u8,h as *const u8,len); (*local_paca).mce_data_buf as *mut rtas_error_log
}
unsafe fn fwnmi_release_errinfo(){ let mut a=core::mem::zeroed::<rtas_args>(); rtas_call_unlocked(&mut a,ibm_nmi_interlock_token,0,1,core::ptr::null_mut()); let ret=be32_to_cpu(a.rets[0]); if ret!=0 { printk!(KERN_ERR,"FWNMI: nmi-interlock failed: %d\n",ret); } }

pub unsafe fn pSeries_system_reset_exception(regs:*mut pt_regs)->i32 { if fwnmi_active { let p=fwnmi_get_savep(regs); if !p.is_null(){(*regs).gpr[3]=be64_to_cpu(*p);} } if smp_handle_nmi_ipi(regs)!=0 {1}else{0} }
pub unsafe fn pSeries_machine_check_log_err(){log_error(fwnmi_get_errlog() as *mut u8,ERR_TYPE_RTAS_LOG,0);}
pub unsafe fn pseries_machine_check_realmode(regs:*mut pt_regs)->i64 { if fwnmi_active { let e=fwnmi_get_errinfo(regs); let _=e; fwnmi_release_errinfo(); } 0 }

pub unsafe fn mce_handle_err_realmode(disposition:i32,error_type:u8)->i32 { if disposition==RTAS_DISP_NOT_RECOVERED { if error_type==MC_ERROR_TYPE_ERAT {flush_erat();return RTAS_DISP_FULLY_RECOVERED;} if error_type==MC_ERROR_TYPE_SLB {flush_and_reload_slb();return RTAS_DISP_FULLY_RECOVERED;} } else if disposition==RTAS_DISP_LIMITED_RECOVERY {pr_err!("MCE: limited recovery, system may be degraded\n");return RTAS_DISP_FULLY_RECOVERED;} disposition }
pub unsafe fn mce_handle_error(regs:*mut pt_regs,errp:*mut rtas_error_log)->i32 { if errp.is_null(){return 0;} let d=rtas_error_disposition(errp); let p=get_pseries_errorlog(errp,PSERIES_ELOG_SECT_ID_MCE); if p.is_null(){return d;} mce_handle_err_realmode(d,(*(p)).data[0]) }
pub unsafe fn pSeries_machine_check_exception(_regs:*mut pt_regs)->i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
