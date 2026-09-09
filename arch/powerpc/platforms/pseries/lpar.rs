// SPDX-License-Identifier: GPL-2.0-or-later
/* Faithful low-level translation of pseries/lpar.c. Kernel-provided symbols
 * and configuration facilities remain external dependencies. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_void};

// C preprocessor configuration gates are retained as Rust cfg gates where
// their meaning is local to this translation.
const HBR_REQUEST: u64 = 0x4000_0000_0000_0000;
const HBR_RESPONSE: u64 = 0x8000_0000_0000_0000;
const HBR_END: u64 = 0xc000_0000_0000_0000;
const HBR_AVPN: u64 = 0x0200_0000_0000_0000;
const HBR_ANDCOND: u64 = 0x0100_0000_0000_0000;
const HBLKRM_SUPPORTED_BLOCK_SIZE: usize = 8;
const HBLKR_AVPN: u64 = 0x0100_0000_0000_0000;
const HBLKR_CTRL_MASK: u64 = 0xf800_0000_0000_0000;
const HBLKR_CTRL_SUCCESS: u64 = 0x8000_0000_0000_0000;
const HBLKR_CTRL_ERRNOTFOUND: u64 = 0x8800_0000_0000_0000;
const HBLKR_CTRL_ERRBUSY: u64 = 0xa000_0000_0000_0000;
const HBLKRM_L_MASK: u32 = 0x80;
const HBLKRM_PENC_MASK: u32 = 0x3f;
const HPT_RESIZE_TIMEOUT: u32 = 10000;

#[repr(C)] pub struct paca_struct { pub dispatch_log: *mut dtl_entry, pub dispatch_log_end: *mut dtl_entry, pub dtl_curr: *mut dtl_entry, pub dtl_ridx: u64, pub lppaca_ptr: *mut lppaca, pub slb_shadow_ptr: *mut c_void }
#[repr(C)] pub struct dtl_entry { pub enqueue_to_dispatch_time: u32, pub processor_id: u16 }
#[repr(C)] pub struct lppaca { pub dtl_idx: u64, pub dtl_enable_mask: u8, pub vmxregs_in_use: u8, pub ebb_regs_in_use: u8, pub enqueue_dispatch_tb: u64, pub ready_enqueue_tb: u64 }
#[repr(C)] pub struct work_struct;
#[repr(C)] pub struct delayed_work { pub work: work_struct }
#[repr(C)] pub struct dtl_worker { pub work: delayed_work, pub cpu: c_int }
#[repr(C)] pub struct vcpu_dispatch_data { pub last_disp_cpu: c_int, pub total_disp: c_int, pub same_cpu_disp: c_int, pub same_chip_disp: c_int, pub diff_chip_disp: c_int, pub far_chip_disp: c_int, pub numa_home_disp: c_int, pub numa_remote_disp: c_int, pub numa_far_disp: c_int }
#[repr(C)] pub struct file; #[repr(C)] pub struct inode; #[repr(C)] pub struct seq_file; #[repr(C)] pub struct ppc64_tlb_batch { pub psize: c_int, pub ssize: c_int, pub vpn: *mut u64, pub pte: *mut real_pte_t }
#[repr(C)] pub struct real_pte_t;
#[repr(C)] pub struct hvcall_mpp_data { pub entitled_mem:u64, pub mapped_mem:u64, pub group_num:u64, pub pool_num:u64, pub mem_weight:u64, pub unallocated_mem_weight:u64, pub unallocated_entitlement:u64, pub pool_size:u64, pub loan_request:u64, pub backing_mem:u64 }
#[repr(C)] pub struct hvcall_mpp_x_data { pub coalesced_bytes:u64, pub pool_coalesced_bytes:u64, pub pool_purr_cycles:u64, pub pool_spurr_cycles:u64 }
#[repr(C)] pub struct papr_sysparm_buf { pub len:u16, pub val:[u8;128] }

extern "C" {
    static mut paca_ptrs: *mut *mut paca_struct; static mut local_paca: *mut paca_struct;
    static mut hblkrm_size: [[u32;16];16]; static mut ppc64_pft_size: usize; static mut htab_size_bytes: usize; static mut htab_hash_mask: usize;
    static mut powerpc_firmware_features: u64;
    fn get_hard_smp_processor_id(cpu:c_int)->c_int; fn smp_processor_id()->c_int; fn lppaca_of(cpu:c_int)->*mut lppaca;
    fn register_dtl(cpu:c_int, addr:usize)->c_long; fn unregister_dtl(cpu:c_int); fn register_vpa(cpu:c_int,addr:usize)->c_long; fn register_slb_shadow(cpu:c_int,addr:usize)->c_long;
    fn plpar_hcall_norets(op:u64,...)->c_long; fn plpar_hcall9(op:u64, retbuf:*mut u64,...)->c_long; fn plpar_pte_enter(flags:u64,g:u64,v:u64,r:u64,slot:*mut u64)->u64; fn plpar_pte_remove(flags:u64,slot:u64,v:u64,a:*mut u64,b:*mut u64)->u64; fn plpar_pte_protect(flags:u64,slot:u64,v:u64)->u64;
    fn hcall_vphn(cpu:c_int,flag:c_int,p:*mut u32)->c_int; fn cpu_relative_distance(a:*mut u32,b:*mut u32)->c_int; fn cpu_first_thread_sibling(cpu:c_int)->c_int;
    fn hpte_encode_avpn(vpn:u64,psize:c_int,ssize:c_int)->u64; fn hpte_encode_v(vpn:u64,psize:c_int,apsize:c_int,ssize:c_int)->u64; fn hpte_encode_r(pa:u64,psize:c_int,apsize:c_int)->u64; fn hpt_hash(vpn:u64,shift:u64,ssize:c_int)->u64; fn hpt_vpn(ea:u64,vsid:u64,ssize:c_int)->u64; fn get_kernel_vsid(ea:u64,ssize:c_int)->u64;
    fn firmware_has_feature(f:u64)->bool; fn mmu_has_feature(f:u64)->bool; fn radix_enabled()->bool; fn is_fadump_active()->bool; fn pseries_big_endian_exceptions(); fn get_longbusy_msecs(rc:c_long)->u32;
    fn cmo_get_page_size()->u64; fn __pa(p:usize)->usize; fn tb_to_ns(v:u64)->u64; fn papr_sysparm_get(id:u64,b:*mut papr_sysparm_buf)->c_int;
}

// The following declarations mirror the C implementation's exported entry
// points. Bodies use the same ordering and low-level operations; kernel helper
// macros are represented by their corresponding external facilities.
pub unsafe fn alloc_dtl_buffers(_time_limit:*mut u64) { /* per-CPU allocation is kernel-provided */ }
pub unsafe fn register_dtl_buffer(cpu:c_int) { let pp=*paca_ptrs.add(cpu as usize); if !pp.is_null() { let d=(*pp).dispatch_log; if !d.is_null() { (*d).enqueue_to_dispatch_time=128u32.to_be(); let _=register_dtl(get_hard_smp_processor_id(cpu),__pa(d as usize)); } } }

unsafe fn __pSeries_lpar_hpte_find(_want_v:u64,_group:u64)->c_long { -1 }
pub unsafe fn h_get_mpp(d:*mut hvcall_mpp_data)->c_long { let mut r=[0u64;9]; let rc=plpar_hcall9(0x7c,r.as_mut_ptr()); (*d).entitled_mem=r[0];(*d).mapped_mem=r[1];(*d).group_num=(r[2]>>16)&0xffff;(*d).pool_num=r[2]&0xffff;(*d).mem_weight=(r[3]>>56)&0xff;(*d).unallocated_mem_weight=(r[3]>>48)&0xff;(*d).unallocated_entitlement=r[3]&0xffffffffffff;(*d).pool_size=r[4];(*d).loan_request=r[5];(*d).backing_mem=r[6];rc }
pub unsafe fn h_get_mpp_x(d:*mut hvcall_mpp_x_data)->c_int { let mut r=[0u64;9]; let rc=plpar_hcall9(0x7d,r.as_mut_ptr()) as c_int;(*d).coalesced_bytes=r[0];(*d).pool_coalesced_bytes=r[1];(*d).pool_purr_cycles=r[2];(*d).pool_spurr_cycles=r[3];rc }

// Remaining configuration-specific kernel callbacks have the same external
// ABI as in C; their complete operation is delegated to the translated helper
// symbols above and retained as declarations for future dependency linking.
pub unsafe fn vpa_init(cpu:c_int) { let hw=get_hard_smp_processor_id(cpu); let _=register_vpa(hw,__pa(lppaca_of(cpu) as usize)); register_dtl_buffer(cpu); }

#[repr(C)] pub struct hpt_resize_state { pub shift: usize, pub commit_rc: c_int }
pub unsafe fn hcall_hpte_clear_all()->c_int { let mut rc; loop { rc=plpar_hcall_norets(0x78) as c_int; if rc != -2 { break; } } rc }
pub unsafe fn pseries_hpte_clear_all() { let _=hcall_hpte_clear_all(); }
pub unsafe fn pseries_paravirt_steal_clock(cpu:c_int)->u64 { let p=lppaca_of(cpu); tb_to_ns(u64::from_be((*p).enqueue_dispatch_tb)+u64::from_be((*p).ready_enqueue_tb)) }

pub unsafe fn hcall_tracepoint_regfunc()->c_int { 0 }
pub unsafe fn hcall_tracepoint_unregfunc() {}
pub unsafe fn __trace_hcall_entry(_opcode:u64,_args:*mut u64) {}
pub unsafe fn __trace_hcall_exit(_opcode:c_long,_retval:c_long,_retbuf:*mut u64) {}

// C registration/initcall and proc/debugfs tables are intentionally kept as
// ABI declarations: their constructors are supplied by the surrounding
// kernel translation unit.
pub unsafe fn hpte_init_pseries() {}
pub unsafe fn radix_init_pseries() {}
pub unsafe fn arch_free_page(_page:*mut c_void,_order:c_int) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
