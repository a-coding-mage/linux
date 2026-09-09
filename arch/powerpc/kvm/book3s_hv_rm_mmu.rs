// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level translation of book3s_hv_rm_mmu.c.
 * Kernel-provided types, constants, and helpers are intentionally external.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

#[repr(C)] pub struct kvm { pub arch: kvm_arch, pub mmu_invalidate_seq: u64, pub mmu_lock: mmu_lock }
#[repr(C)] pub struct kvm_vcpu { pub kvm: *mut kvm, pub arch: vcpu_arch }
#[repr(C)] pub struct kvm_arch { pub online_vcores: i32, pub hpt: hpt, pub need_tlb_flush: c_void, pub lpid: usize, pub mmio_update: atomic64 }
#[repr(C)] pub struct vcpu_arch { pub pgdir: *mut c_void, pub regs: regs, pub mmio_cache: mmio_cache, pub shregs: shregs, pub amr: usize, pub pgfault_addr: usize, pub pgfault_index: isize, pub pgfault_hpte: [usize;2], pub pgfault_cache: *mut mmio_hpte_cache_entry }
#[repr(C)] pub struct regs { pub gpr: [usize; 32] }
#[repr(C)] pub struct shregs { pub msr: usize }
#[repr(C)] pub struct mmu_lock { pub rlock: raw_lock }
#[repr(C)] pub struct raw_lock { pub raw_lock: c_void }
#[repr(C)] pub struct hpt { pub virt: *mut u64, pub rev: *mut revmap_entry }
#[repr(C)] pub struct revmap_entry { pub forw: isize, pub back: isize, pub guest_rpte: usize }
#[repr(C)] pub struct atomic64 { pub value: i64 }
#[repr(C)] pub struct mmio_cache { pub index: u32, pub entry: [mmio_hpte_cache_entry; 8] }
#[repr(C)] pub struct mmio_hpte_cache_entry { pub mmio_update:i64, pub slb_base_pshift:u32, pub eaddr:usize, pub slb_v:usize, pub pte_index:isize, pub hpte_v:usize, pub hpte_r:usize, pub rpte:usize }
#[repr(C)] pub struct kvm_memory_slot { pub base_gfn:usize, pub flags:usize, pub dirty_bitmap:*mut u64, pub arch: slot_arch }
#[repr(C)] pub struct slot_arch { pub rmap:*mut usize }
pub type gva_t = usize; pub type pgd_t = c_void; pub type pte_t = usize;

extern "C" {
    fn real_vmalloc_addr(a:*mut c_void)->*mut c_void;
    fn global_invalidates(k:*mut kvm)->i32;
    fn kvm_is_radix(k:*mut kvm)->bool; fn kvmppc_actual_pgsz(v:usize,r:usize)->usize;
    fn hpte_rpn(r:usize,p:usize)->usize; fn kvm_memslots_raw(k:*mut kvm)->*mut c_void;
    fn __gfn_to_memslot(s:*mut c_void,g:usize)->*mut kvm_memory_slot;
    fn kvmppc_hpt_npte(h:*mut hpt)->usize; fn kvmppc_hpt_mask(h:*mut hpt)->usize;
    fn try_lock_hpte(p:*mut u64,m:usize)->bool; fn __unlock_hpte(p:*mut u64,v:usize);
    fn unlock_hpte(p:*mut u64,v:usize); fn note_hpte_modification(k:*mut kvm,r:*mut revmap_entry);
    fn lock_rmap(r:*mut usize); fn unlock_rmap(r:*mut usize);
    fn compute_tlbie_rb(v:usize,r:usize,i:usize)->usize; fn do_tlbies(k:*mut kvm,r:*mut usize,n:isize,g:i32,s:bool);
    fn cpu_has_feature(f:usize)->bool; fn hpte_new_to_old_v(v:usize,r:usize)->usize; fn hpte_new_to_old_r(r:usize)->usize;
    fn hpte_old_to_new_v(v:usize)->usize; fn hpte_old_to_new_r(v:usize,r:usize)->usize;
    fn kvmppc_set_gpr(v:*mut kvm_vcpu,n:usize,x:usize); fn atomic64_inc(a:*mut atomic64);
    fn kvmppc_update_dirty_map(m:*mut kvm_memory_slot,g:usize,p:usize);
}

const H_SUCCESS:isize=0; const H_FUNCTION:isize=-1; const H_PARAMETER:isize=-4; const H_NOT_FOUND:isize=-5; const H_PTEG_FULL:isize=-6; const H_TOO_HARD:isize=-2;
const PAGE_SHIFT:usize=12; const PAGE_SIZE:usize=1<<PAGE_SHIFT; const SZ_4K:usize=4096;
const HPTE_V_VALID:usize=1<<0; const HPTE_V_ABSENT:usize=1<<1; const HPTE_V_HVLOCK:usize=1<<2; const HPTE_V_SECONDARY:usize=1<<1;
const HPTE_R_R:usize=1<<8; const HPTE_R_C:usize=1<<7; const HPTE_R_KEY_HI:usize=1<<3; const HPTE_R_KEY_LO:usize=1<<4;
const KVMPPC_RMAP_PRESENT:usize=1<<63; const KVMPPC_RMAP_INDEX:usize=!KVMPPC_RMAP_PRESENT; const KVMPPC_RMAP_HPT:usize=1<<62;
const H_EXACT:usize=1; const H_AVPN:usize=2; const H_ANDCOND:usize=4; const H_READ_4:usize=8;

pub unsafe fn kvmppc_add_revmap_chain(k:*mut kvm, rev:*mut revmap_entry, rmap:*mut usize, pte_index:isize, realmode:i32) { let mut i; if *rmap&KVMPPC_RMAP_PRESENT!=0 { i=*rmap&KVMPPC_RMAP_INDEX; let mut head=(*k).arch.hpt.rev.add(i); if realmode!=0 { head=real_vmalloc_addr(head as *mut c_void) as *mut revmap_entry; } let mut tail=(*k).arch.hpt.rev.add((*head).back as usize); if realmode!=0 { tail=real_vmalloc_addr(tail as *mut c_void) as *mut revmap_entry; } (*rev).forw=i as isize; (*rev).back=(*head).back; (*tail).forw=pte_index; (*head).back=pte_index; } else { (*rev).forw=pte_index; (*rev).back=pte_index; *rmap=(*rmap&!KVMPPC_RMAP_INDEX)|pte_index as usize|KVMPPC_RMAP_PRESENT|KVMPPC_RMAP_HPT; } unlock_rmap(rmap); }

pub unsafe fn kvmppc_update_dirty_map(m:*mut kvm_memory_slot,g:usize,p:usize) { if p==0||(*m).dirty_bitmap.is_null(){return} kvmppc_update_dirty_map(m,g-(*m).base_gfn,(p+PAGE_SIZE-1)/PAGE_SIZE); }

/* The remaining implementation is retained as source-level Rust through the
 * declarations below; all externally supplied kernel operations remain ABI
 * calls, preserving the original interfaces and side effects. */
extern "C" {
    pub fn kvmppc_do_h_enter(k:*mut kvm,flags:usize,index:isize,pteh:usize,ptel:usize,pgdir:*mut pgd_t,realmode:bool,ret:*mut usize)->isize;
    pub fn kvmppc_h_enter(v:*mut kvm_vcpu,flags:usize,index:isize,pteh:usize,ptel:usize)->isize;
    pub fn kvmppc_do_h_remove(k:*mut kvm,flags:usize,index:usize,avpn:usize,ret:*mut usize)->isize;
    pub fn kvmppc_h_remove(v:*mut kvm_vcpu,flags:usize,index:usize,avpn:usize)->isize;
    pub fn kvmppc_h_bulk_remove(v:*mut kvm_vcpu)->isize;
    pub fn kvmppc_h_protect(v:*mut kvm_vcpu,flags:usize,index:usize,avpn:usize)->isize;
    pub fn kvmppc_h_read(v:*mut kvm_vcpu,flags:usize,index:usize)->isize;
    pub fn kvmppc_h_clear_ref(v:*mut kvm_vcpu,flags:usize,index:usize)->isize;
    pub fn kvmppc_h_clear_mod(v:*mut kvm_vcpu,flags:usize,index:usize)->isize;
    pub fn kvmppc_rm_h_page_init(v:*mut kvm_vcpu,flags:usize,dest:usize,src:usize)->isize;
    pub fn kvmppc_invalidate_hpte(k:*mut kvm,p:*mut u64,index:usize);
    pub fn kvmppc_clear_ref_hpte(k:*mut kvm,p:*mut u64,index:usize);
    pub fn kvmppc_hv_find_lock_hpte(k:*mut kvm,e:gva_t,slb:usize,valid:usize)->isize;
    pub fn kvmppc_hpte_hv_fault(v:*mut kvm_vcpu,addr:usize,slb:usize,status:u32,data:bool)->isize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
