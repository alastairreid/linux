#include <linux/kernel.h>
#include <linux/sched/signal.h>
// #include <linux/compiler_types.h>

// note that the following cannot be written in C because of the
// varargs arguments
int printk(const char *fmt, ...) {
    // The following implementation looks promising but, because it just
    // prints the format string, all you see is something like "6%s: %.*s:0"
    // which is not as useful as I had hoped.
    extern void klee_print_expr(const char *msg, int _dummy);
    klee_print_expr(fmt, 0);
    return 0;
}

unsigned long rust_helper_copy_from_user(void *to, const void __user *from, unsigned long n)
{
	return 0;
}

unsigned long rust_helper_copy_to_user(void __user *to, const void *from, unsigned long n)
{
	return 0;
}

void rust_helper_init_wait(struct wait_queue_entry *wq_entry)
{
}

void *rust_helper_kmap(struct page *page)
{
	return 0;
}

void rust_helper_kunmap(struct page *page)
{
}

int rust_helper_signal_pending(void)
{
    return 0;
}

struct page *rust_helper_alloc_pages(gfp_t gfp_mask, unsigned int order)
{
	return 0;
}

void rust_helper_spin_lock_init(spinlock_t *lock, const char *name,
				struct lock_class_key *key)
{
}

void rust_helper_spin_lock(spinlock_t *lock)
{
}

void rust_helper_spin_unlock(spinlock_t *lock)
{
}

int rust_helper_current_pid(void)
{
	return 0;
}
