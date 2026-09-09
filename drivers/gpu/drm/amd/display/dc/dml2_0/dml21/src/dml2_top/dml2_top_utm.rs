// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

unsafe fn dml2_top_backup_worksheet(
    dml: *mut dml2_instance,
    worksheet: *const dml2_optimization_worksheet,
) {
    core::ptr::copy_nonoverlapping(
        worksheet,
        &mut (*dml).scratch.worksheet_backup,
        1,
    );
}

unsafe fn dml2_top_restore_worksheet(
    dml: *mut dml2_instance,
    worksheet: *mut dml2_optimization_worksheet,
) {
    core::ptr::copy_nonoverlapping(
        &(*dml).scratch.worksheet_backup,
        worksheet,
        1,
    );
}

unsafe fn dml2_top_validate_worksheet(
    dml: *mut dml2_instance,
    worksheet: *mut dml2_optimization_worksheet,
) -> dml2_status {
    let pmo = &mut (*dml).pmo_instance;
    let core = &mut (*dml).core_instance;
    let solution = &mut (*dml).scratch.solution;
    let mut status;

    status = (pmo.optional_sanity_check)(pmo, worksheet);
    if status == DML2_STATUS_OK {
        (pmo.convert_worksheet_to_solution)(pmo, worksheet, solution);
        status = (core.validate_solution)(
            core,
            solution,
            &mut (*worksheet).validation_result,
        );
        (pmo.clear_pre_validation_states)(pmo, worksheet);
    } else {
        (*worksheet).validation_result.is_mode_support_valid = false;
    }
    status
}

unsafe fn dml2_top_perform_stage_optimization(
    dml: *mut dml2_instance,
    optimizer: *mut dml2_pmo_stage_optimizer,
    worksheet: *mut dml2_optimization_worksheet,
) -> dml2_status {
    const MAX_OPTIMIZATION_ITERATIONS: u32 = 20;
    let mut iteration: u32 = 0;
    let mut cur_validate_status = DML2_STATUS_OK;
    let mut cur_optimize_status = DML2_STATUS_UNKNOWN;
    let mut is_permissible_found = false;

    dml2_top_backup_worksheet(dml, worksheet);
    ((*optimizer).init)(optimizer, worksheet);

    while ((*optimizer).optimize_next)(optimizer, worksheet) {
        iteration = iteration.wrapping_add(1);
        if iteration >= MAX_OPTIMIZATION_ITERATIONS {
            cur_optimize_status = DML2_STATUS_OPTIMIZE_FAIL_EXCEED_MAX_ITERATION;
            break;
        }
        cur_optimize_status = DML2_STATUS_UNKNOWN;
        cur_validate_status = dml2_top_validate_worksheet(dml, worksheet);
        if cur_validate_status == DML2_STATUS_OK {
            cur_optimize_status = ((*optimizer).test_permissibility)(optimizer, worksheet);
        }
        if cur_validate_status == DML2_STATUS_OK && cur_optimize_status == DML2_STATUS_OK {
            is_permissible_found = true;
            dml2_top_backup_worksheet(dml, worksheet);
        }
    }

    if cur_validate_status == DML2_STATUS_OK && cur_optimize_status == DML2_STATUS_UNKNOWN {
        cur_optimize_status = ((*optimizer).test_permissibility)(optimizer, worksheet);
        if cur_optimize_status == DML2_STATUS_OK {
            is_permissible_found = true;
        }
    }
    if cur_validate_status != DML2_STATUS_OK || cur_optimize_status != DML2_STATUS_OK {
        dml2_top_restore_worksheet(dml, worksheet);
    }

    DML_ASSERT_MSG(
        (*worksheet).validation_result.is_mode_support_valid
            && (*worksheet).validation_result.is_mcache_allocation_valid
            && (*worksheet).validation_result.is_prefetch_valid,
        "worksheet must be valid on exit independent from optmization resul!\n",
    );

    if is_permissible_found {
        DML2_STATUS_OK
    } else if cur_validate_status != DML2_STATUS_OK {
        cur_validate_status
    } else {
        cur_optimize_status
    }
}

unsafe fn dml2_top_build_and_validate_unoptimized_worksheet(
    dml: *mut dml2_instance,
    orig_dispcfg: *const dml2_display_cfg,
    worksheet: *mut dml2_optimization_worksheet,
) -> dml2_status {
    let mut status = DML2_STATUS_OK;
    let pmo = &mut (*dml).pmo_instance;
    let mut optimizers: [*mut dml2_pmo_stage_optimizer; dml2_pmo_stage_index_max] =
        [core::ptr::null_mut(); dml2_pmo_stage_index_max];
    let mut count: i32;
    let mut i: i32;

    if status == DML2_STATUS_OK {
        (pmo.initialize_worksheet)(pmo, orig_dispcfg, worksheet);
        status = dml2_top_validate_worksheet(dml, worksheet);
    }
    if status == DML2_STATUS_OK {
        count = (pmo.get_ordered_mandatory_stage_optimizers)(pmo, optimizers.as_mut_ptr());
        i = 0;
        while i < count {
            status = dml2_top_perform_stage_optimization(dml, optimizers[i as usize], worksheet);
            if status != DML2_STATUS_OK { break; }
            i += 1;
        }
    }
    status
}

unsafe fn dml2_top_optimize_worksheet(
    dml: *mut dml2_instance,
    worksheet: *mut dml2_optimization_worksheet,
) {
    let pmo = &mut (*dml).pmo_instance;
    let mut optimizers: [*mut dml2_pmo_stage_optimizer; dml2_pmo_stage_index_max] =
        [core::ptr::null_mut(); dml2_pmo_stage_index_max];
    let count: i32;
    let mut i: i32 = 0;

    DML_ASSERT!((*worksheet).validation_result.is_mode_support_valid);
    count = (pmo.get_ordered_optional_stage_optimizers)(pmo, optimizers.as_mut_ptr());
    while i < count {
        dml2_top_perform_stage_optimization(dml, optimizers[i as usize], worksheet);
        i += 1;
    }
}

unsafe fn dml2_top_map_minimum_clock_state(
    dml: *mut dml2_instance,
    solution: *mut dml2_display_solution,
    programming: *mut dml2_display_cfg_programming,
) -> dml2_status {
    let params = &mut (*dml).scratch.build_mode_programming_locals.dppm_map_mode_params;
    let dpmm = &mut (*dml).dpmm_instance;
    if dpmm.map_mode_to_soc_dpm.is_none() { return DML2_STATUS_OK; }
    params.utm_soc_bb = &mut (*dml).utm_soc_bb;
    params.ip = &mut (*dml).core_instance.clean_me_up.mode_lib.ip;
    params.solution = solution;
    params.programming = programming;
    if (dpmm.map_mode_to_soc_dpm.unwrap())(params) {
        DML2_STATUS_OK
    } else {
        DML2_STATUS_POPULATE_FAIL_MIN_CLOCK_STATE
    }
}

unsafe fn dml2_top_populate_mode_programming(
    dml: *mut dml2_instance,
    solution: *const dml2_display_solution,
    programming: *mut dml2_display_cfg_programming,
) -> dml2_status {
    let core = &mut (*dml).core_instance;
    (core.populate_programming)(core, solution, programming)
}

unsafe fn dml2_top_populate_informative(
    dml: *mut dml2_instance,
    status: dml2_status,
    programming: *mut dml2_display_cfg_programming,
) {
    let params = &mut (*dml).scratch.build_mode_programming_locals.informative_params;
    let core = &mut (*dml).core_instance;
    params.instance = core;
    params.programming = programming;
    params.mode_is_supported = status == DML2_STATUS_OK;
    params.instance.scratch.mode_programming_locals.mode_programming_ex_params.min_clk_index =
        (*dml).scratch.solution.sop_constraint.dcn5.min_sop_index;
    (params.instance.populate_informative)(params);

    if status == DML2_STATUS_POPULATE_FAIL_PROGRAMMING
        || status == DML2_STATUS_VALIDATE_FAIL_MODE_SUPPORT_PREFETCH
        || status == DML2_STATUS_VALIDATE_FAIL_MODE_SUPPORT_PREFETCH_URGENT {
        (*programming).informative.failed_mode_programming_prefetch = true;
    } else if status == DML2_STATUS_POPULATE_FAIL_PROGRAMMING_DCFCLK {
        (*programming).informative.failed_mode_programming_dcfclk = true;
    } else if status == DML2_STATUS_POPULATE_FAIL_PROGRAMMING_FLIP_BANDWIDTH
        || status == DML2_STATUS_VALIDATE_FAIL_MODE_SUPPORT_QOS_BANDWIDTH {
        (*programming).informative.failed_mode_programming_flip = true;
    } else if status == DML2_STATUS_POPULATE_FAIL_MIN_CLOCK_STATE {
        (*programming).informative.failed_dpmm = true;
    } else if status == DML2_STATUS_VALIDATE_FAIL_MCACHE
        || status == DML2_STATUS_OPTIMIZE_FAIL_MCACHE
        || status == DML2_STATUS_VALIDATE_FAIL_PMO_SANITY_TOTAL_PIPE_USAGE {
        (*programming).informative.failed_mcache_validation = true;
    } else if status == DML2_STATUS_OPTIMIZE_FAIL_UCLK_PSTATE {
        (*programming).informative.failed_uclk_pstate = true;
    } else if status == DML2_STATUS_VALIDATE_FAIL_PREFETCH {
        (*programming).informative.failed_prefetch = true;
    }
}

unsafe fn dml2_top_build_programming_for_worksheet(
    dml: *mut dml2_instance,
    worksheet: *const dml2_optimization_worksheet,
    programming: *mut dml2_display_cfg_programming,
) -> dml2_status {
    let mut status = DML2_STATUS_OK;
    let solution = &mut (*dml).scratch.solution;
    let pmo = &mut (*dml).pmo_instance;
    core::ptr::write_bytes(programming, 0, 1);
    (pmo.convert_worksheet_to_solution)(pmo, worksheet, solution);
    if status == DML2_STATUS_OK { status = dml2_top_map_minimum_clock_state(dml, solution, programming); }
    if status == DML2_STATUS_OK { status = dml2_top_populate_mode_programming(dml, solution, programming); }
    status
}

unsafe fn dml2_top_utm_check_mode_supported(in_out: *mut dml2_check_mode_supported_in_out) -> bool {
    let dml = (*in_out).dml2_instance;
    let status = dml2_top_build_and_validate_unoptimized_worksheet(
        dml, (*in_out).display_config, &mut (*dml).scratch.worksheet,
    );
    (*in_out).is_supported = status == DML2_STATUS_OK;
    true
}

unsafe fn dml2_top_utm_build_mode_programming(in_out: *mut dml2_build_mode_programming_in_out) -> bool {
    let dml = (*in_out).dml2_instance;
    let worksheet = &mut (*dml).scratch.worksheet;
    let mut status = dml2_top_build_and_validate_unoptimized_worksheet(dml, (*in_out).display_config, worksheet);
    if status == DML2_STATUS_OK {
        dml2_top_optimize_worksheet(dml, worksheet);
        status = dml2_top_build_programming_for_worksheet(dml, worksheet, (*in_out).programming);
    }
    dml2_top_populate_informative(dml, status, (*in_out).programming);
    status == DML2_STATUS_OK
}

static mut utm_funcs: dml2_top_funcs = dml2_top_funcs {
    check_mode_supported: Some(dml2_top_utm_check_mode_supported),
    build_mode_programming: Some(dml2_top_utm_build_mode_programming),
    build_mcache_programming: Some(dml2_top_soc15_build_mcache_programming),
};

pub unsafe fn dml2_top_utm_initialize_instance(
    in_out: *mut dml2_initialize_instance_in_out,
) -> bool {
    let dml = (*in_out).dml2_instance;
    let mut core_init_params: dml2_core_initialize_in_out = core::mem::zeroed();
    let mut pmo_init_params: dml2_pmo_initialize_in_out = core::mem::zeroed();
    let mut cga_init_params: dml2_cga_initialize_in_out = core::mem::zeroed();
    let mut result = true;

    core::ptr::write_bytes(dml, 0, 1);
    if result {
        core::ptr::copy_nonoverlapping(&(*in_out).ip_caps, &mut (*dml).ip_caps, 1);
        (*dml).project_id = (*in_out).options.project_id;
        (*dml).pmo_options = (*in_out).options.pmo_options;
        (*dml).funcs = utm_funcs;
    }
    if result { result = dml2_dpmm_create((*in_out).options.project_id, &mut (*dml).dpmm_instance); }
    if result { result = dml2_core_create((*in_out).options.project_id, &mut (*dml).core_instance); }
    if result { result = dml2_pmo_create((*in_out).options.project_id, &mut (*dml).pmo_instance); }
    if result {
        result = dml2_utm_soc_bb_create(
            (*in_out).options.project_id, &mut (*dml).utm_soc_bb,
            &mut (*in_out).soc_bb, (*in_out).overrides.explicit_qos_model,
        );
    }
    if result { result = dml2_cga_create((*in_out).options.project_id, &mut (*dml).clock_adjuster); }
    if result {
        core_init_params.project_id = (*in_out).options.project_id;
        core_init_params.instance = &mut (*dml).core_instance;
        core_init_params.explicit_ip_bb = (*in_out).overrides.explicit_ip_bb;
        core_init_params.explicit_ip_bb_size = (*in_out).overrides.explicit_ip_bb_size;
        core_init_params.ip_caps = &mut (*dml).ip_caps;
        core_init_params.utm_soc_bb = &mut (*dml).utm_soc_bb;
        core_init_params.clock_adjuster = &mut (*dml).clock_adjuster;
        result = ((*dml).core_instance.initialize)(&mut core_init_params);
    }
    if result {
        pmo_init_params.instance = &mut (*dml).pmo_instance;
        pmo_init_params.ip_caps = &mut (*dml).ip_caps;
        pmo_init_params.utm_soc_bb = &mut (*dml).utm_soc_bb;
        pmo_init_params.options = &mut (*dml).pmo_options;
        ((*dml).pmo_instance.initialize)(&mut pmo_init_params);
    }
    if result && (*dml).clock_adjuster.initialize.is_some() {
        cga_init_params.adjuster = &mut (*dml).clock_adjuster;
        cga_init_params.soc_bb = &mut (*in_out).soc_bb;
        cga_init_params.ip = &mut (*dml).core_instance.clean_me_up.mode_lib.ip;
        ((*dml).clock_adjuster.initialize.unwrap())(&mut cga_init_params);
    }
    result
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
