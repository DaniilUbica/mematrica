pub mod matrix {
    use num::Num;
    pub use std::ops::Add;

    use crate::{CMatrix, CMatrixTrait};

    pub trait Matrix<T: Num + Default + Clone + PartialOrd + std::str::FromStr> {

        /// Transpose matrix
        fn transpose(&mut self)
        where Self: Sized,
        {
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
                i += j;
            }

            self.set_elements(a);
        }

        /// Changes size of matrix, if it was formated
        fn resize(&mut self);

        /// Multiplies a matrix by another matrix
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

        /// Counts determinant of matrix
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

        /// Counts inversed matrix
        fn inverse(&mut self) -> &mut Self {
            let rows = self.get_rows();
            let columns = self.get_columns();
            let mut aug_matrix = self.get_elements();

            if (rows != columns) {
                panic!("Can't inverse this matrix! Maybe rows != columns?");
            }

            if (self.det() == T::zero()) {
                panic!("Can't inverse this matrix! determinant = 0");
            }

            let mut id_matrix = CMatrix::<T>::identity(rows, columns).get_elements();
            for i in 0..rows {
                let mut max_row = i;
                for j in i + 1..rows {
                    if aug_matrix[j][i] < T::zero() {
                        aug_matrix[j][i] = aug_matrix[j][i].clone() * (T::zero() - T::one());
                    }
                    if aug_matrix[max_row][i] < T::zero() {
                        aug_matrix[max_row][i] = aug_matrix[max_row][i].clone() * (T::zero() - T::one());
                    }
                    if aug_matrix[j][i] > aug_matrix[max_row][i] {
                        max_row = j;
                    }
                }
                if max_row != i {
                    aug_matrix.swap(i, max_row);
                    id_matrix.swap(i, max_row);
                }

                let pivot = aug_matrix[i][i].clone();

                for j in i..rows {
                    aug_matrix[i][j] =  aug_matrix[i][j].clone() / pivot.clone();
                }
                for j in 0..rows {
                    id_matrix[i][j] = id_matrix[i][j].clone() / pivot.clone();
                }
                
                for j in 0..rows {
                    if j != i {
                        let factor = aug_matrix[j][i].clone();
                        for k in i..rows {
                            aug_matrix[j][k] = aug_matrix[j][k].clone() - factor.clone() * aug_matrix[i][k].clone();
                        }
                        for k in 0..rows {
                            id_matrix[j][k] = id_matrix[j][k].clone() - factor.clone() * id_matrix[i][k].clone();
                        }
                    }
                }
            }

            self.set_elements(id_matrix);
            self
        }

        /// Returns rows amount
        fn get_rows(&self) -> usize;
        /// Returns columns amount
        fn get_columns(&self) -> usize;
        /// Returns elements of matrix as Vec<Vec>
        fn get_elements(&self) -> Vec<Vec<T>>;

        /// Set elements to matrix
        fn set_elements(&mut self, v: Vec<Vec<T>>);
    }
}