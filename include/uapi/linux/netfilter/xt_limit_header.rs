/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Dependency: __u32 is supplied by the translated linux types definitions. */

/* timings are in milliseconds. */
pub const XT_LIMIT_SCALE: u32 = 10000;

pub struct xt_limit_priv;

/* 1/10,000 sec period => max of 10,000/sec.  Min rate is then 429490
   seconds, or one every 59 hours. */
#[repr(C)]
pub struct xt_rateinfo {
	pub avg: u32,    /* Average secs between packets * scale */
	pub burst: u32,  /* Period multiplier for upper limit. */

	/* Used internally by the kernel */
	pub prev: usize, /* moved to xt_limit_priv */
	pub credit: u32, /* moved to xt_limit_priv */
	pub credit_cap: u32,
	pub cost: u32,

	pub master: *mut xt_limit_priv,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
