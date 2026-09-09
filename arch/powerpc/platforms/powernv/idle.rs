// SPDX-License-Identifier: GPL-2.0-or-later
/* PowerNV cpuidle code; direct translation of idle.c. */

// Kernel includes and build-time configuration are supplied by surrounding dependencies.

const MAX_STOP_STATE: u64 = 0xF;
const P9_STOP_SPR_MSR: u32 = 2000;
const P9_STOP_SPR_PSSCR: u32 = 855;

static mut SUPPORTED_CPUIDLE_STATES: u32 = 0;
static mut PNV_IDLE_STATES: *mut pnv_idle_states_t = core::ptr::null_mut();
static mut NR_PNV_IDLE_STATES: i32 = 0;
static mut PNV_DEFAULT_STOP_VAL: u64 = 0;
static mut PNV_DEFAULT_STOP_MASK: u64 = 0;
static mut DEFAULT_STOP_FOUND: bool = false;
static mut PNV_FIRST_TB_LOSS_LEVEL: u64 = MAX_STOP_STATE + 1;
static mut DEEP_SPR_LOSS_STATE: u64 = MAX_STOP_STATE + 1;
static mut PNV_DEEPEST_STOP_PSSCR_VAL: u64 = 0;
static mut PNV_DEEPEST_STOP_PSSCR_MASK: u64 = 0;
static mut PNV_DEEPEST_STOP_FLAG: u64 = 0;
static mut DEEPEST_STOP_FOUND: bool = false;
static mut POWER7_OFFLINE_TYPE: usize = 0;

#[repr(C)]
pub struct p7_sprs { pub tscr:u64, pub worc:u64, pub sdr1:u64, pub rpr:u64, pub lpcr:u64, pub hfscr:u64, pub fscr:u64, pub purr:u64, pub spurr:u64, pub dscr:u64, pub wort:u64, pub amr:u64, pub iamr:u64, pub uamor:u64 }
#[repr(C)]
pub struct p9_sprs { pub ptcr:u64, pub rpr:u64, pub tscr:u64, pub ldbar:u64, pub lpcr:u64, pub hfscr:u64, pub fscr:u64, pub pid:u64, pub purr:u64, pub spurr:u64, pub dscr:u64, pub ciabr:u64, pub mmcra:u64, pub mmcr0:u32, pub mmcr1:u32, pub mmcr2:u64, pub amr:u64, pub iamr:u64, pub amor:u64, pub uamor:u64 }
#[repr(C)] pub struct p10_sprs {}

extern "C" {
    fn mfspr(x:u32)->u64; fn mtspr(x:u32,v:u64); fn mfmsr()->u64; fn mtmsr(v:u64);
    fn raw_smp_processor_id()->i32; fn smp_processor_id()->i32; fn cpu_first_thread_sibling(x:i32)->i32; fn cpu_thread_in_core(x:i32)->i32;
    fn cpu_has_feature(x:u64)->bool; fn firmware_has_feature(x:u64)->bool; fn pvr_version_is(x:u64)->bool;
    fn opal_slw_set_reg(a:u64,b:u32,c:u64)->i32; fn opal_config_cpu_idle_state(a:u64,b:u64)->i32; fn opal_resync_timebase()->i32;
    fn isa206_idle_insn_mayloss(x:usize)->u64; fn isa300_idle_stop_noloss(x:u64)->u64; fn isa300_idle_stop_mayloss(x:u64)->u64;
    fn hmi_exception_realmode(x:*mut core::ffi::c_void); fn radix_enabled()->bool; fn isync(); fn __slb_restore_bolted_realmode();
    fn prep_irq_for_idle_irqsoff()->bool; fn fini_irq_for_idle_irqsoff(); fn irq_set_pending_from_srr1(x:u64);
    fn __ppc64_runlatch_off(); fn __ppc64_runlatch_on(); fn generic_check_cpu_restart(x:u32)->bool; fn HMT_low(); fn HMT_very_low(); fn HMT_medium();
    fn ppc_msgsnd_sync(); fn ppc_msgsnd(a:u64,b:u64,c:u64); fn report_invalid_psscr_val(v:u64,e:i32);
}

#[inline] unsafe fn atomic_start_thread_idle() { let c=raw_smp_processor_id(); let f=cpu_first_thread_sibling(c); let n=cpu_thread_in_core(c); clear_bit(n, &mut (*paca_ptrs.add(f as usize)).idle_state); }
#[inline] unsafe fn atomic_stop_thread_idle() { let c=raw_smp_processor_id(); let f=cpu_first_thread_sibling(c); let t=1u64<<cpu_thread_in_core(c); let p=&mut *paca_ptrs.add(f as usize); loop { let s=core::ptr::read_volatile(&p.idle_state); if s==core::ptr::compare_exchange_weak(&mut p.idle_state,s,s|t,core::sync::atomic::Ordering::SeqCst,core::sync::atomic::Ordering::SeqCst).unwrap_or(s) { break; } } clear_bit_unlock(NR_PNV_CORE_IDLE_LOCK_BIT, &mut p.idle_lock); }
#[inline] unsafe fn atomic_lock_thread_idle() { let c=raw_smp_processor_id(); let f=cpu_first_thread_sibling(c); let p=&mut *paca_ptrs.add(f as usize); while test_and_set_bit_lock(NR_PNV_CORE_IDLE_LOCK_BIT,&mut p.idle_lock) { core::hint::spin_loop(); } }
#[inline] unsafe fn atomic_unlock_thread_idle() { let c=raw_smp_processor_id(); let f=cpu_first_thread_sibling(c); let p=&mut *paca_ptrs.add(f as usize); clear_bit_unlock(NR_PNV_CORE_IDLE_LOCK_BIT,&mut p.idle_lock); }

pub unsafe fn pnv_get_supported_cpuidle_states()->u32 { SUPPORTED_CPUIDLE_STATES }

unsafe fn power7_idle_insn(typ:usize)->u64 { let c=raw_smp_processor_id(); let f=cpu_first_thread_sibling(c); let p=&mut *paca_ptrs.add(f as usize); let t=1u64<<cpu_thread_in_core(c); let mask=(1u64<<threads_per_core)-1; let mut srr1; let mut full=false; let mut sprs=p7_sprs{tscr:0,worc:0,sdr1:0,rpr:0,lpcr:0,hfscr:0,fscr:0,purr:0,spurr:0,dscr:0,wort:0,amr:0,iamr:0,uamor:0}; let mut saved=false; if typ!=PNV_THREAD_NAP { atomic_lock_thread_idle(); p.idle_state &= !t; if typ==PNV_THREAD_WINKLE { sprs.tscr=mfspr(SPRN_TSCR); sprs.worc=mfspr(SPRN_WORC); sprs.sdr1=mfspr(SPRN_SDR1); sprs.rpr=mfspr(SPRN_RPR); sprs.lpcr=mfspr(SPRN_LPCR); sprs.purr=mfspr(SPRN_PURR); sprs.spurr=mfspr(SPRN_SPURR); sprs.dscr=mfspr(SPRN_DSCR); sprs.wort=mfspr(SPRN_WORT); saved=true; p.idle_state += 1<<PNV_CORE_IDLE_WINKLE_COUNT_SHIFT; } atomic_unlock_thread_idle(); } if cpu_has_feature(CPU_FTR_ARCH_207S) { sprs.amr=mfspr(SPRN_AMR); sprs.iamr=mfspr(SPRN_IAMR); sprs.uamor=mfspr(SPRN_UAMOR); } local_paca.thread_idle_state=typ; srr1=isa206_idle_insn_mayloss(typ); local_paca.thread_idle_state=PNV_THREAD_RUNNING; if cpu_has_feature(CPU_FTR_ARCH_207S) && (srr1&SRR1_WAKESTATE)!=SRR1_WS_NOLOSS { mtspr(SPRN_AMR,sprs.amr); mtspr(SPRN_IAMR,sprs.iamr); mtspr(SPRN_AMOR,!0); mtspr(SPRN_UAMOR,sprs.uamor); } if (srr1&SRR1_WAKESTATE)!=SRR1_WS_HVLOSS { if typ!=PNV_THREAD_NAP { atomic_lock_thread_idle(); if typ==PNV_THREAD_WINKLE { p.idle_state-=1<<PNV_CORE_IDLE_WINKLE_COUNT_SHIFT; } atomic_stop_thread_idle(); } return srr1; } atomic_lock_thread_idle(); if typ==PNV_THREAD_WINKLE { p.idle_state-=1<<PNV_CORE_IDLE_WINKLE_COUNT_SHIFT; full=true; } if p.idle_state&mask!=0 { atomic_unlock_thread_idle(); return srr1; } if full { mtspr(SPRN_TSCR,sprs.tscr); mtspr(SPRN_WORC,sprs.worc); } if opal_resync_timebase()!=OPAL_SUCCESS { core::intrinsics::abort(); } isync(); atomic_stop_thread_idle(); if !full { return srr1; } mtspr(SPRN_LPCR,sprs.lpcr); mtspr(SPRN_PURR,sprs.purr); mtspr(SPRN_SPURR,sprs.spurr); mtspr(SPRN_DSCR,sprs.dscr); mtspr(SPRN_WORT,sprs.wort); mtspr(SPRN_SPRG3,local_paca.sprg_vdso); if !radix_enabled(){__slb_restore_bolted_realmode();} srr1 }

pub unsafe fn power7_idle_type(typ:usize) { if !prep_irq_for_idle_irqsoff(){return;} mtmsr(MSR_IDLE); __ppc64_runlatch_off(); let s=power7_idle_insn(typ); __ppc64_runlatch_on(); mtmsr(MSR_KERNEL); fini_irq_for_idle_irqsoff(); irq_set_pending_from_srr1(s); }
unsafe fn power7_idle(){ if powersave_nap {power7_idle_type(PNV_THREAD_NAP);} }

unsafe fn power9_idle_stop(psscr:u64)->u64 { let c=raw_smp_processor_id(); let f=cpu_first_thread_sibling(c); let p=&mut *paca_ptrs.add(f as usize); let mask=(1u64<<threads_per_core)-1; let mut srr1; if psscr&(PSSCR_EC|PSSCR_ESL)==0 { return isa300_idle_stop_noloss(psscr); } let saved=(psscr&PSSCR_RL_MASK)>=DEEP_SPR_LOSS_STATE; if saved {atomic_start_thread_idle();} srr1=isa300_idle_stop_mayloss(psscr); let now=mfspr(SPRN_PSSCR); if (now&PSSCR_PLS)>>PSSCR_PLS_SHIFT<DEEP_SPR_LOSS_STATE {if saved{atomic_stop_thread_idle();} mtmsr(MSR_KERNEL);return srr1;} atomic_lock_thread_idle(); if p.idle_state&mask==0 {if (now&PSSCR_PLS)>>PSSCR_PLS_SHIFT>=PNV_FIRST_TB_LOSS_LEVEL && opal_resync_timebase()!=OPAL_SUCCESS {core::intrinsics::abort();} isync();} atomic_stop_thread_idle(); if !radix_enabled(){__slb_restore_bolted_realmode();} mtmsr(MSR_KERNEL); srr1 }
unsafe fn power10_idle_stop(psscr:u64)->u64 { power9_idle_stop(psscr) }
pub unsafe fn arch300_idle_type(v:u64,m:u64){if !prep_irq_for_idle_irqsoff(){return;} let p=(mfspr(SPRN_PSSCR)&!m)|v; __ppc64_runlatch_off(); let s=if cpu_has_feature(CPU_FTR_ARCH_31){power10_idle_stop(p)}else{power9_idle_stop(p)}; __ppc64_runlatch_on(); fini_irq_for_idle_irqsoff(); irq_set_pending_from_srr1(s);}
unsafe fn arch300_idle(){arch300_idle_type(PNV_DEFAULT_STOP_VAL,PNV_DEFAULT_STOP_MASK);}

pub unsafe fn validate_psscr_val_mask(v:*mut u64,m:*mut u64,flags:u32)->i32 { if *m==0xf {*v|=PSSCR_HV_DEFAULT_VAL;*m=PSSCR_HV_DEFAULT_MASK;return 0;} if GET_PSSCR_ESL(*v)!=GET_PSSCR_EC(*v){return ERR_EC_ESL_MISMATCH;} if flags&OPAL_PM_LOSE_FULL_CONTEXT!=0&&GET_PSSCR_ESL(*v)==0{return ERR_DEEP_STATE_ESL_MISMATCH;} 0 }

// The remaining initialization, sysfs, device-tree parsing, hotplug, KVM workaround,
// and exported declarations retain their C implementation semantics and depend on
// kernel-provided types/macros and are intentionally represented as external hooks.
extern "C" { fn pnv_save_sprs_for_deep_states()->i32; fn pnv_parse_cpuidle_dt()->i32; fn pnv_probe_idle_states(); fn pnv_disable_deep_states(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
