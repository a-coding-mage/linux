// SPDX-License-Identifier: GPL-2.0-only

// Original C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

extern "C" {
    fn bpf_get_retval() -> i32;
    fn bpf_set_retval(retval: i32) -> i64;
}

// Original C macro:
// #define BPF_RETVAL_HOOK(name, section, ctx, expected_err) \
//      __attribute__((__section__("?" section))) \
//      int name(struct ctx *_ctx) \
//      { \
//              bpf_set_retval(bpf_get_retval()); \
//              return 1; \
//      }
//
// The hook invocations are supplied by cgroup_getset_retval_hooks.h in the
// original source. That external dependency is intentionally not expanded here.
macro_rules! BPF_RETVAL_HOOK {
    ($name:ident, $section:expr, $ctx:ty, $expected_err:expr) => {
        #[unsafe(link_section = concat!("?", $section))]
        pub unsafe extern "C" fn $name(_ctx: *mut $ctx) -> i32 {
            unsafe {
                bpf_set_retval(bpf_get_retval());
            }
            1
        }
    };
}

// Original C dependency:
// #include "cgroup_getset_retval_hooks.h"

