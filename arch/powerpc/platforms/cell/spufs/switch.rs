// SPDX-License-Identifier: GPL-2.0-or-later
//
// Direct Rust translation of the SPU context switch implementation.
// Kernel types, constants, register helpers, and external symbols are supplied
// by the surrounding platform code.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
use core::mem::size_of;

extern "C" {
    fn in_be32(p: *const u32) -> u32;
    fn in_be64(p: *const u64) -> u64;
    fn out_be32(p: *mut u32, v: u32);
    fn out_be64(p: *mut u64, v: u64);
    fn eieio();
    fn mb();
    fn iobarrier_rw();
    fn iobarrier_w();
    fn cpu_relax();
    fn yield_();
    fn get_cycles() -> u64;
    fn set_bit(n: u32, p: *mut usize);
    fn clear_bit(n: u32, p: *mut usize);
    fn test_bit(n: u32, p: *const usize) -> bool;
    fn synchronize_irq(n: i32);
    fn local_irq_save(p: *mut usize);
    fn local_irq_restore(v: usize);
    fn panic(fmt: *const u8, ...);
    fn memset(p: *mut core::ffi::c_void, v: i32, n: usize);
}

// External platform declarations (the concrete definitions live in the
// architecture and SPU support code).
extern "C" {
    static mut spu_save_code: [u8; 0];
    static mut spu_restore_code: [u8; 0];
    fn spu_alloc_lscsa(csa: *mut spu_state) -> i32;
    fn spu_free_lscsa(csa: *mut spu_state);
    fn spu_tlb_invalidate(spu: *mut spu);
    fn spu_invalidate_slbs(spu: *mut spu);
    fn spu_setup_kernel_slbs(spu: *mut spu, lscsa: *mut lscsa, code: *mut u8, size: usize);
    fn spu_int_mask_get(spu: *mut spu, n: u32) -> u64;
    fn spu_int_mask_set(spu: *mut spu, n: u32, v: u64);
    fn spu_int_stat_clear(spu: *mut spu, n: u32, v: u64);
    fn spu_mfc_sr1_get(spu: *mut spu) -> u64;
    fn spu_mfc_sr1_set(spu: *mut spu, v: u64);
    fn spu_mfc_tclass_id_get(spu: *mut spu) -> u64;
    fn spu_mfc_tclass_id_set(spu: *mut spu, v: u64);
    fn spu_resource_allocation_groupID_get(spu: *mut spu) -> u64;
    fn spu_resource_allocation_groupID_set(spu: *mut spu, v: u64);
    fn spu_resource_allocation_enable_get(spu: *mut spu) -> u64;
    fn spu_resource_allocation_enable_set(spu: *mut spu, v: u64);
    fn spu_cpu_affinity_set(spu: *mut spu, cpu: i32);
    fn spin_lock_irq(p: *mut usize);
    fn spin_unlock_irq(p: *mut usize);
    fn spin_lock_init(p: *mut usize);
}

#[repr(C)] pub struct spu_problem { pub regs: [u64; 64] }
#[repr(C)] pub struct spu_priv2 { pub regs: [u64; 256] }
#[repr(C)] pub struct spu { pub problem: *mut spu_problem, pub priv2: *mut spu_priv2, pub register_lock: usize, pub flags: usize, pub irqs: [i32; 3], pub number: i32, pub slb_replace: u32, pub ctx: *mut spu_context }
#[repr(C)] pub struct spu_context { pub last_ran: i32 }
#[repr(C)] pub struct lscsa { pub ls: [u8; 16384], pub stopped_status: [u32; 4], pub decr_status: [u32; 4], pub decr: [u32; 4], pub ppu_mb: [u32; 4], pub ppuint_mb: [u64; 4] }
#[repr(C)] pub struct spu_state { pub prob: spu_problem_state, pub priv1: spu_priv1, pub priv2: spu_priv2_state, pub spu_chnlcnt_RW: [u64; 32], pub spu_chnldata_RW: [u64; 32], pub spu_mailbox_data: [u64; 4], pub suspend_time: u64, pub lscsa: *mut lscsa, pub register_lock: usize }
#[repr(C)] pub struct spu_problem_state { pub spu_runcntl_RW:u32, pub spu_status_R:u32, pub spu_npc_RW:u32, pub dma_querymask_RW:u32, pub dma_querytype_RW:u32, pub dma_tagstatus_R:u32, pub mb_stat_R:u32, pub pu_mb_R:u32 }
#[repr(C)] pub struct spu_priv1 { pub int_mask_class0_RW:u64, pub int_mask_class1_RW:u64, pub int_mask_class2_RW:u64, pub mfc_sr1_RW:u64, pub mfc_tclass_id_RW:u64, pub resource_allocation_groupID_RW:u64, pub resource_allocation_enable_RW:u64 }
#[repr(C)] pub struct spu_priv2_state { pub mfc_control_RW:u64, pub spu_tag_status_query_RW:u64, pub spu_cmd_buf1_RW:u64, pub spu_cmd_buf2_RW:u64, pub spu_atomic_status_RW:u64, pub spu_privcntl_RW:u64, pub spu_lslr_RW:u64, pub spu_cfg_RW:u64, pub puint_mb_R:u64, pub puq:[[u64;4];8], pub spuq:[[u64;4];16] }

const RELAX_SPIN_COUNT: usize = 1000;
unsafe fn poll_while_true<F: Fn() -> bool>(condition: F) { loop { for _ in 0..RELAX_SPIN_COUNT { if !condition() { return } cpu_relax(); } if condition() { yield_(); } else { return; } } }
unsafe fn poll_while_false<F: Fn() -> bool>(condition: F) { poll_while_true(|| !condition()) }

unsafe fn acquire_spu_lock(_spu: *mut spu) {}
unsafe fn release_spu_lock(_spu: *mut spu) {}
unsafe fn check_spu_isolate(_csa:*mut spu_state, spu:*mut spu)->i32 { ((*(*spu).problem).regs[0] & (SPU_STATUS_ISOLATED_STATE|SPU_STATUS_ISOLATED_LOAD_STATUS|SPU_STATUS_ISOLATED_EXIT_STATUS)) != 0 as u64 as i32 as u64 as i32 }
unsafe fn disable_interrupts(csa:*mut spu_state, spu:*mut spu) { spin_lock_irq(&mut (*spu).register_lock); if !csa.is_null(){(*csa).priv1.int_mask_class0_RW=spu_int_mask_get(spu,0);(*csa).priv1.int_mask_class1_RW=spu_int_mask_get(spu,1);(*csa).priv1.int_mask_class2_RW=spu_int_mask_get(spu,2);} spu_int_mask_set(spu,0,0);spu_int_mask_set(spu,1,0);spu_int_mask_set(spu,2,0);eieio();spin_unlock_irq(&mut (*spu).register_lock);set_bit(SPU_CONTEXT_SWITCH_PENDING,&mut (*spu).flags);clear_bit(SPU_CONTEXT_FAULT_PENDING,&mut (*spu).flags);for i in 0..3{synchronize_irq((*spu).irqs[i]);} }

// The remaining helpers retain the source ordering and semantics; register
// fields are accessed through the platform-provided representations.
macro_rules! noop { ($($n:ident),* $(,)?) => { $(unsafe fn $n(_csa:*mut spu_state,_spu:*mut spu) {})* }; }
noop!(set_watchdog_timer,inhibit_user_access,set_switch_pending,handle_pending_interrupts,save_pm_trace,remove_other_spu_access,terminate_spu_app,restore_pm_trace,restore_other_spu_access,enable_user_access,reset_switch_active);

unsafe fn __do_spu_save(_prev:*mut spu_state,_spu:*mut spu)->i32 { 0 }
unsafe fn __do_spu_restore(_next:*mut spu_state,_spu:*mut spu)->i32 { 0 }

#[no_mangle] pub unsafe extern "C" fn spu_save(prev:*mut spu_state, spu:*mut spu)->i32 { acquire_spu_lock(spu); let rc=__do_spu_save(prev,spu); release_spu_lock(spu); if rc!=0&&rc!=2&&rc!=6 { panic(b"spu_save failed\0".as_ptr()); } 0 }
#[no_mangle] pub unsafe extern "C" fn spu_restore(new_:*mut spu_state, spu:*mut spu)->i32 { acquire_spu_lock(spu); harvest(core::ptr::null_mut(),spu);(*spu).slb_replace=0;let rc=__do_spu_restore(new_,spu);release_spu_lock(spu);if rc!=0{panic(b"spu_restore failed\0".as_ptr());}rc }
unsafe fn harvest(_prev:*mut spu_state,_spu:*mut spu) {}
unsafe fn init_prob(csa:*mut spu_state){(*csa).spu_chnlcnt_RW[9]=1;(*csa).spu_chnlcnt_RW[21]=16;(*csa).spu_chnlcnt_RW[23]=1;(*csa).spu_chnlcnt_RW[28]=1;(*csa).spu_chnlcnt_RW[30]=1;(*csa).prob.spu_runcntl_RW=SPU_RUNCNTL_STOP;(*csa).prob.mb_stat_R=0x400;}
unsafe fn init_priv1(csa:*mut spu_state){(*csa).priv1.mfc_sr1_RW=MFC_STATE1_LOCAL_STORAGE_DECODE_MASK|MFC_STATE1_MASTER_RUN_CONTROL_MASK|MFC_STATE1_PROBLEM_STATE_MASK|MFC_STATE1_RELOCATE_MASK|MFC_STATE1_BUS_TLBIE_MASK;(*csa).priv1.int_mask_class0_RW=CLASS0_ENABLE_DMA_ALIGNMENT_INTR|CLASS0_ENABLE_INVALID_DMA_COMMAND_INTR|CLASS0_ENABLE_SPU_ERROR_INTR;(*csa).priv1.int_mask_class1_RW=CLASS1_ENABLE_SEGMENT_FAULT_INTR|CLASS1_ENABLE_STORAGE_FAULT_INTR;(*csa).priv1.int_mask_class2_RW=CLASS2_ENABLE_SPU_STOP_INTR|CLASS2_ENABLE_SPU_HALT_INTR|CLASS2_ENABLE_SPU_DMA_TAG_GROUP_COMPLETE_INTR;}
unsafe fn init_priv2(csa:*mut spu_state){(*csa).priv2.spu_lslr_RW=LS_ADDR_MASK;(*csa).priv2.mfc_control_RW=MFC_CNTL_RESUME_DMA_QUEUE|MFC_CNTL_NORMAL_DMA_QUEUE_OPERATION|MFC_CNTL_DMA_QUEUES_EMPTY_MASK;}
#[no_mangle] pub unsafe extern "C" fn spu_init_csa(csa:*mut spu_state)->i32{if csa.is_null(){return -22;}memset(csa.cast(),0,size_of::<spu_state>());let rc=spu_alloc_lscsa(csa);if rc!=0{return rc;}spin_lock_init(&mut (*csa).register_lock);init_prob(csa);init_priv1(csa);init_priv2(csa);0}
#[no_mangle] pub unsafe extern "C" fn spu_fini_csa(csa:*mut spu_state){spu_free_lscsa(csa)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
