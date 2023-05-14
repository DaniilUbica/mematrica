pub mod matrix {
    use num::{Num, traits::Pow};
    pub use std::ops::Add;

    use crate::{CMatrix, CMatrixTrait};

    pub trait Matrix<T: Num + Default + Clone + PartialOrd> {
        fn transpose(&mut self) -> &mut Self {
            let r = self.get_rows();
            let c = self.get_columns();

            println!("{}", self.get_columns());

            let v = self.get_elements();
            let mut t = vec![];
            
            for i in 0..r {
                for j in 0..c {
                    t.push(v[i][j].clone());
                }
            }

            self.get_elements().clear();

            let mut q = vec![];
            let mut a = vec![];

            let mut i = 0;
            let mut j = 0;
            for _ in 0..c {
                while i < t.len() {
                    q.push(t[i].clone());
                    i += c;
                }
                j += 1;
                a.push(q.clone());
                q.clear();
                i = 0;
                i += j
            }

            self.set_elements(a)
        }
        fn resize(&mut self) -> &mut Self;

        fn multiplicate<M>(&mut self, rhs: M) -> CMatrix<T> 
        where
            M: Matrix<T>,
        {

            if self.get_columns() != rhs.get_rows() {
                panic!("Can't multiplicate this matrices: self.columns != rhs.rows");
            }

            let mut v = vec![];

            for i in 0..self.get_rows() {
                if v.len() < self.get_rows() {
                    v.push(vec![]);
                }

                for _j in 0..rhs.get_columns() {
                    if v[i].len() < rhs.get_columns() {
                        v[i].push(T::zero());
                    }
                }
            }

            for i in 0..self.get_rows() {
                for j in 0..rhs.get_columns() {
                    for k in 0..self.get_columns() {
                        v[i][j] = v[i][j].clone() + self.get_elements()[i][k].clone() * rhs.get_elements()[k][j].clone();
                    }
                }
            }

            let mut c = CMatrix::one(self.get_rows(), rhs.get_columns());

            c.set_elements(v);

            c
        }

        fn get_rows(&self) -> usize;
        fn get_columns(&self) -> usize;
        fn get_elements(&self) -> Vec<Vec<T>>;

        fn set_elements(&mut self, v: Vec<Vec<T>>) -> &mut Self;
    }
}