// SPDX-License-Identifier: GPL-2.0
/*
 * kabi_ex.c
 *
 * Copyright (C) 2024 Google LLC
 *
 * Examples for kABI stability features with --stable. See kabi_ex.h
 * for details.
 */

// Dependency declarations from kabi_ex.h are supplied by other files.

pub static mut e0: s = unsafe { core::mem::zeroed() };
pub static mut e1: e = unsafe { core::mem::zeroed() };

pub static mut ex0a: ex0a = unsafe { core::mem::zeroed() };
pub static mut ex0b: ex0b = unsafe { core::mem::zeroed() };
pub static mut ex0c: ex0c = unsafe { core::mem::zeroed() };

pub static mut ex1a: ex1a = unsafe { core::mem::zeroed() };
pub static mut ex1b: ex1b = unsafe { core::mem::zeroed() };
pub static mut ex1c: ex1c = unsafe { core::mem::zeroed() };

pub static mut ex2a: ex2a = unsafe { core::mem::zeroed() };
pub static mut ex2b: ex2b = unsafe { core::mem::zeroed() };
pub static mut ex2c: ex2c = unsafe { core::mem::zeroed() };

pub static mut ex3a: ex3a = unsafe { core::mem::zeroed() };
pub static mut ex3b: ex3b = unsafe { core::mem::zeroed() };
pub static mut ex3c: ex3c = unsafe { core::mem::zeroed() };

pub static mut ex4a: ex4a = unsafe { core::mem::zeroed() };

pub static mut ex5a: ex5a = unsafe { core::mem::zeroed() };
pub static mut ex5b: ex5b = unsafe { core::mem::zeroed() };

pub static mut ex6a: i32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
