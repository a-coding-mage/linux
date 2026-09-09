/* SPDX-License-Identifier: GPL-2.0 */

pub const REMOTE_TEST_EVENT_ID: u32 = 1;

// External event-generation declaration corresponding to:
// REMOTE_EVENT(selftest, REMOTE_TEST_EVENT_ID,
//     RE_STRUCT(
//         re_field(u64, id)
//     ),
//     RE_PRINTK("id=%llu", __entry->id)
// );

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
