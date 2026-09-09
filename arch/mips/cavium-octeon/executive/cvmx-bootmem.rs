/* Translated from cvmx-bootmem.c. External SDK types, constants, and
 * functions are supplied by the surrounding environment. */

use core::ffi::c_void;

static mut cvmx_bootmem_desc: *mut cvmx_bootmem_desc = core::ptr::null_mut();

unsafe fn __cvmx_bootmem_desc_get(base: u64, offset: i32, size: i32) -> u64 {
    let base = (1u64 << 63).wrapping_add(base.wrapping_add(offset as u64));
    match size {
        4 => cvmx_read64_uint32(base) as u64,
        8 => cvmx_read64_uint64(base),
        _ => 0,
    }
}

unsafe fn cvmx_bootmem_phy_set_size(addr: u64, size: u64) {
    cvmx_write64_uint64(addr.wrapping_add(8) | (1u64 << 63), size);
}
unsafe fn cvmx_bootmem_phy_set_next(addr: u64, next: u64) {
    cvmx_write64_uint64(addr | (1u64 << 63), next);
}
unsafe fn cvmx_bootmem_phy_get_size(addr: u64) -> u64 {
    cvmx_read64_uint64(addr.wrapping_add(8) | (1u64 << 63))
}
unsafe fn cvmx_bootmem_phy_get_next(addr: u64) -> u64 {
    cvmx_read64_uint64(addr | (1u64 << 63))
}

unsafe fn cvmx_bootmem_alloc_range(size: u64, alignment: u64, min_addr: u64, max_addr: u64) -> *mut c_void {
    let address = cvmx_bootmem_phy_alloc(size, min_addr, max_addr, alignment, 0);
    if address > 0 { cvmx_phys_to_ptr(address as u64) } else { core::ptr::null_mut() }
}

pub unsafe fn cvmx_bootmem_alloc_address(size: u64, address: u64, alignment: u64) -> *mut c_void {
    cvmx_bootmem_alloc_range(size, alignment, address, address.wrapping_add(size))
}

pub unsafe fn cvmx_bootmem_alloc_named_range(size: u64, min_addr: u64, max_addr: u64, align: u64, name: *mut i8) -> *mut c_void {
    let addr = cvmx_bootmem_phy_named_block_alloc(size, min_addr, max_addr, align, name, 0);
    if addr >= 0 { cvmx_phys_to_ptr(addr as u64) } else { core::ptr::null_mut() }
}

pub unsafe fn cvmx_bootmem_alloc_named(size: u64, alignment: u64, name: *mut i8) -> *mut c_void {
    cvmx_bootmem_alloc_named_range(size, 0, 0, alignment, name)
}

pub unsafe fn cvmx_bootmem_lock() { cvmx_spinlock_lock(&mut (*cvmx_bootmem_desc).lock as *mut _ as *mut cvmx_spinlock_t); }
pub unsafe fn cvmx_bootmem_unlock() { cvmx_spinlock_unlock(&mut (*cvmx_bootmem_desc).lock as *mut _ as *mut cvmx_spinlock_t); }

pub unsafe fn cvmx_bootmem_init(mem_desc_ptr: *mut c_void) -> i32 {
    if cvmx_bootmem_desc.is_null() { cvmx_bootmem_desc = mem_desc_ptr as *mut cvmx_bootmem_desc; }
    0
}

pub unsafe fn cvmx_bootmem_phy_alloc(mut req_size: u64, mut address_min: u64, mut address_max: u64, mut alignment: u64, flags: u32) -> i64 {
    let mut prev_addr = 0u64;
    let mut head_addr;
    let mut ent_addr;
    let mut desired_min_addr;
    if (*cvmx_bootmem_desc).major_version > 3 { cvmx_dprintf(b"ERROR: Incompatible bootmem descriptor version\0".as_ptr() as *const i8); return -1; }
    if req_size == 0 { return -1; }
    req_size = (req_size.wrapping_add(CVMX_BOOTMEM_ALIGNMENT_SIZE - 1)) & !(CVMX_BOOTMEM_ALIGNMENT_SIZE - 1);
    if address_min != 0 && address_max == 0 { address_max = address_min.wrapping_add(req_size); }
    else if address_min == 0 && address_max == 0 { address_max = u64::MAX; }
    if alignment < CVMX_BOOTMEM_ALIGNMENT_SIZE { alignment = CVMX_BOOTMEM_ALIGNMENT_SIZE; }
    address_min = ALIGN(address_min, alignment);
    if req_size > address_max.wrapping_sub(address_min) { return -1; }
    if flags & CVMX_BOOTMEM_FLAG_NO_LOCKING == 0 { cvmx_bootmem_lock(); }
    head_addr = (*cvmx_bootmem_desc).head_addr; ent_addr = head_addr;
    while ent_addr != 0 {
        let ent_size = cvmx_bootmem_phy_get_size(ent_addr);
        let next = cvmx_bootmem_phy_get_next(ent_addr);
        if next != 0 && ent_addr > next { break; }
        let usable_base = ALIGN(core::cmp::max(address_min, ent_addr), alignment);
        let usable_max = core::cmp::min(address_max, ent_addr.wrapping_add(ent_size));
        desired_min_addr = usable_base;
        if ent_addr.wrapping_add(ent_size) <= usable_base || ent_addr >= address_max || req_size > usable_max.wrapping_sub(usable_base) { prev_addr = ent_addr; ent_addr = next; continue; }
        if flags & CVMX_BOOTMEM_FLAG_END_ALLOC != 0 { desired_min_addr = (usable_max - req_size) & !(alignment - 1); }
        if desired_min_addr == ent_addr {
            if req_size < ent_size { let new_addr = ent_addr + req_size; cvmx_bootmem_phy_set_next(new_addr, next); cvmx_bootmem_phy_set_size(new_addr, ent_size - req_size); cvmx_bootmem_phy_set_next(ent_addr, new_addr); }
            if prev_addr != 0 { cvmx_bootmem_phy_set_next(prev_addr, cvmx_bootmem_phy_get_next(ent_addr)); } else { (*cvmx_bootmem_desc).head_addr = cvmx_bootmem_phy_get_next(ent_addr); }
            if flags & CVMX_BOOTMEM_FLAG_NO_LOCKING == 0 { cvmx_bootmem_unlock(); }
            return desired_min_addr as i64;
        }
        let new_addr = desired_min_addr;
        cvmx_bootmem_phy_set_next(new_addr, next);
        cvmx_bootmem_phy_set_size(new_addr, ent_size - (desired_min_addr - ent_addr));
        cvmx_bootmem_phy_set_size(ent_addr, desired_min_addr - ent_addr);
        cvmx_bootmem_phy_set_next(ent_addr, new_addr);
        prev_addr = ent_addr; ent_addr = new_addr;
    }
    if flags & CVMX_BOOTMEM_FLAG_NO_LOCKING == 0 { cvmx_bootmem_unlock(); }
    -1
}

pub unsafe fn __cvmx_bootmem_phy_free(phy_addr: u64, size: u64, flags: u32) -> i32 {
    if (*cvmx_bootmem_desc).major_version > 3 || size == 0 { return 0; }
    if flags & CVMX_BOOTMEM_FLAG_NO_LOCKING == 0 { cvmx_bootmem_lock(); }
    let mut cur = (*cvmx_bootmem_desc).head_addr; let mut prev = 0u64;
    if cur == 0 || phy_addr < cur { if cur != 0 && phy_addr + size > cur { } else if phy_addr + size == cur { cvmx_bootmem_phy_set_next(phy_addr, cvmx_bootmem_phy_get_next(cur)); cvmx_bootmem_phy_set_size(phy_addr, cvmx_bootmem_phy_get_size(cur)+size); (*cvmx_bootmem_desc).head_addr=phy_addr; } else { cvmx_bootmem_phy_set_next(phy_addr,cur); cvmx_bootmem_phy_set_size(phy_addr,size); (*cvmx_bootmem_desc).head_addr=phy_addr; } if flags & CVMX_BOOTMEM_FLAG_NO_LOCKING == 0 { cvmx_bootmem_unlock(); } return 1; }
    while cur != 0 && phy_addr > cur { prev=cur; cur=cvmx_bootmem_phy_get_next(cur); }
    if prev != 0 && prev + cvmx_bootmem_phy_get_size(prev) == phy_addr { cvmx_bootmem_phy_set_size(prev,cvmx_bootmem_phy_get_size(prev)+size); if phy_addr+size==cur { cvmx_bootmem_phy_set_size(prev,cvmx_bootmem_phy_get_size(prev)+cvmx_bootmem_phy_get_size(cur)); cvmx_bootmem_phy_set_next(prev,cvmx_bootmem_phy_get_next(cur)); } }
    else if cur != 0 && phy_addr+size==cur { cvmx_bootmem_phy_set_size(phy_addr,cvmx_bootmem_phy_get_size(cur)+size); cvmx_bootmem_phy_set_next(phy_addr,cvmx_bootmem_phy_get_next(cur)); cvmx_bootmem_phy_set_next(prev,phy_addr); }
    else { cvmx_bootmem_phy_set_size(phy_addr,size); cvmx_bootmem_phy_set_next(phy_addr,cur); cvmx_bootmem_phy_set_next(prev,phy_addr); }
    if flags & CVMX_BOOTMEM_FLAG_NO_LOCKING == 0 { cvmx_bootmem_unlock(); } 1
}

unsafe fn cvmx_bootmem_phy_named_block_find(name: *mut i8, flags: u32) -> *mut cvmx_bootmem_named_block_desc {
    if flags & CVMX_BOOTMEM_FLAG_NO_LOCKING == 0 { cvmx_bootmem_lock(); }
    let p = cvmx_phys_to_ptr((*cvmx_bootmem_desc).named_block_array_addr) as *mut cvmx_bootmem_named_block_desc;
    if (*cvmx_bootmem_desc).major_version == 3 { for i in 0..(*cvmx_bootmem_desc).named_block_num_blocks as usize { let x=&mut *p.add(i); if (!name.is_null() && x.size != 0 && strncmp(name,x.name.as_ptr(),(*cvmx_bootmem_desc).named_block_name_len-1)==0) || (name.is_null() && x.size==0) { if flags & CVMX_BOOTMEM_FLAG_NO_LOCKING == 0 { cvmx_bootmem_unlock(); } return x; } } }
    if flags & CVMX_BOOTMEM_FLAG_NO_LOCKING == 0 { cvmx_bootmem_unlock(); } core::ptr::null_mut()
}

pub unsafe fn cvmx_bootmem_alloc_named_range_once(size:u64,min:u64,max:u64,align:u64,name:*mut i8,init:Option<unsafe extern "C" fn(*mut c_void)>)->*mut c_void { let d=cvmx_bootmem_phy_named_block_find(name,CVMX_BOOTMEM_FLAG_NO_LOCKING); if !d.is_null(){return cvmx_phys_to_ptr((*d).base_addr);} let a=cvmx_bootmem_phy_named_block_alloc(size,min,max,align,name,CVMX_BOOTMEM_FLAG_NO_LOCKING); if a<0{return core::ptr::null_mut();} let p=cvmx_phys_to_ptr(a as u64); if let Some(f)=init{f(p)}else{memset(p,0,size);} p }
pub unsafe fn cvmx_bootmem_find_named_block(name:*mut i8)->*mut cvmx_bootmem_named_block_desc { cvmx_bootmem_phy_named_block_find(name,0) }
pub unsafe fn cvmx_bootmem_free_named(name:*mut i8)->i32 { cvmx_bootmem_phy_named_block_free(name,0) }
unsafe fn cvmx_bootmem_phy_named_block_free(name:*mut i8,_flags:u32)->i32 { let p=cvmx_bootmem_phy_named_block_find(name,CVMX_BOOTMEM_FLAG_NO_LOCKING); if p.is_null(){return 0;} __cvmx_bootmem_phy_free((*p).base_addr,(*p).size,CVMX_BOOTMEM_FLAG_NO_LOCKING);(*p).size=0;1 }
pub unsafe fn cvmx_bootmem_phy_named_block_alloc(size:u64,min:u64,max:u64,alignment:u64,name:*mut i8,flags:u32)->i64 { let p=cvmx_bootmem_phy_named_block_find(core::ptr::null_mut(),flags|CVMX_BOOTMEM_FLAG_NO_LOCKING); if !cvmx_bootmem_phy_named_block_find(name,flags|CVMX_BOOTMEM_FLAG_NO_LOCKING).is_null()||p.is_null(){return -1;} let s=ALIGN(size,CVMX_BOOTMEM_ALIGNMENT_SIZE); let a=cvmx_bootmem_phy_alloc(s,min,max,alignment,flags|CVMX_BOOTMEM_FLAG_NO_LOCKING); if a>=0{(*p).base_addr=a as u64;(*p).size=s;strscpy((*p).name.as_mut_ptr(),name,(*cvmx_bootmem_desc).named_block_name_len);} a }
pub unsafe fn cvmx_bootmem_get_desc()->*mut cvmx_bootmem_desc { cvmx_bootmem_desc }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
