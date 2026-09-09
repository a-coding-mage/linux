// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2017 SiFive
 */

// C dependencies are supplied by the surrounding kernel translation unit.

#[cfg(CONFIG_SMP)]
unsafe fn ipi_remote_fence_i(_info: *mut core::ffi::c_void) {
    local_flush_icache_all();
}

#[cfg(CONFIG_SMP)]
pub unsafe fn flush_icache_all() {
    local_flush_icache_all();

    if num_online_cpus() < 2 {
        return;
    }

    // Order previous D$ writes before triggering the remote fence.i.
    riscv_fence_w_o();

    if riscv_use_sbi_for_rfence() {
        sbi_remote_fence_i(core::ptr::null_mut());
    } else {
        on_each_cpu(Some(ipi_remote_fence_i), core::ptr::null_mut(), 1);
    }
}

#[cfg(CONFIG_SMP)]
pub unsafe fn flush_icache_mm(mm: *mut mm_struct, mut local: bool) {
    let cpu: core::ffi::c_uint;
    let mut others = core::mem::MaybeUninit::<cpumask_t>::uninit();
    let mask: *mut cpumask_t;

    preempt_disable();

    // Mark every hart's icache as needing a flush for this MM.
    mask = &mut (*mm).context.icache_stale_mask;
    cpumask_setall(mask);
    // Flush this hart's I$ now, and mark it as flushed.
    cpu = smp_processor_id();
    cpumask_clear_cpu(cpu, mask);
    local_flush_icache_all();

    // Flush the I$ of other harts concurrently executing, and mark them as flushed.
    cpumask_andnot(others.as_mut_ptr(), mm_cpumask(mm), cpumask_of(cpu));
    local |= cpumask_empty(others.as_mut_ptr());
    if mm == (*current).active_mm && local {
        smp_mb();
    } else if riscv_use_sbi_for_rfence() {
        sbi_remote_fence_i(others.as_mut_ptr());
    } else {
        on_each_cpu_mask(others.as_mut_ptr(), Some(ipi_remote_fence_i), core::ptr::null_mut(), 1);
    }

    preempt_enable();
}

#[cfg(CONFIG_MMU)]
pub unsafe fn flush_icache_pte(mm: *mut mm_struct, pte: pte_t) {
    let folio = page_folio(pte_page(pte));

    if !test_bit(PG_dcache_clean, &(*folio).flags.f) {
        flush_icache_mm(mm, false);
        set_bit(PG_dcache_clean, &mut (*folio).flags.f);
    }
}

pub static mut riscv_cbom_block_size: core::ffi::c_uint = 0;
pub static mut riscv_cboz_block_size: core::ffi::c_uint = 0;
pub static mut riscv_cbop_block_size: core::ffi::c_uint = 0;

unsafe fn cbo_get_block_size(
    node: *mut device_node,
    name: *const core::ffi::c_char,
    block_size: *mut u32,
    first_hartid: *mut c_ulong,
) {
    let mut hartid: c_ulong = 0;
    let mut val: u32 = 0;

    if riscv_of_processor_hartid(node, &mut hartid) != 0 {
        return;
    }
    if of_property_read_u32(node, name, &mut val) != 0 {
        return;
    }
    if *block_size == 0 {
        *block_size = val;
        *first_hartid = hartid;
    } else if *block_size != val {
        pr_warn(name, *first_hartid, hartid);
    }
}

pub unsafe fn riscv_init_cbo_blocksizes() {
    let mut cbom_hartid: c_ulong = 0;
    let mut cboz_hartid: c_ulong = 0;
    let mut cbop_hartid: c_ulong = 0;
    let mut cbom_block_size: u32 = 0;
    let mut cboz_block_size: u32 = 0;
    let mut cbop_block_size: u32 = 0;
    let mut node: *mut device_node;
    let mut rhct: *mut acpi_table_header = core::ptr::null_mut();
    let mut status: acpi_status;

    if acpi_disabled {
        for_each_of_cpu_node!(node {
            cbo_get_block_size(node, c"riscv,cbom-block-size".as_ptr(), &mut cbom_block_size, &mut cbom_hartid);
            cbo_get_block_size(node, c"riscv,cboz-block-size".as_ptr(), &mut cboz_block_size, &mut cboz_hartid);
            cbo_get_block_size(node, c"riscv,cbop-block-size".as_ptr(), &mut cbop_block_size, &mut cbop_hartid);
        });
    } else {
        status = acpi_get_table(ACPI_SIG_RHCT, 0, &mut rhct);
        if acpi_failure(status) {
            return;
        }
        acpi_get_cbo_block_size(rhct, &mut cbom_block_size, &mut cboz_block_size, &mut cbop_block_size);
        acpi_put_table(rhct as *mut acpi_table_header);
    }

    if cbom_block_size != 0 { riscv_cbom_block_size = cbom_block_size; }
    if cboz_block_size != 0 { riscv_cboz_block_size = cboz_block_size; }
    if cbop_block_size != 0 { riscv_cbop_block_size = cbop_block_size; }
}

#[cfg(CONFIG_SMP)]
unsafe fn set_icache_stale_mask() {
    let cpu = get_cpu();
    let mask = &mut (*(*current).mm).context.icache_stale_mask;
    let stale_cpu = cpumask_test_cpu(cpu, mask);
    cpumask_setall(mask);
    assign_cpu(cpu, mask, stale_cpu);
    put_cpu();
}

pub unsafe fn riscv_set_icache_flush_ctx(ctx: c_ulong, scope: c_ulong) -> i32 {
    #[cfg(CONFIG_SMP)]
    {
        match ctx {
            PR_RISCV_CTX_SW_FENCEI_ON => match scope {
                PR_RISCV_SCOPE_PER_PROCESS => { (*(*current).mm).context.force_icache_flush = true; }
                PR_RISCV_SCOPE_PER_THREAD => { (*current).thread.force_icache_flush = true; }
                _ => return -EINVAL,
            },
            PR_RISCV_CTX_SW_FENCEI_OFF => match scope {
                PR_RISCV_SCOPE_PER_PROCESS => { set_icache_stale_mask(); (*(*current).mm).context.force_icache_flush = false; }
                PR_RISCV_SCOPE_PER_THREAD => { set_icache_stale_mask(); (*current).thread.force_icache_flush = false; }
                _ => return -EINVAL,
            },
            _ => return -EINVAL,
        }
        0
    }
    #[cfg(not(CONFIG_SMP))]
    {
        match ctx {
            PR_RISCV_CTX_SW_FENCEI_ON | PR_RISCV_CTX_SW_FENCEI_OFF => 0,
            _ => -EINVAL,
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
