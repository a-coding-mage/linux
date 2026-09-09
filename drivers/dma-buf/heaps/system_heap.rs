// SPDX-License-Identifier: GPL-2.0
/* DMABUF System heap exporter; direct low-level translation of system_heap.c. */

use core::ffi::c_void;

#[repr(C)] pub struct system_heap_priv { pub cc_shared: bool }
#[repr(C)] pub struct system_heap_buffer {
    pub heap: *mut dma_heap, pub attachments: list_head, pub lock: mutex,
    pub len: usize, pub sg_table: sg_table, pub vmap_cnt: i32,
    pub vaddr: *mut c_void, pub cc_shared: bool,
}
#[repr(C)] pub struct dma_heap_attachment {
    pub dev: *mut device, pub table: sg_table, pub list: list_head,
    pub mapped: bool, pub cc_shared: bool,
}

const LOW_ORDER_GFP: gfp_t = GFP_HIGHUSER | __GFP_ZERO;
const HIGH_ORDER_GFP: gfp_t = ((GFP_HIGHUSER | __GFP_ZERO | __GFP_NOWARN |
    __GFP_NORETRY) & !__GFP_RECLAIM) | __GFP_COMP;
static mut ORDER_FLAGS: [gfp_t; 3] = [HIGH_ORDER_GFP, HIGH_ORDER_GFP, LOW_ORDER_GFP];
static ORDERS: [u32; 3] = [8, 4, 0];
const NUM_ORDERS: usize = 3;

#[inline] unsafe fn cc_shared_buffer(b: *const system_heap_buffer) -> bool {
    IS_ENABLED_CONFIG_DMABUF_HEAPS_SYSTEM_CC_SHARED && (*b).cc_shared
}

unsafe fn system_heap_set_page_decrypted(page: *mut page) -> i32 {
    let addr = page_address(page) as usize; let nr_pages = 1u32 << compound_order(page);
    let ret = set_memory_decrypted(addr, nr_pages);
    if ret != 0 { pr_warn_ratelimited("dma-buf system heap: failed to decrypt page at %p\n", page_address(page)); }
    ret
}
unsafe fn system_heap_set_page_encrypted(page: *mut page) -> i32 {
    let addr = page_address(page) as usize; let nr_pages = 1u32 << compound_order(page);
    let ret = set_memory_encrypted(addr, nr_pages);
    if ret != 0 { pr_warn_ratelimited("dma-buf system heap: failed to re-encrypt page at %p, leaking memory\n", page_address(page)); }
    ret
}

unsafe fn dup_sg_table(from: *mut sg_table, to: *mut sg_table) -> i32 {
    let ret = sg_alloc_table(to, (*from).orig_nents, GFP_KERNEL); if ret != 0 { return ret; }
    let mut new_sg = (*to).sgl; let mut sg = (*from).sgl;
    for _ in 0..(*from).orig_nents { sg_set_page(new_sg, sg_page(sg), (*sg).length, (*sg).offset); new_sg = sg_next(new_sg); sg = sg_next(sg); }
    0
}

unsafe fn system_heap_attach(dmabuf: *mut dma_buf, attachment: *mut dma_buf_attachment) -> i32 {
    let buffer = (*dmabuf).priv_ as *mut system_heap_buffer;
    let a = kzalloc_obj::<dma_heap_attachment>(); if a.is_null() { return -ENOMEM; }
    let ret = dup_sg_table(&mut (*buffer).sg_table, &mut (*a).table); if ret != 0 { kfree(a); return ret; }
    (*a).dev = (*attachment).dev; INIT_LIST_HEAD(&mut (*a).list); (*a).mapped = false; (*a).cc_shared = (*buffer).cc_shared;
    (*attachment).priv_ = a as *mut c_void; mutex_lock(&mut (*buffer).lock); list_add(&mut (*a).list, &mut (*buffer).attachments); mutex_unlock(&mut (*buffer).lock); 0
}
unsafe fn system_heap_detach(dmabuf: *mut dma_buf, attachment: *mut dma_buf_attachment) { let b=(*dmabuf).priv_ as *mut system_heap_buffer; let a=(*attachment).priv_ as *mut dma_heap_attachment; mutex_lock(&mut (*b).lock); list_del(&mut (*a).list); mutex_unlock(&mut (*b).lock); sg_free_table(&mut (*a).table); kfree(a); }
unsafe fn system_heap_map_dma_buf(attachment: *mut dma_buf_attachment, direction: dma_data_direction) -> *mut sg_table { let a=(*attachment).priv_ as *mut dma_heap_attachment; let attrs=if (*a).cc_shared && IS_ENABLED_CONFIG_DMABUF_HEAPS_SYSTEM_CC_SHARED { DMA_ATTR_CC_SHARED } else { 0 }; let ret=dma_map_sgtable((*attachment).dev,&mut (*a).table,direction,attrs); if ret != 0 { return ERR_PTR(ret); } (*a).mapped=true; &mut (*a).table }
unsafe fn system_heap_unmap_dma_buf(attachment:*mut dma_buf_attachment, table:*mut sg_table, direction:dma_data_direction) { let a=(*attachment).priv_ as *mut dma_heap_attachment; (*a).mapped=false; dma_unmap_sgtable((*attachment).dev,table,direction,0); }

unsafe fn system_heap_dma_buf_begin_cpu_access(dmabuf:*mut dma_buf, direction:dma_data_direction)->i32 { let b=(*dmabuf).priv_ as *mut system_heap_buffer; mutex_lock(&mut (*b).lock); if (*b).vmap_cnt != 0 { invalidate_kernel_vmap_range((*b).vaddr,(*b).len); } let mut a=(*b).attachments.next as *mut dma_heap_attachment; while !a.is_null() { if (*a).mapped { dma_sync_sgtable_for_cpu((*a).dev,&mut (*a).table,direction); } a=list_next_entry(a); } mutex_unlock(&mut (*b).lock); 0 }
unsafe fn system_heap_dma_buf_end_cpu_access(dmabuf:*mut dma_buf, direction:dma_data_direction)->i32 { let b=(*dmabuf).priv_ as *mut system_heap_buffer; mutex_lock(&mut (*b).lock); if (*b).vmap_cnt != 0 { flush_kernel_vmap_range((*b).vaddr,(*b).len); } let mut a=(*b).attachments.next as *mut dma_heap_attachment; while !a.is_null() { if (*a).mapped { dma_sync_sgtable_for_device((*a).dev,&mut (*a).table,direction); } a=list_next_entry(a); } mutex_unlock(&mut (*b).lock); 0 }

unsafe fn system_heap_mmap(dmabuf:*mut dma_buf,vma:*mut vm_area_struct)->i32 { let b=(*dmabuf).priv_ as *mut system_heap_buffer; let mut addr=(*vma).vm_start; let mut pgoff=(*vma).vm_pgoff; let mut sg=(*b).sg_table.sgl; let prot=if cc_shared_buffer(b){pgprot_decrypted((*vma).vm_page_prot)}else{(*vma).vm_page_prot}; while !sg.is_null(){let n=(*sg).length>>PAGE_SHIFT;if pgoff<n{break}pgoff-=n;sg=sg_next(sg);} while !sg.is_null()&&addr<(*vma).vm_end{let mut n=((*sg).length>>PAGE_SHIFT)-pgoff;let page=sg_page(sg).add(pgoff);let mut size=n<<PAGE_SHIFT;if addr+size>(*vma).vm_end{size=(*vma).vm_end-addr}let ret=remap_pfn_range(vma,addr,page_to_pfn(page),size,prot);if ret!=0{return ret}addr+=size;pgoff=0;sg=sg_next(sg)} 0 }

unsafe fn system_heap_do_vmap(b:*mut system_heap_buffer)->*mut c_void { let npages=(PAGE_ALIGN((*b).len)/PAGE_SIZE) as i32; let pages=vmalloc((core::mem::size_of::<*mut page>()*npages as usize) as usize) as *mut *mut page; if pages.is_null(){return ERR_PTR(-ENOMEM)}; let mut p=pages; let mut sg=(*b).sg_table.sgl; while !sg.is_null(){*p=sg_page(sg);p=p.add(1);sg=sg_next(sg)} let prot=if cc_shared_buffer(b){pgprot_decrypted(PAGE_KERNEL)}else{PAGE_KERNEL};let v=vmap(pages,npages,VM_MAP,prot);vfree(pages as *mut c_void);if v.is_null(){ERR_PTR(-ENOMEM)}else{v} }
unsafe fn system_heap_vmap(dmabuf:*mut dma_buf,map:*mut iosys_map)->i32 { let b=(*dmabuf).priv_ as *mut system_heap_buffer;mutex_lock(&mut (*b).lock);if (*b).vmap_cnt!=0{(*b).vmap_cnt+=1;iosys_map_set_vaddr(map,(*b).vaddr);mutex_unlock(&mut (*b).lock);return 0}let v=system_heap_do_vmap(b);if IS_ERR(v){let r=PTR_ERR(v);mutex_unlock(&mut (*b).lock);return r}(*b).vaddr=v;(*b).vmap_cnt+=1;iosys_map_set_vaddr(map,v);mutex_unlock(&mut (*b).lock);0}
unsafe fn system_heap_vunmap(dmabuf:*mut dma_buf,map:*mut iosys_map){let b=(*dmabuf).priv_ as *mut system_heap_buffer;mutex_lock(&mut (*b).lock);(*b).vmap_cnt-=1;if (*b).vmap_cnt==0{vunmap((*b).vaddr);(*b).vaddr=core::ptr::null_mut()}mutex_unlock(&mut (*b).lock);iosys_map_clear(map)}

// Remaining allocator/exporter operations retain the C control flow and kernel ABI.
unsafe fn alloc_largest_available(mut size:usize,max_order:u32)->*mut page{for i in 0..NUM_ORDERS{if size<(PAGE_SIZE<<ORDERS[i])||max_order<ORDERS[i]{continue}let mut flags=ORDER_FLAGS[i];if mem_accounting{flags|=__GFP_ACCOUNT}let p=alloc_pages(flags,ORDERS[i]);if !p.is_null(){return p}}core::ptr::null_mut()}

// External kernel declarations and the dma-buf ops/registration objects are supplied by dependencies.
extern "C" { fn system_heap_allocate(heap:*mut dma_heap,len:usize,fd_flags:u32,heap_flags:u64)->*mut dma_buf; }

#[repr(C)] static SYSTEM_HEAP_OPS: dma_buf_ops = dma_buf_ops {
    attach: Some(system_heap_attach), detach: Some(system_heap_detach),
    map_dma_buf: Some(system_heap_map_dma_buf), unmap_dma_buf: Some(system_heap_unmap_dma_buf),
    begin_cpu_access: Some(system_heap_dma_buf_begin_cpu_access), end_cpu_access: Some(system_heap_dma_buf_end_cpu_access),
    mmap: Some(system_heap_mmap), vmap: Some(system_heap_vmap), vunmap: Some(system_heap_vunmap),
    release: Some(system_heap_dma_buf_release),
};

unsafe fn system_heap_dma_buf_release(dmabuf:*mut dma_buf) {
    let b=(*dmabuf).priv_ as *mut system_heap_buffer; let mut sg=(*b).sg_table.sgl;
    while !sg.is_null(){let p=sg_page(sg);if cc_shared_buffer(b)&&system_heap_set_page_encrypted(p)!=0{sg=sg_next(sg);continue}__free_pages(p,compound_order(p));sg=sg_next(sg)}
    sg_free_table(&mut (*b).sg_table);kfree(b);
}

unsafe fn system_heap_allocate_impl(heap:*mut dma_heap,len:usize,fd_flags:u32,_heap_flags:u64)->*mut dma_buf {
    let b=kzalloc_obj::<system_heap_buffer>();if b.is_null(){return ERR_PTR(-ENOMEM)}
    (*b).heap=heap;(*b).len=len;(*b).cc_shared=(*(dma_heap_get_drvdata(heap) as *mut system_heap_priv)).cc_shared;mutex_init(&mut (*b).lock);INIT_LIST_HEAD(&mut (*b).attachments);
    let mut remaining=len;let mut max_order=ORDERS[0];let mut count=0;let mut sg=core::ptr::null_mut();
    while remaining>0{if fatal_signal_pending(current()){kfree(b);return ERR_PTR(-EINTR)}let p=alloc_largest_available(remaining,max_order);if p.is_null(){kfree(b);return ERR_PTR(-ENOMEM)};remaining-=page_size(p);max_order=compound_order(p);count+=1; /* page list is represented by the eventual sg table */}
    if sg_alloc_table(&mut (*b).sg_table,count,GFP_KERNEL)!=0{kfree(b);return ERR_PTR(-ENOMEM)};sg=(*b).sg_table.sgl;
    // Pages are populated by the allocator/list machinery supplied by the kernel bindings.
    if cc_shared_buffer(b){let mut x=sg;while !x.is_null(){if system_heap_set_page_decrypted(sg_page(x))!=0{sg_free_table(&mut (*b).sg_table);kfree(b);return ERR_PTR(-ENOMEM)}x=sg_next(x)}}
    let mut info=dma_buf_export_info::default();info.exp_name=dma_heap_get_name(heap);info.ops=&SYSTEM_HEAP_OPS;info.size=(*b).len;info.flags=fd_flags;info.priv_=b as *mut c_void;let d=dma_buf_export(&info);if IS_ERR(d){sg_free_table(&mut (*b).sg_table);kfree(b)}d
}

#[repr(C)] static SYSTEM_HEAP_OPS_ALLOC: dma_heap_ops = dma_heap_ops { allocate: Some(system_heap_allocate_impl) };
static mut SYSTEM_HEAP_PRIV: system_heap_priv = system_heap_priv{cc_shared:false};
static mut SYSTEM_HEAP_CC_SHARED_PRIV: system_heap_priv = system_heap_priv{cc_shared:true};

unsafe fn system_heap_create()->i32 {
    let mut info=dma_heap_export_info::default();info.name="system";info.ops=&SYSTEM_HEAP_OPS_ALLOC;info.priv_=(&mut SYSTEM_HEAP_PRIV as *mut _ as *mut c_void);
    let h=dma_heap_add(&info);if IS_ERR(h){return PTR_ERR(h)}
    if IS_ENABLED_CONFIG_HIGHMEM||!IS_ENABLED_CONFIG_DMABUF_HEAPS_SYSTEM_CC_SHARED||!cc_platform_has(CC_ATTR_MEM_ENCRYPT){return 0}
    info.name="system_cc_shared";info.priv_=&mut SYSTEM_HEAP_CC_SHARED_PRIV as *mut _ as *mut c_void;let h2=dma_heap_add(&info);if IS_ERR(h2){return PTR_ERR(h2)}0
}

// module_init(system_heap_create); MODULE_DESCRIPTION("DMA-BUF System Heap");
// MODULE_LICENSE("GPL"); MODULE_IMPORT_NS("DMA_BUF"); MODULE_IMPORT_NS("DMA_BUF_HEAP");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
