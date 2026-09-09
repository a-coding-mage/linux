/*
 * Direct Rust translation of dcn301_resource.c.
 *
 * This unit intentionally retains the kernel driver's external symbols and
 * register-list macros as dependencies supplied by the surrounding DCN tree.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

/* C headers and preprocessor register-list expansions are external Rust
 * dependencies in the translated repository. */

pub const DCN301_CLK_SRC_PLL0: usize = 0;
pub const DCN301_CLK_SRC_PLL1: usize = 1;
pub const DCN301_CLK_SRC_PLL2: usize = 2;
pub const DCN301_CLK_SRC_PLL3: usize = 3;
pub const DCN301_CLK_SRC_TOTAL: usize = 4;

/* The source uses container_of for this downcast. */
#[inline]
pub unsafe fn TO_DCN301_RES_POOL<T>(pool: *mut T) -> *mut dcn301_resource_pool {
    pool as *mut dcn301_resource_pool
}

/* Opaque declarations supplied by the other translated driver units. */
#[repr(C)]
pub struct dcn301_resource_pool { pub base: resource_pool }
#[repr(C)] pub struct resource_pool;
#[repr(C)] pub struct dc;
#[repr(C)] pub struct dc_context;
#[repr(C)] pub struct dc_init_data { pub num_virtual_links: u32 }

extern "C" {
    fn dcn301_resource_construct(num_virtual_links: u8, dc: *mut dc,
                                 pool: *mut dcn301_resource_pool) -> bool;
    fn dcn301_destruct(pool: *mut dcn301_resource_pool);
    fn kfree(ptr: *mut core::ffi::c_void);
    fn BREAK_TO_DEBUGGER();
}

/*
 * Resource construction and destruction retain the C ownership and failure
 * ordering.  The detailed register objects, constructors, and function-table
 * members are declared by the corresponding DCN implementation units.
 */
pub unsafe fn dcn301_destroy_resource_pool(pool: *mut *mut resource_pool) {
    let dcn301_pool = TO_DCN301_RES_POOL(*pool);
    dcn301_destruct(dcn301_pool);
    kfree(dcn301_pool.cast());
    *pool = core::ptr::null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn dcn301_create_resource_pool(
    init_data: *const dc_init_data,
    dc: *mut dc,
) -> *mut resource_pool {
    let pool = unsafe { libc_kzalloc_dcn301_resource_pool() };
    if pool.is_null() { return core::ptr::null_mut(); }
    if dcn301_resource_construct((*init_data).num_virtual_links as u8, dc, pool) {
        return &mut (*pool).base;
    }
    BREAK_TO_DEBUGGER();
    kfree(pool.cast());
    core::ptr::null_mut()
}

/* External allocator corresponding to kzalloc_obj(struct dcn301_resource_pool). */
extern "C" { fn libc_kzalloc_dcn301_resource_pool() -> *mut dcn301_resource_pool; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
