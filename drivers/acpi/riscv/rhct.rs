// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2022-2023, Ventana Micro Systems Inc
 *	Author: Sunil V L <sunilvl@ventanamicro.com>
 */

// Dependency declarations and ACPI structures are supplied by the surrounding
// kernel translation.

static mut RHCT: *mut acpi_table_header = core::ptr::null_mut();

unsafe fn acpi_get_rhct() -> *mut acpi_table_rhct {
    let mut status: acpi_status;

    /*
     * RHCT will be used at runtime on every CPU, so we
     * don't need to call acpi_put_table() to release the table mapping.
     */
    if RHCT.is_null() {
        status = acpi_get_table(ACPI_SIG_RHCT, 0, &mut RHCT);
        if ACPI_FAILURE(status) {
            pr_warn_once!("No RHCT table found\n");
            return core::ptr::null_mut();
        }
    }

    RHCT as *mut acpi_table_rhct
}

/*
 * During early boot, the caller should call acpi_get_table() and pass its pointer to
 * these functions(and free up later). At run time, since this table can be used
 * multiple times, NULL may be passed in order to use the cached table.
 */
pub unsafe fn acpi_get_riscv_isa(
    table: *mut acpi_table_header,
    cpu: c_uint,
    isa: *mut *const c_char,
) -> c_int {
    let size_hdr: u32 = core::mem::size_of::<acpi_rhct_node_header>() as u32;
    let size_hartinfo: u32 = core::mem::size_of::<acpi_rhct_hart_info>() as u32;
    let mut acpi_cpu_id: u32 = 0;
    let ret = acpi_get_cpu_uid(cpu, &mut acpi_cpu_id);

    BUG_ON!(acpi_disabled);
    if ret != 0 {
        return ret;
    }

    let rhct = if table.is_null() {
        let value = acpi_get_rhct();
        if value.is_null() {
            return -ENOENT;
        }
        value
    } else {
        table as *mut acpi_table_rhct
    };

    let mut node = (rhct as *mut u8).add((*rhct).node_offset as usize)
        as *mut acpi_rhct_node_header;
    let end = (rhct as *mut u8).add((*rhct).header.length as usize)
        as *mut acpi_rhct_node_header;

    while node < end {
        if (*node).type_ == ACPI_RHCT_NODE_TYPE_HART_INFO {
            let hart_info = (node as *mut u8).add(size_hdr as usize)
                as *mut acpi_rhct_hart_info;
            let hart_info_node_offset = (hart_info as *mut u8).add(size_hartinfo as usize)
                as *mut u32;
            if acpi_cpu_id != (*hart_info).uid {
                node = (node as *mut u8).add((*node).length as usize)
                    as *mut acpi_rhct_node_header;
                continue;
            }

            for i in 0..(*hart_info).num_offsets {
                let ref_node = (rhct as *mut u8)
                    .add(*hart_info_node_offset.add(i as usize) as usize)
                    as *mut acpi_rhct_node_header;
                if (*ref_node).type_ == ACPI_RHCT_NODE_TYPE_ISA_STRING {
                    let isa_node = (ref_node as *mut u8).add(size_hdr as usize)
                        as *mut acpi_rhct_isa_string;
                    *isa = (*isa_node).isa;
                    return 0;
                }
            }
        }
        node = (node as *mut u8).add((*node).length as usize)
            as *mut acpi_rhct_node_header;
    }

    -1
}

unsafe fn acpi_parse_hart_info_cmo_node(
    rhct: *mut acpi_table_rhct,
    hart_info: *mut acpi_rhct_hart_info,
    cbom_size: *mut u32,
    cboz_size: *mut u32,
    cbop_size: *mut u32,
) {
    let size_hartinfo = core::mem::size_of::<acpi_rhct_hart_info>();
    let size_hdr = core::mem::size_of::<acpi_rhct_node_header>();
    let offsets = (hart_info as *mut u8).add(size_hartinfo) as *mut u32;

    for i in 0..(*hart_info).num_offsets {
        let ref_node = (rhct as *mut u8).add(*offsets.add(i as usize) as usize)
            as *mut acpi_rhct_node_header;
        if (*ref_node).type_ == ACPI_RHCT_NODE_TYPE_CMO {
            let cmo_node = (ref_node as *mut u8).add(size_hdr)
                as *mut acpi_rhct_cmo_node;
            if !cbom_size.is_null() && (*cmo_node).cbom_size <= 30 {
                if *cbom_size == 0 { *cbom_size = 1u32 << (*cmo_node).cbom_size; }
                else if *cbom_size != 1u32 << (*cmo_node).cbom_size { pr_warn!("CBOM size is not the same across harts\n"); }
            }
            if !cboz_size.is_null() && (*cmo_node).cboz_size <= 30 {
                if *cboz_size == 0 { *cboz_size = 1u32 << (*cmo_node).cboz_size; }
                else if *cboz_size != 1u32 << (*cmo_node).cboz_size { pr_warn!("CBOZ size is not the same across harts\n"); }
            }
            if !cbop_size.is_null() && (*cmo_node).cbop_size <= 30 {
                if *cbop_size == 0 { *cbop_size = 1u32 << (*cmo_node).cbop_size; }
                else if *cbop_size != 1u32 << (*cmo_node).cbop_size { pr_warn!("CBOP size is not the same across harts\n"); }
            }
        }
    }
}

/*
 * During early boot, the caller should call acpi_get_table() and pass its pointer to
 * these functions (and free up later). At run time, since this table can be used
 * multiple times, pass NULL so that the table remains in memory.
 */
pub unsafe fn acpi_get_cbo_block_size(
    table: *mut acpi_table_header,
    cbom_size: *mut u32,
    cboz_size: *mut u32,
    cbop_size: *mut u32,
) {
    if acpi_disabled { return; }

    let rhct = if !table.is_null() { table as *mut acpi_table_rhct } else {
        let value = acpi_get_rhct();
        if value.is_null() { return; }
        value
    };

    if !cbom_size.is_null() { *cbom_size = 0; }
    if !cboz_size.is_null() { *cboz_size = 0; }
    if !cbop_size.is_null() { *cbop_size = 0; }

    let mut node = (rhct as *mut u8).add((*rhct).node_offset as usize)
        as *mut acpi_rhct_node_header;
    let end = (rhct as *mut u8).add((*rhct).header.length as usize)
        as *mut acpi_rhct_node_header;
    while node < end {
        if (*node).type_ == ACPI_RHCT_NODE_TYPE_HART_INFO {
            let hart_info = (node as *mut u8).add(core::mem::size_of::<acpi_rhct_node_header>())
                as *mut acpi_rhct_hart_info;
            acpi_parse_hart_info_cmo_node(rhct, hart_info, cbom_size, cboz_size, cbop_size);
        }
        node = (node as *mut u8).add((*node).length as usize)
            as *mut acpi_rhct_node_header;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
