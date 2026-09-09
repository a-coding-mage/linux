// Translated from the AArch64 assembly linkage header.
// The assembler-only include and preprocessor header guard have no Rust equivalent.

// `CONFIG_FUNCTION_ALIGNMENT` is a build-time assembler configuration value.
// These macros preserve the corresponding alignment directive for assembly-oriented callers.
macro_rules! __ALIGN {
    () => { ".balign CONFIG_FUNCTION_ALIGNMENT" };
}

macro_rules! __ALIGN_STR {
    () => { ".balign " /* CONFIG_FUNCTION_ALIGNMENT */ };
}

/*
 * When using in-kernel BTI we need to ensure that PCS-conformant
 * assembly functions have suitable annotations.  Override
 * SYM_FUNC_START to insert a BTI landing pad at the start of
 * everything, the override is done unconditionally so we're more
 * likely to notice any drift from the overridden definitions.
 */
macro_rules! SYM_FUNC_START {
    ($name:expr) => {
        SYM_START!($name, SYM_L_GLOBAL, SYM_A_ALIGN);
        ::core::arch::asm!("bti c");
    };
}

macro_rules! SYM_FUNC_START_NOALIGN {
    ($name:expr) => {
        SYM_START!($name, SYM_L_GLOBAL, SYM_A_NONE);
        ::core::arch::asm!("bti c");
    };
}

macro_rules! SYM_FUNC_START_LOCAL {
    ($name:expr) => {
        SYM_START!($name, SYM_L_LOCAL, SYM_A_ALIGN);
        ::core::arch::asm!("bti c");
    };
}

macro_rules! SYM_FUNC_START_LOCAL_NOALIGN {
    ($name:expr) => {
        SYM_START!($name, SYM_L_LOCAL, SYM_A_NONE);
        ::core::arch::asm!("bti c");
    };
}

macro_rules! SYM_FUNC_START_WEAK {
    ($name:expr) => {
        SYM_START!($name, SYM_L_WEAK, SYM_A_ALIGN);
        ::core::arch::asm!("bti c");
    };
}

macro_rules! SYM_FUNC_START_WEAK_NOALIGN {
    ($name:expr) => {
        SYM_START!($name, SYM_L_WEAK, SYM_A_NONE);
        ::core::arch::asm!("bti c");
    };
}

macro_rules! SYM_TYPED_FUNC_START {
    ($name:expr) => {
        SYM_TYPED_START!($name, SYM_L_GLOBAL, SYM_A_ALIGN);
        ::core::arch::asm!("bti c");
    };
}

// Equivalent to the C statement expression obtaining the current instruction address.
#[inline(always)]
pub unsafe fn _THIS_IP_() -> usize {
    let ip: usize;
    ::core::arch::asm!("adr {0}, .", out(reg) ip);
    ip
}

// `__section(".bss..pgtbl") __aligned(PAGE_SIZE)` is a build-time attribute
// combination; apply the equivalent attributes to the declaration using it.
macro_rules! __bss_pgtbl {
    ($item:item) => {
        #[link_section = ".bss..pgtbl"]
        $item
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
