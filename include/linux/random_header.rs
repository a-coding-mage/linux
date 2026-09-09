/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

extern "C" {
    pub fn add_device_randomness(buf: *const core::ffi::c_void, len: usize);
    pub fn add_bootloader_randomness(buf: *const core::ffi::c_void, len: usize);
    pub fn add_input_randomness(type_: u32, code: u32, value: u32);
    pub fn add_interrupt_randomness(irq: i32);
    pub fn add_hwgenerator_randomness(
        buf: *const core::ffi::c_void,
        len: usize,
        entropy: usize,
        sleep_after: bool,
    );

    pub fn add_vmfork_randomness(unique_vm_id: *const core::ffi::c_void, len: usize);
    pub fn register_random_vmfork_notifier(nb: *mut notifier_block) -> i32;
    pub fn unregister_random_vmfork_notifier(nb: *mut notifier_block) -> i32;

    pub fn get_random_bytes(buf: *mut core::ffi::c_void, len: usize);
    pub fn get_random_u8() -> u8;
    pub fn get_random_u16() -> u16;
    pub fn get_random_u32() -> u32;
    pub fn get_random_u64() -> u64;
    pub fn __get_random_u32_below(ceil: u32) -> u32;

    pub fn random_init_early(command_line: *const i8);
    pub fn random_init();
    pub fn rng_is_initialized() -> bool;
    pub fn wait_for_random_bytes() -> i32;
    pub fn execute_with_initialized_rng(nb: *mut notifier_block) -> i32;

    // CONFIG_SMP
    pub fn random_prepare_cpu(cpu: u32) -> i32;
    pub fn random_online_cpu(cpu: u32) -> i32;
}

#[repr(C)]
pub struct file_operations {
    _private: [u8; 0],
}

extern "C" {
    // MODULE is not defined: these globals are absent for module builds.
    pub static random_fops: file_operations;
    pub static urandom_fops: file_operations;
}

#[inline]
pub unsafe fn add_latent_entropy() {
    // LATENT_ENTROPY_PLUGIN and __CHECKER__ are build-time conditions.
    add_device_randomness(core::ptr::null(), 0);
}

#[inline]
pub unsafe fn get_random_long() -> usize {
    // BITS_PER_LONG == 64 selects get_random_u64(); otherwise get_random_u32().
    if core::mem::size_of::<usize>() == 8 {
        get_random_u64() as usize
    } else {
        get_random_u32() as usize
    }
}

#[inline]
pub unsafe fn get_random_u32_below(ceil: u32) -> u32 {
    // __builtin_constant_p(ceil) and BUILD_BUG_ON_MSG are compiler/build-time checks.
    if ceil <= 1 {
        return 0;
    }
    loop {
        if ceil <= 1u32 << 8 {
            let mult = ceil * get_random_u8() as u32;
            if ceil.is_power_of_two() || (mult as u8 as u32) >= (1u32 << 8) % ceil {
                return mult >> 8;
            }
        } else if ceil <= 1u32 << 16 {
            let mult = ceil * get_random_u16() as u32;
            if ceil.is_power_of_two() || (mult as u16 as u32) >= (1u32 << 16) % ceil {
                return mult >> 16;
            }
        } else {
            let mult = (ceil as u64) * get_random_u32() as u64;
            if ceil.is_power_of_two() || (mult as u32) >= (0u32.wrapping_sub(ceil)) % ceil {
                return (mult >> 32) as u32;
            }
        }
    }
}

#[inline]
pub unsafe fn get_random_u32_above(floor: u32) -> u32 {
    // BUILD_BUG_ON_MSG(__builtin_constant_p(floor) && floor == U32_MAX, ...).
    floor.wrapping_add(1).wrapping_add(get_random_u32_below(u32::MAX - floor))
}

#[inline]
pub unsafe fn get_random_u32_inclusive(floor: u32, ceil: u32) -> u32 {
    // BUILD_BUG_ON_MSG on constant ordering/range is a compiler/build-time check.
    floor.wrapping_add(get_random_u32_below(ceil.wrapping_sub(floor).wrapping_add(1)))
}

#[inline]
pub unsafe fn get_random_bytes_wait(buf: *mut core::ffi::c_void, nbytes: usize) -> i32 {
    let ret = wait_for_random_bytes();
    get_random_bytes(buf, nbytes);
    ret
}

// CONFIG_VMGENID controls whether the vmfork declarations above are provided;
// when disabled, the notifier functions are inline no-op returns of zero.
// CONFIG_SMP controls random_prepare_cpu/random_online_cpu declarations.
// MODULE controls the random_fops and urandom_fops declarations.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
