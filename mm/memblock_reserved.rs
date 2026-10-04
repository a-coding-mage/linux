// SPDX-License-Identifier: GPL-2.0-or-later
// Included in the memblock owner. Named reservations are private Rust state;
// the native mutex bridges only locking, lockdep and static lock initialization.
const RESERVE_MEM_MAX_ENTRIES: usize = 8;
const RESERVE_MEM_NAME_SIZE: usize = 16;
#[repr(C)]
struct ReserveMem {
    name: [CChar; RESERVE_MEM_NAME_SIZE],
    start: Phys,
    size: Phys,
}
static mut reserved_mem_table: [ReserveMem; RESERVE_MEM_MAX_ENTRIES] = unsafe { zeroed() };
static mut reserved_mem_count: i32 = 0;
struct ReserveMemGuard;
impl ReserveMemGuard {
    unsafe fn lock() -> Self {
        rust_memblock_reserve_lock();
        Self
    }
}
impl Drop for ReserveMemGuard {
    fn drop(&mut self) {
        unsafe {
            rust_memblock_reserve_unlock();
        }
    }
}
#[inline]
unsafe fn reserve_entry(i: i32) -> *mut ReserveMem {
    (&raw mut reserved_mem_table)
        .cast::<ReserveMem>()
        .add(i as usize)
}
#[link_section = ".init.text"]
unsafe fn reserved_mem_add(start: Phys, size: Phys, name: *const CChar) {
    let entry = reserve_entry(reserved_mem_count);
    reserved_mem_count += 1;
    (*entry).start = start;
    (*entry).size = size;
    // Same bounded, NUL-terminated copy as strscpy(map->name, name).
    let dst = (&raw mut (*entry).name).cast::<CChar>();
    let mut i = 0;
    while i < RESERVE_MEM_NAME_SIZE - 1 && *name.add(i) != 0 {
        *dst.add(i) = *name.add(i);
        i += 1;
    }
    *dst.add(i) = 0;
}
unsafe fn reserve_mem_find_by_name_nolock(name: *const CChar) -> *mut ReserveMem {
    for i in 0..reserved_mem_count {
        let entry = reserve_entry(i);
        if (*entry).size != 0 && strcmp(name, (&raw const (*entry).name).cast()) == 0 {
            return entry;
        }
    }
    null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn reserve_mem_find_by_name(
    name: *const CChar,
    start: *mut Phys,
    size: *mut Phys,
) -> i32 {
    let _guard = ReserveMemGuard::lock();
    let entry = reserve_mem_find_by_name_nolock(name);
    if entry.is_null() {
        return 0;
    }
    *start = (*entry).start;
    *size = (*entry).size;
    1
}
#[no_mangle]
pub unsafe extern "C" fn reserve_mem_release_by_name(name: *const CChar) -> i32 {
    let _guard = ReserveMemGuard::lock();
    let entry = reserve_mem_find_by_name_nolock(name);
    if entry.is_null() {
        return 0;
    }
    let start = rust_memblock_phys_to_virt((*entry).start);
    let end = start
        .cast::<u8>()
        .wrapping_add((*entry).size as usize)
        .cast();
    let mut buf = [0 as CChar; RESERVE_MEM_NAME_SIZE + 12];
    snprintf(
        buf.as_mut_ptr(),
        buf.len(),
        c"reserve_mem:%s".as_ptr().cast::<CChar>(),
        name,
    );
    free_reserved_area(start, end, 0, buf.as_ptr());
    (*entry).size = 0;
    1
}

#[cfg(CONFIG_KEXEC_HANDOVER)]
#[link_section = ".init.text"]
unsafe fn reserved_mem_preserve() -> i32 {
    let mut nr_preserved = 0;
    while nr_preserved < reserved_mem_count {
        let entry = reserve_entry(nr_preserved);
        let page = rust_memblock_phys_to_page((*entry).start);
        // The original count is unsigned int, even on 64-bit architectures.
        let count = ((*entry).size >> PAGE_BITS) as u32;
        let err = kho_preserve_pages(page, count as ULong);
        if err != 0 {
            for i in 0..nr_preserved {
                let entry = reserve_entry(i);
                kho_unpreserve_pages(
                    rust_memblock_phys_to_page((*entry).start),
                    ((*entry).size >> PAGE_BITS) as u32 as ULong,
                );
            }
            return err;
        }
        nr_preserved += 1;
    }
    0
}
#[cfg(CONFIG_KEXEC_HANDOVER)]
#[link_section = ".init.text"]
unsafe fn prepare_kho_fdt() -> i32 {
    let page = rust_memblock_alloc_page();
    if page.is_null() {
        let err = -(ENOMEM as i32);
        info!(c"\x013failed to prepare memblock FDT for KHO: %d\n", err);
        return err;
    }
    let fdt = rust_memblock_page_to_virt(page);
    let mut err = kho_preserve_pages(page, 1);
    if err != 0 {
        rust_memblock_put_page(page);
        info!(c"\x013failed to prepare memblock FDT for KHO: %d\n", err);
        return err;
    }
    err |= fdt_create(fdt, PAGE_BYTES as i32);
    err |= fdt_finish_reservemap(fdt);
    err |= fdt_begin_node(fdt, c"".as_ptr().cast::<CChar>());
    err |= fdt_property(
        fdt,
        c"compatible".as_ptr().cast::<CChar>(),
        MEMBLOCK_KHO_NODE_COMPATIBLE.as_ptr().cast(),
        MEMBLOCK_KHO_NODE_COMPATIBLE.len() as i32,
    );
    let mut i = 0;
    while err == 0 && i < reserved_mem_count {
        let entry = reserve_entry(i);
        err |= fdt_begin_node(fdt, (&raw const (*entry).name).cast());
        err |= fdt_property(
            fdt,
            c"compatible".as_ptr().cast::<CChar>(),
            RESERVE_MEM_KHO_NODE_COMPATIBLE.as_ptr().cast(),
            RESERVE_MEM_KHO_NODE_COMPATIBLE.len() as i32,
        );
        // The KHO ABI stores these native phys_addr_t bytes, not FDT cells.
        err |= fdt_property(
            fdt,
            c"start".as_ptr().cast::<CChar>(),
            (&raw const (*entry).start).cast(),
            size_of::<Phys>() as i32,
        );
        err |= fdt_property(
            fdt,
            c"size".as_ptr().cast::<CChar>(),
            (&raw const (*entry).size).cast(),
            size_of::<Phys>() as i32,
        );
        err |= fdt_end_node(fdt);
        i += 1;
    }
    err |= fdt_end_node(fdt);
    err |= fdt_finish(fdt);
    if err == 0 {
        err = kho_add_subtree(
            MEMBLOCK_KHO_FDT.as_ptr().cast(),
            fdt,
            rust_memblock_fdt_totalsize(fdt) as usize,
        );
        if err == 0 {
            err = reserved_mem_preserve();
            if err == 0 {
                return 0;
            }
            kho_remove_subtree(fdt);
        }
    }
    kho_unpreserve_pages(page, 1);
    rust_memblock_put_page(page);
    info!(c"\x013failed to prepare memblock FDT for KHO: %d\n", err);
    err
}
#[cfg(CONFIG_KEXEC_HANDOVER)]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_memblock_reserve_mem_init() -> i32 {
    if !kho_is_enabled() || reserved_mem_count == 0 {
        0
    } else {
        prepare_kho_fdt()
    }
}
#[cfg(CONFIG_KEXEC_HANDOVER)]
static mut reserve_mem_kho_fdt: *mut Void = null_mut();
#[cfg(CONFIG_KEXEC_HANDOVER)]
#[link_section = ".init.text"]
unsafe fn reserve_mem_kho_retrieve_fdt() -> *const Void {
    if !reserve_mem_kho_fdt.is_null() {
        return reserve_mem_kho_fdt;
    }
    let mut fdt_phys = 0;
    let err = kho_retrieve_subtree(MEMBLOCK_KHO_FDT.as_ptr().cast(), &mut fdt_phys, null_mut());
    if err != 0 {
        if err != -(ENOENT as i32) {
            info!(
                c"\x014failed to retrieve FDT '%s' from KHO: %d\n",
                MEMBLOCK_KHO_FDT.as_ptr(),
                err
            );
        }
        return null();
    }
    reserve_mem_kho_fdt = rust_memblock_phys_to_virt(fdt_phys);
    let err = fdt_node_check_compatible(
        reserve_mem_kho_fdt,
        0,
        MEMBLOCK_KHO_NODE_COMPATIBLE.as_ptr().cast(),
    );
    if err != 0 {
        info!(
            c"\x014FDT '%s' is incompatible with '%s': %d\n",
            MEMBLOCK_KHO_FDT.as_ptr(),
            MEMBLOCK_KHO_NODE_COMPATIBLE.as_ptr(),
            err
        );
        reserve_mem_kho_fdt = null_mut();
    }
    reserve_mem_kho_fdt
}
#[link_section = ".init.text"]
unsafe fn reserve_mem_kho_revive(name: *const CChar, size: Phys, align: Phys) -> bool {
    #[cfg(CONFIG_KEXEC_HANDOVER)]
    {
        let fdt = reserve_mem_kho_retrieve_fdt();
        if fdt.is_null() {
            return false;
        }
        let offset = fdt_subnode_offset(fdt, 0, name);
        if offset < 0 {
            info!(
                c"\x014FDT '%s' has no child '%s': %d\n",
                MEMBLOCK_KHO_FDT.as_ptr(),
                name,
                offset
            );
            return false;
        }
        let err =
            fdt_node_check_compatible(fdt, offset, RESERVE_MEM_KHO_NODE_COMPATIBLE.as_ptr().cast());
        if err != 0 {
            info!(
                c"\x014Node '%s' is incompatible with '%s': %d\n",
                name,
                RESERVE_MEM_KHO_NODE_COMPATIBLE.as_ptr(),
                err
            );
            return false;
        }
        let (mut len_start, mut len_size) = (0, 0);
        let p_start = fdt_getprop(
            fdt,
            offset,
            c"start".as_ptr().cast::<CChar>(),
            &mut len_start,
        )
        .cast::<Phys>();
        let p_size = fdt_getprop(fdt, offset, c"size".as_ptr().cast::<CChar>(), &mut len_size)
            .cast::<Phys>();
        if p_start.is_null()
            || len_start != size_of::<Phys>() as i32
            || p_size.is_null()
            || len_size != size_of::<Phys>() as i32
        {
            return false;
        }
        // FDT properties have four-byte alignment, while phys_addr_t may need
        // eight; read_unaligned preserves the native-byte ABI safely in Rust.
        let start = p_start.read_unaligned();
        let old_size = p_size.read_unaligned();
        if start & align.wrapping_sub(1) != 0 {
            info!(
                c"\x014KHO reserve-mem '%s' has wrong alignment (0x%lx, 0x%lx)\n",
                name, align as ULong, start as ULong
            );
            return false;
        }
        if old_size != size {
            info!(
                c"\x014KHO reserve-mem '%s' has wrong size (0x%lx != 0x%lx)\n",
                name, old_size as ULong, size as ULong
            );
            return false;
        }
        reserved_mem_add(start, size, name);
        info!(c"\x016Revived memory reservation '%s' from KHO\n", name);
        true
    }
    #[cfg(not(CONFIG_KEXEC_HANDOVER))]
    {
        let _ = (name, size, align);
        false
    }
}
#[link_section = ".init.text"]
unsafe fn reserve_mem_bad_param() -> i32 {
    info!(c"\x013reserve_mem: empty or malformed parameter\n");
    -(EINVAL as i32)
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_memblock_reserve_mem(mut p: *mut CChar) -> i32 {
    if p.is_null() {
        return reserve_mem_bad_param();
    }
    if reserved_mem_count >= RESERVE_MEM_MAX_ENTRIES as i32 {
        info!(c"\x013reserve_mem: no more room for reserved memory\n");
        return -(EBUSY as i32);
    }
    let oldp = p;
    let size = memparse(p, &mut p) as Phys;
    if size == 0 || p == oldp || *p != b':' as CChar {
        return reserve_mem_bad_param();
    }
    let mut align = memparse(p.add(1), &mut p) as Phys;
    if *p != b':' as CChar {
        return reserve_mem_bad_param();
    }
    align = max(align, RUST_MEMBLOCK_SMP_CACHE_BYTES as Phys);
    let name = p.add(1);
    let len = strlen(name);
    if len == 0 || len >= RESERVE_MEM_NAME_SIZE {
        return reserve_mem_bad_param();
    }
    p = name;
    while *p != 0 && matches!(*p as u8, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c) {
        p = p.add(1);
    }
    if *p == 0 {
        return reserve_mem_bad_param();
    }
    let (mut start, mut tmp) = (0, 0);
    if reserve_mem_find_by_name(name, &mut start, &mut tmp) != 0 {
        info!(c"\x013reserve_mem: name \"%s\" was already used\n", name);
        return -(EBUSY as i32);
    }
    if reserve_mem_kho_revive(name, size, align) {
        return 1;
    }
    // Preserve upstream allocation policy, including its existing KHO scratch
    // limitation for reserve_mem= (HugeTLB has a separate overlap exclusion).
    start = memblock_phys_alloc_range(size, align, 0, ACCESSIBLE);
    if start == 0 {
        info!(c"\x013reserve_mem: memblock allocation failed\n");
        return -(ENOMEM as i32);
    }
    reserved_mem_add(start, size, name);
    1
}

#[cfg(all(CONFIG_DEBUG_FS, CONFIG_ARCH_KEEP_MEMBLOCK))]
#[no_mangle]
pub unsafe extern "C" fn rust_memblock_debug_show(m: *mut seq_file, _private: *mut Void) -> i32 {
    let ty = (*m).private.cast::<memblock_type>();
    // Match the original first-set-known-flag presentation, including UNKNOWN
    // for a region whose only bit is MEMBLOCK_RSRV_HUGETLB.
    let names = [
        c"HOTPLUG",
        c"MIRROR",
        c"NOMAP",
        c"DRV_MNG",
        c"RSV_NIT",
        c"RSV_KERN",
        c"KHO_SCRATCH",
    ];
    for i in 0..(*ty).cnt {
        let r = region(ty, i);
        let end = region_end(r).wrapping_sub(1);
        let nid = node(r);
        seq_printf(m, c"%4d: ".as_ptr().cast::<CChar>(), i as i32);
        seq_printf(
            m,
            c"%pa..%pa ".as_ptr().cast::<CChar>(),
            &raw const (*r).base,
            &end,
        );
        if valid_node(nid) {
            seq_printf(m, c"%4d ".as_ptr().cast::<CChar>(), nid);
        } else {
            seq_printf(m, c"%4c ".as_ptr().cast::<CChar>(), b'x' as i32);
        }
        let mut name = c"NONE";
        if (*r).flags != 0 {
            name = c"UNKNOWN";
            for (j, flag_name) in names.iter().enumerate() {
                if (*r).flags & (1 << j) != 0 {
                    name = flag_name;
                    break;
                }
            }
        }
        seq_printf(m, c"%s\n".as_ptr().cast::<CChar>(), name.as_ptr());
    }
    0
}
#[cfg(CONFIG_DEBUG_FS)]
#[no_mangle]
pub unsafe extern "C" fn rust_memblock_reserve_mem_show(
    m: *mut seq_file,
    _private: *mut Void,
) -> i32 {
    let _guard = ReserveMemGuard::lock();
    for i in 0..reserved_mem_count {
        let entry = reserve_entry(i);
        if (*entry).size == 0 {
            continue;
        }
        let mut txtsz = [0 as CChar; 16];
        string_get_size(
            (*entry).size as u64,
            1,
            STRING_UNITS_2,
            txtsz.as_mut_ptr(),
            txtsz.len() as i32,
        );
        seq_printf(
            m,
            c"%s\t\t(%s)\n".as_ptr().cast::<CChar>(),
            (&raw const (*entry).name).cast::<CChar>(),
            txtsz.as_ptr(),
        );
    }
    0
}
#[cfg(CONFIG_DEBUG_FS)]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_memblock_init_debugfs() -> i32 {
    if !cfg!(CONFIG_ARCH_KEEP_MEMBLOCK) && reserved_mem_count == 0 {
        return 0;
    }
    let root = debugfs_create_dir(c"memblock".as_ptr().cast::<CChar>(), null_mut());
    if reserved_mem_count != 0 {
        rust_memblock_debugfs_reservation_file(root);
    }
    #[cfg(CONFIG_ARCH_KEEP_MEMBLOCK)]
    {
        rust_memblock_debugfs_array_file(c"memory".as_ptr().cast::<CChar>(), root, memory());
        rust_memblock_debugfs_array_file(c"reserved".as_ptr().cast::<CChar>(), root, reserved());
        #[cfg(CONFIG_HAVE_MEMBLOCK_PHYS_MAP)]
        rust_memblock_debugfs_array_file(
            c"physmem".as_ptr().cast::<CChar>(),
            root,
            &raw mut physmem,
        );
    }
    0
}
