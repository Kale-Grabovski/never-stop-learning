fn main() {
	let mut x = 100;
	let r1 = &x;
	let r2 = &x; // ok: multiple shared borrows permitted
	x += 10; // error: cannot assign to x because it's borrowed
	let m = &mut x; // error: cannot borrow x as mutable because it's already borrowed as immutable

	let mut y = 100;
	let s1 = &mut y;
	let s2 = &mut y; // error: cannot borrow mutable more than once
	let z = y; // error: cannot use because it's mutably borrowed

	let mut t = (1, 2);
	let t1 = &t;
	let t0 = &t.0; // ok: reborrowing shared as shared
	let t2 = &mut t.1; // error: cant reborrow shared as mutable


	let mut u = (1, 2);
	let u1 = &u;
	let u0 = &mut u.0; // ok: reborrowing mutable from mutable
	*u0 = 11;
	let ux = &u1.1; // ok: reborrow shared from mutable and doesnt overlap with u0
	u.1; // error: access through other paths is forbidden
}
