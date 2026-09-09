/* Rust translation of linux/netfilter/nf_tables.h. */
/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub const NFT_NAME_MAXLEN: _ = 256;
pub const NFT_TABLE_MAXNAMELEN: _ = NFT_NAME_MAXLEN;
pub const NFT_CHAIN_MAXNAMELEN: _ = NFT_NAME_MAXLEN;
pub const NFT_SET_MAXNAMELEN: _ = NFT_NAME_MAXLEN;
pub const NFT_OBJ_MAXNAMELEN: _ = NFT_NAME_MAXLEN;
pub const NFT_USERDATA_MAXLEN: _ = 256;
pub const NFT_OSF_MAXGENRELEN: _ = 16;

/**
 * enum nft_registers - nf_tables registers
 *
 * nf_tables used to have five registers: a verdict register and four data
 * registers of size 16. The data registers have been changed to 16 registers
 * of size 4. For compatibility reasons, the NFT_REG_[1-4] registers still
 * map to areas of size 16, the 4 byte registers are addressed using
 * NFT_REG32_00 - NFT_REG32_15.
 */
pub mod nft_registers {
    pub const NFT_REG_VERDICT: _ = 0;
    pub const NFT_REG_1: _ = 1;
    pub const NFT_REG_2: _ = 2;
    pub const NFT_REG_3: _ = 3;
    pub const NFT_REG_4: _ = 4;
    pub const __NFT_REG_MAX: _ = 5;

    pub const NFT_REG32_00: _ = 8;
    pub const NFT_REG32_01: _ = 9;
    pub const NFT_REG32_02: _ = 10;
    pub const NFT_REG32_03: _ = 11;
    pub const NFT_REG32_04: _ = 12;
    pub const NFT_REG32_05: _ = 13;
    pub const NFT_REG32_06: _ = 14;
    pub const NFT_REG32_07: _ = 15;
    pub const NFT_REG32_08: _ = 16;
    pub const NFT_REG32_09: _ = 17;
    pub const NFT_REG32_10: _ = 18;
    pub const NFT_REG32_11: _ = 19;
    pub const NFT_REG32_12: _ = 20;
    pub const NFT_REG32_13: _ = 21;
    pub const NFT_REG32_14: _ = 22;
    pub const NFT_REG32_15: _ = 23;
}
pub const NFT_REG_MAX: _ = (__NFT_REG_MAX - 1);

pub const NFT_REG32_MAX: _ = NFT_REG32_15;

pub const NFT_REG_SIZE: _ = 16;
pub const NFT_REG32_SIZE: _ = 4;
pub const NFT_REG32_COUNT: _ = (NFT_REG32_15 - NFT_REG32_00 + 1);

/**
 * enum nft_verdicts - nf_tables internal verdicts
 *
 * @NFT_CONTINUE: continue evaluation of the current rule
 * @NFT_BREAK: terminate evaluation of the current rule
 * @NFT_JUMP: push the current chain on the jump stack and jump to a chain
 * @NFT_GOTO: jump to a chain without pushing the current chain on the jump stack
 * @NFT_RETURN: return to the topmost chain on the jump stack
 *
 * The nf_tables verdicts share their numeric space with the netfilter verdicts.
 */
pub mod nft_verdicts {
    pub const NFT_CONTINUE: _ = -1;
    pub const NFT_BREAK: _ = -2;
    pub const NFT_JUMP: _ = -3;
    pub const NFT_GOTO: _ = -4;
    pub const NFT_RETURN: _ = -5;
}

/**
 * enum nf_tables_msg_types - nf_tables netlink message types
 *
 * @NFT_MSG_NEWTABLE: create a new table (enum nft_table_attributes)
 * @NFT_MSG_GETTABLE: get a table (enum nft_table_attributes)
 * @NFT_MSG_DELTABLE: delete a table (enum nft_table_attributes)
 * @NFT_MSG_NEWCHAIN: create a new chain (enum nft_chain_attributes)
 * @NFT_MSG_GETCHAIN: get a chain (enum nft_chain_attributes)
 * @NFT_MSG_DELCHAIN: delete a chain (enum nft_chain_attributes)
 * @NFT_MSG_NEWRULE: create a new rule (enum nft_rule_attributes)
 * @NFT_MSG_GETRULE: get a rule (enum nft_rule_attributes)
 * @NFT_MSG_DELRULE: delete a rule (enum nft_rule_attributes)
 * @NFT_MSG_NEWSET: create a new set (enum nft_set_attributes)
 * @NFT_MSG_GETSET: get a set (enum nft_set_attributes)
 * @NFT_MSG_DELSET: delete a set (enum nft_set_attributes)
 * @NFT_MSG_NEWSETELEM: create a new set element (enum nft_set_elem_attributes)
 * @NFT_MSG_GETSETELEM: get a set element (enum nft_set_elem_attributes)
 * @NFT_MSG_DELSETELEM: delete a set element (enum nft_set_elem_attributes)
 * @NFT_MSG_NEWGEN: announce a new generation, only for events (enum nft_gen_attributes)
 * @NFT_MSG_GETGEN: get the rule-set generation (enum nft_gen_attributes)
 * @NFT_MSG_TRACE: trace event (enum nft_trace_attributes)
 * @NFT_MSG_NEWOBJ: create a stateful object (enum nft_obj_attributes)
 * @NFT_MSG_GETOBJ: get a stateful object (enum nft_obj_attributes)
 * @NFT_MSG_DELOBJ: delete a stateful object (enum nft_obj_attributes)
 * @NFT_MSG_GETOBJ_RESET: get and reset a stateful object (enum nft_obj_attributes)
 * @NFT_MSG_NEWFLOWTABLE: add new flow table (enum nft_flowtable_attributes)
 * @NFT_MSG_GETFLOWTABLE: get flow table (enum nft_flowtable_attributes)
 * @NFT_MSG_DELFLOWTABLE: delete flow table (enum nft_flowtable_attributes)
 * @NFT_MSG_GETRULE_RESET: get rules and reset stateful expressions (enum nft_obj_attributes)
 * @NFT_MSG_DESTROYTABLE: destroy a table (enum nft_table_attributes)
 * @NFT_MSG_DESTROYCHAIN: destroy a chain (enum nft_chain_attributes)
 * @NFT_MSG_DESTROYRULE: destroy a rule (enum nft_rule_attributes)
 * @NFT_MSG_DESTROYSET: destroy a set (enum nft_set_attributes)
 * @NFT_MSG_DESTROYSETELEM: destroy a set element (enum nft_set_elem_attributes)
 * @NFT_MSG_DESTROYOBJ: destroy a stateful object (enum nft_object_attributes)
 * @NFT_MSG_DESTROYFLOWTABLE: destroy flow table (enum nft_flowtable_attributes)
 * @NFT_MSG_GETSETELEM_RESET: get set elements and reset attached stateful expressions (enum nft_set_elem_attributes)
 */
pub mod nf_tables_msg_types {
    pub const NFT_MSG_NEWTABLE: _ = 0;
    pub const NFT_MSG_GETTABLE: _ = 1;
    pub const NFT_MSG_DELTABLE: _ = 2;
    pub const NFT_MSG_NEWCHAIN: _ = 3;
    pub const NFT_MSG_GETCHAIN: _ = 4;
    pub const NFT_MSG_DELCHAIN: _ = 5;
    pub const NFT_MSG_NEWRULE: _ = 6;
    pub const NFT_MSG_GETRULE: _ = 7;
    pub const NFT_MSG_DELRULE: _ = 8;
    pub const NFT_MSG_NEWSET: _ = 9;
    pub const NFT_MSG_GETSET: _ = 10;
    pub const NFT_MSG_DELSET: _ = 11;
    pub const NFT_MSG_NEWSETELEM: _ = 12;
    pub const NFT_MSG_GETSETELEM: _ = 13;
    pub const NFT_MSG_DELSETELEM: _ = 14;
    pub const NFT_MSG_NEWGEN: _ = 15;
    pub const NFT_MSG_GETGEN: _ = 16;
    pub const NFT_MSG_TRACE: _ = 17;
    pub const NFT_MSG_NEWOBJ: _ = 18;
    pub const NFT_MSG_GETOBJ: _ = 19;
    pub const NFT_MSG_DELOBJ: _ = 20;
    pub const NFT_MSG_GETOBJ_RESET: _ = 21;
    pub const NFT_MSG_NEWFLOWTABLE: _ = 22;
    pub const NFT_MSG_GETFLOWTABLE: _ = 23;
    pub const NFT_MSG_DELFLOWTABLE: _ = 24;
    pub const NFT_MSG_GETRULE_RESET: _ = 25;
    pub const NFT_MSG_DESTROYTABLE: _ = 26;
    pub const NFT_MSG_DESTROYCHAIN: _ = 27;
    pub const NFT_MSG_DESTROYRULE: _ = 28;
    pub const NFT_MSG_DESTROYSET: _ = 29;
    pub const NFT_MSG_DESTROYSETELEM: _ = 30;
    pub const NFT_MSG_DESTROYOBJ: _ = 31;
    pub const NFT_MSG_DESTROYFLOWTABLE: _ = 32;
    pub const NFT_MSG_GETSETELEM_RESET: _ = 33;
    pub const NFT_MSG_MAX: _ = 34;
}

/**
 * enum nft_list_attributes - nf_tables generic list netlink attributes
 *
 * @NFTA_LIST_ELEM: list element (NLA_NESTED)
 */
pub mod nft_list_attributes {
    pub const NFTA_LIST_UNSPEC: _ = 0;
    pub const NFTA_LIST_ELEM: _ = 1;
    pub const __NFTA_LIST_MAX: _ = 2;
}
pub const NFTA_LIST_MAX: _ = (__NFTA_LIST_MAX - 1);

/**
 * enum nft_hook_attributes - nf_tables netfilter hook netlink attributes
 *
 * @NFTA_HOOK_HOOKNUM: netfilter hook number (NLA_U32)
 * @NFTA_HOOK_PRIORITY: netfilter hook priority (NLA_U32)
 * @NFTA_HOOK_DEV: netdevice name (NLA_STRING)
 * @NFTA_HOOK_DEVS: list of netdevices (NLA_NESTED)
 */
pub mod nft_hook_attributes {
    pub const NFTA_HOOK_UNSPEC: _ = 0;
    pub const NFTA_HOOK_HOOKNUM: _ = 1;
    pub const NFTA_HOOK_PRIORITY: _ = 2;
    pub const NFTA_HOOK_DEV: _ = 3;
    pub const NFTA_HOOK_DEVS: _ = 4;
    pub const __NFTA_HOOK_MAX: _ = 5;
}
pub const NFTA_HOOK_MAX: _ = (__NFTA_HOOK_MAX - 1);

/**
 * enum nft_table_flags - nf_tables table flags
 *
 * @NFT_TABLE_F_DORMANT: this table is not active
 * @NFT_TABLE_F_OWNER:   this table is owned by a process
 * @NFT_TABLE_F_PERSIST: this table shall outlive its owner
 */
pub mod nft_table_flags {
    pub const NFT_TABLE_F_DORMANT: _ = 0x1;
    pub const NFT_TABLE_F_OWNER: _ = 0x2;
    pub const NFT_TABLE_F_PERSIST: _ = 0x4;
}
pub const NFT_TABLE_F_MASK: _ = (NFT_TABLE_F_DORMANT | \;
				 NFT_TABLE_F_OWNER | \
				 NFT_TABLE_F_PERSIST)

/**
 * enum nft_table_attributes - nf_tables table netlink attributes
 *
 * @NFTA_TABLE_NAME: name of the table (NLA_STRING)
 * @NFTA_TABLE_FLAGS: bitmask of enum nft_table_flags (NLA_U32)
 * @NFTA_TABLE_USE: number of chains in this table (NLA_U32)
 * @NFTA_TABLE_USERDATA: user data (NLA_BINARY)
 * @NFTA_TABLE_OWNER: owner of this table through netlink portID (NLA_U32)
 */
pub mod nft_table_attributes {
    pub const NFTA_TABLE_UNSPEC: _ = 0;
    pub const NFTA_TABLE_NAME: _ = 1;
    pub const NFTA_TABLE_FLAGS: _ = 2;
    pub const NFTA_TABLE_USE: _ = 3;
    pub const NFTA_TABLE_HANDLE: _ = 4;
    pub const NFTA_TABLE_PAD: _ = 5;
    pub const NFTA_TABLE_USERDATA: _ = 6;
    pub const NFTA_TABLE_OWNER: _ = 7;
    pub const __NFTA_TABLE_MAX: _ = 8;
}
pub const NFTA_TABLE_MAX: _ = (__NFTA_TABLE_MAX - 1);

pub mod nft_chain_flags {
    pub const NFT_CHAIN_BASE: _ = (1 << 0);
    pub const NFT_CHAIN_HW_OFFLOAD: _ = (1 << 1);
    pub const NFT_CHAIN_BINDING: _ = (1 << 2);
}
pub const NFT_CHAIN_FLAGS: _ = (NFT_CHAIN_BASE		| \;
				 NFT_CHAIN_HW_OFFLOAD	| \
				 NFT_CHAIN_BINDING)

/**
 * enum nft_chain_attributes - nf_tables chain netlink attributes
 *
 * @NFTA_CHAIN_TABLE: name of the table containing the chain (NLA_STRING)
 * @NFTA_CHAIN_HANDLE: numeric handle of the chain (NLA_U64)
 * @NFTA_CHAIN_NAME: name of the chain (NLA_STRING)
 * @NFTA_CHAIN_HOOK: hook specification for basechains (NLA_NESTED: nft_hook_attributes)
 * @NFTA_CHAIN_POLICY: numeric policy of the chain (NLA_U32)
 * @NFTA_CHAIN_USE: number of references to this chain (NLA_U32)
 * @NFTA_CHAIN_TYPE: type name of the string (NLA_NUL_STRING)
 * @NFTA_CHAIN_COUNTERS: counter specification of the chain (NLA_NESTED: nft_counter_attributes)
 * @NFTA_CHAIN_FLAGS: chain flags
 * @NFTA_CHAIN_ID: uniquely identifies a chain in a transaction (NLA_U32)
 * @NFTA_CHAIN_USERDATA: user data (NLA_BINARY)
 */
pub mod nft_chain_attributes {
    pub const NFTA_CHAIN_UNSPEC: _ = 0;
    pub const NFTA_CHAIN_TABLE: _ = 1;
    pub const NFTA_CHAIN_HANDLE: _ = 2;
    pub const NFTA_CHAIN_NAME: _ = 3;
    pub const NFTA_CHAIN_HOOK: _ = 4;
    pub const NFTA_CHAIN_POLICY: _ = 5;
    pub const NFTA_CHAIN_USE: _ = 6;
    pub const NFTA_CHAIN_TYPE: _ = 7;
    pub const NFTA_CHAIN_COUNTERS: _ = 8;
    pub const NFTA_CHAIN_PAD: _ = 9;
    pub const NFTA_CHAIN_FLAGS: _ = 10;
    pub const NFTA_CHAIN_ID: _ = 11;
    pub const NFTA_CHAIN_USERDATA: _ = 12;
    pub const __NFTA_CHAIN_MAX: _ = 13;
}
pub const NFTA_CHAIN_MAX: _ = (__NFTA_CHAIN_MAX - 1);

/**
 * enum nft_rule_attributes - nf_tables rule netlink attributes
 *
 * @NFTA_RULE_TABLE: name of the table containing the rule (NLA_STRING)
 * @NFTA_RULE_CHAIN: name of the chain containing the rule (NLA_STRING)
 * @NFTA_RULE_HANDLE: numeric handle of the rule (NLA_U64)
 * @NFTA_RULE_EXPRESSIONS: list of expressions (NLA_NESTED: nft_expr_attributes)
 * @NFTA_RULE_COMPAT: compatibility specifications of the rule (NLA_NESTED: nft_rule_compat_attributes)
 * @NFTA_RULE_POSITION: numeric handle of the previous rule (NLA_U64)
 * @NFTA_RULE_USERDATA: user data (NLA_BINARY, NFT_USERDATA_MAXLEN)
 * @NFTA_RULE_ID: uniquely identifies a rule in a transaction (NLA_U32)
 * @NFTA_RULE_POSITION_ID: transaction unique identifier of the previous rule (NLA_U32)
 * @NFTA_RULE_CHAIN_ID: add the rule to chain by ID, alternative to @NFTA_RULE_CHAIN (NLA_U32)
 */
pub mod nft_rule_attributes {
    pub const NFTA_RULE_UNSPEC: _ = 0;
    pub const NFTA_RULE_TABLE: _ = 1;
    pub const NFTA_RULE_CHAIN: _ = 2;
    pub const NFTA_RULE_HANDLE: _ = 3;
    pub const NFTA_RULE_EXPRESSIONS: _ = 4;
    pub const NFTA_RULE_COMPAT: _ = 5;
    pub const NFTA_RULE_POSITION: _ = 6;
    pub const NFTA_RULE_USERDATA: _ = 7;
    pub const NFTA_RULE_PAD: _ = 8;
    pub const NFTA_RULE_ID: _ = 9;
    pub const NFTA_RULE_POSITION_ID: _ = 10;
    pub const NFTA_RULE_CHAIN_ID: _ = 11;
    pub const __NFTA_RULE_MAX: _ = 12;
}
pub const NFTA_RULE_MAX: _ = (__NFTA_RULE_MAX - 1);

/**
 * enum nft_rule_compat_flags - nf_tables rule compat flags
 *
 * @NFT_RULE_COMPAT_F_UNUSED: unused
 * @NFT_RULE_COMPAT_F_INV: invert the check result
 */
pub mod nft_rule_compat_flags {
    pub const NFT_RULE_COMPAT_F_UNUSED: _ = (1 << 0);
    pub const NFT_RULE_COMPAT_F_INV: _ = (1 << 1);
    pub const NFT_RULE_COMPAT_F_MASK: _ = NFT_RULE_COMPAT_F_INV;
}

/**
 * enum nft_rule_compat_attributes - nf_tables rule compat attributes
 *
 * @NFTA_RULE_COMPAT_PROTO: numeric value of handled protocol (NLA_U32)
 * @NFTA_RULE_COMPAT_FLAGS: bitmask of enum nft_rule_compat_flags (NLA_U32)
 */
pub mod nft_rule_compat_attributes {
    pub const NFTA_RULE_COMPAT_UNSPEC: _ = 0;
    pub const NFTA_RULE_COMPAT_PROTO: _ = 1;
    pub const NFTA_RULE_COMPAT_FLAGS: _ = 2;
    pub const __NFTA_RULE_COMPAT_MAX: _ = 3;
}
pub const NFTA_RULE_COMPAT_MAX: _ = (__NFTA_RULE_COMPAT_MAX - 1);

/**
 * enum nft_set_flags - nf_tables set flags
 *
 * @NFT_SET_ANONYMOUS: name allocation, automatic cleanup on unlink
 * @NFT_SET_CONSTANT: set contents may not change while bound
 * @NFT_SET_INTERVAL: set contains intervals
 * @NFT_SET_MAP: set is used as a dictionary
 * @NFT_SET_TIMEOUT: set uses timeouts
 * @NFT_SET_EVAL: set can be updated from the evaluation path
 * @NFT_SET_OBJECT: set contains stateful objects
 * @NFT_SET_CONCAT: set contains a concatenation
 * @NFT_SET_EXPR: set contains expressions
 */
pub mod nft_set_flags {
    pub const NFT_SET_ANONYMOUS: _ = 0x1;
    pub const NFT_SET_CONSTANT: _ = 0x2;
    pub const NFT_SET_INTERVAL: _ = 0x4;
    pub const NFT_SET_MAP: _ = 0x8;
    pub const NFT_SET_TIMEOUT: _ = 0x10;
    pub const NFT_SET_EVAL: _ = 0x20;
    pub const NFT_SET_OBJECT: _ = 0x40;
    pub const NFT_SET_CONCAT: _ = 0x80;
    pub const NFT_SET_EXPR: _ = 0x100;
}

/**
 * enum nft_set_policies - set selection policy
 *
 * @NFT_SET_POL_PERFORMANCE: prefer high performance over low memory use
 * @NFT_SET_POL_MEMORY: prefer low memory use over high performance
 */
pub mod nft_set_policies {
    pub const NFT_SET_POL_PERFORMANCE: _ = 0;
    pub const NFT_SET_POL_MEMORY: _ = 1;
}

/**
 * enum nft_set_desc_attributes - set element description
 *
 * @NFTA_SET_DESC_SIZE: number of elements in set (NLA_U32)
 * @NFTA_SET_DESC_CONCAT: description of field concatenation (NLA_NESTED)
 */
pub mod nft_set_desc_attributes {
    pub const NFTA_SET_DESC_UNSPEC: _ = 0;
    pub const NFTA_SET_DESC_SIZE: _ = 1;
    pub const NFTA_SET_DESC_CONCAT: _ = 2;
    pub const __NFTA_SET_DESC_MAX: _ = 3;
}
pub const NFTA_SET_DESC_MAX: _ = (__NFTA_SET_DESC_MAX - 1);

/**
 * enum nft_set_field_attributes - attributes of concatenated fields
 *
 * @NFTA_SET_FIELD_LEN: length of single field, in bits (NLA_U32)
 */
pub mod nft_set_field_attributes {
    pub const NFTA_SET_FIELD_UNSPEC: _ = 0;
    pub const NFTA_SET_FIELD_LEN: _ = 1;
    pub const __NFTA_SET_FIELD_MAX: _ = 2;
}
pub const NFTA_SET_FIELD_MAX: _ = (__NFTA_SET_FIELD_MAX - 1);

/**
 * enum nft_set_attributes - nf_tables set netlink attributes
 *
 * @NFTA_SET_TABLE: table name (NLA_STRING)
 * @NFTA_SET_NAME: set name (NLA_STRING)
 * @NFTA_SET_FLAGS: bitmask of enum nft_set_flags (NLA_U32)
 * @NFTA_SET_KEY_TYPE: key data type, informational purpose only (NLA_U32)
 * @NFTA_SET_KEY_LEN: key data length (NLA_U32)
 * @NFTA_SET_DATA_TYPE: mapping data type (NLA_U32)
 * @NFTA_SET_DATA_LEN: mapping data length (NLA_U32)
 * @NFTA_SET_POLICY: selection policy (NLA_U32)
 * @NFTA_SET_DESC: set description (NLA_NESTED)
 * @NFTA_SET_ID: uniquely identifies a set in a transaction (NLA_U32)
 * @NFTA_SET_TIMEOUT: default timeout value (NLA_U64)
 * @NFTA_SET_GC_INTERVAL: garbage collection interval (NLA_U32)
 * @NFTA_SET_USERDATA: user data (NLA_BINARY)
 * @NFTA_SET_OBJ_TYPE: stateful object type (NLA_U32: NFT_OBJECT_*)
 * @NFTA_SET_HANDLE: set handle (NLA_U64)
 * @NFTA_SET_EXPR: set expression (NLA_NESTED: nft_expr_attributes)
 * @NFTA_SET_EXPRESSIONS: list of expressions (NLA_NESTED: nft_list_attributes)
 * @NFTA_SET_TYPE: set backend type (NLA_STRING)
 * @NFTA_SET_COUNT: number of set elements (NLA_U32)
 */
pub mod nft_set_attributes {
    pub const NFTA_SET_UNSPEC: _ = 0;
    pub const NFTA_SET_TABLE: _ = 1;
    pub const NFTA_SET_NAME: _ = 2;
    pub const NFTA_SET_FLAGS: _ = 3;
    pub const NFTA_SET_KEY_TYPE: _ = 4;
    pub const NFTA_SET_KEY_LEN: _ = 5;
    pub const NFTA_SET_DATA_TYPE: _ = 6;
    pub const NFTA_SET_DATA_LEN: _ = 7;
    pub const NFTA_SET_POLICY: _ = 8;
    pub const NFTA_SET_DESC: _ = 9;
    pub const NFTA_SET_ID: _ = 10;
    pub const NFTA_SET_TIMEOUT: _ = 11;
    pub const NFTA_SET_GC_INTERVAL: _ = 12;
    pub const NFTA_SET_USERDATA: _ = 13;
    pub const NFTA_SET_PAD: _ = 14;
    pub const NFTA_SET_OBJ_TYPE: _ = 15;
    pub const NFTA_SET_HANDLE: _ = 16;
    pub const NFTA_SET_EXPR: _ = 17;
    pub const NFTA_SET_EXPRESSIONS: _ = 18;
    pub const NFTA_SET_TYPE: _ = 19;
    pub const NFTA_SET_COUNT: _ = 20;
    pub const __NFTA_SET_MAX: _ = 21;
}
pub const NFTA_SET_MAX: _ = (__NFTA_SET_MAX - 1);

/**
 * enum nft_set_elem_flags - nf_tables set element flags
 *
 * @NFT_SET_ELEM_INTERVAL_END: element ends the previous interval
 * @NFT_SET_ELEM_CATCHALL: special catch-all element
 */
pub mod nft_set_elem_flags {
    pub const NFT_SET_ELEM_INTERVAL_END: _ = 0x1;
    pub const NFT_SET_ELEM_CATCHALL: _ = 0x2;
}

/**
 * enum nft_set_elem_attributes - nf_tables set element netlink attributes
 *
 * @NFTA_SET_ELEM_KEY: key value (NLA_NESTED: nft_data)
 * @NFTA_SET_ELEM_DATA: data value of mapping (NLA_NESTED: nft_data_attributes)
 * @NFTA_SET_ELEM_FLAGS: bitmask of nft_set_elem_flags (NLA_U32)
 * @NFTA_SET_ELEM_TIMEOUT: timeout value, zero means never times out (NLA_U64)
 * @NFTA_SET_ELEM_EXPIRATION: expiration time (NLA_U64)
 * @NFTA_SET_ELEM_USERDATA: user data (NLA_BINARY)
 * @NFTA_SET_ELEM_EXPR: expression (NLA_NESTED: nft_expr_attributes)
 * @NFTA_SET_ELEM_OBJREF: stateful object reference (NLA_STRING)
 * @NFTA_SET_ELEM_KEY_END: closing key value (NLA_NESTED: nft_data)
 * @NFTA_SET_ELEM_EXPRESSIONS: list of expressions (NLA_NESTED: nft_list_attributes)
 */
pub mod nft_set_elem_attributes {
    pub const NFTA_SET_ELEM_UNSPEC: _ = 0;
    pub const NFTA_SET_ELEM_KEY: _ = 1;
    pub const NFTA_SET_ELEM_DATA: _ = 2;
    pub const NFTA_SET_ELEM_FLAGS: _ = 3;
    pub const NFTA_SET_ELEM_TIMEOUT: _ = 4;
    pub const NFTA_SET_ELEM_EXPIRATION: _ = 5;
    pub const NFTA_SET_ELEM_USERDATA: _ = 6;
    pub const NFTA_SET_ELEM_EXPR: _ = 7;
    pub const NFTA_SET_ELEM_PAD: _ = 8;
    pub const NFTA_SET_ELEM_OBJREF: _ = 9;
    pub const NFTA_SET_ELEM_KEY_END: _ = 10;
    pub const NFTA_SET_ELEM_EXPRESSIONS: _ = 11;
    pub const __NFTA_SET_ELEM_MAX: _ = 12;
}
pub const NFTA_SET_ELEM_MAX: _ = (__NFTA_SET_ELEM_MAX - 1);

/**
 * enum nft_set_elem_list_attributes - nf_tables set element list netlink attributes
 *
 * @NFTA_SET_ELEM_LIST_TABLE: table of the set to be changed (NLA_STRING)
 * @NFTA_SET_ELEM_LIST_SET: name of the set to be changed (NLA_STRING)
 * @NFTA_SET_ELEM_LIST_ELEMENTS: list of set elements (NLA_NESTED: nft_set_elem_attributes)
 * @NFTA_SET_ELEM_LIST_SET_ID: uniquely identifies a set in a transaction (NLA_U32)
 */
pub mod nft_set_elem_list_attributes {
    pub const NFTA_SET_ELEM_LIST_UNSPEC: _ = 0;
    pub const NFTA_SET_ELEM_LIST_TABLE: _ = 1;
    pub const NFTA_SET_ELEM_LIST_SET: _ = 2;
    pub const NFTA_SET_ELEM_LIST_ELEMENTS: _ = 3;
    pub const NFTA_SET_ELEM_LIST_SET_ID: _ = 4;
    pub const __NFTA_SET_ELEM_LIST_MAX: _ = 5;
}
pub const NFTA_SET_ELEM_LIST_MAX: _ = (__NFTA_SET_ELEM_LIST_MAX - 1);

/**
 * enum nft_data_types - nf_tables data types
 *
 * @NFT_DATA_VALUE: generic data
 * @NFT_DATA_VERDICT: netfilter verdict
 *
 * The type of data is usually determined by the kernel directly and is not
 * explicitly specified by userspace. The only difference are sets, where
 * userspace specifies the key and mapping data types.
 *
 * The values 0xffffff00-0xffffffff are reserved for internally used types.
 * The remaining range can be freely used by userspace to encode types, all
 * values are equivalent to NFT_DATA_VALUE.
 */
pub mod nft_data_types {
    pub const NFT_DATA_VALUE: _ = 0;
    pub const NFT_DATA_VERDICT: _ = 0xffffff00;
}

pub const NFT_DATA_RESERVED_MASK: _ = 0xffffff00;

/**
 * enum nft_data_attributes - nf_tables data netlink attributes
 *
 * @NFTA_DATA_VALUE: generic data (NLA_BINARY)
 * @NFTA_DATA_VERDICT: nf_tables verdict (NLA_NESTED: nft_verdict_attributes)
 */
pub mod nft_data_attributes {
    pub const NFTA_DATA_UNSPEC: _ = 0;
    pub const NFTA_DATA_VALUE: _ = 1;
    pub const NFTA_DATA_VERDICT: _ = 2;
    pub const __NFTA_DATA_MAX: _ = 3;
}
pub const NFTA_DATA_MAX: _ = (__NFTA_DATA_MAX - 1);

/* Maximum length of a value */
pub const NFT_DATA_VALUE_MAXLEN: _ = 64;

/**
 * enum nft_verdict_attributes - nf_tables verdict netlink attributes
 *
 * @NFTA_VERDICT_CODE: nf_tables verdict (NLA_U32: enum nft_verdicts)
 * @NFTA_VERDICT_CHAIN: jump target chain name (NLA_STRING)
 * @NFTA_VERDICT_CHAIN_ID: jump target chain ID (NLA_U32)
 */
pub mod nft_verdict_attributes {
    pub const NFTA_VERDICT_UNSPEC: _ = 0;
    pub const NFTA_VERDICT_CODE: _ = 1;
    pub const NFTA_VERDICT_CHAIN: _ = 2;
    pub const NFTA_VERDICT_CHAIN_ID: _ = 3;
    pub const __NFTA_VERDICT_MAX: _ = 4;
}
pub const NFTA_VERDICT_MAX: _ = (__NFTA_VERDICT_MAX - 1);

/**
 * enum nft_expr_attributes - nf_tables expression netlink attributes
 *
 * @NFTA_EXPR_NAME: name of the expression type (NLA_STRING)
 * @NFTA_EXPR_DATA: type specific data (NLA_NESTED)
 */
pub mod nft_expr_attributes {
    pub const NFTA_EXPR_UNSPEC: _ = 0;
    pub const NFTA_EXPR_NAME: _ = 1;
    pub const NFTA_EXPR_DATA: _ = 2;
    pub const __NFTA_EXPR_MAX: _ = 3;
}
pub const NFTA_EXPR_MAX: _ = (__NFTA_EXPR_MAX - 1);

/**
 * enum nft_immediate_attributes - nf_tables immediate expression netlink attributes
 *
 * @NFTA_IMMEDIATE_DREG: destination register to load data into (NLA_U32)
 * @NFTA_IMMEDIATE_DATA: data to load (NLA_NESTED: nft_data_attributes)
 */
pub mod nft_immediate_attributes {
    pub const NFTA_IMMEDIATE_UNSPEC: _ = 0;
    pub const NFTA_IMMEDIATE_DREG: _ = 1;
    pub const NFTA_IMMEDIATE_DATA: _ = 2;
    pub const __NFTA_IMMEDIATE_MAX: _ = 3;
}
pub const NFTA_IMMEDIATE_MAX: _ = (__NFTA_IMMEDIATE_MAX - 1);

/**
 * enum nft_bitwise_ops - nf_tables bitwise operations
 *
 * @NFT_BITWISE_MASK_XOR: mask-and-xor operation used to implement NOT, AND, OR
 *                        and XOR boolean operations
 * @NFT_BITWISE_LSHIFT: left-shift operation
 * @NFT_BITWISE_RSHIFT: right-shift operation
 * @NFT_BITWISE_AND: and operation
 * @NFT_BITWISE_OR: or operation
 * @NFT_BITWISE_XOR: xor operation
 */
pub mod nft_bitwise_ops {
    pub const NFT_BITWISE_MASK_XOR: _ = 0;
    pub const NFT_BITWISE_LSHIFT: _ = 1;
    pub const NFT_BITWISE_RSHIFT: _ = 2;
    pub const NFT_BITWISE_AND: _ = 3;
    pub const NFT_BITWISE_OR: _ = 4;
    pub const NFT_BITWISE_XOR: _ = 5;
}
/*
 * Old name for NFT_BITWISE_MASK_XOR.  Retained for backwards-compatibility.
 */
pub const NFT_BITWISE_BOOL: _ = NFT_BITWISE_MASK_XOR;

/**
 * enum nft_bitwise_attributes - nf_tables bitwise expression netlink attributes
 *
 * @NFTA_BITWISE_SREG: source register (NLA_U32: nft_registers)
 * @NFTA_BITWISE_DREG: destination register (NLA_U32: nft_registers)
 * @NFTA_BITWISE_LEN: length of operands (NLA_U32)
 * @NFTA_BITWISE_MASK: mask value (NLA_NESTED: nft_data_attributes)
 * @NFTA_BITWISE_XOR: xor value (NLA_NESTED: nft_data_attributes)
 * @NFTA_BITWISE_OP: type of operation (NLA_U32: nft_bitwise_ops)
 * @NFTA_BITWISE_DATA: argument for non-boolean operations
 *                     (NLA_NESTED: nft_data_attributes)
 * @NFTA_BITWISE_SREG2: second source register (NLA_U32: nft_registers)
 *
 * The bitwise expression supports boolean and shift operations.  It implements
 * the boolean operations by performing the following operation:
 *
 * dreg = (sreg & mask) ^ xor
 *
 * with these mask and xor values:
 *
 * 		mask	xor
 * NOT:		1	1
 * OR:		~x	x
 * XOR:		1	x
 * AND:		x	0
 */
pub mod nft_bitwise_attributes {
    pub const NFTA_BITWISE_UNSPEC: _ = 0;
    pub const NFTA_BITWISE_SREG: _ = 1;
    pub const NFTA_BITWISE_DREG: _ = 2;
    pub const NFTA_BITWISE_LEN: _ = 3;
    pub const NFTA_BITWISE_MASK: _ = 4;
    pub const NFTA_BITWISE_XOR: _ = 5;
    pub const NFTA_BITWISE_OP: _ = 6;
    pub const NFTA_BITWISE_DATA: _ = 7;
    pub const NFTA_BITWISE_SREG2: _ = 8;
    pub const __NFTA_BITWISE_MAX: _ = 9;
}
pub const NFTA_BITWISE_MAX: _ = (__NFTA_BITWISE_MAX - 1);

/**
 * enum nft_byteorder_ops - nf_tables byteorder operators
 *
 * @NFT_BYTEORDER_NTOH: network to host operator
 * @NFT_BYTEORDER_HTON: host to network operator
 */
pub mod nft_byteorder_ops {
    pub const NFT_BYTEORDER_NTOH: _ = 0;
    pub const NFT_BYTEORDER_HTON: _ = 1;
}

/**
 * enum nft_byteorder_attributes - nf_tables byteorder expression netlink attributes
 *
 * @NFTA_BYTEORDER_SREG: source register (NLA_U32: nft_registers)
 * @NFTA_BYTEORDER_DREG: destination register (NLA_U32: nft_registers)
 * @NFTA_BYTEORDER_OP: operator (NLA_U32: enum nft_byteorder_ops)
 * @NFTA_BYTEORDER_LEN: length of the data (NLA_U32)
 * @NFTA_BYTEORDER_SIZE: data size in bytes (NLA_U32: 2 or 4)
 */
pub mod nft_byteorder_attributes {
    pub const NFTA_BYTEORDER_UNSPEC: _ = 0;
    pub const NFTA_BYTEORDER_SREG: _ = 1;
    pub const NFTA_BYTEORDER_DREG: _ = 2;
    pub const NFTA_BYTEORDER_OP: _ = 3;
    pub const NFTA_BYTEORDER_LEN: _ = 4;
    pub const NFTA_BYTEORDER_SIZE: _ = 5;
    pub const __NFTA_BYTEORDER_MAX: _ = 6;
}
pub const NFTA_BYTEORDER_MAX: _ = (__NFTA_BYTEORDER_MAX - 1);

/**
 * enum nft_cmp_ops - nf_tables relational operator
 *
 * @NFT_CMP_EQ: equal
 * @NFT_CMP_NEQ: not equal
 * @NFT_CMP_LT: less than
 * @NFT_CMP_LTE: less than or equal to
 * @NFT_CMP_GT: greater than
 * @NFT_CMP_GTE: greater than or equal to
 */
pub mod nft_cmp_ops {
    pub const NFT_CMP_EQ: _ = 0;
    pub const NFT_CMP_NEQ: _ = 1;
    pub const NFT_CMP_LT: _ = 2;
    pub const NFT_CMP_LTE: _ = 3;
    pub const NFT_CMP_GT: _ = 4;
    pub const NFT_CMP_GTE: _ = 5;
}

/**
 * enum nft_cmp_attributes - nf_tables cmp expression netlink attributes
 *
 * @NFTA_CMP_SREG: source register of data to compare (NLA_U32: nft_registers)
 * @NFTA_CMP_OP: cmp operation (NLA_U32: nft_cmp_ops)
 * @NFTA_CMP_DATA: data to compare against (NLA_NESTED: nft_data_attributes)
 */
pub mod nft_cmp_attributes {
    pub const NFTA_CMP_UNSPEC: _ = 0;
    pub const NFTA_CMP_SREG: _ = 1;
    pub const NFTA_CMP_OP: _ = 2;
    pub const NFTA_CMP_DATA: _ = 3;
    pub const __NFTA_CMP_MAX: _ = 4;
}
pub const NFTA_CMP_MAX: _ = (__NFTA_CMP_MAX - 1);

/**
 * enum nft_range_ops - nf_tables range operator
 *
 * @NFT_RANGE_EQ: equal
 * @NFT_RANGE_NEQ: not equal
 */
pub mod nft_range_ops {
    pub const NFT_RANGE_EQ: _ = 0;
    pub const NFT_RANGE_NEQ: _ = 1;
}

/**
 * enum nft_range_attributes - nf_tables range expression netlink attributes
 *
 * @NFTA_RANGE_SREG: source register of data to compare (NLA_U32: nft_registers)
 * @NFTA_RANGE_OP: cmp operation (NLA_U32: nft_range_ops)
 * @NFTA_RANGE_FROM_DATA: data range from (NLA_NESTED: nft_data_attributes)
 * @NFTA_RANGE_TO_DATA: data range to (NLA_NESTED: nft_data_attributes)
 */
pub mod nft_range_attributes {
    pub const NFTA_RANGE_UNSPEC: _ = 0;
    pub const NFTA_RANGE_SREG: _ = 1;
    pub const NFTA_RANGE_OP: _ = 2;
    pub const NFTA_RANGE_FROM_DATA: _ = 3;
    pub const NFTA_RANGE_TO_DATA: _ = 4;
    pub const __NFTA_RANGE_MAX: _ = 5;
}
pub const NFTA_RANGE_MAX: _ = (__NFTA_RANGE_MAX - 1);

pub mod nft_lookup_flags {
    pub const NFT_LOOKUP_F_INV: _ = (1 << 0);
}

/**
 * enum nft_lookup_attributes - nf_tables set lookup expression netlink attributes
 *
 * @NFTA_LOOKUP_SET: name of the set where to look for (NLA_STRING)
 * @NFTA_LOOKUP_SREG: source register of the data to look for (NLA_U32: nft_registers)
 * @NFTA_LOOKUP_DREG: destination register (NLA_U32: nft_registers)
 * @NFTA_LOOKUP_SET_ID: uniquely identifies a set in a transaction (NLA_U32)
 * @NFTA_LOOKUP_FLAGS: flags (NLA_U32: enum nft_lookup_flags)
 */
pub mod nft_lookup_attributes {
    pub const NFTA_LOOKUP_UNSPEC: _ = 0;
    pub const NFTA_LOOKUP_SET: _ = 1;
    pub const NFTA_LOOKUP_SREG: _ = 2;
    pub const NFTA_LOOKUP_DREG: _ = 3;
    pub const NFTA_LOOKUP_SET_ID: _ = 4;
    pub const NFTA_LOOKUP_FLAGS: _ = 5;
    pub const __NFTA_LOOKUP_MAX: _ = 6;
}
pub const NFTA_LOOKUP_MAX: _ = (__NFTA_LOOKUP_MAX - 1);

pub mod nft_dynset_ops {
    pub const NFT_DYNSET_OP_ADD: _ = 0;
    pub const NFT_DYNSET_OP_UPDATE: _ = 1;
    pub const NFT_DYNSET_OP_DELETE: _ = 2;
}

pub mod nft_dynset_flags {
    pub const NFT_DYNSET_F_INV: _ = (1 << 0);
    pub const NFT_DYNSET_F_EXPR: _ = (1 << 1);
}

/**
 * enum nft_dynset_attributes - dynset expression attributes
 *
 * @NFTA_DYNSET_SET_NAME: name of set the to add data to (NLA_STRING)
 * @NFTA_DYNSET_SET_ID: uniquely identifier of the set in the transaction (NLA_U32)
 * @NFTA_DYNSET_OP: operation (NLA_U32)
 * @NFTA_DYNSET_SREG_KEY: source register of the key (NLA_U32)
 * @NFTA_DYNSET_SREG_DATA: source register of the data (NLA_U32)
 * @NFTA_DYNSET_TIMEOUT: timeout value for the new element (NLA_U64)
 * @NFTA_DYNSET_EXPR: expression (NLA_NESTED: nft_expr_attributes)
 * @NFTA_DYNSET_FLAGS: flags (NLA_U32)
 * @NFTA_DYNSET_EXPRESSIONS: list of expressions (NLA_NESTED: nft_list_attributes)
 */
pub mod nft_dynset_attributes {
    pub const NFTA_DYNSET_UNSPEC: _ = 0;
    pub const NFTA_DYNSET_SET_NAME: _ = 1;
    pub const NFTA_DYNSET_SET_ID: _ = 2;
    pub const NFTA_DYNSET_OP: _ = 3;
    pub const NFTA_DYNSET_SREG_KEY: _ = 4;
    pub const NFTA_DYNSET_SREG_DATA: _ = 5;
    pub const NFTA_DYNSET_TIMEOUT: _ = 6;
    pub const NFTA_DYNSET_EXPR: _ = 7;
    pub const NFTA_DYNSET_PAD: _ = 8;
    pub const NFTA_DYNSET_FLAGS: _ = 9;
    pub const NFTA_DYNSET_EXPRESSIONS: _ = 10;
    pub const __NFTA_DYNSET_MAX: _ = 11;
}
pub const NFTA_DYNSET_MAX: _ = (__NFTA_DYNSET_MAX - 1);

/**
 * enum nft_payload_bases - nf_tables payload expression offset bases
 *
 * @NFT_PAYLOAD_LL_HEADER: link layer header
 * @NFT_PAYLOAD_NETWORK_HEADER: network header
 * @NFT_PAYLOAD_TRANSPORT_HEADER: transport header
 * @NFT_PAYLOAD_INNER_HEADER: inner header / payload
 */
pub mod nft_payload_bases {
    pub const NFT_PAYLOAD_LL_HEADER: _ = 0;
    pub const NFT_PAYLOAD_NETWORK_HEADER: _ = 1;
    pub const NFT_PAYLOAD_TRANSPORT_HEADER: _ = 2;
    pub const NFT_PAYLOAD_INNER_HEADER: _ = 3;
    pub const NFT_PAYLOAD_TUN_HEADER: _ = 4;
}

/**
 * enum nft_payload_csum_types - nf_tables payload expression checksum types
 *
 * @NFT_PAYLOAD_CSUM_NONE: no checksumming
 * @NFT_PAYLOAD_CSUM_INET: internet checksum (RFC 791)
 * @NFT_PAYLOAD_CSUM_SCTP: CRC-32c, for use in SCTP header (RFC 3309)
 */
pub mod nft_payload_csum_types {
    pub const NFT_PAYLOAD_CSUM_NONE: _ = 0;
    pub const NFT_PAYLOAD_CSUM_INET: _ = 1;
    pub const NFT_PAYLOAD_CSUM_SCTP: _ = 2;
}

pub mod nft_payload_csum_flags {
    pub const NFT_PAYLOAD_L4CSUM_PSEUDOHDR: _ = (1 << 0);
}

pub mod nft_inner_type {
    pub const NFT_INNER_UNSPEC: _ = 0;
    pub const NFT_INNER_VXLAN: _ = 1;
    pub const NFT_INNER_GENEVE: _ = 2;
}

pub mod nft_inner_flags {
    pub const NFT_INNER_HDRSIZE: _ = (1 << 0);
    pub const NFT_INNER_LL: _ = (1 << 1);
    pub const NFT_INNER_NH: _ = (1 << 2);
    pub const NFT_INNER_TH: _ = (1 << 3);
}
pub const NFT_INNER_MASK: _ = (NFT_INNER_HDRSIZE | NFT_INNER_LL | \;
				 NFT_INNER_NH | NFT_INNER_TH)

pub mod nft_inner_attributes {
    pub const NFTA_INNER_UNSPEC: _ = 0;
    pub const NFTA_INNER_NUM: _ = 1;
    pub const NFTA_INNER_TYPE: _ = 2;
    pub const NFTA_INNER_FLAGS: _ = 3;
    pub const NFTA_INNER_HDRSIZE: _ = 4;
    pub const NFTA_INNER_EXPR: _ = 5;
    pub const __NFTA_INNER_MAX: _ = 6;
}
pub const NFTA_INNER_MAX: _ = (__NFTA_INNER_MAX - 1);

/**
 * enum nft_payload_attributes - nf_tables payload expression netlink attributes
 *
 * @NFTA_PAYLOAD_DREG: destination register to load data into (NLA_U32: nft_registers)
 * @NFTA_PAYLOAD_BASE: payload base (NLA_U32: nft_payload_bases)
 * @NFTA_PAYLOAD_OFFSET: payload offset relative to base (NLA_U32)
 * @NFTA_PAYLOAD_LEN: payload length (NLA_U32)
 * @NFTA_PAYLOAD_SREG: source register to load data from (NLA_U32: nft_registers)
 * @NFTA_PAYLOAD_CSUM_TYPE: checksum type (NLA_U32)
 * @NFTA_PAYLOAD_CSUM_OFFSET: checksum offset relative to base (NLA_U32)
 * @NFTA_PAYLOAD_CSUM_FLAGS: checksum flags (NLA_U32)
 */
pub mod nft_payload_attributes {
    pub const NFTA_PAYLOAD_UNSPEC: _ = 0;
    pub const NFTA_PAYLOAD_DREG: _ = 1;
    pub const NFTA_PAYLOAD_BASE: _ = 2;
    pub const NFTA_PAYLOAD_OFFSET: _ = 3;
    pub const NFTA_PAYLOAD_LEN: _ = 4;
    pub const NFTA_PAYLOAD_SREG: _ = 5;
    pub const NFTA_PAYLOAD_CSUM_TYPE: _ = 6;
    pub const NFTA_PAYLOAD_CSUM_OFFSET: _ = 7;
    pub const NFTA_PAYLOAD_CSUM_FLAGS: _ = 8;
    pub const __NFTA_PAYLOAD_MAX: _ = 9;
}
pub const NFTA_PAYLOAD_MAX: _ = (__NFTA_PAYLOAD_MAX - 1);

pub mod nft_exthdr_flags {
    pub const NFT_EXTHDR_F_PRESENT: _ = (1 << 0);
}

/**
 * enum nft_exthdr_op - nf_tables match options
 *
 * @NFT_EXTHDR_OP_IPV6: match against ipv6 extension headers
 * @NFT_EXTHDR_OP_TCPOPT: match against tcp options
 * @NFT_EXTHDR_OP_IPV4: match against ipv4 options
 * @NFT_EXTHDR_OP_SCTP: match against sctp chunks
 * @NFT_EXTHDR_OP_DCCP: match against dccp options
 */
pub mod nft_exthdr_op {
    pub const NFT_EXTHDR_OP_IPV6: _ = 0;
    pub const NFT_EXTHDR_OP_TCPOPT: _ = 1;
    pub const NFT_EXTHDR_OP_IPV4: _ = 2;
    pub const NFT_EXTHDR_OP_SCTP: _ = 3;
    pub const NFT_EXTHDR_OP_DCCP: _ = 4;
    pub const __NFT_EXTHDR_OP_MAX: _ = 5;
}
pub const NFT_EXTHDR_OP_MAX: _ = (__NFT_EXTHDR_OP_MAX - 1);

/**
 * enum nft_exthdr_attributes - nf_tables extension header expression netlink attributes
 *
 * @NFTA_EXTHDR_DREG: destination register (NLA_U32: nft_registers)
 * @NFTA_EXTHDR_TYPE: extension header type (NLA_U8)
 * @NFTA_EXTHDR_OFFSET: extension header offset (NLA_U32)
 * @NFTA_EXTHDR_LEN: extension header length (NLA_U32)
 * @NFTA_EXTHDR_FLAGS: extension header flags (NLA_U32)
 * @NFTA_EXTHDR_OP: option match type (NLA_U32)
 * @NFTA_EXTHDR_SREG: source register (NLA_U32: nft_registers)
 */
pub mod nft_exthdr_attributes {
    pub const NFTA_EXTHDR_UNSPEC: _ = 0;
    pub const NFTA_EXTHDR_DREG: _ = 1;
    pub const NFTA_EXTHDR_TYPE: _ = 2;
    pub const NFTA_EXTHDR_OFFSET: _ = 3;
    pub const NFTA_EXTHDR_LEN: _ = 4;
    pub const NFTA_EXTHDR_FLAGS: _ = 5;
    pub const NFTA_EXTHDR_OP: _ = 6;
    pub const NFTA_EXTHDR_SREG: _ = 7;
    pub const __NFTA_EXTHDR_MAX: _ = 8;
}
pub const NFTA_EXTHDR_MAX: _ = (__NFTA_EXTHDR_MAX - 1);

/**
 * enum nft_meta_keys - nf_tables meta expression keys
 *
 * @NFT_META_LEN: packet length (skb->len)
 * @NFT_META_PROTOCOL: packet ethertype protocol (skb->protocol), invalid in OUTPUT
 * @NFT_META_PRIORITY: packet priority (skb->priority)
 * @NFT_META_MARK: packet mark (skb->mark)
 * @NFT_META_IIF: packet input interface index (dev->ifindex)
 * @NFT_META_OIF: packet output interface index (dev->ifindex)
 * @NFT_META_IIFNAME: packet input interface name (dev->name)
 * @NFT_META_OIFNAME: packet output interface name (dev->name)
 * @NFT_META_IIFTYPE: packet input interface type (dev->type)
 * @NFT_META_OIFTYPE: packet output interface type (dev->type)
 * @NFT_META_SKUID: originating socket UID (fsuid)
 * @NFT_META_SKGID: originating socket GID (fsgid)
 * @NFT_META_NFTRACE: packet nftrace bit
 * @NFT_META_RTCLASSID: realm value of packet's route (skb->dst->tclassid)
 * @NFT_META_SECMARK: packet secmark (skb->secmark)
 * @NFT_META_NFPROTO: netfilter protocol
 * @NFT_META_L4PROTO: layer 4 protocol number
 * @NFT_META_BRI_IIFNAME: packet input bridge interface name
 * @NFT_META_BRI_OIFNAME: packet output bridge interface name
 * @NFT_META_PKTTYPE: packet type (skb->pkt_type), special handling for loopback
 * @NFT_META_CPU: cpu id through smp_processor_id()
 * @NFT_META_IIFGROUP: packet input interface group
 * @NFT_META_OIFGROUP: packet output interface group
 * @NFT_META_CGROUP: socket control group (skb->sk->sk_classid)
 * @NFT_META_PRANDOM: a 32bit pseudo-random number
 * @NFT_META_SECPATH: boolean, secpath_exists (!!skb->sp)
 * @NFT_META_IIFKIND: packet input interface kind name (dev->rtnl_link_ops->kind)
 * @NFT_META_OIFKIND: packet output interface kind name (dev->rtnl_link_ops->kind)
 * @NFT_META_BRI_IIFPVID: packet input bridge port pvid
 * @NFT_META_BRI_IIFVPROTO: packet input bridge vlan proto
 * @NFT_META_TIME_NS: time since epoch (in nanoseconds)
 * @NFT_META_TIME_DAY: day of week (from 0 = Sunday to 6 = Saturday)
 * @NFT_META_TIME_HOUR: hour of day (in seconds)
 * @NFT_META_SDIF: slave device interface index
 * @NFT_META_SDIFNAME: slave device interface name
 * @NFT_META_BRI_BROUTE: packet br_netfilter_broute bit
 * @NFT_META_BRI_IIFHWADDR: packet input bridge interface ethernet address
 */
pub mod nft_meta_keys {
    pub const NFT_META_LEN: _ = 0;
    pub const NFT_META_PROTOCOL: _ = 1;
    pub const NFT_META_PRIORITY: _ = 2;
    pub const NFT_META_MARK: _ = 3;
    pub const NFT_META_IIF: _ = 4;
    pub const NFT_META_OIF: _ = 5;
    pub const NFT_META_IIFNAME: _ = 6;
    pub const NFT_META_OIFNAME: _ = 7;
    pub const NFT_META_IFTYPE: _ = 8;
#define NFT_META_IIFTYPE	NFT_META_IFTYPE
    pub const NFT_META_OIFTYPE: _ = 9;
    pub const NFT_META_SKUID: _ = 10;
    pub const NFT_META_SKGID: _ = 11;
    pub const NFT_META_NFTRACE: _ = 12;
    pub const NFT_META_RTCLASSID: _ = 13;
    pub const NFT_META_SECMARK: _ = 14;
    pub const NFT_META_NFPROTO: _ = 15;
    pub const NFT_META_L4PROTO: _ = 16;
    pub const NFT_META_BRI_IIFNAME: _ = 17;
    pub const NFT_META_BRI_OIFNAME: _ = 18;
    pub const NFT_META_PKTTYPE: _ = 19;
    pub const NFT_META_CPU: _ = 20;
    pub const NFT_META_IIFGROUP: _ = 21;
    pub const NFT_META_OIFGROUP: _ = 22;
    pub const NFT_META_CGROUP: _ = 23;
    pub const NFT_META_PRANDOM: _ = 24;
    pub const NFT_META_SECPATH: _ = 25;
    pub const NFT_META_IIFKIND: _ = 26;
    pub const NFT_META_OIFKIND: _ = 27;
    pub const NFT_META_BRI_IIFPVID: _ = 28;
    pub const NFT_META_BRI_IIFVPROTO: _ = 29;
    pub const NFT_META_TIME_NS: _ = 30;
    pub const NFT_META_TIME_DAY: _ = 31;
    pub const NFT_META_TIME_HOUR: _ = 32;
    pub const NFT_META_SDIF: _ = 33;
    pub const NFT_META_SDIFNAME: _ = 34;
    pub const NFT_META_BRI_BROUTE: _ = 35;
    pub const __NFT_META_IIFTYPE: _ = 36;
    pub const NFT_META_BRI_IIFHWADDR: _ = 37;
}

/**
 * enum nft_rt_keys - nf_tables routing expression keys
 *
 * @NFT_RT_CLASSID: realm value of packet's route (skb->dst->tclassid)
 * @NFT_RT_NEXTHOP4: routing nexthop for IPv4
 * @NFT_RT_NEXTHOP6: routing nexthop for IPv6
 * @NFT_RT_TCPMSS: fetch current path tcp mss
 * @NFT_RT_XFRM: boolean, skb->dst->xfrm != NULL
 */
pub mod nft_rt_keys {
    pub const NFT_RT_CLASSID: _ = 0;
    pub const NFT_RT_NEXTHOP4: _ = 1;
    pub const NFT_RT_NEXTHOP6: _ = 2;
    pub const NFT_RT_TCPMSS: _ = 3;
    pub const NFT_RT_XFRM: _ = 4;
    pub const __NFT_RT_MAX: _ = 5;
}
pub const NFT_RT_MAX: _ = (__NFT_RT_MAX - 1);

/**
 * enum nft_hash_types - nf_tables hash expression types
 *
 * @NFT_HASH_JENKINS: Jenkins Hash
 * @NFT_HASH_SYM: Symmetric Hash
 */
pub mod nft_hash_types {
    pub const NFT_HASH_JENKINS: _ = 0;
    pub const NFT_HASH_SYM: _ = 1;
}

/**
 * enum nft_hash_attributes - nf_tables hash expression netlink attributes
 *
 * @NFTA_HASH_SREG: source register (NLA_U32)
 * @NFTA_HASH_DREG: destination register (NLA_U32)
 * @NFTA_HASH_LEN: source data length (NLA_U32)
 * @NFTA_HASH_MODULUS: modulus value (NLA_U32)
 * @NFTA_HASH_SEED: seed value (NLA_U32)
 * @NFTA_HASH_OFFSET: add this offset value to hash result (NLA_U32)
 * @NFTA_HASH_TYPE: hash operation (NLA_U32: nft_hash_types)
 * @NFTA_HASH_SET_NAME: name of the map to lookup (NLA_STRING)
 * @NFTA_HASH_SET_ID: id of the map (NLA_U32)
 */
pub mod nft_hash_attributes {
    pub const NFTA_HASH_UNSPEC: _ = 0;
    pub const NFTA_HASH_SREG: _ = 1;
    pub const NFTA_HASH_DREG: _ = 2;
    pub const NFTA_HASH_LEN: _ = 3;
    pub const NFTA_HASH_MODULUS: _ = 4;
    pub const NFTA_HASH_SEED: _ = 5;
    pub const NFTA_HASH_OFFSET: _ = 6;
    pub const NFTA_HASH_TYPE: _ = 7;
    pub const NFTA_HASH_SET_NAME: _ = 8;
    pub const NFTA_HASH_SET_ID: _ = 9;
    pub const __NFTA_HASH_MAX: _ = 10;
}
pub const NFTA_HASH_MAX: _ = (__NFTA_HASH_MAX - 1);

/**
 * enum nft_meta_attributes - nf_tables meta expression netlink attributes
 *
 * @NFTA_META_DREG: destination register (NLA_U32)
 * @NFTA_META_KEY: meta data item to load (NLA_U32: nft_meta_keys)
 * @NFTA_META_SREG: source register (NLA_U32)
 */
pub mod nft_meta_attributes {
    pub const NFTA_META_UNSPEC: _ = 0;
    pub const NFTA_META_DREG: _ = 1;
    pub const NFTA_META_KEY: _ = 2;
    pub const NFTA_META_SREG: _ = 3;
    pub const __NFTA_META_MAX: _ = 4;
}
pub const NFTA_META_MAX: _ = (__NFTA_META_MAX - 1);

/**
 * enum nft_rt_attributes - nf_tables routing expression netlink attributes
 *
 * @NFTA_RT_DREG: destination register (NLA_U32)
 * @NFTA_RT_KEY: routing data item to load (NLA_U32: nft_rt_keys)
 */
pub mod nft_rt_attributes {
    pub const NFTA_RT_UNSPEC: _ = 0;
    pub const NFTA_RT_DREG: _ = 1;
    pub const NFTA_RT_KEY: _ = 2;
    pub const __NFTA_RT_MAX: _ = 3;
}
pub const NFTA_RT_MAX: _ = (__NFTA_RT_MAX - 1);

/**
 * enum nft_socket_attributes - nf_tables socket expression netlink attributes
 *
 * @NFTA_SOCKET_KEY: socket key to match
 * @NFTA_SOCKET_DREG: destination register
 * @NFTA_SOCKET_LEVEL: cgroups2 ancestor level (only for cgroupsv2)
 */
pub mod nft_socket_attributes {
    pub const NFTA_SOCKET_UNSPEC: _ = 0;
    pub const NFTA_SOCKET_KEY: _ = 1;
    pub const NFTA_SOCKET_DREG: _ = 2;
    pub const NFTA_SOCKET_LEVEL: _ = 3;
    pub const __NFTA_SOCKET_MAX: _ = 4;
}
pub const NFTA_SOCKET_MAX: _ = (__NFTA_SOCKET_MAX - 1);

/*
 * enum nft_socket_keys - nf_tables socket expression keys
 *
 * @NFT_SOCKET_TRANSPARENT: Value of the IP(V6)_TRANSPARENT socket option
 * @NFT_SOCKET_MARK: Value of the socket mark
 * @NFT_SOCKET_WILDCARD: Whether the socket is zero-bound (e.g. 0.0.0.0 or ::0)
 * @NFT_SOCKET_CGROUPV2: Match on cgroups version 2
 */
pub mod nft_socket_keys {
    pub const NFT_SOCKET_TRANSPARENT: _ = 0;
    pub const NFT_SOCKET_MARK: _ = 1;
    pub const NFT_SOCKET_WILDCARD: _ = 2;
    pub const NFT_SOCKET_CGROUPV2: _ = 3;
    pub const __NFT_SOCKET_MAX: _ = 4;
}
pub const NFT_SOCKET_MAX: _ = (__NFT_SOCKET_MAX - 1);

/**
 * enum nft_ct_keys - nf_tables ct expression keys
 *
 * @NFT_CT_STATE: conntrack state (bitmask of enum ip_conntrack_info)
 * @NFT_CT_DIRECTION: conntrack direction (enum ip_conntrack_dir)
 * @NFT_CT_STATUS: conntrack status (bitmask of enum ip_conntrack_status)
 * @NFT_CT_MARK: conntrack mark value
 * @NFT_CT_SECMARK: conntrack secmark value
 * @NFT_CT_EXPIRATION: relative conntrack expiration time in ms
 * @NFT_CT_HELPER: connection tracking helper assigned to conntrack
 * @NFT_CT_L3PROTOCOL: conntrack layer 3 protocol
 * @NFT_CT_SRC: conntrack layer 3 protocol source (IPv4/IPv6 address, deprecated)
 * @NFT_CT_DST: conntrack layer 3 protocol destination (IPv4/IPv6 address, deprecated)
 * @NFT_CT_PROTOCOL: conntrack layer 4 protocol
 * @NFT_CT_PROTO_SRC: conntrack layer 4 protocol source
 * @NFT_CT_PROTO_DST: conntrack layer 4 protocol destination
 * @NFT_CT_LABELS: conntrack labels
 * @NFT_CT_PKTS: conntrack packets
 * @NFT_CT_BYTES: conntrack bytes
 * @NFT_CT_AVGPKT: conntrack average bytes per packet
 * @NFT_CT_ZONE: conntrack zone
 * @NFT_CT_EVENTMASK: ctnetlink events to be generated for this conntrack
 * @NFT_CT_SRC_IP: conntrack layer 3 protocol source (IPv4 address)
 * @NFT_CT_DST_IP: conntrack layer 3 protocol destination (IPv4 address)
 * @NFT_CT_SRC_IP6: conntrack layer 3 protocol source (IPv6 address)
 * @NFT_CT_DST_IP6: conntrack layer 3 protocol destination (IPv6 address)
 * @NFT_CT_ID: conntrack id
 */
pub mod nft_ct_keys {
    pub const NFT_CT_STATE: _ = 0;
    pub const NFT_CT_DIRECTION: _ = 1;
    pub const NFT_CT_STATUS: _ = 2;
    pub const NFT_CT_MARK: _ = 3;
    pub const NFT_CT_SECMARK: _ = 4;
    pub const NFT_CT_EXPIRATION: _ = 5;
    pub const NFT_CT_HELPER: _ = 6;
    pub const NFT_CT_L3PROTOCOL: _ = 7;
    pub const NFT_CT_SRC: _ = 8;
    pub const NFT_CT_DST: _ = 9;
    pub const NFT_CT_PROTOCOL: _ = 10;
    pub const NFT_CT_PROTO_SRC: _ = 11;
    pub const NFT_CT_PROTO_DST: _ = 12;
    pub const NFT_CT_LABELS: _ = 13;
    pub const NFT_CT_PKTS: _ = 14;
    pub const NFT_CT_BYTES: _ = 15;
    pub const NFT_CT_AVGPKT: _ = 16;
    pub const NFT_CT_ZONE: _ = 17;
    pub const NFT_CT_EVENTMASK: _ = 18;
    pub const NFT_CT_SRC_IP: _ = 19;
    pub const NFT_CT_DST_IP: _ = 20;
    pub const NFT_CT_SRC_IP6: _ = 21;
    pub const NFT_CT_DST_IP6: _ = 22;
    pub const NFT_CT_ID: _ = 23;
    pub const __NFT_CT_MAX: _ = 24;
}
pub const NFT_CT_MAX: _ = (__NFT_CT_MAX - 1);

/**
 * enum nft_ct_attributes - nf_tables ct expression netlink attributes
 *
 * @NFTA_CT_DREG: destination register (NLA_U32)
 * @NFTA_CT_KEY: conntrack data item to load (NLA_U32: nft_ct_keys)
 * @NFTA_CT_DIRECTION: direction in case of directional keys (NLA_U8)
 * @NFTA_CT_SREG: source register (NLA_U32)
 */
pub mod nft_ct_attributes {
    pub const NFTA_CT_UNSPEC: _ = 0;
    pub const NFTA_CT_DREG: _ = 1;
    pub const NFTA_CT_KEY: _ = 2;
    pub const NFTA_CT_DIRECTION: _ = 3;
    pub const NFTA_CT_SREG: _ = 4;
    pub const __NFTA_CT_MAX: _ = 5;
}
pub const NFTA_CT_MAX: _ = (__NFTA_CT_MAX - 1);

/**
 * enum nft_offload_attributes - ct offload expression attributes
 * @NFTA_FLOW_TABLE_NAME: flow table name (NLA_STRING)
 */
pub mod nft_offload_attributes {
    pub const NFTA_FLOW_UNSPEC: _ = 0;
    pub const NFTA_FLOW_TABLE_NAME: _ = 1;
    pub const __NFTA_FLOW_MAX: _ = 2;
}
pub const NFTA_FLOW_MAX: _ = (__NFTA_FLOW_MAX - 1);

pub mod nft_limit_type {
    pub const NFT_LIMIT_PKTS: _ = 0;
    pub const NFT_LIMIT_PKT_BYTES: _ = 1;
}

pub mod nft_limit_flags {
    pub const NFT_LIMIT_F_INV: _ = (1 << 0);
}

/**
 * enum nft_limit_attributes - nf_tables limit expression netlink attributes
 *
 * @NFTA_LIMIT_RATE: refill rate (NLA_U64)
 * @NFTA_LIMIT_UNIT: refill unit (NLA_U64)
 * @NFTA_LIMIT_BURST: burst (NLA_U32)
 * @NFTA_LIMIT_TYPE: type of limit (NLA_U32: enum nft_limit_type)
 * @NFTA_LIMIT_FLAGS: flags (NLA_U32: enum nft_limit_flags)
 */
pub mod nft_limit_attributes {
    pub const NFTA_LIMIT_UNSPEC: _ = 0;
    pub const NFTA_LIMIT_RATE: _ = 1;
    pub const NFTA_LIMIT_UNIT: _ = 2;
    pub const NFTA_LIMIT_BURST: _ = 3;
    pub const NFTA_LIMIT_TYPE: _ = 4;
    pub const NFTA_LIMIT_FLAGS: _ = 5;
    pub const NFTA_LIMIT_PAD: _ = 6;
    pub const __NFTA_LIMIT_MAX: _ = 7;
}
pub const NFTA_LIMIT_MAX: _ = (__NFTA_LIMIT_MAX - 1);

pub mod nft_connlimit_flags {
    pub const NFT_CONNLIMIT_F_INV: _ = (1 << 0);
}

/**
 * enum nft_connlimit_attributes - nf_tables connlimit expression netlink attributes
 *
 * @NFTA_CONNLIMIT_COUNT: number of connections (NLA_U32)
 * @NFTA_CONNLIMIT_FLAGS: flags (NLA_U32: enum nft_connlimit_flags)
 */
pub mod nft_connlimit_attributes {
    pub const NFTA_CONNLIMIT_UNSPEC: _ = 0;
    pub const NFTA_CONNLIMIT_COUNT: _ = 1;
    pub const NFTA_CONNLIMIT_FLAGS: _ = 2;
    pub const __NFTA_CONNLIMIT_MAX: _ = 3;
}
pub const NFTA_CONNLIMIT_MAX: _ = (__NFTA_CONNLIMIT_MAX - 1);

/**
 * enum nft_counter_attributes - nf_tables counter expression netlink attributes
 *
 * @NFTA_COUNTER_BYTES: number of bytes (NLA_U64)
 * @NFTA_COUNTER_PACKETS: number of packets (NLA_U64)
 */
pub mod nft_counter_attributes {
    pub const NFTA_COUNTER_UNSPEC: _ = 0;
    pub const NFTA_COUNTER_BYTES: _ = 1;
    pub const NFTA_COUNTER_PACKETS: _ = 2;
    pub const NFTA_COUNTER_PAD: _ = 3;
    pub const __NFTA_COUNTER_MAX: _ = 4;
}
pub const NFTA_COUNTER_MAX: _ = (__NFTA_COUNTER_MAX - 1);

/**
 * enum nft_last_attributes - nf_tables last expression netlink attributes
 *
 * @NFTA_LAST_SET: last update has been set, zero means never updated (NLA_U32)
 * @NFTA_LAST_MSECS: milliseconds since last update (NLA_U64)
 */
pub mod nft_last_attributes {
    pub const NFTA_LAST_UNSPEC: _ = 0;
    pub const NFTA_LAST_SET: _ = 1;
    pub const NFTA_LAST_MSECS: _ = 2;
    pub const NFTA_LAST_PAD: _ = 3;
    pub const __NFTA_LAST_MAX: _ = 4;
}
pub const NFTA_LAST_MAX: _ = (__NFTA_LAST_MAX - 1);

/**
 * enum nft_log_attributes - nf_tables log expression netlink attributes
 *
 * @NFTA_LOG_GROUP: netlink group to send messages to (NLA_U16)
 * @NFTA_LOG_PREFIX: prefix to prepend to log messages (NLA_STRING)
 * @NFTA_LOG_SNAPLEN: length of payload to include in netlink message (NLA_U32)
 * @NFTA_LOG_QTHRESHOLD: queue threshold (NLA_U16)
 * @NFTA_LOG_LEVEL: log level (NLA_U32)
 * @NFTA_LOG_FLAGS: logging flags (NLA_U32)
 */
pub mod nft_log_attributes {
    pub const NFTA_LOG_UNSPEC: _ = 0;
    pub const NFTA_LOG_GROUP: _ = 1;
    pub const NFTA_LOG_PREFIX: _ = 2;
    pub const NFTA_LOG_SNAPLEN: _ = 3;
    pub const NFTA_LOG_QTHRESHOLD: _ = 4;
    pub const NFTA_LOG_LEVEL: _ = 5;
    pub const NFTA_LOG_FLAGS: _ = 6;
    pub const __NFTA_LOG_MAX: _ = 7;
}
pub const NFTA_LOG_MAX: _ = (__NFTA_LOG_MAX - 1);

/**
 * enum nft_log_level - nf_tables log levels
 *
 * @NFT_LOGLEVEL_EMERG: system is unusable
 * @NFT_LOGLEVEL_ALERT: action must be taken immediately
 * @NFT_LOGLEVEL_CRIT: critical conditions
 * @NFT_LOGLEVEL_ERR: error conditions
 * @NFT_LOGLEVEL_WARNING: warning conditions
 * @NFT_LOGLEVEL_NOTICE: normal but significant condition
 * @NFT_LOGLEVEL_INFO: informational
 * @NFT_LOGLEVEL_DEBUG: debug-level messages
 * @NFT_LOGLEVEL_AUDIT: enabling audit logging
 */
pub mod nft_log_level {
    pub const NFT_LOGLEVEL_EMERG: _ = 0;
    pub const NFT_LOGLEVEL_ALERT: _ = 1;
    pub const NFT_LOGLEVEL_CRIT: _ = 2;
    pub const NFT_LOGLEVEL_ERR: _ = 3;
    pub const NFT_LOGLEVEL_WARNING: _ = 4;
    pub const NFT_LOGLEVEL_NOTICE: _ = 5;
    pub const NFT_LOGLEVEL_INFO: _ = 6;
    pub const NFT_LOGLEVEL_DEBUG: _ = 7;
    pub const NFT_LOGLEVEL_AUDIT: _ = 8;
    pub const __NFT_LOGLEVEL_MAX: _ = 9;
}
pub const NFT_LOGLEVEL_MAX: _ = (__NFT_LOGLEVEL_MAX - 1);

/**
 * enum nft_queue_attributes - nf_tables queue expression netlink attributes
 *
 * @NFTA_QUEUE_NUM: netlink queue to send messages to (NLA_U16)
 * @NFTA_QUEUE_TOTAL: number of queues to load balance packets on (NLA_U16)
 * @NFTA_QUEUE_FLAGS: various flags (NLA_U16)
 * @NFTA_QUEUE_SREG_QNUM: source register of queue number (NLA_U32: nft_registers)
 */
pub mod nft_queue_attributes {
    pub const NFTA_QUEUE_UNSPEC: _ = 0;
    pub const NFTA_QUEUE_NUM: _ = 1;
    pub const NFTA_QUEUE_TOTAL: _ = 2;
    pub const NFTA_QUEUE_FLAGS: _ = 3;
    pub const NFTA_QUEUE_SREG_QNUM: _ = 4;
    pub const __NFTA_QUEUE_MAX: _ = 5;
}
pub const NFTA_QUEUE_MAX: _ = (__NFTA_QUEUE_MAX - 1);

pub const NFT_QUEUE_FLAG_BYPASS: _ = 0x01;
pub const NFT_QUEUE_FLAG_CPU_FANOUT: _ = 0x02;
pub const NFT_QUEUE_FLAG_MASK: _ = 0x03;

pub mod nft_quota_flags {
    pub const NFT_QUOTA_F_INV: _ = (1 << 0);
    pub const NFT_QUOTA_F_DEPLETED: _ = (1 << 1);
}

/**
 * enum nft_quota_attributes - nf_tables quota expression netlink attributes
 *
 * @NFTA_QUOTA_BYTES: quota in bytes (NLA_U16)
 * @NFTA_QUOTA_FLAGS: flags (NLA_U32)
 * @NFTA_QUOTA_CONSUMED: quota already consumed in bytes (NLA_U64)
 */
pub mod nft_quota_attributes {
    pub const NFTA_QUOTA_UNSPEC: _ = 0;
    pub const NFTA_QUOTA_BYTES: _ = 1;
    pub const NFTA_QUOTA_FLAGS: _ = 2;
    pub const NFTA_QUOTA_PAD: _ = 3;
    pub const NFTA_QUOTA_CONSUMED: _ = 4;
    pub const __NFTA_QUOTA_MAX: _ = 5;
}
pub const NFTA_QUOTA_MAX: _ = (__NFTA_QUOTA_MAX - 1);

/**
 * enum nft_secmark_attributes - nf_tables secmark object netlink attributes
 *
 * @NFTA_SECMARK_CTX: security context (NLA_STRING)
 */
pub mod nft_secmark_attributes {
    pub const NFTA_SECMARK_UNSPEC: _ = 0;
    pub const NFTA_SECMARK_CTX: _ = 1;
    pub const __NFTA_SECMARK_MAX: _ = 2;
}
pub const NFTA_SECMARK_MAX: _ = (__NFTA_SECMARK_MAX - 1);

/* Max security context length */
pub const NFT_SECMARK_CTX_MAXLEN: _ = 4096;

/**
 * enum nft_reject_types - nf_tables reject expression reject types
 *
 * @NFT_REJECT_ICMP_UNREACH: reject using ICMP unreachable
 * @NFT_REJECT_TCP_RST: reject using TCP RST
 * @NFT_REJECT_ICMPX_UNREACH: abstracted ICMP unreachable for bridge and inet
 */
pub mod nft_reject_types {
    pub const NFT_REJECT_ICMP_UNREACH: _ = 0;
    pub const NFT_REJECT_TCP_RST: _ = 1;
    pub const NFT_REJECT_ICMPX_UNREACH: _ = 2;
}

/**
 * enum nft_reject_inet_code - Generic reject codes for IPv4/IPv6
 *
 * @NFT_REJECT_ICMPX_NO_ROUTE: no route to host / network unreachable
 * @NFT_REJECT_ICMPX_PORT_UNREACH: port unreachable
 * @NFT_REJECT_ICMPX_HOST_UNREACH: host unreachable
 * @NFT_REJECT_ICMPX_ADMIN_PROHIBITED: administratively prohibited
 *
 * These codes are mapped to real ICMP and ICMPv6 codes.
 */
pub mod nft_reject_inet_code {
    pub const NFT_REJECT_ICMPX_NO_ROUTE: _ = 0;
    pub const NFT_REJECT_ICMPX_PORT_UNREACH: _ = 1;
    pub const NFT_REJECT_ICMPX_HOST_UNREACH: _ = 2;
    pub const NFT_REJECT_ICMPX_ADMIN_PROHIBITED: _ = 3;
    pub const __NFT_REJECT_ICMPX_MAX: _ = 4;
}
pub const NFT_REJECT_ICMPX_MAX: _ = (__NFT_REJECT_ICMPX_MAX - 1);

/**
 * enum nft_reject_attributes - nf_tables reject expression netlink attributes
 *
 * @NFTA_REJECT_TYPE: packet type to use (NLA_U32: nft_reject_types)
 * @NFTA_REJECT_ICMP_CODE: ICMP code to use (NLA_U8)
 */
pub mod nft_reject_attributes {
    pub const NFTA_REJECT_UNSPEC: _ = 0;
    pub const NFTA_REJECT_TYPE: _ = 1;
    pub const NFTA_REJECT_ICMP_CODE: _ = 2;
    pub const __NFTA_REJECT_MAX: _ = 3;
}
pub const NFTA_REJECT_MAX: _ = (__NFTA_REJECT_MAX - 1);

/**
 * enum nft_nat_types - nf_tables nat expression NAT types
 *
 * @NFT_NAT_SNAT: source NAT
 * @NFT_NAT_DNAT: destination NAT
 */
pub mod nft_nat_types {
    pub const NFT_NAT_SNAT: _ = 0;
    pub const NFT_NAT_DNAT: _ = 1;
}

/**
 * enum nft_nat_attributes - nf_tables nat expression netlink attributes
 *
 * @NFTA_NAT_TYPE: NAT type (NLA_U32: nft_nat_types)
 * @NFTA_NAT_FAMILY: NAT family (NLA_U32)
 * @NFTA_NAT_REG_ADDR_MIN: source register of address range start (NLA_U32: nft_registers)
 * @NFTA_NAT_REG_ADDR_MAX: source register of address range end (NLA_U32: nft_registers)
 * @NFTA_NAT_REG_PROTO_MIN: source register of proto range start (NLA_U32: nft_registers)
 * @NFTA_NAT_REG_PROTO_MAX: source register of proto range end (NLA_U32: nft_registers)
 * @NFTA_NAT_FLAGS: NAT flags (see NF_NAT_RANGE_* in linux/netfilter/nf_nat.h) (NLA_U32)
 */
pub mod nft_nat_attributes {
    pub const NFTA_NAT_UNSPEC: _ = 0;
    pub const NFTA_NAT_TYPE: _ = 1;
    pub const NFTA_NAT_FAMILY: _ = 2;
    pub const NFTA_NAT_REG_ADDR_MIN: _ = 3;
    pub const NFTA_NAT_REG_ADDR_MAX: _ = 4;
    pub const NFTA_NAT_REG_PROTO_MIN: _ = 5;
    pub const NFTA_NAT_REG_PROTO_MAX: _ = 6;
    pub const NFTA_NAT_FLAGS: _ = 7;
    pub const __NFTA_NAT_MAX: _ = 8;
}
pub const NFTA_NAT_MAX: _ = (__NFTA_NAT_MAX - 1);

/**
 * enum nft_tproxy_attributes - nf_tables tproxy expression netlink attributes
 *
 * @NFTA_TPROXY_FAMILY: Target address family (NLA_U32: nft_registers)
 * @NFTA_TPROXY_REG_ADDR: Target address register (NLA_U32: nft_registers)
 * @NFTA_TPROXY_REG_PORT: Target port register (NLA_U32: nft_registers)
 */
pub mod nft_tproxy_attributes {
    pub const NFTA_TPROXY_UNSPEC: _ = 0;
    pub const NFTA_TPROXY_FAMILY: _ = 1;
    pub const NFTA_TPROXY_REG_ADDR: _ = 2;
    pub const NFTA_TPROXY_REG_PORT: _ = 3;
    pub const __NFTA_TPROXY_MAX: _ = 4;
}
pub const NFTA_TPROXY_MAX: _ = (__NFTA_TPROXY_MAX - 1);

/**
 * enum nft_masq_attributes - nf_tables masquerade expression attributes
 *
 * @NFTA_MASQ_FLAGS: NAT flags (see NF_NAT_RANGE_* in linux/netfilter/nf_nat.h) (NLA_U32)
 * @NFTA_MASQ_REG_PROTO_MIN: source register of proto range start (NLA_U32: nft_registers)
 * @NFTA_MASQ_REG_PROTO_MAX: source register of proto range end (NLA_U32: nft_registers)
 */
pub mod nft_masq_attributes {
    pub const NFTA_MASQ_UNSPEC: _ = 0;
    pub const NFTA_MASQ_FLAGS: _ = 1;
    pub const NFTA_MASQ_REG_PROTO_MIN: _ = 2;
    pub const NFTA_MASQ_REG_PROTO_MAX: _ = 3;
    pub const __NFTA_MASQ_MAX: _ = 4;
}
pub const NFTA_MASQ_MAX: _ = (__NFTA_MASQ_MAX - 1);

/**
 * enum nft_redir_attributes - nf_tables redirect expression netlink attributes
 *
 * @NFTA_REDIR_REG_PROTO_MIN: source register of proto range start (NLA_U32: nft_registers)
 * @NFTA_REDIR_REG_PROTO_MAX: source register of proto range end (NLA_U32: nft_registers)
 * @NFTA_REDIR_FLAGS: NAT flags (see NF_NAT_RANGE_* in linux/netfilter/nf_nat.h) (NLA_U32)
 */
pub mod nft_redir_attributes {
    pub const NFTA_REDIR_UNSPEC: _ = 0;
    pub const NFTA_REDIR_REG_PROTO_MIN: _ = 1;
    pub const NFTA_REDIR_REG_PROTO_MAX: _ = 2;
    pub const NFTA_REDIR_FLAGS: _ = 3;
    pub const __NFTA_REDIR_MAX: _ = 4;
}
pub const NFTA_REDIR_MAX: _ = (__NFTA_REDIR_MAX - 1);

/**
 * enum nft_dup_attributes - nf_tables dup expression netlink attributes
 *
 * @NFTA_DUP_SREG_ADDR: source register of address (NLA_U32: nft_registers)
 * @NFTA_DUP_SREG_DEV: source register of output interface (NLA_U32: nft_register)
 */
pub mod nft_dup_attributes {
    pub const NFTA_DUP_UNSPEC: _ = 0;
    pub const NFTA_DUP_SREG_ADDR: _ = 1;
    pub const NFTA_DUP_SREG_DEV: _ = 2;
    pub const __NFTA_DUP_MAX: _ = 3;
}
pub const NFTA_DUP_MAX: _ = (__NFTA_DUP_MAX - 1);

/**
 * enum nft_fwd_attributes - nf_tables fwd expression netlink attributes
 *
 * @NFTA_FWD_SREG_DEV: source register of output interface (NLA_U32: nft_register)
 * @NFTA_FWD_SREG_ADDR: source register of destination address (NLA_U32: nft_register)
 * @NFTA_FWD_NFPROTO: layer 3 family of source register address (NLA_U32: enum nfproto)
 */
pub mod nft_fwd_attributes {
    pub const NFTA_FWD_UNSPEC: _ = 0;
    pub const NFTA_FWD_SREG_DEV: _ = 1;
    pub const NFTA_FWD_SREG_ADDR: _ = 2;
    pub const NFTA_FWD_NFPROTO: _ = 3;
    pub const __NFTA_FWD_MAX: _ = 4;
}
pub const NFTA_FWD_MAX: _ = (__NFTA_FWD_MAX - 1);

/**
 * enum nft_objref_attributes - nf_tables stateful object expression netlink attributes
 *
 * @NFTA_OBJREF_IMM_TYPE: object type for immediate reference (NLA_U32: nft_register)
 * @NFTA_OBJREF_IMM_NAME: object name for immediate reference (NLA_STRING)
 * @NFTA_OBJREF_SET_SREG: source register of the data to look for (NLA_U32: nft_registers)
 * @NFTA_OBJREF_SET_NAME: name of the set where to look for (NLA_STRING)
 * @NFTA_OBJREF_SET_ID: id of the set where to look for in this transaction (NLA_U32)
 */
pub mod nft_objref_attributes {
    pub const NFTA_OBJREF_UNSPEC: _ = 0;
    pub const NFTA_OBJREF_IMM_TYPE: _ = 1;
    pub const NFTA_OBJREF_IMM_NAME: _ = 2;
    pub const NFTA_OBJREF_SET_SREG: _ = 3;
    pub const NFTA_OBJREF_SET_NAME: _ = 4;
    pub const NFTA_OBJREF_SET_ID: _ = 5;
    pub const __NFTA_OBJREF_MAX: _ = 6;
}
pub const NFTA_OBJREF_MAX: _ = (__NFTA_OBJREF_MAX - 1);

/**
 * enum nft_gen_attributes - nf_tables ruleset generation attributes
 *
 * @NFTA_GEN_ID: Ruleset generation ID (NLA_U32)
 */
pub mod nft_gen_attributes {
    pub const NFTA_GEN_UNSPEC: _ = 0;
    pub const NFTA_GEN_ID: _ = 1;
    pub const NFTA_GEN_PROC_PID: _ = 2;
    pub const NFTA_GEN_PROC_NAME: _ = 3;
    pub const __NFTA_GEN_MAX: _ = 4;
}
pub const NFTA_GEN_MAX: _ = (__NFTA_GEN_MAX - 1);

/*
 * enum nft_fib_attributes - nf_tables fib expression netlink attributes
 *
 * @NFTA_FIB_DREG: destination register (NLA_U32)
 * @NFTA_FIB_RESULT: desired result (NLA_U32)
 * @NFTA_FIB_FLAGS: flowi fields to initialize when querying the FIB (NLA_U32)
 *
 * The FIB expression performs a route lookup according
 * to the packet data.
 */
pub mod nft_fib_attributes {
    pub const NFTA_FIB_UNSPEC: _ = 0;
    pub const NFTA_FIB_DREG: _ = 1;
    pub const NFTA_FIB_RESULT: _ = 2;
    pub const NFTA_FIB_FLAGS: _ = 3;
    pub const __NFTA_FIB_MAX: _ = 4;
}
pub const NFTA_FIB_MAX: _ = (__NFTA_FIB_MAX - 1);

pub mod nft_fib_result {
    pub const NFT_FIB_RESULT_UNSPEC: _ = 0;
    pub const NFT_FIB_RESULT_OIF: _ = 1;
    pub const NFT_FIB_RESULT_OIFNAME: _ = 2;
    pub const NFT_FIB_RESULT_ADDRTYPE: _ = 3;
    pub const __NFT_FIB_RESULT_MAX: _ = 4;
}
pub const NFT_FIB_RESULT_MAX: _ = (__NFT_FIB_RESULT_MAX - 1);

pub mod nft_fib_flags {
    pub const NFTA_FIB_F_SADDR: _ = 1 << 0;
    pub const NFTA_FIB_F_DADDR: _ = 1 << 1;
    pub const NFTA_FIB_F_MARK: _ = 1 << 2;
    pub const NFTA_FIB_F_IIF: _ = 1 << 3;
    pub const NFTA_FIB_F_OIF: _ = 1 << 4;
    pub const NFTA_FIB_F_PRESENT: _ = 1 << 5;
}

pub mod nft_ct_helper_attributes {
    pub const NFTA_CT_HELPER_UNSPEC: _ = 0;
    pub const NFTA_CT_HELPER_NAME: _ = 1;
    pub const NFTA_CT_HELPER_L3PROTO: _ = 2;
    pub const NFTA_CT_HELPER_L4PROTO: _ = 3;
    pub const __NFTA_CT_HELPER_MAX: _ = 4;
}
pub const NFTA_CT_HELPER_MAX: _ = (__NFTA_CT_HELPER_MAX - 1);

pub mod nft_ct_timeout_timeout_attributes {
    pub const NFTA_CT_TIMEOUT_UNSPEC: _ = 0;
    pub const NFTA_CT_TIMEOUT_L3PROTO: _ = 1;
    pub const NFTA_CT_TIMEOUT_L4PROTO: _ = 2;
    pub const NFTA_CT_TIMEOUT_DATA: _ = 3;
    pub const __NFTA_CT_TIMEOUT_MAX: _ = 4;
}
pub const NFTA_CT_TIMEOUT_MAX: _ = (__NFTA_CT_TIMEOUT_MAX - 1);

pub mod nft_ct_expectation_attributes {
    pub const NFTA_CT_EXPECT_UNSPEC: _ = 0;
    pub const NFTA_CT_EXPECT_L3PROTO: _ = 1;
    pub const NFTA_CT_EXPECT_L4PROTO: _ = 2;
    pub const NFTA_CT_EXPECT_DPORT: _ = 3;
    pub const NFTA_CT_EXPECT_TIMEOUT: _ = 4;
    pub const NFTA_CT_EXPECT_SIZE: _ = 5;
    pub const __NFTA_CT_EXPECT_MAX: _ = 6;
}
pub const NFTA_CT_EXPECT_MAX: _ = (__NFTA_CT_EXPECT_MAX - 1);

pub const NFT_OBJECT_UNSPEC: _ = 0;
pub const NFT_OBJECT_COUNTER: _ = 1;
pub const NFT_OBJECT_QUOTA: _ = 2;
pub const NFT_OBJECT_CT_HELPER: _ = 3;
pub const NFT_OBJECT_LIMIT: _ = 4;
pub const NFT_OBJECT_CONNLIMIT: _ = 5;
pub const NFT_OBJECT_TUNNEL: _ = 6;
pub const NFT_OBJECT_CT_TIMEOUT: _ = 7;
pub const NFT_OBJECT_SECMARK: _ = 8;
pub const NFT_OBJECT_CT_EXPECT: _ = 9;
pub const NFT_OBJECT_SYNPROXY: _ = 10;
pub const __NFT_OBJECT_MAX: _ = 11;
pub const NFT_OBJECT_MAX: _ = (__NFT_OBJECT_MAX - 1);

/**
 * enum nft_object_attributes - nf_tables stateful object netlink attributes
 *
 * @NFTA_OBJ_TABLE: name of the table containing the expression (NLA_STRING)
 * @NFTA_OBJ_NAME: name of this expression type (NLA_STRING)
 * @NFTA_OBJ_TYPE: stateful object type (NLA_U32)
 * @NFTA_OBJ_DATA: stateful object data (NLA_NESTED)
 * @NFTA_OBJ_USE: number of references to this expression (NLA_U32)
 * @NFTA_OBJ_HANDLE: object handle (NLA_U64)
 * @NFTA_OBJ_USERDATA: user data (NLA_BINARY)
 */
pub mod nft_object_attributes {
    pub const NFTA_OBJ_UNSPEC: _ = 0;
    pub const NFTA_OBJ_TABLE: _ = 1;
    pub const NFTA_OBJ_NAME: _ = 2;
    pub const NFTA_OBJ_TYPE: _ = 3;
    pub const NFTA_OBJ_DATA: _ = 4;
    pub const NFTA_OBJ_USE: _ = 5;
    pub const NFTA_OBJ_HANDLE: _ = 6;
    pub const NFTA_OBJ_PAD: _ = 7;
    pub const NFTA_OBJ_USERDATA: _ = 8;
    pub const __NFTA_OBJ_MAX: _ = 9;
}
pub const NFTA_OBJ_MAX: _ = (__NFTA_OBJ_MAX - 1);

/**
 * enum nft_flowtable_flags - nf_tables flowtable flags
 *
 * @NFT_FLOWTABLE_HW_OFFLOAD: flowtable hardware offload is enabled
 * @NFT_FLOWTABLE_COUNTER: enable flow counters
 */
pub mod nft_flowtable_flags {
    pub const NFT_FLOWTABLE_HW_OFFLOAD: _ = 0x1;
    pub const NFT_FLOWTABLE_COUNTER: _ = 0x2;
    pub const NFT_FLOWTABLE_MASK: _ = (NFT_FLOWTABLE_HW_OFFLOAD |;
					   NFT_FLOWTABLE_COUNTER)
}

/**
 * enum nft_flowtable_attributes - nf_tables flow table netlink attributes
 *
 * @NFTA_FLOWTABLE_TABLE: name of the table containing the expression (NLA_STRING)
 * @NFTA_FLOWTABLE_NAME: name of this flow table (NLA_STRING)
 * @NFTA_FLOWTABLE_HOOK: netfilter hook configuration (NLA_NESTED)
 * @NFTA_FLOWTABLE_USE: number of references to this flow table (NLA_U32)
 * @NFTA_FLOWTABLE_HANDLE: object handle (NLA_U64)
 * @NFTA_FLOWTABLE_FLAGS: flags (NLA_U32)
 */
pub mod nft_flowtable_attributes {
    pub const NFTA_FLOWTABLE_UNSPEC: _ = 0;
    pub const NFTA_FLOWTABLE_TABLE: _ = 1;
    pub const NFTA_FLOWTABLE_NAME: _ = 2;
    pub const NFTA_FLOWTABLE_HOOK: _ = 3;
    pub const NFTA_FLOWTABLE_USE: _ = 4;
    pub const NFTA_FLOWTABLE_HANDLE: _ = 5;
    pub const NFTA_FLOWTABLE_PAD: _ = 6;
    pub const NFTA_FLOWTABLE_FLAGS: _ = 7;
    pub const __NFTA_FLOWTABLE_MAX: _ = 8;
}
pub const NFTA_FLOWTABLE_MAX: _ = (__NFTA_FLOWTABLE_MAX - 1);

/**
 * enum nft_flowtable_hook_attributes - nf_tables flow table hook netlink attributes
 *
 * @NFTA_FLOWTABLE_HOOK_NUM: netfilter hook number (NLA_U32)
 * @NFTA_FLOWTABLE_HOOK_PRIORITY: netfilter hook priority (NLA_U32)
 * @NFTA_FLOWTABLE_HOOK_DEVS: input devices this flow table is bound to (NLA_NESTED)
 */
pub mod nft_flowtable_hook_attributes {
    pub const NFTA_FLOWTABLE_HOOK_UNSPEC: _ = 0;
    pub const NFTA_FLOWTABLE_HOOK_NUM: _ = 1;
    pub const NFTA_FLOWTABLE_HOOK_PRIORITY: _ = 2;
    pub const NFTA_FLOWTABLE_HOOK_DEVS: _ = 3;
    pub const __NFTA_FLOWTABLE_HOOK_MAX: _ = 4;
}
pub const NFTA_FLOWTABLE_HOOK_MAX: _ = (__NFTA_FLOWTABLE_HOOK_MAX - 1);

/**
 * enum nft_osf_attributes - nftables osf expression netlink attributes
 *
 * @NFTA_OSF_DREG: destination register (NLA_U32: nft_registers)
 * @NFTA_OSF_TTL: Value of the TTL osf option (NLA_U8)
 * @NFTA_OSF_FLAGS: flags (NLA_U32)
 */
pub mod nft_osf_attributes {
    pub const NFTA_OSF_UNSPEC: _ = 0;
    pub const NFTA_OSF_DREG: _ = 1;
    pub const NFTA_OSF_TTL: _ = 2;
    pub const NFTA_OSF_FLAGS: _ = 3;
    pub const __NFTA_OSF_MAX: _ = 4;
}
pub const NFTA_OSF_MAX: _ = (__NFTA_OSF_MAX - 1);

pub mod nft_osf_flags {
    pub const NFT_OSF_F_VERSION: _ = (1 << 0);
}

/**
 * enum nft_synproxy_attributes - nf_tables synproxy expression netlink attributes
 *
 * @NFTA_SYNPROXY_MSS: mss value sent to the backend (NLA_U16)
 * @NFTA_SYNPROXY_WSCALE: wscale value sent to the backend (NLA_U8)
 * @NFTA_SYNPROXY_FLAGS: flags (NLA_U32)
 */
pub mod nft_synproxy_attributes {
    pub const NFTA_SYNPROXY_UNSPEC: _ = 0;
    pub const NFTA_SYNPROXY_MSS: _ = 1;
    pub const NFTA_SYNPROXY_WSCALE: _ = 2;
    pub const NFTA_SYNPROXY_FLAGS: _ = 3;
    pub const __NFTA_SYNPROXY_MAX: _ = 4;
}
pub const NFTA_SYNPROXY_MAX: _ = (__NFTA_SYNPROXY_MAX - 1);

/**
 * enum nft_devices_attributes - nf_tables device netlink attributes
 *
 * @NFTA_DEVICE_NAME: name of this device (NLA_STRING)
 * @NFTA_DEVICE_PREFIX: device name prefix, a simple wildcard (NLA_STRING)
 */
pub mod nft_devices_attributes {
    pub const NFTA_DEVICE_UNSPEC: _ = 0;
    pub const NFTA_DEVICE_NAME: _ = 1;
    pub const NFTA_DEVICE_PREFIX: _ = 2;
    pub const __NFTA_DEVICE_MAX: _ = 3;
}
pub const NFTA_DEVICE_MAX: _ = (__NFTA_DEVICE_MAX - 1);

/*
 * enum nft_xfrm_attributes - nf_tables xfrm expr netlink attributes
 *
 * @NFTA_XFRM_DREG: destination register (NLA_U32)
 * @NFTA_XFRM_KEY: enum nft_xfrm_keys (NLA_U32)
 * @NFTA_XFRM_DIR: direction (NLA_U8)
 * @NFTA_XFRM_SPNUM: index in secpath array (NLA_U32)
 */
pub mod nft_xfrm_attributes {
    pub const NFTA_XFRM_UNSPEC: _ = 0;
    pub const NFTA_XFRM_DREG: _ = 1;
    pub const NFTA_XFRM_KEY: _ = 2;
    pub const NFTA_XFRM_DIR: _ = 3;
    pub const NFTA_XFRM_SPNUM: _ = 4;
    pub const __NFTA_XFRM_MAX: _ = 5;
}
pub const NFTA_XFRM_MAX: _ = (__NFTA_XFRM_MAX - 1);

pub mod nft_xfrm_keys {
    pub const NFT_XFRM_KEY_UNSPEC: _ = 0;
    pub const NFT_XFRM_KEY_DADDR_IP4: _ = 1;
    pub const NFT_XFRM_KEY_DADDR_IP6: _ = 2;
    pub const NFT_XFRM_KEY_SADDR_IP4: _ = 3;
    pub const NFT_XFRM_KEY_SADDR_IP6: _ = 4;
    pub const NFT_XFRM_KEY_REQID: _ = 5;
    pub const NFT_XFRM_KEY_SPI: _ = 6;
    pub const __NFT_XFRM_KEY_MAX: _ = 7;
}
pub const NFT_XFRM_KEY_MAX: _ = (__NFT_XFRM_KEY_MAX - 1);

/**
 * enum nft_trace_attributes - nf_tables trace netlink attributes
 *
 * @NFTA_TRACE_TABLE: name of the table (NLA_STRING)
 * @NFTA_TRACE_CHAIN: name of the chain (NLA_STRING)
 * @NFTA_TRACE_RULE_HANDLE: numeric handle of the rule (NLA_U64)
 * @NFTA_TRACE_TYPE: type of the event (NLA_U32: nft_trace_types)
 * @NFTA_TRACE_VERDICT: verdict returned by hook (NLA_NESTED: nft_verdicts)
 * @NFTA_TRACE_ID: pseudo-id, same for each skb traced (NLA_U32)
 * @NFTA_TRACE_LL_HEADER: linklayer header (NLA_BINARY)
 * @NFTA_TRACE_NETWORK_HEADER: network header (NLA_BINARY)
 * @NFTA_TRACE_TRANSPORT_HEADER: transport header (NLA_BINARY)
 * @NFTA_TRACE_IIF: indev ifindex (NLA_U32)
 * @NFTA_TRACE_IIFTYPE: netdev->type of indev (NLA_U16)
 * @NFTA_TRACE_OIF: outdev ifindex (NLA_U32)
 * @NFTA_TRACE_OIFTYPE: netdev->type of outdev (NLA_U16)
 * @NFTA_TRACE_MARK: nfmark (NLA_U32)
 * @NFTA_TRACE_NFPROTO: nf protocol processed (NLA_U32)
 * @NFTA_TRACE_POLICY: policy that decided fate of packet (NLA_U32)
 * @NFTA_TRACE_CT_ID: conntrack id (NLA_U32)
 * @NFTA_TRACE_CT_DIRECTION: packets direction (NLA_U8)
 * @NFTA_TRACE_CT_STATUS: conntrack status (NLA_U32)
 * @NFTA_TRACE_CT_STATE: packet state (new, established, ...) (NLA_U32)
 */
pub mod nft_trace_attributes {
    pub const NFTA_TRACE_UNSPEC: _ = 0;
    pub const NFTA_TRACE_TABLE: _ = 1;
    pub const NFTA_TRACE_CHAIN: _ = 2;
    pub const NFTA_TRACE_RULE_HANDLE: _ = 3;
    pub const NFTA_TRACE_TYPE: _ = 4;
    pub const NFTA_TRACE_VERDICT: _ = 5;
    pub const NFTA_TRACE_ID: _ = 6;
    pub const NFTA_TRACE_LL_HEADER: _ = 7;
    pub const NFTA_TRACE_NETWORK_HEADER: _ = 8;
    pub const NFTA_TRACE_TRANSPORT_HEADER: _ = 9;
    pub const NFTA_TRACE_IIF: _ = 10;
    pub const NFTA_TRACE_IIFTYPE: _ = 11;
    pub const NFTA_TRACE_OIF: _ = 12;
    pub const NFTA_TRACE_OIFTYPE: _ = 13;
    pub const NFTA_TRACE_MARK: _ = 14;
    pub const NFTA_TRACE_NFPROTO: _ = 15;
    pub const NFTA_TRACE_POLICY: _ = 16;
    pub const NFTA_TRACE_PAD: _ = 17;
    pub const NFTA_TRACE_CT_ID: _ = 18;
    pub const NFTA_TRACE_CT_DIRECTION: _ = 19;
    pub const NFTA_TRACE_CT_STATUS: _ = 20;
    pub const NFTA_TRACE_CT_STATE: _ = 21;
    pub const __NFTA_TRACE_MAX: _ = 22;
}
pub const NFTA_TRACE_MAX: _ = (__NFTA_TRACE_MAX - 1);

pub mod nft_trace_types {
    pub const NFT_TRACETYPE_UNSPEC: _ = 0;
    pub const NFT_TRACETYPE_POLICY: _ = 1;
    pub const NFT_TRACETYPE_RETURN: _ = 2;
    pub const NFT_TRACETYPE_RULE: _ = 3;
    pub const __NFT_TRACETYPE_MAX: _ = 4;
}
pub const NFT_TRACETYPE_MAX: _ = (__NFT_TRACETYPE_MAX - 1);

/**
 * enum nft_ng_attributes - nf_tables number generator expression netlink attributes
 *
 * @NFTA_NG_DREG: destination register (NLA_U32)
 * @NFTA_NG_MODULUS: maximum counter value (NLA_U32)
 * @NFTA_NG_TYPE: operation type (NLA_U32)
 * @NFTA_NG_OFFSET: offset to be added to the counter (NLA_U32)
 * @NFTA_NG_SET_NAME: name of the map to lookup (NLA_STRING)
 * @NFTA_NG_SET_ID: id of the map (NLA_U32)
 */
pub mod nft_ng_attributes {
    pub const NFTA_NG_UNSPEC: _ = 0;
    pub const NFTA_NG_DREG: _ = 1;
    pub const NFTA_NG_MODULUS: _ = 2;
    pub const NFTA_NG_TYPE: _ = 3;
    pub const NFTA_NG_OFFSET: _ = 4;
    pub const NFTA_NG_SET_NAME: _ = 5;
    pub const NFTA_NG_SET_ID: _ = 6;
    pub const __NFTA_NG_MAX: _ = 7;
}
pub const NFTA_NG_MAX: _ = (__NFTA_NG_MAX - 1);

pub mod nft_ng_types {
    pub const NFT_NG_INCREMENTAL: _ = 0;
    pub const NFT_NG_RANDOM: _ = 1;
    pub const __NFT_NG_MAX: _ = 2;
}
pub const NFT_NG_MAX: _ = (__NFT_NG_MAX - 1);

pub mod nft_tunnel_key_ip_attributes {
    pub const NFTA_TUNNEL_KEY_IP_UNSPEC: _ = 0;
    pub const NFTA_TUNNEL_KEY_IP_SRC: _ = 1;
    pub const NFTA_TUNNEL_KEY_IP_DST: _ = 2;
    pub const __NFTA_TUNNEL_KEY_IP_MAX: _ = 3;
}
pub const NFTA_TUNNEL_KEY_IP_MAX: _ = (__NFTA_TUNNEL_KEY_IP_MAX - 1);

pub mod nft_tunnel_ip6_attributes {
    pub const NFTA_TUNNEL_KEY_IP6_UNSPEC: _ = 0;
    pub const NFTA_TUNNEL_KEY_IP6_SRC: _ = 1;
    pub const NFTA_TUNNEL_KEY_IP6_DST: _ = 2;
    pub const NFTA_TUNNEL_KEY_IP6_FLOWLABEL: _ = 3;
    pub const __NFTA_TUNNEL_KEY_IP6_MAX: _ = 4;
}
pub const NFTA_TUNNEL_KEY_IP6_MAX: _ = (__NFTA_TUNNEL_KEY_IP6_MAX - 1);

pub mod nft_tunnel_opts_attributes {
    pub const NFTA_TUNNEL_KEY_OPTS_UNSPEC: _ = 0;
    pub const NFTA_TUNNEL_KEY_OPTS_VXLAN: _ = 1;
    pub const NFTA_TUNNEL_KEY_OPTS_ERSPAN: _ = 2;
    pub const NFTA_TUNNEL_KEY_OPTS_GENEVE: _ = 3;
    pub const __NFTA_TUNNEL_KEY_OPTS_MAX: _ = 4;
}
pub const NFTA_TUNNEL_KEY_OPTS_MAX: _ = (__NFTA_TUNNEL_KEY_OPTS_MAX - 1);

pub mod nft_tunnel_opts_vxlan_attributes {
    pub const NFTA_TUNNEL_KEY_VXLAN_UNSPEC: _ = 0;
    pub const NFTA_TUNNEL_KEY_VXLAN_GBP: _ = 1;
    pub const __NFTA_TUNNEL_KEY_VXLAN_MAX: _ = 2;
}
pub const NFTA_TUNNEL_KEY_VXLAN_MAX: _ = (__NFTA_TUNNEL_KEY_VXLAN_MAX - 1);

pub mod nft_tunnel_opts_erspan_attributes {
    pub const NFTA_TUNNEL_KEY_ERSPAN_UNSPEC: _ = 0;
    pub const NFTA_TUNNEL_KEY_ERSPAN_VERSION: _ = 1;
    pub const NFTA_TUNNEL_KEY_ERSPAN_V1_INDEX: _ = 2;
    pub const NFTA_TUNNEL_KEY_ERSPAN_V2_HWID: _ = 3;
    pub const NFTA_TUNNEL_KEY_ERSPAN_V2_DIR: _ = 4;
    pub const __NFTA_TUNNEL_KEY_ERSPAN_MAX: _ = 5;
}
pub const NFTA_TUNNEL_KEY_ERSPAN_MAX: _ = (__NFTA_TUNNEL_KEY_ERSPAN_MAX - 1);

pub mod nft_tunnel_opts_geneve_attributes {
    pub const NFTA_TUNNEL_KEY_GENEVE_UNSPEC: _ = 0;
    pub const NFTA_TUNNEL_KEY_GENEVE_CLASS: _ = 1;
    pub const NFTA_TUNNEL_KEY_GENEVE_TYPE: _ = 2;
    pub const NFTA_TUNNEL_KEY_GENEVE_DATA: _ = 3;
    pub const __NFTA_TUNNEL_KEY_GENEVE_MAX: _ = 4;
}
pub const NFTA_TUNNEL_KEY_GENEVE_MAX: _ = (__NFTA_TUNNEL_KEY_GENEVE_MAX - 1);

pub mod nft_tunnel_flags {
    pub const NFT_TUNNEL_F_ZERO_CSUM_TX: _ = (1 << 0);
    pub const NFT_TUNNEL_F_DONT_FRAGMENT: _ = (1 << 1);
    pub const NFT_TUNNEL_F_SEQ_NUMBER: _ = (1 << 2);
}
pub const NFT_TUNNEL_F_MASK: _ = (NFT_TUNNEL_F_ZERO_CSUM_TX | \;
				 NFT_TUNNEL_F_DONT_FRAGMENT | \
				 NFT_TUNNEL_F_SEQ_NUMBER)

pub mod nft_tunnel_key_attributes {
    pub const NFTA_TUNNEL_KEY_UNSPEC: _ = 0;
    pub const NFTA_TUNNEL_KEY_ID: _ = 1;
    pub const NFTA_TUNNEL_KEY_IP: _ = 2;
    pub const NFTA_TUNNEL_KEY_IP6: _ = 3;
    pub const NFTA_TUNNEL_KEY_FLAGS: _ = 4;
    pub const NFTA_TUNNEL_KEY_TOS: _ = 5;
    pub const NFTA_TUNNEL_KEY_TTL: _ = 6;
    pub const NFTA_TUNNEL_KEY_SPORT: _ = 7;
    pub const NFTA_TUNNEL_KEY_DPORT: _ = 8;
    pub const NFTA_TUNNEL_KEY_OPTS: _ = 9;
    pub const __NFTA_TUNNEL_KEY_MAX: _ = 10;
}
pub const NFTA_TUNNEL_KEY_MAX: _ = (__NFTA_TUNNEL_KEY_MAX - 1);

pub mod nft_tunnel_keys {
    pub const NFT_TUNNEL_PATH: _ = 0;
    pub const NFT_TUNNEL_ID: _ = 1;
    pub const __NFT_TUNNEL_MAX: _ = 2;
}
pub const NFT_TUNNEL_MAX: _ = (__NFT_TUNNEL_MAX - 1);

pub mod nft_tunnel_mode {
    pub const NFT_TUNNEL_MODE_NONE: _ = 0;
    pub const NFT_TUNNEL_MODE_RX: _ = 1;
    pub const NFT_TUNNEL_MODE_TX: _ = 2;
    pub const __NFT_TUNNEL_MODE_MAX: _ = 3;
}
pub const NFT_TUNNEL_MODE_MAX: _ = (__NFT_TUNNEL_MODE_MAX - 1);

pub mod nft_tunnel_attributes {
    pub const NFTA_TUNNEL_UNSPEC: _ = 0;
    pub const NFTA_TUNNEL_KEY: _ = 1;
    pub const NFTA_TUNNEL_DREG: _ = 2;
    pub const NFTA_TUNNEL_MODE: _ = 3;
    pub const __NFTA_TUNNEL_MAX: _ = 4;
}
pub const NFTA_TUNNEL_MAX: _ = (__NFTA_TUNNEL_MAX - 1);



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
