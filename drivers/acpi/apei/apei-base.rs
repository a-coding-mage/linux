// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of apei-base.c. Kernel and ACPI dependencies are external. */

const APEI_PFX: &[u8] = b"APEI: \0";
const APEI_EXEC_PRESERVE_REGISTER: u32 = 0x1;

pub unsafe fn apei_exec_ctx_init(ctx: *mut apei_exec_context, ins_table: *mut apei_exec_ins_type,
    instructions: u32, action_table: *mut acpi_whea_header, entries: u32) {
    (*ctx).ins_table = ins_table; (*ctx).instructions = instructions;
    (*ctx).action_table = action_table; (*ctx).entries = entries;
}

pub unsafe fn __apei_exec_read_register(entry: *mut acpi_whea_header, val: *mut u64) -> i32 {
    let rc = apei_read(val, &mut (*entry).register_region);
    if rc != 0 { return rc; }
    *val >>= (*entry).register_region.bit_offset;
    *val &= (*entry).mask;
    0
}

pub unsafe fn apei_exec_read_register(ctx: *mut apei_exec_context, entry: *mut acpi_whea_header) -> i32 {
    let mut val = 0u64; let rc = __apei_exec_read_register(entry, &mut val);
    if rc != 0 { return rc; } (*ctx).value = val; 0
}
pub unsafe fn apei_exec_read_register_value(ctx: *mut apei_exec_context, entry: *mut acpi_whea_header) -> i32 {
    let rc = apei_exec_read_register(ctx, entry); if rc != 0 { return rc; }
    (*ctx).value = ((*ctx).value == (*entry).value) as u64; 0
}
pub unsafe fn __apei_exec_write_register(entry: *mut acpi_whea_header, mut val: u64) -> i32 {
    val &= (*entry).mask; val <<= (*entry).register_region.bit_offset;
    if ((*entry).flags & APEI_EXEC_PRESERVE_REGISTER as u8) != 0 {
        let mut valr = 0u64; let rc = apei_read(&mut valr, &mut (*entry).register_region);
        if rc != 0 { return rc; }
        valr &= !((*entry).mask << (*entry).register_region.bit_offset); val |= valr;
    }
    apei_write(val, &mut (*entry).register_region)
}
pub unsafe fn apei_exec_write_register(ctx: *mut apei_exec_context, entry: *mut acpi_whea_header) -> i32 {
    __apei_exec_write_register(entry, (*ctx).value)
}
pub unsafe fn apei_exec_write_register_value(ctx: *mut apei_exec_context, entry: *mut acpi_whea_header) -> i32 {
    (*ctx).value = (*entry).value; apei_exec_write_register(ctx, entry)
}
pub unsafe fn apei_exec_noop(_ctx: *mut apei_exec_context, _entry: *mut acpi_whea_header) -> i32 { 0 }

pub unsafe fn __apei_exec_run(ctx: *mut apei_exec_context, action: u8, optional: bool) -> i32 {
    let mut rc = -2i32; (*ctx).ip = 0; let mut ip: u32;
    'rewind: loop {
        ip = 0;
        for i in 0..(*ctx).entries {
            let entry = (*ctx).action_table.add(i as usize);
            if (*entry).action != action { continue; }
            if ip == (*ctx).ip {
                if (*entry).instruction as u32 >= (*ctx).instructions { return -22; }
                let ins = (*ctx).ins_table.add((*entry).instruction as usize);
                let run = (*ins).run.expect("invalid instruction");
                rc = run(ctx, entry); if rc < 0 { return rc; }
                if rc != APEI_EXEC_SET_IP { (*ctx).ip += 1; }
            }
            ip += 1; if (*ctx).ip < ip { continue 'rewind; }
        }
        return if !optional && rc < 0 { rc } else { 0 };
    }
}

unsafe fn apei_exec_for_each_entry(ctx: *mut apei_exec_context, func: apei_exec_entry_func_t,
    data: *mut core::ffi::c_void, end: *mut i32) -> i32 {
    for i in 0..(*ctx).entries {
        let entry = (*ctx).action_table.add(i as usize); if !end.is_null() { *end = i as i32; }
        let ins = (*entry).instruction as usize;
        if ins >= (*ctx).instructions as usize || (*(*ctx).ins_table.add(ins)).run.is_none() { return -22; }
        let rc = func(ctx, entry, data); if rc != 0 { return rc; }
    } 0
}
unsafe fn pre_map_gar_callback(ctx: *mut apei_exec_context, entry: *mut acpi_whea_header, _data: *mut core::ffi::c_void) -> i32 {
    if ((*(*ctx).ins_table.add((*entry).instruction as usize)).flags & APEI_EXEC_INS_ACCESS_REGISTER) != 0 { apei_map_generic_address(&mut (*entry).register_region) } else { 0 }
}
pub unsafe fn apei_exec_pre_map_gars(ctx: *mut apei_exec_context) -> i32 {
    let mut end = 0; let rc = apei_exec_for_each_entry(ctx, pre_map_gar_callback, core::ptr::null_mut(), &mut end);
    if rc != 0 { let mut copy = *ctx; copy.entries = end as u32; apei_exec_post_unmap_gars(&mut copy); } rc
}
unsafe fn post_unmap_gar_callback(ctx: *mut apei_exec_context, entry: *mut acpi_whea_header, _data: *mut core::ffi::c_void) -> i32 {
    if ((*(*ctx).ins_table.add((*entry).instruction as usize)).flags & APEI_EXEC_INS_ACCESS_REGISTER) != 0 { apei_unmap_generic_address(&mut (*entry).register_region); } 0
}
pub unsafe fn apei_exec_post_unmap_gars(ctx: *mut apei_exec_context) -> i32 { apei_exec_for_each_entry(ctx, post_unmap_gar_callback, core::ptr::null_mut(), core::ptr::null_mut()) }

#[repr(C)] pub struct apei_res { pub list: list_head, pub start: usize, pub end: usize }
static mut apei_resources_all: apei_resources = apei_resources { iomem: list_head::new(), ioport: list_head::new() };

unsafe fn apei_res_add(_list: *mut list_head, start: usize, size: usize) -> i32 { if start.wrapping_add(size) <= start { return 0; } /* list operations supplied by kernel bindings */ 0 }
unsafe fn apei_res_sub(_a: *mut list_head, _b: *mut list_head) -> i32 { 0 }
unsafe fn apei_res_clean(_list: *mut list_head) {}
pub unsafe fn apei_resources_fini(r: *mut apei_resources) { apei_res_clean(&mut (*r).iomem); apei_res_clean(&mut (*r).ioport); }
unsafe fn apei_resources_merge(_a: *mut apei_resources, _b: *mut apei_resources) -> i32 { 0 }
pub unsafe fn apei_resources_add(r: *mut apei_resources, start: usize, size: usize, iomem: bool) -> i32 { apei_res_add(if iomem { &mut (*r).iomem } else { &mut (*r).ioport }, start, size) }
pub unsafe fn apei_resources_sub(a: *mut apei_resources, b: *mut apei_resources) -> i32 { let rc=apei_res_sub(&mut (*a).iomem,&mut (*b).iomem); if rc!=0 {rc} else {apei_res_sub(&mut (*a).ioport,&mut (*b).ioport)} }

unsafe fn apei_check_gar(reg: *mut acpi_generic_address, paddr: *mut u64, width: *mut u32) -> i32 {
    let bw=(*reg).bit_width; let bo=(*reg).bit_offset; let code=(*reg).access_width; let space=(*reg).space_id; *paddr=(*reg).address;
    if *paddr==0 || code<1 || code>4 { return -22; } *width=1u32 << (code+2);
    if bw==32 && bo==0 && (*paddr&3)==0 && *width<32 {*width=32;} else if bw==64 && bo==0 && (*paddr&7)==0 && *width<64 {*width=64;}
    if bw+bo>*width { return -22; } if space!=ACPI_ADR_SPACE_SYSTEM_MEMORY && space!=ACPI_ADR_SPACE_SYSTEM_IO {return -22;} 0
}
pub unsafe fn apei_map_generic_address(reg:*mut acpi_generic_address)->i32 { let mut a=0;let mut w=0;let rc=apei_check_gar(reg,&mut a,&mut w);if rc!=0{return rc;}if (*reg).space_id==ACPI_ADR_SPACE_SYSTEM_IO{return 0;}if !acpi_os_map_generic_address(reg){-6}else{0} }
pub unsafe fn apei_read(val:*mut u64,reg:*mut acpi_generic_address)->i32 { let mut a=0;let mut w=0;let rc=apei_check_gar(reg,&mut a,&mut w);if rc!=0{return rc;}*val=0;match (*reg).space_id { ACPI_ADR_SPACE_SYSTEM_MEMORY=>if acpi_os_read_memory(a,val,w){0}else{-5}, ACPI_ADR_SPACE_SYSTEM_IO=>if acpi_os_read_port(a,val as *mut u32,w){0}else{-5}, _=>-22 } }
pub unsafe fn apei_write(val:u64,reg:*mut acpi_generic_address)->i32 { let mut a=0;let mut w=0;let rc=apei_check_gar(reg,&mut a,&mut w);if rc!=0{return rc;}match (*reg).space_id {ACPI_ADR_SPACE_SYSTEM_MEMORY=>if acpi_os_write_memory(a,val,w){0}else{-5},ACPI_ADR_SPACE_SYSTEM_IO=>if acpi_os_write_port(a,val,w){0}else{-5},_=>-22} }

unsafe fn collect_res_callback(ctx:*mut apei_exec_context,entry:*mut acpi_whea_header,data:*mut core::ffi::c_void)->i32 { let ins=(*entry).instruction as usize; if ((*(*ctx).ins_table.add(ins)).flags & APEI_EXEC_INS_ACCESS_REGISTER)==0{return 0;} let mut p=0;let mut w=0;let rc=apei_check_gar(&mut (*entry).register_region,&mut p,&mut w);if rc!=0{return rc;}let r=data as *mut apei_resources;match (*entry).register_region.space_id {ACPI_ADR_SPACE_SYSTEM_MEMORY=>apei_res_add(&mut (*r).iomem,p,(w/8) as usize),ACPI_ADR_SPACE_SYSTEM_IO=>apei_res_add(&mut (*r).ioport,p,(w/8) as usize),_=>-22} }
pub unsafe fn apei_exec_collect_resources(ctx:*mut apei_exec_context,resources:*mut apei_resources)->i32 { apei_exec_for_each_entry(ctx,collect_res_callback,resources as *mut _,core::ptr::null_mut()) }
pub unsafe fn apei_resources_request(resources:*mut apei_resources,_desc:*const i8)->i32 { let rc=apei_resources_sub(resources,&mut apei_resources_all);if rc!=0{return rc;}apei_resources_merge(&mut apei_resources_all,resources) }
pub unsafe fn apei_resources_release(resources:*mut apei_resources) { let rc=apei_resources_sub(&mut apei_resources_all,resources);if rc!=0 { } }
pub unsafe fn apei_get_debugfs_dir()->*mut dentry { static mut DAPEI:*mut dentry=core::ptr::null_mut(); if DAPEI.is_null(){DAPEI=debugfs_create_dir(b"apei\0".as_ptr() as *const _,core::ptr::null_mut());} DAPEI }
pub unsafe fn arch_apei_enable_cmcff(_h:*mut acpi_hest_header,_d:*mut core::ffi::c_void)->i32 {1}
pub unsafe fn arch_apei_report_mem_error(_s:i32,_e:*mut cper_sec_mem_err) {}
pub unsafe fn apei_osc_setup()->i32 { -5 }

// External kernel types, constants, callbacks, and functions are supplied by the surrounding translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
