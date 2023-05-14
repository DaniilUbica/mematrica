pub mod matrix {
    use num::Num;

    pub trait Matrix<T: Num + Default + Clone> {
        fn transpose(&mut self) -> &mut Self {
            let r = self.get_rows();
            let c = self.get_columns();

            let v = self.get_elements();
            let mut t = vec![];
            
            for i in 0..r {
                for j in 0..c {
                    t.push(v[i][j].clone());
                }
            }

            self.get_elements().clear();

            let mut q = vec![];

            let mut i = 0;
            let mut j = 0;
            for _ in 0..c {
                while i < t.len() {
                    q.push(t[i].clone());
                    i += c;
                }
                j += 1;
                self.get_elements().push(q.clone());
                q.clear();
                i = 0;
                i += j
            }

            self.resize()
        }

        fn resize(&mut self) -> &mut Self;

        fn get_rows(&self) -> usize;
        fn get_columns(&self) -> usize;
        fn get_elements(&mut self) -> &mut Vec<Vec<T>>;
    }
}