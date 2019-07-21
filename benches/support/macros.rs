#[macro_export]
macro_rules! bench_func {
    ($b: ident, op => $func: ident, ty => $t: ty) => {{
        const LEN: usize = 1 << 13;
        let elems = <$t as mathbench::RandomVec>::random_vec(0, LEN);
        let mut i = 0;
        $b.iter(|| {
            i = (i + 1) & (LEN - 1);
            unsafe { $func(elems.get_unchecked(i)) }
        })
    }};
    ($b: ident, op => $func: ident, ty1 => $ty1:ty, ty2 => $ty2:ty) => {{
        const LEN: usize = 1 << 7;
        let elems1 = <$ty1 as mathbench::RandomVec>::random_vec(0, LEN);
        let elems2 = <$ty2 as mathbench::RandomVec>::random_vec(1, LEN);
        let mut i = 0;
        for lhs in elems1.iter() {
            $b.iter(|| {
                i = (i + 1) & (LEN - 1);
                unsafe { $func(lhs, elems2.get_unchecked(i)) }
            })
        }
    }};
}

#[macro_export]
macro_rules! bench_unop {
    ($b: ident, op => $unop: ident, ty => $t:ty) => {{
        const LEN: usize = 1 << 13;
        let elems = <$t as mathbench::RandomVec>::random_vec(0, LEN);
        let mut i = 0;
        $b.iter(|| {
            i = (i + 1) & (LEN - 1);
            unsafe { elems.get_unchecked(i).$unop() }
        })
    }};
}

#[macro_export]
macro_rules! by_value {
    ($e:expr) => {*$e};
}

#[macro_export]
macro_rules! by_ref {
    ($e:expr) => {$e};
}

#[macro_export]
macro_rules! bench_binop {
    ($b: ident, op => $binop: ident, ty1 => $ty1:ty, ty2 => $ty2:ty, param => $param:tt) => {{
        const LEN: usize = 1 << 7;
        let elems1 = <$ty1 as mathbench::RandomVec>::random_vec(0, LEN);
        let elems2 = <$ty2 as mathbench::RandomVec>::random_vec(1, LEN);
        let mut i = 0;
        for lhs in elems1.iter() {
            $b.iter(|| {
                i = (i + 1) & (LEN - 1);
                unsafe { lhs.$binop($param!(elems2.get_unchecked(i))) }
            })
        }
    }};
    ($b: ident, op => $binop: ident, ty1 => $ty1:ty, ty2 => $ty2:ty) => {{
        bench_binop!($b, op => $binop, ty1 => $ty1, ty2 => $ty2, param => by_value)
    }};
    // ($b: ident, op => $binop: ident, ty => $ty:ty) => {{
    //     bench_binop!($b, op => $binop, ty1 => $ty, ty2 => $ty)
    // }};
}
