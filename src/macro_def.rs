#[macro_export]
macro_rules! mat2 {
    [$m1: expr, $m2: expr, $m3: expr, $m4: expr] => {
        Matrix2::new($m1, $m2, $m3, $m4)
    };
}

#[macro_export]
macro_rules! mat3 {
    [$m1: expr, $m2: expr, $m3: expr, $m4: expr, $m5: expr, $m6: expr, $m7: expr, $m8: expr, $m9: expr] => {
        Matrix3::new($m1, $m2, $m3, $m4, $m5, $m6, $m7, $m8, $m9)
    };
}

#[macro_export]
macro_rules! cmat {
    ($( $( $x: expr ),* ); *) => {
        {
        let mut e = vec![];
        let mut r = 0;
        let mut c = 0;
        let mut f = true;
        $(
            let mut v = vec![];
            let mut t = 0;
            $(
                v.push($x);
                t += 1;
                if f {
                    c += 1;
                }
            )*
            f = false;
            assert_eq!(t, c);
            t = 0;
            c = v.len();
            e.push(v);
            r += 1;
        )*

        let mut c: CMatrix<_> = CMatrix::zero(r, c);
        c.set_elements(e);
        c
    }
    };
}