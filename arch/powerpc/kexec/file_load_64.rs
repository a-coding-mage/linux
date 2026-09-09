// SPDX-License-Identifier: GPL-2.0-only
/*
 * ppc64 code to implement the kexec_file_load syscall
 *
 * Copyright (C) 2004  Adam Litke (agl@us.ibm.com)
 * Copyright (C) 2004  IBM Corp.
 * Copyright (C) 2004,2005  Milton D Miller II, IBM Corporation
 * Copyright (C) 2005  R Sharada (sharada@in.ibm.com)
 * Copyright (C) 2006  Mohan Kumar M (mohan@in.ibm.com)
 * Copyright (C) 2020  IBM Corporation
 *
 * Based on kexec-tools' kexec-ppc64.c, kexec-elf-rel-ppc64.c, fs2dt.c.
 * Heavily modified for the kernel by
 * Hari Bathini, IBM Corporation.
 */

// External Linux, device-tree, and architecture declarations are supplied by
// the surrounding kernel translation unit.

#[repr(C)]
pub struct umem_info {
    pub buf: *mut __be64,
    pub size: u32,
    pub max_entries: u32,
    pub idx: u32,
    pub nr_ranges: c_uint,
    pub ranges: *const range,
}

pub static kexec_file_loaders: [*const kexec_file_ops; 2] = [
    &kexec_elf64_ops,
    core::ptr::null(),
];

pub unsafe fn arch_check_excluded_range(image: *mut kimage, start: c_ulong, end: c_ulong) -> c_int {
    let emem = (*image).arch.exclude_ranges;
    let mut i = 0;
    while i < (*emem).nr_ranges {
        if start < (*emem).ranges.add(i as usize).end && end > (*emem).ranges.add(i as usize).start {
            return 1;
        }
        i += 1;
    }
    0
}

#[cfg(CONFIG_CRASH_DUMP)]
unsafe fn check_realloc_usable_mem(um_info: *mut umem_info, cnt: c_int) -> *mut __be64 {
    if ((*um_info).idx + cnt as u32) <= (*um_info).max_entries { return (*um_info).buf; }
    let new_size = (*um_info).size + MEM_RANGE_CHUNK_SZ;
    let tbuf = krealloc((*um_info).buf as *mut c_void, new_size as usize, GFP_KERNEL) as *mut __be64;
    if !tbuf.is_null() {
        (*um_info).buf = tbuf;
        (*um_info).size = new_size;
        (*um_info).max_entries = (*um_info).size / core::mem::size_of::<u64>() as u32;
    }
    tbuf
}

#[cfg(CONFIG_CRASH_DUMP)]
unsafe fn add_usable_mem(um_info: *mut umem_info, base: u64, end: u64) -> c_int {
    let mut i = 0;
    while i < (*um_info).nr_ranges {
        let r = (*um_info).ranges.add(i as usize);
        let mut loc_base = (*r).start;
        let mut loc_end = (*r).end;
        let mut add = false;
        if loc_base >= base && loc_end <= end { add = true; }
        else if base < loc_end && end > loc_base {
            if loc_base < base { loc_base = base; }
            if loc_end > end { loc_end = end; }
            add = true;
        }
        if add {
            if check_realloc_usable_mem(um_info, 2).is_null() { return -ENOMEM; }
            *(*um_info).buf.add((*um_info).idx as usize) = cpu_to_be64(loc_base);
            (*um_info).idx += 1;
            *(*um_info).buf.add((*um_info).idx as usize) = cpu_to_be64(loc_end - loc_base + 1);
            (*um_info).idx += 1;
        }
        i += 1;
    }
    0
}

#[cfg(CONFIG_CRASH_DUMP)]
unsafe fn kdump_setup_usable_lmb(lmb: *mut drmem_lmb, usm: *const *const __be32, data: *mut c_void) -> c_int {
    if !(*usm).is_null() { pr_err!("linux,drconf-usable-memory property already exists!"); return -EINVAL; }
    let um_info = data as *mut umem_info;
    let tmp_idx = (*um_info).idx;
    if check_realloc_usable_mem(um_info, 1).is_null() { return -ENOMEM; }
    (*um_info).idx += 1;
    let base = (*lmb).base_addr;
    let end = base + drmem_lmb_size() - 1;
    let ret = add_usable_mem(um_info, base, end);
    if ret == 0 { *(*um_info).buf.add(tmp_idx as usize) = cpu_to_be64(((*um_info).idx - tmp_idx - 1) as u64 / 2); }
    ret
}

#[cfg(CONFIG_CRASH_DUMP)]
unsafe fn add_usable_mem_property(fdt: *mut c_void, dn: *mut device_node, um_info: *mut umem_info) -> c_int {
    let mut path = [0i8; NODE_PATH_LEN];
    of_node_get(dn);
    if snprintf(path.as_mut_ptr(), NODE_PATH_LEN, c"%pOF".as_ptr(), dn) > NODE_PATH_LEN - 1 { return -EOVERFLOW; }
    kexec_dprintk!("Memory node path: %s\n", path.as_ptr());
    let node = fdt_path_offset(fdt, path.as_ptr());
    if node < 0 { of_node_put(dn); return -EINVAL; }
    (*um_info).idx = 0;
    if check_realloc_usable_mem(um_info, 2).is_null() { of_node_put(dn); return -ENOMEM; }
    let mut i = 0;
    let mut ret;
    loop {
        let mut base = 0u64; let mut size = 0u64;
        ret = of_property_read_reg(dn, i, &mut base, &mut size);
        if ret != 0 { break; }
        ret = add_usable_mem(um_info, base, base + size - 1);
        if ret != 0 { of_node_put(dn); return ret; }
        i += 1;
    }
    if i == 0 { of_node_put(dn); return ret; }
    if (*um_info).idx == 0 { *(*um_info).buf = 0; *(*um_info).buf.add(1) = 0; (*um_info).idx = 2; }
    ret = fdt_setprop(fdt, node, c"linux,usable-memory".as_ptr(), (*um_info).buf as *const c_void, (*um_info).idx as usize * 8);
    of_node_put(dn); ret
}

pub unsafe fn setup_purgatory_ppc64(image: *mut kimage, slave_code: *const c_void, fdt: *const c_void, kernel_load_addr: c_ulong, fdt_load_addr: c_ulong) -> c_int {
    let mut dn = core::ptr::null_mut();
    let mut ret = setup_purgatory(image, slave_code, fdt, kernel_load_addr, fdt_load_addr);
    if ret != 0 { return ret; }
    if (*image).type_ == KEXEC_TYPE_CRASH {
        let mut my_run_at_load = 1u32;
        ret = kexec_purgatory_get_set_symbol(image, c"run_at_load".as_ptr(), &mut my_run_at_load as *mut _ as *mut c_void, 4, false);
        if ret != 0 { return ret; }
    }
    ret = kexec_purgatory_get_set_symbol(image, c"backup_start".as_ptr(), &(*image).arch.backup_start as *const _ as *mut c_void, core::mem::size_of_val(&(*image).arch.backup_start), false);
    if ret != 0 { return ret; }
    dn = of_find_node_by_path(c"/ibm,opal".as_ptr());
    if !dn.is_null() {
        let mut val = 0u64;
        ret = of_property_read_u64(dn, c"opal-base-address".as_ptr(), &mut val);
        if ret == 0 { ret = kexec_purgatory_get_set_symbol(image, c"opal_base".as_ptr(), &mut val as *mut _ as *mut c_void, 8, false); }
        if ret == 0 { ret = of_property_read_u64(dn, c"opal-entry-address".as_ptr(), &mut val); }
        if ret == 0 { ret = kexec_purgatory_get_set_symbol(image, c"opal_entry".as_ptr(), &mut val as *mut _ as *mut c_void, 8, false); }
    }
    of_node_put(dn); ret
}

pub unsafe fn arch_kexec_kernel_image_probe(image: *mut kimage, buf: *mut c_void, buf_len: c_ulong) -> c_int {
    let ret = get_exclude_memory_ranges(&mut (*image).arch.exclude_ranges);
    if ret != 0 { return ret; }
    kexec_image_probe_default(image, buf, buf_len)
}

pub unsafe fn arch_kimage_file_post_load_cleanup(image: *mut kimage) -> c_int {
    kfree((*image).arch.exclude_ranges as *mut c_void); (*image).arch.exclude_ranges = core::ptr::null_mut();
    vfree((*image).arch.backup_buf); (*image).arch.backup_buf = core::ptr::null_mut();
    vfree((*image).elf_headers); (*image).elf_headers = core::ptr::null_mut(); (*image).elf_headers_sz = 0;
    kvfree((*image).arch.fdt); (*image).arch.fdt = core::ptr::null_mut();
    kexec_image_post_load_cleanup_default(image)
}

#[cfg(CONFIG_CRASH_DUMP)]
unsafe fn load_backup_segment(image: *mut kimage, kbuf: *mut kexec_buf) -> c_int {
    let buf = vzalloc(BACKUP_SRC_SIZE as usize);
    if buf.is_null() { return -ENOMEM; }
    (*kbuf).buffer = buf; (*kbuf).mem = KEXEC_BUF_MEM_UNKNOWN;
    (*kbuf).bufsz = BACKUP_SRC_SIZE; (*kbuf).memsz = BACKUP_SRC_SIZE; (*kbuf).top_down = false;
    let ret = kexec_add_buffer(kbuf);
    if ret != 0 { vfree(buf); return ret; }
    (*image).arch.backup_buf = buf; (*image).arch.backup_start = (*kbuf).mem; 0
}

#[cfg(CONFIG_CRASH_DUMP)]
unsafe fn kdump_extra_elfcorehdr_size(cmem: *mut crash_mem) -> c_uint {
    #[cfg(all(CONFIG_CRASH_HOTPLUG, CONFIG_MEMORY_HOTPLUG))]
    {
        if CONFIG_CRASH_MAX_MEMORY_RANGES > PN_XNUM as c_uint || (*cmem).nr_ranges >= CONFIG_CRASH_MAX_MEMORY_RANGES { return 0; }
        return (CONFIG_CRASH_MAX_MEMORY_RANGES - (*cmem).nr_ranges) * core::mem::size_of::<Elf64_Phdr>() as c_uint;
    }
    0
}

#[cfg(CONFIG_CRASH_DUMP)]
unsafe fn load_elfcorehdr_segment(image: *mut kimage, kbuf: *mut kexec_buf) -> c_int {
    let mut cmem = core::ptr::null_mut(); let mut headers_sz = 0usize; let mut headers = core::ptr::null_mut();
    let mut ret = get_crash_memory_ranges(&mut cmem);
    if ret == 0 { ret = crash_prepare_elf64_headers(cmem, false, &mut headers, &mut headers_sz); }
    if ret != 0 { kfree(cmem as *mut c_void); return ret; }
    sync_backup_region_phdr(image, headers, false);
    (*kbuf).buffer = headers; (*kbuf).mem = KEXEC_BUF_MEM_UNKNOWN; (*kbuf).bufsz = headers_sz;
    (*kbuf).memsz = headers_sz + kdump_extra_elfcorehdr_size(cmem) as usize; (*kbuf).top_down = false;
    ret = kexec_add_buffer(kbuf);
    if ret != 0 { vfree(headers); } else { (*image).elf_load_addr = (*kbuf).mem; (*image).elf_headers_sz = (*kbuf).memsz; (*image).elf_headers = headers; }
    kfree(cmem as *mut c_void); ret
}

#[cfg(CONFIG_CRASH_DUMP)]
pub unsafe fn load_crashdump_segments_ppc64(image: *mut kimage, kbuf: *mut kexec_buf) -> c_int {
    let mut ret = load_backup_segment(image, kbuf); if ret != 0 { return ret; }
    ret = load_elfcorehdr_segment(image, kbuf); ret
}

pub unsafe fn kexec_extra_fdt_size_ppc64(image: *mut kimage, rmem: *mut crash_mem) -> c_uint {
    let mut extra_size = if plpks_is_available() { plpks_get_passwordlen() as c_uint } else { 0 };
    let mut cpu_nodes = 0;
    for_each_node_by_type!(_dn, "cpu") { cpu_nodes += 1; }
    if cpu_nodes > boot_cpu_node_count { extra_size += (cpu_nodes - boot_cpu_node_count) * cpu_node_size(); }
    if !rmem.is_null() && (*rmem).nr_ranges > 0 { extra_size += core::mem::size_of::<fdt_reserve_entry>() as c_uint * (*rmem).nr_ranges; }
    extra_size + kdump_extra_fdt_size_ppc64(image, cpu_nodes)
}

pub unsafe fn setup_new_fdt_ppc64(image: *const kimage, fdt: *mut c_void, rmem: *mut crash_mem) -> c_int {
    let mut ret = update_cpus_node(fdt);
    if ret < 0 { return ret; }
    ret = update_pci_dma_nodes(fdt, DIRECT64_PROPNAME); if ret < 0 { return ret; }
    ret = update_pci_dma_nodes(fdt, DMA64_PROPNAME); if ret < 0 { return ret; }
    if !rmem.is_null() { for i in 0..(*rmem).nr_ranges { let r = (*rmem).ranges.add(i as usize); ret = fdt_add_mem_rsv(fdt, (*r).start, (*r).end - (*r).start + 1); if ret != 0 { return ret; } } }
    if plpks_is_available() { ret = plpks_populate_fdt(fdt); }
    ret
}

unsafe fn cpu_node_size() -> c_uint {
    static mut SIZE: c_uint = 0;
    if SIZE != 0 { return SIZE; }
    let dn = of_find_node_by_type(core::ptr::null_mut(), c"cpu".as_ptr());
    if dn.is_null() { return 0; }
    SIZE += strlen((*dn).name) as c_uint + 5;
    let mut pp = core::ptr::null_mut();
    for_each_property_of_node!(dn, pp) { SIZE += strlen((*pp).name) as c_uint; SIZE += (*pp).length; }
    of_node_put(dn); SIZE
}

unsafe fn kdump_extra_fdt_size_ppc64(image: *mut kimage, cpu_nodes: c_uint) -> c_uint {
    if !IS_ENABLED!(CONFIG_CRASH_DUMP) || (*image).type_ != KEXEC_TYPE_CRASH { return 0; }
    let mut extra = 0u32;
    let lmb = drmem_lmb_size();
    if lmb != 0 { extra += ((memory_hotplug_max() / lmb) + 2 * (resource_size(&crashk_res) / lmb)) as c_uint * 8; }
    #[cfg(CONFIG_CRASH_HOTPLUG)]
    { let possible = num_possible_cpus() / threads_per_core; if possible > cpu_nodes { extra += (possible - cpu_nodes) * cpu_node_size(); } }
    extra
}

unsafe fn copy_property(fdt: *mut c_void, node_offset: c_int, dn: *const device_node, propname: *const c_char) -> c_int {
    let mut len = 0; let mut fdtlen = 0;
    let prop = of_get_property(dn, propname, &mut len); let fdtprop = fdt_getprop(fdt, node_offset, propname, &mut fdtlen);
    if !fdtprop.is_null() && prop.is_null() { fdt_delprop(fdt, node_offset, propname) }
    else if !prop.is_null() { fdt_setprop(fdt, node_offset, propname, prop, len as usize) }
    else { -FDT_ERR_NOTFOUND }
}

unsafe fn update_pci_dma_nodes(fdt: *mut c_void, dmapropname: *const c_char) -> c_int {
    if !firmware_has_feature(FW_FEATURE_LPAR) { return 0; }
    let root = fdt_path_offset(fdt, c"/".as_ptr()); let mut ret = 0;
    for_each_node_with_property!(_dn, dmapropname) {
        let off = fdt_subnode_offset(fdt, root, of_node_full_name(_dn)); if off < 0 { continue; }
        ret = copy_property(fdt, off, _dn, c"ibm,dma-window".as_ptr()); if ret < 0 { break; }
        ret = copy_property(fdt, off, _dn, dmapropname); if ret < 0 { break; }
    }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
