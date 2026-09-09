// SPDX-License-Identifier: GPL-2.0-only
/* Kernel module to match ESP parameters. */

/* (C) 1999-2000 Yon Uriarte <yon@astaro.de>
 */

// Dependencies supplied by the kernel and netfilter bindings.

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Yon Uriarte <yon@astaro.de>");
// MODULE_DESCRIPTION("Xtables: IPsec-ESP packet match");
// MODULE_ALIAS("ipt_esp");
// MODULE_ALIAS("ip6t_esp");

/* Returns 1 if the spi is matched by the range, 0 otherwise */
#[inline]
unsafe fn spi_match(min: u32, max: u32, spi: u32, invert: bool) -> bool {
    ((spi >= min && spi <= max) as u8 != invert as u8)
}

unsafe fn esp_mt(
    skb: *const sk_buff,
    par: *mut xt_action_param,
) -> bool {
    let mut _esp: ip_esp_hdr = core::mem::zeroed();
    let eh: *const ip_esp_hdr;
    let espinfo: *const xt_esp = (*par).matchinfo as *const xt_esp;

    /* Must not be a fragment. */
    if (*par).fragoff != 0 {
        return false;
    }

    eh = skb_header_pointer(
        skb,
        (*par).thoff,
        core::mem::size_of::<ip_esp_hdr>(),
        &mut _esp as *mut ip_esp_hdr as *mut core::ffi::c_void,
    ) as *const ip_esp_hdr;
    if eh.is_null() {
        /* We've been asked to examine this packet, and we
         * can't.  Hence, no choice but to drop.
         */
        (*par).hotdrop = true;
        return false;
    }

    spi_match(
        (*espinfo).spis[0],
        (*espinfo).spis[1],
        u32::from_be((*eh).spi),
        ((*espinfo).invflags & XT_ESP_INV_SPI) != 0,
    )
}

unsafe fn esp_mt_check(par: *const xt_mtchk_param) -> i32 {
    let espinfo: *const xt_esp = (*par).matchinfo as *const xt_esp;

    if (*espinfo).invflags & !XT_ESP_INV_MASK != 0 {
        pr_info_ratelimited!("unknown flags %X\n", (*espinfo).invflags);
        return -EINVAL;
    }

    0
}

static mut esp_mt_reg: [xt_match; 2] = [
    xt_match {
        name: *b"esp\0",
        family: NFPROTO_IPV4,
        checkentry: Some(esp_mt_check),
        r#match: Some(esp_mt),
        matchsize: core::mem::size_of::<xt_esp>(),
        proto: IPPROTO_ESP,
        me: THIS_MODULE,
    },
    xt_match {
        name: *b"esp\0",
        family: NFPROTO_IPV6,
        checkentry: Some(esp_mt_check),
        r#match: Some(esp_mt),
        matchsize: core::mem::size_of::<xt_esp>(),
        proto: IPPROTO_ESP,
        me: THIS_MODULE,
    },
];

unsafe fn esp_mt_init() -> i32 {
    xt_register_matches(esp_mt_reg.as_mut_ptr(), esp_mt_reg.len())
}

unsafe fn esp_mt_exit() {
    xt_unregister_matches(esp_mt_reg.as_mut_ptr(), esp_mt_reg.len());
}

// module_init(esp_mt_init);
// module_exit(esp_mt_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
