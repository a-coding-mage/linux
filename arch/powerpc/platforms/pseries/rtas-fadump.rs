// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Firmware-Assisted Dump support on POWERVM platform.
 *
 * Copyright 2011, Mahesh Salgaonkar, IBM Corporation.
 * Copyright 2019, Hari Bathini, IBM Corporation.
 */

// C includes and build-time kernel dependencies are supplied externally.

static mut fdm: rtas_fadump_mem_struct = unsafe { core::mem::zeroed() };
static mut fdm_active: *const rtas_fadump_mem_struct = core::ptr::null();

unsafe fn rtas_fadump_update_config(fadump_conf: *mut fw_dump, _fdm: *const rtas_fadump_mem_struct) {
    (*fadump_conf).fadumphdr_addr = (*fadump_conf).boot_mem_dest_addr + (*fadump_conf).boot_memory_size;
}

unsafe fn rtas_fadump_get_config(fadump_conf: *mut fw_dump, fdm: *const rtas_fadump_mem_struct) {
    let mut last_end: c_ulong = 0;
    let mut hole_size: c_ulong = 0;
    (*fadump_conf).boot_memory_size = 0;
    (*fadump_conf).boot_mem_regs_cnt = 0;
    pr_debug!("Boot memory regions:\n");
    for i in 0..be16_to_cpu((*fdm).header.dump_num_sections) {
        let type_ = be16_to_cpu((*fdm).rgn[i as usize].source_data_type);
        let mut addr: u64;
        match type_ {
            RTAS_FADUMP_CPU_STATE_DATA => {
                addr = be64_to_cpu((*fdm).rgn[i as usize].destination_address);
                (*fadump_conf).cpu_state_dest_vaddr = __va(addr) as u64;
                (*fadump_conf).reserve_dump_area_start = addr;
            }
            RTAS_FADUMP_HPTE_REGION => {}
            RTAS_FADUMP_REAL_MODE_REGION => {
                let base = be64_to_cpu((*fdm).rgn[i as usize].source_address);
                let size = be64_to_cpu((*fdm).rgn[i as usize].source_len);
                pr_debug!("\t[%03d] base: 0x%lx, size: 0x%lx\n", i, base, size);
                if base == 0 { (*fadump_conf).boot_mem_dest_addr = be64_to_cpu((*fdm).rgn[i as usize].destination_address); }
                let n = (*fadump_conf).boot_mem_regs_cnt as usize;
                (*fadump_conf).boot_mem_addr[n] = base;
                (*fadump_conf).boot_mem_sz[n] = size;
                (*fadump_conf).boot_memory_size += size;
                hole_size += base - last_end;
                last_end = base + size;
                (*fadump_conf).boot_mem_regs_cnt += 1;
            }
            RTAS_FADUMP_PARAM_AREA => (*fadump_conf).param_area = be64_to_cpu((*fdm).rgn[i as usize].destination_address),
            _ => pr_warn!("Section type %d unsupported on this kernel. Ignoring!\n", type_),
        }
    }
    (*fadump_conf).boot_mem_top = (*fadump_conf).boot_memory_size + hole_size;
    rtas_fadump_update_config(fadump_conf, fdm);
}

unsafe fn rtas_fadump_init_mem_struct(fadump_conf: *mut fw_dump) -> u64 {
    let mut addr = (*fadump_conf).reserve_dump_area_start & PAGE_MASK;
    let mut sec_cnt: u16 = 0;
    memset(&mut fdm as *mut _ as *mut _, 0, core::mem::size_of::<rtas_fadump_mem_struct>());
    fdm.header.dump_format_version = cpu_to_be32(1);
    fdm.header.dump_status_flag = 0;
    fdm.header.offset_first_dump_section = cpu_to_be32(core::mem::offset_of!(rtas_fadump_mem_struct, rgn) as u32);
    fdm.header.dd_block_size = 0; fdm.header.dd_block_offset = 0; fdm.header.dd_num_blocks = 0;
    fdm.header.dd_offset_disk_path = 0; fdm.header.max_time_auto = 0;
    fdm.rgn[sec_cnt as usize].request_flag = cpu_to_be32(RTAS_FADUMP_REQUEST_FLAG);
    fdm.rgn[sec_cnt as usize].source_data_type = cpu_to_be16(RTAS_FADUMP_CPU_STATE_DATA);
    fdm.rgn[sec_cnt as usize].source_address = 0;
    fdm.rgn[sec_cnt as usize].source_len = cpu_to_be64((*fadump_conf).cpu_state_data_size);
    fdm.rgn[sec_cnt as usize].destination_address = cpu_to_be64(addr);
    addr += (*fadump_conf).cpu_state_data_size; sec_cnt += 1;
    fdm.rgn[sec_cnt as usize].request_flag = cpu_to_be32(RTAS_FADUMP_REQUEST_FLAG);
    fdm.rgn[sec_cnt as usize].source_data_type = cpu_to_be16(RTAS_FADUMP_HPTE_REGION);
    fdm.rgn[sec_cnt as usize].source_address = 0;
    fdm.rgn[sec_cnt as usize].source_len = cpu_to_be64((*fadump_conf).hpte_region_size);
    fdm.rgn[sec_cnt as usize].destination_address = cpu_to_be64(addr);
    addr += (*fadump_conf).hpte_region_size; sec_cnt += 1;
    addr = PAGE_ALIGN(addr);
    (*fadump_conf).boot_mem_dest_addr = addr;
    for i in 0..(*fadump_conf).boot_mem_regs_cnt {
        let n = sec_cnt as usize;
        fdm.rgn[n].request_flag = cpu_to_be32(RTAS_FADUMP_REQUEST_FLAG);
        fdm.rgn[n].source_data_type = cpu_to_be16(RTAS_FADUMP_REAL_MODE_REGION);
        fdm.rgn[n].source_address = cpu_to_be64((*fadump_conf).boot_mem_addr[i as usize]);
        fdm.rgn[n].source_len = cpu_to_be64((*fadump_conf).boot_mem_sz[i as usize]);
        fdm.rgn[n].destination_address = cpu_to_be64(addr);
        addr += (*fadump_conf).boot_mem_sz[i as usize]; sec_cnt += 1;
    }
    if (*fadump_conf).param_area != 0 {
        let n = sec_cnt as usize;
        fdm.rgn[n].request_flag = cpu_to_be32(RTAS_FADUMP_REQUEST_FLAG);
        fdm.rgn[n].source_data_type = cpu_to_be16(RTAS_FADUMP_PARAM_AREA);
        fdm.rgn[n].source_address = cpu_to_be64((*fadump_conf).param_area);
        fdm.rgn[n].source_len = cpu_to_be64(COMMAND_LINE_SIZE);
        fdm.rgn[n].destination_address = cpu_to_be64((*fadump_conf).param_area); sec_cnt += 1;
    }
    fdm.header.dump_num_sections = cpu_to_be16(sec_cnt);
    rtas_fadump_update_config(fadump_conf, &fdm); addr
}

unsafe fn rtas_fadump_get_bootmem_min() -> u64 { RTAS_FADUMP_MIN_BOOT_MEM }

unsafe fn rtas_fadump_call(conf: *mut fw_dump, operation: c_int, fdm_ptr: *mut c_void, fdm_size: c_uint, op_name: *const c_char) -> c_int {
    let mut total_wait: c_uint = 0;
    loop {
        let rc = rtas_call((*conf).ibm_configure_kernel_dump, 3, 1, core::ptr::null_mut(), operation, fdm_ptr, fdm_size);
        let wait_time = rtas_busy_delay_time(rc);
        if wait_time != 0 {
            pr_debug!("Firmware busy during fadump %s, waiting %ums (total %ums)\n", op_name, wait_time, total_wait);
            if total_wait >= RTAS_FADUMP_MAX_WAIT_MS { pr_err!("Timed out waiting for firmware to complete fadump %s\n", op_name); return -ETIMEDOUT; }
            total_wait += wait_time; mdelay(wait_time);
        } else { return rc; }
    }
}

unsafe fn rtas_fadump_register(conf: *mut fw_dump) -> c_int {
    let size = core::mem::size_of::<rtas_fadump_section_header>() + be16_to_cpu(fdm.header.dump_num_sections) as usize * core::mem::size_of::<rtas_fadump_section>();
    let rc = rtas_fadump_call(conf, FADUMP_REGISTER, &mut fdm as *mut _ as *mut c_void, size as c_uint, c"register".as_ptr());
    if rc == -ETIMEDOUT { return rc; }
    match rc { 0 => { pr_info!("Registration is successful!\n"); (*conf).dump_registered = 1; 0 }, -1 => { pr_err!("Failed to register. Hardware Error(%d).\n", rc); -EIO }, -3 => { if !is_fadump_reserved_mem_contiguous() { pr_err!("Can't have holes in reserved memory area.\n"); } pr_err!("Failed to register. Parameter Error(%d).\n", rc); -EINVAL }, -9 => { pr_err!("Already registered!\n"); (*conf).dump_registered = 1; -EEXIST }, _ => { pr_err!("Failed to register. Unknown Error(%d).\n", rc); -EIO } }
}

unsafe fn rtas_fadump_unregister(conf: *mut fw_dump) -> c_int { let rc = rtas_fadump_call(conf, FADUMP_UNREGISTER, &mut fdm as *mut _ as *mut c_void, core::mem::size_of::<rtas_fadump_mem_struct>() as c_uint, c"unregister".as_ptr()); if rc == -ETIMEDOUT { return rc; } if rc != 0 { pr_err!("Failed to un-register - unexpected error(%d).\n", rc); return -EIO; } (*conf).dump_registered = 0; 0 }
unsafe fn rtas_fadump_invalidate(conf: *mut fw_dump) -> c_int { let rc = rtas_fadump_call(conf, FADUMP_INVALIDATE, fdm_active as *mut c_void, core::mem::size_of::<rtas_fadump_mem_struct>() as c_uint, c"invalidate".as_ptr()); if rc == -ETIMEDOUT { return rc; } if rc != 0 { pr_err!("Failed to invalidate - unexpected error (%d).\n", rc); return -EIO; } (*conf).dump_active = 0; fdm_active = core::ptr::null(); 0 }

const RTAS_FADUMP_GPR_MASK: u64 = 0xffffff0000000000;
unsafe fn rtas_fadump_gpr_index(mut id: u64) -> c_int { let mut i = -1; if id & RTAS_FADUMP_GPR_MASK == fadump_str_to_u64(c"GPR".as_ptr()) { id = (id & !RTAS_FADUMP_GPR_MASK) >> 24; let str_ = [((id >> 8) & 0xff) as c_char, (id & 0xff) as c_char, 0]; if kstrtoint(str_.as_ptr(), 10, &mut i) != 0 { i = -EINVAL; } if i > 31 { i = -1; } } i }

unsafe fn rtas_fadump_set_regval(regs: *mut pt_regs, id: u64, val: u64) { let i = rtas_fadump_gpr_index(id); if i >= 0 { (*regs).gpr[i as usize] = val as c_ulong; } else if id == fadump_str_to_u64(c"NIA".as_ptr()) { (*regs).nip=val as c_ulong; } else if id == fadump_str_to_u64(c"MSR".as_ptr()) { (*regs).msr=val as c_ulong; } else if id == fadump_str_to_u64(c"CTR".as_ptr()) { (*regs).ctr=val as c_ulong; } else if id == fadump_str_to_u64(c"LR".as_ptr()) { (*regs).link=val as c_ulong; } else if id == fadump_str_to_u64(c"XER".as_ptr()) { (*regs).xer=val as c_ulong; } else if id == fadump_str_to_u64(c"CR".as_ptr()) { (*regs).ccr=val as c_ulong; } else if id == fadump_str_to_u64(c"DAR".as_ptr()) { (*regs).dar=val as c_ulong; } else if id == fadump_str_to_u64(c"DSISR".as_ptr()) { (*regs).dsisr=val as c_ulong; } }

unsafe fn rtas_fadump_read_regs(mut e: *mut rtas_fadump_reg_entry, regs: *mut pt_regs) -> *mut rtas_fadump_reg_entry { memset(regs as *mut c_void, 0, core::mem::size_of::<pt_regs>()); while be64_to_cpu((*e).reg_id) != fadump_str_to_u64(c"CPUEND".as_ptr()) { rtas_fadump_set_regval(regs, be64_to_cpu((*e).reg_id), be64_to_cpu((*e).reg_value)); e = e.add(1); } e.add(1) }

// Read CPU state dump data and convert it into ELF notes. The firmware format
// consists of REGSAVE, CPUSTRT, register entries, and CPUEND records.
unsafe fn rtas_fadump_build_cpu_notes(conf: *mut fw_dump) -> c_int { let h = (*conf).cpu_state_dest_vaddr as *mut rtas_fadump_reg_save_area_header; if be64_to_cpu((*h).magic_number) != fadump_str_to_u64(c"REGSAVE".as_ptr()) { pr_err!("Unable to read register save area.\n"); return -ENOENT; } let mut v = (h as *mut u8).add(be32_to_cpu((*h).num_cpu_offset) as usize); let n = be32_to_cpu(*(v as *const __be32)); v = v.add(4); let mut e = v as *mut rtas_fadump_reg_entry; let rc = fadump_setup_cpu_notes_buf(n); if rc != 0 { return rc; } let mut notes = (*conf).cpu_notes_buf_vaddr as *mut u32; let fdh = if (*conf).fadumphdr_addr != 0 { __va((*conf).fadumphdr_addr) as *mut fadump_crash_info_header } else { core::ptr::null_mut() }; for _ in 0..n { if be64_to_cpu((*e).reg_id) != fadump_str_to_u64(c"CPUSTRT".as_ptr()) { pr_err!("Unable to read CPU state data\n"); fadump_free_cpu_notes_buf(); return -ENOENT; } let cpu = (be64_to_cpu((*e).reg_value) & RTAS_FADUMP_CPU_ID_MASK) as c_int; if !fdh.is_null() && !cpumask_test_cpu(cpu, &(*fdh).cpu_mask) { RTAS_FADUMP_SKIP_TO_NEXT_CPU!(e); continue; } if !fdh.is_null() && (*fdh).crashing_cpu == cpu { let mut regs = (*fdh).regs; notes = fadump_regs_to_elf_notes(notes, &mut regs); RTAS_FADUMP_SKIP_TO_NEXT_CPU!(e); } else { e = rtas_fadump_read_regs(e.add(1), &mut *(core::ptr::addr_of_mut!((*fdh).regs))); notes = fadump_regs_to_elf_notes(notes, &mut *(core::ptr::addr_of_mut!((*fdh).regs))); } } final_note(notes); fadump_update_elfcore_header((*conf).elfcorehdr_addr as *mut c_char); 0 }

unsafe fn rtas_fadump_process(conf: *mut fw_dump) -> c_int {
    if fdm_active.is_null() || (*conf).fadumphdr_addr == 0 { return -EINVAL; }
    for i in 0..be16_to_cpu((*fdm_active).header.dump_num_sections) {
        let r = &(*fdm_active).rgn[i as usize];
        match be16_to_cpu(r.source_data_type) {
            RTAS_FADUMP_CPU_STATE_DATA | RTAS_FADUMP_HPTE_REGION | RTAS_FADUMP_REAL_MODE_REGION => {
                if r.error_flags != 0 || r.bytes_dumped != r.source_len { pr_err!("Dump taken by platform is not valid or incomplete (%d)\n", i); return -EINVAL; }
            }
            RTAS_FADUMP_PARAM_AREA => { if r.bytes_dumped != r.source_len || r.error_flags != 0 { pr_warn!("Failed to process additional parameters! Proceeding anyway..\n"); (*conf).param_area = 0; } }
            _ => pr_warn!("Unknown region found: type: %u src addr: 0x%llx dest addr: 0x%llx\n", be16_to_cpu(r.source_data_type), be64_to_cpu(r.source_address), be64_to_cpu(r.destination_address)),
        }
    }
    rtas_fadump_build_cpu_notes(conf)
}

unsafe fn rtas_fadump_region_show(conf: *mut fw_dump, m: *mut seq_file) {
    let p = if !fdm_active.is_null() { fdm_active } else { &fdm };
    for i in 0..be16_to_cpu((*p).header.dump_num_sections) { let r = &(*p).rgn[i as usize]; match be16_to_cpu(r.source_data_type) {
        RTAS_FADUMP_CPU_STATE_DATA => seq_printf!(m, "CPU :[%#016llx-%#016llx] %#llx bytes, Dumped: %#llx\n", be64_to_cpu(r.destination_address), be64_to_cpu(r.destination_address)+be64_to_cpu(r.source_len)-1, be64_to_cpu(r.source_len), be64_to_cpu(r.bytes_dumped)),
        RTAS_FADUMP_HPTE_REGION => seq_printf!(m, "HPTE:[%#016llx-%#016llx] %#llx bytes, Dumped: %#llx\n", be64_to_cpu(r.destination_address), be64_to_cpu(r.destination_address)+be64_to_cpu(r.source_len)-1, be64_to_cpu(r.source_len), be64_to_cpu(r.bytes_dumped)),
        RTAS_FADUMP_REAL_MODE_REGION => { seq_printf!(m, "DUMP: Src: %#016llx, Dest: %#016llx, ", be64_to_cpu(r.source_address), be64_to_cpu(r.destination_address)); seq_printf!(m, "Size: %#llx, Dumped: %#llx bytes\n", be64_to_cpu(r.source_len), be64_to_cpu(r.bytes_dumped)); },
        RTAS_FADUMP_PARAM_AREA => seq_printf!(m, "\n[%#016llx-%#016llx]: cmdline append: '%s'\n", be64_to_cpu(r.destination_address), be64_to_cpu(r.destination_address)+be64_to_cpu(r.source_len)-1, __va(be64_to_cpu(r.destination_address)) as *const c_char),
        t => seq_printf!(m, "Unknown region type %d : Src: %#016llx, Dest: %#016llx, ", t, be64_to_cpu(r.source_address), be64_to_cpu(r.destination_address)),
    }}
    if !fdm_active.is_null() { seq_printf!(m, "\nMemory above %#016llx is reserved for saving crash dump\n", (*conf).boot_mem_top); }
}
unsafe fn rtas_fadump_trigger(_fdh: *mut fadump_crash_info_header, msg: *const c_char) { rtas_os_term(msg as *mut c_char); }
unsafe fn rtas_fadump_max_boot_mem_rgns() -> c_int { RTAS_FADUMP_MAX_BOOT_MEM_REGS }

static mut rtas_fadump_ops: fadump_ops = fadump_ops {
    fadump_init_mem_struct: Some(rtas_fadump_init_mem_struct), fadump_get_bootmem_min: Some(rtas_fadump_get_bootmem_min),
    fadump_register: Some(rtas_fadump_register), fadump_unregister: Some(rtas_fadump_unregister), fadump_invalidate: Some(rtas_fadump_invalidate),
    fadump_process: Some(rtas_fadump_process), fadump_region_show: Some(rtas_fadump_region_show), fadump_trigger: Some(rtas_fadump_trigger),
    fadump_max_boot_mem_rgns: Some(rtas_fadump_max_boot_mem_rgns),
};

unsafe fn rtas_fadump_dt_scan(conf: *mut fw_dump, node: u64) {
    let token = of_get_flat_dt_prop(node, c"ibm,configure-kernel-dump".as_ptr(), core::ptr::null_mut()); if token.is_null() { return; }
    (*conf).ibm_configure_kernel_dump = be32_to_cpu(*token); (*conf).ops = &mut rtas_fadump_ops; (*conf).fadump_supported=1; (*conf).param_area_supported=1; (*conf).max_copy_size = ALIGN_DOWN(U64_MAX, PAGE_SIZE);
    fdm_active = of_get_flat_dt_prop(node, c"ibm,kernel-dump".as_ptr(), core::ptr::null_mut()) as *const rtas_fadump_mem_struct;
    if !fdm_active.is_null() { (*conf).dump_active=1; rtas_fadump_get_config(conf, __pa(fdm_active as u64) as *const rtas_fadump_mem_struct); }
    let mut size=0; let mut sections=of_get_flat_dt_prop(node, c"ibm,configure-kernel-dump-sizes".as_ptr(), &mut size); if sections.is_null() { return; }
    for _ in 0..(size/(3*core::mem::size_of::<u32>() as c_int)) { let type_=of_read_number(sections,1) as u32; match type_ { RTAS_FADUMP_CPU_STATE_DATA => (*conf).cpu_state_data_size=of_read_ulong(sections.add(1),2), RTAS_FADUMP_HPTE_REGION => (*conf).hpte_region_size=of_read_ulong(sections.add(1),2), _=>{} } sections=sections.add(3); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
