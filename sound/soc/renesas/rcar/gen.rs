// SPDX-License-Identifier: GPL-2.0
//
// Renesas R-Car Gen1 SRU/SSI support
//
// Copyright (C) 2013 Renesas Solutions Corp.
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>

/*
 * #define DEBUG
 *
 * you can also add below in
 * ${LINUX}/drivers/base/regmap/regmap.c
 * for regmap debug
 *
 * #define LOG_DEVICE "xxxx.rcar_sound"
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

type u32 = u32;
type phys_addr_t = usize;

#[repr(C)]
pub struct rsnd_gen {
    ops: *mut rsnd_gen_ops,

    /* RSND_BASE_MAX base */
    base: [*mut c_void; RSND_BASE_MAX as usize],
    res: [phys_addr_t; RSND_BASE_MAX as usize],
    regmap: [*mut regmap; RSND_BASE_MAX as usize],

    /* RSND_REG_MAX base */
    regs: [*mut regmap_field; REG_MAX as usize],
    reg_name: [*const c_char; REG_MAX as usize],
}

unsafe fn rsnd_priv_to_gen(p: *mut rsnd_priv) -> *mut rsnd_gen {
    (*p).gen as *mut rsnd_gen
}

unsafe fn rsnd_reg_name(gen: *mut rsnd_gen, id: rsnd_reg) -> *const c_char {
    (*gen).reg_name[id as usize]
}

#[repr(C)]
pub struct rsnd_regmap_field_conf {
    idx: c_int,
    reg_offset: c_uint,
    id_offset: c_uint,
    reg_name: *const c_char,
}

macro_rules! rsnd_reg_set {
    ($id:ident, $offset:expr, $id_offset:expr, $n:expr) => {
        rsnd_regmap_field_conf {
            idx: $id as c_int,
            reg_offset: $offset,
            id_offset: $id_offset,
            reg_name: $n.as_ptr() as *const c_char,
        }
    };
}

/* single address mapping */
macro_rules! rsnd_gen_s_reg {
    ($id:ident, $offset:expr) => {
        rsnd_reg_set!($id, $offset, 0, concat!(stringify!($id), "\0"))
    };
}

/* multi address mapping */
macro_rules! rsnd_gen_m_reg {
    ($id:ident, $offset:expr, $id_offset:expr) => {
        rsnd_reg_set!($id, $offset, $id_offset, concat!(stringify!($id), "\0"))
    };
}

/*
 *		basic function
 */
unsafe fn rsnd_is_accessible_reg(
    priv_: *mut rsnd_priv,
    gen: *mut rsnd_gen,
    reg: rsnd_reg,
) -> c_int {
    if (*gen).regs[reg as usize].is_null() {
        let dev = rsnd_priv_to_dev(priv_);

        dev_err(dev, c"unsupported register access %x\n".as_ptr(), reg as c_int);
        return 0;
    }

    1
}

unsafe fn rsnd_mod_id_cmd(mod_: *mut rsnd_mod) -> c_int {
    if !(*(*mod_).ops).id_cmd.is_none() {
        return ((*(*mod_).ops).id_cmd.unwrap())(mod_);
    }

    rsnd_mod_id(mod_)
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_mod_read(mod_: *mut rsnd_mod, reg: rsnd_reg) -> u32 {
    let priv_ = rsnd_mod_to_priv(mod_);
    let dev = rsnd_priv_to_dev(priv_);
    let gen = rsnd_priv_to_gen(priv_);
    let mut val: u32 = 0;

    if rsnd_is_accessible_reg(priv_, gen, reg) == 0 {
        return 0;
    }

    regmap_fields_read((*gen).regs[reg as usize], rsnd_mod_id_cmd(mod_), &mut val);

    dev_dbg(
        dev,
        c"r %s - %-18s (%4d) : %08x\n".as_ptr(),
        rsnd_mod_name(mod_),
        rsnd_reg_name(gen, reg),
        reg as c_int,
        val,
    );

    val
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_mod_write(mod_: *mut rsnd_mod, reg: rsnd_reg, data: u32) {
    let priv_ = rsnd_mod_to_priv(mod_);
    let dev = rsnd_priv_to_dev(priv_);
    let gen = rsnd_priv_to_gen(priv_);

    if rsnd_is_accessible_reg(priv_, gen, reg) == 0 {
        return;
    }

    regmap_fields_force_write((*gen).regs[reg as usize], rsnd_mod_id_cmd(mod_), data);

    dev_dbg(
        dev,
        c"w %s - %-18s (%4d) : %08x\n".as_ptr(),
        rsnd_mod_name(mod_),
        rsnd_reg_name(gen, reg),
        reg as c_int,
        data,
    );
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_mod_bset(mod_: *mut rsnd_mod, reg: rsnd_reg, mask: u32, data: u32) {
    let priv_ = rsnd_mod_to_priv(mod_);
    let dev = rsnd_priv_to_dev(priv_);
    let gen = rsnd_priv_to_gen(priv_);

    if rsnd_is_accessible_reg(priv_, gen, reg) == 0 {
        return;
    }

    regmap_fields_force_update_bits((*gen).regs[reg as usize], rsnd_mod_id_cmd(mod_), mask, data);

    dev_dbg(
        dev,
        c"b %s - %-18s (%4d) : %08x/%08x\n".as_ptr(),
        rsnd_mod_name(mod_),
        rsnd_reg_name(gen, reg),
        reg as c_int,
        data,
        mask,
    );
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_gen_get_phy_addr(priv_: *mut rsnd_priv, reg_id: c_int) -> phys_addr_t {
    let gen = rsnd_priv_to_gen(priv_);

    (*gen).res[reg_id as usize]
}

/* CONFIG_DEBUG_FS */
#[no_mangle]
pub unsafe extern "C" fn rsnd_gen_get_base_addr(
    priv_: *mut rsnd_priv,
    reg_id: c_int,
) -> *mut c_void {
    let gen = rsnd_priv_to_gen(priv_);

    (*gen).base[reg_id as usize]
}

unsafe fn rsnd_gen_regmap_init(
    priv_: *mut rsnd_priv,
    id_size: c_int,
    reg_id: c_int,
    name: *const c_char,
    conf: &[rsnd_regmap_field_conf],
) -> c_int {
    _rsnd_gen_regmap_init(priv_, id_size, reg_id, name, conf.as_ptr(), conf.len() as c_int)
}

unsafe fn _rsnd_gen_regmap_init(
    priv_: *mut rsnd_priv,
    id_size: c_int,
    reg_id: c_int,
    name: *const c_char,
    conf: *const rsnd_regmap_field_conf,
    conf_size: c_int,
) -> c_int {
    let pdev = rsnd_priv_to_pdev(priv_);
    let gen = rsnd_priv_to_gen(priv_);
    let dev = rsnd_priv_to_dev(priv_);
    let mut res: *mut resource;
    let mut regc: regmap_config = mem::zeroed();
    let mut regs: *mut regmap_field;
    let mut regmap_: *mut regmap;
    let mut regf: reg_field = mem::zeroed();
    let mut base: *mut c_void;
    let mut i: c_int;

    regc.reg_bits = 32;
    regc.val_bits = 32;
    regc.reg_stride = 4;
    regc.name = name;

    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, name);
    if res.is_null() {
        return -ENODEV;
    }

    base = devm_ioremap_resource(dev, res);
    if IS_ERR(base) != 0 {
        return PTR_ERR(base) as c_int;
    }

    regmap_ = devm_regmap_init_mmio(dev, base, &mut regc);
    if IS_ERR(regmap_ as *mut c_void) != 0 {
        return PTR_ERR(regmap_ as *mut c_void) as c_int;
    }

    /* RSND_BASE_MAX base */
    (*gen).base[reg_id as usize] = base;
    (*gen).regmap[reg_id as usize] = regmap_;
    (*gen).res[reg_id as usize] = (*res).start;

    i = 0;
    while i < conf_size {
        regf.reg = (*conf.add(i as usize)).reg_offset;
        regf.id_offset = (*conf.add(i as usize)).id_offset;
        regf.lsb = 0;
        regf.msb = 31;
        regf.id_size = id_size;

        regs = devm_regmap_field_alloc(dev, regmap_, regf);
        if IS_ERR(regs as *mut c_void) != 0 {
            return PTR_ERR(regs as *mut c_void) as c_int;
        }

        /* RSND_REG_MAX base */
        (*gen).regs[(*conf.add(i as usize)).idx as usize] = regs;
        (*gen).reg_name[(*conf.add(i as usize)).idx as usize] =
            (*conf.add(i as usize)).reg_name;

        i += 1;
    }

    0
}

/*
 * (A) : Gen4 is 0xa0c, but it is not used.
 *	see
 *		rsnd_ssiu_init()
 */
static conf_common_ssiu: &[rsnd_regmap_field_conf] = &[
    rsnd_gen_s_reg!(SSI_MODE0, 0x800),
    rsnd_gen_s_reg!(SSI_MODE1, 0x804),
    rsnd_gen_s_reg!(SSI_MODE2, 0x808), // (A)
    rsnd_gen_s_reg!(SSI_CONTROL, 0x810),
    rsnd_gen_s_reg!(SSI_SYS_STATUS0, 0x840),
    rsnd_gen_s_reg!(SSI_SYS_STATUS1, 0x844),
    rsnd_gen_s_reg!(SSI_SYS_STATUS2, 0x848),
    rsnd_gen_s_reg!(SSI_SYS_STATUS3, 0x84c),
    rsnd_gen_s_reg!(SSI_SYS_STATUS4, 0x880),
    rsnd_gen_s_reg!(SSI_SYS_STATUS5, 0x884),
    rsnd_gen_s_reg!(SSI_SYS_STATUS6, 0x888),
    rsnd_gen_s_reg!(SSI_SYS_STATUS7, 0x88c),
    rsnd_gen_s_reg!(SSI_SYS_INT_ENABLE0, 0x850),
    rsnd_gen_s_reg!(SSI_SYS_INT_ENABLE1, 0x854),
    rsnd_gen_s_reg!(SSI_SYS_INT_ENABLE2, 0x858),
    rsnd_gen_s_reg!(SSI_SYS_INT_ENABLE3, 0x85c),
    rsnd_gen_s_reg!(SSI_SYS_INT_ENABLE4, 0x890),
    rsnd_gen_s_reg!(SSI_SYS_INT_ENABLE5, 0x894),
    rsnd_gen_s_reg!(SSI_SYS_INT_ENABLE6, 0x898),
    rsnd_gen_s_reg!(SSI_SYS_INT_ENABLE7, 0x89c),
    rsnd_gen_s_reg!(HDMI0_SEL, 0x9e0),
    rsnd_gen_s_reg!(HDMI1_SEL, 0x9e4),
    rsnd_gen_m_reg!(SSI_BUSIF0_MODE, 0x0, 0x80),
    rsnd_gen_m_reg!(SSI_BUSIF0_ADINR, 0x4, 0x80),
    rsnd_gen_m_reg!(SSI_BUSIF0_DALIGN, 0x8, 0x80),
    rsnd_gen_m_reg!(SSI_BUSIF1_MODE, 0x20, 0x80),
    rsnd_gen_m_reg!(SSI_BUSIF1_ADINR, 0x24, 0x80),
    rsnd_gen_m_reg!(SSI_BUSIF1_DALIGN, 0x28, 0x80),
    rsnd_gen_m_reg!(SSI_BUSIF2_MODE, 0x40, 0x80),
    rsnd_gen_m_reg!(SSI_BUSIF2_ADINR, 0x44, 0x80),
    rsnd_gen_m_reg!(SSI_BUSIF2_DALIGN, 0x48, 0x80),
    rsnd_gen_m_reg!(SSI_BUSIF3_MODE, 0x60, 0x80),
    rsnd_gen_m_reg!(SSI_BUSIF3_ADINR, 0x64, 0x80),
    rsnd_gen_m_reg!(SSI_BUSIF3_DALIGN, 0x68, 0x80),
    rsnd_gen_m_reg!(SSI_BUSIF4_MODE, 0x500, 0x80),
    rsnd_gen_m_reg!(SSI_BUSIF4_ADINR, 0x504, 0x80),
    rsnd_gen_m_reg!(SSI_BUSIF4_DALIGN, 0x508, 0x80),
    rsnd_gen_m_reg!(SSI_BUSIF5_MODE, 0x520, 0x80),
    rsnd_gen_m_reg!(SSI_BUSIF5_ADINR, 0x524, 0x80),
    rsnd_gen_m_reg!(SSI_BUSIF5_DALIGN, 0x528, 0x80),
    rsnd_gen_m_reg!(SSI_BUSIF6_MODE, 0x540, 0x80),
    rsnd_gen_m_reg!(SSI_BUSIF6_ADINR, 0x544, 0x80),
    rsnd_gen_m_reg!(SSI_BUSIF6_DALIGN, 0x548, 0x80),
    rsnd_gen_m_reg!(SSI_BUSIF7_MODE, 0x560, 0x80),
    rsnd_gen_m_reg!(SSI_BUSIF7_ADINR, 0x564, 0x80),
    rsnd_gen_m_reg!(SSI_BUSIF7_DALIGN, 0x568, 0x80),
    rsnd_gen_m_reg!(SSI_MODE, 0xc, 0x80),
    rsnd_gen_m_reg!(SSI_CTRL, 0x10, 0x80),
    rsnd_gen_m_reg!(SSI_INT_ENABLE, 0x18, 0x80),
    rsnd_gen_s_reg!(SSI9_BUSIF0_MODE, 0x48c),
    rsnd_gen_s_reg!(SSI9_BUSIF0_ADINR, 0x484),
    rsnd_gen_s_reg!(SSI9_BUSIF0_DALIGN, 0x488),
    rsnd_gen_s_reg!(SSI9_BUSIF1_MODE, 0x4a0),
    rsnd_gen_s_reg!(SSI9_BUSIF1_ADINR, 0x4a4),
    rsnd_gen_s_reg!(SSI9_BUSIF1_DALIGN, 0x4a8),
    rsnd_gen_s_reg!(SSI9_BUSIF2_MODE, 0x4c0),
    rsnd_gen_s_reg!(SSI9_BUSIF2_ADINR, 0x4c4),
    rsnd_gen_s_reg!(SSI9_BUSIF2_DALIGN, 0x4c8),
    rsnd_gen_s_reg!(SSI9_BUSIF3_MODE, 0x4e0),
    rsnd_gen_s_reg!(SSI9_BUSIF3_ADINR, 0x4e4),
    rsnd_gen_s_reg!(SSI9_BUSIF3_DALIGN, 0x4e8),
    rsnd_gen_s_reg!(SSI9_BUSIF4_MODE, 0xd80),
    rsnd_gen_s_reg!(SSI9_BUSIF4_ADINR, 0xd84),
    rsnd_gen_s_reg!(SSI9_BUSIF4_DALIGN, 0xd88),
    rsnd_gen_s_reg!(SSI9_BUSIF5_MODE, 0xda0),
    rsnd_gen_s_reg!(SSI9_BUSIF5_ADINR, 0xda4),
    rsnd_gen_s_reg!(SSI9_BUSIF5_DALIGN, 0xda8),
    rsnd_gen_s_reg!(SSI9_BUSIF6_MODE, 0xdc0),
    rsnd_gen_s_reg!(SSI9_BUSIF6_ADINR, 0xdc4),
    rsnd_gen_s_reg!(SSI9_BUSIF6_DALIGN, 0xdc8),
    rsnd_gen_s_reg!(SSI9_BUSIF7_MODE, 0xde0),
    rsnd_gen_s_reg!(SSI9_BUSIF7_ADINR, 0xde4),
    rsnd_gen_s_reg!(SSI9_BUSIF7_DALIGN, 0xde8),
];

static conf_common_scu: &[rsnd_regmap_field_conf] = &[
    rsnd_gen_m_reg!(SRC_I_BUSIF_MODE, 0x0, 0x20),
    rsnd_gen_m_reg!(SRC_O_BUSIF_MODE, 0x4, 0x20),
    rsnd_gen_m_reg!(SRC_BUSIF_DALIGN, 0x8, 0x20),
    rsnd_gen_m_reg!(SRC_ROUTE_MODE0, 0xc, 0x20),
    rsnd_gen_m_reg!(SRC_CTRL, 0x10, 0x20),
    rsnd_gen_m_reg!(SRC_INT_ENABLE0, 0x18, 0x20),
    rsnd_gen_m_reg!(CMD_BUSIF_MODE, 0x184, 0x20),
    rsnd_gen_m_reg!(CMD_BUSIF_DALIGN, 0x188, 0x20),
    rsnd_gen_m_reg!(CMD_ROUTE_SLCT, 0x18c, 0x20),
    rsnd_gen_m_reg!(CMD_CTRL, 0x190, 0x20),
    rsnd_gen_s_reg!(SCU_SYS_STATUS0, 0x1c8),
    rsnd_gen_s_reg!(SCU_SYS_INT_EN0, 0x1cc),
    rsnd_gen_s_reg!(SCU_SYS_STATUS1, 0x1d0),
    rsnd_gen_s_reg!(SCU_SYS_INT_EN1, 0x1d4),
    rsnd_gen_m_reg!(SRC_SWRSR, 0x200, 0x40),
    rsnd_gen_m_reg!(SRC_SRCIR, 0x204, 0x40),
    rsnd_gen_m_reg!(SRC_ADINR, 0x214, 0x40),
    rsnd_gen_m_reg!(SRC_IFSCR, 0x21c, 0x40),
    rsnd_gen_m_reg!(SRC_IFSVR, 0x220, 0x40),
    rsnd_gen_m_reg!(SRC_SRCCR, 0x224, 0x40),
    rsnd_gen_m_reg!(SRC_BSDSR, 0x22c, 0x40),
    rsnd_gen_m_reg!(SRC_BSISR, 0x238, 0x40),
    rsnd_gen_m_reg!(CTU_SWRSR, 0x500, 0x100),
    rsnd_gen_m_reg!(CTU_CTUIR, 0x504, 0x100),
    rsnd_gen_m_reg!(CTU_ADINR, 0x508, 0x100),
    rsnd_gen_m_reg!(CTU_CPMDR, 0x510, 0x100),
    rsnd_gen_m_reg!(CTU_SCMDR, 0x514, 0x100),
    rsnd_gen_m_reg!(CTU_SV00R, 0x518, 0x100),
    rsnd_gen_m_reg!(CTU_SV01R, 0x51c, 0x100),
    rsnd_gen_m_reg!(CTU_SV02R, 0x520, 0x100),
    rsnd_gen_m_reg!(CTU_SV03R, 0x524, 0x100),
    rsnd_gen_m_reg!(CTU_SV04R, 0x528, 0x100),
    rsnd_gen_m_reg!(CTU_SV05R, 0x52c, 0x100),
    rsnd_gen_m_reg!(CTU_SV06R, 0x530, 0x100),
    rsnd_gen_m_reg!(CTU_SV07R, 0x534, 0x100),
    rsnd_gen_m_reg!(CTU_SV10R, 0x538, 0x100),
    rsnd_gen_m_reg!(CTU_SV11R, 0x53c, 0x100),
    rsnd_gen_m_reg!(CTU_SV12R, 0x540, 0x100),
    rsnd_gen_m_reg!(CTU_SV13R, 0x544, 0x100),
    rsnd_gen_m_reg!(CTU_SV14R, 0x548, 0x100),
    rsnd_gen_m_reg!(CTU_SV15R, 0x54c, 0x100),
    rsnd_gen_m_reg!(CTU_SV16R, 0x550, 0x100),
    rsnd_gen_m_reg!(CTU_SV17R, 0x554, 0x100),
    rsnd_gen_m_reg!(CTU_SV20R, 0x558, 0x100),
    rsnd_gen_m_reg!(CTU_SV21R, 0x55c, 0x100),
    rsnd_gen_m_reg!(CTU_SV22R, 0x560, 0x100),
    rsnd_gen_m_reg!(CTU_SV23R, 0x564, 0x100),
    rsnd_gen_m_reg!(CTU_SV24R, 0x568, 0x100),
    rsnd_gen_m_reg!(CTU_SV25R, 0x56c, 0x100),
    rsnd_gen_m_reg!(CTU_SV26R, 0x570, 0x100),
    rsnd_gen_m_reg!(CTU_SV27R, 0x574, 0x100),
    rsnd_gen_m_reg!(CTU_SV30R, 0x578, 0x100),
    rsnd_gen_m_reg!(CTU_SV31R, 0x57c, 0x100),
    rsnd_gen_m_reg!(CTU_SV32R, 0x580, 0x100),
    rsnd_gen_m_reg!(CTU_SV33R, 0x584, 0x100),
    rsnd_gen_m_reg!(CTU_SV34R, 0x588, 0x100),
    rsnd_gen_m_reg!(CTU_SV35R, 0x58c, 0x100),
    rsnd_gen_m_reg!(CTU_SV36R, 0x590, 0x100),
    rsnd_gen_m_reg!(CTU_SV37R, 0x594, 0x100),
    rsnd_gen_m_reg!(MIX_SWRSR, 0xd00, 0x40),
    rsnd_gen_m_reg!(MIX_MIXIR, 0xd04, 0x40),
    rsnd_gen_m_reg!(MIX_ADINR, 0xd08, 0x40),
    rsnd_gen_m_reg!(MIX_MIXMR, 0xd10, 0x40),
    rsnd_gen_m_reg!(MIX_MVPDR, 0xd14, 0x40),
    rsnd_gen_m_reg!(MIX_MDBAR, 0xd18, 0x40),
    rsnd_gen_m_reg!(MIX_MDBBR, 0xd1c, 0x40),
    rsnd_gen_m_reg!(MIX_MDBCR, 0xd20, 0x40),
    rsnd_gen_m_reg!(MIX_MDBDR, 0xd24, 0x40),
    rsnd_gen_m_reg!(MIX_MDBER, 0xd28, 0x40),
    rsnd_gen_m_reg!(DVC_SWRSR, 0xe00, 0x100),
    rsnd_gen_m_reg!(DVC_DVUIR, 0xe04, 0x100),
    rsnd_gen_m_reg!(DVC_ADINR, 0xe08, 0x100),
    rsnd_gen_m_reg!(DVC_DVUCR, 0xe10, 0x100),
    rsnd_gen_m_reg!(DVC_ZCMCR, 0xe14, 0x100),
    rsnd_gen_m_reg!(DVC_VRCTR, 0xe18, 0x100),
    rsnd_gen_m_reg!(DVC_VRPDR, 0xe1c, 0x100),
    rsnd_gen_m_reg!(DVC_VRDBR, 0xe20, 0x100),
    rsnd_gen_m_reg!(DVC_VOL0R, 0xe28, 0x100),
    rsnd_gen_m_reg!(DVC_VOL1R, 0xe2c, 0x100),
    rsnd_gen_m_reg!(DVC_VOL2R, 0xe30, 0x100),
    rsnd_gen_m_reg!(DVC_VOL3R, 0xe34, 0x100),
    rsnd_gen_m_reg!(DVC_VOL4R, 0xe38, 0x100),
    rsnd_gen_m_reg!(DVC_VOL5R, 0xe3c, 0x100),
    rsnd_gen_m_reg!(DVC_VOL6R, 0xe40, 0x100),
    rsnd_gen_m_reg!(DVC_VOL7R, 0xe44, 0x100),
    rsnd_gen_m_reg!(DVC_DVUER, 0xe48, 0x100),
];

static conf_common_adg: &[rsnd_regmap_field_conf] = &[
    rsnd_gen_s_reg!(BRRA, 0x00),
    rsnd_gen_s_reg!(BRRB, 0x04),
    rsnd_gen_s_reg!(BRGCKR, 0x08),
    rsnd_gen_s_reg!(AUDIO_CLK_SEL0, 0x0c),
    rsnd_gen_s_reg!(AUDIO_CLK_SEL1, 0x10),
    rsnd_gen_s_reg!(AUDIO_CLK_SEL2, 0x14),
    rsnd_gen_s_reg!(DIV_EN, 0x30),
    rsnd_gen_s_reg!(SRCIN_TIMSEL0, 0x34),
    rsnd_gen_s_reg!(SRCIN_TIMSEL1, 0x38),
    rsnd_gen_s_reg!(SRCIN_TIMSEL2, 0x3c),
    rsnd_gen_s_reg!(SRCIN_TIMSEL3, 0x40),
    rsnd_gen_s_reg!(SRCIN_TIMSEL4, 0x44),
    rsnd_gen_s_reg!(SRCOUT_TIMSEL0, 0x48),
    rsnd_gen_s_reg!(SRCOUT_TIMSEL1, 0x4c),
    rsnd_gen_s_reg!(SRCOUT_TIMSEL2, 0x50),
    rsnd_gen_s_reg!(SRCOUT_TIMSEL3, 0x54),
    rsnd_gen_s_reg!(SRCOUT_TIMSEL4, 0x58),
    rsnd_gen_s_reg!(CMDOUT_TIMSEL, 0x5c),
];

static conf_common_ssi: &[rsnd_regmap_field_conf] = &[
    rsnd_gen_m_reg!(SSICR, 0x00, 0x40),
    rsnd_gen_m_reg!(SSISR, 0x04, 0x40),
    rsnd_gen_m_reg!(SSITDR, 0x08, 0x40),
    rsnd_gen_m_reg!(SSIRDR, 0x0c, 0x40),
    rsnd_gen_m_reg!(SSIWSR, 0x20, 0x40),
];

/*
 *		Gen4
 */
unsafe fn rsnd_gen4_probe(priv_: *mut rsnd_priv) -> c_int {
    let conf_null: &[rsnd_regmap_field_conf] = &[];

    /*
     * ssiu: SSIU0
     * ssi : SSI0
     */
    let ret_ssiu = rsnd_gen_regmap_init(priv_, 1, RSND_BASE_SSIU, c"ssiu".as_ptr(), conf_common_ssiu);
    let ret_ssi = rsnd_gen_regmap_init(priv_, 1, RSND_BASE_SSI, c"ssi".as_ptr(), conf_common_ssi);
    let ret_adg = rsnd_gen_regmap_init(priv_, 1, RSND_BASE_ADG, c"adg".as_ptr(), conf_common_adg);
    let ret_sdmc = rsnd_gen_regmap_init(priv_, 1, RSND_BASE_SDMC, c"sdmc".as_ptr(), conf_null);

    ret_adg | ret_ssiu | ret_ssi | ret_sdmc
}

/*
 *		Gen2
 */
unsafe fn rsnd_gen2_probe(priv_: *mut rsnd_priv) -> c_int {
    /*
     * ssi : SSI0  - SSI9
     * ssiu: SSIU0 - SSIU9
     * scu : SRC0  - SRC9 etc
     */
    let ret_ssi = rsnd_gen_regmap_init(priv_, 10, RSND_BASE_SSI, c"ssi".as_ptr(), conf_common_ssi);
    let ret_ssiu = rsnd_gen_regmap_init(priv_, 10, RSND_BASE_SSIU, c"ssiu".as_ptr(), conf_common_ssiu);
    let ret_scu = rsnd_gen_regmap_init(priv_, 10, RSND_BASE_SCU, c"scu".as_ptr(), conf_common_scu);
    let ret_adg = rsnd_gen_regmap_init(priv_, 1, RSND_BASE_ADG, c"adg".as_ptr(), conf_common_adg);

    ret_ssi | ret_ssiu | ret_scu | ret_adg
}

/*
 *		Gen1
 */
unsafe fn rsnd_gen1_probe(priv_: *mut rsnd_priv) -> c_int {
    /*
     * ssi : SSI0 - SSI8
     */
    let ret_ssi = rsnd_gen_regmap_init(priv_, 9, RSND_BASE_SSI, c"ssi".as_ptr(), conf_common_ssi);
    let ret_adg = rsnd_gen_regmap_init(priv_, 1, RSND_BASE_ADG, c"adg".as_ptr(), conf_common_adg);

    ret_adg | ret_ssi
}

/*
 *		RZ/G3E Generation
 */
unsafe fn rsnd_rzg3e_probe(priv_: *mut rsnd_priv) -> c_int {
    static conf_ssiu: &[rsnd_regmap_field_conf] = &[
        rsnd_gen_s_reg!(SSI_MODE1, 0x804),
        rsnd_gen_s_reg!(SSI_MODE2, 0x808),
        rsnd_gen_s_reg!(SSI_MODE3, 0x80c),
        rsnd_gen_s_reg!(SSI_CONTROL, 0x810),
        rsnd_gen_s_reg!(SSI_CONTROL2, 0x814),
        rsnd_gen_s_reg!(SSI_SYS_STATUS0, 0x840),
        rsnd_gen_s_reg!(SSI_SYS_STATUS1, 0x844),
        rsnd_gen_s_reg!(SSI_SYS_STATUS2, 0x848),
        rsnd_gen_s_reg!(SSI_SYS_STATUS3, 0x84c),
        rsnd_gen_s_reg!(SSI_SYS_INT_ENABLE0, 0x850),
        rsnd_gen_s_reg!(SSI_SYS_INT_ENABLE1, 0x854),
        rsnd_gen_s_reg!(SSI_SYS_INT_ENABLE2, 0x858),
        rsnd_gen_s_reg!(SSI_SYS_INT_ENABLE3, 0x85c),
        rsnd_gen_m_reg!(SSI_BUSIF0_MODE, 0x0, 0x80),
        rsnd_gen_m_reg!(SSI_BUSIF0_ADINR, 0x4, 0x80),
        rsnd_gen_m_reg!(SSI_BUSIF0_DALIGN, 0x8, 0x80),
        rsnd_gen_m_reg!(SSI_BUSIF1_MODE, 0x20, 0x80),
        rsnd_gen_m_reg!(SSI_BUSIF1_ADINR, 0x24, 0x80),
        rsnd_gen_m_reg!(SSI_BUSIF1_DALIGN, 0x28, 0x80),
        rsnd_gen_m_reg!(SSI_BUSIF2_MODE, 0x40, 0x80),
        rsnd_gen_m_reg!(SSI_BUSIF2_ADINR, 0x44, 0x80),
        rsnd_gen_m_reg!(SSI_BUSIF2_DALIGN, 0x48, 0x80),
        rsnd_gen_m_reg!(SSI_BUSIF3_MODE, 0x60, 0x80),
        rsnd_gen_m_reg!(SSI_BUSIF3_ADINR, 0x64, 0x80),
        rsnd_gen_m_reg!(SSI_BUSIF3_DALIGN, 0x68, 0x80),
        rsnd_gen_m_reg!(SSI_MODE, 0xc, 0x80),
        rsnd_gen_m_reg!(SSI_CTRL, 0x10, 0x80),
        rsnd_gen_m_reg!(SSI_INT_ENABLE, 0x18, 0x80),
        rsnd_gen_s_reg!(SSI9_BUSIF0_MODE, 0x480),
        rsnd_gen_s_reg!(SSI9_BUSIF0_ADINR, 0x484),
        rsnd_gen_s_reg!(SSI9_BUSIF0_DALIGN, 0x488),
        rsnd_gen_s_reg!(SSI9_BUSIF1_MODE, 0x4a0),
        rsnd_gen_s_reg!(SSI9_BUSIF1_ADINR, 0x4a4),
        rsnd_gen_s_reg!(SSI9_BUSIF1_DALIGN, 0x4a8),
        rsnd_gen_s_reg!(SSI9_BUSIF2_MODE, 0x4c0),
        rsnd_gen_s_reg!(SSI9_BUSIF2_ADINR, 0x4c4),
        rsnd_gen_s_reg!(SSI9_BUSIF2_DALIGN, 0x4c8),
        rsnd_gen_s_reg!(SSI9_BUSIF3_MODE, 0x4e0),
        rsnd_gen_s_reg!(SSI9_BUSIF3_ADINR, 0x4e4),
        rsnd_gen_s_reg!(SSI9_BUSIF3_DALIGN, 0x4e8),
    ];
    static conf_scu: &[rsnd_regmap_field_conf] = conf_common_scu;
    static conf_adg: &[rsnd_regmap_field_conf] = &[
        rsnd_gen_s_reg!(BRRA, 0x00),
        rsnd_gen_s_reg!(BRRB, 0x04),
        rsnd_gen_s_reg!(BRGCKR, 0x08),
        rsnd_gen_s_reg!(AUDIO_CLK_SEL0, 0x0c),
        rsnd_gen_s_reg!(AUDIO_CLK_SEL1, 0x10),
        rsnd_gen_s_reg!(AUDIO_CLK_SEL2, 0x14),
        rsnd_gen_s_reg!(AUDIO_CLK_SEL3, 0x18),
        rsnd_gen_s_reg!(DIV_EN, 0x30),
        rsnd_gen_s_reg!(SRCIN_TIMSEL0, 0x34),
        rsnd_gen_s_reg!(SRCIN_TIMSEL1, 0x38),
        rsnd_gen_s_reg!(SRCIN_TIMSEL2, 0x3c),
        rsnd_gen_s_reg!(SRCIN_TIMSEL3, 0x40),
        rsnd_gen_s_reg!(SRCIN_TIMSEL4, 0x44),
        rsnd_gen_s_reg!(SRCOUT_TIMSEL0, 0x48),
        rsnd_gen_s_reg!(SRCOUT_TIMSEL1, 0x4c),
        rsnd_gen_s_reg!(SRCOUT_TIMSEL2, 0x50),
        rsnd_gen_s_reg!(SRCOUT_TIMSEL3, 0x54),
        rsnd_gen_s_reg!(SRCOUT_TIMSEL4, 0x58),
        rsnd_gen_s_reg!(CMDOUT_TIMSEL, 0x5c),
    ];
    static conf_ssi: &[rsnd_regmap_field_conf] = &[
        rsnd_gen_m_reg!(SSICR, 0x00, 0x40),
        rsnd_gen_m_reg!(SSISR, 0x04, 0x40),
        rsnd_gen_m_reg!(SSIWSR, 0x20, 0x40),
    ];
    let mut ret: c_int;

    ret = rsnd_gen_regmap_init(priv_, 10, RSND_BASE_SCU, c"scu".as_ptr(), conf_scu);
    if ret < 0 {
        return ret;
    }

    ret = rsnd_gen_regmap_init(priv_, 1, RSND_BASE_ADG, c"adg".as_ptr(), conf_adg);
    if ret < 0 {
        return ret;
    }

    ret = rsnd_gen_regmap_init(priv_, 10, RSND_BASE_SSIU, c"ssiu".as_ptr(), conf_ssiu);
    if ret < 0 {
        return ret;
    }

    rsnd_gen_regmap_init(priv_, 10, RSND_BASE_SSI, c"ssi".as_ptr(), conf_ssi)
}

/*
 *		Gen
 */
#[no_mangle]
pub unsafe extern "C" fn rsnd_gen_probe(priv_: *mut rsnd_priv) -> c_int {
    let dev = rsnd_priv_to_dev(priv_);
    let gen: *mut rsnd_gen;
    let mut ret: c_int;

    gen = devm_kzalloc(dev, mem::size_of::<rsnd_gen>(), GFP_KERNEL) as *mut rsnd_gen;
    if gen.is_null() {
        return -ENOMEM;
    }

    (*priv_).gen = gen as *mut c_void;

    ret = -ENODEV;
    if rsnd_is_gen1(priv_) != 0 {
        ret = rsnd_gen1_probe(priv_);
    } else if rsnd_is_gen2(priv_) != 0 || rsnd_is_gen3(priv_) != 0 {
        ret = rsnd_gen2_probe(priv_);
    } else if rsnd_is_gen4(priv_) != 0 {
        ret = rsnd_gen4_probe(priv_);
    } else if rsnd_is_rzg3e(priv_) != 0 {
        ret = rsnd_rzg3e_probe(priv_);
    }

    if ret < 0 {
        dev_err(dev, c"unknown generation R-Car sound device\n".as_ptr());
    }

    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
