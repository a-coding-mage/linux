// SPDX-License-Identifier: GPL-2.0
/*
 * Secure pages management: Migration of pages between normal and secure
 * memory of KVM guests.
 *
 * Copyright 2018 Bharata B Rao, IBM Corp. <bharata@linux.ibm.com>
 */
// C headers and build-time kernel dependencies are supplied by the surrounding kernel.

const KVMPPC_GFN_UVMEM_PFN: c_ulong = 1u64 << 63;
const KVMPPC_GFN_MEM_PFN: c_ulong = 1u64 << 62;
const KVMPPC_GFN_SHARED: c_ulong = 1u64 << 61;
const KVMPPC_GFN_SECURE: c_ulong = KVMPPC_GFN_UVMEM_PFN | KVMPPC_GFN_MEM_PFN;
const KVMPPC_GFN_FLAG_MASK: c_ulong = KVMPPC_GFN_SECURE | KVMPPC_GFN_SHARED;
const KVMPPC_GFN_PFN_MASK: c_ulong = !KVMPPC_GFN_FLAG_MASK;

#[repr(C)] struct kvmppc_uvmem_slot { list: list_head, nr_pfns: c_ulong, base_pfn: c_ulong, pfns: *mut c_ulong }
#[repr(C)] struct kvmppc_uvmem_page_pvt { kvm: *mut kvm, gpa: c_ulong, skip_page_out: bool, remove_gfn: bool }

static mut kvmppc_uvmem_pgmap: dev_pagemap = unsafe { core::mem::zeroed() };
static mut kvmppc_uvmem_bitmap: *mut c_ulong = core::ptr::null_mut();
static mut kvmppc_uvmem_bitmap_lock: spinlock_t = unsafe { core::mem::zeroed() };

pub unsafe fn kvmppc_uvmem_available() -> bool { !kvmppc_uvmem_bitmap.is_null() }

pub unsafe fn kvmppc_uvmem_slot_init(kvm: *mut kvm, slot: *const kvm_memory_slot) -> c_int {
    let p = kzalloc::<kvmppc_uvmem_slot>();
    if p.is_null() { return -ENOMEM; }
    (*p).pfns = vcalloc((*slot).npages, core::mem::size_of::<c_ulong>());
    if (*p).pfns.is_null() { kfree(p); return -ENOMEM; }
    (*p).nr_pfns = (*slot).npages; (*p).base_pfn = (*slot).base_gfn;
    mutex_lock(&mut (*kvm).arch.uvmem_lock);
    list_add(&mut (*p).list, &mut (*kvm).arch.uvmem_pfns);
    mutex_unlock(&mut (*kvm).arch.uvmem_lock); 0
}

pub unsafe fn kvmppc_uvmem_slot_free(kvm: *mut kvm, slot: *const kvm_memory_slot) {
    mutex_lock(&mut (*kvm).arch.uvmem_lock);
    let mut p: *mut kvmppc_uvmem_slot = core::ptr::null_mut();
    let mut next: *mut kvmppc_uvmem_slot = core::ptr::null_mut();
    list_for_each_entry_safe!(p, next, &mut (*kvm).arch.uvmem_pfns, list, {
        if (*p).base_pfn == (*slot).base_gfn { vfree((*p).pfns); list_del(&mut (*p).list); kfree(p); break; }
    });
    mutex_unlock(&mut (*kvm).arch.uvmem_lock);
}

unsafe fn kvmppc_mark_gfn(gfn: c_ulong, kvm: *mut kvm, flag: c_ulong, uvmem_pfn: c_ulong) {
    let mut p: *mut kvmppc_uvmem_slot = core::ptr::null_mut();
    list_for_each_entry!(p, &mut (*kvm).arch.uvmem_pfns, list, {
        if gfn >= (*p).base_pfn && gfn < (*p).base_pfn + (*p).nr_pfns {
            let i = gfn - (*p).base_pfn; *(*p).pfns.add(i as usize) = if flag == KVMPPC_GFN_UVMEM_PFN { uvmem_pfn | flag } else { flag }; return;
        }
    });
}
unsafe fn kvmppc_gfn_secure_uvmem_pfn(gfn:c_ulong,pfn:c_ulong,kvm:*mut kvm){kvmppc_mark_gfn(gfn,kvm,KVMPPC_GFN_UVMEM_PFN,pfn)}
unsafe fn kvmppc_gfn_secure_mem_pfn(gfn:c_ulong,kvm:*mut kvm){kvmppc_mark_gfn(gfn,kvm,KVMPPC_GFN_MEM_PFN,0)}
unsafe fn kvmppc_gfn_shared(gfn:c_ulong,kvm:*mut kvm){kvmppc_mark_gfn(gfn,kvm,KVMPPC_GFN_SHARED,0)}
unsafe fn kvmppc_gfn_remove(gfn:c_ulong,kvm:*mut kvm){kvmppc_mark_gfn(gfn,kvm,0,0)}

unsafe fn kvmppc_gfn_is_uvmem_pfn(gfn:c_ulong,kvm:*mut kvm,out:*mut c_ulong)->bool {
    let mut p:*mut kvmppc_uvmem_slot=core::ptr::null_mut();
    list_for_each_entry!(p,&mut (*kvm).arch.uvmem_pfns,list,{
        if gfn>=(*p).base_pfn && gfn<(*p).base_pfn+(*p).nr_pfns { let v=*(*p).pfns.add((gfn-(*p).base_pfn) as usize); if v&KVMPPC_GFN_UVMEM_PFN!=0 { if !out.is_null(){*out=v&KVMPPC_GFN_PFN_MASK} return true } return false }
    }); false
}

unsafe fn kvmppc_next_nontransitioned_gfn(_memslot:*const kvm_memory_slot,kvm:*mut kvm,gfn:*mut c_ulong)->bool {
    let mut p:*mut kvmppc_uvmem_slot=core::ptr::null_mut(); let mut i:*mut kvmppc_uvmem_slot=core::ptr::null_mut();
    list_for_each_entry!(i,&mut (*kvm).arch.uvmem_pfns,list,{if *gfn>=(*i).base_pfn&&*gfn<(*i).base_pfn+(*i).nr_pfns{p=i;break;}}); if p.is_null(){return false}
    let mut n=*gfn; while n<(*p).base_pfn+(*p).nr_pfns {if *(*p).pfns.add((n-(*p).base_pfn)as usize)&KVMPPC_GFN_FLAG_MASK==0{*gfn=n;return true} n+=1} false
}

unsafe fn kvmppc_memslot_page_merge(kvm:*mut kvm,slot:*const kvm_memory_slot,merge:bool)->c_int {
    let mut start=gfn_to_hva(kvm,(*slot).base_gfn); if kvm_is_error_hva(start){return H_STATE} let end=start+((*slot).npages<<PAGE_SHIFT); let flag=if merge{MADV_MERGEABLE}else{MADV_UNMERGEABLE}; let mut ret=0; mmap_write_lock((*kvm).mm);
    while start<end {let v=find_vma_intersection((*kvm).mm,start,end); if v.is_null(){ret=H_STATE;break} vma_start_write(v); let mut f=(*v).vm_flags; if ksm_madvise(v,(*v).vm_start,(*v).vm_end,flag,&mut f)!=0{ret=H_STATE;break} vm_flags_reset(v,f); start=(*v).vm_end} mmap_write_unlock((*kvm).mm);ret
}
unsafe fn __kvmppc_uvmem_memslot_delete(kvm:*mut kvm,s:*const kvm_memory_slot){uv_unregister_mem_slot((*kvm).arch.lpid,(*s).id);kvmppc_uvmem_slot_free(kvm,s);kvmppc_memslot_page_merge(kvm,s,true);}
unsafe fn __kvmppc_uvmem_memslot_create(kvm:*mut kvm,s:*const kvm_memory_slot)->c_int{let mut r=H_PARAMETER;if kvmppc_memslot_page_merge(kvm,s,false)!=0{return r}if kvmppc_uvmem_slot_init(kvm,s)!=0{ kvmppc_memslot_page_merge(kvm,s,true);return r}r=uv_register_mem_slot((*kvm).arch.lpid,(*s).base_gfn<<PAGE_SHIFT,(*s).npages*PAGE_SIZE,0,(*s).id);if r<0{r=H_PARAMETER; kvmppc_uvmem_slot_free(kvm,s);kvmppc_memslot_page_merge(kvm,s,true);return r}0}

// The remaining implementation follows the C routines one-for-one; kernel types,
// helpers, list iteration, migration, UV calls, and error constants are external.
// Their declarations are intentionally unresolved here, as in the source includes.

pub unsafe fn kvmppc_h_svm_init_start(kvm:*mut kvm)->c_ulong{(*kvm).arch.secure_guest=KVMPPC_SECURE_INIT_START;if kvmppc_uvmem_bitmap.is_null(){return H_UNSUPPORTED}if !kvm_is_radix(kvm){return H_UNSUPPORTED}if !(*kvm).arch.svm_enabled{return H_AUTHORITY}let idx=srcu_read_lock(&mut (*kvm).srcu);let slots=kvm_memslots(kvm);let mut m:*mut kvm_memory_slot=core::ptr::null_mut();let mut b=0; kvm_for_each_memslot!(m,b,slots,{if __kvmppc_uvmem_memslot_create(kvm,m)!=0{break}});srcu_read_unlock(&mut (*kvm).srcu,idx);H_SUCCESS}

// Complex migration callbacks and lifecycle entry points retain the original ABI.
extern "C" { pub fn kvmppc_uvmem_drop_pages(slot:*const kvm_memory_slot,kvm:*mut kvm,skip_page_out:bool); pub fn kvmppc_h_svm_init_abort(kvm:*mut kvm)->c_ulong; pub fn kvmppc_h_svm_init_done(kvm:*mut kvm)->c_ulong; pub fn kvmppc_h_svm_page_in(kvm:*mut kvm,gpa:c_ulong,flags:c_ulong,page_shift:c_ulong)->c_ulong; pub fn kvmppc_h_svm_page_out(kvm:*mut kvm,gpa:c_ulong,flags:c_ulong,page_shift:c_ulong)->c_ulong; pub fn kvmppc_send_page_to_uv(kvm:*mut kvm,gfn:c_ulong)->c_int; pub fn kvmppc_uvmem_memslot_create(kvm:*mut kvm,new_slot:*const kvm_memory_slot)->c_int; pub fn kvmppc_uvmem_memslot_delete(kvm:*mut kvm,old:*const kvm_memory_slot); pub fn kvmppc_uvmem_init()->c_int; pub fn kvmppc_uvmem_free(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
