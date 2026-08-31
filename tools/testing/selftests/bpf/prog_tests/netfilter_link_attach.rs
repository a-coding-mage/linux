// SPDX-License-Identifier: GPL-2.0-or-later

// C dependencies:
// #include <netinet/in.h>
// #include <linux/netfilter.h>
// #include "test_progs.h"
// #include "test_netfilter_link_attach.skel.h"

#[repr(C)]
struct nf_link_test {
    pf: __u32,
    hooknum: __u32,
    priority: __s32,
    flags: __u32,

    expect_success: bool,
    name: *const ::core::ffi::c_char,
}

static nf_hook_link_tests: [nf_link_test; 9] = [
    nf_link_test {
        pf: 0,
        hooknum: 0,
        priority: 0,
        flags: 0,
        expect_success: false,
        name: c"allzero".as_ptr(),
    },
    nf_link_test {
        pf: NFPROTO_NUMPROTO,
        hooknum: 0,
        priority: 0,
        flags: 0,
        expect_success: false,
        name: c"invalid-pf".as_ptr(),
    },
    nf_link_test {
        pf: NFPROTO_IPV4,
        hooknum: 42,
        priority: 0,
        flags: 0,
        expect_success: false,
        name: c"invalid-hooknum".as_ptr(),
    },
    nf_link_test {
        pf: NFPROTO_IPV4,
        hooknum: 0,
        priority: INT_MIN,
        flags: 0,
        expect_success: false,
        name: c"invalid-priority-min".as_ptr(),
    },
    nf_link_test {
        pf: NFPROTO_IPV4,
        hooknum: 0,
        priority: INT_MAX,
        flags: 0,
        expect_success: false,
        name: c"invalid-priority-max".as_ptr(),
    },
    nf_link_test {
        pf: NFPROTO_IPV4,
        hooknum: 0,
        priority: 0,
        flags: UINT_MAX,
        expect_success: false,
        name: c"invalid-flags".as_ptr(),
    },
    nf_link_test {
        pf: NFPROTO_INET,
        hooknum: 0,
        priority: 1,
        flags: 0,
        expect_success: false,
        name: c"invalid-inet-not-supported".as_ptr(),
    },
    nf_link_test {
        pf: NFPROTO_IPV4,
        hooknum: NF_INET_POST_ROUTING,
        priority: -10000,
        flags: 0,
        expect_success: true,
        name: c"attach ipv4".as_ptr(),
    },
    nf_link_test {
        pf: NFPROTO_IPV6,
        hooknum: NF_INET_FORWARD,
        priority: 10001,
        flags: BPF_F_NETFILTER_IP_DEFRAG,
        expect_success: true,
        name: c"attach ipv6".as_ptr(),
    },
];

unsafe fn verify_netfilter_link_info(link: *mut bpf_link, nf_expected: nf_link_test) {
    let mut info: bpf_link_info = ::core::mem::zeroed();
    let mut len: __u32 = ::core::mem::size_of_val(&info) as __u32;
    let err: ::core::ffi::c_int;
    let fd: ::core::ffi::c_int;

    memset(
        &mut info as *mut _ as *mut ::core::ffi::c_void,
        0,
        len as usize,
    );

    fd = bpf_link__fd(link);
    err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
    ASSERT_OK(err, c"get_link_info".as_ptr());

    ASSERT_EQ(info.type_, BPF_LINK_TYPE_NETFILTER, c"info link type".as_ptr());
    ASSERT_EQ(
        info.netfilter.pf,
        nf_expected.pf,
        c"info nf protocol family".as_ptr(),
    );
    ASSERT_EQ(
        info.netfilter.hooknum,
        nf_expected.hooknum,
        c"info nf hooknum".as_ptr(),
    );
    ASSERT_EQ(
        info.netfilter.priority,
        nf_expected.priority,
        c"info nf priority".as_ptr(),
    );
    ASSERT_EQ(
        info.netfilter.flags,
        nf_expected.flags,
        c"info nf flags".as_ptr(),
    );
}

pub unsafe extern "C" fn test_netfilter_link_attach() {
    let mut skel: *mut test_netfilter_link_attach;
    let mut prog: *mut bpf_program;
    let mut opts: bpf_netfilter_opts = bpf_netfilter_opts::default();
    let mut i: ::core::ffi::c_int;

    skel = test_netfilter_link_attach__open_and_load();
    if !ASSERT_OK_PTR(
        skel as *mut ::core::ffi::c_void,
        c"test_netfilter_link_attach__open_and_load".as_ptr(),
    ) {
        goto_out(skel);
        return;
    }

    prog = (*skel).progs.nf_link_attach_test;
    if !ASSERT_OK_PTR(prog as *mut ::core::ffi::c_void, c"attach program".as_ptr()) {
        goto_out(skel);
        return;
    }

    i = 0;
    while i < nf_hook_link_tests.len() as ::core::ffi::c_int {
        let mut link: *mut bpf_link;
        let test = &nf_hook_link_tests[i as usize];

        if !test__start_subtest(test.name) {
            i += 1;
            continue;
        }

        opts.pf = test.pf;
        opts.hooknum = test.hooknum;
        opts.priority = test.priority;
        opts.flags = test.flags;

        link = bpf_program__attach_netfilter(prog, &mut opts);
        if test.expect_success {
            let mut link2: *mut bpf_link;

            if !ASSERT_OK_PTR(
                link as *mut ::core::ffi::c_void,
                c"program attach successful".as_ptr(),
            ) {
                i += 1;
                continue;
            }

            verify_netfilter_link_info(link, *test);

            link2 = bpf_program__attach_netfilter(prog, &mut opts);
            ASSERT_ERR_PTR(
                link2 as *mut ::core::ffi::c_void,
                c"attach program with same pf/hook/priority".as_ptr(),
            );

            if !ASSERT_OK(bpf_link__destroy(link), c"link destroy".as_ptr()) {
                break;
            }

            link2 = bpf_program__attach_netfilter(prog, &mut opts);
            if !ASSERT_OK_PTR(
                link2 as *mut ::core::ffi::c_void,
                c"program reattach successful".as_ptr(),
            ) {
                i += 1;
                continue;
            }

            verify_netfilter_link_info(link2, *test);

            if !ASSERT_OK(bpf_link__destroy(link2), c"link destroy".as_ptr()) {
                break;
            }
        } else {
            ASSERT_ERR_PTR(
                link as *mut ::core::ffi::c_void,
                c"program load failure".as_ptr(),
            );
        }

        i += 1;
    }

    goto_out(skel);
}

unsafe fn goto_out(skel: *mut test_netfilter_link_attach) {
    test_netfilter_link_attach__destroy(skel);
}
