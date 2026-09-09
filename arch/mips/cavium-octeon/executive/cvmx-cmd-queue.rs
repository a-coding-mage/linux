/***********************license start***************
 * Author: Cavium Networks
 *
 * Contact: support@caviumnetworks.com
 * This file is part of the OCTEON SDK
 *
 * Copyright (c) 2003-2008 Cavium Networks
 *
 * This file is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License, Version 2, as
 * published by the Free Software Foundation.
 ***********************license end**************************************/

/* Support functions for managing command queues used for various hardware blocks. */

// C header dependencies are supplied by the surrounding translation unit.

pub static mut __cvmx_cmd_queue_state_ptr: *mut __cvmx_cmd_queue_all_state_t = core::ptr::null_mut();

unsafe fn __cvmx_cmd_queue_init_state_ptr() -> cvmx_cmd_queue_result_t {
    let alloc_name: *mut i8 = b"cvmx_cmd_queues\0".as_ptr() as *mut i8;
    extern "C" {
        static mut octeon_reserve32_memory: u64;
    }

    if !__cvmx_cmd_queue_state_ptr.is_null() {
        return CVMX_CMD_QUEUE_SUCCESS;
    }

    if octeon_reserve32_memory != 0 {
        __cvmx_cmd_queue_state_ptr = cvmx_bootmem_alloc_named_range(
            core::mem::size_of::<__cvmx_cmd_queue_all_state_t>(),
            octeon_reserve32_memory,
            octeon_reserve32_memory + ((CONFIG_CAVIUM_RESERVE32 as u64) << 20) - 1,
            128,
            alloc_name,
        ) as *mut __cvmx_cmd_queue_all_state_t;
    } else {
        __cvmx_cmd_queue_state_ptr = cvmx_bootmem_alloc_named(
            core::mem::size_of::<__cvmx_cmd_queue_all_state_t>(),
            128,
            alloc_name,
        ) as *mut __cvmx_cmd_queue_all_state_t;
    }

    if !__cvmx_cmd_queue_state_ptr.is_null() {
        core::ptr::write_bytes(
            __cvmx_cmd_queue_state_ptr as *mut u8,
            0,
            core::mem::size_of::<__cvmx_cmd_queue_all_state_t>(),
        );
    } else {
        let block_desc = cvmx_bootmem_find_named_block(alloc_name);
        if !block_desc.is_null() {
            __cvmx_cmd_queue_state_ptr = cvmx_phys_to_ptr((*block_desc).base_addr)
                as *mut __cvmx_cmd_queue_all_state_t;
        } else {
            cvmx_dprintf(b"ERROR: cvmx_cmd_queue_initialize: Unable to get named block %s.\n\0".as_ptr() as *const i8, alloc_name);
            return CVMX_CMD_QUEUE_NO_MEMORY;
        }
    }
    CVMX_CMD_QUEUE_SUCCESS
}

pub unsafe fn cvmx_cmd_queue_initialize(
    queue_id: cvmx_cmd_queue_id_t,
    max_depth: i32,
    fpa_pool: i32,
    pool_size: i32,
) -> cvmx_cmd_queue_result_t {
    let result = __cvmx_cmd_queue_init_state_ptr();
    if result != CVMX_CMD_QUEUE_SUCCESS { return result; }

    let qstate = __cvmx_cmd_queue_get_state(queue_id);
    if qstate.is_null() { return CVMX_CMD_QUEUE_INVALID_PARAM; }

    if CVMX_CMD_QUEUE_ENABLE_MAX_DEPTH {
        if max_depth < 0 || max_depth > (1 << 20) { return CVMX_CMD_QUEUE_INVALID_PARAM; }
    } else if max_depth != 0 { return CVMX_CMD_QUEUE_INVALID_PARAM; }
    if fpa_pool < 0 || fpa_pool > 7 { return CVMX_CMD_QUEUE_INVALID_PARAM; }
    if pool_size < 128 || pool_size > 65536 { return CVMX_CMD_QUEUE_INVALID_PARAM; }

    if (*qstate).base_ptr_div128 != 0 {
        if max_depth != (*qstate).max_depth as i32 || fpa_pool != (*qstate).fpa_pool {
            return CVMX_CMD_QUEUE_INVALID_PARAM;
        }
        if ((pool_size >> 3) - 1) as u32 != (*qstate).pool_size_m1 {
            return CVMX_CMD_QUEUE_INVALID_PARAM;
        }
        CVMX_SYNCWS!();
        return CVMX_CMD_QUEUE_ALREADY_SETUP;
    }

    let status = cvmx_read_csr(CVMX_FPA_CTL_STATUS);
    if !status.s.enb { return CVMX_CMD_QUEUE_NO_MEMORY; }
    let buffer = cvmx_fpa_alloc(fpa_pool);
    if buffer.is_null() { return CVMX_CMD_QUEUE_NO_MEMORY; }

    core::ptr::write_bytes(qstate as *mut u8, 0, core::mem::size_of::<__cvmx_cmd_queue_state_t>());
    (*qstate).max_depth = max_depth as u32;
    (*qstate).fpa_pool = fpa_pool;
    (*qstate).pool_size_m1 = ((pool_size >> 3) - 1) as u32;
    (*qstate).base_ptr_div128 = cvmx_ptr_to_phys(buffer) / 128;
    (*__cvmx_cmd_queue_state_ptr).ticket[__cvmx_cmd_queue_get_index(queue_id)] = 0;
    CVMX_SYNCWS!();
    CVMX_CMD_QUEUE_SUCCESS
}

pub unsafe fn cvmx_cmd_queue_shutdown(queue_id: cvmx_cmd_queue_id_t) -> cvmx_cmd_queue_result_t {
    let qptr = __cvmx_cmd_queue_get_state(queue_id);
    if qptr.is_null() { return CVMX_CMD_QUEUE_INVALID_PARAM; }
    if cvmx_cmd_queue_length(queue_id) > 0 { return CVMX_CMD_QUEUE_FULL; }
    __cvmx_cmd_queue_lock(queue_id, qptr);
    if (*qptr).base_ptr_div128 != 0 {
        cvmx_fpa_free(cvmx_phys_to_ptr((*qptr).base_ptr_div128 << 7), (*qptr).fpa_pool, 0);
        (*qptr).base_ptr_div128 = 0;
    }
    __cvmx_cmd_queue_unlock(qptr);
    CVMX_CMD_QUEUE_SUCCESS
}

pub unsafe fn cvmx_cmd_queue_length(queue_id: cvmx_cmd_queue_id_t) -> i32 {
    if CVMX_ENABLE_PARAMETER_CHECKING && __cvmx_cmd_queue_get_state(queue_id).is_null() {
        return CVMX_CMD_QUEUE_INVALID_PARAM as i32;
    }
    match queue_id & 0xff0000 {
        CVMX_CMD_QUEUE_PKO_BASE => {
            cvmx_write_csr(CVMX_PKO_REG_READ_IDX, queue_id & 0xffff);
            if OCTEON_IS_MODEL(OCTEON_CN3XXX) {
                cvmx_read_csr(CVMX_PKO_MEM_DEBUG9).cn38xx.doorbell as i32
            } else {
                cvmx_read_csr(CVMX_PKO_MEM_DEBUG8).cn50xx.doorbell as i32
            }
        }
        CVMX_CMD_QUEUE_ZIP | CVMX_CMD_QUEUE_DFA | CVMX_CMD_QUEUE_RAID => 0,
        CVMX_CMD_QUEUE_DMA_BASE => cvmx_read_csr(CVMX_PEXP_NPEI_DMAX_COUNTS(queue_id & 7)).s.dbell as i32,
        CVMX_CMD_QUEUE_END => CVMX_CMD_QUEUE_INVALID_PARAM as i32,
        _ => CVMX_CMD_QUEUE_INVALID_PARAM as i32,
    }
}

pub unsafe fn cvmx_cmd_queue_buffer(queue_id: cvmx_cmd_queue_id_t) -> *mut core::ffi::c_void {
    let qptr = __cvmx_cmd_queue_get_state(queue_id);
    if !qptr.is_null() && (*qptr).base_ptr_div128 != 0 {
        cvmx_phys_to_ptr((*qptr).base_ptr_div128 << 7)
    } else { core::ptr::null_mut() }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
