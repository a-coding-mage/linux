/* SPDX-License-Identifier: GPL-2.0 */

// External dependency: `struct rv_monitor` is declared by another translation unit.
extern "C" {
    pub static mut rv_sched: rv_monitor;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
