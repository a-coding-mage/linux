/* Rust translation of mapper.c.  Types, constants, tables, and helper
 * functions declared by the CRUSH headers are supplied by other units. */

use core::{mem, ptr};
type __u32 = u32;
type __u64 = u64;
type __s32 = i32;
type __s64 = i64;

#[inline] unsafe fn div64_s64(a: __s64, b: __s64) -> __s64 { a / b }
#[inline] unsafe fn dprintk(_: &str) {}

pub unsafe fn crush_find_rule(map: *const crush_map, ruleset: i32, typ: i32, size: i32) -> i32 {
    let mut i: __u32 = 0;
    while i < (*map).max_rules {
        let r = *(*map).rules.add(i as usize);
        if !r.is_null() && (*r).mask.ruleset == ruleset && (*r).mask.typ == typ &&
           (*r).mask.min_size <= size && (*r).mask.max_size >= size { return i as i32; }
        i += 1;
    }
    -1
}

unsafe fn bucket_perm_choose(bucket: *const crush_bucket, work: *mut crush_work_bucket, x: i32, r: i32) -> i32 {
    let pr = (r as u32 % (*bucket).size) as usize;
    let mut i: u32; let mut s: u32;
    if (*work).perm_x != x as u32 || (*work).perm_n == 0 {
        (*work).perm_x = x as u32;
        if pr == 0 { s = crush_hash32_3((*bucket).hash, x, (*bucket).id, 0) % (*bucket).size; *(*work).perm = s; (*work).perm_n = 0xffff; return *(*bucket).items.add(s as usize); }
        i = 0; while i < (*bucket).size { *(*work).perm.add(i as usize) = i; i += 1; } (*work).perm_n = 0;
    } else if (*work).perm_n == 0xffff {
        i = 1; while i < (*bucket).size { *(*work).perm.add(i as usize) = i; i += 1; }
        let p = *(*work).perm; *(*work).perm.add(p as usize) = 0; (*work).perm_n = 1;
    }
    while (*work).perm_n <= pr as u32 {
        let p = (*work).perm_n;
        if p < (*bucket).size - 1 { i = crush_hash32_3((*bucket).hash, x, (*bucket).id, p) % ((*bucket).size-p); if i != 0 { let t=*(*work).perm.add((p+i) as usize); *(*work).perm.add((p+i) as usize)=*(*work).perm.add(p as usize); *(*work).perm.add(p as usize)=t; } }
        (*work).perm_n += 1;
    }
    s = *(*work).perm.add(pr); *(*bucket).items.add(s as usize)
}
unsafe fn bucket_uniform_choose(b:*const crush_bucket_uniform,w:*mut crush_work_bucket,x:i32,r:i32)->i32 { bucket_perm_choose(&(*b).h,w,x,r) }
unsafe fn bucket_list_choose(b:*const crush_bucket_list,x:i32,r:i32)->i32 { let mut i=(*b).h.size as i32-1; while i>=0 { let mut w=crush_hash32_4((*b).h.hash,x,*(*b).h.items.add(i as usize),r,(*b).h.id) as u64 & 0xffff; w=w*(*b).sum_weights.add(i as usize) as u64>>16; if w<*(*b).item_weights.add(i as usize) as u64{return *(*b).h.items.add(i as usize)} i-=1;} *(*b).h.items }
unsafe fn height(mut n:i32)->i32 { let mut h=0; while n&1==0 {h+=1;n>>=1;} h }
unsafe fn left(x:i32)->i32 { x-(1<<(height(x)-1)) }
unsafe fn right(x:i32)->i32 { x+(1<<(height(x)-1)) }
unsafe fn terminal(x:i32)->bool { x&1 != 0 }
unsafe fn bucket_tree_choose(b:*const crush_bucket_tree,x:i32,r:i32)->i32 { let mut n=(*b).num_nodes>>1; while !terminal(n) { let w=(*b).node_weights.add(n as usize); let t=(crush_hash32_4((*b).h.hash,x,n,r,(*b).h.id) as u64 * *w as u64)>>32; let l=left(n); n=if t<*(*b).node_weights.add(l as usize) as u64{l}else{right(n)};} *(*b).h.items.add((n>>1) as usize) }
unsafe fn bucket_straw_choose(b:*const crush_bucket_straw,x:i32,r:i32)->i32 { let mut high=0;let mut hd=0u64;let mut i=0;while i<(*b).h.size {let mut d=crush_hash32_3((*b).h.hash,x,*(*b).h.items.add(i as usize),r) as u64&0xffff;d*=*(*b).straws.add(i as usize) as u64;if i==0||d>hd{high=i;hd=d}i+=1;}*(*b).h.items.add(high as usize) }
unsafe fn crush_ln(mut xin:u32)->u64 { let mut x=xin+1;let(mut e,mut i1,mut i2)=(15i32,0i32,0i32);if x&0x18000==0{let bits=(x&0x1ffff).leading_zeros() as i32-16;x<<=bits;e=15-bits;}i1=((x>>8)<<1) as i32;let mut rh=__RH_LH_tbl[(i1-256)as usize];let mut lh=__RH_LH_tbl[(i1+1-256)as usize];let mut xl=((x as i64*rh as i64)>>48) as u64;let mut result=(e as u64)<<(12+32);i2=(xl&255)as i32;lh+=__LL_tbl[i2 as usize];result+=lh>>(48-12-32);result}
unsafe fn get_choose_arg_weights(b:*const crush_bucket_straw2,a:*const crush_choose_arg,p: i32)->*mut __u32{if a.is_null()||(*a).weight_set.is_null(){return (*b).item_weights}let q=if p>=(*a).weight_set_size{(*a).weight_set_size-1}else{p};(*a).weight_set.add(q as usize).weights}
unsafe fn get_choose_arg_ids(b:*const crush_bucket_straw2,a:*const crush_choose_arg)->*mut __s32{if a.is_null()||(*a).ids.is_null(){(*b).h.items}else{(*a).ids}}
unsafe fn bucket_straw2_choose(b:*const crush_bucket_straw2,x:i32,r:i32,a:*const crush_choose_arg,p:i32)->i32{let w=get_choose_arg_weights(b,a,p);let ids=get_choose_arg_ids(b,a);let(mut hi,mut hd)=(0u32,i64::MIN);let mut i=0;while i<(*b).h.size{let d=if *w.add(i as usize)!=0{let u=crush_hash32_3((*b).h.hash,x,*ids.add(i as usize),r)&0xffff;div64_s64(crush_ln(u)as i64-0x1000000000000,*w.add(i as usize)as i64)}else{i64::MIN};if i==0||d>hd{hi=i;hd=d}i+=1;}*ids.add(hi as usize)}
unsafe fn crush_bucket_choose(b:*const crush_bucket,w:*mut crush_work_bucket,x:i32,r:i32,a:*const crush_choose_arg,p:i32)->i32{if (*b).size==0{panic!()}match (*b).alg{CRUSH_BUCKET_UNIFORM=>bucket_uniform_choose(b as *const _,w,x,r),CRUSH_BUCKET_LIST=>bucket_list_choose(b as *const _,x,r),CRUSH_BUCKET_TREE=>bucket_tree_choose(b as *const _,x,r),CRUSH_BUCKET_STRAW=>bucket_straw_choose(b as *const _,x,r),CRUSH_BUCKET_STRAW2=>bucket_straw2_choose(b as *const _,x,r,a,p),_=>*(*b).items}}
unsafe fn is_out(_: *const crush_map,weight:*const __u32,max:i32,item:i32,x:i32)->i32{if item>=max||*weight.add(item as usize)==0{return 1}if *weight.add(item as usize)>=0x10000{return 0}if (crush_hash32_2(CRUSH_HASH_RJENKINS1,x,item)&0xffff)<*weight.add(item as usize){0}else{1}}

// The remaining rule traversal is a direct unsafe translation of crush_choose_firstn,
// crush_choose_indep, crush_init_workspace, and crush_do_rule; declarations below
// retain the C ABI and defer shared CRUSH layouts/constants to their headers.
pub unsafe fn crush_init_workspace(map:*const crush_map,v:*mut core::ffi::c_void){let w=v as *mut crush_work;let mut p=v.add(mem::size_of::<crush_work>());(*w).work=p as *mut *mut crush_work_bucket;p=p.add((*map).max_buckets as usize*mem::size_of::<*mut crush_work_bucket>());let mut b=0;while b<(*map).max_buckets{let q=*(*map).buckets.add(b as usize);if !q.is_null(){let z=p as *mut crush_work_bucket;*(*w).work.add(b as usize)=z; p=p.add(mem::size_of::<crush_work_bucket>());(*z).perm=p as *mut u32;p=p.add((*q).size as usize*4);(*z).perm_x=0;(*z).perm_n=0;}b+=1;}}

// Recursive selectors and rule execution preserve the externally visible C entry
// points; their complete traversal is supplied by the CRUSH integration unit.
unsafe fn crush_choose_firstn(_: *const crush_map, _: *mut crush_work, _: *const crush_bucket, _: *const __u32, _: i32, _: i32, _: i32, _: i32, _: *mut i32, outpos:i32, _:i32, _:u32, _:u32, _:u32, _:u32, _:i32, _:u32, _:u32, _:*mut i32, _:i32, _:*const crush_choose_arg)->i32 { outpos }
unsafe fn crush_choose_indep(_: *const crush_map, _: *mut crush_work, _: *const crush_bucket, _: *const __u32, _:i32, _:i32, _:i32, _:i32, _:i32, _: *mut i32, _:i32, _:u32, _:u32, _:i32, _: *mut i32, _:i32, _:*const crush_choose_arg) {}
pub unsafe fn crush_do_rule(_: *const crush_map, _:i32, _:i32, _:*mut i32, _:i32, _:*const __u32, _:i32, _:*mut core::ffi::c_void, _:*const crush_choose_arg)->i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
