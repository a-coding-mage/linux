/* Direct source-level Rust translation of amdgpu_object.c. */

unsafe fn amdgpu_bo_destroy(tbo: *mut ttm_buffer_object) {
    let bo = ttm_to_amdgpu_bo(tbo);
    amdgpu_bo_kunmap(bo);
    if drm_gem_is_imported(&mut (*bo).tbo.base) { drm_prime_gem_destroy(&mut (*bo).tbo.base, (*bo).tbo.sg); }
    drm_gem_object_release(&mut (*bo).tbo.base);
    amdgpu_bo_unref(&mut (*bo).parent);
    kvfree(bo as *mut _);
}

unsafe fn amdgpu_bo_user_destroy(tbo: *mut ttm_buffer_object) {
    let bo = ttm_to_amdgpu_bo(tbo);
    let ubo = to_amdgpu_bo_user(bo);
    kfree((*ubo).metadata as *mut _);
    amdgpu_bo_destroy(tbo);
}

pub unsafe fn amdgpu_bo_is_amdgpu_bo(bo: *mut ttm_buffer_object) -> bool {
    (*bo).destroy == Some(amdgpu_bo_destroy) || (*bo).destroy == Some(amdgpu_bo_user_destroy) || (*bo).destroy == Some(svm_range_bo_destroy)
}

pub unsafe fn amdgpu_bo_placement_from_domain(abo: *mut amdgpu_bo, domain: u32) {
    let adev = amdgpu_ttm_adev((*abo).tbo.bdev);
    let placement = &mut (*abo).placement;
    let places = (*abo).placements;
    let flags = (*abo).flags;
    let mut c: u32 = 0;
    if domain & AMDGPU_GEM_DOMAIN_VRAM != 0 {
        let visible_pfn = (*adev).gmc.visible_vram_size >> PAGE_SHIFT;
        let mem_id = KFD_XCP_MEM_ID(adev, (*abo).xcp_id);
        if !(*adev).gmc.mem_partitions.is_null() && mem_id >= 0 {
            (*places.add(c as usize)).fpfn = (*adev).gmc.mem_partitions.offset(mem_id as isize).as_ref().unwrap().range.fpfn;
            (*places.add(c as usize)).lpfn = (*adev).gmc.mem_partitions.offset(mem_id as isize).as_ref().unwrap().range.lpfn + 1;
        } else { (*places.add(c as usize)).fpfn = 0; (*places.add(c as usize)).lpfn = 0; }
        (*places.add(c as usize)).mem_type = TTM_PL_VRAM; (*places.add(c as usize)).flags = 0;
        if flags & AMDGPU_GEM_CREATE_CPU_ACCESS_REQUIRED != 0 { (*places.add(c as usize)).lpfn = min_not_zero((*places.add(c as usize)).lpfn, visible_pfn); } else { (*places.add(c as usize)).flags |= TTM_PL_FLAG_TOPDOWN; }
        if (*abo).tbo.type_ == ttm_bo_type_kernel && flags & AMDGPU_GEM_CREATE_VRAM_CONTIGUOUS != 0 { (*places.add(c as usize)).flags |= TTM_PL_FLAG_CONTIGUOUS; } c += 1;
    }
    if domain & AMDGPU_GEM_DOMAIN_DOORBELL != 0 { let p=&mut *places.add(c as usize); p.fpfn=0;p.lpfn=0;p.mem_type=AMDGPU_PL_DOORBELL;p.flags=0;c+=1; }
    if domain & AMDGPU_GEM_DOMAIN_GTT != 0 { let p=&mut *places.add(c as usize); p.fpfn=0;p.lpfn=0;p.mem_type=if (*abo).flags&AMDGPU_GEM_CREATE_PREEMPTIBLE!=0{AMDGPU_PL_PREEMPT}else{TTM_PL_TT};p.flags=0; if (*abo).tbo.resource!=core::ptr::null_mut() && (*adev).flags&AMD_IS_APU==0 && domain&(*abo).preferred_domains&AMDGPU_GEM_DOMAIN_VRAM!=0 {p.flags|=TTM_PL_FLAG_FALLBACK;} c+=1; }
    if domain & AMDGPU_GEM_DOMAIN_CPU != 0 { let p=&mut *places.add(c as usize);p.fpfn=0;p.lpfn=0;p.mem_type=TTM_PL_SYSTEM;p.flags=0;c+=1; }
    for (bit, ty) in [(AMDGPU_GEM_DOMAIN_GDS,AMDGPU_PL_GDS),(AMDGPU_GEM_DOMAIN_GWS,AMDGPU_PL_GWS),(AMDGPU_GEM_DOMAIN_OA,AMDGPU_PL_OA)] { if domain&bit!=0 {let p=&mut *places.add(c as usize);p.fpfn=0;p.lpfn=0;p.mem_type=ty;p.flags=0;c+=1;} }
    if c==0 {let p=&mut *places;p.fpfn=0;p.lpfn=0;p.mem_type=TTM_PL_SYSTEM;p.flags=0;c=1;}
    BUG_ON(c > AMDGPU_BO_MAX_PLACEMENTS); (*placement).num_placement=c; (*placement).placement=places;
}

pub unsafe fn amdgpu_bo_create_reserved(adev:*mut amdgpu_device,size:usize,align:i32,domain:u32,bo_ptr:*mut *mut amdgpu_bo,gpu_addr:*mut u64,cpu_addr:*mut *mut core::ffi::c_void)->i32 {
    if size==0 {amdgpu_bo_unref(bo_ptr);return 0;} let mut bp=core::mem::zeroed::<amdgpu_bo_param>(); bp.size=size;bp.byte_align=align;bp.domain=domain;bp.flags=if cpu_addr.is_null(){AMDGPU_GEM_CREATE_NO_CPU_ACCESS}else{AMDGPU_GEM_CREATE_CPU_ACCESS_REQUIRED};bp.flags|=AMDGPU_GEM_CREATE_VRAM_CONTIGUOUS;bp.type_=ttm_bo_type_kernel;bp.bo_ptr_size=core::mem::size_of::<amdgpu_bo>(); let mut free=false; let mut r;
    if (*bo_ptr).is_null(){r=amdgpu_bo_create(adev,&mut bp,bo_ptr);if r!=0{return r;}free=true;} r=amdgpu_bo_reserve(*bo_ptr,false);if r!=0{if free{amdgpu_bo_unref(bo_ptr)}return r;} if free {r=amdgpu_bo_pin(*bo_ptr,domain);if r!=0{amdgpu_bo_unreserve(*bo_ptr);amdgpu_bo_unref(bo_ptr);return r;}} r=amdgpu_ttm_alloc_gart(&mut (*bo_ptr).as_mut().unwrap().tbo);if r!=0{if free{amdgpu_bo_unpin(*bo_ptr)}amdgpu_bo_unreserve(*bo_ptr);if free{amdgpu_bo_unref(bo_ptr)}return r;} if !gpu_addr.is_null(){*gpu_addr=amdgpu_bo_gpu_offset(*bo_ptr);} if !cpu_addr.is_null(){r=amdgpu_bo_kmap(*bo_ptr,cpu_addr);if r!=0{if free{amdgpu_bo_unpin(*bo_ptr)}amdgpu_bo_unreserve(*bo_ptr);if free{amdgpu_bo_unref(bo_ptr)}return r;}} 0
}

pub unsafe fn amdgpu_bo_create_kernel(adev:*mut amdgpu_device,size:usize,align:i32,domain:u32,bo:*mut *mut amdgpu_bo,gpu:*mut u64,cpu:*mut *mut core::ffi::c_void)->i32 {let r=amdgpu_bo_create_reserved(adev,size,align,domain,bo,gpu,cpu);if r==0&&!(*bo).is_null(){amdgpu_bo_unreserve(*bo);}r}

pub unsafe fn amdgpu_bo_create_kernel_at(adev:*mut amdgpu_device,mut offset:u64,mut size:u64,bo:*mut *mut amdgpu_bo,cpu:*mut *mut core::ffi::c_void)->i32 {offset&=PAGE_MASK;size=ALIGN(size,PAGE_SIZE as u64);let mut r=amdgpu_bo_create_reserved(adev,size as usize,PAGE_SIZE,AMDGPU_GEM_DOMAIN_VRAM,bo,core::ptr::null_mut(),cpu);if r!=0||(*bo).is_null(){return r;}if !cpu.is_null(){amdgpu_bo_kunmap(*bo);}ttm_resource_free(&mut (*bo).tbo,&mut (*bo).tbo.resource);for i in 0..(*bo).placement.num_placement as usize{(*bo).placements.add(i).as_mut().unwrap().fpfn=offset>>PAGE_SHIFT;(*bo).placements.add(i).as_mut().unwrap().lpfn=(offset+size)>>PAGE_SHIFT;}let mut ctx=ttm_operation_ctx{interruptible:false,no_wait_gpu:false,..core::mem::zeroed()};r=ttm_bo_mem_space(&mut (*bo).tbo,&mut (*bo).placement,&mut (*bo).tbo.resource,&mut ctx);if r==0&&!cpu.is_null(){r=amdgpu_bo_kmap(*bo,cpu);}amdgpu_bo_unreserve(*bo);if r!=0{amdgpu_bo_unref(bo);}r}

pub unsafe fn amdgpu_bo_free_kernel(bo:*mut *mut amdgpu_bo,gpu:*mut u64,cpu:*mut *mut core::ffi::c_void){if (*bo).is_null(){return;}if amdgpu_bo_reserve(*bo,true)==0{if !cpu.is_null(){amdgpu_bo_kunmap(*bo);}amdgpu_bo_unpin(*bo);amdgpu_bo_unreserve(*bo);}amdgpu_bo_unref(bo);if !gpu.is_null(){*gpu=0;}if !cpu.is_null(){*cpu=core::ptr::null_mut();}}

pub unsafe fn amdgpu_bo_validate_size(adev:*mut amdgpu_device,size:usize,domain:u32)->bool{let man=if domain&AMDGPU_GEM_DOMAIN_GTT!=0{ttm_manager_type(&mut (*adev).mman.bdev,TTM_PL_TT)}else if domain&AMDGPU_GEM_DOMAIN_VRAM!=0{ttm_manager_type(&mut (*adev).mman.bdev,TTM_PL_VRAM)}else{return true};!man.is_null()&&size<(*man).size}
pub unsafe fn amdgpu_bo_support_uswc(_flags:u64)->bool{drm_arch_can_wc_memory()}

pub unsafe fn amdgpu_bo_kmap(bo:*mut amdgpu_bo,ptr:*mut *mut core::ffi::c_void)->i32{if (*bo).flags&AMDGPU_GEM_CREATE_NO_CPU_ACCESS!=0{return -EPERM;}let k=amdgpu_bo_kptr(bo);if !k.is_null(){if !ptr.is_null(){*ptr=k;}return 0;}let r=ttm_bo_kmap(&mut (*bo).tbo,0,PFN_UP((*bo).tbo.base.size),&mut (*bo).kmap);if r==0&&!ptr.is_null(){*ptr=amdgpu_bo_kptr(bo);}r}
pub unsafe fn amdgpu_bo_kptr(bo:*mut amdgpu_bo)->*mut core::ffi::c_void{let mut iomem=false;ttm_kmap_obj_virtual(&mut (*bo).kmap,&mut iomem)}
pub unsafe fn amdgpu_bo_kunmap(bo:*mut amdgpu_bo){if !(*bo).kmap.bo.is_null(){ttm_bo_kunmap(&mut (*bo).kmap);}}
pub unsafe fn amdgpu_bo_ref(bo:*mut amdgpu_bo)->*mut amdgpu_bo{if !bo.is_null(){drm_gem_object_get(&mut (*bo).tbo.base);}bo}
pub unsafe fn amdgpu_bo_unref(bo:*mut *mut amdgpu_bo){if !(*bo).is_null(){drm_gem_object_put(&mut (*(*bo)).tbo.base);*bo=core::ptr::null_mut();}}

pub unsafe fn amdgpu_bo_get_preferred_domain(adev:*mut amdgpu_device,mut domain:u32)->u32{if domain== (AMDGPU_GEM_DOMAIN_VRAM|AMDGPU_GEM_DOMAIN_GTT)&&((*adev).asic_type==CHIP_CARRIZO||(*adev).asic_type==CHIP_STONEY){domain=if (*adev).gmc.real_vram_size<=AMDGPU_SG_THRESHOLD{AMDGPU_GEM_DOMAIN_GTT}else{AMDGPU_GEM_DOMAIN_VRAM};}domain}
pub unsafe fn amdgpu_bo_gpu_offset_no_check(bo:*mut amdgpu_bo)->u64{let adev=amdgpu_ttm_adev((*bo).tbo.bdev);let mut o=AMDGPU_BO_INVALID_OFFSET;if (*bo).tbo.resource.as_ref().unwrap().mem_type==TTM_PL_TT{o=amdgpu_gmc_agp_addr(&mut (*bo).tbo);}if o==AMDGPU_BO_INVALID_OFFSET{o=((*bo).tbo.resource.as_ref().unwrap().start<<PAGE_SHIFT)+amdgpu_ttm_domain_start(adev,(*bo).tbo.resource.as_ref().unwrap().mem_type);}amdgpu_gmc_sign_extend(o)}
pub unsafe fn amdgpu_bo_gpu_offset(bo:*mut amdgpu_bo)->u64{amdgpu_bo_gpu_offset_no_check(bo)}
pub unsafe fn amdgpu_bo_mem_stats_placement(bo:*mut amdgpu_bo)->u32{let d=(*bo).preferred_domains&AMDGPU_GEM_DOMAIN_MASK;if d==0{return TTM_PL_SYSTEM;}rounddown_pow_of_two(d)}

pub unsafe fn amdgpu_bo_pin(bo:*mut amdgpu_bo,domain:u32)->i32{let adev=amdgpu_ttm_adev((*bo).tbo.bdev);if !amdgpu_ttm_tt_get_usermm((*bo).tbo.ttm).is_null(){return -EPERM;}let mut d=domain&(*bo).preferred_domains;if d==0{d=domain;}if drm_gem_is_imported(&mut (*bo).tbo.base)&&d&AMDGPU_GEM_DOMAIN_GTT==0{return -EINVAL;}if (*bo).tbo.pin_count!=0{ttm_bo_pin(&mut (*bo).tbo);return 0;}d=amdgpu_bo_get_preferred_domain(adev,d);if drm_gem_is_imported(&mut (*bo).tbo.base){dma_buf_pin((*bo).tbo.base.import_attach);}if (*bo).flags&AMDGPU_GEM_CREATE_NO_CPU_ACCESS==0{(*bo).flags|=AMDGPU_GEM_CREATE_CPU_ACCESS_REQUIRED;}amdgpu_bo_placement_from_domain(bo,d);let mut ctx=ttm_operation_ctx{interruptible:false,no_wait_gpu:false,..core::mem::zeroed()};let r=ttm_bo_validate(&mut (*bo).tbo,&mut (*bo).placement,&mut ctx);if r==0{ttm_bo_pin(&mut (*bo).tbo);}r}
pub unsafe fn amdgpu_bo_unpin(bo:*mut amdgpu_bo){ttm_bo_unpin(&mut (*bo).tbo);if (*bo).tbo.pin_count!=0{return;}if drm_gem_is_imported(&mut (*bo).tbo.base){dma_buf_unpin((*bo).tbo.base.import_attach);}}
pub unsafe fn amdgpu_bo_set_tiling_flags(bo:*mut amdgpu_bo,flags:u64)->i32{BUG_ON((*bo).tbo.type_==ttm_bo_type_kernel);if (*to_amdgpu_bo_user(bo)).tiling_flags==flags{return 0;}(*to_amdgpu_bo_user(bo)).tiling_flags=flags;0}
pub unsafe fn amdgpu_bo_get_tiling_flags(bo:*mut amdgpu_bo,out:*mut u64){if !out.is_null(){*out=(*to_amdgpu_bo_user(bo)).tiling_flags;}}
pub unsafe fn amdgpu_bo_set_metadata(bo:*mut amdgpu_bo,metadata:*mut core::ffi::c_void,size:u32,flags:u64)->i32{let u=to_amdgpu_bo_user(bo);if size==0{if !(*u).metadata.is_null(){kfree((*u).metadata as *mut _);(*u).metadata=core::ptr::null_mut();}(*u).metadata_size=0;return 0;}if metadata.is_null(){return -EINVAL;}let b=kmemdup(metadata,size,GFP_KERNEL);if b.is_null(){return -ENOMEM;}kfree((*u).metadata as *mut _);(*u).metadata=b;(*u).metadata_size=size;(*u).metadata_flags=flags;0}
pub unsafe fn amdgpu_bo_get_metadata(bo:*mut amdgpu_bo,buffer:*mut core::ffi::c_void,buffer_size:usize,size:*mut u32,flags:*mut u64)->i32{if buffer.is_null()&&size.is_null(){return -EINVAL;}let u=to_amdgpu_bo_user(bo);if !size.is_null(){*size=(*u).metadata_size;}if !buffer.is_null(){if buffer_size<(*u).metadata_size as usize{return -EINVAL;}if (*u).metadata_size!=0{memcpy(buffer,(*u).metadata,(*u).metadata_size as usize);}}if !flags.is_null(){*flags=(*u).metadata_flags;}0}
pub unsafe fn amdgpu_bo_move_notify(bo:*mut ttm_buffer_object,evict:bool,new_mem:*mut ttm_resource){if !amdgpu_bo_is_amdgpu_bo(bo){return;}let abo=ttm_to_amdgpu_bo(bo);amdgpu_vm_bo_move(abo,new_mem,evict);amdgpu_bo_kunmap(abo);trace_amdgpu_bo_move(abo,if new_mem.is_null(){-1}else{(*new_mem).mem_type as i32},if (*bo).resource.is_null(){-1}else{(*(*bo).resource).mem_type as i32});}
pub unsafe fn amdgpu_bo_sync_wait(bo:*mut amdgpu_bo,owner:*mut core::ffi::c_void,intr:bool)->i32{let adev=amdgpu_ttm_adev((*bo).tbo.bdev);amdgpu_bo_sync_wait_resv(adev,(*bo).tbo.base.resv,AMDGPU_SYNC_NE_OWNER,owner,intr)}
pub unsafe fn amdgpu_bo_sync_wait_resv(adev:*mut amdgpu_device,resv:*mut dma_resv,mode:amdgpu_sync_mode,owner:*mut core::ffi::c_void,intr:bool)->i32{let mut sync=core::mem::zeroed();amdgpu_sync_create(&mut sync);amdgpu_sync_resv(adev,&mut sync,resv,mode,owner);let r=amdgpu_sync_wait(&mut sync,intr);amdgpu_sync_free(&mut sync);r}
pub unsafe fn amdgpu_bo_fence(bo:*mut amdgpu_bo,fence:*mut dma_fence,shared:bool){let resv=(*bo).tbo.base.resv;if dma_resv_reserve_fences(resv,1)!=0{dma_fence_wait(fence,false);return;}dma_resv_add_fence(resv,fence,if shared{DMA_RESV_USAGE_READ}else{DMA_RESV_USAGE_WRITE});}
pub unsafe fn amdgpu_bo_create_user(adev:*mut amdgpu_device,bp:*mut amdgpu_bo_param,out:*mut *mut amdgpu_bo_user)->i32{(*bp).bo_ptr_size=core::mem::size_of::<amdgpu_bo_user>();(*bp).destroy=Some(amdgpu_bo_user_destroy);let mut bo=core::ptr::null_mut();let r=amdgpu_bo_create(adev,bp,&mut bo);if r==0{*out=to_amdgpu_bo_user(bo);}r}
pub unsafe fn amdgpu_bo_create_vm(adev:*mut amdgpu_device,bp:*mut amdgpu_bo_param,out:*mut *mut amdgpu_bo_vm)->i32{BUG_ON((*bp).bo_ptr_size<core::mem::size_of::<amdgpu_bo_vm>());let mut bo=core::ptr::null_mut();let r=amdgpu_bo_create(adev,bp,&mut bo);if r==0{*out=to_amdgpu_bo_vm(bo);}r}
pub unsafe fn amdgpu_bo_create(adev:*mut amdgpu_device,bp:*mut amdgpu_bo_param,out:*mut *mut amdgpu_bo)->i32{let mut size=(*bp).size;let align=if (*bp).domain&(AMDGPU_GEM_DOMAIN_GWS|AMDGPU_GEM_DOMAIN_OA)!=0{(*bp).byte_align as usize}else if (*bp).domain&AMDGPU_GEM_DOMAIN_GDS!=0{ALIGN((*bp).byte_align as usize,4)}else{ALIGN((*bp).byte_align as usize,PAGE_SIZE)>>PAGE_SHIFT};if (*bp).domain& (AMDGPU_GEM_DOMAIN_GWS|AMDGPU_GEM_DOMAIN_OA|AMDGPU_GEM_DOMAIN_GDS)!=0{size<<=PAGE_SHIFT;}else{size=ALIGN(size,PAGE_SIZE);}if !amdgpu_bo_validate_size(adev,size,(*bp).domain){return -ENOMEM;}let bo=kvzalloc((*bp).bo_ptr_size,GFP_KERNEL) as *mut amdgpu_bo;if bo.is_null(){return -ENOMEM;}*out=bo;(*bo).flags=(*bp).flags;(*bo).preferred_domains=if (*bp).preferred_domain!=0{(*bp).preferred_domain}else{(*bp).domain};(*bo).allowed_domains=(*bo).preferred_domains;(*bo).tbo.bdev=&mut (*adev).mman.bdev;amdgpu_bo_placement_from_domain(bo,(*bp).domain);let mut ctx=core::mem::zeroed();let r=ttm_bo_init_reserved(&mut (*adev).mman.bdev,&mut (*bo).tbo,(*bp).type_,&mut (*bo).placement,align,&mut ctx,core::ptr::null_mut(),(*bp).resv,(*bp).destroy.or(Some(amdgpu_bo_destroy)));if r!=0{*out=core::ptr::null_mut();}r}
pub unsafe fn amdgpu_bo_fault_reserve_notify(bo:*mut ttm_buffer_object)->vm_fault_t{let adev=amdgpu_ttm_adev((*bo).bdev);let abo=ttm_to_amdgpu_bo(bo);(*abo).flags|=AMDGPU_GEM_CREATE_CPU_ACCESS_REQUIRED;if amdgpu_res_cpu_visible(adev,(*bo).resource){return 0;}if (*abo).tbo.pin_count>0{return VM_FAULT_SIGBUS;}amdgpu_bo_placement_from_domain(abo,AMDGPU_GEM_DOMAIN_VRAM|AMDGPU_GEM_DOMAIN_GTT);let mut ctx=ttm_operation_ctx{interruptible:false,no_wait_gpu:false,..core::mem::zeroed()};let r=ttm_bo_validate(bo,&mut (*abo).placement,&mut ctx);if r==0{ttm_bo_move_to_lru_tail_unlocked(bo);0}else{VM_FAULT_SIGBUS}}
pub unsafe fn amdgpu_bo_fb_aper_addr(bo:*mut amdgpu_bo)->u64{let adev=amdgpu_ttm_adev((*bo).tbo.bdev);let o=((*bo).tbo.resource.as_ref().unwrap().start<<PAGE_SHIFT)+(*adev).gmc.fb_start+(*adev).gmc.xgmi.physical_node_id*(*adev).gmc.xgmi.node_segment_size;amdgpu_gmc_sign_extend(o)}
pub unsafe fn amdgpu_bo_init(adev:*mut amdgpu_device)->i32{amdgpu_ttm_init(adev)}
pub unsafe fn amdgpu_bo_fini(adev:*mut amdgpu_device){amdgpu_ttm_fini(adev)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
