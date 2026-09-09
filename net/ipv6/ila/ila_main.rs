// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the kernel and ila.h are intentionally external.

#[repr(C)]
pub struct NlaPolicy {
    pub type_: u32,
}

#[repr(C)]
pub struct GenlOps {
    pub cmd: u8,
    pub validate: u32,
    pub doit: Option<unsafe extern "C" fn() -> i32>,
    pub flags: u32,
    pub start: Option<unsafe extern "C" fn() -> i32>,
    pub dumpit: Option<unsafe extern "C" fn() -> i32>,
    pub done: Option<unsafe extern "C" fn() -> i32>,
}

#[repr(C)]
pub struct GenlFamily {
    pub hdrsize: u16,
    pub name: *const u8,
    pub version: u8,
    pub maxattr: u8,
    pub policy: *const NlaPolicy,
    pub netnsok: bool,
    pub parallel_ops: bool,
    pub module: *mut core::ffi::c_void,
    pub ops: *const GenlOps,
    pub n_ops: usize,
    pub resv_start_op: u8,
}

#[repr(C)]
pub struct PernetOperations {
    pub init: Option<unsafe extern "C" fn(*mut Net) -> i32>,
    pub pre_exit: Option<unsafe extern "C" fn(*mut Net)>,
    pub exit: Option<unsafe extern "C" fn(*mut Net)>,
    pub id: *mut u32,
    pub size: usize,
}

#[repr(C)]
pub struct Net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct IlaNet {
    _private: [u8; 0],
}

extern "C" {
    static THIS_MODULE: *mut core::ffi::c_void;
    static ILA_GENL_NAME: *const u8;
    static ILA_GENL_VERSION: u8;
    static ILA_ATTR_MAX: u8;
    static ILA_CMD_FLUSH: u8;

    fn ila_xlat_nl_cmd_add_mapping() -> i32;
    fn ila_xlat_nl_cmd_del_mapping() -> i32;
    fn ila_xlat_nl_cmd_flush() -> i32;
    fn ila_xlat_nl_cmd_get_mapping() -> i32;
    fn ila_xlat_nl_dump_start() -> i32;
    fn ila_xlat_nl_dump() -> i32;
    fn ila_xlat_nl_dump_done() -> i32;
    fn ila_xlat_init_net(net: *mut Net) -> i32;
    fn ila_xlat_pre_exit_net(net: *mut Net);
    fn ila_xlat_exit_net(net: *mut Net);
    fn register_pernet_device(ops: *mut PernetOperations) -> i32;
    fn unregister_pernet_device(ops: *mut PernetOperations);
    fn genl_register_family(family: *mut GenlFamily) -> i32;
    fn genl_unregister_family(family: *mut GenlFamily);
    fn ila_lwt_init() -> i32;
    fn ila_lwt_fini();
}

const NLA_U8: u32 = 1;
const NLA_U32: u32 = 5;
const NLA_U64: u32 = 11;
const GENL_DONT_VALIDATE_STRICT: u32 = 1;
const GENL_DONT_VALIDATE_DUMP: u32 = 2;
const GENL_ADMIN_PERM: u32 = 1;

static mut ILA_NL_POLICY: [NlaPolicy; 6] = [NlaPolicy { type_: 0 }; 6];

static mut ILA_NL_OPS: [GenlOps; 4] = [
    GenlOps { cmd: 0, validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP, doit: None, flags: GENL_ADMIN_PERM, start: None, dumpit: None, done: None },
    GenlOps { cmd: 0, validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP, doit: None, flags: GENL_ADMIN_PERM, start: None, dumpit: None, done: None },
    GenlOps { cmd: 0, validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP, doit: None, flags: GENL_ADMIN_PERM, start: None, dumpit: None, done: None },
    GenlOps { cmd: 0, validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP, doit: None, flags: 0, start: None, dumpit: None, done: None },
];

pub static mut ILA_NET_ID: u32 = 0;
pub static mut ILA_NL_FAMILY: GenlFamily = GenlFamily {
    hdrsize: 0, name: core::ptr::null(), version: 0, maxattr: 0,
    policy: core::ptr::null(), netnsok: true, parallel_ops: true,
    module: core::ptr::null_mut(), ops: core::ptr::null(), n_ops: 0,
    resv_start_op: 0,
};

unsafe extern "C" fn ila_init_net(net: *mut Net) -> i32 {
    let err = ila_xlat_init_net(net);
    if err != 0 { return err; }
    0
}

unsafe extern "C" fn ila_pre_exit_net(net: *mut Net) { ila_xlat_pre_exit_net(net); }
unsafe extern "C" fn ila_exit_net(net: *mut Net) { ila_xlat_exit_net(net); }

static mut ILA_NET_OPS: PernetOperations = PernetOperations {
    init: Some(ila_init_net), pre_exit: Some(ila_pre_exit_net), exit: Some(ila_exit_net),
    id: unsafe { &raw mut ILA_NET_ID }, size: core::mem::size_of::<IlaNet>(),
};

unsafe extern "C" fn ila_init() -> i32 {
    let mut ret = register_pernet_device(&raw mut ILA_NET_OPS);
    if ret != 0 { return ret; }
    ret = genl_register_family(&raw mut ILA_NL_FAMILY);
    if ret != 0 { unregister_pernet_device(&raw mut ILA_NET_OPS); return ret; }
    ret = ila_lwt_init();
    if ret != 0 {
        genl_unregister_family(&raw mut ILA_NL_FAMILY);
        unregister_pernet_device(&raw mut ILA_NET_OPS);
        return ret;
    }
    0
}

unsafe extern "C" fn ila_fini() {
    ila_lwt_fini();
    genl_unregister_family(&raw mut ILA_NL_FAMILY);
    unregister_pernet_device(&raw mut ILA_NET_OPS);
}

// module_init(ila_init); module_exit(ila_fini);
// MODULE_AUTHOR("Tom Herbert <tom@herbertland.com>");
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("IPv6: Identifier Locator Addressing (ILA)");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
