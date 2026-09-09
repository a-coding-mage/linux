// SPDX-License-Identifier: GPL-2.0-only
/* (C) 2001-2002 Magnus Boden <mb@ozaba.mine.nu>
 * (C) 2006-2012 Patrick McHardy <kaber@trash.net>
 */

// #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// Linux kernel module and netfilter headers are supplied by external dependencies.

const HELPER_NAME: &[u8] = b"tftp\0";

// MODULE_AUTHOR("Magnus Boden <mb@ozaba.mine.nu>");
// MODULE_DESCRIPTION("TFTP connection tracking helper");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("ip_conntrack_tftp");
// MODULE_ALIAS_NFCT_HELPER(HELPER_NAME);

static mut nf_nat_tftp_hook: Option<nf_nat_tftp_hook_fn> = None;
// EXPORT_SYMBOL_GPL(nf_nat_tftp_hook);

unsafe fn tftp_help(
    skb: *mut sk_buff,
    protoff: c_uint,
    ct: *mut nf_conn,
    ctinfo: ip_conntrack_info,
) -> c_int {
    let mut tfh: *const tftphdr;
    let mut _tftph = core::mem::MaybeUninit::<tftphdr>::uninit();
    let mut exp: *mut nf_conntrack_expect;
    let mut tuple: *mut nf_conntrack_tuple;
    let mut ret: c_uint = NF_ACCEPT;
    let nf_nat_tftp: Option<nf_nat_tftp_hook_fn>;

    tfh = skb_header_pointer(
        skb,
        protoff + core::mem::size_of::<udphdr>() as c_uint,
        core::mem::size_of::<tftphdr>() as c_uint,
        _tftph.as_mut_ptr().cast(),
    );
    if tfh.is_null() {
        return NF_ACCEPT as c_int;
    }

    match ntohs((*tfh).opcode) {
        TFTP_OPCODE_READ | TFTP_OPCODE_WRITE => {
            // RRQ and WRQ works the same way
            nf_ct_dump_tuple(&(*ct).tuplehash[IP_CT_DIR_ORIGINAL].tuple);
            nf_ct_dump_tuple(&(*ct).tuplehash[IP_CT_DIR_REPLY].tuple);

            exp = nf_ct_expect_alloc(ct);
            if exp.is_null() {
                nf_ct_helper_log(skb, ct, b"cannot alloc expectation\0".as_ptr().cast());
                return NF_DROP as c_int;
            }
            tuple = &mut (*ct).tuplehash[IP_CT_DIR_REPLY].tuple;
            nf_ct_expect_init(
                exp,
                NF_CT_EXPECT_CLASS_DEFAULT,
                nf_ct_l3num(ct),
                &(*tuple).src.u3,
                &(*tuple).dst.u3,
                IPPROTO_UDP,
                core::ptr::null_mut(),
                &(*tuple).dst.u.udp.port,
            );

            pr_debug!("expect: ");
            nf_ct_dump_tuple(&(*exp).tuple);

            nf_nat_tftp = rcu_dereference(nf_nat_tftp_hook);
            if nf_nat_tftp.is_some() && ((*ct).status & IPS_NAT_MASK) != 0 {
                ret = nf_nat_tftp.unwrap()(skb, ct, ctinfo, exp);
            } else if nf_ct_expect_related(exp, 0) != 0 {
                nf_ct_helper_log(skb, ct, b"cannot add expectation\0".as_ptr().cast());
                ret = NF_DROP;
            }
            nf_ct_expect_put(exp);
        }
        TFTP_OPCODE_DATA | TFTP_OPCODE_ACK => {
            pr_debug!("Data/ACK opcode\n");
        }
        TFTP_OPCODE_ERROR => {
            pr_debug!("Error opcode\n");
        }
        _ => {
            pr_debug!("Unknown opcode\n");
        }
    }
    ret as c_int
}

static mut tftp: nf_conntrack_helper = unsafe { core::mem::zeroed() };
static mut tftp_ptr: *mut nf_conntrack_helper = core::ptr::null_mut();

static tftp_exp_policy: nf_conntrack_expect_policy = nf_conntrack_expect_policy {
    max_expected: 1,
    timeout: 5 * 60,
};

unsafe fn nf_conntrack_tftp_fini() {
    nf_conntrack_helper_unregister(tftp_ptr);
}

unsafe fn nf_conntrack_tftp_init() -> c_int {
    let ret: c_int;

    NF_CT_HELPER_BUILD_BUG_ON!(0);

    nf_ct_helper_init(
        &mut tftp,
        NFPROTO_UNSPEC,
        IPPROTO_UDP,
        HELPER_NAME.as_ptr().cast(),
        &tftp_exp_policy,
        0,
        Some(tftp_help),
        None,
        THIS_MODULE,
    );

    ret = nf_conntrack_helper_register(&mut tftp, &mut tftp_ptr);
    if ret < 0 {
        pr_err!("failed to register helpers\n");
        return ret;
    }
    0
}

// module_init(nf_conntrack_tftp_init);
// module_exit(nf_conntrack_tftp_fini);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
