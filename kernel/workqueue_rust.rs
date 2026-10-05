// SPDX-License-Identifier: GPL-2.0-only
//! Native workqueue algorithm owner under source review.
//!
//! Authority: kernel/workqueue.c at 126a30fae3bba11420ec2fcbde51a0a01bab1b5b.
//! This new entrypoint preserves both the historical kernel/workqueue.rs
//! comment artifact and the working rust/kernel/workqueue.rs client API.
#![no_std]

compile_error!("SOURCE ONLY HOLD: workqueue native owner is incomplete and not admitted");

#[path = "workqueue_rust/common.rs"]
mod common;
