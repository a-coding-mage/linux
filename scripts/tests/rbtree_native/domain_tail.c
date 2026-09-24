/* Independent C-defined no-touch/partial initialization/self-alias cases. */
static void domain_noop(struct rb_node *a, struct rb_node *b) { (void)a; (void)b; }
static int domain_tests(void)
{
    struct rb_node n, p, new, sibling;
    struct rb_root root = RB_ROOT;
    struct rb_augment_callbacks cb;
    /* Empty-node iteration reads only its membership word. */
    RB_CLEAR_NODE(&n);
    CHECK(!oracle_rb_next(&n) && !rb_next(&n));
    CHECK(!oracle_rb_prev(&n) && !rb_prev(&n));
    /* Root insertion reads no child pointers and need not touch root. */
    n.__rb_parent_color = 0;
    oracle___rb_insert_augmented(&n, NULL, NULL);
    CHECK(n.__rb_parent_color == RB_BLACK);
    n.__rb_parent_color = 0;
    __rb_insert_augmented(&n, NULL, NULL);
    CHECK(n.__rb_parent_color == RB_BLACK);
    n.__rb_parent_color = 0;
    rb_insert_color(&n, NULL);
    CHECK(n.__rb_parent_color == RB_BLACK);
    /* Postorder completion needs only a null parent word. */
    CHECK(!oracle_rb_next_postorder(&n) && !rb_next_postorder(&n));
    for (int s = 0; s < 2; s++) {
        n = (struct rb_node){ .__rb_parent_color = RB_BLACK };
        root.rb_node = &n;
        if (s) rb_replace_node(&n, &n, &root);
        else oracle_rb_replace_node(&n, &n, &root);
        CHECK(root.rb_node == &n && n.__rb_parent_color == RB_BLACK);
        if (s) rb_replace_node_rcu(&n, &n, &root);
        else oracle_rb_replace_node_rcu(&n, &n, &root);
        CHECK(root.rb_node == &n);
        /* Nonroot replacement may leave root null and new uninitialized. */
        p = (struct rb_node){ .rb_left = &n };
        n.__rb_parent_color = (unsigned long)&p | RB_BLACK;
        if (s) rb_replace_node(&n, &new, NULL);
        else oracle_rb_replace_node(&n, &new, NULL);
        CHECK(p.rb_left == &new && rb_parent(&new) == &p);
        /* Black sibling recoloring reaches neither rotate nor root. */
        p = (struct rb_node){ .rb_right = &sibling };
        sibling = (struct rb_node){ .__rb_parent_color = (unsigned long)&p | RB_BLACK };
        if (s) __rb_erase_color(&p, NULL, NULL);
        else oracle___rb_erase_color(&p, NULL, NULL);
        CHECK(rb_is_black(&p) && rb_is_red(&sibling));
        /* Only propagate is initialized; neither copy nor rotate is reached. */
        n = (struct rb_node){ .__rb_parent_color = RB_BLACK };
        root.rb_node = &n;
        cb.propagate = domain_noop;
        if (s) rust_erase_aug(&n, &root, &cb);
        else oracle_erase_aug(&n, &root, &cb);
        CHECK(!root.rb_node);
    }
    return 0;
}
