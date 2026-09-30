// SPDX-License-Identifier: GPL-2.0-only
//! Architecture-specific state initialized directly by start_kernel.

#[allow(unused_imports)]
use super::bindings;
#[allow(unused_imports)]
use core::ptr;
#[allow(unused_imports)]
use kernel::ffi::{c_int, c_ulong};

#[cfg(not(any(CONFIG_X86_64, CONFIG_ARM64)))]
compile_error!("start_kernel architecture state requires its native implementation");

/// Translate the original per_cpu pointer arithmetic without dereferencing a
/// linker-relative base before applying its configured CPU offset.
#[cfg(CONFIG_USE_PERCPU_NUMA_NODE_ID)]
unsafe fn per_cpu_node(base: *mut c_int, cpu: usize) -> *mut c_int {
    #[cfg(CONFIG_SMP)]
    {
        // SAFETY: the caller supplies a configured CPU and initialized offsets.
        let offset = unsafe {
            ptr::addr_of!(bindings::__per_cpu_offset)
                .cast::<c_ulong>()
                .add(cpu)
                .read()
        };
        return ptr::with_exposed_provenance_mut(
            base.expose_provenance().wrapping_add(offset as usize),
        );
    }
    #[cfg(not(CONFIG_SMP))]
    {
        let _ = cpu;
        base
    }
}

#[cfg(CONFIG_USE_PERCPU_NUMA_NODE_ID)]
unsafe fn initialize_cpu_node(cpu: usize) {
    // SAFETY: per-CPU allocation and the architecture's early mappings have
    // been initialized. The boot caller owns these destination node fields.
    unsafe {
        #[cfg(all(CONFIG_X86_64, CONFIG_NUMA))]
        let node = {
            #[cfg(CONFIG_SMP)]
            let early = bindings::x86_cpu_to_node_map_early_ptr;
            #[cfg(not(CONFIG_SMP))]
            let early = ptr::null_mut::<c_int>();
            if !early.is_null() {
                early.add(cpu).read()
            } else {
                per_cpu_node(ptr::addr_of_mut!(bindings::x86_cpu_to_node_map), cpu).read()
            }
        };
        #[cfg(all(CONFIG_ARM64, CONFIG_NUMA))]
        let node = bindings::early_cpu_to_node(cpu as c_int);
        #[cfg(not(CONFIG_NUMA))]
        let node = 0;
        per_cpu_node(ptr::addr_of_mut!(bindings::numa_node), cpu).write(node);
    }
}

/// Copy every possible CPU's early NUMA node into its allocated per-CPU state.
///
/// # Safety
/// The caller completed setup_per_cpu_areas and the architecture's early CPU
/// mapping. Possible-CPU topology and node storage are serialized at boot.
#[link_section = ".init.text"]
pub(super) unsafe fn early_numa_node_init() {
    #[cfg(CONFIG_USE_PERCPU_NUMA_NODE_ID)]
    // SAFETY: the caller supplies the original early NUMA phase and mappings.
    unsafe {
        // Some architectures override cpu_to_node with a macro. Preserve the
        // actual header's preprocessor gate instead of guessing its presence.
        if bindings::RUST_INIT_MAIN_INITIALIZE_NUMA_NODES == 0 {
            return;
        }
        if bindings::NR_CPUS == 1 {
            initialize_cpu_node(0);
            return;
        }
        let bits = c_ulong::BITS as usize;
        let mask = ptr::addr_of!(bindings::__cpu_possible_mask).cast::<c_ulong>();
        let mut cpu = 0usize;
        loop {
            // small_cpumask_bits is constant only when NR_CPUS fits one word.
            let limit = if bindings::NR_CPUS as usize <= bits {
                bindings::NR_CPUS as usize
            } else {
                bindings::nr_cpu_ids as usize
            };
            if cpu >= limit {
                break;
            }
            if mask.add(cpu / bits).read() & (1 << (cpu % bits)) != 0 {
                initialize_cpu_node(cpu);
            }
            cpu += 1;
        }
    }
}

/// Initialize the boot task's stack guard and ARM64 pointer authentication.
///
/// # Safety
/// This must inline into a function which never returns, with stack protection
/// disabled for that caller. Randomness, current task and per-CPU state must
/// be initialized. ARM64 boot capabilities must have been finalized.
#[inline(always)]
pub(super) unsafe fn boot_init_stack_canary() {
    #[cfg(CONFIG_STACKPROTECTOR)]
    // SAFETY: the caller owns boot task/per-CPU guards and initialized RNG.
    unsafe {
        let canary =
            bindings::get_random_u64() as c_ulong & bindings::RUST_INIT_MAIN_CANARY_MASK as c_ulong;
        let current = kernel::bindings::get_current().cast::<bindings::task_struct>();
        ptr::addr_of_mut!((*current).stack_canary).write(canary);
        #[cfg(CONFIG_X86_64)]
        {
            #[cfg(CONFIG_SMP)]
            core::arch::asm!("mov gs:[{guard}], {canary}", guard = sym bindings::__stack_chk_guard,
                canary = in(reg) canary, options(nostack, preserves_flags));
            #[cfg(not(CONFIG_SMP))]
            core::arch::asm!("mov [{guard}], {canary}", guard = sym bindings::__stack_chk_guard,
                canary = in(reg) canary, options(nostack, preserves_flags));
        }
        #[cfg(all(CONFIG_ARM64, not(CONFIG_STACKPROTECTOR_PER_TASK)))]
        {
            bindings::__stack_chk_guard = ptr::addr_of!((*current).stack_canary).read();
        }
    }

    #[cfg(CONFIG_ARM64_PTR_AUTH_KERNEL)]
    // SAFETY: retain each original capability query and the two key register
    // writes followed by ISB. Key generation precedes installing either half.
    unsafe {
        if bindings::rust_helper_system_supports_address_auth() {
            let current = kernel::bindings::get_current().cast::<bindings::task_struct>();
            let key = ptr::addr_of_mut!((*current).thread.keys_kernel.apia);
            bindings::get_random_bytes(key.cast(), core::mem::size_of::<bindings::ptrauth_key>());
        }
        if bindings::rust_helper_system_supports_address_auth() {
            let current = kernel::bindings::get_current().cast::<bindings::task_struct>();
            let key = ptr::addr_of!((*current).thread.keys_kernel.apia);
            let low = ptr::addr_of!((*key).lo).read();
            let high = ptr::addr_of!((*key).hi).read();
            core::arch::asm!("msr S3_0_C2_C1_0, {low}", "msr S3_0_C2_C1_1, {high}", "isb",
                low = in(reg) low, high = in(reg) high, options(nostack, preserves_flags));
        }
    }
    #[cfg(CONFIG_ARM64_PTR_AUTH)]
    // SAFETY: ptrauth_enable updates SCTLR only if the enable bits change,
    // then always issues ISB on the supported-capability path.
    unsafe {
        if bindings::rust_helper_system_supports_address_auth() {
            let old: c_ulong;
            core::arch::asm!("mrs {old}, sctlr_el1", old = out(reg) old, options(nostack, preserves_flags));
            let new = old | bindings::RUST_INIT_MAIN_PTRAUTH_ENABLE_BITS as c_ulong;
            if new != old {
                core::arch::asm!("msr sctlr_el1, {new}", new = in(reg) new, options(nostack, preserves_flags));
            }
            core::arch::asm!("isb", options(nostack, preserves_flags));
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
