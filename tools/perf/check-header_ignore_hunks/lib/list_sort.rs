// @@ -50,6 +50,7 @@
//                      struct list_head *a, struct list_head *b)
// {
let mut tail: *mut list_head = head;
let mut count: u8 = 0;

loop {
    /* if equal, take 'a' -- important for sort stability */
}

// @@ -75,6 +76,15 @@
/* Finish linking remainder of list b on to tail */
unsafe {
    (*tail).next = b;
}
loop {
    /*
     * If the merge is highly unbalanced (e.g. the input is
     * already sorted), this loop may run many iterations.
     * Continue callbacks to the client even though no
     * element comparison is needed, so the client's cmp()
     * routine can invoke cond_resched() periodically.
     */
    count = count.wrapping_add(1);
    if unlikely(count == 0) {
        cmp(priv_, b, b);
    }
    unsafe {
        (*b).prev = tail;
    }
    tail = b;
    unsafe {
        b = (*b).next;
    }
}
