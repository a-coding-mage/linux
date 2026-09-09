/* Common NFSv4 ACL handling code; direct translation of nfs4acl.c. */

const NFS4_ACL_TYPE_DEFAULT: u32 = 0x01;
const NFS4_ACL_DIR: u32 = 0x02;
const NFS4_ACL_OWNER: u32 = 0x04;
const NFS4_READ_MODE: u32 = NFS4_ACE_READ_DATA;
const NFS4_WRITE_MODE: u32 = NFS4_ACE_WRITE_DATA | NFS4_ACE_APPEND_DATA;
const NFS4_EXECUTE_MODE: u32 = NFS4_ACE_EXECUTE;
const NFS4_ANYONE_MODE: u32 = NFS4_ACE_READ_ATTRIBUTES | NFS4_ACE_READ_ACL | NFS4_ACE_SYNCHRONIZE;
const NFS4_OWNER_MODE: u32 = NFS4_ACE_WRITE_ATTRIBUTES | NFS4_ACE_WRITE_ACL;
const NFS4_INHERITANCE_FLAGS: u32 = NFS4_ACE_FILE_INHERIT_ACE | NFS4_ACE_DIRECTORY_INHERIT_ACE;
const NFS4_SUPPORTED_FLAGS: u32 = NFS4_INHERITANCE_FLAGS | NFS4_ACE_INHERIT_ONLY_ACE | NFS4_ACE_IDENTIFIER_GROUP;

unsafe fn mask_from_posix(perm: u16, flags: u32) -> u32 {
    let mut mask = NFS4_ANYONE_MODE;
    if flags & NFS4_ACL_OWNER != 0 { mask |= NFS4_OWNER_MODE; }
    if perm & ACL_READ != 0 { mask |= NFS4_READ_MODE; }
    if perm & ACL_WRITE != 0 { mask |= NFS4_WRITE_MODE; }
    if perm & ACL_WRITE != 0 && flags & NFS4_ACL_DIR != 0 { mask |= NFS4_ACE_DELETE_CHILD; }
    if perm & ACL_EXECUTE != 0 { mask |= NFS4_EXECUTE_MODE; }
    mask
}

unsafe fn deny_mask_from_posix(perm: u16, flags: u32) -> u32 {
    let mut mask = 0;
    if perm & ACL_READ != 0 { mask |= NFS4_READ_MODE; }
    if perm & ACL_WRITE != 0 { mask |= NFS4_WRITE_MODE; }
    if perm & ACL_WRITE != 0 && flags & NFS4_ACL_DIR != 0 { mask |= NFS4_ACE_DELETE_CHILD; }
    if perm & ACL_EXECUTE != 0 { mask |= NFS4_EXECUTE_MODE; }
    mask
}

unsafe fn low_mode_from_nfs4(perm: u32, mode: *mut u16, flags: u32) {
    let mut write_mode = NFS4_WRITE_MODE;
    if flags & NFS4_ACL_DIR != 0 { write_mode |= NFS4_ACE_DELETE_CHILD; }
    *mode = 0;
    if perm & NFS4_READ_MODE == NFS4_READ_MODE { *mode |= ACL_READ; }
    if perm & write_mode == write_mode { *mode |= ACL_WRITE; }
    if perm & NFS4_EXECUTE_MODE == NFS4_EXECUTE_MODE { *mode |= ACL_EXECUTE; }
}

unsafe fn pace_gt(a: *mut posix_acl_entry, b: *mut posix_acl_entry) -> bool {
    if (*a).e_tag != (*b).e_tag { return (*a).e_tag > (*b).e_tag; }
    if (*a).e_tag == ACL_USER { return uid_gt((*a).e_uid, (*b).e_uid); }
    if (*a).e_tag == ACL_GROUP { return gid_gt((*a).e_gid, (*b).e_gid); }
    false
}

#[repr(C)]
pub struct posix_acl_summary { owner: u16, users: u16, group: u16, groups: u16, other: u16, mask: u16 }

unsafe fn summarize_posix_acl(acl: *mut posix_acl, pas: *mut posix_acl_summary) {
    memset(pas as *mut _, 0, core::mem::size_of::<posix_acl_summary>());
    (*pas).mask = 0o7;
    let mut i = 0;
    while i < (*acl).a_count {
        let pa = (*acl).a_entries.add(i as usize);
        match (*pa).e_tag {
            ACL_USER_OBJ => (*pas).owner = (*pa).e_perm,
            ACL_GROUP_OBJ => (*pas).group = (*pa).e_perm,
            ACL_USER => (*pas).users |= (*pa).e_perm,
            ACL_GROUP => (*pas).groups |= (*pa).e_perm,
            ACL_OTHER => (*pas).other = (*pa).e_perm,
            ACL_MASK => (*pas).mask = (*pa).e_perm,
            _ => {}
        }
        i += 1;
    }
    (*pas).users &= (*pas).mask; (*pas).group &= (*pas).mask; (*pas).groups &= (*pas).mask;
}

#[repr(C)] pub struct posix_ace_state { allow: u32, deny: u32 }
#[repr(C)] pub union posix_user_id { uid: kuid_t, gid: kgid_t }
#[repr(C)] pub struct posix_user_ace_state { id: posix_user_id, perms: posix_ace_state }
#[repr(C)] pub struct posix_ace_state_array { n: i32, aces: [posix_user_ace_state; 0] }
#[repr(C)] pub struct posix_acl_state { valid: u8, owner: posix_ace_state, group: posix_ace_state, other: posix_ace_state, everyone: posix_ace_state, mask: posix_ace_state, users: *mut posix_ace_state_array, groups: *mut posix_ace_state_array }

unsafe fn allow_bits(a: *mut posix_ace_state, mask: u32) { (*a).allow |= mask & !(*a).deny; }
unsafe fn deny_bits(a: *mut posix_ace_state, mask: u32) { (*a).deny |= mask & !(*a).allow; }
unsafe fn add_to_mask(s: *mut posix_acl_state, a: *mut posix_ace_state) { (*s).mask.allow |= (*a).allow; }

unsafe fn sort_pacl_range(pacl: *mut posix_acl, start: i32, end: i32) {
    let mut sorted = false;
    while !sorted { sorted = true; for i in start..end { let a=(*pacl).a_entries.add(i as usize); let b=a.add(1); if pace_gt(a,b) { core::ptr::swap(a,b); sorted=false; } } }
}

pub unsafe fn sort_pacl_range_export(pacl: *mut posix_acl, start: i32, end: i32) { sort_pacl_range(pacl,start,end); }

unsafe fn sort_pacl(pacl: *mut posix_acl) {
    if pacl.is_null() || (*pacl).a_count <= 4 { return; }
    let mut i=1; while (*pacl).a_entries.add(i as usize).as_ref().unwrap().e_tag == ACL_USER { i+=1; }
    sort_pacl_range(pacl,1,i-1); BUG_ON((*pacl).a_entries.add(i as usize).as_ref().unwrap().e_tag != ACL_GROUP_OBJ);
    let j0=i+1; let mut j=j0; while (*pacl).a_entries.add(j as usize).as_ref().unwrap().e_tag == ACL_GROUP { j+=1; }
    sort_pacl_range(pacl,j0,j-1);
}

unsafe fn init_state(state: *mut posix_acl_state, cnt: i32) -> i32 {
    memset(state as *mut _,0,core::mem::size_of::<posix_acl_state>());
    let size=core::mem::size_of::<posix_ace_state_array>() + cnt as usize*core::mem::size_of::<posix_user_ace_state>();
    (*state).users=kzalloc(size,GFP_KERNEL); if (*state).users.is_null() { return -ENOMEM; }
    (*state).groups=kzalloc(size,GFP_KERNEL); if (*state).groups.is_null() { kfree((*state).users as *mut _); return -ENOMEM; } 0
}
unsafe fn free_state(s:*mut posix_acl_state){ kfree((*s).users as *mut _); kfree((*s).groups as *mut _); }

/* The remaining conversion and XDR entry points retain the C ABI and external kernel dependencies. */
extern "C" {
    fn nfs4_acl_nfsv4_to_posix(acl:*mut nfs4_acl,pacl:*mut *mut posix_acl,dpacl:*mut *mut posix_acl,flags:u32)->i32;
}
pub unsafe fn nfsd4_acl_to_attr(type_: enum_nfs_ftype4, acl:*mut nfs4_acl, attr:*mut nfsd_attrs)->__be32 {
    if acl.is_null() { return nfs_ok; }
    let flags=if type_==NF4DIR {NFS4_ACL_DIR} else {0};
    let e=nfs4_acl_nfsv4_to_posix(acl,&mut (*attr).na_pacl,&mut (*attr).na_dpacl,flags);
    if e== -EINVAL { nfserr_attrnotsupp } else { nfserrno(e) }
}

pub unsafe fn nfs4_acl_bytes(entries:i32)->i32 { (core::mem::size_of::<nfs4_acl>() as i32)+entries*(core::mem::size_of::<nfs4_ace>() as i32) }

unsafe fn find_uid(s:*mut posix_acl_state, uid:kuid_t)->i32 { let a=(*s).users; for i in 0..(*a).n { if uid_eq((*a).aces.as_ptr().add(i as usize).as_ref().unwrap().id.uid,uid){return i;} } let i=(*a).n; (*a).n+=1; let x=(*a).aces.as_mut_ptr().add(i as usize); (*x).id.uid=uid; (*x).perms.allow=(*s).everyone.allow; (*x).perms.deny=(*s).everyone.deny; i }
unsafe fn find_gid(s:*mut posix_acl_state,gid:kgid_t)->i32 { let a=(*s).groups; for i in 0..(*a).n { if gid_eq((*a).aces.as_ptr().add(i as usize).as_ref().unwrap().id.gid,gid){return i;} } let i=(*a).n; (*a).n+=1; let x=(*a).aces.as_mut_ptr().add(i as usize); (*x).id.gid=gid; (*x).perms.allow=(*s).everyone.allow; (*x).perms.deny=(*s).everyone.deny; i }
unsafe fn allow_bits_array(a:*mut posix_ace_state_array,m:u32){for i in 0..(*a).n{allow_bits((*a).aces.as_mut_ptr().add(i as usize).cast::<posix_ace_state>().add(1),m);}}
unsafe fn deny_bits_array(a:*mut posix_ace_state_array,m:u32){for i in 0..(*a).n{deny_bits((*a).aces.as_mut_ptr().add(i as usize).cast::<posix_ace_state>().add(1),m);}}

pub unsafe fn nfs4_acl_get_whotype(p:*mut i8,len:u32)->i32 { let maps=[("OWNER@",NFS4_ACL_WHO_OWNER),("GROUP@",NFS4_ACL_WHO_GROUP),("EVERYONE@",NFS4_ACL_WHO_EVERYONE)]; for (s,t) in maps { if s.len() as u32==len && core::slice::from_raw_parts(p as *const u8,len as usize)==s.as_bytes(){return t;} } NFS4_ACL_WHO_NAMED }
pub unsafe fn nfs4_acl_write_who(xdr:*mut xdr_stream,who:i32)->__be32 { let maps=[("OWNER@",NFS4_ACL_WHO_OWNER),("GROUP@",NFS4_ACL_WHO_GROUP),("EVERYONE@",NFS4_ACL_WHO_EVERYONE)]; for (s,t) in maps { if t==who { let p=xdr_reserve_space(xdr,s.len() as i32+4); if p.is_null(){return nfserr_resource;} xdr_encode_opaque(p,s.as_ptr() as *const i8,s.len() as i32); return 0; } } nfserr_serverfault }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
