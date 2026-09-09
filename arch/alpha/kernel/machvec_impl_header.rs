/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Translation of linux/arch/alpha/kernel/machvec_impl.h.
 * C preprocessor configuration and token-pasting are preserved below as
 * Rust macro-level intent; referenced machine-vector symbols are external.
 */

// Systems without an HAE use the machine vector's cache field as its address.
pub const IRONGATE_HAE_ADDRESS: *mut _ = unsafe { &raw mut alpha_mv.hae_cache };
pub const MARVEL_HAE_ADDRESS: *mut _ = unsafe { &raw mut alpha_mv.hae_cache };
pub const POLARIS_HAE_ADDRESS: *mut _ = unsafe { &raw mut alpha_mv.hae_cache };
pub const TSUNAMI_HAE_ADDRESS: *mut _ = unsafe { &raw mut alpha_mv.hae_cache };
pub const TITAN_HAE_ADDRESS: *mut _ = unsafe { &raw mut alpha_mv.hae_cache };
pub const WILDFIRE_HAE_ADDRESS: *mut _ = unsafe { &raw mut alpha_mv.hae_cache };

// These are enabled by the corresponding build-time configuration symbols.
#[cfg(CIA_ONE_HAE_WINDOW)]
pub const CIA_HAE_ADDRESS: *mut _ = unsafe { &raw mut alpha_mv.hae_cache };
#[cfg(MCPCIA_ONE_HAE_WINDOW)]
pub const MCPCIA_HAE_ADDRESS: *mut _ = unsafe { &raw mut alpha_mv.hae_cache };
#[cfg(T2_ONE_HAE_WINDOW)]
pub const T2_HAE_ADDRESS: *mut _ = unsafe { &raw mut alpha_mv.hae_cache };

pub const JENSEN_IACK_SC: i32 = 1;
pub const T2_IACK_SC: i32 = 1;
pub const WILDFIRE_IACK_SC: i32 = 1; // FIXME

pub const DO_DEFAULT_RTC: u32 = 0x70;

// Build-time constants supplied by the surrounding translation unit.
macro_rules! DO_EV5_MMU { () => { .max_asn = EV5_MAX_ASN }; }
macro_rules! DO_EV6_MMU { () => { .max_asn = EV6_MAX_ASN }; }
macro_rules! DO_EV7_MMU { () => { .max_asn = EV6_MAX_ASN }; }

// C token-pasting (CAT/CAT1) has no stable direct Rust equivalent; callers
// must provide the already-resolved identifiers when expanding these forms.
macro_rules! CAT1 { ($x:ident, $y:ident) => { $x $y }; }
macro_rules! CAT { ($x:ident, $y:ident) => { CAT1!($x, $y) }; }

// The following initializer macros retain the original field ordering and
// external symbol references. Their arguments are expected to resolve names
// in the same way as the C CAT(UP, suffix) and CAT(low, suffix) expressions.
macro_rules! IO_LITE {
    ($up:ident, $low:ident) => {
        .hae_register = CAT!($up, _HAE_ADDRESS),
        .iack_sc = CAT!($up, _IACK_SC),
        .mv_ioread8 = CAT!($low, _ioread8),
        .mv_ioread16 = CAT!($low, _ioread16),
        .mv_ioread32 = CAT!($low, _ioread32),
        .mv_ioread64 = CAT!($low, _ioread64),
        .mv_iowrite8 = CAT!($low, _iowrite8),
        .mv_iowrite16 = CAT!($low, _iowrite16),
        .mv_iowrite32 = CAT!($low, _iowrite32),
        .mv_iowrite64 = CAT!($low, _iowrite64),
        .mv_readb = CAT!($low, _readb), .mv_readw = CAT!($low, _readw),
        .mv_readl = CAT!($low, _readl), .mv_readq = CAT!($low, _readq),
        .mv_writeb = CAT!($low, _writeb), .mv_writew = CAT!($low, _writew),
        .mv_writel = CAT!($low, _writel), .mv_writeq = CAT!($low, _writeq),
        .mv_ioportmap = CAT!($low, _ioportmap), .mv_ioremap = CAT!($low, _ioremap),
        .mv_iounmap = CAT!($low, _iounmap), .mv_is_ioaddr = CAT!($low, _is_ioaddr),
        .mv_is_mmio = CAT!($low, _is_mmio)
    };
}
macro_rules! IO { ($up:ident, $low:ident) => {
    IO_LITE!($up, $low), .pci_ops = &CAT!($low, _pci_ops),
    .mv_pci_tbi = CAT!($low, _pci_tbi)
}; }

macro_rules! DO_APECS_IO { () => { IO!(APECS, apecs) }; }
macro_rules! DO_CIA_IO { () => { IO!(CIA, cia) }; }
macro_rules! DO_IRONGATE_IO { () => { IO!(IRONGATE, irongate) }; }
macro_rules! DO_LCA_IO { () => { IO!(LCA, lca) }; }
macro_rules! DO_MARVEL_IO { () => { IO!(MARVEL, marvel) }; }
macro_rules! DO_MCPCIA_IO { () => { IO!(MCPCIA, mcpcia) }; }
macro_rules! DO_POLARIS_IO { () => { IO!(POLARIS, polaris) }; }
macro_rules! DO_T2_IO { () => { IO!(T2, t2) }; }
macro_rules! DO_TSUNAMI_IO { () => { IO!(TSUNAMI, tsunami) }; }
macro_rules! DO_TITAN_IO { () => { IO!(TITAN, titan) }; }
macro_rules! DO_WILDFIRE_IO { () => { IO!(WILDFIRE, wildfire) }; }
macro_rules! DO_PYXIS_IO { () => {
    IO_LITE!(CIA, cia_bwx), .pci_ops = &cia_pci_ops, .mv_pci_tbi = cia_pci_tbi
}; }

// CONFIG_ALPHA_GENERIC selects init-data placement and copy/alias behavior.
#[cfg(CONFIG_ALPHA_GENERIC)]
macro_rules! __initmv { () => { __initdata }; }
#[cfg(CONFIG_ALPHA_GENERIC)]
macro_rules! ALIAS_MV { ($x:ident) => {}; }
#[cfg(not(CONFIG_ALPHA_GENERIC))]
macro_rules! __initmv { () => { __refdata }; }
#[cfg(not(CONFIG_ALPHA_GENERIC))]
macro_rules! ALIAS_MV { ($system:ident) => {
    // C: asm(".global alpha_mv\nalpha_mv = " #system "_mv"); EXPORT_SYMBOL(alpha_mv);
}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
