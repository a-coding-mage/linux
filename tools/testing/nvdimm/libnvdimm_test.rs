// SPDX-License-Identifier: GPL-2.0
// Copyright(c) 2018 Intel Corporation. All rights reserved.

// C dependencies: <linux/module.h>, <linux/printk.h>, "watermark.h".
// The original source invokes the watermark macro for libnvdimm:
// nfit_test_watermark(libnvdimm);
nfit_test_watermark!(libnvdimm);
