#[macro_export]
macro_rules! bench_func {
    ($b: ident, op => $func: ident, ty => $t: ty) => {{
        use rand::{Rng, SeedableRng};
        use rand_xoshiro::Xoshiro256Plus;
        const LEN: usize = 1 << 13;
        let mut rng = Xoshiro256Plus::seed_from_u64(0);
        let elems: Vec<$t> = (0..LEN).map(|_| rng.gen::<$t>().into()).collect();
        let mut i = 0;
        $b.iter(|| {
            i = (i + 1) & (LEN - 1);
            unsafe { criterion::black_box($func(elems.get_unchecked(i))) }
        })
    }};
}

#[macro_export]
macro_rules! bench_unop {
    ($b: ident, op => $unop: ident, ty => $t: ty) => {{
        use rand::{Rng, SeedableRng};
        use rand_xoshiro::Xoshiro256Plus;
        const LEN: usize = 1 << 13;
        let mut rng = Xoshiro256Plus::seed_from_u64(0);
        let elems: Vec<$t> = (0..LEN).map(|_| rng.gen::<$t>().into()).collect();
        let mut i = 0;
        $b.iter(|| {
            i = (i + 1) & (LEN - 1);
            unsafe { criterion::black_box(elems.get_unchecked(i).$unop()) }
        })
    }};
}

#[macro_export]
macro_rules! bench_binop {
    ($b: ident, op => $binop: ident, ty1 => $ty1:ty, ty2 => $ty2:ty) => {{
        use rand::{Rng, SeedableRng};
        use rand_xoshiro::Xoshiro256Plus;
        const LEN: usize = 1 << 7;
        let mut rng = Xoshiro256Plus::seed_from_u64(0);
        let elems1: Vec<$ty1> = (0..LEN).map(|_| rng.gen::<$ty1>().into()).collect();
        let elems2: Vec<$ty2> = (0..LEN).map(|_| rng.gen::<$ty2>().into()).collect();
        let mut i = 0;
        for lhs in elems1.iter() {
            $b.iter(|| {
                i = (i + 1) & (LEN - 1);
                unsafe {
                    criterion::black_box(lhs.$binop(*elems2.get_unchecked(i)));
                }
            })
        }
    }};
}
