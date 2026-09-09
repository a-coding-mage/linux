/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/* Copyright (c) 2018 Mellanox Technologies. */

// Translated from the C header `eq.h`.

pub const MLX5_NUM_CMD_EQE: usize = 32;
pub const MLX5_NUM_ASYNC_EQE: usize = 0x1000;
pub const MLX5_NUM_SPARE_EQE: usize = 0x80;

pub struct mlx5_eq;
pub struct mlx5_irq;
pub struct mlx5_core_dev;
pub struct mlx5_eqe;
pub struct notifier_block;

#[repr(C)]
pub struct mlx5_eq_param {
    pub nent: core::ffi::c_int,
    pub mask: [u64; 4],
    pub irq: *mut mlx5_irq,
}

extern "C" {
    pub fn mlx5_eq_create_generic(
        dev: *mut mlx5_core_dev,
        param: *mut mlx5_eq_param,
    ) -> *mut mlx5_eq;
    pub fn mlx5_eq_destroy_generic(dev: *mut mlx5_core_dev, eq: *mut mlx5_eq)
        -> core::ffi::c_int;
    pub fn mlx5_eq_enable(
        dev: *mut mlx5_core_dev,
        eq: *mut mlx5_eq,
        nb: *mut notifier_block,
    ) -> core::ffi::c_int;
    pub fn mlx5_eq_disable(
        dev: *mut mlx5_core_dev,
        eq: *mut mlx5_eq,
        nb: *mut notifier_block,
    );
    pub fn mlx5_eq_get_eqe(eq: *mut mlx5_eq, cc: u32) -> *mut mlx5_eqe;
    pub fn mlx5_eq_update_ci(eq: *mut mlx5_eq, cc: u32, arm: bool);
}

/* The HCA will think the queue has overflowed if we
 * don't tell it we've been processing events.  We
 * create EQs with MLX5_NUM_SPARE_EQE extra entries,
 * so we must update our consumer index at
 * least that often.
 *
 * mlx5_eq_update_cc must be called on every EQE @EQ irq handler
 */
#[inline]
pub unsafe fn mlx5_eq_update_cc(eq: *mut mlx5_eq, mut cc: u32) -> u32 {
    if cc >= MLX5_NUM_SPARE_EQE as u32 {
        mlx5_eq_update_ci(eq, cc, false);
        cc = 0;
    }
    cc
}

#[repr(C)]
pub struct mlx5_nb {
    pub nb: notifier_block,
    pub event_type: u8,
}

// C equivalent of container_of(container_of(ptr, struct mlx5_nb, nb), type, member).
#[macro_export]
macro_rules! mlx5_nb_cof {
    ($ptr:expr, $type:ty, $member:ident) => {{
        let __nb: *mut $crate::mlx5_nb = $ptr as *mut $crate::mlx5_nb;
        let __base = __nb as *mut u8;
        (__base.wrapping_sub(core::mem::offset_of!($crate::mlx5_nb, nb))
            .wrapping_sub(core::mem::offset_of!($type, $member))) as *mut $type
    }};
}

// MLX5_NB_INIT(name, handler, event)
#[macro_export]
macro_rules! MLX5_NB_INIT {
    ($name:expr, $handler:expr, $event:ident) => {{
        ($name).nb.notifier_call = $handler;
        ($name).event_type = MLX5_EVENT_TYPE_$event;
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
