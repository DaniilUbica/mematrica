pub mod matrix {
    use num::Num;
    pub use std::ops::Add;

    use crate::{CMatrix, CMatrixTrait};

    pub trait Matrix<T: Num + Default + Clone + PartialOrd + std::str::FromStr> {
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

        fn det(&self) -> T {

            if self.get_rows() != self.get_columns() {
                panic!("Can't find determinant! Maybe rows != columns?");
            }

            let mut mat = self.get_elements();
            let elems = self.get_elements();
            let mut temp = vec![];
            let mut total = T::one();
            let mut det = T::one();

            let n = self.get_rows();

            for i in 0..n {
                temp.push(T::zero());
            }

            for i in 0..n {
                let mut index = i;
                while index < n && elems[index][i] == T::zero() {
                    index += 1;
                }

                if index == n {
                    continue;
                }

                if index != i {
                    for j in 0..n {
                        (mat[index][j], mat[i][j]) = (mat[i][j].clone(), mat[index][j].clone());
                    }

                    let exp = index - i;
                    if exp % 2 == 1 {
                        det = det * (T::zero() - T::one());
                    }
                }

                for j in 0..n {
                    temp[j] = mat[i][j].clone();
                }

                for j in (i + 1)..n {
                    let num1 = temp[i].clone();
                    let num2 = mat[j][i].clone();
    
                    for k in 0..n {
                        mat[j][k] = (num1.clone() * mat[j][k].clone()) - (num2.clone() * temp[k].clone());
                    }
    
                    total = total * num1;
                }
            }

            for i in 0..n {
                det = det * mat[i][i].clone();
            }
    
            if total == T::zero() {
                total = T::one();
            }

            det / total
        }

        fn inverse(&mut self) -> &mut Self {
            todo!();
        }

        fn get_rows(&self) -> usize;
        fn get_columns(&self) -> usize;
        fn get_elements(&self) -> Vec<Vec<T>>;

        fn set_elements(&mut self, v: Vec<Vec<T>>) -> &mut Self;
    }
}