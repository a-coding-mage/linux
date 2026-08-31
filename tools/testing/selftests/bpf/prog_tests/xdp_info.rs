// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/if_link.h>, <test_progs.h>

use core::mem::size_of;
use core::ptr::null_mut;

const IFINDEX_LO: i32 = 1;

pub unsafe fn serial_test_xdp_info() {
    let mut len: __u32 = size_of::<bpf_prog_info>() as __u32;
    let mut duration: __u32 = 0;
    let mut prog_id: __u32 = 0;
    let file = c"./xdp_dummy.bpf.o";
    let mut opts: bpf_xdp_query_opts = LIBBPF_OPTS!(bpf_xdp_query_opts);
    let mut info: bpf_prog_info = core::mem::zeroed();
    let mut obj: *mut bpf_object = null_mut();
    let mut err: i32;
    let mut prog_fd: i32 = 0;

    /* Get prog_id for XDP_ATTACHED_NONE mode */

    err = bpf_xdp_query_id(IFINDEX_LO, 0, &mut prog_id);
    if CHECK!(err, c"get_xdp_none", c"errno=%d\n", errno) {
        return;
    }
    if CHECK!(
        prog_id,
        c"prog_id_none",
        c"unexpected prog_id=%u\n",
        prog_id
    ) {
        return;
    }

    err = bpf_xdp_query_id(IFINDEX_LO, XDP_FLAGS_SKB_MODE, &mut prog_id);
    if CHECK!(err, c"get_xdp_none_skb", c"errno=%d\n", errno) {
        return;
    }
    if CHECK!(
        prog_id,
        c"prog_id_none_skb",
        c"unexpected prog_id=%u\n",
        prog_id
    ) {
        return;
    }

    /* Setup prog */

    err = bpf_prog_test_load(file.as_ptr(), BPF_PROG_TYPE_XDP, &mut obj, &mut prog_fd);
    if CHECK_FAIL!(err) {
        return;
    }

    err = bpf_prog_get_info_by_fd(prog_fd, &mut info, &mut len);
    if CHECK!(err, c"get_prog_info", c"errno=%d\n", errno) {
        bpf_object__close(obj);
        return;
    }

    err = bpf_xdp_attach(IFINDEX_LO, prog_fd, XDP_FLAGS_SKB_MODE, null_mut());
    if CHECK!(err, c"set_xdp_skb", c"errno=%d\n", errno) {
        bpf_object__close(obj);
        return;
    }

    'out: {
        /* Get prog_id for single prog mode */

        err = bpf_xdp_query_id(IFINDEX_LO, 0, &mut prog_id);
        if CHECK!(err, c"get_xdp", c"errno=%d\n", errno) {
            break 'out;
        }
        if CHECK!(prog_id != info.id, c"prog_id", c"prog_id not available\n") {
            break 'out;
        }

        err = bpf_xdp_query_id(IFINDEX_LO, XDP_FLAGS_SKB_MODE, &mut prog_id);
        if CHECK!(err, c"get_xdp_skb", c"errno=%d\n", errno) {
            break 'out;
        }
        if CHECK!(
            prog_id != info.id,
            c"prog_id_skb",
            c"prog_id not available\n"
        ) {
            break 'out;
        }

        err = bpf_xdp_query_id(IFINDEX_LO, XDP_FLAGS_DRV_MODE, &mut prog_id);
        if CHECK!(err, c"get_xdp_drv", c"errno=%d\n", errno) {
            break 'out;
        }
        if CHECK!(
            prog_id,
            c"prog_id_drv",
            c"unexpected prog_id=%u\n",
            prog_id
        ) {
            break 'out;
        }

        /* Check xdp features supported by lo device */
        opts.feature_flags = !0;
        err = bpf_xdp_query(IFINDEX_LO, XDP_FLAGS_DRV_MODE, &mut opts);
        if !ASSERT_OK!(err, c"bpf_xdp_query") {
            break 'out;
        }

        ASSERT_EQ!(opts.feature_flags, 0, c"opts.feature_flags");
    }

    bpf_xdp_detach(IFINDEX_LO, 0, null_mut());
    bpf_object__close(obj);
}
