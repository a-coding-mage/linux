// SPDX-License-Identifier: GPL-2.0-only
/*
 * nl802154.h
 *
 * Copyright (C) 2007, 2008 Siemens AG
 */

// Dependencies supplied by the surrounding kernel translation.

const NLA_HW_ADDR: u16 = NLA_U64;

pub static ieee802154_policy: [nla_policy; IEEE802154_ATTR_MAX + 1] = {
    let mut policy = [nla_policy { type_: 0, len: 0 }; IEEE802154_ATTR_MAX + 1];

    policy[IEEE802154_ATTR_DEV_NAME] = nla_policy { type_: NLA_STRING, len: 0 };
    policy[IEEE802154_ATTR_DEV_INDEX] = nla_policy { type_: NLA_U32, len: 0 };
    policy[IEEE802154_ATTR_PHY_NAME] = nla_policy { type_: NLA_STRING, len: 0 };

    policy[IEEE802154_ATTR_STATUS] = nla_policy { type_: NLA_U8, len: 0 };
    policy[IEEE802154_ATTR_SHORT_ADDR] = nla_policy { type_: NLA_U16, len: 0 };
    policy[IEEE802154_ATTR_HW_ADDR] = nla_policy { type_: NLA_HW_ADDR, len: 0 };
    policy[IEEE802154_ATTR_PAN_ID] = nla_policy { type_: NLA_U16, len: 0 };
    policy[IEEE802154_ATTR_CHANNEL] = nla_policy { type_: NLA_U8, len: 0 };
    policy[IEEE802154_ATTR_BCN_ORD] = nla_policy { type_: NLA_U8, len: 0 };
    policy[IEEE802154_ATTR_SF_ORD] = nla_policy { type_: NLA_U8, len: 0 };
    policy[IEEE802154_ATTR_PAN_COORD] = nla_policy { type_: NLA_U8, len: 0 };
    policy[IEEE802154_ATTR_BAT_EXT] = nla_policy { type_: NLA_U8, len: 0 };
    policy[IEEE802154_ATTR_COORD_REALIGN] = nla_policy { type_: NLA_U8, len: 0 };
    policy[IEEE802154_ATTR_PAGE] = nla_policy { type_: NLA_U8, len: 0 };
    policy[IEEE802154_ATTR_DEV_TYPE] = nla_policy { type_: NLA_U8, len: 0 };
    policy[IEEE802154_ATTR_COORD_SHORT_ADDR] = nla_policy { type_: NLA_U16, len: 0 };
    policy[IEEE802154_ATTR_COORD_HW_ADDR] = nla_policy { type_: NLA_HW_ADDR, len: 0 };
    policy[IEEE802154_ATTR_COORD_PAN_ID] = nla_policy { type_: NLA_U16, len: 0 };
    policy[IEEE802154_ATTR_SRC_SHORT_ADDR] = nla_policy { type_: NLA_U16, len: 0 };
    policy[IEEE802154_ATTR_SRC_HW_ADDR] = nla_policy { type_: NLA_HW_ADDR, len: 0 };
    policy[IEEE802154_ATTR_SRC_PAN_ID] = nla_policy { type_: NLA_U16, len: 0 };
    policy[IEEE802154_ATTR_DEST_SHORT_ADDR] = nla_policy { type_: NLA_U16, len: 0 };
    policy[IEEE802154_ATTR_DEST_HW_ADDR] = nla_policy { type_: NLA_HW_ADDR, len: 0 };
    policy[IEEE802154_ATTR_DEST_PAN_ID] = nla_policy { type_: NLA_U16, len: 0 };

    policy[IEEE802154_ATTR_CAPABILITY] = nla_policy { type_: NLA_U8, len: 0 };
    policy[IEEE802154_ATTR_REASON] = nla_policy { type_: NLA_U8, len: 0 };
    policy[IEEE802154_ATTR_SCAN_TYPE] = nla_policy { type_: NLA_U8, len: 0 };
    policy[IEEE802154_ATTR_CHANNELS] = nla_policy { type_: NLA_U32, len: 0 };
    policy[IEEE802154_ATTR_DURATION] = nla_policy { type_: NLA_U8, len: 0 };
    policy[IEEE802154_ATTR_ED_LIST] = nla_policy { type_: 0, len: 27 };
    policy[IEEE802154_ATTR_CHANNEL_PAGE_LIST] = nla_policy { type_: 0, len: 32 * 4 };

    policy[IEEE802154_ATTR_TXPOWER] = nla_policy { type_: NLA_S8, len: 0 };
    policy[IEEE802154_ATTR_LBT_ENABLED] = nla_policy { type_: NLA_U8, len: 0 };
    policy[IEEE802154_ATTR_CCA_MODE] = nla_policy { type_: NLA_U8, len: 0 };
    policy[IEEE802154_ATTR_CCA_ED_LEVEL] = nla_policy { type_: NLA_S32, len: 0 };
    policy[IEEE802154_ATTR_CSMA_RETRIES] = nla_policy { type_: NLA_U8, len: 0 };
    policy[IEEE802154_ATTR_CSMA_MIN_BE] = nla_policy { type_: NLA_U8, len: 0 };
    policy[IEEE802154_ATTR_CSMA_MAX_BE] = nla_policy { type_: NLA_U8, len: 0 };
    policy[IEEE802154_ATTR_FRAME_RETRIES] = nla_policy { type_: NLA_S8, len: 0 };

    policy[IEEE802154_ATTR_LLSEC_ENABLED] = nla_policy { type_: NLA_U8, len: 0 };
    policy[IEEE802154_ATTR_LLSEC_SECLEVEL] = nla_policy { type_: NLA_U8, len: 0 };
    policy[IEEE802154_ATTR_LLSEC_KEY_MODE] = nla_policy { type_: NLA_U8, len: 0 };
    policy[IEEE802154_ATTR_LLSEC_KEY_SOURCE_SHORT] = nla_policy { type_: NLA_U32, len: 0 };
    policy[IEEE802154_ATTR_LLSEC_KEY_SOURCE_EXTENDED] = nla_policy { type_: NLA_HW_ADDR, len: 0 };
    policy[IEEE802154_ATTR_LLSEC_KEY_ID] = nla_policy { type_: NLA_U8, len: 0 };
    policy[IEEE802154_ATTR_LLSEC_FRAME_COUNTER] = nla_policy { type_: NLA_U32, len: 0 };
    policy[IEEE802154_ATTR_LLSEC_KEY_BYTES] = nla_policy { type_: 0, len: 16 };
    policy[IEEE802154_ATTR_LLSEC_KEY_USAGE_FRAME_TYPES] = nla_policy { type_: NLA_U8, len: 0 };
    policy[IEEE802154_ATTR_LLSEC_KEY_USAGE_COMMANDS] = nla_policy { type_: 0, len: 258 / 8 };
    policy[IEEE802154_ATTR_LLSEC_FRAME_TYPE] = nla_policy { type_: NLA_U8, len: 0 };
    policy[IEEE802154_ATTR_LLSEC_CMD_FRAME_ID] = nla_policy { type_: NLA_U8, len: 0 };
    policy[IEEE802154_ATTR_LLSEC_SECLEVELS] = nla_policy { type_: NLA_U8, len: 0 };
    policy[IEEE802154_ATTR_LLSEC_DEV_OVERRIDE] = nla_policy { type_: NLA_U8, len: 0 };
    policy[IEEE802154_ATTR_LLSEC_DEV_KEY_MODE] = nla_policy { type_: NLA_U8, len: 0 };

    policy
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
