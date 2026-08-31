// SPDX-License-Identifier: GPL-2.0
// Converted from tools/testing/selftests/bpf/verifier/helper_value_access.c

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::arch::asm;

type __u32 = u32;
type __s32 = i32;
type __u64 = u64;
type __s64 = i64;

const BPF_MAP_TYPE_HASH: u32 = 1;
pub const MAX_ENTRIES: usize = 11;

#[repr(C)]
pub struct other_val {
    pub foo: i64,
    pub bar: i64,
}

#[repr(C)]
pub struct test_val {
    pub index: u32,
    pub foo: [i32; MAX_ENTRIES],
}

// Original C map definitions used libbpf macros:
//   type = BPF_MAP_TYPE_HASH, max_entries = 1, key = long long
//   value = struct other_val / struct test_val / long long
extern "C" {
    static mut map_hash_16b: u8;
    static mut map_hash_48b: u8;
    static mut map_hash_8b: u8;

    fn bpf_map_lookup_elem(map: *mut u8, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bpf_map_update_elem(map: *mut u8, key: *const core::ffi::c_void, value: *const core::ffi::c_void, flags: u64) -> i64;
    fn bpf_probe_read_kernel(dst: *mut core::ffi::c_void, size: u32, src: *const core::ffi::c_void) -> i64;
    fn bpf_trace_printk(fmt: *const i8, fmt_size: u32, ...) -> i64;
}

const SIZEOF_TEST_VAL: usize = core::mem::size_of::<test_val>();
const SIZEOF_OTHER_VAL: usize = core::mem::size_of::<other_val>();
const TEST_VAL_FOO: usize = core::mem::offset_of!(test_val, foo);
const OTHER_VAL_BAR: usize = core::mem::offset_of!(other_val, bar);

macro_rules! bpf_asm_prog {
    ($name:ident, $asm_body:literal) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name() {
            unsafe {
                asm!(
                    $asm_body,
                    options(noreturn)
                );
            }
        }
    };
}

// SEC("tracepoint")
// __description("helper access to map: full range")
// __success
// operands: __imm(bpf_map_lookup_elem), __imm(bpf_probe_read_kernel), __imm_addr(map_hash_48b), __imm_const(sizeof_test_val, sizeof(struct test_val))
bpf_asm_prog!(access_to_map_full_range, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r2 = SIZEOF_TEST_VAL;            
    r3 = 0;                        
    call bpf_probe_read_kernel;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("helper access to map: partial range")
// __success
// operands: __imm(bpf_map_lookup_elem), __imm(bpf_probe_read_kernel), __imm_addr(map_hash_48b)
bpf_asm_prog!(access_to_map_partial_range, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r2 = 8;                        
    r3 = 0;                        
    call bpf_probe_read_kernel;            
l0_%=:    exit;                        \
"#);


/* Call a function taking a pointer and a size which doesn't allow the size to
 * be zero (i.e. bpf_trace_printk() declares the second argument to be
 * ARG_MEM_SIZE, not ARG_MEM_SIZE_OR_ZERO). We attempt to pass zero for the
 * size and expect to fail.
 */
// SEC("tracepoint")
// __description("helper access to map: empty range")
// __failure __msg("R2 invalid zero-sized read: u64=[0,0]")
// operands: __imm(bpf_map_lookup_elem), __imm(bpf_trace_printk), __imm_addr(map_hash_48b)
bpf_asm_prog!(access_to_map_empty_range, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r2 = 0;                        
    call bpf_trace_printk;            
l0_%=:    exit;                        \
"#);


/* Like the test above, but this time the size register is not known to be zero;
 * its lower-bound is zero though, which is still unacceptable.
 */
// SEC("tracepoint")
// __description("helper access to map: possibly-empty ange")
// __failure __msg("R2 invalid zero-sized read: u64=[0,4]")
// operands: __imm(bpf_map_lookup_elem), __imm(bpf_trace_printk), __imm_addr(map_hash_48b)
bpf_asm_prog!(access_to_map_possibly_empty_range, r#"
    r2 = r10;                                               
    r2 += -8;                                               
    r1 = 0;                                                 
    *(u64*)(r2 + 0) = r1;                                   
    r1 = map_hash_48b ll;                                
    call bpf_map_lookup_elem;                            
    if r0 == 0 goto l0_%=;                                  
    r1 = r0;                                                
    /* Read an unknown value */                             
    r7 = *(u64*)(r0 + 0);                                   
    /* Make it small and positive, to avoid other errors */ 
    r7 &= 4;                                                
    r2 = 0;                                                 
    r2 += r7;                                               
    call bpf_trace_printk;                               
l0_%=:    exit;                                               \
"#);


// SEC("tracepoint")
// __description("helper access to map: out-of-bound range")
// __failure __msg("invalid access to map value, value_size=48 off=0 size=56")
// operands: __imm(bpf_map_lookup_elem), __imm(bpf_probe_read_kernel), __imm_addr(map_hash_48b), __imm_const(__imm_0, sizeof(struct test_val) + 8)
bpf_asm_prog!(map_out_of_bound_range, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r2 = __imm_0;                
    r3 = 0;                        
    call bpf_probe_read_kernel;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("helper access to map: negative range")
// __failure __msg("R2 min value is negative")
// operands: __imm(bpf_map_lookup_elem), __imm(bpf_probe_read_kernel), __imm_addr(map_hash_48b)
bpf_asm_prog!(access_to_map_negative_range, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r2 = -8;                    
    r3 = 0;                        
    call bpf_probe_read_kernel;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("helper access to adjusted map (via const imm): full range")
// __success
// operands: __imm(bpf_map_lookup_elem), __imm(bpf_probe_read_kernel), __imm_addr(map_hash_48b), __imm_const(__imm_0, sizeof(struct test_val) - offsetof(struct test_val, foo)), __imm_const(test_val_foo, offsetof(struct test_val, foo))
bpf_asm_prog!(via_const_imm_full_range, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r1 += TEST_VAL_FOO;                
    r2 = __imm_0;                
    r3 = 0;                        
    call bpf_probe_read_kernel;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("helper access to adjusted map (via const imm): partial range")
// __success
// operands: __imm(bpf_map_lookup_elem), __imm(bpf_probe_read_kernel), __imm_addr(map_hash_48b), __imm_const(test_val_foo, offsetof(struct test_val, foo))
bpf_asm_prog!(via_const_imm_partial_range, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r1 += TEST_VAL_FOO;                
    r2 = 8;                        
    r3 = 0;                        
    call bpf_probe_read_kernel;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("helper access to adjusted map (via const imm): empty range")
// __failure __msg("R2 invalid zero-sized read")
// operands: __imm(bpf_map_lookup_elem), __imm(bpf_trace_printk), __imm_addr(map_hash_48b), __imm_const(test_val_foo, offsetof(struct test_val, foo))
bpf_asm_prog!(via_const_imm_empty_range, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r1 += TEST_VAL_FOO;                
    r2 = 0;                        
    call bpf_trace_printk;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("helper access to adjusted map (via const imm): out-of-bound range")
// __failure __msg("invalid access to map value, value_size=48 off=4 size=52")
// operands: __imm(bpf_map_lookup_elem), __imm(bpf_probe_read_kernel), __imm_addr(map_hash_48b), __imm_const(__imm_0, sizeof(struct test_val) - offsetof(struct test_val, foo) + 8), __imm_const(test_val_foo, offsetof(struct test_val, foo))
bpf_asm_prog!(imm_out_of_bound_range, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r1 += TEST_VAL_FOO;                
    r2 = __imm_0;                
    r3 = 0;                        
    call bpf_probe_read_kernel;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("helper access to adjusted map (via const imm): negative range (> adjustment)")
// __failure __msg("R2 min value is negative")
// operands: __imm(bpf_map_lookup_elem), __imm(bpf_probe_read_kernel), __imm_addr(map_hash_48b), __imm_const(test_val_foo, offsetof(struct test_val, foo))
bpf_asm_prog!(const_imm_negative_range_adjustment_1, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r1 += TEST_VAL_FOO;                
    r2 = -8;                    
    r3 = 0;                        
    call bpf_probe_read_kernel;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("helper access to adjusted map (via const imm): negative range (< adjustment)")
// __failure __msg("R2 min value is negative")
// operands: __imm(bpf_map_lookup_elem), __imm(bpf_probe_read_kernel), __imm_addr(map_hash_48b), __imm_const(test_val_foo, offsetof(struct test_val, foo))
bpf_asm_prog!(const_imm_negative_range_adjustment_2, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r1 += TEST_VAL_FOO;                
    r2 = -1;                    
    r3 = 0;                        
    call bpf_probe_read_kernel;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("helper access to adjusted map (via const reg): full range")
// __success
// operands: __imm(bpf_map_lookup_elem), __imm(bpf_probe_read_kernel), __imm_addr(map_hash_48b), __imm_const(__imm_0, sizeof(struct test_val) - offsetof(struct test_val, foo)), __imm_const(test_val_foo, offsetof(struct test_val, foo))
bpf_asm_prog!(via_const_reg_full_range, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r3 = TEST_VAL_FOO;                
    r1 += r3;                    
    r2 = __imm_0;                
    r3 = 0;                        
    call bpf_probe_read_kernel;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("helper access to adjusted map (via const reg): partial range")
// __success
// operands: __imm(bpf_map_lookup_elem), __imm(bpf_probe_read_kernel), __imm_addr(map_hash_48b), __imm_const(test_val_foo, offsetof(struct test_val, foo))
bpf_asm_prog!(via_const_reg_partial_range, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r3 = TEST_VAL_FOO;                
    r1 += r3;                    
    r2 = 8;                        
    r3 = 0;                        
    call bpf_probe_read_kernel;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("helper access to adjusted map (via const reg): empty range")
// __failure __msg("R2 invalid zero-sized read")
// operands: __imm(bpf_map_lookup_elem), __imm(bpf_trace_printk), __imm_addr(map_hash_48b)
bpf_asm_prog!(via_const_reg_empty_range, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r3 = 0;                        
    r1 += r3;                    
    r2 = 0;                        
    call bpf_trace_printk;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("helper access to adjusted map (via const reg): out-of-bound range")
// __failure __msg("invalid access to map value, value_size=48 off=4 size=52")
// operands: __imm(bpf_map_lookup_elem), __imm(bpf_probe_read_kernel), __imm_addr(map_hash_48b), __imm_const(__imm_0, sizeof(struct test_val) - offsetof(struct test_val, foo) + 8), __imm_const(test_val_foo, offsetof(struct test_val, foo))
bpf_asm_prog!(reg_out_of_bound_range, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r3 = TEST_VAL_FOO;                
    r1 += r3;                    
    r2 = __imm_0;                
    r3 = 0;                        
    call bpf_probe_read_kernel;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("helper access to adjusted map (via const reg): negative range (> adjustment)")
// __failure __msg("R2 min value is negative")
// operands: __imm(bpf_map_lookup_elem), __imm(bpf_probe_read_kernel), __imm_addr(map_hash_48b), __imm_const(test_val_foo, offsetof(struct test_val, foo))
bpf_asm_prog!(const_reg_negative_range_adjustment_1, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r3 = TEST_VAL_FOO;                
    r1 += r3;                    
    r2 = -8;                    
    r3 = 0;                        
    call bpf_probe_read_kernel;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("helper access to adjusted map (via const reg): negative range (< adjustment)")
// __failure __msg("R2 min value is negative")
// operands: __imm(bpf_map_lookup_elem), __imm(bpf_probe_read_kernel), __imm_addr(map_hash_48b), __imm_const(test_val_foo, offsetof(struct test_val, foo))
bpf_asm_prog!(const_reg_negative_range_adjustment_2, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r3 = TEST_VAL_FOO;                
    r1 += r3;                    
    r2 = -1;                    
    r3 = 0;                        
    call bpf_probe_read_kernel;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("helper access to adjusted map (via variable): full range")
// __success
// operands: __imm(bpf_map_lookup_elem), __imm(bpf_probe_read_kernel), __imm_addr(map_hash_48b), __imm_const(__imm_0, sizeof(struct test_val) - offsetof(struct test_val, foo)), __imm_const(test_val_foo, offsetof(struct test_val, foo))
bpf_asm_prog!(map_via_variable_full_range, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r3 = *(u32*)(r0 + 0);                
    if r3 > TEST_VAL_FOO goto l0_%=;        
    r1 += r3;                    
    r2 = __imm_0;                
    r3 = 0;                        
    call bpf_probe_read_kernel;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("helper access to adjusted map (via variable): partial range")
// __success
// operands: __imm(bpf_map_lookup_elem), __imm(bpf_probe_read_kernel), __imm_addr(map_hash_48b), __imm_const(test_val_foo, offsetof(struct test_val, foo))
bpf_asm_prog!(map_via_variable_partial_range, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r3 = *(u32*)(r0 + 0);                
    if r3 > TEST_VAL_FOO goto l0_%=;        
    r1 += r3;                    
    r2 = 8;                        
    r3 = 0;                        
    call bpf_probe_read_kernel;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("helper access to adjusted map (via variable): empty range")
// __failure __msg("R2 invalid zero-sized read")
// operands: __imm(bpf_map_lookup_elem), __imm(bpf_trace_printk), __imm_addr(map_hash_48b), __imm_const(test_val_foo, offsetof(struct test_val, foo))
bpf_asm_prog!(map_via_variable_empty_range, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r3 = *(u32*)(r0 + 0);                
    if r3 > TEST_VAL_FOO goto l0_%=;        
    r1 += r3;                    
    r2 = 0;                        
    call bpf_trace_printk;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("helper access to adjusted map (via variable): no max check")
// __failure __msg("R1 unbounded memory access")
// operands: __imm(bpf_map_lookup_elem), __imm(bpf_probe_read_kernel), __imm_addr(map_hash_48b)
bpf_asm_prog!(via_variable_no_max_check_1, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r3 = *(u32*)(r0 + 0);                
    r1 += r3;                    
    r2 = 1;                        
    r3 = 0;                        
    call bpf_probe_read_kernel;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("helper access to adjusted map (via variable): wrong max check")
// __failure __msg("invalid access to map value, value_size=48 off=4 size=45")
// operands: __imm(bpf_map_lookup_elem), __imm(bpf_probe_read_kernel), __imm_addr(map_hash_48b), __imm_const(__imm_0, sizeof(struct test_val) - offsetof(struct test_val, foo) + 1), __imm_const(test_val_foo, offsetof(struct test_val, foo))
bpf_asm_prog!(via_variable_wrong_max_check_1, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r3 = *(u32*)(r0 + 0);                
    if r3 > TEST_VAL_FOO goto l0_%=;        
    r1 += r3;                    
    r2 = __imm_0;                
    r3 = 0;                        
    call bpf_probe_read_kernel;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("helper access to map: bounds check using <, good access")
// __success
// operands: __imm(bpf_map_lookup_elem), __imm_addr(map_hash_48b)
bpf_asm_prog!(bounds_check_using_good_access_1, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r3 = *(u32*)(r0 + 0);                
    if r3 < 32 goto l1_%=;                
    r0 = 0;                        
l0_%=:    exit;                        
l1_%=:    r1 += r3;                    
    r0 = 0;                        
    *(u8*)(r1 + 0) = r0;                
    r0 = 0;                        
    exit;                        \
"#);


// SEC("tracepoint")
// __description("helper access to map: bounds check using <, bad access")
// __failure __msg("R1 unbounded memory access")
// operands: __imm(bpf_map_lookup_elem), __imm_addr(map_hash_48b)
bpf_asm_prog!(bounds_check_using_bad_access_1, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r3 = *(u32*)(r0 + 0);                
    if r3 < 32 goto l1_%=;                
    r1 += r3;                    
l0_%=:    r0 = 0;                        
    *(u8*)(r1 + 0) = r0;                
    r0 = 0;                        
    exit;                        
l1_%=:    r0 = 0;                        
    exit;                        \
"#);


// SEC("tracepoint")
// __description("helper access to map: bounds check using <=, good access")
// __success
// operands: __imm(bpf_map_lookup_elem), __imm_addr(map_hash_48b)
bpf_asm_prog!(bounds_check_using_good_access_2, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r3 = *(u32*)(r0 + 0);                
    if r3 <= 32 goto l1_%=;                
    r0 = 0;                        
l0_%=:    exit;                        
l1_%=:    r1 += r3;                    
    r0 = 0;                        
    *(u8*)(r1 + 0) = r0;                
    r0 = 0;                        
    exit;                        \
"#);


// SEC("tracepoint")
// __description("helper access to map: bounds check using <=, bad access")
// __failure __msg("R1 unbounded memory access")
// operands: __imm(bpf_map_lookup_elem), __imm_addr(map_hash_48b)
bpf_asm_prog!(bounds_check_using_bad_access_2, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r3 = *(u32*)(r0 + 0);                
    if r3 <= 32 goto l1_%=;                
    r1 += r3;                    
l0_%=:    r0 = 0;                        
    *(u8*)(r1 + 0) = r0;                
    r0 = 0;                        
    exit;                        
l1_%=:    r0 = 0;                        
    exit;                        \
"#);


// SEC("tracepoint")
// __description("helper access to map: bounds check using s<, good access")
// __success
// operands: __imm(bpf_map_lookup_elem), __imm_addr(map_hash_48b)
bpf_asm_prog!(check_using_s_good_access_1, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r3 = *(u32*)(r0 + 0);                
    if r3 s< 32 goto l1_%=;                
l2_%=:    r0 = 0;                        
l0_%=:    exit;                        
l1_%=:    if r3 s< 0 goto l2_%=;                
    r1 += r3;                    
    r0 = 0;                        
    *(u8*)(r1 + 0) = r0;                
    r0 = 0;                        
    exit;                        \
"#);


// SEC("tracepoint")
// __description("helper access to map: bounds check using s<, good access 2")
// __success
// operands: __imm(bpf_map_lookup_elem), __imm_addr(map_hash_48b)
bpf_asm_prog!(using_s_good_access_2_1, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r3 = *(u32*)(r0 + 0);                
    if r3 s< 32 goto l1_%=;                
l2_%=:    r0 = 0;                        
l0_%=:    exit;                        
l1_%=:    if r3 s< -3 goto l2_%=;                
    r1 += r3;                    
    r0 = 0;                        
    *(u8*)(r1 + 0) = r0;                
    r0 = 0;                        
    exit;                        \
"#);


// SEC("tracepoint")
// __description("helper access to map: bounds check using s<, bad access")
// __failure __msg("R1 min value is negative")
// operands: __imm(bpf_map_lookup_elem), __imm_addr(map_hash_48b)
bpf_asm_prog!(check_using_s_bad_access_1, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r3 = *(u64*)(r0 + 0);                
    if r3 s< 32 goto l1_%=;                
l2_%=:    r0 = 0;                        
l0_%=:    exit;                        
l1_%=:    if r3 s< -3 goto l2_%=;                
    r1 += r3;                    
    r0 = 0;                        
    *(u8*)(r1 + 0) = r0;                
    r0 = 0;                        
    exit;                        \
"#);


// SEC("tracepoint")
// __description("helper access to map: bounds check using s<=, good access")
// __success
// operands: __imm(bpf_map_lookup_elem), __imm_addr(map_hash_48b)
bpf_asm_prog!(check_using_s_good_access_2, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r3 = *(u32*)(r0 + 0);                
    if r3 s<= 32 goto l1_%=;            
l2_%=:    r0 = 0;                        
l0_%=:    exit;                        
l1_%=:    if r3 s<= 0 goto l2_%=;                
    r1 += r3;                    
    r0 = 0;                        
    *(u8*)(r1 + 0) = r0;                
    r0 = 0;                        
    exit;                        \
"#);


// SEC("tracepoint")
// __description("helper access to map: bounds check using s<=, good access 2")
// __success
// operands: __imm(bpf_map_lookup_elem), __imm_addr(map_hash_48b)
bpf_asm_prog!(using_s_good_access_2_2, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r3 = *(u32*)(r0 + 0);                
    if r3 s<= 32 goto l1_%=;            
l2_%=:    r0 = 0;                        
l0_%=:    exit;                        
l1_%=:    if r3 s<= -3 goto l2_%=;            
    r1 += r3;                    
    r0 = 0;                        
    *(u8*)(r1 + 0) = r0;                
    r0 = 0;                        
    exit;                        \
"#);


// SEC("tracepoint")
// __description("helper access to map: bounds check using s<=, bad access")
// __failure __msg("R1 min value is negative")
// operands: __imm(bpf_map_lookup_elem), __imm_addr(map_hash_48b)
bpf_asm_prog!(check_using_s_bad_access_2, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_48b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r1 = r0;                    
    r3 = *(u64*)(r0 + 0);                
    if r3 s<= 32 goto l1_%=;            
l2_%=:    r0 = 0;                        
l0_%=:    exit;                        
l1_%=:    if r3 s<= -3 goto l2_%=;            
    r1 += r3;                    
    r0 = 0;                        
    *(u8*)(r1 + 0) = r0;                
    r0 = 0;                        
    exit;                        \
"#);


// SEC("tracepoint")
// __description("map lookup helper access to map")
// __success
// operands: __imm(bpf_map_lookup_elem), __imm_addr(map_hash_16b)
bpf_asm_prog!(lookup_helper_access_to_map, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_16b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r2 = r0;                    
    r1 = map_hash_16b ll;            
    call bpf_map_lookup_elem;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("map update helper access to map")
// __success
// operands: __imm(bpf_map_lookup_elem), __imm(bpf_map_update_elem), __imm_addr(map_hash_16b)
bpf_asm_prog!(update_helper_access_to_map, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_16b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r4 = 0;                        
    r3 = r0;                    
    r2 = r0;                    
    r1 = map_hash_16b ll;            
    call bpf_map_update_elem;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("map update helper access to map: wrong size")
// __failure __msg("invalid access to map value, value_size=8 off=0 size=16")
// operands: __imm(bpf_map_lookup_elem), __imm(bpf_map_update_elem), __imm_addr(map_hash_16b), __imm_addr(map_hash_8b)
bpf_asm_prog!(access_to_map_wrong_size, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_8b ll;                
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r4 = 0;                        
    r3 = r0;                    
    r2 = r0;                    
    r1 = map_hash_16b ll;            
    call bpf_map_update_elem;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("map helper access to adjusted map (via const imm)")
// __success
// operands: __imm(bpf_map_lookup_elem), __imm_addr(map_hash_16b), __imm_const(other_val_bar, offsetof(struct other_val, bar))
bpf_asm_prog!(adjusted_map_via_const_imm, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_16b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r2 = r0;                    
    r2 += OTHER_VAL_BAR;                
    r1 = map_hash_16b ll;            
    call bpf_map_lookup_elem;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("map helper access to adjusted map (via const imm): out-of-bound 1")
// __failure __msg("invalid access to map value, value_size=16 off=12 size=8")
// operands: __imm(bpf_map_lookup_elem), __imm_addr(map_hash_16b), __imm_const(__imm_0, sizeof(struct other_val) - 4)
bpf_asm_prog!(imm_out_of_bound_1, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_16b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r2 = r0;                    
    r2 += __imm_0;                
    r1 = map_hash_16b ll;            
    call bpf_map_lookup_elem;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("map helper access to adjusted map (via const imm): out-of-bound 2")
// __failure __msg("R2 min value is negative")
// operands: __imm(bpf_map_lookup_elem), __imm_addr(map_hash_16b)
bpf_asm_prog!(imm_out_of_bound_2, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_16b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r2 = r0;                    
    r2 += -4;                    
    r1 = map_hash_16b ll;            
    call bpf_map_lookup_elem;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("map helper access to adjusted map (via const reg)")
// __success
// operands: __imm(bpf_map_lookup_elem), __imm_addr(map_hash_16b), __imm_const(other_val_bar, offsetof(struct other_val, bar))
bpf_asm_prog!(adjusted_map_via_const_reg, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_16b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r2 = r0;                    
    r3 = OTHER_VAL_BAR;                
    r2 += r3;                    
    r1 = map_hash_16b ll;            
    call bpf_map_lookup_elem;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("map helper access to adjusted map (via const reg): out-of-bound 1")
// __failure __msg("invalid access to map value, value_size=16 off=12 size=8")
// operands: __imm(bpf_map_lookup_elem), __imm_addr(map_hash_16b), __imm_const(__imm_0, sizeof(struct other_val) - 4)
bpf_asm_prog!(reg_out_of_bound_1, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_16b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r2 = r0;                    
    r3 = __imm_0;                
    r2 += r3;                    
    r1 = map_hash_16b ll;            
    call bpf_map_lookup_elem;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("map helper access to adjusted map (via const reg): out-of-bound 2")
// __failure __msg("R2 min value is negative")
// operands: __imm(bpf_map_lookup_elem), __imm_addr(map_hash_16b)
bpf_asm_prog!(reg_out_of_bound_2, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_16b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r2 = r0;                    
    r3 = -4;                    
    r2 += r3;                    
    r1 = map_hash_16b ll;            
    call bpf_map_lookup_elem;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("map helper access to adjusted map (via variable)")
// __success
// operands: __imm(bpf_map_lookup_elem), __imm_addr(map_hash_16b), __imm_const(other_val_bar, offsetof(struct other_val, bar))
bpf_asm_prog!(to_adjusted_map_via_variable, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_16b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r2 = r0;                    
    r3 = *(u32*)(r0 + 0);                
    if r3 > OTHER_VAL_BAR goto l0_%=;        
    r2 += r3;                    
    r1 = map_hash_16b ll;            
    call bpf_map_lookup_elem;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("map helper access to adjusted map (via variable): no max check")
// __failure
// __msg("R2 unbounded memory access, make sure to bounds check any such access")
// operands: __imm(bpf_map_lookup_elem), __imm_addr(map_hash_16b)
bpf_asm_prog!(via_variable_no_max_check_2, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_16b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r2 = r0;                    
    r3 = *(u32*)(r0 + 0);                
    r2 += r3;                    
    r1 = map_hash_16b ll;            
    call bpf_map_lookup_elem;            
l0_%=:    exit;                        \
"#);


// SEC("tracepoint")
// __description("map helper access to adjusted map (via variable): wrong max check")
// __failure __msg("invalid access to map value, value_size=16 off=9 size=8")
// operands: __imm(bpf_map_lookup_elem), __imm_addr(map_hash_16b), __imm_const(__imm_0, offsetof(struct other_val, bar) + 1)
bpf_asm_prog!(via_variable_wrong_max_check_2, r#"
    r2 = r10;                    
    r2 += -8;                    
    r1 = 0;                        
    *(u64*)(r2 + 0) = r1;                
    r1 = map_hash_16b ll;            
    call bpf_map_lookup_elem;            
    if r0 == 0 goto l0_%=;                
    r2 = r0;                    
    r3 = *(u32*)(r0 + 0);                
    if r3 > __imm_0 goto l0_%=;            
    r2 += r3;                    
    r1 = map_hash_16b ll;            
    call bpf_map_lookup_elem;            
l0_%=:    exit;                        \
"#);


#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
