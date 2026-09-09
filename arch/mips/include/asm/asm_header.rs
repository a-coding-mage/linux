/* Rust translation of asm.h; assembler/header dependencies remain external. */
#[cfg(not(feature = "vdso"))] pub const CFI_SECTIONS: &str = ".cfi_sections .debug_frame";
#[cfg(feature = "vdso")] pub const CFI_SECTIONS: &str = "";
#[macro_export] macro_rules! LEAF { ($s:ident) => { stringify!($s) }; }
#[macro_export] macro_rules! NESTED { ($s:ident, $f:expr, $r:ident) => { stringify!($s) }; }
#[macro_export] macro_rules! END { ($f:ident) => { stringify!($f) }; }
#[macro_export] macro_rules! EXPORT { ($s:ident) => { stringify!($s) }; }
#[macro_export] macro_rules! FEXPORT { ($s:ident) => { stringify!($s) }; }
#[macro_export] macro_rules! ABS { ($s:ident, $v:expr) => { stringify!($s) }; }
#[macro_export] macro_rules! TEXT { ($m:expr) => { $m }; }
#[macro_export] macro_rules! ASM_PANIC { ($m:expr) => { $m }; }
#[cfg(feature = "config_printk")] #[macro_export] macro_rules! ASM_PRINT { ($s:expr) => { $s }; }
#[cfg(not(feature = "config_printk"))] #[macro_export] macro_rules! ASM_PRINT { ($s:expr) => {}; }
#[cfg(feature = "mips_abi32")] pub const ALSZ:i32=7;
#[cfg(feature = "mips_abi32")] pub const ALMASK:i32=!7;
#[cfg(any(feature="mips_nabi32",feature="mips_abi64"))] pub const ALSZ:i32=15;
#[cfg(any(feature="mips_nabi32",feature="mips_abi64"))] pub const ALMASK:i32=!15;
#[cfg(feature="mips64")] pub const SZREG:usize=8;
#[cfg(not(feature="mips64"))] pub const SZREG:usize=4;
#[cfg(feature="mips_abi32")] pub const REG_S:&str="sw";
#[cfg(feature="mips_abi32")] pub const REG_L:&str="lw";
#[cfg(feature="mips_abi32")] pub const REG_SUBU:&str="subu";
#[cfg(feature="mips_abi32")] pub const REG_ADDU:&str="addu";
#[cfg(any(feature="mips_nabi32",feature="mips_abi64"))] pub const REG_S:&str="sd";
#[cfg(any(feature="mips_nabi32",feature="mips_abi64"))] pub const REG_L:&str="ld";
#[cfg(any(feature="mips_nabi32",feature="mips_abi64"))] pub const REG_SUBU:&str="dsubu";
#[cfg(any(feature="mips_nabi32",feature="mips_abi64"))] pub const REG_ADDU:&str="daddu";
#[cfg(feature="mips_int32")] pub mod int_ops { pub const ADD:&str="add"; pub const ADDU:&str="addu"; pub const ADDI:&str="addi"; pub const ADDIU:&str="addiu"; pub const SUB:&str="sub"; pub const SUBU:&str="subu"; pub const L:&str="lw"; pub const S:&str="sw"; pub const SLL:&str="sll"; pub const SLLV:&str="sllv"; pub const SRL:&str="srl"; pub const SRLV:&str="srlv"; pub const SRA:&str="sra"; pub const SRAV:&str="srav"; }
#[cfg(feature="mips_int64")] pub mod int_ops { pub const ADD:&str="dadd"; pub const ADDU:&str="daddu"; pub const ADDI:&str="daddi"; pub const ADDIU:&str="daddiu"; pub const SUB:&str="dsub"; pub const SUBU:&str="dsubu"; pub const L:&str="ld"; pub const S:&str="sd"; pub const SLL:&str="dsll"; pub const SLLV:&str="dsllv"; pub const SRL:&str="dsrl"; pub const SRLV:&str="dsrlv"; pub const SRA:&str="dsra"; pub const SRAV:&str="dsrav"; }
#[cfg(feature="mips_long32")] pub const LONGSIZE:usize=4; #[cfg(feature="mips_long32")] pub const LONGMASK:usize=3; #[cfg(feature="mips_long32")] pub const LONGLOG:usize=2;
#[cfg(feature="mips_long64")] pub const LONGSIZE:usize=8; #[cfg(feature="mips_long64")] pub const LONGMASK:usize=7; #[cfg(feature="mips_long64")] pub const LONGLOG:usize=3;
#[cfg(feature="mips_ptr32")] pub const PTR_SCALESHIFT:usize=2; #[cfg(feature="mips_ptr32")] pub const PTRSIZE:usize=4; #[cfg(feature="mips_ptr32")] pub const PTRLOG:usize=2;
#[cfg(feature="mips_ptr64")] pub const PTR_SCALESHIFT:usize=3; #[cfg(feature="mips_ptr64")] pub const PTRSIZE:usize=8; #[cfg(feature="mips_ptr64")] pub const PTRLOG:usize=3;
#[cfg(feature="mips_abi32")] pub const MFC0:&str="mfc0"; #[cfg(feature="mips_abi32")] pub const MTC0:&str="mtc0";
#[cfg(any(feature="mips_nabi32",feature="mips_abi64"))] pub const MFC0:&str="dmfc0"; #[cfg(any(feature="mips_nabi32",feature="mips_abi64"))] pub const MTC0:&str="dmtc0";
pub const SSNOP:&str="sll zero, zero, 1";
#[cfg(feature="config_war_r10000_llsc")] pub const SC_BEQZ:&str="beqzl";
#[cfg(all(not(feature="config_war_r10000_llsc"),feature="mips_isa_rev6",not(feature="config_cc_has_broken_inline_compat_branch")))] pub const SC_BEQZ:&str="beqzc";
#[cfg(all(not(feature="config_war_r10000_llsc"),not(all(feature="mips_isa_rev6",not(feature="config_cc_has_broken_inline_compat_branch")))))] pub const SC_BEQZ:&str="beqz";
#[cfg(feature="config_sgi_ip28")] #[macro_export] macro_rules! R10KCBARRIER { ($a:expr) => { stringify!($a) }; }
#[cfg(not(feature="config_sgi_ip28"))] #[macro_export] macro_rules! R10KCBARRIER { ($a:expr) => {}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
