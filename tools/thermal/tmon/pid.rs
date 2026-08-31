// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * pid.c PID controller for testing cooling devices
 *
 * Copyright (C) 2012 Intel Corporation. All rights reserved.
 *
 * Author Name Jacob Pan <jacob.jun.pan@linux.intel.com>
 */

// C includes removed; declarations from "tmon.h" and libc are expected from
// surrounding bindings when integrated.

use std::os::raw::{c_char, c_double, c_int};

/**************************************************************************
 * PID (Proportional-Integral-Derivative) controller is commonly used in
 * linear control system, consider the process.
 * G(s) = U(s)/E(s)
 * kp = proportional gain
 * ki = integral gain
 * kd = derivative gain
 * Ts
 * We use type C Alan Bradley equation which takes set point off the
 * output dependency in P and D term.
 *
 *   y[k] = y[k-1] - kp*(x[k] - x[k-1]) + Ki*Ts*e[k] - Kd*(x[k]
 *          - 2*x[k-1]+x[k-2])/Ts
 *
 *
 ***********************************************************************/
#[repr(C)]
pub struct pid_params {
    pub ts: c_double,
    pub kp: c_double,
    pub ki: c_double,
    pub kd: c_double,
    pub t_target: c_double,
    pub y_k: c_double,
}

pub static mut p_param: pid_params = pid_params {
    ts: 0.0,
    kp: 0.0,
    ki: 0.0,
    kd: 0.0,
    t_target: 0.0,
    y_k: 0.0,
};

/* cached data from previous loop */
static mut xk_1: c_double = 0.0;
static mut xk_2: c_double = 0.0; /* input temperature x[k-#] */

unsafe extern "C" {
    static mut ticktime: c_double;
    static mut target_temp_user: c_double;
    static LOG_DEBUG: c_int;
    static LIMIT_HIGH: c_double;
    static LIMIT_LOW: c_double;

    fn syslog(priority: c_int, format: *const c_char, ...);
    fn set_ctrl_state(state: c_int);
}

/*
 * TODO: make PID parameters tuned automatically,
 * 1. use CPU burn to produce open loop unit step response
 * 2. calculate PID based on Ziegler-Nichols rule
 *
 * add a flag for tuning PID
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_thermal_controller() -> c_int {
    unsafe {
        /* init pid params */
        p_param.ts = ticktime;
        /* TODO: get it from TUI tuning tab */
        p_param.kp = 0.36;
        p_param.ki = 5.0;
        p_param.kd = 0.19;

        p_param.t_target = target_temp_user;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn controller_reset() {
    unsafe {
        /* TODO: relax control data when not over thermal limit */
        syslog(
            LOG_DEBUG,
            b"TC inactive, relax p-state\n\0".as_ptr() as *const c_char,
        );
        p_param.y_k = 0.0;
        xk_1 = 0.0;
        xk_2 = 0.0;
        set_ctrl_state(0);
    }
}

/* To be called at time interval Ts. Type C PID controller.
 *    y[k] = y[k-1] - kp*(x[k] - x[k-1]) + Ki*Ts*e[k] - Kd*(x[k]
 *          - 2*x[k-1]+x[k-2])/Ts
 * TODO: add low pass filter for D term
 */
const GUARD_BAND: c_int = 2;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn controller_handler(xk: c_double, yk: *mut c_double) {
    unsafe {
        let ek: c_double;
        let p_term: c_double;
        let i_term: c_double;
        let d_term: c_double;

        ek = p_param.t_target - xk; /* error */
        if ek >= 3.0 {
            syslog(
                LOG_DEBUG,
                b"PID: %3.1f Below set point %3.1f, stop\n\0".as_ptr() as *const c_char,
                xk,
                p_param.t_target,
            );
            controller_reset();
            *yk = 0.0;
            return;
        }
        /* compute intermediate PID terms */
        p_term = -p_param.kp * (xk - xk_1);
        i_term = p_param.kp * p_param.ki * p_param.ts * ek;
        d_term = -p_param.kp * p_param.kd * (xk - 2.0 * xk_1 + xk_2) / p_param.ts;
        /* compute output */
        *yk += p_term + i_term + d_term;
        /* update sample data */
        xk_1 = xk;
        xk_2 = xk_1;

        /* clamp output adjustment range */
        if *yk < -LIMIT_HIGH {
            *yk = -LIMIT_HIGH;
        } else if *yk > -LIMIT_LOW {
            *yk = -LIMIT_LOW;
        }

        p_param.y_k = *yk;

        set_ctrl_state(p_param.y_k.abs().round() as c_int);
    }
}
