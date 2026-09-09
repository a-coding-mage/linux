// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Firmware-Assisted Dump support on POWER platform (OPAL).
 *
 * Copyright 2019, Hari Bathini, IBM Corporation.
 */

// Kernel dependencies and build-time configuration are supplied by the surrounding tree.

#[cfg(CONFIG_PRESERVE_FA_DUMP)]
pub unsafe extern "C" fn opal_fadump_dt_scan(fadump_conf: *mut fw_dump, node: u64) {
    let mut addr: u64 = 0;
    let mut prop: *const __be32;
    let dn = of_get_flat_dt_subnode_by_name(node, c"dump".as_ptr());
    if dn == (-FDT_ERR_NOTFOUND as u64) { return; }
    prop = of_get_flat_dt_prop(dn, c"mpipl-boot".as_ptr(), core::ptr::null_mut());
    if prop.is_null() { return; }
    let ret = opal_mpipl_query_tag(OPAL_MPIPL_TAG_KERNEL, &mut addr);
    if ret != OPAL_SUCCESS || addr == 0 {
        pr_debug!("Could not get Kernel metadata ({})\n", ret);
        return;
    }
    addr = be64_to_cpu(addr);
    pr_debug!("Kernel metadata addr: {:x}\n", addr);
    let opal_fdm_active = addr as *const opal_fadump_mem_struct;
    if be16_to_cpu((*opal_fdm_active).registered_regions) == 0 { return; }
    let ret = opal_mpipl_query_tag(OPAL_MPIPL_TAG_BOOT_MEM, &mut addr);
    if ret != OPAL_SUCCESS || addr == 0 {
        pr_err!("Failed to get boot memory tag ({})\n", ret);
        return;
    }
    (*fadump_conf).boot_mem_top = be64_to_cpu(addr);
    pr_debug!("Preserve everything above {:x}\n", (*fadump_conf).boot_mem_top);
    pr_info!("Firmware-assisted dump is active.\n");
    (*fadump_conf).dump_active = 1;
}

#[cfg(not(CONFIG_PRESERVE_FA_DUMP))]
static mut opal_fdm_active: *const opal_fadump_mem_struct = core::ptr::null();
#[cfg(not(CONFIG_PRESERVE_FA_DUMP))]
static mut opal_cpu_metadata: *const opal_mpipl_fadump = core::ptr::null();
#[cfg(not(CONFIG_PRESERVE_FA_DUMP))]
static mut opal_fdm: *mut opal_fadump_mem_struct = core::ptr::null_mut();

#[cfg(all(not(CONFIG_PRESERVE_FA_DUMP), CONFIG_OPAL_CORE))]
extern "C" { static mut kernel_initiated: bool; }

#[cfg(not(CONFIG_PRESERVE_FA_DUMP))]
unsafe fn opal_fadump_update_config(fadump_conf: *mut fw_dump, fdm: *const opal_fadump_mem_struct) {
    pr_debug!("Boot memory regions count: {}\n", be16_to_cpu((*fdm).region_cnt));
    (*fadump_conf).boot_mem_dest_addr = be64_to_cpu((*fdm).rgn[0].dest);
    pr_debug!("Destination address of boot memory regions: {:#016x}\n", (*fadump_conf).boot_mem_dest_addr);
    (*fadump_conf).fadumphdr_addr = be64_to_cpu((*fdm).fadumphdr_addr);
}

#[cfg(not(CONFIG_PRESERVE_FA_DUMP))]
unsafe fn opal_fadump_get_config(fadump_conf: *mut fw_dump, fdm: *const opal_fadump_mem_struct) {
    if (*fadump_conf).dump_active == 0 { return; }
    let mut last_end: usize = 0;
    let mut hole_size: usize = 0;
    (*fadump_conf).boot_memory_size = 0;
    pr_debug!("Boot memory regions:\n");
    let mut i = 0;
    while i < be16_to_cpu((*fdm).region_cnt) as usize {
        let base = be64_to_cpu((*fdm).rgn[i].src) as usize;
        let size = be64_to_cpu((*fdm).rgn[i].size) as usize;
        pr_debug!("\t[{:#03}] base: 0x{:x}, size: 0x{:x}\n", i, base, size);
        (*fadump_conf).boot_mem_addr[i] = base as u64;
        (*fadump_conf).boot_mem_sz[i] = size as u64;
        (*fadump_conf).boot_memory_size += size as u64;
        hole_size += base.wrapping_sub(last_end);
        last_end = base.wrapping_add(size);
        i += 1;
    }
    (*fadump_conf).reserve_dump_area_start = be64_to_cpu((*fdm).rgn[0].dest);
    if be16_to_cpu((*fdm).region_cnt) > be16_to_cpu((*fdm).registered_regions) {
        pr_warn!("Not all memory regions were saved!!!\n");
        pr_warn!("  Unsaved memory regions:\n");
        i = be16_to_cpu((*fdm).registered_regions) as usize;
        while i < be16_to_cpu((*fdm).region_cnt) as usize {
            pr_warn!("\t[{:#03}] base: 0x{:x}, size: 0x{:x}\n", i, be64_to_cpu((*fdm).rgn[i].src), be64_to_cpu((*fdm).rgn[i].size));
            i += 1;
        }
        pr_warn!("If the unsaved regions only contain pages that are filtered out (eg. free/user pages), the vmcore should still be usable.\n");
        pr_warn!("WARNING: If the unsaved regions contain kernel pages, the vmcore will be corrupted.\n");
    }
    (*fadump_conf).boot_mem_top = (*fadump_conf).boot_memory_size + hole_size as u64;
    (*fadump_conf).boot_mem_regs_cnt = be16_to_cpu((*fdm).region_cnt);
    opal_fadump_update_config(fadump_conf, fdm);
}

#[cfg(not(CONFIG_PRESERVE_FA_DUMP))]
unsafe fn opal_fadump_init_metadata(fdm: *mut opal_fadump_mem_struct) {
    (*fdm).version = OPAL_FADUMP_VERSION;
    (*fdm).region_cnt = cpu_to_be16(0);
    (*fdm).registered_regions = cpu_to_be16(0);
    (*fdm).fadumphdr_addr = cpu_to_be64(0);
}

#[cfg(not(CONFIG_PRESERVE_FA_DUMP))]
unsafe fn opal_fadump_init_mem_struct(fadump_conf: *mut fw_dump) -> u64 {
    let mut addr = (*fadump_conf).reserve_dump_area_start;
    opal_fdm = __va((*fadump_conf).kernel_metadata);
    opal_fadump_init_metadata(opal_fdm);
    let mut reg_cnt = be16_to_cpu((*opal_fdm).region_cnt);
    let mut i = 0;
    while i < (*fadump_conf).boot_mem_regs_cnt as usize {
        (*opal_fdm).rgn[i].src = cpu_to_be64((*fadump_conf).boot_mem_addr[i]);
        (*opal_fdm).rgn[i].dest = cpu_to_be64(addr);
        (*opal_fdm).rgn[i].size = cpu_to_be64((*fadump_conf).boot_mem_sz[i]);
        reg_cnt += 1;
        addr += (*fadump_conf).boot_mem_sz[i];
        i += 1;
    }
    (*opal_fdm).region_cnt = cpu_to_be16(reg_cnt);
    (*opal_fdm).fadumphdr_addr = cpu_to_be64(be64_to_cpu((*opal_fdm).rgn[0].dest) + (*fadump_conf).boot_memory_size);
    opal_fadump_update_config(fadump_conf, opal_fdm);
    addr
}

#[cfg(not(CONFIG_PRESERVE_FA_DUMP))]
unsafe fn opal_fadump_get_metadata_size() -> u64 { PAGE_ALIGN(core::mem::size_of::<opal_fadump_mem_struct>() as u64) }

#[cfg(not(CONFIG_PRESERVE_FA_DUMP))]
unsafe fn opal_fadump_setup_metadata(fadump_conf: *mut fw_dump) -> i32 {
    (*fadump_conf).kernel_metadata = (*fadump_conf).reserve_dump_area_start + (*fadump_conf).reserve_dump_area_size - opal_fadump_get_metadata_size();
    pr_info!("Kernel metadata addr: {:x}\n", (*fadump_conf).kernel_metadata);
    opal_fdm = __va((*fadump_conf).kernel_metadata);
    opal_fadump_init_metadata(opal_fdm);
    let mut err = 0;
    let mut ret = opal_mpipl_register_tag(OPAL_MPIPL_TAG_KERNEL, (*fadump_conf).kernel_metadata);
    if ret != OPAL_SUCCESS { pr_err!("Failed to set kernel metadata tag!\n"); err = -EPERM; }
    ret = opal_mpipl_register_tag(OPAL_MPIPL_TAG_BOOT_MEM, (*fadump_conf).boot_mem_top);
    if ret != OPAL_SUCCESS { pr_err!("Failed to set boot memory tag!\n"); err = -EPERM; }
    err
}

#[cfg(not(CONFIG_PRESERVE_FA_DUMP))]
unsafe fn opal_fadump_get_bootmem_min() -> u64 { OPAL_FADUMP_MIN_BOOT_MEM }

#[cfg(not(CONFIG_PRESERVE_FA_DUMP))]
unsafe fn opal_fadump_register(fadump_conf: *mut fw_dump) -> i32 {
    let mut rc = OPAL_PARAMETER;
    let mut registered_regs = be16_to_cpu((*opal_fdm).registered_regions);
    let mut i = 0;
    while i < be16_to_cpu((*opal_fdm).region_cnt) as usize {
        rc = opal_mpipl_update(OPAL_MPIPL_ADD_RANGE, be64_to_cpu((*opal_fdm).rgn[i].src), be64_to_cpu((*opal_fdm).rgn[i].dest), be64_to_cpu((*opal_fdm).rgn[i].size));
        if rc != OPAL_SUCCESS { break; }
        registered_regs += 1; i += 1;
    }
    (*opal_fdm).registered_regions = cpu_to_be16(registered_regs);
    let mut err = -EIO;
    match rc {
        OPAL_SUCCESS => { pr_info!("Registration is successful!\n"); (*fadump_conf).dump_registered = 1; err = 0; }
        OPAL_RESOURCE => { pr_warn!("{} regions could not be registered for MPIPL as MAX limit is reached!\n", be16_to_cpu((*opal_fdm).region_cnt) - be16_to_cpu((*opal_fdm).registered_regions)); (*fadump_conf).dump_registered = 1; err = 0; }
        OPAL_PARAMETER => pr_err!("Failed to register. Parameter Error({}).\n", rc),
        OPAL_HARDWARE => { pr_err!("Support not available.\n"); (*fadump_conf).fadump_supported = 0; (*fadump_conf).fadump_enabled = 0; },
        _ => pr_err!("Failed to register. Unknown Error({}).\n", rc),
    }
    if err < 0 && be16_to_cpu((*opal_fdm).registered_regions) > 0 { opal_fadump_unregister(fadump_conf); }
    err
}

#[cfg(not(CONFIG_PRESERVE_FA_DUMP))]
unsafe fn opal_fadump_unregister(fadump_conf: *mut fw_dump) -> i32 {
    let rc = opal_mpipl_update(OPAL_MPIPL_REMOVE_ALL, 0, 0, 0);
    if rc != 0 { pr_err!("Failed to un-register - unexpected Error({}).\n", rc); return -EIO; }
    (*opal_fdm).registered_regions = cpu_to_be16(0); (*fadump_conf).dump_registered = 0; 0
}

#[cfg(not(CONFIG_PRESERVE_FA_DUMP))]
unsafe fn opal_fadump_invalidate(fadump_conf: *mut fw_dump) -> i32 {
    let rc = opal_mpipl_update(OPAL_MPIPL_FREE_PRESERVED_MEMORY, 0, 0, 0);
    if rc != 0 { pr_err!("Failed to invalidate - unexpected Error({}).\n", rc); return -EIO; }
    (*fadump_conf).dump_active = 0; opal_fdm_active = core::ptr::null(); 0
}

#[cfg(not(CONFIG_PRESERVE_FA_DUMP))]
unsafe fn opal_fadump_cleanup(_fadump_conf: *mut fw_dump) {
    let ret = opal_mpipl_register_tag(OPAL_MPIPL_TAG_KERNEL, 0);
    if ret != OPAL_SUCCESS { pr_warn!("Could not reset ({}) kernel metadata tag!\n", ret); }
}

// The remaining CPU-note processing and device-tree registration retain the C ABI and external data layouts.
// External structures/functions are intentionally referenced rather than reimplemented.
#[cfg(not(CONFIG_PRESERVE_FA_DUMP))]
unsafe fn opal_fadump_process(fadump_conf: *mut fw_dump) -> i32 {
    if opal_fdm_active.is_null() || (*fadump_conf).fadumphdr_addr == 0 { return -EINVAL; }
    let fdh = __va((*fadump_conf).fadumphdr_addr) as *mut fadump_crash_info_header;
    #[cfg(CONFIG_OPAL_CORE)]
    if (*fdh).crashing_cpu != FADUMP_CPU_UNKNOWN { kernel_initiated = true; }
    opal_fadump_build_cpu_notes(fadump_conf, fdh)
}

#[cfg(not(CONFIG_PRESERVE_FA_DUMP))]
unsafe fn opal_fadump_build_cpu_notes(fadump_conf: *mut fw_dump, fdh: *mut fadump_crash_info_header) -> i32 {
    let mut num_cpus = 1u32;
    let mut valid = false;
    if !opal_cpu_metadata.is_null() {
        (*fadump_conf).cpu_state_data_version = be32_to_cpu((*opal_cpu_metadata).cpu_data_version);
        (*fadump_conf).cpu_state_entry_size = be32_to_cpu((*opal_cpu_metadata).cpu_data_size);
        (*fadump_conf).cpu_state_dest_vaddr = __va(be64_to_cpu((*opal_cpu_metadata).region[0].dest)) as u64;
        (*fadump_conf).cpu_state_data_size = be64_to_cpu((*opal_cpu_metadata).region[0].size);
        if (*fadump_conf).cpu_state_entry_size != 0 { num_cpus = (*fadump_conf).cpu_state_data_size as u32 / (*fadump_conf).cpu_state_entry_size; valid = true; }
    }
    let rc = fadump_setup_cpu_notes_buf(num_cpus);
    if rc != 0 { return rc; }
    let mut note_buf = (*fadump_conf).cpu_notes_buf_vaddr as *mut u32;
    if valid {
        let mut bufp = (*fadump_conf).cpu_state_dest_vaddr as *mut u8;
        let thdr = bufp as *const hdat_fadump_thread_hdr;
        let regs_offset = core::mem::offset_of!(hdat_fadump_thread_hdr, offset) + be32_to_cpu((*thdr).offset) as usize;
        let reg_esize = be32_to_cpu((*thdr).esize);
        let regs_cnt = be32_to_cpu((*thdr).ecnt);
        let size_per_thread = (*fadump_conf).cpu_state_entry_size as usize;
        for i in 0..num_cpus {
            let thdr = bufp as *const hdat_fadump_thread_hdr;
            let pir = be32_to_cpu((*thdr).pir);
            if (*fdh).crashing_cpu == pir { note_buf = fadump_regs_to_elf_notes(note_buf, &mut (*fdh).regs); }
            else if (*thdr).core_state != HDAT_FADUMP_CORE_INACTIVE {
                let mut regs: pt_regs = core::mem::zeroed();
                opal_fadump_read_regs(bufp.add(regs_offset), regs_cnt, reg_esize, true, &mut regs);
                note_buf = fadump_regs_to_elf_notes(note_buf, &mut regs);
            }
            bufp = bufp.add(size_per_thread);
            let _ = i;
        }
    }
    if (*fadump_conf).cpu_notes_buf_vaddr == note_buf as u64 {
        if (*fdh).crashing_cpu == FADUMP_CPU_UNKNOWN { fadump_free_cpu_notes_buf(); return -ENODEV; }
        note_buf = fadump_regs_to_elf_notes(note_buf, &mut (*fdh).regs);
    }
    final_note(note_buf);
    fadump_update_elfcore_header((*fadump_conf).elfcorehdr_addr as *mut i8);
    0
}

#[cfg(not(CONFIG_PRESERVE_FA_DUMP))]
unsafe fn opal_fadump_max_boot_mem_rgns() -> i32 { FADUMP_MAX_MEM_REGS }

#[cfg(not(CONFIG_PRESERVE_FA_DUMP))]
unsafe fn opal_fadump_region_show(fadump_conf: *mut fw_dump, m: *mut seq_file) {
    let fdm = if (*fadump_conf).dump_active != 0 { opal_fdm_active } else { opal_fdm as *const _ };
    let mut dumped_bytes = 0u64;
    for i in 0..be16_to_cpu((*fdm).region_cnt) as usize {
        if (*fadump_conf).dump_active != 0 && i < be16_to_cpu((*fdm).registered_regions) as usize {
            dumped_bytes = be64_to_cpu((*fdm).rgn[i].size);
        }
        seq_printf(m, c"DUMP: Src: %#016llx, Dest: %#016llx, ".as_ptr(), be64_to_cpu((*fdm).rgn[i].src), be64_to_cpu((*fdm).rgn[i].dest));
        seq_printf(m, c"Size: %#llx, Dumped: %#llx bytes\n".as_ptr(), be64_to_cpu((*fdm).rgn[i].size), dumped_bytes);
    }
    if (*fadump_conf).dump_active != 0 {
        seq_printf(m, c"\nMemory above %#016llx is reserved for saving crash dump\n".as_ptr(), (*fadump_conf).boot_mem_top);
    }
}

#[cfg(not(CONFIG_PRESERVE_FA_DUMP))]
unsafe fn opal_fadump_trigger(fdh: *mut fadump_crash_info_header, msg: *const i8) {
    (*fdh).crashing_cpu = mfspr(SPRN_PIR) as u32;
    let rc = opal_cec_reboot2(OPAL_REBOOT_MPIPL, msg);
    if rc == OPAL_UNSUPPORTED { pr_emerg!("Reboot type {} not supported.\n", OPAL_REBOOT_MPIPL); }
    else if rc == OPAL_HARDWARE { pr_emerg!("No backend support for MPIPL!\n"); }
}

#[cfg(not(CONFIG_PRESERVE_FA_DUMP))]
static mut opal_fadump_ops: fadump_ops = fadump_ops {
    fadump_init_mem_struct: Some(opal_fadump_init_mem_struct),
    fadump_get_metadata_size: Some(opal_fadump_get_metadata_size),
    fadump_setup_metadata: Some(opal_fadump_setup_metadata),
    fadump_get_bootmem_min: Some(opal_fadump_get_bootmem_min),
    fadump_register: Some(opal_fadump_register),
    fadump_unregister: Some(opal_fadump_unregister),
    fadump_invalidate: Some(opal_fadump_invalidate),
    fadump_cleanup: Some(opal_fadump_cleanup),
    fadump_process: Some(opal_fadump_process),
    fadump_region_show: Some(opal_fadump_region_show),
    fadump_trigger: Some(opal_fadump_trigger),
    fadump_max_boot_mem_rgns: Some(opal_fadump_max_boot_mem_rgns),
};

#[cfg(not(CONFIG_PRESERVE_FA_DUMP))]
pub unsafe extern "C" fn opal_fadump_dt_scan(fadump_conf: *mut fw_dump, node: u64) {
    let dn = of_get_flat_dt_subnode_by_name(node, c"dump".as_ptr());
    if dn == (-FDT_ERR_NOTFOUND as u64) { pr_debug!("FADump support is missing!\n"); return; }
    if !of_flat_dt_is_compatible(dn, c"ibm,opal-dump".as_ptr()) { pr_err!("Support missing for this f/w version!\n"); return; }
    let mut len = 0;
    let prop = of_get_flat_dt_prop(dn, c"fw-load-area".as_ptr(), &mut len);
    if !prop.is_null() {
        for i in 0..(len as usize / (core::mem::size_of::<__be32>() * 4)) {
            let base = of_read_number(prop.add(i * 4), 2);
            let end = base + of_read_number(prop.add(i * 4 + 2), 2);
            if end > OPAL_FADUMP_MIN_BOOT_MEM { pr_err!("F/W load area: 0x{:x}-0x{:x}\n", base, end); pr_err!("F/W version not supported!\n"); return; }
        }
    }
    (*fadump_conf).ops = &mut opal_fadump_ops;
    (*fadump_conf).fadump_supported = 1;
    (*fadump_conf).param_area_supported = 0;
    (*fadump_conf).max_copy_size = ALIGN_DOWN(U32_MAX, PAGE_SIZE);
    let prop = of_get_flat_dt_prop(dn, c"mpipl-boot".as_ptr(), core::ptr::null_mut());
    if prop.is_null() { return; }
    let mut be_addr = 0u64;
    let ret = opal_mpipl_query_tag(OPAL_MPIPL_TAG_KERNEL, &mut be_addr);
    if ret != OPAL_SUCCESS || be_addr == 0 { pr_err!("Failed to get Kernel metadata ({})\n", ret); return; }
    let addr = be64_to_cpu(be_addr);
    pr_debug!("Kernel metadata addr: {:x}\n", addr);
    opal_fdm_active = __va(addr);
    if (*opal_fdm_active).version != OPAL_FADUMP_VERSION { pr_warn!("Supported kernel metadata version: {}, found: {}!\n", OPAL_FADUMP_VERSION, (*opal_fdm_active).version); pr_warn!("WARNING: Kernel metadata format mismatch identified! Core file maybe corrupted..\n"); }
    if be16_to_cpu((*opal_fdm_active).registered_regions) == 0 { opal_fdm_active = core::ptr::null(); return; }
    let _ = opal_mpipl_query_tag(OPAL_MPIPL_TAG_CPU, &mut be_addr);
    if be_addr != 0 { opal_cpu_metadata = __va(be64_to_cpu(be_addr)); }
    pr_info!("Firmware-assisted dump is active.\n");
    (*fadump_conf).dump_active = 1;
    opal_fadump_get_config(fadump_conf, opal_fdm_active);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
