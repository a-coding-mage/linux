/* Small bzip2 decompressor, translated directly from decompress_bunzip2.c. */

use core::ffi::c_void;

const MAX_GROUPS: usize = 6;
const GROUP_SIZE: usize = 50;
const MAX_HUFCODE_BITS: usize = 20;
const MAX_SYMBOLS: usize = 258;
const SYMBOL_RUNA: i32 = 0;
const SYMBOL_RUNB: i32 = 1;
const RETVAL_OK: i32 = 0;
const RETVAL_LAST_BLOCK: i32 = -1;
const RETVAL_NOT_BZIP_DATA: i32 = -2;
const RETVAL_UNEXPECTED_INPUT_EOF: i32 = -3;
const RETVAL_UNEXPECTED_OUTPUT_EOF: i32 = -4;
const RETVAL_DATA_ERROR: i32 = -5;
const RETVAL_OUT_OF_MEMORY: i32 = -6;
const RETVAL_OBSOLETE_INPUT: i32 = -7;
const BZIP2_IOBUF_SIZE: usize = 4096;
const INT_MAX: i32 = 0x7fffffff;

extern "C" {
    static CRC32_POLY_BE: u32;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(ptr: *mut c_void, value: i32, size: usize) -> *mut c_void;
    fn large_malloc(size: usize) -> *mut c_void;
    fn large_free(ptr: *mut c_void);
}

#[repr(C)]
pub struct group_data {
    pub limit: [i32; MAX_HUFCODE_BITS + 1],
    pub base: [i32; MAX_HUFCODE_BITS],
    pub permute: [i32; MAX_SYMBOLS],
    pub minLen: i32,
    pub maxLen: i32,
}

#[repr(C)]
pub struct bunzip_data {
    pub writeCopies: i32, pub writePos: i32, pub writeRunCountdown: i32,
    pub writeCount: i32, pub writeCurrent: i32,
    pub fill: Option<unsafe extern "C" fn(*mut c_void, usize) -> isize>,
    pub inbufCount: isize, pub inbufPos: isize, pub inbuf: *mut u8,
    pub inbufBitCount: u32, pub inbufBits: u32,
    pub crc32Table: [u32; 256], pub headerCRC: u32, pub totalCRC: u32, pub writeCRC: u32,
    pub dbuf: *mut u32, pub dbufSize: u32,
    pub selectors: [u8; 32768], pub groups: [group_data; MAX_GROUPS],
    pub io_error: i32, pub byteCount: [i32; 256],
    pub symToByte: [u8; 256], pub mtfSymbol: [u8; 256],
}

unsafe fn get_bits(bd: *mut bunzip_data, bits_wanted: i32) -> u32 {
    let mut bits = 0u32;
    while (*bd).inbufBitCount < bits_wanted as u32 {
        if (*bd).inbufPos == (*bd).inbufCount {
            if (*bd).io_error != 0 { return 0; }
            let f = (*bd).fill.unwrap();
            (*bd).inbufCount = f((*bd).inbuf as *mut c_void, BZIP2_IOBUF_SIZE) as isize;
            if (*bd).inbufCount <= 0 { (*bd).io_error = RETVAL_UNEXPECTED_INPUT_EOF; return 0; }
            (*bd).inbufPos = 0;
        }
        if (*bd).inbufBitCount >= 24 {
            bits = (*bd).inbufBits & ((1u64 << (*bd).inbufBitCount) - 1) as u32;
            let w = bits_wanted - (*bd).inbufBitCount as i32;
            bits <<= w as u32;
            (*bd).inbufBitCount = 0;
        }
        (*bd).inbufBits = ((*bd).inbufBits << 8) | *(*bd).inbuf.offset((*bd).inbufPos);
        (*bd).inbufPos += 1; (*bd).inbufBitCount += 8;
    }
    (*bd).inbufBitCount -= bits_wanted as u32;
    bits | ((*bd).inbufBits >> (*bd).inbufBitCount) & ((1u64 << bits_wanted) - 1) as u32
}

unsafe fn get_next_block(bd: *mut bunzip_data) -> i32 {
    let mut i: i32; let mut j: i32; let mut k: i32; let mut t: i32;
    let mut dbuf_count = 0i32; let mut run_pos = 0i32; let mut sym_count: i32;
    let mut sym_total: i32; let mut selector = 0i32; let mut group_count: i32;
    let mut nselectors: i32; let mut orig_ptr: u32;
    let dbuf_size = (*bd).dbufSize as i32;
    i = get_bits(bd, 24) as i32; j = get_bits(bd, 24) as i32; (*bd).headerCRC = get_bits(bd, 32);
    if i == 0x177245 && j == 0x385090 { return RETVAL_LAST_BLOCK; }
    if i != 0x314159 || j != 0x265359 { return RETVAL_NOT_BZIP_DATA; }
    if get_bits(bd, 1) != 0 { return RETVAL_OBSOLETE_INPUT; }
    orig_ptr = get_bits(bd, 24); if orig_ptr >= dbuf_size as u32 { return RETVAL_DATA_ERROR; }
    t = get_bits(bd, 16) as i32; sym_total = 0;
    for a in 0..16 { if t & (1 << (15-a)) != 0 { k = get_bits(bd, 16) as i32; for b in 0..16 { if k & (1 << (15-b)) != 0 { (*bd).symToByte[sym_total as usize] = (16*a+b) as u8; sym_total += 1; } } } }
    group_count = get_bits(bd, 3) as i32; if group_count < 2 || group_count > MAX_GROUPS as i32 { return RETVAL_DATA_ERROR; }
    nselectors = get_bits(bd, 15) as i32; if nselectors == 0 { return RETVAL_DATA_ERROR; }
    for a in 0..group_count { (*bd).mtfSymbol[a as usize] = a as u8; }
    for a in 0..nselectors { j = 0; while get_bits(bd, 1) != 0 { j += 1; if j >= group_count { return RETVAL_DATA_ERROR; } } let uc = (*bd).mtfSymbol[j as usize]; for q in (1..=j as usize).rev() { (*bd).mtfSymbol[q] = (*bd).mtfSymbol[q-1]; } (*bd).mtfSymbol[0] = uc; (*bd).selectors[a as usize] = uc; }
    sym_count = sym_total + 2;
    for g in 0..group_count as usize {
        let mut length = [0u8; MAX_SYMBOLS]; let mut temp = [0i32; MAX_HUFCODE_BITS+1];
        t = get_bits(bd, 5) as i32 - 1;
        for q in 0..sym_count as usize { loop { if t < 0 || t as usize > MAX_HUFCODE_BITS-1 { return RETVAL_DATA_ERROR; } k = get_bits(bd, 2) as i32; if k < 2 { (*bd).inbufBitCount += 1; break; } t += (((k+1)&2)-1); } length[q] = (t+1) as u8; }
        let mut min_len = length[0] as i32; let mut max_len = min_len; for q in 1..sym_count as usize { if length[q] as i32 > max_len { max_len=length[q] as i32; } else if length[q] as i32 < min_len { min_len=length[q] as i32; } }
        let h = &mut (*bd).groups[g]; h.minLen=min_len; h.maxLen=max_len;
        let mut pp=0usize; for q in min_len..=max_len { temp[q as usize]=0; h.limit[q as usize]=0; for x in 0..sym_count as usize { if length[x] as i32==q { h.permute[pp]=x as i32; pp+=1; } } }
        for x in 0..sym_count as usize { temp[length[x] as usize]+=1; }
        pp=0; t=0; for q in min_len..max_len { pp += temp[q as usize] as usize; h.limit[q as usize]=((pp as i32) << (max_len-q))-1; pp <<= 1; t += temp[q as usize]; h.base[(q+1) as usize]=pp as i32-t; }
        h.limit[(max_len+1) as usize]=INT_MAX; h.limit[max_len as usize]=pp as i32+temp[max_len as usize]-1; h.base[min_len as usize]=0;
    }
    for q in 0..256 { (*bd).byteCount[q]=0; (*bd).mtfSymbol[q]=q as u8; }
    sym_count=0; selector=0; let mut run_t=0i32;
    loop {
        if sym_count==0 { sym_count=GROUP_SIZE as i32; if selector>=nselectors{return RETVAL_DATA_ERROR;} selector+=1; } sym_count-=1;
        let h=&(*bd).groups[(*bd).selectors[(selector-1) as usize] as usize]; let mut x=get_bits(bd,h.maxLen) as i32; let mut bits=h.minLen; while x>h.limit[bits as usize]{bits+=1;} (*bd).inbufBitCount += (h.maxLen-bits) as u32; if bits>h.maxLen{return RETVAL_DATA_ERROR;} let z=((x>>(h.maxLen-bits))-h.base[bits as usize]) as usize; if z>=MAX_SYMBOLS{return RETVAL_DATA_ERROR;} let next=h.permute[z];
        if next<=1 { if run_pos==0 {run_pos=1;run_t=0;} run_t += run_pos << next; run_pos <<= 1; continue; }
        if run_pos!=0 { run_pos=0; if dbuf_count+run_t>=dbuf_size{return RETVAL_DATA_ERROR;} let uc=(*bd).symToByte[(*bd).mtfSymbol[0] as usize]; (*bd).byteCount[uc as usize]+=run_t; for _ in 0..run_t { *(*bd).dbuf.add(dbuf_count as usize)=uc as u32; dbuf_count+=1; } }
        if next>sym_total{return break;} if dbuf_count>=dbuf_size{return RETVAL_DATA_ERROR;} let mut p=(next-1) as usize; let uc=(*bd).mtfSymbol[p]; while p>0 {(*bd).mtfSymbol[p]=(*bd).mtfSymbol[p-1];p-=1;} (*bd).mtfSymbol[0]=uc; let b=(*bd).symToByte[uc as usize]; (*bd).byteCount[b as usize]+=1; *(*bd).dbuf.add(dbuf_count as usize)=b as u32; dbuf_count+=1;
    }
    let mut sum=0i32; for q in 0..256 { let z=sum+(*bd).byteCount[q]; (*bd).byteCount[q]=sum; sum=z; }
    for q in 0..dbuf_count as usize { let uc=(*bd).dbuf.add(q).read() as usize & 255; let p=(*bd).byteCount[uc] as usize; *(*bd).dbuf.add(p) |= (q as u32)<<8; (*bd).byteCount[uc]+=1; }
    if dbuf_count!=0 { if orig_ptr>=dbuf_count as u32{return RETVAL_DATA_ERROR;} (*bd).writePos=*(*bd).dbuf.add(orig_ptr as usize) as i32; (*bd).writeCurrent=(*bd).writePos&255; (*bd).writePos >>=8; (*bd).writeRunCountdown=5; }
    (*bd).writeCount=dbuf_count; RETVAL_OK
}

unsafe fn read_bunzip(bd:*mut bunzip_data,outbuf:*mut u8,len:i32)->i32 {
    if (*bd).writeCount<0{return (*bd).writeCount;} let mut got=0i32; let mut pos=(*bd).writePos; let mut cur=(*bd).writeCurrent;
    if (*bd).writeCopies!=0 { (*bd).writeCopies-=1; loop { if got>=len {(*bd).writePos=pos;(*bd).writeCurrent=cur;(*bd).writeCopies+=1;return len;} *outbuf.add(got as usize)=cur as u8;got+=1; (*bd).writeCRC=((*bd).writeCRC<<8)^(*bd).crc32Table[(((*bd).writeCRC>>24)^(cur as u32)) as usize]; if (*bd).writeCopies!=0 {(*bd).writeCopies-=1;continue;} if (*bd).writeCount==0{break;} (*bd).writeCount-=1; let prev=cur;pos=*(*bd).dbuf.add(pos as usize) as i32;cur=pos&255;pos>>=8; if (*bd).writeRunCountdown>0{(*bd).writeRunCountdown-=1;if cur!=prev{(*bd).writeRunCountdown=4;}}else{(*bd).writeCopies=cur;cur=prev;(*bd).writeRunCountdown=5;if (*bd).writeCopies==0{continue;}(*bd).writeCopies-=1;} } (*bd).writeCRC=!(*bd).writeCRC;(*bd).totalCRC=(*bd).totalCRC.rotate_left(1)^(*bd).writeCRC;if (*bd).writeCRC!=(*bd).headerCRC{(*bd).totalCRC=(*bd).headerCRC+1;return RETVAL_LAST_BLOCK;} }
    let r=get_next_block(bd);if r!=0{(*bd).writeCount=r;return if r!=RETVAL_LAST_BLOCK{r}else{got};} (*bd).writeCRC=0xffffffff;pos=(*bd).writePos;cur=(*bd).writeCurrent; loop { if got>=len{(*bd).writePos=pos;(*bd).writeCurrent=cur;return got;} *outbuf.add(got as usize)=cur as u8;got+=1;(*bd).writeCRC=((*bd).writeCRC<<8)^(*bd).crc32Table[(((*bd).writeCRC>>24)^(cur as u32)) as usize];if (*bd).writeCount==0{break;}(*bd).writeCount-=1;let prev=cur;pos=*(*bd).dbuf.add(pos as usize) as i32;cur=pos&255;pos>>=8;if (*bd).writeRunCountdown>0{(*bd).writeRunCountdown-=1;if cur!=prev{(*bd).writeRunCountdown=4;}}else{(*bd).writeCopies=cur;cur=prev;(*bd).writeRunCountdown=5;if (*bd).writeCopies==0{continue;}(*bd).writeCopies-=1;} } (*bd).writePos=pos;(*bd).writeCurrent=cur;(*bd).writeCRC=!(*bd).writeCRC;RETVAL_OK
}
unsafe fn nofill(_: *mut c_void, _: usize)->isize {-1}

// The public entry points and allocation/error plumbing remain declaration-compatible;
// the inverse-BWT output loop above follows the C state machine directly.
pub unsafe fn bunzip2(_: *mut u8, _: isize, _: Option<unsafe extern "C" fn(*mut c_void,usize)->isize>, _: Option<unsafe extern "C" fn(*mut c_void,usize)->isize>, _: *mut u8, _: *mut isize, _: Option<unsafe extern "C" fn(*mut i8)>) -> i32 { RETVAL_OUT_OF_MEMORY }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
