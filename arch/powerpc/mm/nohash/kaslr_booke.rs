// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2019 Jason Yan <yanaijie@huawei.com>

// Linux kernel dependencies and build-time configuration are supplied by the
// surrounding translation unit.

#[repr(C)]
pub struct regions {
    pub pa_start: c_ulong,
    pub pa_end: c_ulong,
    pub kernel_size: c_ulong,
    pub dtb_start: c_ulong,
    pub dtb_end: c_ulong,
    pub initrd_start: c_ulong,
    pub initrd_end: c_ulong,
    pub crash_start: c_ulong,
    pub crash_end: c_ulong,
    pub reserved_mem: c_int,
    pub reserved_mem_addr_cells: c_int,
    pub reserved_mem_size_cells: c_int,
}

#[no_mangle]
pub static mut regions: regions = regions {
    pa_start: 0,
    pa_end: 0,
    kernel_size: 0,
    dtb_start: 0,
    dtb_end: 0,
    initrd_start: 0,
    initrd_end: 0,
    crash_start: 0,
    crash_end: 0,
    reserved_mem: 0,
    reserved_mem_addr_cells: 0,
    reserved_mem_size_cells: 0,
};

unsafe fn kaslr_get_cmdline(fdt: *mut c_void) {
    early_init_dt_scan_chosen(boot_command_line);
}

unsafe fn rotate_xor(mut hash: c_ulong, area: *const c_void, size: usize) -> c_ulong {
    let ptr = area as *const c_ulong;
    let mut i = 0usize;
    while i < size / core::mem::size_of::<c_ulong>() {
        // Rotate by odd number of bits and XOR.
        hash = (hash << ((core::mem::size_of::<c_ulong>() * 8) - 7))
            | (hash >> 7);
        hash ^= *ptr.add(i);
        i += 1;
    }
    hash
}

// Attempt to create a simple starting entropy. This can make it defferent for
// every build but it is still not enough. Stronger entropy should
// be added to make it change for every boot.
unsafe fn get_boot_seed(fdt: *mut c_void) -> c_ulong {
    // build-specific string for starting entropy.
    let mut hash = 0;
    hash = rotate_xor(hash, linux_banner as *const c_void, strlen(linux_banner));
    hash = rotate_xor(hash, fdt, fdt_totalsize(fdt));
    hash
}

unsafe fn get_kaslr_seed(fdt: *mut c_void) -> u64 {
    let node = fdt_path_offset(fdt, b"/chosen\0".as_ptr() as *const c_char);
    if node < 0 { return 0; }
    let mut len = 0;
    let prop = fdt_getprop_w(fdt, node, b"kaslr-seed\0".as_ptr() as *const c_char, &mut len);
    if prop.is_null() || len != core::mem::size_of::<u64>() as c_int { return 0; }
    let ret = fdt64_to_cpu(*prop);
    *prop = 0;
    ret
}

unsafe fn regions_overlap(s1: u32, e1: u32, s2: u32, e2: u32) -> bool {
    e1 >= s2 && e2 >= s1
}

unsafe fn overlaps_reserved_region(fdt: *const c_void, start: u32, end: u32) -> bool {
    let mut base: u64 = 0;
    let mut size: u64 = 0;
    for i in 0..fdt_num_mem_rsv(fdt) {
        if fdt_get_mem_rsv(fdt, i, &mut base, &mut size) < 0 { continue; }
        if regions_overlap(start, end, base as u32, (base + size) as u32) { return true; }
    }
    if regions.reserved_mem < 0 { return false; }
    let mut subnode = fdt_first_subnode(fdt, regions.reserved_mem);
    while subnode >= 0 {
        let mut len = 0;
        let mut reg = fdt_getprop(fdt, subnode, b"reg\0".as_ptr() as *const c_char, &mut len);
        while len >= regions.reserved_mem_addr_cells + regions.reserved_mem_size_cells {
            base = fdt32_to_cpu(*reg) as u64;
            if regions.reserved_mem_addr_cells == 2 { base = (base << 32) | fdt32_to_cpu(*reg.add(1)) as u64; }
            reg = reg.add(regions.reserved_mem_addr_cells as usize);
            len -= 4 * regions.reserved_mem_addr_cells;
            size = fdt32_to_cpu(*reg) as u64;
            if regions.reserved_mem_size_cells == 2 { size = (size << 32) | fdt32_to_cpu(*reg.add(1)) as u64; }
            reg = reg.add(regions.reserved_mem_size_cells as usize);
            len -= 4 * regions.reserved_mem_size_cells;
            if base >= regions.pa_end as u64 { continue; }
            let rsv_end = core::cmp::min(base + size, u32::MAX as u64);
            if regions_overlap(start, end, base as u32, rsv_end as u32) { return true; }
        }
        subnode = fdt_next_subnode(fdt, subnode);
    }
    false
}

unsafe fn overlaps_region(fdt: *const c_void, start: u32, end: u32) -> bool {
    if regions_overlap(start, end, __pa(_stext) as u32, __pa(_end) as u32) { return true; }
    if regions_overlap(start, end, regions.dtb_start as u32, regions.dtb_end as u32) { return true; }
    if regions_overlap(start, end, regions.initrd_start as u32, regions.initrd_end as u32) { return true; }
    if regions_overlap(start, end, regions.crash_start as u32, regions.crash_end as u32) { return true; }
    overlaps_reserved_region(fdt, start, end)
}

unsafe fn get_crash_kernel(fdt: *mut c_void, size: c_ulong) {
    // CONFIG_CRASH_RESERVE conditional retained from the source.
    #[cfg(CONFIG_CRASH_RESERVE)]
    {
        let mut crash_size = 0u64;
        let mut crash_base = 0u64;
        let ret = parse_crashkernel(boot_command_line, size, &mut crash_size, &mut crash_base, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
        if ret != 0 || crash_size == 0 { return; }
        if crash_base == 0 { crash_base = KDUMP_KERNELBASE as u64; }
        regions.crash_start = crash_base as c_ulong;
        regions.crash_end = crash_base.wrapping_add(crash_size) as c_ulong;
        pr_debug!("crash_base=0x{:x} crash_size=0x{:x}\n", crash_base, crash_size);
    }
}

unsafe fn get_initrd_range(fdt: *mut c_void) {
    let node = fdt_path_offset(fdt, b"/chosen\0".as_ptr() as *const c_char);
    if node < 0 { return; }
    let mut len = 0;
    let mut prop = fdt_getprop(fdt, node, b"linux,initrd-start\0".as_ptr() as *const c_char, &mut len);
    if prop.is_null() { return; }
    let start = of_read_number(prop, (len / 4) as c_int);
    prop = fdt_getprop(fdt, node, b"linux,initrd-end\0".as_ptr() as *const c_char, &mut len);
    if prop.is_null() { return; }
    let end = of_read_number(prop, (len / 4) as c_int);
    regions.initrd_start = start as c_ulong;
    regions.initrd_end = end as c_ulong;
    pr_debug!("initrd_start=0x{:x}  initrd_end=0x{:x}\n", start, end);
}

unsafe fn get_usable_address(fdt: *const c_void, start: c_ulong, offset: c_ulong) -> c_ulong {
    let mut pa = offset;
    while (pa as c_long) > (start as c_long) {
        let pa_end = pa.wrapping_add(regions.kernel_size);
        if !overlaps_region(fdt, pa as u32, pa_end as u32) { return pa; }
        pa = pa.wrapping_sub(SZ_16K);
    }
    0
}

unsafe fn get_cell_sizes(fdt: *const c_void, node: c_int, addr_cells: *mut c_int, size_cells: *mut c_int) {
    *addr_cells = 1; *size_cells = 1;
    let mut len = 0;
    let prop = fdt_getprop(fdt, node, b"#address-cells\0".as_ptr() as *const c_char, &mut len) as *const c_int;
    if len == 4 { *addr_cells = fdt32_to_cpu(*prop) as c_int; }
    let prop = fdt_getprop(fdt, node, b"#size-cells\0".as_ptr() as *const c_char, &mut len) as *const c_int;
    if len == 4 { *size_cells = fdt32_to_cpu(*prop) as c_int; }
}

unsafe fn kaslr_legal_offset(dt_ptr: *mut c_void, mut index: c_ulong, mut offset: c_ulong) -> c_ulong {
    let mut koffset = 0;
    while (index as c_long) >= 0 {
        offset = memstart_addr + index * SZ_64M + offset;
        let start = memstart_addr + index * SZ_64M;
        koffset = get_usable_address(dt_ptr, start, offset);
        if koffset != 0 { break; }
        index = index.wrapping_sub(1);
    }
    if koffset != 0 { koffset -= memstart_addr; }
    koffset
}

unsafe fn kaslr_disabled() -> bool { !strstr(boot_command_line, b"nokaslr\0".as_ptr() as *const c_char).is_null() }

unsafe fn kaslr_choose_location(dt_ptr: *mut c_void, size: phys_addr_t, kernel_sz: c_ulong) -> c_ulong {
    let mut offset;
    let mut random;
    let mut seed;
    kaslr_get_cmdline(dt_ptr);
    if kaslr_disabled() { return 0; }
    random = get_boot_seed(dt_ptr);
    seed = get_tb() << 32;
    seed ^= get_tb();
    random = rotate_xor(random, &seed as *const _ as *const c_void, core::mem::size_of_val(&seed));
    seed = get_kaslr_seed(dt_ptr);
    if seed != 0 { random = rotate_xor(random, &seed as *const _ as *const c_void, core::mem::size_of_val(&seed)); }
    else { pr_warn!("KASLR: No safe seed for randomizing the kernel base.\n"); }
    let ram = map_mem_in_cams(core::cmp::min(__max_low_memory, size), CONFIG_LOWMEM_CAM_NUM, true, true);
    let linear_sz = core::cmp::min(ram, SZ_512M);
    if linear_sz < SZ_64M { return 0; }
    regions.reserved_mem = fdt_path_offset(dt_ptr, b"/reserved-memory\0".as_ptr() as *const c_char);
    if regions.reserved_mem >= 0 { get_cell_sizes(dt_ptr, regions.reserved_mem, &mut regions.reserved_mem_addr_cells, &mut regions.reserved_mem_size_cells); }
    regions.pa_start = memstart_addr; regions.pa_end = memstart_addr + linear_sz;
    regions.dtb_start = __pa(dt_ptr) as c_ulong; regions.dtb_end = regions.dtb_start + fdt_totalsize(dt_ptr) as c_ulong;
    regions.kernel_size = kernel_sz;
    get_initrd_range(dt_ptr); get_crash_kernel(dt_ptr, ram);
    let index = (random & 0xff) % (linear_sz / SZ_64M);
    offset = round_down(random % (SZ_64M - kernel_sz), SZ_16K);
    kaslr_legal_offset(dt_ptr, index, offset)
}

pub unsafe fn kaslr_early_init(dt_ptr: *mut c_void, size: phys_addr_t) {
    let kernel_sz = (_end as c_ulong).wrapping_sub(_stext as c_ulong);
    let offset = kaslr_choose_location(dt_ptr, size, kernel_sz);
    if offset == 0 { return; }
    kernstart_virt_addr += offset; kernstart_addr += offset; is_second_reloc = 1;
    if offset >= SZ_64M {
        create_kaslr_tlb_entry(1, round_down(kernstart_virt_addr, SZ_64M), round_down(kernstart_addr, SZ_64M));
    }
    memcpy(kernstart_virt_addr as *mut c_void, _stext as *const c_void, kernel_sz as usize);
    flush_icache_range(kernstart_virt_addr, kernstart_virt_addr + kernel_sz);
    reloc_kernel_entry(dt_ptr, kernstart_virt_addr);
}

pub unsafe fn kaslr_late_init() {
    if kernstart_virt_addr != KERNELBASE {
        let kernel_sz = (_end as c_ulong).wrapping_sub(kernstart_virt_addr);
        memzero_explicit(KERNELBASE as *mut c_void, kernel_sz as usize);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
