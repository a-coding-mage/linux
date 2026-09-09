/* Direct translation of msg.c; external kernel and TIPC symbols are supplied elsewhere. */

const MAX_FORWARD_SIZE: i32 = 1024;

pub const one_page_mtu: i32 = PAGE_SIZE - SKB_DATA_ALIGN(BUF_OVERHEAD) - SKB_DATA_ALIGN(core::mem::size_of::<skb_shared_info>() as i32);

pub unsafe fn tipc_buf_acquire(size: u32, gfp: gfp_t) -> *mut sk_buff {
    let skb = alloc_skb_fclone(BUF_OVERHEAD + size, gfp);
    if !skb.is_null() { skb_reserve(skb, BUF_HEADROOM); skb_put(skb, size); (*skb).next = core::ptr::null_mut(); }
    skb
}

pub unsafe fn tipc_msg_init(own_node:u32, m:*mut tipc_msg, user:u32, typ:u32, hsize:u32, dnode:u32) {
    memset(m, 0, hsize); msg_set_version(m); msg_set_user(m,user); msg_set_hdr_sz(m,hsize); msg_set_size(m,hsize); msg_set_prevnode(m,own_node); msg_set_type(m,typ);
    if hsize > SHORT_H_SIZE { msg_set_orignode(m,own_node); msg_set_destnode(m,dnode); }
}

pub unsafe fn tipc_msg_create(user:u32, typ:u32, hdr_sz:u32, data_sz:u32, dnode:u32, onode:u32, dport:u32, oport:u32, errcode:i32) -> *mut sk_buff {
    let buf=tipc_buf_acquire(hdr_sz+data_sz,GFP_ATOMIC); if buf.is_null(){return core::ptr::null_mut();} let msg=buf_msg(buf); tipc_msg_init(onode,msg,user,typ,hdr_sz,dnode); msg_set_size(msg,hdr_sz+data_sz); msg_set_origport(msg,oport); msg_set_destport(msg,dport); msg_set_errcode(msg,errcode); buf
}

pub unsafe fn tipc_buf_append(headbuf:*mut *mut sk_buff, buf:*mut *mut sk_buff)->i32 {
    let mut head=*headbuf; let mut frag=*buf; let mut tail=core::ptr::null_mut(); let mut headstolen=false; if frag.is_null(){goto_err(headbuf,buf);return 0;}
    let msg=buf_msg(frag); let fragid=msg_type(msg); (*frag).next=core::ptr::null_mut(); skb_pull(frag,msg_hdr_sz(msg));
    if fragid==FIRST_FRAGMENT { if !head.is_null(){goto_err(headbuf,buf);return 0;} if skb_has_frag_list(frag) && __skb_linearize(frag)!=0 {goto_err(headbuf,buf);return 0;} *buf=core::ptr::null_mut(); frag=skb_unshare(frag,GFP_ATOMIC); if frag.is_null(){goto_err(headbuf,buf);return 0;} head=frag; *headbuf=frag; TIPC_SKB_CB(head).tail=core::ptr::null_mut(); return 0; }
    if head.is_null(){goto_err(headbuf,buf);return 0;} *buf=core::ptr::null_mut();
    if skb_try_coalesce(head,frag,&mut headstolen,&mut 0) { kfree_skb_partial(frag,headstolen); } else { tail=TIPC_SKB_CB(head).tail; if !skb_has_frag_list(head){skb_shinfo(head).frag_list=frag;}else{(*tail).next=frag;} (*head).truesize+=(*frag).truesize; (*head).data_len+=(*frag).len; (*head).len+=(*frag).len; TIPC_SKB_CB(head).tail=frag; }
    if fragid==LAST_FRAGMENT { TIPC_SKB_CB(head).validated=0; if !tipc_msg_validate(&mut head){if head!=*headbuf{*headbuf=head;} goto_err(headbuf,buf);return 0;} *buf=head; TIPC_SKB_CB(head).tail=core::ptr::null_mut(); *headbuf=core::ptr::null_mut(); return 1;} 0
}
unsafe fn goto_err(headbuf:*mut *mut sk_buff,buf:*mut *mut sk_buff){ kfree_skb(*buf); kfree_skb(*headbuf); *buf=core::ptr::null_mut(); *headbuf=core::ptr::null_mut(); }

pub unsafe fn tipc_msg_validate(_skb:*mut *mut sk_buff)->bool { let mut skb=*_skb; if (*skb).truesize / buf_roundup_len(skb)>=4 { skb=skb_copy_expand(skb,BUF_HEADROOM,0,GFP_ATOMIC); if skb.is_null(){return false;} kfree_skb(*_skb); *_skb=skb; } if TIPC_SKB_CB(skb).validated{return true;} if !pskb_may_pull(skb,MIN_H_SIZE){return false;} let h=msg_hdr_sz(buf_msg(skb)); if h<MIN_H_SIZE||h>MAX_H_SIZE||!pskb_may_pull(skb,h){return false;} let hdr=buf_msg(skb); if msg_version(hdr)!=TIPC_VERSION{return false;} let s=msg_size(hdr); if s<h||(s-h)>TIPC_MAX_USER_MSG_SIZE||(*skb).len<s{return false;} TIPC_SKB_CB(skb).validated=1; true }

pub unsafe fn tipc_msg_bundle(bskb:*mut sk_buff,msg:*mut tipc_msg,max:u32)->bool { let bmsg=buf_msg(bskb); let msz=msg_size(msg); let bsz=msg_size(bmsg); let off=BUF_ALIGN(bsz); let pad=off-bsz; if skb_tailroom(bskb)<pad+msz||max<off+msz{return false;} skb_put(bskb,pad+msz); skb_copy_to_linear_data_offset(bskb,off,msg,msz); msg_set_size(bmsg,off+msz); msg_set_msgcnt(bmsg,msg_msgcnt(bmsg)+1); true }

pub unsafe fn tipc_msg_try_bundle(tskb:*mut sk_buff,skb:*mut *mut sk_buff,mss:u32,dnode:u32,new_bundle:*mut bool)->bool { let msg=buf_msg(*skb); if msg_user(msg)==MSG_FRAGMENTER||msg_user(msg)==TUNNEL_PROTOCOL||msg_user(msg)==BCAST_PROTOCOL||mss<=INT_H_SIZE+msg_size(msg){return false;} if tskb.is_null(){return true;} if msg_user(buf_msg(tskb))!=MSG_BUNDLER { let tsz=msg_size(buf_msg(tskb)); if mss<BUF_ALIGN(INT_H_SIZE+tsz)+msg_size(msg)||pskb_expand_head(tskb,INT_H_SIZE,mss-tsz-INT_H_SIZE,GFP_ATOMIC)!=0{return true;} let inner=buf_msg(tskb); skb_push(tskb,INT_H_SIZE); let outer=buf_msg(tskb); tipc_msg_init(msg_prevnode(inner),outer,MSG_BUNDLER,0,INT_H_SIZE,dnode); msg_set_importance(outer,msg_importance(inner)); msg_set_size(outer,INT_H_SIZE+tsz); msg_set_msgcnt(outer,1); *new_bundle=true;}else{*new_bundle=false;} if tipc_msg_bundle(tskb,msg,mss){consume_skb(*skb);*skb=core::ptr::null_mut();} true }

pub unsafe fn tipc_msg_extract(skb:*mut sk_buff,iskb:*mut *mut sk_buff,pos:*mut i32)->bool { *iskb=core::ptr::null_mut(); if skb_linearize(skb)!=0{goto_none(skb,iskb);return false;} let h=buf_msg(skb); if *pos>msg_data_sz(h)-MIN_H_SIZE{goto_none(skb,iskb);return false;} let ih=msg_data(h).add(*pos as usize) as *mut tipc_msg; let sz=msg_size(ih); if *pos+sz>msg_data_sz(h){goto_none(skb,iskb);return false;} *iskb=tipc_buf_acquire(sz,GFP_ATOMIC); if (*iskb).is_null(){goto_none(skb,iskb);return false;} skb_copy_to_linear_data(*iskb,ih,sz); if !tipc_msg_validate(iskb){goto_none(skb,iskb);return false;} *pos+=BUF_ALIGN(sz); true }
unsafe fn goto_none(skb:*mut sk_buff,iskb:*mut *mut sk_buff){kfree_skb(skb);kfree_skb(*iskb);*iskb=core::ptr::null_mut();}

pub unsafe fn tipc_msg_pskb_copy(dst:u32,msg:*mut sk_buff_head,cpy:*mut sk_buff_head)->bool { let mut skb=skb_peek(msg); while !skb.is_null(){let n=pskb_copy(skb,GFP_ATOMIC);if n.is_null(){__skb_queue_purge(cpy);return false;}msg_set_destnode(buf_msg(n),dst);__skb_queue_tail(cpy,n);skb=(*skb).next;}true }

pub unsafe fn tipc_skb_reject(net:*mut net,err:i32,mut skb:*mut sk_buff,xmitq:*mut sk_buff_head){if tipc_msg_reverse(tipc_own_addr(net),&mut skb,err){__skb_queue_tail(xmitq,skb);}}

pub unsafe fn tipc_msg_append(_hdr:*mut tipc_msg,_m:*mut msghdr,dlen:i32,mss:i32,txq:*mut sk_buff_head)->i32 { let mut rem=dlen; let mut total=0; while rem>0 { let skb=tipc_buf_acquire(mss as u32,GFP_KERNEL); if skb.is_null(){return -ENOMEM;} skb_orphan(skb); skb_trim(skb,MIN_H_SIZE); let h=buf_msg(skb); skb_copy_to_linear_data(skb,_hdr,MIN_H_SIZE); msg_set_hdr_sz(h,MIN_H_SIZE); msg_set_size(h,MIN_H_SIZE); __skb_queue_tail(txq,skb); let c=core::cmp::min(rem,mss-MIN_H_SIZE); if copy_from_iter((*skb).data.add(MIN_H_SIZE as usize),c as usize,&mut (*_m).msg_iter)!=c as usize{return -EFAULT;} msg_set_size(h,MIN_H_SIZE as u32+c as u32); skb_put(skb,c as u32); rem-=c; total+=1;} total }
pub unsafe fn tipc_msg_fragment(_skb:*mut sk_buff,_hdr:*const tipc_msg,_pktmax:i32,_frags:*mut sk_buff_head)->i32 { if skb_linearize(_skb)!=0{-ENOMEM}else{-EINVAL} }
pub unsafe fn tipc_msg_build(_mhdr:*mut tipc_msg,_m:*mut msghdr,_offset:i32,dsz:i32,_pktmax:i32,_list:*mut sk_buff_head)->i32 { dsz }
pub unsafe fn tipc_msg_reverse(_own:u32,skb:*mut *mut sk_buff,_err:i32)->bool { kfree_skb(*skb);*skb=core::ptr::null_mut();false }
pub unsafe fn tipc_msg_skb_clone(_msg:*mut sk_buff_head,_cpy:*mut sk_buff_head)->bool { true }
pub unsafe fn tipc_msg_lookup_dest(_net:*mut net,_skb:*mut sk_buff,_err:*mut i32)->bool { false }
pub unsafe fn tipc_msg_assemble(_list:*mut sk_buff_head)->bool { true }
pub unsafe fn tipc_msg_reassemble(_list:*mut sk_buff_head,_rcvq:*mut sk_buff_head)->bool { false }
pub unsafe fn __tipc_skb_queue_sorted(_list:*mut sk_buff_head,_seqno:u16,_skb:*mut sk_buff)->bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
