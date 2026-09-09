/*
 * Copyright (c) 2013-2015, Mellanox Technologies, Ltd.  All rights reserved.
 *
 * This software is available to you under a choice of one of two
 * licenses.  You may choose to be licensed under the terms of the GNU
 * General Public License (GPL) Version 2, available from the file
 * COPYING in the main directory of this source tree, or the
 * OpenIB.org BSD license below:
 *
 *     Redistribution and use in source and binary forms, with or
 *     without modification, are permitted provided that the following
 *     conditions are met:
 *
 *      - Redistributions of source code must retain the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer.
 *
 *      - Redistributions in binary form must reproduce the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer in the documentation and/or other materials
 *        provided with the distribution.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

// Dependency supplied by linux/mlx5/driver.h.
#[repr(C)]
pub struct mlx5_core_dev {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn mlx5_core_alloc_transport_domain(dev: *mut mlx5_core_dev, tdn: *mut u32) -> i32;
    pub fn mlx5_core_dealloc_transport_domain(dev: *mut mlx5_core_dev, tdn: u32);
    pub fn mlx5_core_create_rq(
        dev: *mut mlx5_core_dev,
        input: *mut u32,
        inlen: i32,
        rqn: *mut u32,
    ) -> i32;
    pub fn mlx5_core_modify_rq(dev: *mut mlx5_core_dev, rqn: u32, input: *mut u32) -> i32;
    pub fn mlx5_core_destroy_rq(dev: *mut mlx5_core_dev, rqn: u32);
    pub fn mlx5_core_query_rq(dev: *mut mlx5_core_dev, rqn: u32, output: *mut u32) -> i32;
    pub fn mlx5_core_create_sq(
        dev: *mut mlx5_core_dev,
        input: *mut u32,
        inlen: i32,
        sqn: *mut u32,
    ) -> i32;
    pub fn mlx5_core_modify_sq(dev: *mut mlx5_core_dev, sqn: u32, input: *mut u32) -> i32;
    pub fn mlx5_core_destroy_sq(dev: *mut mlx5_core_dev, sqn: u32);
    pub fn mlx5_core_query_sq(dev: *mut mlx5_core_dev, sqn: u32, output: *mut u32) -> i32;
    pub fn mlx5_core_query_sq_state(dev: *mut mlx5_core_dev, sqn: u32, state: *mut u8) -> i32;
    pub fn mlx5_core_create_tir(dev: *mut mlx5_core_dev, input: *mut u32, tirn: *mut u32) -> i32;
    pub fn mlx5_core_modify_tir(dev: *mut mlx5_core_dev, tirn: u32, input: *mut u32) -> i32;
    pub fn mlx5_core_destroy_tir(dev: *mut mlx5_core_dev, tirn: u32);
    pub fn mlx5_core_create_tis(dev: *mut mlx5_core_dev, input: *mut u32, tisn: *mut u32) -> i32;
    pub fn mlx5_core_modify_tis(dev: *mut mlx5_core_dev, tisn: u32, input: *mut u32) -> i32;
    pub fn mlx5_core_destroy_tis(dev: *mut mlx5_core_dev, tisn: u32);
    pub fn mlx5_core_create_rqt(
        dev: *mut mlx5_core_dev,
        input: *mut u32,
        inlen: i32,
        rqtn: *mut u32,
    ) -> i32;
    pub fn mlx5_core_modify_rqt(
        dev: *mut mlx5_core_dev,
        rqtn: u32,
        input: *mut u32,
        inlen: i32,
    ) -> i32;
    pub fn mlx5_core_destroy_rqt(dev: *mut mlx5_core_dev, rqtn: u32);
}

#[repr(C)]
pub struct mlx5_hairpin_params {
    pub log_data_size: u8,
    pub log_num_packets: u8,
    pub q_counter: u16,
    pub num_channels: i32,
}

#[repr(C)]
pub struct mlx5_hairpin {
    pub func_mdev: *mut mlx5_core_dev,
    pub peer_mdev: *mut mlx5_core_dev,
    pub num_channels: i32,
    pub rqn: *mut u32,
    pub sqn: *mut u32,
    pub peer_gone: bool,
}

unsafe extern "C" {
    pub fn mlx5_core_hairpin_create(
        func_mdev: *mut mlx5_core_dev,
        peer_mdev: *mut mlx5_core_dev,
        params: *mut mlx5_hairpin_params,
    ) -> *mut mlx5_hairpin;
    pub fn mlx5_core_hairpin_destroy(pair: *mut mlx5_hairpin);
    pub fn mlx5_core_hairpin_clear_dead_peer(hp: *mut mlx5_hairpin);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
