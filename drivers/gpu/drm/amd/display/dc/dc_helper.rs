/* Rust translation of dc_helper.c. External kernel/DC symbols are supplied by
 * the surrounding translation unit. */

#[repr(C)]
struct dc_reg_value_masks { value: u32, mask: u32 }

#[inline]
unsafe fn set_reg_field_value_masks(f: *mut dc_reg_value_masks, value: u32, mask: u32, shift: u8) {
    assert!(mask != 0);
    (*f).value = ((*f).value & !mask) | (mask & value.wrapping_shl(shift as u32));
    (*f).mask |= mask;
}

unsafe fn set_reg_field_values(f: *mut dc_reg_value_masks, _addr: u32, n: i32,
    shift1: u8, mask1: u32, field_value1: u32, ap: *mut u32) {
    set_reg_field_value_masks(f, field_value1, mask1, shift1);
    let mut i = 1;
    while i < n {
        let shift = *ap; let mask = *ap.add(1); let value = *ap.add(2);
        set_reg_field_value_masks(f, value, mask, shift as u8);
        ap = ap.add(3); i += 1;
    }
}

extern "C" {
    fn dm_read_reg(ctx: *const dc_context, addr: u32) -> u32;
    fn dm_write_reg(ctx: *const dc_context, addr: u32, value: u32);
    fn get_reg_field_value_ex(value: u32, mask: u32, shift: u8) -> u32;
    fn set_reg_field_value_ex(value: u32, field_value: u32, mask: u32, shift: u8) -> u32;
    fn dm_write_index_reg(ctx: *const dc_context, index_type: u32, index: u32, value: u32);
    fn dm_read_index_reg(ctx: *const dc_context, index_type: u32, index: u32) -> u32;
}
#[repr(C)] pub struct dc_context;
pub const CGS_IND_REG__PCIE: u32 = 0;

pub unsafe fn generic_reg_update_ex(ctx: *const dc_context, addr: u32, n: i32, shift1: u8, mask1: u32, value1: u32, ap: *mut u32) -> u32 {
    let mut f = dc_reg_value_masks { value: 0, mask: 0 };
    set_reg_field_values(&mut f, addr, n, shift1, mask1, value1, ap);
    let value = (dm_read_reg(ctx, addr) & !f.mask) | f.value;
    dm_write_reg(ctx, addr, value); value
}
pub unsafe fn generic_reg_set_ex(ctx: *const dc_context, addr: u32, mut reg_val: u32, n: i32, shift1: u8, mask1: u32, value1: u32, ap: *mut u32) -> u32 {
    let mut f = dc_reg_value_masks { value: 0, mask: 0 };
    set_reg_field_values(&mut f, addr, n, shift1, mask1, value1, ap);
    reg_val = (reg_val & !f.mask) | f.value; dm_write_reg(ctx, addr, reg_val); reg_val
}

pub unsafe fn generic_reg_get(ctx: *const dc_context, addr: u32, shift: u8, mask: u32, out: *mut u32) -> u32 { let r=dm_read_reg(ctx,addr); *out=get_reg_field_value_ex(r,mask,shift); r }

pub unsafe fn generic_reg_wait(ctx:*const dc_context, addr:u32, shift:u32, mask:u32, condition:u32, delay_us:u32, tries:u32, _func:*const u8, _line:i32) {
    assert!(delay_us.saturating_mul(tries) <= 3_000_000);
    for i in 0..=tries {
        let _ = i; // Delay and diagnostic logging are supplied by the platform layer.
        let r=dm_read_reg(ctx,addr);
        if get_reg_field_value_ex(r,mask,shift as u8)==condition { return; }
    }
    panic!("REG_WAIT timeout");
}

macro_rules! reg_get_n { ($name:ident, $(($s:ident,$m:ident,$o:ident)),+) => { pub unsafe fn $name(ctx:*const dc_context, addr:u32, $( $s:u8,$m:u32,$o:*mut u32),+) -> u32 { let r=dm_read_reg(ctx,addr); $(*$o=get_reg_field_value_ex(r,$m,$s);)+ r } }; }
reg_get_n!(generic_reg_get2,(s1,m1,o1),(s2,m2,o2));
reg_get_n!(generic_reg_get3,(s1,m1,o1),(s2,m2,o2),(s3,m3,o3));
reg_get_n!(generic_reg_get4,(s1,m1,o1),(s2,m2,o2),(s3,m3,o3),(s4,m4,o4));
reg_get_n!(generic_reg_get5,(s1,m1,o1),(s2,m2,o2),(s3,m3,o3),(s4,m4,o4),(s5,m5,o5));
reg_get_n!(generic_reg_get6,(s1,m1,o1),(s2,m2,o2),(s3,m3,o3),(s4,m4,o4),(s5,m5,o5),(s6,m6,o6));
reg_get_n!(generic_reg_get7,(s1,m1,o1),(s2,m2,o2),(s3,m3,o3),(s4,m4,o4),(s5,m5,o5),(s6,m6,o6),(s7,m7,o7));
reg_get_n!(generic_reg_get8,(s1,m1,o1),(s2,m2,o2),(s3,m3,o3),(s4,m4,o4),(s5,m5,o5),(s6,m6,o6),(s7,m7,o7),(s8,m8,o8));

pub unsafe fn generic_write_indirect_reg(ctx:*const dc_context, ai:u32, ad:u32, index:u32, data:u32){dm_write_reg(ctx,ai,index);dm_write_reg(ctx,ad,data)}
pub unsafe fn generic_read_indirect_reg(ctx:*const dc_context, ai:u32, ad:u32, index:u32)->u32{dm_write_reg(ctx,ai,index);dm_read_reg(ctx,ad)}
pub unsafe fn generic_indirect_reg_get(ctx:*const dc_context, ai:u32, ad:u32, index:u32, _n:i32, s:u8,m:u32,o:*mut u32, ap:*mut u32)->u32{let v=generic_read_indirect_reg(ctx,ai,ad,index);*o=get_reg_field_value_ex(v,m,s);let mut p=ap;for _ in 1.._n{let sh=*p;let ma=*p.add(1);let dst=*p.add(2) as *mut u32;*dst=get_reg_field_value_ex(v,ma,sh as u8);p=p.add(3)}v}

pub unsafe fn generic_indirect_reg_update_ex(ctx:*const dc_context,ai:u32,ad:u32,index:u32,mut r:u32,n:i32,s:u8,m:u32,v:u32,ap:*mut u32)->u32{r=set_reg_field_value_ex(r,v,m,s);let mut p=ap;for _ in 1..n{r=set_reg_field_value_ex(r,*p.add(2),*p.add(1),*p as u8);p=p.add(3)}generic_write_indirect_reg(ctx,ai,ad,index,r);r}
pub unsafe fn generic_indirect_reg_update_ex_sync(ctx:*const dc_context,index:u32,mut r:u32,n:i32,s:u8,m:u32,v:u32,ap:*mut u32)->u32{r=set_reg_field_value_ex(r,v,m,s);let mut p=ap;for _ in 1..n{r=set_reg_field_value_ex(r,*p.add(2),*p.add(1),*p as u8);p=p.add(3)}dm_write_index_reg(ctx,CGS_IND_REG__PCIE,index,r);r}
pub unsafe fn generic_indirect_reg_get_sync(ctx:*const dc_context,index:u32,_n:i32,s:u8,m:u32,o:*mut u32,ap:*mut u32)->u32{let v=dm_read_index_reg(ctx,CGS_IND_REG__PCIE,index);*o=get_reg_field_value_ex(v,m,s);let mut p=ap;for _ in 1.._n{let sh=*p;let ma=*p.add(1);let dst=*p.add(2) as *mut u32;*dst=get_reg_field_value_ex(v,ma,sh as u8);p=p.add(3)}v}

pub unsafe fn dce_version_to_string(v:i32)->*const u8 { match v {
    60=>b"DCE 6.0\0".as_ptr(),61=>b"DCE 6.1\0".as_ptr(),64=>b"DCE 6.4\0".as_ptr(),80=>b"DCE 8.0\0".as_ptr(),81=>b"DCE 8.1\0".as_ptr(),83=>b"DCE 8.3\0".as_ptr(),100=>b"DCE 10.0\0".as_ptr(),110=>b"DCE 11.0\0".as_ptr(),112=>b"DCE 11.2\0".as_ptr(),1122=>b"DCE 11.22\0".as_ptr(),120=>b"DCE 12.0\0".as_ptr(),121=>b"DCE 12.1\0".as_ptr(),
    1000=>b"DCN 1.0\0".as_ptr(),1001=>b"DCN 1.0.1\0".as_ptr(),2000=>b"DCN 2.0\0".as_ptr(),2100=>b"DCN 2.1\0".as_ptr(),2001=>b"DCN 2.0.1\0".as_ptr(),3000=>b"DCN 3.0\0".as_ptr(),3001=>b"DCN 3.0.1\0".as_ptr(),3002=>b"DCN 3.0.2\0".as_ptr(),3003=>b"DCN 3.0.3\0".as_ptr(),3100=>b"DCN 3.1.2\0".as_ptr(),3114=>b"DCN 3.1.4\0".as_ptr(),3115=>b"DCN 3.1.5\0".as_ptr(),3116=>b"DCN 3.1.6\0".as_ptr(),3200=>b"DCN 3.2\0".as_ptr(),3201=>b"DCN 3.2.1\0".as_ptr(),3500=>b"DCN 3.5\0".as_ptr(),3501=>b"DCN 3.5.1\0".as_ptr(),3600=>b"DCN 3.6\0".as_ptr(),401=>b"DCN 4.0.1\0".as_ptr(),4200=>b"DCN 4.2\0".as_ptr(),4201=>b"DCN 4.2B\0".as_ptr(),6000=>b"DCN 6.0\0".as_ptr(),_=>b"Unknown\0".as_ptr() } }
pub fn dc_supports_vrr(v:i32)->bool { v >= 8 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
