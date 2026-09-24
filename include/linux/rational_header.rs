// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2009 emlix GmbH, Oskar Schirmer <oskar@scara.com>

//! Safe rational approximation without foreign declarations or symbol owners.

#[path = "../../lib/math/rational.rs"]
mod fractions;

pub use fractions::rational_best_approximation;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
