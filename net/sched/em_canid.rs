// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * em_canid.c  Ematch rule to match CAN frames according to their CAN IDs
 *
 * Idea:       Oliver Hartkopp <oliver.hartkopp@volkswagen.de>
 * Copyright:  (c) 2011 Czech Technical University in Prague
 *             (c) 2011 Volkswagen Group Research
 * Authors:    Michal Sojka <sojkam1@fel.cvut.cz>
 *             Pavel Pisa <pisa@cmp.felk.cvut.cz>
 *             Rostislav Lisovy <lisovy@gmail.cz>
 * Funded by:  Volkswagen Group Research
 */

// Kernel headers and build-provided symbols are supplied by other translated units.

const EM_CAN_RULES_MAX: usize = 500;

#[repr(C)]
struct canid_match {
    /* For each SFF CAN ID (11 bit) there is one record in this bitfield */
    match_sff: [u64; (1usize << CAN_SFF_ID_BITS) / 64],

    rules_count: i32,
    sff_rules_count: i32,
    eff_rules_count: i32,

    /*
     * Raw rules copied from netlink message; Used for sending
     * information to userspace (when 'tc filter show' is invoked)
     * AND when matching EFF frames
     */
    rules_raw: [can_filter; 0],
}

/**
 * em_canid_get_id() - Extracts Can ID out of the sk_buff structure.
 * @skb: buffer to extract Can ID from
 */
unsafe fn em_canid_get_id(skb: *mut sk_buff) -> canid_t {
    /* CAN ID is stored within the data field */
    let cf = (*skb).data as *mut can_frame;

    (*cf).can_id
}

unsafe fn em_canid_sff_match_add(cm: *mut canid_match, mut can_id: u32, mut can_mask: u32) {
    let mut i: i32;

    /*
     * Limit can_mask and can_id to SFF range to
     * protect against write after end of array
     */
    can_mask &= CAN_SFF_MASK;
    can_id &= can_mask;

    /* Single frame */
    if can_mask == CAN_SFF_MASK {
        set_bit(can_id as usize, (*cm).match_sff.as_mut_ptr());
        return;
    }

    /* All frames */
    if can_mask == 0 {
        bitmap_fill((*cm).match_sff.as_mut_ptr(), 1usize << CAN_SFF_ID_BITS);
        return;
    }

    /*
     * Individual frame filter.
     * Add record (set bit to 1) for each ID that
     * conforms particular rule
     */
    i = 0;
    while i < (1 << CAN_SFF_ID_BITS) {
        if ((i as u32) & can_mask) == can_id {
            set_bit(i as usize, (*cm).match_sff.as_mut_ptr());
        }
        i += 1;
    }
}

unsafe fn em_canid_priv(m: *mut tcf_ematch) -> *mut canid_match {
    (*m).data as *mut canid_match
}

unsafe fn em_canid_match(
    skb: *mut sk_buff,
    m: *mut tcf_ematch,
    _info: *mut tcf_pkt_info,
) -> i32 {
    let cm = em_canid_priv(m);
    let can_id: canid_t;
    let mut match_: i32 = 0;
    let mut i: i32;
    let mut lp: *const can_filter;

    if pskb_may_pull(skb, CAN_MTU) == 0 {
        return 0;
    }

    can_id = em_canid_get_id(skb);

    if can_id & CAN_EFF_FLAG != 0 {
        i = 0;
        lp = (*cm).rules_raw.as_ptr();
        while i < (*cm).eff_rules_count {
            if ((*lp).can_id ^ can_id) & (*lp).can_mask == 0 {
                match_ = 1;
                break;
            }
            i += 1;
            lp = lp.add(1);
        }
    } else {
        /* SFF */
        let sff_id = can_id & CAN_SFF_MASK;
        match_ = if test_bit(sff_id as usize, (*cm).match_sff.as_ptr()) != 0 { 1 } else { 0 };
    }

    match_
}

unsafe fn em_canid_change(
    _net: *mut net,
    data: *mut core::ffi::c_void,
    len: usize,
    m: *mut tcf_ematch,
) -> i32 {
    let conf = data as *mut can_filter; /* Array with rules */
    let cm: *mut canid_match;
    let mut i: i32;

    if len == 0 || len % core::mem::size_of::<can_filter>() != 0 {
        return -EINVAL;
    }

    if len > core::mem::size_of::<can_filter>() * EM_CAN_RULES_MAX {
        return -EINVAL;
    }

    cm = kzalloc(core::mem::size_of::<canid_match>() + len, GFP_KERNEL) as *mut canid_match;
    if cm.is_null() {
        return -ENOMEM;
    }

    (*cm).rules_count = (len / core::mem::size_of::<can_filter>()) as i32;

    /*
     * We need two for() loops for copying rules into two contiguous
     * areas in rules_raw to process all eff rules with a simple loop.
     * NB: The configuration interface supports sff and eff rules.
     * We do not support filters here that match for the same can_id
     * provided in a SFF and EFF frame (e.g. 0x123 / 0x80000123).
     * For this (unusual case) two filters have to be specified. The
     * SFF/EFF separation is done with the CAN_EFF_FLAG in the can_id.
     */

    /* Fill rules_raw with EFF rules first */
    i = 0;
    while i < (*cm).rules_count {
        if (*conf.add(i as usize)).can_id & CAN_EFF_FLAG != 0 {
            core::ptr::copy_nonoverlapping(
                conf.add(i as usize),
                (*cm).rules_raw.as_mut_ptr().add((*cm).eff_rules_count as usize),
                1,
            );
            (*cm).eff_rules_count += 1;
        }
        i += 1;
    }

    /* append SFF frame rules */
    i = 0;
    while i < (*cm).rules_count {
        if (*conf.add(i as usize)).can_id & CAN_EFF_FLAG == 0 {
            core::ptr::copy_nonoverlapping(
                conf.add(i as usize),
                (*cm).rules_raw.as_mut_ptr().add(
                    ((*cm).eff_rules_count + (*cm).sff_rules_count) as usize,
                ),
                1,
            );

            (*cm).sff_rules_count += 1;

            em_canid_sff_match_add(
                cm,
                (*conf.add(i as usize)).can_id,
                (*conf.add(i as usize)).can_mask,
            );
        }
        i += 1;
    }

    (*m).datalen = core::mem::size_of::<canid_match>() + len;
    (*m).data = cm as usize;
    0
}

unsafe fn em_canid_destroy(m: *mut tcf_ematch) {
    let cm = em_canid_priv(m);

    kfree(cm as *mut core::ffi::c_void);
}

unsafe fn em_canid_dump(skb: *mut sk_buff, m: *mut tcf_ematch) -> i32 {
    let cm = em_canid_priv(m);

    /*
     * When configuring this ematch 'rules_count' is set not to exceed
     * 'rules_raw' array size
     */
    if nla_put_nohdr(
        skb,
        core::mem::size_of::<can_filter>() * (*cm).rules_count as usize,
        (*cm).rules_raw.as_ptr() as *const core::ffi::c_void,
    ) < 0 {
        return -EMSGSIZE;
    }

    0
}

static mut em_canid_ops: tcf_ematch_ops = tcf_ematch_ops {
    kind: TCF_EM_CANID,
    change: Some(em_canid_change),
    match_: Some(em_canid_match),
    destroy: Some(em_canid_destroy),
    dump: Some(em_canid_dump),
    owner: THIS_MODULE,
    link: LIST_HEAD_INIT,
};

unsafe fn init_em_canid() -> i32 {
    tcf_em_register(&raw mut em_canid_ops)
}

unsafe fn exit_em_canid() {
    tcf_em_unregister(&raw mut em_canid_ops);
}

// MODULE_DESCRIPTION("ematch classifier to match CAN IDs embedded in skb CAN frames");
// MODULE_LICENSE("GPL");
// module_init(init_em_canid);
// module_exit(exit_em_canid);
// MODULE_ALIAS_TCF_EMATCH(TCF_EM_CANID);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
