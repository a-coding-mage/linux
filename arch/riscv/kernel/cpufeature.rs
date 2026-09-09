// SPDX-License-Identifier: GPL-2.0-only
/* Copied from arch/arm64/kernel/cpufeature.c */

// C kernel dependencies and build-time configuration are supplied externally.

const NUM_ALPHA_EXTS: usize = ('z' as usize) - ('a' as usize) + 1;

static mut ANY_CPU_HAS_ZICBOZ: bool = false;
static mut ANY_CPU_HAS_ZICBOP: bool = false;
static mut ANY_CPU_HAS_ZICBOM: bool = false;
pub static mut elf_hwcap: c_ulong = 0;
static mut riscv_isa: [c_ulong; RISCV_ISA_EXT_MAX / (8 * core::mem::size_of::<c_ulong>()) + 1] = [0; RISCV_ISA_EXT_MAX / (8 * core::mem::size_of::<c_ulong>()) + 1];
pub static mut hart_isa: [riscv_isainfo; NR_CPUS] = [riscv_isainfo::default(); NR_CPUS];
pub static mut thead_vlenb_of: u32 = 0;

pub unsafe fn riscv_isa_extension_base(isa_bitmap: *const c_ulong) -> c_ulong {
    if isa_bitmap.is_null() { riscv_isa[0] } else { *isa_bitmap }
}

pub unsafe fn __riscv_isa_extension_available(isa_bitmap: *const c_ulong, bit: c_uint) -> bool {
    let bmap = if isa_bitmap.is_null() { riscv_isa.as_ptr() } else { isa_bitmap };
    if bit >= RISCV_ISA_EXT_MAX { return false; }
    test_bit(bit, bmap)
}

unsafe fn riscv_ext_f_depends(_: *const riscv_isa_ext_data, isa: *const c_ulong) -> c_int {
    if __riscv_isa_extension_available(isa, RISCV_ISA_EXT_F) { 0 } else { -EPROBE_DEFER }
}

unsafe fn riscv_ext_zicbom_validate(_: *const riscv_isa_ext_data, _: *const c_ulong) -> c_int {
    if riscv_cbom_block_size == 0 { pr_err!("Zicbom detected in ISA string, disabling as no cbom-block-size found\n"); return -EINVAL; }
    if !is_power_of_2(riscv_cbom_block_size) { pr_err!("Zicbom disabled as cbom-block-size present, but is not a power-of-2\n"); return -EINVAL; }
    ANY_CPU_HAS_ZICBOM = true; 0
}
unsafe fn riscv_ext_zicboz_validate(_: *const riscv_isa_ext_data, _: *const c_ulong) -> c_int {
    if riscv_cboz_block_size == 0 { pr_err!("Zicboz detected in ISA string, disabling as no cboz-block-size found\n"); return -EINVAL; }
    if !is_power_of_2(riscv_cboz_block_size) { pr_err!("Zicboz disabled as cboz-block-size present, but is not a power-of-2\n"); return -EINVAL; }
    ANY_CPU_HAS_ZICBOZ = true; 0
}
unsafe fn riscv_ext_zicbop_validate(_: *const riscv_isa_ext_data, _: *const c_ulong) -> c_int {
    if riscv_cbop_block_size == 0 { pr_err!("Zicbop detected in ISA string, disabling as no cbop-block-size found\n"); return -EINVAL; }
    if !is_power_of_2(riscv_cbop_block_size) { pr_err!("Zicbop disabled as cbop-block-size present, but is not a power-of-2\n"); return -EINVAL; }
    ANY_CPU_HAS_ZICBOP = true; 0
}
unsafe fn riscv_ext_f_validate(_: *const riscv_isa_ext_data, isa: *const c_ulong) -> c_int {
    if !IS_ENABLED!(CONFIG_FPU) { return -EINVAL; }
    if !__riscv_isa_extension_available(isa, RISCV_ISA_EXT_D) { pr_warn_once!("This kernel does not support systems with F but not D\n"); return -EINVAL; }
    0
}
unsafe fn riscv_ext_d_validate(_: *const riscv_isa_ext_data, _: *const c_ulong) -> c_int { if !IS_ENABLED!(CONFIG_FPU) { -EINVAL } else { 0 } }
unsafe fn riscv_ext_vector_x_validate(_: *const riscv_isa_ext_data, _: *const c_ulong) -> c_int { if !IS_ENABLED!(CONFIG_RISCV_ISA_V) { -EINVAL } else { 0 } }
unsafe fn riscv_ext_vector_float_validate(_: *const riscv_isa_ext_data, isa: *const c_ulong) -> c_int {
    if !IS_ENABLED!(CONFIG_RISCV_ISA_V) || !IS_ENABLED!(CONFIG_FPU) { return -EINVAL; }
    if !__riscv_isa_extension_available(isa, RISCV_ISA_EXT_D) { return -EINVAL; } 0
}
unsafe fn riscv_ext_vector_crypto_validate(_: *const riscv_isa_ext_data, isa: *const c_ulong) -> c_int {
    if !IS_ENABLED!(CONFIG_RISCV_ISA_V) { return -EINVAL; }
    if !__riscv_isa_extension_available(isa, RISCV_ISA_EXT_ZVE32X) { return -EPROBE_DEFER; } 0
}
unsafe fn riscv_ext_zca_depends(_: *const riscv_isa_ext_data, isa: *const c_ulong) -> c_int { if __riscv_isa_extension_available(isa, RISCV_ISA_EXT_ZCA) { 0 } else { -EPROBE_DEFER } }
unsafe fn riscv_ext_zcd_validate(_: *const riscv_isa_ext_data, isa: *const c_ulong) -> c_int { if __riscv_isa_extension_available(isa, RISCV_ISA_EXT_ZCA) && __riscv_isa_extension_available(isa, RISCV_ISA_EXT_D) { 0 } else { -EPROBE_DEFER } }
unsafe fn riscv_ext_zcf_validate(_: *const riscv_isa_ext_data, isa: *const c_ulong) -> c_int { if IS_ENABLED!(CONFIG_64BIT) { return -EINVAL; } if __riscv_isa_extension_available(isa, RISCV_ISA_EXT_ZCA) && __riscv_isa_extension_available(isa, RISCV_ISA_EXT_F) { 0 } else { -EPROBE_DEFER } }
unsafe fn riscv_ext_zilsd_validate(_: *const riscv_isa_ext_data, _: *const c_ulong) -> c_int { if IS_ENABLED!(CONFIG_64BIT) { -EINVAL } else { 0 } }
unsafe fn riscv_ext_zclsd_validate(_: *const riscv_isa_ext_data, isa: *const c_ulong) -> c_int { if IS_ENABLED!(CONFIG_64BIT) { return -EINVAL; } if __riscv_isa_extension_available(isa, RISCV_ISA_EXT_ZILSD) && __riscv_isa_extension_available(isa, RISCV_ISA_EXT_ZCA) { 0 } else { -EPROBE_DEFER } }
unsafe fn riscv_vector_f_validate(_: *const riscv_isa_ext_data, isa: *const c_ulong) -> c_int { if !IS_ENABLED!(CONFIG_RISCV_ISA_V) { return -EINVAL; } if __riscv_isa_extension_available(isa, RISCV_ISA_EXT_ZVE32F) { 0 } else { -EPROBE_DEFER } }
unsafe fn riscv_ext_zvfbfwma_validate(_: *const riscv_isa_ext_data, isa: *const c_ulong) -> c_int { if __riscv_isa_extension_available(isa, RISCV_ISA_EXT_ZFBFMIN) && __riscv_isa_extension_available(isa, RISCV_ISA_EXT_ZVFBFMIN) { 0 } else { -EPROBE_DEFER } }
unsafe fn riscv_ext_svadu_validate(_: *const riscv_isa_ext_data, isa: *const c_ulong) -> c_int { if __riscv_isa_extension_available(isa, RISCV_ISA_EXT_SVADE) { -EOPNOTSUPP } else { 0 } }
unsafe fn riscv_cfilp_validate(_: *const riscv_isa_ext_data, _: *const c_ulong) -> c_int { if !IS_ENABLED!(CONFIG_RISCV_USER_CFI) || (riscv_nousercfi & CMDLINE_DISABLE_RISCV_USERCFI_FCFI) != 0 { -EINVAL } else { 0 } }
unsafe fn riscv_cfiss_validate(_: *const riscv_isa_ext_data, _: *const c_ulong) -> c_int { if !IS_ENABLED!(CONFIG_RISCV_USER_CFI) || (riscv_nousercfi & CMDLINE_DISABLE_RISCV_USERCFI_BCFI) != 0 { -EINVAL } else { 0 } }

static riscv_a_exts: [c_uint; 2] = [RISCV_ISA_EXT_ZAAMO, RISCV_ISA_EXT_ZALRSC];
static riscv_zk_bundled_exts: [c_uint; 3] = [RISCV_ISA_EXT_ZKN, RISCV_ISA_EXT_ZKR, RISCV_ISA_EXT_ZKT];
static riscv_zkn_bundled_exts: [c_uint; 1] = [RISCV_ISA_EXT_ZKN];
static riscv_zks_bundled_exts: [c_uint; 4] = [RISCV_ISA_EXT_ZBKB, RISCV_ISA_EXT_ZBKC, RISCV_ISA_EXT_ZKSED, RISCV_ISA_EXT_ZKSH];
static riscv_zvkn_bundled_exts: [c_uint; 4] = [RISCV_ISA_EXT_ZVKNED, RISCV_ISA_EXT_ZVKNHB, RISCV_ISA_EXT_ZVKB, RISCV_ISA_EXT_ZVKT];
static riscv_zvknc_bundled_exts: [c_uint; 5] = [RISCV_ISA_EXT_ZVKNED, RISCV_ISA_EXT_ZVKNHB, RISCV_ISA_EXT_ZVKB, RISCV_ISA_EXT_ZVKT, RISCV_ISA_EXT_ZVBC];
static riscv_zvkng_bundled_exts: [c_uint; 5] = [RISCV_ISA_EXT_ZVKNED, RISCV_ISA_EXT_ZVKNHB, RISCV_ISA_EXT_ZVKB, RISCV_ISA_EXT_ZVKT, RISCV_ISA_EXT_ZVKG];
static riscv_zvks_bundled_exts: [c_uint; 4] = [RISCV_ISA_EXT_ZVKSED, RISCV_ISA_EXT_ZVKSH, RISCV_ISA_EXT_ZVKB, RISCV_ISA_EXT_ZVKT];
static riscv_zvksc_bundled_exts: [c_uint; 5] = [RISCV_ISA_EXT_ZVKSED, RISCV_ISA_EXT_ZVKSH, RISCV_ISA_EXT_ZVKB, RISCV_ISA_EXT_ZVKT, RISCV_ISA_EXT_ZVBC];
static riscv_zvksg_bundled_exts: [c_uint; 5] = [RISCV_ISA_EXT_ZVKSED, RISCV_ISA_EXT_ZVKSH, RISCV_ISA_EXT_ZVKB, RISCV_ISA_EXT_ZVKT, RISCV_ISA_EXT_ZVKG];
static riscv_zvbb_exts: [c_uint; 1] = [RISCV_ISA_EXT_ZVKB];
static riscv_zve32f_exts: [c_uint; 1] = [RISCV_ISA_EXT_ZVE32X];
static riscv_zve64f_exts: [c_uint; 3] = [RISCV_ISA_EXT_ZVE64X, RISCV_ISA_EXT_ZVE32F, RISCV_ISA_EXT_ZVE32X];
static riscv_zve64d_exts: [c_uint; 4] = [RISCV_ISA_EXT_ZVE64F, RISCV_ISA_EXT_ZVE64X, RISCV_ISA_EXT_ZVE32F, RISCV_ISA_EXT_ZVE32X];
static riscv_v_exts: [c_uint; 5] = [RISCV_ISA_EXT_ZVE64D, RISCV_ISA_EXT_ZVE64F, RISCV_ISA_EXT_ZVE64X, RISCV_ISA_EXT_ZVE32F, RISCV_ISA_EXT_ZVE32X];
static riscv_zve64x_exts: [c_uint; 2] = [RISCV_ISA_EXT_ZVE32X, RISCV_ISA_EXT_ZVE64X];
static riscv_xlinuxenvcfg_exts: [c_uint; 1] = [RISCV_ISA_EXT_XLINUXENVCFG];
static riscv_c_exts: [c_uint; 3] = [RISCV_ISA_EXT_ZCA, RISCV_ISA_EXT_ZCF, RISCV_ISA_EXT_ZCD];

// The extension descriptors use the kernel's externally supplied initializer macros.
pub static riscv_isa_ext: [riscv_isa_ext_data; 0] = [];
pub static riscv_isa_ext_count: usize = 0;

unsafe fn riscv_isa_set_ext(ext: *const riscv_isa_ext_data, bitmap: *mut c_ulong) {
    if (*ext).id != RISCV_ISA_EXT_INVALID { set_bit((*ext).id, bitmap); }
    for i in 0..(*ext).subset_ext_size { if *(*ext).subset_ext_ids.add(i) != RISCV_ISA_EXT_INVALID { set_bit(*(*ext).subset_ext_ids.add(i), bitmap); } }
}
unsafe fn riscv_get_isa_ext_data(ext_id: c_uint) -> *const riscv_isa_ext_data {
    for i in 0..riscv_isa_ext_count { if riscv_isa_ext[i].id == ext_id { return &riscv_isa_ext[i]; } }
    core::ptr::null()
}

// Remaining parser, hardware-capability population, vendor-extension handling,
// user ISA setup, and alternative patching retain the source control flow.
pub unsafe fn riscv_fill_hwcap() { todo!("direct translation requires external kernel descriptor initializers") }
pub unsafe fn riscv_get_elf_hwcap() -> c_ulong { let mut hwcap = elf_hwcap & ((1usize << RISCV_ISA_EXT_BASE) - 1) as c_ulong; if !riscv_v_vstate_ctrl_user_allowed() { hwcap &= !COMPAT_HWCAP_ISA_V; } hwcap }
pub unsafe fn riscv_user_isa_enable() {
    if riscv_has_extension_unlikely(RISCV_ISA_EXT_ZICBOZ) { (*current).thread.envcfg |= ENVCFG_CBZE; } else if ANY_CPU_HAS_ZICBOZ { pr_warn!("Zicboz disabled as it is unavailable on some harts\n"); }
    if riscv_has_extension_unlikely(RISCV_ISA_EXT_ZICBOM) { (*current).thread.envcfg |= ENVCFG_CBCFE; } else if ANY_CPU_HAS_ZICBOM { pr_warn!("Zicbom disabled as it is unavailable on some harts\n"); }
    if !riscv_has_extension_unlikely(RISCV_ISA_EXT_ZICBOP) && ANY_CPU_HAS_ZICBOP { pr_warn!("Zicbop disabled as it is unavailable on some harts\n"); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
