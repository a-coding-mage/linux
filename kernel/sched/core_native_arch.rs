// SPDX-License-Identifier: GPL-2.0-only
// arch/x86/include/asm/linkage.h:16 uses LEA from RIP for _THIS_IP_.
// Expand in the Rust caller: no intermediary native frame or static marker.
#[cfg(CONFIG_X86_64)]
macro_rules! lupos_core_this_ip {
    () => {{
        let ip: c_ulong;
        core::arch::asm!(
            "lea {ip}, [rip]",
            ip = out(reg) ip,
            options(nomem, nostack, preserves_flags),
        );
        ip
    }};
}
#[cfg(not(CONFIG_X86_64))]
macro_rules! lupos_core_this_ip {
    () => {{
        // No replacement of an architecture-specific IP macro with a guess.
        compile_error!("scheduler core requires source-derived architecture _THIS_IP_ lowering")
    }};
}
