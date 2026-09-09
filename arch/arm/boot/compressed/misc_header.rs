// Translated from misc.h. The C compiler dependency <linux/compiler.h>
// supplies the __noreturn annotation.

extern "C" {
    pub fn error(x: *mut core::ffi::c_char) -> !;

    pub static mut free_mem_ptr: usize;
    pub static mut free_mem_end_ptr: usize;

    pub fn __div0();

    pub fn decompress_kernel(
        output_start: usize,
        free_mem_ptr_p: usize,
        free_mem_ptr_end_p: usize,
        arch_id: core::ffi::c_int,
    );

    pub fn __fortify_panic(reason: u8, avail: usize, size: usize);

    pub fn atags_to_fdt(
        atag_list: *mut core::ffi::c_void,
        fdt: *mut core::ffi::c_void,
        total_space: core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub fn fdt_check_mem_start(
        mem_start: u32,
        fdt: *const core::ffi::c_void,
    ) -> u32;

    pub fn do_decompress(
        input: *mut u8,
        len: core::ffi::c_int,
        output: *mut u8,
        error: Option<unsafe extern "C" fn(x: *mut core::ffi::c_char)>,
    ) -> core::ffi::c_int;

    pub static mut input_data: [core::ffi::c_char; 0];
    pub static mut input_data_end: [core::ffi::c_char; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
