// SPDX-License-Identifier: GPL-2.0
/*
 * sleep.c - x86-specific ACPI sleep support.
 *
 *  Copyright (C) 2001-2003 Patrick Mochel
 *  Copyright (C) 2001-2003 Pavel Machek <pavel@ucw.cz>
 */

// Dependencies supplied by the surrounding kernel translation.

pub static mut acpi_realmode_flags: ::core::ffi::c_ulong = 0;

#[cfg(all(feature = "CONFIG_SMP", feature = "CONFIG_64BIT"))]
static mut temp_stack: [::core::ffi::c_char; 4096] = [0; 4096];

/**
 * acpi_get_wakeup_address - provide physical address for S3 wakeup
 *
 * Returns the physical address where the kernel should be resumed after the
 * system awakes from S3, e.g. for programming into the firmware waking vector.
 */
pub unsafe extern "C" fn acpi_get_wakeup_address() -> ::core::ffi::c_ulong {
	return (real_mode_header.wakeup_start as ::core::ffi::c_ulong);
}

/**
 * x86_acpi_enter_sleep_state - enter sleep state
 * @state: Sleep state to enter.
 *
 * Wrapper around acpi_enter_sleep_state() to be called by assembly.
 */
pub unsafe extern "C" fn x86_acpi_enter_sleep_state(
	state: u8,
) -> acpi_status {
	return acpi_enter_sleep_state(state);
}

/**
 * x86_acpi_suspend_lowlevel - save kernel state
 *
 * Create an identity mapped page table and copy the wakeup routine to
 * low memory.
 */
pub unsafe extern "C" fn x86_acpi_suspend_lowlevel() -> ::core::ffi::c_int {
	let header = &mut *((__va(real_mode_header.wakeup_header)) as *mut wakeup_header);
	let mut val: msr = ::core::mem::zeroed();

	if header.signature != WAKEUP_HEADER_SIGNATURE {
		printk(KERN_ERR, "wakeup header does not match\n");
		return -EINVAL;
	}

	header.video_mode = saved_video_mode;
	header.pmode_behavior = 0;

#[cfg(not(feature = "CONFIG_64BIT"))]
	{
		native_store_gdt((&mut header.pmode_gdt) as *mut _ as *mut desc_ptr);

		/*
		 * We have to check that we can write back the value, and not
		 * just read it.  At least on 90 nm Pentium M (Family 6, Model
		 * 13), reading an invalid MSR is not guaranteed to trap, see
		 * Erratum X4 in "Intel Pentium M Processor on 90 nm Process
		 * with 2-MB L2 Cache and Intel® Processor A100 and A110 on 90
		 * nm process with 512-KB L2 Cache Specification Update".
		 */
		if !rdmsrq_safe(MSR_EFER, &mut val.q) && !wrmsrq_safe(MSR_EFER, val.q) {
			header.pmode_behavior |= 1 << WAKEUP_BEHAVIOR_RESTORE_EFER;
		}
		header.pmode_efer_low = val.l;
		header.pmode_efer_high = val.h;
	}

	header.pmode_cr0 = read_cr0();
	if __this_cpu_read(cpu_info.cpuid_level) >= 0 {
		header.pmode_cr4 = __read_cr4();
		header.pmode_behavior |= 1 << WAKEUP_BEHAVIOR_RESTORE_CR4;
	}
	if !rdmsrq_safe(MSR_IA32_MISC_ENABLE, &mut val.q)
		&& !wrmsrq_safe(MSR_IA32_MISC_ENABLE, val.q)
	{
		header.pmode_behavior |= 1 << WAKEUP_BEHAVIOR_RESTORE_MISC_ENABLE;
	}
	header.pmode_misc_en_low = val.l;
	header.pmode_misc_en_high = val.h;
	header.realmode_flags = acpi_realmode_flags;
	header.real_magic = 0x12345678;

#[cfg(not(feature = "CONFIG_64BIT"))]
	{
		header.pmode_entry = &wakeup_pmode_return as *const _ as u32;
		header.pmode_cr3 = __pa_symbol(initial_page_table) as u32;
		saved_magic = 0x12345678;
	}

#[cfg(feature = "CONFIG_64BIT")]
	{
#[cfg(feature = "CONFIG_SMP")]
		{
			current.thread.sp = temp_stack.as_mut_ptr() as ::core::ffi::c_ulong
				+ ::core::mem::size_of_val(&temp_stack) as ::core::ffi::c_ulong;
			if (smpboot_control & STARTUP_PARALLEL_MASK) == 0 {
				smpboot_control = smp_processor_id();
			}
		}
		initial_code = wakeup_long64 as ::core::ffi::c_ulong;
		saved_magic = 0x123456789abcdef0u64 as _;
	}

	/*
	 * Pause/unpause graph tracing around do_suspend_lowlevel as it has
	 * inconsistent call/return info after it jumps to the wakeup vector.
	 */
	pause_graph_tracing();
	do_suspend_lowlevel();
	unpause_graph_tracing();
	0
}

unsafe extern "C" fn acpi_sleep_setup(mut str_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
	while !str_.is_null() && *str_ != 0 {
		if strncmp(str_, c"s3_bios".as_ptr(), 7) == 0 { acpi_realmode_flags |= 1; }
		if strncmp(str_, c"s3_mode".as_ptr(), 7) == 0 { acpi_realmode_flags |= 2; }
		if strncmp(str_, c"s3_beep".as_ptr(), 7) == 0 { acpi_realmode_flags |= 4; }
#[cfg(feature = "CONFIG_HIBERNATION")]
		if strncmp(str_, c"s4_hwsig".as_ptr(), 8) == 0 { acpi_check_s4_hw_signature = 1; }
#[cfg(feature = "CONFIG_HIBERNATION")]
		if strncmp(str_, c"s4_nohwsig".as_ptr(), 10) == 0 { acpi_check_s4_hw_signature = 0; }
		if strncmp(str_, c"nonvs".as_ptr(), 5) == 0 { acpi_nvs_nosave(); }
		if strncmp(str_, c"nonvs_s3".as_ptr(), 8) == 0 { acpi_nvs_nosave_s3(); }
		if strncmp(str_, c"old_ordering".as_ptr(), 12) == 0 { acpi_old_suspend_ordering(); }
		if strncmp(str_, c"nobl".as_ptr(), 4) == 0 { acpi_sleep_no_blacklist(); }
		str_ = strchr(str_, b',' as i32);
		if !str_.is_null() { str_ = str_.add(strspn(str_, c", \t".as_ptr())); }
	}
	1
}

// __setup("acpi_sleep=", acpi_sleep_setup);

#[cfg(all(feature = "CONFIG_HIBERNATION", feature = "CONFIG_HYPERVISOR_GUEST"))]
unsafe extern "C" fn init_s4_sigcheck() -> ::core::ffi::c_int {
	/*
	 * If running on a hypervisor, honour the ACPI specification
	 * by default and trigger a clean reboot when the hardware
	 * signature in FACS is changed after hibernation.
	 */
	if acpi_check_s4_hw_signature == -1 && !hypervisor_is_type(X86_HYPER_NATIVE) {
		acpi_check_s4_hw_signature = 1;
	}
	0
}

// arch_initcall(init_s4_sigcheck);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
