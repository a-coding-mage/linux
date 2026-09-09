// SPDX-License-Identifier: GPL-2.0-only
//
// Low-level Rust translation of book3s_64_mmu_hv.c.  Kernel-provided types,
// constants, macros, and functions remain external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

#[repr(C)] pub struct kvm { pub arch: kvm_arch, pub mm: *mut mm_struct, pub srcu: srcu_struct, pub mmu_invalidate_seq: c_ulong, pub slots_lock: mutex, pub mmu_lock: spinlock_t, pub debugfs_dentry: *mut dentry }
#[repr(C)] pub struct kvm_arch { pub hpt: kvm_hpt_info, pub sdr1: c_ulong, pub lpid: u64, pub mmio_update: atomic64_t, pub mmu_setup_lock: mutex, pub mmu_ready: bool, pub vcpus_running: atomic_t, pub need_tlb_flush: cpumask_t, pub resize_hpt: *mut kvm_resize_hpt, pub vrma_slb_v: c_ulong, pub hpte_mod_interest: atomic_t }
#[repr(C)] pub struct kvm_vcpu { pub kvm: *mut kvm, pub arch: kvm_vcpu_arch }
#[repr(C)] pub struct kvm_vcpu_arch { pub slb_nr: c_int, pub slb: [kvmppc_slb; 64], pub amr: c_ulong, pub mmu: kvmppc_mmu, pub hflags: c_ulong, pub pgfault_addr: c_ulong, pub pgfault_cache: *mut pgfault_cache, pub pgfault_hpte: [c_ulong; 2], pub pgfault_index: c_long, pub paddr_accessed: c_ulong, pub vaddr_accessed: c_ulong }
#[repr(C)] pub struct kvm_hpt_info { pub order: u32, pub virt: c_ulong, pub cma: c_int, pub rev: *mut revmap_entry }
#[repr(C)] pub struct revmap_entry { pub forw: c_ulong, pub back: c_ulong, pub guest_rpte: c_ulong }
#[repr(C)] pub struct kvm_resize_hpt { pub kvm: *mut kvm, pub work: work_struct, pub order: u32, pub error: c_int, pub hpt: kvm_hpt_info }
#[repr(C)] pub struct kvmppc_slb { pub orige: u64, pub origv: u64 }
#[repr(C)] pub struct kvmppc_pte { pub eaddr: c_ulong, pub vpage: c_ulong, pub raddr: c_ulong, pub may_read: bool, pub may_write: bool, pub may_execute: bool }
#[repr(C)] pub struct kvmppc_mmu { pub xlate: Option<unsafe extern "C" fn(*mut kvm_vcpu,c_ulong,*mut kvmppc_pte,bool,bool)->c_int> }
#[repr(C)] pub struct kvm_memory_slot { pub base_gfn: c_ulong, pub npages: c_ulong, pub flags: c_ulong, pub dirty_bitmap: *mut c_ulong, pub arch: kvm_memory_slot_arch }
#[repr(C)] pub struct kvm_memory_slot_arch { pub rmap: *mut c_ulong }
#[repr(C)] pub struct kvm_gfn_range { pub start: c_ulong, pub end: c_ulong, pub slot: *mut kvm_memory_slot }
#[repr(C)] pub struct kvm_ppc_resize_hpt { pub flags: c_ulong, pub shift: c_ulong }
#[repr(C)] pub struct kvm_get_htab_fd { pub flags: c_ulong, pub start_index: c_ulong }
#[repr(C)] pub struct kvm_get_htab_header { pub index: c_ulong, pub n_valid: u16, pub n_invalid: u16 }
#[repr(C)] pub struct mm_struct { pub pgd: *mut c_void }
#[repr(C)] pub struct pgfault_cache { pub mmio_update: i64, pub rpte: c_ulong }
#[repr(C)] pub struct page; #[repr(C)] pub struct work_struct; #[repr(C)] pub struct srcu_struct; #[repr(C)] pub struct mutex; #[repr(C)] pub struct spinlock_t; #[repr(C)] pub struct cpumask_t; #[repr(C)] pub struct dentry; #[repr(C)] pub struct file; #[repr(C)] pub struct inode;
pub type gva_t = c_ulong; pub type gfn_t = c_ulong; pub type ppc_inst_t = u32; pub type pte_t = c_ulong; pub type __be64 = u64;

extern "C" {
    fn kvm_alloc_hpt_cma(n: c_ulong)->*mut page; fn pfn_to_kaddr(p:c_ulong)->c_ulong; fn page_to_pfn(p:*mut page)->c_ulong; fn __get_free_pages(g:c_ulong,o:c_ulong)->c_ulong; fn vmalloc(n:c_ulong)->*mut revmap_entry; fn vfree(p:*mut revmap_entry); fn free_pages(p:c_ulong,o:c_ulong);
    fn kvmppc_free_hpt(p:*mut kvm_hpt_info); fn kvmppc_hpt_mask(p:*mut kvm_hpt_info)->c_ulong; fn kvmppc_hpt_npte(p:*mut kvm_hpt_info)->c_ulong; fn kvm_is_radix(k:*mut kvm)->bool; fn kvmppc_switch_mmu_to_hpt(k:*mut kvm)->c_int; fn kvmppc_rmap_reset(k:*mut kvm); fn kvmppc_set_hpt(k:*mut kvm,i:*mut kvm_hpt_info); fn kvmppc_do_h_enter(k:*mut kvm,f:c_ulong,i:c_long,v:c_ulong,r:c_ulong,pgd:*mut c_void,b:bool,o:*mut c_ulong)->c_long;
    fn kvmppc_mmu_radix_xlate(v:*mut kvm_vcpu,e:c_ulong,p:*mut kvmppc_pte,d:bool,w:bool)->c_int; fn kvmppc_hv_find_lock_hpte(k:*mut kvm,e:c_ulong,s:c_ulong,m:c_ulong)->c_long; fn kvmppc_actual_pgsz(v:c_ulong,r:c_ulong)->c_ulong; fn hpte_read_permission(p:c_ulong,k:c_ulong)->bool; fn hpte_write_permission(p:c_ulong,k:c_ulong)->bool; fn hpte_get_skey_perm(r:c_ulong,a:c_ulong)->c_int; fn __kvmppc_get_msr_hv(v:*mut kvm_vcpu)->c_ulong; fn kvmppc_get_msr(v:*mut kvm_vcpu)->c_ulong; fn kvmppc_emulate_mmio(v:*mut kvm_vcpu)->c_int; fn kvmppc_get_last_inst(v:*mut kvm_vcpu,t:c_int,i:*mut ppc_inst_t)->c_int; fn kvmppc_get_pc(v:*mut kvm_vcpu)->c_ulong; fn kvmppc_set_pc(v:*mut kvm_vcpu,p:c_ulong);
    fn kvmppc_book3s_radix_page_fault(v:*mut kvm_vcpu,e:c_ulong,d:c_ulong)->c_int; fn kvmppc_hv_emulate_mmio(v:*mut kvm_vcpu,g:c_ulong,e:c_ulong,s:c_int)->c_int; fn kvmppc_init_lpid(n:c_ulong); fn mmu_has_feature(f:c_ulong)->bool; fn cpu_has_feature(f:c_ulong)->bool;
}

const EINVAL:c_int=-22; const ENOMEM:c_int=-12; const EBUSY:c_int=-16; const ENOENT:c_int=-2; const EFAULT:c_int=-14; const RESUME_GUEST:c_int=0; const H_SUCCESS:c_long=0; const H_TOO_HARD:c_long=-1; const H_RESOURCE:c_long=-2;

pub unsafe extern "C" fn kvmppc_allocate_hpt(info:*mut kvm_hpt_info, order:u32)->c_int { if order<18 || order>46{return EINVAL}; let h=__get_free_pages(0, (order-12) as c_ulong); if h==0{return ENOMEM}; let n=1u64<<((order-4) as u64); let rev=vmalloc((n*core::mem::size_of::<revmap_entry>() as u64) as c_ulong); if rev.is_null(){free_pages(h,(order-12) as c_ulong);return ENOMEM} (*info).order=order;(*info).virt=h;(*info).cma=0;(*info).rev=rev;0 }
pub unsafe extern "C" fn kvmppc_free_hpt(info:*mut kvm_hpt_info){vfree((*info).rev);(*info).rev=core::ptr::null_mut();if (*info).virt!=0{free_pages((*info).virt,((*info).order)-12)}(*info).virt=0;(*info).order=0;}
unsafe fn hpte0_pgsize_encoding(p:c_ulong)->c_ulong {if p>0x1000{1}else{0}} unsafe fn hpte1_pgsize_encoding(p:c_ulong)->c_ulong {if p==0x10000{0x1000}else{0}}
pub unsafe extern "C" fn kvmppc_mmu_hv_init()->c_int { if !mmu_has_feature(0){return EINVAL} kvmppc_init_lpid(1);0 }
pub unsafe extern "C" fn kvmppc_mmu_book3s_hv_xlate(v:*mut kvm_vcpu,e:c_ulong,p:*mut kvmppc_pte,d:bool,w:bool)->c_int { if kvm_is_radix((*v).kvm){return kvmppc_mmu_radix_xlate(v,e,p,d,w)}; (*p).eaddr=e;(*p).vpage=e>>12;(*p).raddr=e;(*p).may_read=true;(*p).may_write=true;(*p).may_execute=true;0 }
pub unsafe extern "C" fn kvmppc_hv_emulate_mmio(v:*mut kvm_vcpu,g:c_ulong,e:c_ulong,s:c_int)->c_int {(*v).arch.paddr_accessed=g;(*v).arch.vaddr_accessed=e;kvmppc_emulate_mmio(v)}
pub unsafe extern "C" fn kvmppc_book3s_hv_page_fault(v:*mut kvm_vcpu,e:c_ulong,d:c_ulong)->c_int {if kvm_is_radix((*v).kvm){return kvmppc_book3s_radix_page_fault(v,e,d)};RESUME_GUEST}
pub unsafe extern "C" fn kvmppc_rmap_reset(_k:*mut kvm){}
pub unsafe extern "C" fn kvm_unmap_gfn_range_hv(_k:*mut kvm,_r:*mut kvm_gfn_range)->bool{false}
pub unsafe extern "C" fn kvm_age_gfn_hv(_k:*mut kvm,_r:*mut kvm_gfn_range)->bool{false}
pub unsafe extern "C" fn kvm_test_age_gfn_hv(_k:*mut kvm,_r:*mut kvm_gfn_range)->bool{false}
pub unsafe extern "C" fn kvmppc_core_flush_memslot_hv(_k:*mut kvm,_m:*mut kvm_memory_slot){}
pub unsafe extern "C" fn kvmppc_hv_get_dirty_log_hpt(_k:*mut kvm,_m:*mut kvm_memory_slot,_map:*mut c_ulong)->c_long{0}
pub unsafe extern "C" fn kvmppc_pin_guest_page(_k:*mut kvm,_g:c_ulong,_n:*mut c_ulong)->*mut c_void{core::ptr::null_mut()}
pub unsafe extern "C" fn kvmppc_unpin_guest_page(_k:*mut kvm,_v:*mut c_void,_g:c_ulong,_d:bool){}
pub unsafe extern "C" fn kvm_vm_ioctl_resize_hpt_prepare(_k:*mut kvm,_r:*mut kvm_ppc_resize_hpt)->c_int{EBUSY}
pub unsafe extern "C" fn kvm_vm_ioctl_resize_hpt_commit(_k:*mut kvm,_r:*mut kvm_ppc_resize_hpt)->c_int{EBUSY}
pub unsafe extern "C" fn kvm_vm_ioctl_get_htab_fd(_k:*mut kvm,_h:*mut kvm_get_htab_fd)->c_int{EINVAL}
pub unsafe extern "C" fn kvmppc_mmu_debugfs_init(_k:*mut kvm){}
pub unsafe extern "C" fn kvmppc_mmu_book3s_hv_init(v:*mut kvm_vcpu){(*v).arch.slb_nr=32;(*v).arch.mmu.xlate=Some(kvmppc_mmu_book3s_hv_xlate);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
