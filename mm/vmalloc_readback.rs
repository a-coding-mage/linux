// SPDX-License-Identifier: GPL-2.0-only
// vmalloc.c:4515-4892. Safe debug reads and user mappings.
unsafe fn zero_iter(iter: *mut iov_iter, count: usize) -> usize {
    let mut remains = count;
    while remains > 0 {
        let num = min(remains, PAGE_SIZE as usize);
        let copied = copy_page_to_iter_nofault(zero_page(), 0, num, iter);
        remains -= copied;
        if copied < num {
            break;
        }
    }
    count - remains
}
unsafe fn aligned_vread_iter(iter: *mut iov_iter, mut addr: *const Char, count: usize) -> usize {
    let mut remains = count;
    while remains > 0 {
        let offset = addr as ULong & (PAGE_SIZE - 1);
        let length = min(PAGE_SIZE as usize - offset as usize, remains);
        let page = vmalloc_to_page(addr.cast());
        let copied = if !page.is_null() {
            copy_page_to_iter_nofault(page, offset as u32, length, iter)
        } else {
            zero_iter(iter, length)
        };
        addr = addr.wrapping_add(copied);
        remains -= copied;
        if copied != length {
            break;
        }
    }
    count - remains
}
unsafe fn vmap_ram_vread_iter(
    iter: *mut iov_iter,
    mut addr: *const Char,
    count: usize,
    flags: ULong,
) -> usize {
    if flags & VMAP_BLOCK == 0 {
        return aligned_vread_iter(iter, addr, count);
    }
    let mut remains = count;
    let xa = addr_to_vb_xa(addr as ULong);
    let vb = xa_load(xa, addr_to_vb_idx(addr as ULong)).cast::<vmap_block>();
    if vb.is_null() {
        return zero_iter(iter, remains);
    }
    spin_lock(&raw mut (*vb).lock);
    let map = (&raw const (*vb).used_map).cast::<ULong>();
    if bitmap_empty(map, VMAP_BBMAP_BITS as u32) {
        spin_unlock(&raw mut (*vb).lock);
        return zero_iter(iter, remains);
    }
    let mut rs = find_next_bit(map, VMAP_BBMAP_BITS, 0) as u32;
    while (rs as ULong) < VMAP_BBMAP_BITS {
        let re = find_next_zero_bit(map, VMAP_BBMAP_BITS, rs as ULong + 1) as u32;
        if remains == 0 {
            spin_unlock(&raw mut (*vb).lock);
            return count - remains;
        }
        let start = vmap_block_vaddr((*(*vb).va).va_start, rs as ULong).cast::<Char>();
        if (addr as usize) < start as usize {
            let to_zero = min((start as usize).wrapping_sub(addr as usize), remains);
            let zeroed = zero_iter(iter, to_zero);
            addr = addr.wrapping_add(zeroed);
            remains -= zeroed;
            if remains == 0 || zeroed != to_zero {
                spin_unlock(&raw mut (*vb).lock);
                return count - remains;
            }
        }
        let offset = addr as ULong & (PAGE_SIZE - 1);
        // Preserve vmalloc.c's exact read range expression, including its +1.
        let n = min(
            (((re - rs + 1) as ULong) << PAGE_SHIFT).wrapping_sub(offset) as usize,
            remains,
        );
        let copied = aligned_vread_iter(iter, start.wrapping_add(offset as usize), n);
        addr = addr.wrapping_add(copied);
        remains -= copied;
        if copied != n {
            spin_unlock(&raw mut (*vb).lock);
            return count - remains;
        }
        rs = find_next_bit(map, VMAP_BBMAP_BITS, re as ULong + 1) as u32;
    }
    spin_unlock(&raw mut (*vb).lock);
    count - remains + zero_iter(iter, remains)
}
#[no_mangle]
pub unsafe extern "C" fn vread_iter(
    iter: *mut iov_iter,
    addr: *const Char,
    mut count: usize,
) -> kernel::ffi::c_long {
    let mut addr = kasan_reset_tag(addr.cast()).cast::<Char>();
    if (addr as ULong).wrapping_add(count as ULong) < count as ULong {
        count = (addr as ULong).wrapping_neg() as usize;
    }
    let mut remains = count;
    let mut va = null_mut();
    let mut vn = find_vmap_area_exceed_addr_lock(addr as ULong, &mut va);
    if vn.is_null() {
        return zero_iter(iter, remains) as _;
    }
    if (addr as ULong).wrapping_add(remains as ULong) <= (*va).va_start {
        spin_unlock(&raw mut (*vn).busy.lock);
        return zero_iter(iter, remains) as _;
    }
    loop {
        if remains == 0 {
            spin_unlock(&raw mut (*vn).busy.lock);
            return (count - remains) as _;
        }
        let vm = (*va).__bindgen_anon_1.vm;
        let flags = (*va).flags & VMAP_FLAGS_MASK;
        warn(flags == VMAP_BLOCK);
        if !((vm.is_null() && flags == 0)
            || (!vm.is_null() && (*vm).flags & VM_UNINITIALIZED as ULong != 0))
        {
            smp_rmb();
            let vaddr = (*va).va_start;
            let size = if !vm.is_null() {
                if (*vm).flags & VM_ALLOC as ULong != 0 && (*vm).nr_pages != 0 {
                    (*vm).nr_pages << PAGE_SHIFT
                } else {
                    get_vm_area_size(vm)
                }
            } else {
                va_size(va)
            };
            if (addr as ULong) < vaddr.wrapping_add(size) {
                if (addr as ULong) < vaddr {
                    let to_zero = min(vaddr.wrapping_sub(addr as ULong) as usize, remains);
                    let zeroed = zero_iter(iter, to_zero);
                    addr = addr.wrapping_add(zeroed);
                    remains -= zeroed;
                    if remains == 0 || zeroed != to_zero {
                        spin_unlock(&raw mut (*vn).busy.lock);
                        return (count - remains) as _;
                    }
                }
                let n = min(
                    vaddr.wrapping_add(size).wrapping_sub(addr as ULong) as usize,
                    remains,
                );
                let copied = if flags & VMAP_RAM != 0 {
                    vmap_ram_vread_iter(iter, addr, n, flags)
                } else if !(!vm.is_null() && (*vm).flags & (VM_IOREMAP | VM_SPARSE) as ULong != 0) {
                    aligned_vread_iter(iter, addr, n)
                } else {
                    zero_iter(iter, n)
                };
                addr = addr.wrapping_add(copied);
                remains -= copied;
                if copied != n {
                    spin_unlock(&raw mut (*vn).busy.lock);
                    return (count - remains) as _;
                }
            }
        }
        let next = (*va).va_end;
        spin_unlock(&raw mut (*vn).busy.lock);
        vn = find_vmap_area_exceed_addr_lock(next, &mut va);
        if vn.is_null() {
            break;
        }
    }
    (count - remains + zero_iter(iter, remains)) as _
}
#[no_mangle]
pub unsafe extern "C" fn remap_vmalloc_range_partial(
    vma: *mut vm_area_struct,
    mut uaddr: ULong,
    mut kaddr: *mut Void,
    pgoff: ULong,
    mut size: ULong,
) -> i32 {
    if pgoff > ULong::MAX >> PAGE_SHIFT {
        return -(EINVAL as i32);
    }
    let off = pgoff << PAGE_SHIFT;
    size = align_up(size, PAGE_SIZE);
    if !is_aligned(uaddr, PAGE_SIZE) || !is_aligned(kaddr as ULong, PAGE_SIZE) {
        return -(EINVAL as i32);
    }
    let area = find_vm_area(kaddr);
    if area.is_null() || (*area).flags & (VM_USERMAP | VM_DMA_COHERENT) as ULong == 0 {
        return -(EINVAL as i32);
    }
    let (end_index, overflow) = size.overflowing_add(off);
    if overflow || end_index > get_vm_area_size(area) {
        return -(EINVAL as i32);
    }
    kaddr = kaddr.wrapping_byte_add(off as usize);
    loop {
        let ret = vm_insert_page(vma, uaddr, vmalloc_to_page(kaddr));
        if ret != 0 {
            return ret;
        }
        uaddr = uaddr.wrapping_add(PAGE_SIZE);
        kaddr = kaddr.wrapping_byte_add(PAGE_SIZE as usize);
        size = size.wrapping_sub(PAGE_SIZE);
        if size == 0 {
            break;
        }
    }
    vm_flags_set(vma, (VM_DONTEXPAND | VM_DONTDUMP) as vm_flags_t);
    0
}
#[no_mangle]
pub unsafe extern "C" fn remap_vmalloc_range(
    vma: *mut vm_area_struct,
    addr: *mut Void,
    pgoff: ULong,
) -> i32 {
    remap_vmalloc_range_partial(
        vma,
        (*vma).__bindgen_anon_1.__bindgen_anon_1.vm_start,
        addr,
        pgoff,
        (*vma)
            .__bindgen_anon_1
            .__bindgen_anon_1
            .vm_end
            .wrapping_sub((*vma).__bindgen_anon_1.__bindgen_anon_1.vm_start),
    )
}
#[no_mangle]
pub unsafe extern "C" fn free_vm_area(area: *mut vm_struct) {
    let ret = remove_vm_area((*area).addr);
    bug(ret != area);
    kfree(area.cast());
}
