/* SPDX-License-Identifier: GPL-2.0-only */
/* Translated from kvm_book3s_64.h. External kernel types/functions are supplied elsewhere. */

#[repr(C)]
pub struct kvm_nested_guest {
    pub l1_host: *mut kvm,
    pub l1_lpid: i32,
    pub shadow_lpid: i32,
    pub shadow_pgtable: *mut pgd_t,
    pub l1_gr_to_hr: u64,
    pub process_table: u64,
    pub refcnt: isize,
    pub tlb_lock: mutex,
    pub next: *mut kvm_nested_guest,
    pub need_tlb_flush: cpumask_t,
    pub prev_cpu: [i16; NR_CPUS],
    pub radix: u8,
}

pub const RMAP_NESTED_LPID_MASK: u64 = 0xFFF0000000000000;
pub const RMAP_NESTED_LPID_SHIFT: u32 = 52;
pub const RMAP_NESTED_GPA_MASK: u64 = 0x000FFFFFFFFFF000;
pub const RMAP_NESTED_IS_SINGLE_ENTRY: u64 = 0x0000000000000001;

#[repr(C)]
pub struct rmap_nested { pub list: llist_node, pub rmap: u64 }

/* The C iterator is retained as a Rust macro; node is allowed to be NULL. */
#[macro_export]
macro_rules! for_each_nest_rmap_safe {
    ($pos:ident, $node:ident, $rmapp:ident) => {
        for $pos in unsafe { llist_entry($node, core::mem::size_of_val(&$pos), 0) } {
            if $node.is_null() { break; }
            unsafe { *$rmapp = if (RMAP_NESTED_IS_SINGLE_ENTRY & ($node as u64)) != 0 { $node as u64 } else { (*$pos).rmap }; }
            if unsafe { (*$pos).rmap } == 0 { break; }
        }
    };
}

extern "C" {
    pub fn kvmhv_get_nested(kvm: *mut kvm, l1_lpid: i32, create: bool) -> *mut kvm_nested_guest;
    pub fn kvmhv_put_nested(gp: *mut kvm_nested_guest);
    pub fn kvmhv_nested_next_lpid(kvm: *mut kvm, lpid: i32) -> i32;
}

#[inline] pub const fn h_tlbie_p1_enc(ric: u64, prs: u64, r: u64) -> u64 { ___PPC_RIC(ric) | ___PPC_PRS(prs) | ___PPC_R(r) }
pub const PPC_MIN_HPT_ORDER: u32 = 18;
pub const PPC_MAX_HPT_ORDER: u32 = 46;
pub const HDSISR_CANARY: u64 = 0x7fff;
pub const HPTE_V_HVLOCK: u64 = 0x40;
pub const HPTE_V_ABSENT: u64 = 0x20;
pub const HPTE_GR_MODIFIED: u64 = 1u64 << 62;
pub const HPTE_GR_RESERVED: u64 = HPTE_GR_MODIFIED;

#[inline]
pub unsafe fn try_lock_hpte(hpte: *mut __be64, bits: u64) -> i64 {
    let mut old: u64;
    let mut value = core::ptr::read_volatile(hpte as *const u64);
    old = value & bits;
    if old == 0 { value |= HPTE_V_HVLOCK; core::ptr::write_volatile(hpte as *mut u64, value); }
    old as i64 == 0 as i64
}
#[inline] pub unsafe fn unlock_hpte(hpte: *mut __be64, hpte_v: u64) { core::ptr::write_volatile(hpte as *mut u64, hpte_v & !HPTE_V_HVLOCK); }
#[inline] pub unsafe fn __unlock_hpte(hpte: *mut __be64, hpte_v: u64) { core::ptr::write(hpte as *mut u64, hpte_v & !HPTE_V_HVLOCK); }

#[inline]
pub fn kvmppc_hpte_page_shifts(h: u64, l: u64) -> i32 {
    if h & HPTE_V_LARGE == 0 { return 12; }
    let lphi = (l >> 16) & 0xf;
    match (l >> 12) & 0xf {
        0 => if lphi == 0 { 24 } else { 0 },
        1 => 16,
        3 => if lphi == 0 { 34 } else { 0 },
        7 => (16 << 8) + 12,
        8 => if lphi == 0 { (24 << 8) + 16 } else if lphi == 3 { (24 << 8) + 12 } else { 0 },
        _ => 0,
    }
}
#[inline] pub fn kvmppc_hpte_base_page_shift(h:u64,l:u64)->i32 { kvmppc_hpte_page_shifts(h,l)&0xff }
#[inline] pub fn kvmppc_hpte_actual_page_shift(h:u64,l:u64)->i32 { let mut t=kvmppc_hpte_page_shifts(h,l); if t>=0x100 {t>>=8;} t }
#[inline] pub fn kvmppc_actual_pgsz(v:u64,r:u64)->u64 { let s=kvmppc_hpte_actual_page_shift(v,r); if s!=0 {1u64<<s} else {0} }

#[inline]
pub fn kvmppc_pgsize_lp_encoding(base_shift:i32, actual_shift:i32)->i32 {
    match (base_shift,actual_shift) {(12,12)=>0,(12,16)=>7,(12,24)=>0x38,(16,16)=>1,(16,24)=>8,(24,_)=>0,_=>-1}
}

#[inline]
pub fn compute_tlbie_rb(mut v:u64,r:u64,pte_index:u64)->u64 {
    let mut a=kvmppc_hpte_page_shifts(v,r); let mut b=a; if a>=0x100 {b&=0xff;a>>=8;}
    let mut rb=(v&!0x7f)<<16; let mut va_low=pte_index>>3;
    if v&HPTE_V_SECONDARY!=0 {va_low=!va_low;}
    if v&HPTE_V_1TB_SEG==0 {va_low^=v>>(SID_SHIFT-16);} else {va_low^=v>>(SID_SHIFT_1T-16);}
    va_low&=0x7ff;
    if b<=12 {if a>12 {rb|=if a==16 {5} else {4}<<5;} rb|=(va_low&0x7ff)<<12;}
    else {rb|=(va_low<<b)&0x7ff000; rb&=!((1u64<<a)-1); let sh=64-(77-b)+1; rb|=(va_low<<sh)&0xfe; rb|=1; rb|=r&0xff000&((1u64<<a)-1);}
    rb|=(v>>HPTE_V_SSIZE_SHIFT)<<8; rb
}

#[inline] pub unsafe fn set_dirty_bits(map:*mut u64,mut i:u64,mut n:u64){if n>=8{core::ptr::write_bytes((map as *mut u8).add((i/8) as usize),0xff,(n/8) as usize)}else{while n!=0{__set_bit_le(i,map);i+=1;n-=1;}}}
#[inline] pub unsafe fn set_dirty_bits_atomic(map:*mut u64,mut i:u64,mut n:u64){if n>=8{core::ptr::write_bytes((map as *mut u8).add((i/8) as usize),0xff,(n/8) as usize)}else{while n!=0{set_bit_le(i,map);i+=1;n-=1;}}}

#[inline] pub unsafe fn kvm_memslots_raw(k:*mut kvm)->*mut kvm_memslots {rcu_dereference_raw_check((*k).memslots[0])}

#[inline] pub fn hpte_rpn(ptel:u64,psize:u64)->u64 { ((ptel & HPTE_R_RPN) & !(psize-1)) >> PAGE_SHIFT }
#[inline] pub fn hpte_is_writable(ptel:u64)->bool { let pp=ptel&(HPTE_R_PP0|HPTE_R_PP); pp!=PP_RXRX && pp!=PP_RXXX }
#[inline] pub fn hpte_make_readonly(mut p:u64)->u64 { if p&HPTE_R_PP0!=0 || p&HPTE_R_PP==PP_RWXX {(p&!HPTE_R_PP)|PP_RXXX} else {p|PP_RXRX} }
#[inline] pub fn hpte_cache_flags_ok(mut h:u64,is_ci:bool)->bool { let mut w=h&HPTE_R_WIMG; if w==HPTE_R_W|HPTE_R_I|HPTE_R_M && cpu_has_feature(CPU_FTR_ARCH_206){w=HPTE_R_M;} if !is_ci {w==HPTE_R_M} else {w&HPTE_R_W==0 && w&HPTE_R_I!=0} }

#[inline] pub fn hpte_read_permission(pp:u64,key:u64)->bool { if key {PP_RWRX<=pp && pp<=PP_RXRX} else {true} }
#[inline] pub fn hpte_write_permission(pp:u64,key:u64)->bool { if key {pp==PP_RWRW} else {pp<=PP_RWRW} }
#[inline] pub fn hpte_get_skey_perm(h:u64,amr:u64)->u64 { let s=((h&HPTE_R_KEY_HI)>>57)|((h&HPTE_R_KEY_LO)>>9); (amr>>(62-2*s))&3 }

#[inline] pub unsafe fn lock_rmap(r:*mut u64){ loop { while test_bit(KVMPPC_RMAP_LOCK_BIT,r)!=0 {cpu_relax();} if test_and_set_bit_lock(KVMPPC_RMAP_LOCK_BIT,r)==0 {break;} } }
#[inline] pub unsafe fn unlock_rmap(r:*mut u64){ __clear_bit_unlock(KVMPPC_RMAP_LOCK_BIT,r); }
#[inline] pub unsafe fn slot_is_aligned(m:*const kvm_memory_slot,p:u64)->bool { let mask=(p>>PAGE_SHIFT)-1; p<=PAGE_SIZE || ((*m).base_gfn&mask==0 && (*m).npages&mask==0) }
#[inline] pub fn slb_pgsize_encoding(p:u64)->u64 { let mut s=0; if p>0x1000 {s=SLB_VSID_L;if p==0x10000{s|=SLB_VSID_LP_01;}} s }
#[inline] pub fn is_vrma_hpte(v:u64)->bool {(v&!0xffffff)==(HPTE_V_1TB_SEG|(VRMA_VSID<<(40-16)))}

extern "C" {
    pub fn kvmppc_msr_hard_disable_set_facilities(vcpu:*mut kvm_vcpu,msr: u64)->u64;
    pub fn kvmhv_vcpu_entry_p9(vcpu:*mut kvm_vcpu,time_limit:u64,lpcr:u64,tb:*mut u64)->i32;
    pub fn kvmppc_mmu_debugfs_init(kvm:*mut kvm); pub fn kvmhv_radix_debugfs_init(kvm:*mut kvm); pub fn kvmhv_rm_send_ipi(cpu:i32);
    pub fn kvmhv_nestedv2_vcpu_create(vcpu:*mut kvm_vcpu,io:*mut kvmhv_nestedv2_io)->i32;
    pub fn kvmhv_nestedv2_vcpu_free(vcpu:*mut kvm_vcpu,io:*mut kvmhv_nestedv2_io);
    pub fn kvmhv_nestedv2_flush_vcpu(vcpu:*mut kvm_vcpu,time_limit:u64)->i32;
    pub fn kvmhv_nestedv2_set_ptbl_entry(lpid:u64,dw0:u64,dw1:u64)->i32;
    pub fn kvmhv_nestedv2_parse_output(vcpu:*mut kvm_vcpu)->i32;
    pub fn kvmhv_nestedv2_set_vpa(vcpu:*mut kvm_vcpu,vpa:u64)->i32;
    pub fn find_kvm_nested_guest_pte(kvm:*mut kvm,lpid:u64,ea:u64,hshift:*mut u32)->*mut pte_t;
}

#[inline] pub unsafe fn kvmppc_hpt_npte(h:*const kvm_hpt_info)->u64 {1u64<<((*h).order-4)}
#[inline] pub unsafe fn kvmppc_hpt_mask(h:*const kvm_hpt_info)->u64 {(1u64<<((*h).order-7))-1}
#[inline] pub fn sanitize_msr(mut m:u64)->u64 {m&=!MSR_HV;m|=MSR_ME;m}

extern "C" {
    pub fn kvmppc_create_pte(kvm:*mut kvm,pgtable:*mut pgd_t,pte:pte_t,gpa:u64,level:u32,mmu_seq:u64,lpid:u64,rmapp:*mut u64,n_rmap:*mut *mut rmap_nested)->i32;
    pub fn kvmhv_insert_nest_rmap(kvm:*mut kvm,rmapp:*mut u64,n_rmap:*mut *mut rmap_nested);
    pub fn kvmhv_update_nest_rmap_rc_list(kvm:*mut kvm,rmapp:*mut u64,clr:u64,set:u64,hpa:u64,nbytes:u64);
    pub fn kvmhv_remove_nest_rmap_range(kvm:*mut kvm,memslot:*const kvm_memory_slot,gpa:u64,hpa:u64,nbytes:u64);
    pub fn kvmhv_counters_tracepoint_regfunc()->i32; pub fn kvmhv_counters_tracepoint_unregfunc(); pub fn kvmhv_get_l2_counters_status()->i32;
    pub fn kvmhv_set_l2_counters_status(cpu:i32,status:bool); pub fn kvmhv_get_l1_to_l2_cs_time()->u64; pub fn kvmhv_get_l2_to_l1_cs_time()->u64; pub fn kvmhv_get_l2_runtime_agg()->u64;
    pub fn kvmhv_get_l1_to_l2_cs_time_vcpu()->u64; pub fn kvmhv_get_l2_to_l1_cs_time_vcpu()->u64; pub fn kvmhv_get_l2_runtime_agg_vcpu()->u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
