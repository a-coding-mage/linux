// SPDX-License-Identifier: GPL-2.0-only
// Dependencies supplied by the corresponding Linux networking headers:
// linux/netlink.h, linux/nospec.h, linux/rtnetlink.h, linux/types.h,
// net/ip.h, net/net_namespace.h, and net/tcp.h.

unsafe fn ip_metrics_convert(
    fc_mx: *mut nlattr,
    fc_mx_len: i32,
    metrics: *mut u32,
    extack: *mut netlink_ext_ack,
) -> i32 {
    let mut ecn_ca = false;
    let mut nla: *mut nlattr = core::ptr::null_mut();
    let mut remaining: i32 = 0;

    // Equivalent to nla_for_each_attr(nla, fc_mx, fc_mx_len, remaining).
    while nla_for_each_attr_next(&mut nla, fc_mx, fc_mx_len, &mut remaining) {
        let mut type_ = nla_type(nla);
        let mut val: u32;

        if type_ == 0 {
            continue;
        }
        if type_ > RTAX_MAX {
            NL_SET_ERR_MSG!(extack, "Invalid metric type");
            return -EINVAL;
        }

        type_ = array_index_nospec(type_, RTAX_MAX + 1);
        if type_ == RTAX_CC_ALGO {
            let mut tmp = [0i8; TCP_CA_NAME_MAX as usize];

            nla_strscpy(tmp.as_mut_ptr(), nla, tmp.len());
            val = tcp_ca_get_key_by_name(tmp.as_mut_ptr(), &mut ecn_ca);
            if val == TCP_CA_UNSPEC {
                NL_SET_ERR_MSG!(extack, "Unknown tcp congestion algorithm");
                return -EINVAL;
            }
        } else {
            if nla_len(nla) != core::mem::size_of::<u32>() as i32 {
                NL_SET_ERR_MSG_ATTR!(extack, nla, "Invalid attribute in metrics");
                return -EINVAL;
            }
            val = nla_get_u32(nla);
        }
        if type_ == RTAX_ADVMSS && val > 65535 - 40 {
            val = 65535 - 40;
        }
        if type_ == RTAX_MTU && val > 65535 - 15 {
            val = 65535 - 15;
        }
        if type_ == RTAX_HOPLIMIT && val > 255 {
            val = 255;
        }
        if type_ == RTAX_FEATURES && (val & !RTAX_FEATURE_MASK) != 0 {
            NL_SET_ERR_MSG!(extack, "Unknown flag set in feature mask in metrics attribute");
            return -EINVAL;
        }
        *metrics.add((type_ - 1) as usize) = val;
    }

    if ecn_ca {
        *metrics.add((RTAX_FEATURES - 1) as usize) |= DST_FEATURE_ECN_CA;
    }

    0
}

unsafe fn ip_fib_metrics_init(
    fc_mx: *mut nlattr,
    fc_mx_len: i32,
    extack: *mut netlink_ext_ack,
) -> *mut dst_metrics {
    let mut fib_metrics: *mut dst_metrics;
    let err: i32;

    if fc_mx.is_null() {
        return &mut dst_default_metrics as *mut dst_metrics;
    }

    fib_metrics = kzalloc_obj::<dst_metrics>();
    if unlikely(fib_metrics.is_null()) {
        return ERR_PTR(-ENOMEM);
    }

    err = ip_metrics_convert(fc_mx, fc_mx_len, (*fib_metrics).metrics.as_mut_ptr(), extack);
    if err == 0 {
        refcount_set(&mut (*fib_metrics).refcnt, 1);
    } else {
        kfree(fib_metrics);
        fib_metrics = ERR_PTR(err);
    }

    fib_metrics
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
