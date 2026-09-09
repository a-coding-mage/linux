/* SPDX-License-Identifier: GPL-2.0
 *
 * audio-graph.h
 *
 * Copyright (c) 2024 Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
 */

/*
 * used in
 *	link-trigger-order
 *	link-trigger-order-start
 *	link-trigger-order-stop
 *
 * default is
 *	link-trigger-order = <SND_SOC_TRIGGER_LINK
 *	                      SND_SOC_TRIGGER_COMPONENT
 *	                      SND_SOC_TRIGGER_DAI>;
 */
pub const SND_SOC_TRIGGER_LINK: i32 = 0;
pub const SND_SOC_TRIGGER_COMPONENT: i32 = 1;
pub const SND_SOC_TRIGGER_DAI: i32 = 2;
pub const SND_SOC_TRIGGER_SIZE: i32 = 3; /* shoud be last */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
