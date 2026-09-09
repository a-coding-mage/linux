// SPDX-License-Identifier: GPL-2.0-only
/*
 *	xt_u32 - kernel module to match u32 packet content
 *
 *	Original author: Don Cohen <don@isis.cs3-inc.com>
 *	(C) CC Computer Consultants GmbH, 2007
 */

// Kernel declarations supplied by the surrounding translation unit.

unsafe fn u32_match_it(data: *const xt_u32, skb: *const sk_buff) -> i32 {
    let mut ct: *const xt_u32_test;
    let mut testind: u32;
    let mut nnums: u32;
    let mut nvals: u32;
    let mut i: u32;
    let mut n: __be32 = 0;
    let mut pos: u32;
    let mut val: u32;
    let mut at: u32;

    /*
     * Small example: "0 >> 28 == 4 && 8 & 0xFF0000 >> 16 = 6, 17"
     * (=IPv4 and (TCP or UDP)). Outer loop runs over the "&&" operands.
     */
    testind = 0;
    while testind < (*data).ntests {
        ct = &(*data).tests[testind as usize];
        at = 0;
        pos = (*ct).location[0].number;

        if (*skb).len < 4 || pos > (*skb).len - 4 {
            return 0;
        }

        if skb_copy_bits(skb, pos, &mut n as *mut __be32 as *mut core::ffi::c_void,
                         core::mem::size_of::<__be32>()) < 0 {
            return -1;
        }

        val = u32::from_be(n);
        nnums = (*ct).nnums;

        /* Inner loop runs over "&", "<<", ">>" and "@" operands */
        i = 1;
        while i < nnums {
            let number = (*ct).location[i as usize].number;
            match (*ct).location[i as usize].nextop {
                XT_U32_AND => val &= number,
                XT_U32_LEFTSH => val = val.wrapping_shl(number),
                XT_U32_RIGHTSH => val >>= number,
                XT_U32_AT => {
                    if at.wrapping_add(val) < at {
                        return 0;
                    }
                    at = at.wrapping_add(val);
                    pos = number;
                    if at.wrapping_add(4) < at || (*skb).len < at.wrapping_add(4)
                        || pos > (*skb).len - at - 4
                    {
                        return 0;
                    }

                    if skb_copy_bits(skb, at + pos,
                                     &mut n as *mut __be32 as *mut core::ffi::c_void,
                                     core::mem::size_of::<__be32>()) < 0 {
                        return -1;
                    }
                    val = u32::from_be(n);
                }
                _ => {}
            }
            i += 1;
        }

        /* Run over the "," and ":" operands */
        nvals = (*ct).nvalues;
        i = 0;
        while i < nvals {
            if (*ct).value[i as usize].min <= val && val <= (*ct).value[i as usize].max {
                break;
            }
            i += 1;
        }

        if i >= (*ct).nvalues {
            return 0;
        }
        testind += 1;
    }

    1
}

unsafe extern "C" fn u32_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let data = (*par).matchinfo as *const xt_u32;
    let ret = u32_match_it(data, skb);
    if ret < 0 {
        (*par).hotdrop = true;
        return false;
    }

    (ret ^ (*data).invert as i32) != 0
}

unsafe extern "C" fn u32_mt_checkentry(par: *const xt_mtchk_param) -> i32 {
    let data = (*par).matchinfo as *const xt_u32;
    let mut i: u32;
    let mut j: u32;

    if (*data).ntests > core::mem::size_of_val(&(*data).tests) as u32 {
        return -22;
    }

    i = 0;
    while i < (*data).ntests {
        let ct = &(*data).tests[i as usize];

        if ct.nnums > core::mem::size_of_val(&ct.location) as u32
            || ct.nvalues > core::mem::size_of_val(&ct.value) as u32
        {
            return -22;
        }

        j = 1;
        while j < ct.nnums {
            match ct.location[j as usize].nextop {
                XT_U32_LEFTSH | XT_U32_RIGHTSH => {
                    if ct.location[j as usize].number >= 32 {
                        return -22;
                    }
                }
                _ => {}
            }
            j += 1;
        }
        i += 1;
    }

    0
}

// Equivalent to the C module-registration object; its field types are supplied by
// the kernel x_tables declarations.
#[allow(non_upper_case_globals)]
static mut xt_u32_mt_reg: xt_match = xt_match {
    name: *b"u32\0",
    revision: 0,
    family: NFPROTO_UNSPEC,
    match_: Some(u32_mt),
    checkentry: Some(u32_mt_checkentry),
    matchsize: core::mem::size_of::<xt_u32>(),
    me: THIS_MODULE,
};

unsafe extern "C" fn u32_mt_init() -> i32 {
    xt_register_match(&mut xt_u32_mt_reg)
}

unsafe extern "C" fn u32_mt_exit() {
    xt_unregister_match(&mut xt_u32_mt_reg);
}

// module_init(u32_mt_init);
// module_exit(u32_mt_exit);
// MODULE_AUTHOR("Jan Engelhardt <jengelh@medozas.de>");
// MODULE_DESCRIPTION("Xtables: arbitrary byte matching");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("ipt_u32");
// MODULE_ALIAS("ip6t_u32");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
