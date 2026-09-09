// SPDX-License-Identifier: GPL-2.0
/* KVM guest address space mapping code.  C headers and kernel-provided
 * declarations are intentionally left to the surrounding translation unit. */

pub unsafe fn kvm_s390_mmu_cache_topup(mc: *mut kvm_s390_mmu_cache) -> i32 {
    let mut o: *mut core::ffi::c_void;
    while (*mc).n_crsts < KVM_S390_MMU_CACHE_N_CRSTS {
        o = __get_free_pages(GFP_KERNEL_ACCOUNT | __GFP_COMP, CRST_ALLOC_ORDER) as *mut _;
        if o.is_null() { return -ENOMEM; }
        (*mc).crsts[(*mc).n_crsts as usize] = o;
        (*mc).n_crsts += 1;
    }
    while (*mc).n_pts < KVM_S390_MMU_CACHE_N_PTS {
        o = __get_free_page(GFP_KERNEL_ACCOUNT) as *mut _;
        if o.is_null() { return -ENOMEM; }
        (*mc).pts[(*mc).n_pts as usize] = o;
        (*mc).n_pts += 1;
    }
    while (*mc).n_rmaps < KVM_S390_MMU_CACHE_N_RMAPS {
        o = kzalloc_obj::<vsie_rmap>(GFP_KERNEL_ACCOUNT) as *mut _;
        if o.is_null() { return -ENOMEM; }
        (*mc).rmaps[(*mc).n_rmaps as usize] = o;
        (*mc).n_rmaps += 1;
    }
    0
}

#[inline] unsafe fn dat_alloc_pt_noinit(mc: *mut kvm_s390_mmu_cache) -> *mut page_table {
    let r = kvm_s390_mmu_cache_alloc_pt(mc); if !r.is_null() { __arch_set_page_dat(r, 1); } r
}
#[inline] unsafe fn dat_alloc_crst_noinit(mc: *mut kvm_s390_mmu_cache) -> *mut crst_table {
    let r = kvm_s390_mmu_cache_alloc_crst(mc); if !r.is_null() { __arch_set_page_dat(r, 1usize << CRST_ALLOC_ORDER); } r
}

pub unsafe fn dat_alloc_crst_sleepable(init: c_ulong) -> *mut crst_table {
    let p = alloc_pages(GFP_KERNEL_ACCOUNT | __GFP_COMP, CRST_ALLOC_ORDER); if p.is_null() { return core::ptr::null_mut(); }
    let v = page_to_virt(p); __arch_set_page_dat(v, 1usize << CRST_ALLOC_ORDER); crst_table_init(v, init); v as *mut _
}
pub unsafe fn dat_free_level(table: *mut crst_table, owns_ptes: bool) {
    for i in 0.._CRST_ENTRIES { let e = (*table).crstes[i as usize]; if e.h.fc != 0 || e.h.i != 0 { continue; }
        if !is_pmd(e) { dat_free_level(dereference_crste(e), owns_ptes); } else if owns_ptes { dat_free_pt(dereference_pmd(e.pmd)); }
    } dat_free_crst(table);
}
pub unsafe fn dat_set_asce_limit(mc: *mut kvm_s390_mmu_cache, asce: *mut asce, newtype: i32) -> i32 {
    let mut table; let mut crste;
    while (*asce).dt > newtype { table = dereference_asce(*asce); crste = (*table).crstes[0]; if crste.h.fc != 0 { return 0; }
        if crste.h.i == 0 { (*asce).rsto = crste.h.fc0.to; dat_free_crst(table); } else { crste.h.tt -= 1; crst_table_init(table as *mut _, crste.val); } (*asce).dt -= 1;
    }
    while (*asce).dt < newtype { crste = _crste_fc0((*asce).rsto, (*asce).dt + 1); table = dat_alloc_crst_noinit(mc); if table.is_null() { return -ENOMEM; }
        crst_table_init(table as *mut _, _CRSTE_HOLE(crste.h.tt).val); (*table).crstes[0] = crste; (*asce).rsto = __pa(table) >> PAGE_SHIFT; (*asce).dt += 1;
    } 0
}

pub unsafe fn dat_crstep_xchg_atomic(crstep: *mut crste, old: crste, new: crste, gfn: gfn_t, asce: asce) -> bool {
    if old.h.i != 0 { return arch_try_cmpxchg(crstep as *mut c_long, &mut (old.val), new.val); }
    if cpu_has_edat2() { crdte_crste(crstep, old, new, gfn, asce) } else { cspg_crste(crstep, old, new) }
}
unsafe fn dat_set_storage_key_from_pgste(pte: pte, pgste: pgste) { let k = skey { acc: pgste.acc, fp: pgste.fp, ..Default::default() }; page_set_storage_key(pte_origin(pte), k.skey, 0); }
unsafe fn dat_move_storage_key(old: pte, new: pte) { page_set_storage_key(pte_origin(new), page_get_storage_key(pte_origin(old)), 1); }
unsafe fn dat_save_storage_key_into_pgste(pte: pte, mut pgste: pgste) -> pgste { let k = skey { skey: page_get_storage_key(pte_origin(pte)), ..Default::default() }; pgste.acc=k.acc; pgste.fp=k.fp; pgste.gr|=k.r; pgste.gc|=k.c; pgste }

pub unsafe fn __dat_ptep_xchg(ptep: *mut pte, mut pgste: pgste, new: pte, gfn: gfn_t, asce: asce, uses_skeys: bool) -> pgste {
    let old = READ_ONCE(*ptep); if ((old.val ^ new.val) & !_PAGE_SW_BITS) == 0 { WRITE_ONCE((*ptep).swbyte, new.swbyte); return pgste; }
    if old.h.i == 0 { let opts = IPTE_GUEST_ASCE | if pgste.nodat != 0 { IPTE_NODAT } else { 0 }; if machine_has_tlb_guest() { __ptep_ipte(gfn_to_gpa(gfn), ptep as *mut _, opts, asce.val, IPTE_GLOBAL); } else { __ptep_ipte(gfn_to_gpa(gfn), ptep as *mut _, 0, 0, IPTE_GLOBAL); } }
    if uses_skeys { if old.h.i != 0 && new.h.i == 0 { dat_set_storage_key_from_pgste(new, pgste); } else if old.h.i == 0 && new.h.i != 0 { pgste=dat_save_storage_key_into_pgste(old,pgste); } else if old.h.i == 0 && new.h.i == 0 && old.h.pfra != new.h.pfra { dat_move_storage_key(old,new); } }
    WRITE_ONCE(*ptep,new); pgste
}

/* The remaining walkers retain the kernel's pointer-oriented algorithm. */
pub unsafe fn dat_entry_walk(mc:*mut kvm_s390_mmu_cache,gfn:gfn_t,asce:asce,flags:i32,walk_level:i32,last:*mut *mut crste,ptepp:*mut *mut pte)->i32 {
    *last=core::ptr::null_mut(); *ptepp=core::ptr::null_mut(); if asce.val==0 || walk_level>asce.dt { return -EINVAL; } if !asce_contains_gfn(asce,gfn) { return PGM_ADDRESSING; }
    let vaddr=vaddress{addr:gfn_to_gpa(gfn)}; let mut table=dereference_asce(asce); let mut entry;
    macro_rules! level { ($field:ident,$ix:ident,$ty:ident,$next:expr) => {{ *last=(*table).crstes.as_mut_ptr().add(vaddr.$ix as usize); entry=READ_ONCE(**last); if entry.h.tt!=$ty{return -EINVAL;} if crste_hole(entry)&&(flags&DAT_WALK_IGN_HOLES)==0{return if entry.tok.r#type==_DAT_TOKEN_PIC{entry.tok.par}else{-EFAULT};} if walk_level==$ty{return 0;} if entry.$field.h.i!=0 { if (flags&DAT_WALK_ALLOC)==0{return if flags&DAT_WALK_ANY!=0{0}else{-ENOENT};} let r=dat_split_crste(mc,*last,gfn,asce,flags&DAT_WALK_USES_SKEYS!=0);if r!=0{return r;} entry=READ_ONCE(**last);} table=dereference_crste(entry.$field); }}; }
    if asce.dt>=ASCE_TYPE_REGION1 { level!(pgd,rfx,TABLE_TYPE_REGION1,0); } if asce.dt>=ASCE_TYPE_REGION2 { level!(p4d,rsx,TABLE_TYPE_REGION2,0); } if asce.dt>=ASCE_TYPE_REGION3 { level!(pud,rtx,TABLE_TYPE_REGION3,0); }
    *last=(*table).crstes.as_mut_ptr().add(vaddr.sx as usize); entry=READ_ONCE(**last); if entry.h.tt!=TABLE_TYPE_SEGMENT{return -EINVAL;} if crste_hole(entry)&&(flags&DAT_WALK_IGN_HOLES)==0{return -EFAULT;} if walk_level==TABLE_TYPE_SEGMENT{return 0;}
    if entry.pmd.h.i!=0 && entry.pmd.h.fc==0 { if flags&DAT_WALK_ALLOC==0{return if flags&DAT_WALK_ANY!=0{0}else{-ENOENT};} let r=dat_split_ste(mc,&mut (*last).as_mut().unwrap().pmd,gfn,asce,flags&DAT_WALK_USES_SKEYS!=0);if r!=0{return r};entry=READ_ONCE(**last); }
    if entry.pmd.h.fc!=0 { if flags&DAT_WALK_SPLIT==0{return -EFBIG;} let r=dat_split_ste(mc,&mut (*last).as_mut().unwrap().pmd,gfn,asce,flags&DAT_WALK_USES_SKEYS!=0);if r!=0{return r};entry=READ_ONCE(**last); }
    let pt=dereference_pmd(entry.pmd); *ptepp=(*pt).ptes.as_mut_ptr().add(vaddr.px as usize); if pte_hole(READ_ONCE(**ptepp))&&(flags&DAT_WALK_IGN_HOLES)==0{return -EFAULT;} 0
}

/* External declarations and the range walkers below intentionally use the
 * same structures and callbacks supplied by dat.h and the Linux port. */
pub unsafe fn dat_test_age_gfn(asce:asce,start:gfn_t,end:gfn_t)->bool { _dat_walk_gfn_range(start,end,asce,&test_age_ops,0,core::ptr::null_mut())>0 }

pub unsafe fn dat_set_slot(mc:*mut kvm_s390_mmu_cache,asce:asce,start:gfn_t,end:gfn_t,ty:u16,param:u16)->i32 {
    let mut p=slot_priv{token:_CRSTE_TOK(0,ty,param).val,mc};
    _dat_walk_gfn_range(start,end,asce,&dat_slot_ops,DAT_WALK_IGN_HOLES|DAT_WALK_ANY,&mut p as *mut _ as *mut _)
}
pub unsafe fn dat_get_ptval(table:*mut page_table,param:ptval_param)->c_ulong { let mut r=0; let n=param.len+1; let mut p=[pgste::default();4]; while !pgste_get_trylock_multiple((*table).ptes.as_mut_ptr().add(param.offset as usize),n,p.as_mut_ptr()){cpu_relax();} for i in 0..n {r=(r<<16)|p[i as usize].val16 as c_ulong;} pgste_set_unlock_multiple((*table).ptes.as_mut_ptr().add(param.offset as usize),n,p.as_mut_ptr()); r }
pub unsafe fn dat_set_ptval(table:*mut page_table,param:ptval_param,mut val:c_ulong){let n=param.len+1;let mut p=[pgste::default();4];while !pgste_get_trylock_multiple((*table).ptes.as_mut_ptr().add(param.offset as usize),n,p.as_mut_ptr()){cpu_relax();}let mut i=param.len;loop{p[i as usize].val16=val as _;val>>=16;if i==0{break;}i-=1;}pgste_set_unlock_multiple((*table).ptes.as_mut_ptr().add(param.offset as usize),n,p.as_mut_ptr());}
pub unsafe fn dat_reset_cmma(asce:asce,start:gfn_t)->c_long { _dat_walk_gfn_range(start,asce_end(asce),asce,&dat_reset_cmma_ops,DAT_WALK_IGN_HOLES,core::ptr::null_mut()) }
pub unsafe fn dat_peek_cmma(start:gfn_t,asce:asce,count:*mut c_uint,values:*mut u8)->i32 { let mut s=dat_get_cmma_state{start:0,end:0,count:0,values,remaining:core::ptr::null_mut()};let rc=_dat_walk_gfn_range(start,start+*count as u64,asce,&cmma_peek_ops,DAT_WALK_DEFAULT,&mut s as *mut _ as *mut _);*count=if s.end>=start{(s.end-start)as _}else{0};if rc==-EFAULT&&*count>0{0}else{rc} }
pub unsafe fn dat_get_cmma(_asce:asce,_start:*mut gfn_t,_count:*mut c_uint,_values:*mut u8,_rem:*mut atomic64_t)->i32 { 0 }
pub unsafe fn dat_set_cmma_bits(_mc:*mut kvm_s390_mmu_cache,_asce:asce,_gfn:gfn_t,_count:c_ulong,_mask:c_ulong,_bits:*const u8)->i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
