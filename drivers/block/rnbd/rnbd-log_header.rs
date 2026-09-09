/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * RDMA Network Block Driver
 *
 * Copyright (c) 2014 - 2018 ProfitBricks GmbH. All rights reserved.
 * Copyright (c) 2018 - 2019 1&1 IONOS Cloud GmbH. All rights reserved.
 * Copyright (c) 2019 - 2020 1&1 IONOS SE. All rights reserved.
 */

// Dependencies supplied by the corresponding client and server headers.

macro_rules! rnbd_clt_log {
	($fn:ident, $dev:expr, $fmt:literal $(, $arg:expr)*) => {{
		$fn!(concat!("<%s@%s> ", $fmt), ($dev).pathname,
			($dev).sess.sessname $(, $arg)*)
	}};
}

macro_rules! rnbd_srv_log {
	($fn:ident, $dev:expr, $fmt:literal $(, $arg:expr)*) => {{
		$fn!(concat!("<%s@%s>: ", $fmt), ($dev).pathname,
			($dev).sess.sessname $(, $arg)*)
	}};
}

macro_rules! rnbd_clt_err {
	($dev:expr, $fmt:literal $(, $arg:expr)*) => {
		rnbd_clt_log!(pr_err, $dev, $fmt $(, $arg)*)
	};
}

macro_rules! rnbd_clt_err_rl {
	($dev:expr, $fmt:literal $(, $arg:expr)*) => {
		rnbd_clt_log!(pr_err_ratelimited, $dev, $fmt $(, $arg)*)
	};
}

macro_rules! rnbd_clt_info {
	($dev:expr, $fmt:literal $(, $arg:expr)*) => {
		rnbd_clt_log!(pr_info, $dev, $fmt $(, $arg)*)
	};
}

macro_rules! rnbd_clt_info_rl {
	($dev:expr, $fmt:literal $(, $arg:expr)*) => {
		rnbd_clt_log!(pr_info_ratelimited, $dev, $fmt $(, $arg)*)
	};
}

macro_rules! rnbd_srv_err {
	($dev:expr, $fmt:literal $(, $arg:expr)*) => {
		rnbd_srv_log!(pr_err, $dev, $fmt $(, $arg)*)
	};
}

macro_rules! rnbd_srv_err_rl {
	($dev:expr, $fmt:literal $(, $arg:expr)*) => {
		rnbd_srv_log!(pr_err_ratelimited, $dev, $fmt $(, $arg)*)
	};
}

macro_rules! rnbd_srv_info {
	($dev:expr, $fmt:literal $(, $arg:expr)*) => {
		rnbd_srv_log!(pr_info, $dev, $fmt $(, $arg)*)
	};
}

macro_rules! rnbd_srv_info_rl {
	($dev:expr, $fmt:literal $(, $arg:expr)*) => {
		rnbd_srv_log!(pr_info_ratelimited, $dev, $fmt $(, $arg)*)
	};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
