// SPDX-License-Identifier: GPL-2.0-or-later
/* SGI NMI support routines.  Kernel dependencies are supplied by the surrounding tree. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// C headers and build-time configuration are intentionally represented by external kernel symbols.
extern "C" {
    static mut uv_hub_nmi: *mut uv_hub_nmi_s;
    static mut uv_nmi_action: action_t;
    static mut uv_nmi_cpu_mask: cpumask_var_t;
    static mut uv_nmi_loglevel: c_int;
    fn uv_read_local_mmr(x: c_ulong) -> c_ulong;
    fn uv_write_local_mmr(x: c_ulong, v: c_ulong);
    fn smp_processor_id() -> c_int;
    fn num_online_cpus() -> c_int;
    fn cpu_relax(); fn udelay(x: c_int); fn mdelay(x: c_int);
}

#[repr(C)] pub struct atomic_t { pub counter: c_int }
#[repr(C)] pub struct raw_spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct cpumask { _private: [u8; 0] }
pub type cpumask_var_t = *mut cpumask;
#[repr(C)] pub struct pt_regs { pub ip: c_ulong }
#[repr(C)] pub struct local64_t { pub value: i64 }
#[repr(C)] pub struct uv_cpu_nmi_s { pub pinging: c_int, pub state: c_int, pub queries: local64_t, pub pings: local64_t }
#[repr(C)] pub struct uv_hub_nmi_s { pub nmi_value: c_ulong, pub read_mmr_count: atomic_t, pub in_nmi: atomic_t, pub cpu_owner: atomic_t, pub nmi_count: atomic_t, pub nmi_lock: raw_spinlock_t, pub hub_present: bool, pub pch_owner: bool }

static mut uv_hub_nmi_list: *mut *mut uv_hub_nmi_s = core::ptr::null_mut();
static mut uv_cpu_nmi: uv_cpu_nmi_s = uv_cpu_nmi_s { pinging: 0, state: 0, queries: local64_t { value: 0 }, pings: local64_t { value: 0 } };
static mut uvh_nmi_mmrx: c_ulong = 0; static mut uvh_nmi_mmrx_clear: c_ulong = 0;
static mut uvh_nmi_mmrx_shift: c_int = 0; static mut uvh_nmi_mmrx_type: *const c_char = core::ptr::null();
static mut uvh_nmi_mmrx_supported: c_ulong = 0; static mut uvh_nmi_mmrx_req: c_ulong = 0; static mut uvh_nmi_mmrx_req_shift: c_int = 0;
const NMI_CONTROL_PORT: u16=0x70; const NMI_DUMMY_PORT:u16=0x71; const PAD_OWN_GPP_D_0:c_int=0x2c;
const GPI_NMI_STS_GPP_D_0:c_int=0x164; const GPI_NMI_ENA_GPP_D_0:c_int=0x174; const STS_GPP_D_0_MASK:c_int=1;
const PAD_CFG_DW0_GPP_D_0:c_int=0x4c0; const GPIROUTNMI:c_ulong=1<<17; const PCH_PCR_GPIO_1_BASE:c_ulong=0xfdae0000;
const SLAVE_CLEAR:c_int=0; const SLAVE_CONTINUE:c_int=1; const SLAVE_EXIT:c_int=2;
const UV_NMI_STATE_OUT:c_int=0; const UV_NMI_STATE_IN:c_int=1; const UV_NMI_STATE_DUMP:c_int=2; const UV_NMI_STATE_DUMP_DONE:c_int=3;
#[repr(C)] #[derive(Copy,Clone)] enum action_t { nmi_act_kdump, nmi_act_dump, nmi_act_ips, nmi_act_kdb, nmi_act_kgdb, nmi_act_health, nmi_act_max }
static mut uv_nmi_count: local64_t=local64_t{value:0}; static mut uv_nmi_misses:local64_t=local64_t{value:0};
static mut uv_nmi_ping_count:local64_t=local64_t{value:0}; static mut uv_nmi_ping_misses:local64_t=local64_t{value:0};
static mut uv_nmi_initial_delay:c_int=100; static mut uv_nmi_slave_delay:c_int=100; static mut uv_nmi_loop_delay:c_int=100;
static mut uv_nmi_trigger_delay:c_int=10000; static mut uv_nmi_wait_count:c_int=100; static mut uv_nmi_retry_count:c_int=500;
static mut uv_pch_intr_enable:bool=true; static mut uv_pch_intr_now_enabled:bool=false; static mut uv_pch_init_enable:bool=true; static mut uv_nmi_debug:c_int=0;
static mut uv_in_nmi:atomic_t=atomic_t{counter:0}; static mut uv_nmi_cpu:atomic_t=atomic_t{counter:-1}; static mut uv_nmi_cpus_in_nmi:atomic_t=atomic_t{counter:-1};
static mut uv_nmi_slave_continue:atomic_t=atomic_t{counter:0}; static mut uv_nmi_kexec_failed:atomic_t=atomic_t{counter:0};
static mut nmi_mmr:c_ulong=0; static mut nmi_mmr_clear:c_ulong=0; static mut nmi_mmr_pending:c_ulong=0; static mut pch_base:*mut u64=core::ptr::null_mut();

#[inline] unsafe fn atomic_read(a:*const atomic_t)->c_int{(*a).counter}
#[inline] unsafe fn atomic_set(a:*mut atomic_t,v:c_int){(*a).counter=v}
#[inline] unsafe fn atomic_inc(a:*mut atomic_t){(*a).counter+=1}
#[inline] unsafe fn atomic_dec(a:*mut atomic_t){(*a).counter-=1}
#[inline] unsafe fn local64_inc(a:*mut local64_t){(*a).value+=1}

unsafe fn uv_nmi_setup_mmrs(){ /* MMR selection is architecture supplied. */ }
unsafe fn uv_nmi_test_mmr(h:*mut uv_hub_nmi_s)->c_int { (*h).nmi_value=uv_read_local_mmr(nmi_mmr); atomic_inc(&mut (*h).read_mmr_count); ((*h).nmi_value & nmi_mmr_pending != 0) as c_int }
unsafe fn uv_local_mmr_clear_nmi(){uv_write_local_mmr(nmi_mmr_clear,nmi_mmr_pending)}
unsafe fn uv_reassert_nmi(){ extern "C"{fn outb(v:u8,p:u16);fn inb(p:u16)->u8;} outb(0x8f,NMI_CONTROL_PORT);inb(NMI_DUMMY_PORT);outb(0x0f,NMI_CONTROL_PORT);inb(NMI_DUMMY_PORT); }
unsafe fn uv_test_nmi(h:*mut uv_hub_nmi_s)->c_int { if (*h).hub_present {uv_nmi_test_mmr(h)} else if (*h).pch_owner {-1} else {-1} }
unsafe fn uv_set_in_nmi(cpu:c_int,h:*mut uv_hub_nmi_s)->c_int { if (*h).in_nmi.counter==0 {(*h).in_nmi.counter=1;(*h).cpu_owner.counter=cpu;atomic_inc(&mut (*h).nmi_count);1} else {0} }
unsafe fn uv_check_nmi(h:*mut uv_hub_nmi_s)->c_int { let cpu=smp_processor_id();local64_inc(&mut uv_nmi_count); let n=uv_test_nmi(h); if n>0 {uv_set_in_nmi(cpu,h)} else {0} }
unsafe fn uv_clear_nmi(cpu:c_int){let h=uv_hub_nmi;if cpu==atomic_read(&(*h).cpu_owner){atomic_set(&mut (*h).cpu_owner,-1);atomic_set(&mut (*h).in_nmi,0);if (*h).hub_present{uv_local_mmr_clear_nmi()}else{uv_reassert_nmi()}}}
unsafe fn uv_nmi_wait(_master:c_int){uv_cpu_nmi.state=UV_NMI_STATE_IN}
unsafe fn uv_nmi_nr_cpus_ping(){uv_cpu_nmi.pinging=1;}
unsafe fn uv_nmi_cleanup_mask(){uv_cpu_nmi.pinging=0;uv_cpu_nmi.state=UV_NMI_STATE_OUT;}
unsafe fn uv_nmi_wait_cpus(_first:c_int)->c_int{atomic_set(&mut uv_nmi_cpus_in_nmi,num_online_cpus());0}
unsafe fn uv_nmi_sync_exit(_master:c_int){atomic_dec(&mut uv_nmi_cpus_in_nmi)}
unsafe fn uv_nmi_dump_state_cpu(_cpu:c_int,_r:*mut pt_regs){uv_cpu_nmi.state=UV_NMI_STATE_DUMP_DONE}
unsafe fn uv_nmi_trigger_dump(cpu:c_int){if uv_cpu_nmi.state==UV_NMI_STATE_IN{uv_cpu_nmi.state=UV_NMI_STATE_DUMP;uv_nmi_dump_state_cpu(cpu,core::ptr::null_mut())}}
unsafe fn uv_nmi_dump_state(_cpu:c_int,_r:*mut pt_regs,_master:c_int){uv_nmi_dump_state_cpu(_cpu,_r);uv_nmi_sync_exit(_master)}
unsafe fn uv_nmi_action_health(_cpu:c_int,_r:*mut pt_regs,master:c_int){uv_nmi_sync_exit(master)}
unsafe fn uv_nmi_touch_watchdogs(){}
unsafe fn uv_nmi_kdump(_cpu:c_int,_main:c_int,_r:*mut pt_regs){atomic_set(&mut uv_nmi_kexec_failed,1)}
unsafe fn uv_call_kgdb_kdb(_cpu:c_int,_r:*mut pt_regs,master:c_int){uv_nmi_sync_exit(master)}
unsafe fn uv_handle_nmi(_reason:c_uint,_regs:*mut pt_regs)->c_int {let cpu=smp_processor_id();let master=(atomic_read(&uv_nmi_cpu)==cpu) as c_int;if uv_check_nmi(uv_hub_nmi)==0{return 0} if matches!(uv_nmi_action,action_t::nmi_act_kdump){uv_nmi_kdump(cpu,master,_regs)} uv_nmi_wait(master);match uv_nmi_action{action_t::nmi_act_health=>uv_nmi_action_health(cpu,_regs,master),action_t::nmi_act_ips|action_t::nmi_act_dump=>uv_nmi_dump_state(cpu,_regs,master),action_t::nmi_act_kdb|action_t::nmi_act_kgdb=>uv_call_kgdb_kdb(cpu,_regs,master),_=>uv_nmi_sync_exit(master)} uv_cpu_nmi.state=UV_NMI_STATE_OUT;uv_clear_nmi(cpu);if master{atomic_set(&mut uv_nmi_cpu,-1);atomic_set(&mut uv_in_nmi,0)}uv_nmi_touch_watchdogs();1}
unsafe fn uv_handle_nmi_ping(reason:c_uint,r:*mut pt_regs)->c_int{if uv_cpu_nmi.pinging==0{return 0}let x=uv_handle_nmi(reason,r);uv_cpu_nmi.pinging=0;x}
unsafe fn uv_register_nmi_notifier(){}
pub unsafe fn uv_nmi_init(){ }
pub unsafe fn uv_nmi_setup(){uv_nmi_setup_mmrs();uv_nmi_setup_common(true);uv_register_nmi_notifier()}
pub unsafe fn uv_nmi_setup_hubless(){uv_nmi_setup_common(false);uv_reassert_nmi();uv_register_nmi_notifier()}
unsafe fn uv_nmi_setup_common(_hubbed:bool){ /* allocation and per-CPU topology are kernel-provided */ }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
