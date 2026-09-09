// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
/*
 * Copyright (c) 2019, Mellanox Technologies inc.  All rights reserved.
 */

// Dependency declarations supplied by the Linux DIM implementation are
// intentionally left external to this translation.

unsafe fn rdma_dim_step(dim: *mut dim) -> i32 {
    if (*dim).tune_state == DIM_GOING_RIGHT {
        if (*dim).profile_ix == RDMA_DIM_PARAMS_NUM_PROFILES - 1 {
            return DIM_ON_EDGE;
        }
        (*dim).profile_ix += 1;
        (*dim).steps_right += 1;
    }
    if (*dim).tune_state == DIM_GOING_LEFT {
        if (*dim).profile_ix == 0 {
            return DIM_ON_EDGE;
        }
        (*dim).profile_ix -= 1;
        (*dim).steps_left += 1;
    }

    DIM_STEPPED
}

unsafe fn rdma_dim_stats_compare(curr: *mut dim_stats, prev: *mut dim_stats) -> i32 {
    /* first stat */
    if (*prev).cpms == 0 {
        return DIM_STATS_SAME;
    }

    if IS_SIGNIFICANT_DIFF((*curr).cpms, (*prev).cpms) {
        return if (*curr).cpms > (*prev).cpms {
            DIM_STATS_BETTER
        } else {
            DIM_STATS_WORSE
        };
    }

    if IS_SIGNIFICANT_DIFF((*curr).cpe_ratio, (*prev).cpe_ratio) {
        return if (*curr).cpe_ratio > (*prev).cpe_ratio {
            DIM_STATS_BETTER
        } else {
            DIM_STATS_WORSE
        };
    }

    DIM_STATS_SAME
}

unsafe fn rdma_dim_decision(curr_stats: *mut dim_stats, dim: *mut dim) -> bool {
    let prev_ix = (*dim).profile_ix;
    let state = (*dim).tune_state;
    let mut stats_res;
    let mut step_res;

    if state != DIM_PARKING_ON_TOP && state != DIM_PARKING_TIRED {
        stats_res = rdma_dim_stats_compare(curr_stats, &mut (*dim).prev_stats);

        match stats_res {
            DIM_STATS_SAME => {
                if (*curr_stats).cpe_ratio <= 50 * prev_ix {
                    (*dim).profile_ix = 0;
                }
            }
            DIM_STATS_WORSE => {
                dim_turn(dim);
                step_res = rdma_dim_step(dim);
                if step_res == DIM_ON_EDGE {
                    dim_turn(dim);
                }
            }
            DIM_STATS_BETTER => {
                step_res = rdma_dim_step(dim);
                if step_res == DIM_ON_EDGE {
                    dim_turn(dim);
                }
            }
            _ => {}
        }
    }

    (*dim).prev_stats = *curr_stats;

    (*dim).profile_ix != prev_ix
}

pub unsafe fn rdma_dim(dim: *mut dim, completions: u64) {
    let curr_sample = &mut (*dim).measuring_sample as *mut dim_sample;
    let mut curr_stats: dim_stats;
    let mut nevents: u32;

    dim_update_sample_with_comps(
        (*curr_sample).event_ctr + 1,
        0,
        0,
        (*curr_sample).comp_ctr + completions,
        &mut (*dim).measuring_sample,
    );

    match (*dim).state {
        DIM_MEASURE_IN_PROGRESS => {
            nevents = (*curr_sample).event_ctr - (*dim).start_sample.event_ctr;
            if nevents < DIM_NEVENTS {
                return;
            }
            if !dim_calc_stats(&(*dim).start_sample, curr_sample, &mut curr_stats) {
                return;
            }
            if rdma_dim_decision(&mut curr_stats, dim) {
                (*dim).state = DIM_APPLY_NEW_PROFILE;
                schedule_work(&mut (*dim).work);
                return;
            }
            (*dim).state = DIM_MEASURE_IN_PROGRESS;
            dim_update_sample_with_comps(
                (*curr_sample).event_ctr,
                0,
                0,
                (*curr_sample).comp_ctr,
                &mut (*dim).start_sample,
            );
        }
        DIM_START_MEASURE => {
            (*dim).state = DIM_MEASURE_IN_PROGRESS;
            dim_update_sample_with_comps(
                (*curr_sample).event_ctr,
                0,
                0,
                (*curr_sample).comp_ctr,
                &mut (*dim).start_sample,
            );
        }
        DIM_APPLY_NEW_PROFILE => {}
        _ => {}
    }
}

// EXPORT_SYMBOL(rdma_dim);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
