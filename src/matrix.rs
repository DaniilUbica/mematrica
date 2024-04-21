pub mod matrix {
    extern crate num;

    use self::num::Num;
    pub use std::ops::Add;

    use crate::{CMatrix, CMatrixTrait};

    /// An error
    #[derive(Clone, Debug)]
    pub struct Error(pub String);

    impl std::fmt::Display for Error {
        #[inline]
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            self.0.fmt(formatter)
        }
    }

    impl std::error::Error for Error {
        #[inline]
        fn description(&self) -> &str {
            &self.0
        }
    }

    pub trait Matrix<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> {
        /// Transpose matrix
        fn transpose(&mut self)
        where
            Self: Sized,
        {
            let r = self.get_rows();
            let c = self.get_columns();
            let elems = self.get_elements();
            let mut transposed = vec![vec![elems[0][0]; r]; c];

            for (i, row) in elems.iter().enumerate() {
                for (j, &item) in row.iter().enumerate() {
                    transposed[j][i] = item;
                }
            }
        
            self.set_elements(transposed);
        }

        /// Changes size of matrix, if it was formated. It calls automatically
        fn resize(&mut self);

        /// Checks size of matrix, if it was formated. It calls automatically
        fn check_size(&self);

        /// Multiplies a matrix by another matrix
        fn multiplicate<M>(&self, rhs: M) -> CMatrix<T>
        where
            M: Matrix<T>,
        {
            if self.get_columns() != rhs.get_rows() {
                panic!("Can't multiplicate this matrices: self.columns != rhs.rows");
            }

            let mut result = CMatrix::zero(self.get_rows(), rhs.get_columns());

            for i in 0..self.get_rows() {
                for j in 0..rhs.get_columns() {
                    for k in 0..self.get_columns() {
                        result[i][j] = result[i][j] + self.get_elements()[i][k] * rhs.get_elements()[k][j];
                    }
                }
            }
        
            result
        }

        /// Try to multiplicate matrices
        fn try_multiplicate<M>(&self, rhs: M) -> Result<CMatrix<T>, Error>
        where
            M: Matrix<T>,
        {
            if self.get_columns() != rhs.get_rows() {
                return Err(Error(String::from(
                    "Can't multiplicate this matrices: self.columns != rhs.rows",
                )));
            }

            let mut result = CMatrix::zero(self.get_rows(), rhs.get_columns());

            for i in 0..self.get_rows() {
                for j in 0..rhs.get_columns() {
                    for k in 0..self.get_columns() {
                        result[i][j] = result[i][j] + self.get_elements()[i][k] * rhs.get_elements()[k][j];
                    }
                }
            }
        
            Ok(result)
        }

        /// Counts determinant of matrix
        fn det(&self) -> T {
            self.check_size();
            if self.get_rows() != self.get_columns() {
                panic!("Can't find determinant! Maybe rows != columns?");
            }

            let mut mat = self.get_elements();
            let elems = self.get_elements();
            let mut temp = vec![];
            let mut total = T::one();
            let mut det = T::one();

            let n = self.get_rows();

            for _ in 0..n {
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
                        mat[j][k] =
                            (num1.clone() * mat[j][k].clone()) - (num2.clone() * temp[k].clone());
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
            self.check_size();

            det / total
        }
        /// Try to find determinant of matrix
        fn try_det(&self) -> Result<T, Error> {
            if self.get_rows() != self.get_columns() {
                return Err(Error(String::from(
                    "Can't find determinant! Maybe rows != columns?",
                )));
            }

            let mut mat = self.get_elements();
            let elems = self.get_elements();
            let mut temp = vec![];
            let mut total = T::one();
            let mut det = T::one();

            let n = self.get_rows();

            for _ in 0..n {
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
                        mat[j][k] =
                            (num1.clone() * mat[j][k].clone()) - (num2.clone() * temp[k].clone());
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
            self.check_size();

            Ok(det / total)
        }

        /// Counts inversed matrix
        fn inverse(&mut self) {
            let rows = self.get_rows();
            let columns = self.get_columns();
            let mut aug_matrix = self.get_elements();

            if rows != columns {
                panic!("Can't inverse this matrix! Maybe rows != columns?");
            }

            if self.det() == T::zero() {
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
                        aug_matrix[max_row][i] =
                            aug_matrix[max_row][i].clone() * (T::zero() - T::one());
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
                    aug_matrix[i][j] = aug_matrix[i][j].clone() / pivot.clone();
                }
                for j in 0..rows {
                    id_matrix[i][j] = id_matrix[i][j].clone() / pivot.clone();
                }

                for j in 0..rows {
                    if j != i {
                        let factor = aug_matrix[j][i].clone();
                        for k in i..rows {
                            aug_matrix[j][k] = aug_matrix[j][k].clone()
                                - factor.clone() * aug_matrix[i][k].clone();
                        }
                        for k in 0..rows {
                            id_matrix[j][k] =
                                id_matrix[j][k].clone() - factor.clone() * id_matrix[i][k].clone();
                        }
                    }
                }
            }

            self.set_elements(id_matrix);
            self.check_size();
        }

        /// Try to count inversed matrix
        fn try_inverse(&mut self) -> Result<(), Error> {
            let rows = self.get_rows();
            let columns = self.get_columns();
            let mut aug_matrix = self.get_elements();

            if rows != columns {
                return Err(Error(String::from(
                    "Can't inverse this matrix! Maybe rows != columns?",
                )));
            }

            if self.det() == T::zero() {
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
                        aug_matrix[max_row][i] =
                            aug_matrix[max_row][i].clone() * (T::zero() - T::one());
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
                    aug_matrix[i][j] = aug_matrix[i][j].clone() / pivot.clone();
                }
                for j in 0..rows {
                    id_matrix[i][j] = id_matrix[i][j].clone() / pivot.clone();
                }

                for j in 0..rows {
                    if j != i {
                        let factor = aug_matrix[j][i].clone();
                        for k in i..rows {
                            aug_matrix[j][k] = aug_matrix[j][k].clone()
                                - factor.clone() * aug_matrix[i][k].clone();
                        }
                        for k in 0..rows {
                            id_matrix[j][k] =
                                id_matrix[j][k].clone() - factor.clone() * id_matrix[i][k].clone();
                        }
                    }
                }
            }
            self.set_elements(id_matrix);
            Ok(self.check_size())
        }

        /// Writes matrix to file
        fn to_file(&self, filename: String, delimiter: char) {
            use std::fs::OpenOptions;
            use std::io::Write;

            let rows = self.get_rows();
            let columns = self.get_columns();
            let m = self.get_elements();

            let mut file = OpenOptions::new()
                .write(true)
                .read(true)
                .open(filename.clone())
                .expect(&format!("Can't open file with filename {filename}"));

            for i in 0..rows {
                for j in 0..columns {
                    if i == rows - 1 && j == columns - 1 {
                        write!(file, "{:?}", m[i][j])
                            .expect(&format!("Can't write to file with filename {filename}"));
                    } else {
                        write!(file, "{:?}{delimiter}", m[i][j])
                            .expect(&format!("Can't write to file with filename {filename}"));
                    }
                }
            }
            self.check_size();
        }

        /// Try to write matrix to file
        fn try_to_file(&self, filename: String, delimiter: char) -> Result<(), Error> {
            use std::fs::OpenOptions;
            use std::io::Write;

            let rows = self.get_rows();
            let columns = self.get_columns();
            let m = self.get_elements();

            let file = OpenOptions::new()
                .write(true)
                .read(true)
                .open(filename.clone());

            let mut file = match file {
                Ok(file) => file,
                Err(_) => return Err(Error(format!("Can't open file with filename '{filename}'"))),
            };

            for i in 0..rows {
                for j in 0..columns {
                    if i == rows - 1 && j == columns - 1 {
                        let res = write!(file, "{:?}", m[i][j]);
                        match res {
                            Ok(_) => (),
                            Err(_) => {
                                return Err(Error(format!(
                                    "Can't write to file with filename '{filename}'"
                                )))
                            }
                        }
                    } else {
                        let res = write!(file, "{:?}{delimiter}", m[i][j]);
                        match res {
                            Ok(_) => (),
                            Err(_) => {
                                return Err(Error(format!(
                                    "Can't write to file with filename '{filename}'"
                                )))
                            }
                        }
                    }
                }
            }
            self.check_size();
            Ok(())
        }

        /// Counts norm of matrix
        fn norm(&self) -> f64 {
            let rows = self.get_rows();
            let columns = self.get_columns();
            let elems = self.get_elements();
            let mut norm = 0.0;

            for i in 0..rows {
                for j in 0..columns {
                    norm = norm + elems[i][j].clone().into() * elems[i][j].clone().into();
                }
            }
            self.check_size();
            norm.sqrt()
        }

        /// Converts matrix' values to f64
        fn to_f64(&mut self) -> CMatrix<f64> {
            self.check_size();
            let rows = self.get_rows();
            let columns = self.get_columns();
            let elems = self.get_elements();
            let mut f64_elems = vec![];

            for i in 0..rows {
                let mut c = vec![];
                for j in 0..columns {
                    c.push(elems[i][j].clone().into())
                }
                f64_elems.push(c);
            }

            let mut r = CMatrix::from_element(rows, columns, 0.0);
            r.set_elements(f64_elems);
            r
        }
        
        fn try_remove_row(&self, index: usize) -> Result<CMatrix<T>, Error> {
            let mut elems = self.get_elements();
            let rows = self.get_rows();

            if rows <= index {
                return Err(Error(String::from("Wrong index value!")));
            }

            elems.remove(index);
            let mut c = CMatrix::zero(rows - 1, self.get_columns());
            c.set_elements(elems);
            Ok(c)
        }

        fn try_remove_column(&self, index: usize) -> Result<CMatrix<T>, Error> {
            let mut elems = self.get_elements();
            let columns = self.get_columns();

            if columns <= index {
                return Err(Error(String::from("Wrong index value!")));
            }

            for i in 0..self.get_rows() {
                elems[i].remove(index);
            }
            let mut c = CMatrix::zero(self.get_rows(), columns - 1);
            c.set_elements(elems);
            Ok(c)
        }

        fn try_insert_row(&self, index: usize, row: Vec<T>) -> Result<CMatrix<T>, Error> {
            let mut elems = self.get_elements();
            let columns = self.get_columns();
            let rows = self.get_rows();

            if row.len() != columns {
                return Err(Error(String::from("Wrong row size!")));
            }

            elems.insert(index, row);
            let mut c = CMatrix::zero(rows + 1, columns);
            c.set_elements(elems);
            Ok(c)
        }

        fn try_insert_column(&self, index: usize, column: Vec<T>) -> Result<CMatrix<T>, Error> {
            let mut elems = self.get_elements();
            let columns = self.get_columns();
            let rows = self.get_rows();

            if column.len() != columns {
                return Err(Error(String::from("Wrong column size!")));
            }

            for i in 0..columns {
                elems[i].insert(index, column[i]);
            }
            let mut c = CMatrix::zero(rows + 1, columns);
            c.set_elements(elems);
            Ok(c)
        }

        fn try_get_rows(&self, index: usize, amount: usize) -> Result<CMatrix<T>, Error> {
            let mut elems = self.get_elements();
            let columns = self.get_columns();
            let rows = self.get_rows();

            if amount == 0 {
                let mut c = CMatrix::zero(rows + 1, columns);
                c.set_elements(elems);
                return Ok(c);
            }

            if rows <= index {
                return Err(Error(String::from("Wrong index value!")));
            }

            if rows < index + amount {
                return Err(Error(String::from(
                    "Can't take that amount of rows from that index!",
                )));
            }

            for i in 0..index {
                elems.remove(i);
            }
            for i in index + amount..rows {
                elems.remove(i);
            }
            let mut c = CMatrix::zero(rows + 1, columns);
            c.set_elements(elems);
            Ok(c)
        }

        fn try_get_columns(&self, index: usize, amount: usize) -> Result<CMatrix<T>, Error> {
            let mut elems = self.get_elements();
            let columns = self.get_columns();
            let rows = self.get_rows();

            if amount == 0 {
                let mut c = CMatrix::zero(rows + 1, columns);
                c.set_elements(elems);
                return Ok(c);
            }

            if rows <= index {
                return Err(Error(String::from("Wrong index value!")));
            }

            if rows < index + amount {
                return Err(Error(String::from(
                    "Can't take that amount of columns from that index!",
                )));
            }
            for j in 0..columns {
                for i in 0..index {
                    elems[j].remove(i);
                }
            }
            for j in 0..columns {
                for i in index + amount..columns {
                    elems[j].remove(i);
                }
            }
            let mut c = CMatrix::zero(rows + 1, columns);
            c.set_elements(elems);
            Ok(c)
        }

        fn try_replace_row(&self, index: usize, row: Vec<T>) -> Result<CMatrix<T>, Error> {
            let mut elems = self.get_elements();
            let columns = self.get_columns();
            let rows = self.get_rows();

            if rows <= index {
                return Err(Error(String::from("Wrong index value!")));
            }
            if row.len() != columns {
                return Err(Error(String::from("Wrong row size!")));
            }

            elems.remove(index);
            elems.insert(index, row);
            let mut c = CMatrix::zero(rows + 1, columns);
            c.set_elements(elems);
            Ok(c)
        }

        fn try_replace_column(&self, index: usize, column: Vec<T>) -> Result<CMatrix<T>, Error> {
            let mut elems = self.get_elements();
            let columns = self.get_columns();
            let rows = self.get_rows();

            if columns <= index {
                return Err(Error(String::from("Wrong index value!")));
            }
            if column.len() != columns {
                return Err(Error(String::from("Wrong row size!")));
            }
            for i in 0..rows {
                elems[i].remove(index);
                elems[i].insert(index, column[i]);
            }

            let mut c = CMatrix::zero(rows + 1, columns);
            c.set_elements(elems);
            Ok(c)
        }
        /// Checks if this elem contains to matrix
        fn contains(&self, element: T) -> bool {
            let elems = self.get_elements();

            for i in elems {
                if i.contains(&element) {
                    return true;
                }
            }
            false
        }
        /// Search elemnt's position in matrix. Return (-1, -1) if there is no this element in matrix
        fn find(&self, element: T) -> (i32, i32) {
            let elems = self.get_elements();
            let mut c = 0;
            let mut r = 0;

            for i in elems {
                c += 1;
                if i.contains(&element) {
                    r = i.iter().position(|&r| r == element).unwrap() as i32;
                    break;
                }
                r = -1;
            }


            (r, c)
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
