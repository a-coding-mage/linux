// SPDX-License-Identifier: GPL-2.0
// Translation of nlattr.c. Kernel-provided types, constants, and helpers are
// intentionally referenced as external dependencies.

const MAX_POLICY_RECURSION_DEPTH: c_uint = 10;

static NLA_ATTR_LEN: [u8; NLA_TYPE_MAX as usize + 1] = [
    0, size_of::<u8>() as u8, size_of::<u16>() as u8, size_of::<u32>() as u8,
    size_of::<u64>() as u8, 0, 0, 0, size_of::<i8>() as u8,
    size_of::<i16>() as u8, size_of::<i32>() as u8, size_of::<i64>() as u8,
    0, 0, size_of::<__be16>() as u8, size_of::<__be32>() as u8,
];
static NLA_ATTR_MINLEN: [u8; NLA_TYPE_MAX as usize + 1] = [
    0, size_of::<u8>() as u8, size_of::<u16>() as u8, size_of::<u32>() as u8,
    size_of::<u64>() as u8, size_of::<u64>() as u8, NLA_HDRLEN as u8, 0,
    size_of::<i8>() as u8, size_of::<i16>() as u8, size_of::<i32>() as u8,
    size_of::<i64>() as u8, 0, 0, size_of::<__be16>() as u8,
    size_of::<__be32>() as u8,
];

unsafe fn validate_nla_bitfield32(nla: *const nlattr, valid_flags_mask: u32) -> c_int {
    if valid_flags_mask == 0 { return -EINVAL; }
    let bf = nla_data(nla) as *const nla_bitfield32;
    if (*bf).selector & !valid_flags_mask != 0 || (*bf).value & !valid_flags_mask != 0
        || (*bf).value & !(*bf).selector != 0 { return -EINVAL; }
    0
}

unsafe fn nla_validate_array(head: *const nlattr, len: c_int, maxtype: c_int,
    policy: *const nla_policy, extack: *mut netlink_ext_ack, validate: c_uint,
    depth: c_uint) -> c_int {
    let mut entry: *const nlattr = core::ptr::null();
    let mut rem = 0;
    nla_for_each_attr!(entry, head, len, rem) {
        if nla_len(entry) == 0 { continue; }
        if nla_len(entry) < NLA_HDRLEN { NL_SET_ERR_MSG_ATTR_POL!(extack, entry, policy, "Array element too short"); return -ERANGE; }
        let ret = __nla_validate_parse(nla_data(entry), nla_len(entry), maxtype, policy, validate, extack, core::ptr::null_mut(), depth + 1);
        if ret < 0 { return ret; }
    }
    0
}

pub unsafe fn nla_get_range_unsigned(pt: *const nla_policy, range: *mut netlink_range_validation) {
    WARN_ON_ONCE!((*pt).validation_type != NLA_VALIDATE_RANGE_PTR && ((*pt).min < 0 || (*pt).max < 0));
    (*range).min = 0;
    (*range).max = match (*pt).type_ {
        NLA_U8 => U8_MAX as u64,
        NLA_U16 | NLA_BE16 | NLA_BINARY => U16_MAX as u64,
        NLA_U32 | NLA_BE32 => U32_MAX as u64,
        NLA_U64 | NLA_UINT | NLA_MSECS => U64_MAX,
        _ => { WARN_ON_ONCE!(true); return; }
    };
    match (*pt).validation_type {
        NLA_VALIDATE_RANGE | NLA_VALIDATE_RANGE_WARN_TOO_LONG => { (*range).min = (*pt).min; (*range).max = (*pt).max; }
        NLA_VALIDATE_RANGE_PTR => *range = *(*pt).range,
        NLA_VALIDATE_MIN => (*range).min = (*pt).min,
        NLA_VALIDATE_MAX => (*range).max = (*pt).max,
        _ => {}
    }
}

unsafe fn nla_validate_range_unsigned(pt: *const nla_policy, nla: *const nlattr,
    extack: *mut netlink_ext_ack, validate: c_uint) -> c_int {
    let value = match (*pt).type_ {
        NLA_U8 => nla_get_u8(nla) as u64, NLA_U16 => nla_get_u16(nla) as u64,
        NLA_U32 => nla_get_u32(nla) as u64, NLA_U64 | NLA_MSECS => nla_get_u64(nla),
        NLA_UINT => nla_get_uint(nla) as u64, NLA_BINARY => nla_len(nla) as u64,
        NLA_BE16 => ntohs(nla_get_be16(nla)) as u64, NLA_BE32 => ntohl(nla_get_be32(nla)) as u64,
        _ => return -EINVAL,
    };
    let mut range = netlink_range_validation::default(); nla_get_range_unsigned(pt, &mut range);
    if (*pt).validation_type == NLA_VALIDATE_RANGE_WARN_TOO_LONG && (*pt).type_ == NLA_BINARY && value > range.max {
        pr_warn_ratelimited!("netlink: attribute has an invalid length.\n");
        if validate & NL_VALIDATE_STRICT_ATTRS != 0 { NL_SET_ERR_MSG_ATTR_POL!(extack, nla, pt, "invalid attribute length"); return -EINVAL; }
        return 0;
    }
    if value < range.min || value > range.max { if (*pt).type_ == NLA_BINARY { NL_SET_ERR_MSG_ATTR_POL!(extack, nla, pt, "binary attribute size out of range"); } else { NL_SET_ERR_MSG_ATTR_POL!(extack, nla, pt, "integer out of range"); } return -ERANGE; }
    0
}

pub unsafe fn nla_get_range_signed(pt: *const nla_policy, range: *mut netlink_range_validation_signed) {
    match (*pt).type_ { NLA_S8 => {(*range).min=S8_MIN as i64;(*range).max=S8_MAX as i64}, NLA_S16=>{(*range).min=S16_MIN as i64;(*range).max=S16_MAX as i64}, NLA_S32=>{(*range).min=S32_MIN as i64;(*range).max=S32_MAX as i64}, NLA_S64|NLA_SINT=>{(*range).min=S64_MIN;(*range).max=S64_MAX}, _=>{WARN_ON_ONCE!(true);return;} }
    match (*pt).validation_type { NLA_VALIDATE_RANGE=>{(*range).min=(*pt).min as i64;(*range).max=(*pt).max as i64}, NLA_VALIDATE_RANGE_PTR=>*range=*(*pt).range_signed, NLA_VALIDATE_MIN=>(*range).min=(*pt).min as i64, NLA_VALIDATE_MAX=>(*range).max=(*pt).max as i64, _=>{} }
}

unsafe fn nla_validate_int_range_signed(pt:*const nla_policy,nla:*const nlattr,extack:*mut netlink_ext_ack)->c_int { let value=match (*pt).type_ {NLA_S8=>nla_get_s8(nla) as i64,NLA_S16=>nla_get_s16(nla) as i64,NLA_S32=>nla_get_s32(nla) as i64,NLA_S64=>nla_get_s64(nla),NLA_SINT=>nla_get_sint(nla) as i64,_=>return -EINVAL}; let mut range=netlink_range_validation_signed::default();nla_get_range_signed(pt,&mut range);if value<range.min||value>range.max {NL_SET_ERR_MSG_ATTR_POL!(extack,nla,pt,"integer out of range");return -ERANGE} 0 }

unsafe fn nla_validate_int_range(pt:*const nla_policy,nla:*const nlattr,extack:*mut netlink_ext_ack,validate:c_uint)->c_int {match (*pt).type_ {NLA_U8|NLA_U16|NLA_U32|NLA_U64|NLA_UINT|NLA_MSECS|NLA_BINARY|NLA_BE16|NLA_BE32=>nla_validate_range_unsigned(pt,nla,extack,validate),NLA_S8|NLA_S16|NLA_S32|NLA_S64|NLA_SINT=>nla_validate_int_range_signed(pt,nla,extack),_=>{WARN_ON!(true);-EINVAL}}}

unsafe fn nla_validate_mask(pt:*const nla_policy,nla:*const nlattr,extack:*mut netlink_ext_ack)->c_int {let value=match (*pt).type_{NLA_U8=>nla_get_u8(nla)as u64,NLA_U16=>nla_get_u16(nla)as u64,NLA_U32=>nla_get_u32(nla)as u64,NLA_U64=>nla_get_u64(nla),NLA_UINT=>nla_get_uint(nla)as u64,NLA_BE16=>ntohs(nla_get_be16(nla))as u64,NLA_BE32=>ntohl(nla_get_be32(nla))as u64,_=>return -EINVAL};if value&!((*pt).mask as u64)!=0{NL_SET_ERR_MSG_ATTR!(extack,nla,"reserved bit set");return -EINVAL}0}

// The remaining validation and skb helpers retain the C control flow and call
// the corresponding kernel ABI helpers supplied by the surrounding translation.
pub unsafe fn __nla_validate(head:*const nlattr,len:c_int,maxtype:c_int,policy:*const nla_policy,validate:c_uint,extack:*mut netlink_ext_ack)->c_int { __nla_validate_parse(head,len,maxtype,policy,validate,extack,core::ptr::null_mut(),0) }
pub unsafe fn nla_policy_len(mut p:*const nla_policy,n:c_int)->c_int { let mut len=0;for _ in 0..n{if(*p).len!=0{len+=nla_total_size((*p).len)}else if NLA_ATTR_LEN[(*p).type_ as usize]!=0{len+=nla_total_size(NLA_ATTR_LEN[(*p).type_ as usize] as c_int)}else{len+=nla_total_size(NLA_ATTR_MINLEN[(*p).type_ as usize] as c_int)}p=p.add(1)}len }
pub unsafe fn nla_find(head:*const nlattr,len:c_int,attrtype:c_int)->*mut nlattr{let mut n=core::ptr::null();let mut r=0;nla_for_each_attr!(n,head,len,r){if nla_type(n)==attrtype{return n as *mut nlattr}}core::ptr::null_mut()}
pub unsafe fn nla_strscpy(dst:*mut c_char,nla:*const nlattr,dstsize:usize)->isize{let mut s=nla_len(nla)as usize;let src=nla_data(nla)as*mut c_char;if dstsize==0||dstsize>U16_MAX as usize{return -E2BIG as isize}if s>0&&*src.add(s-1)==0{s-=1}let l=if s>=dstsize{dstsize-1}else{s};core::ptr::copy_nonoverlapping(src,dst,l);core::ptr::write_bytes(dst.add(l),0,dstsize-l);if s>=dstsize{-E2BIG as isize}else{l as isize}}
pub unsafe fn nla_memcpy(dest:*mut c_void,src:*const nlattr,count:c_int)->c_int{let n=core::cmp::min(count,nla_len(src));core::ptr::copy_nonoverlapping(nla_data(src),dest,n as usize);if count>n{core::ptr::write_bytes((dest as*mut u8).add(n as usize),0,(count-n)as usize)}n}
pub unsafe fn nla_memcmp(nla:*const nlattr,data:*const c_void,size:usize)->c_int{let d=nla_len(nla)-size as c_int;if d==0{memcmp(nla_data(nla),data,size)}else{d}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
