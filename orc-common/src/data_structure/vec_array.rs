/// A vector of non-growable arrays `Vec<Array<I>>`
#[derive(Debug)]
pub struct VecArray<I> {
    indices: Vec<usize>,
    data: Vec<I>,
}

impl<I> std::ops::Index<usize> for VecArray<I> {
    type Output = [I];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        let end = self.indices[index + 1];
        let start = self.indices[index];
        &self.data[start..end]
    }
}

impl<I> std::ops::IndexMut<usize> for VecArray<I> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let end = self.indices[index + 1];
        let start = self.indices[index];
        &mut self.data[start..end]
    }
}

impl<I> VecArray<I> {
    pub fn new(len: usize) -> VecArrayConstruction<I> {
        let mut indices = Vec::with_capacity(len + 1);
        indices.push(0);
        let data = Vec::new();
        let frozen_vec_vec = VecArray { indices, data };
        VecArrayConstruction {
            vec_array: frozen_vec_vec,
            start_index: 0,
            n_cur_items: 0,
        }
    }

    #[inline]
    pub fn everything(&self) -> &[I] {
        &self.data
    }

    #[inline]
    pub fn array_count(&self) -> usize {
        self.indices.len() - 1
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &[I]> {
        self.indices
            .array_windows()
            .map(|&[start, end]| &self.data[start..end])
    }
}

#[derive(Debug)]
pub struct VecArrayConstruction<I> {
    vec_array: VecArray<I>,
    start_index: usize,
    n_cur_items: usize,
}

impl<I> VecArrayConstruction<I> {
    #[inline]
    pub fn add_item_to_array(&mut self, item: I) {
        self.vec_array.data.push(item);
        self.n_cur_items += 1;
    }

    #[inline]
    pub fn push_array(&mut self, items: impl Iterator<Item = I>) {
        debug_assert_eq!(self.n_cur_items, 0);
        let old_len = self.vec_array.data.len();
        self.vec_array.data.extend(items);
        self.start_index += self.vec_array.data.len() - old_len;
        self.vec_array.indices.push(self.start_index);
    }

    #[inline]
    pub fn done_with_array(&mut self) {
        self.start_index += std::mem::take(&mut self.n_cur_items);
        self.vec_array.indices.push(self.start_index);
    }

    #[inline]
    pub fn done(self) -> VecArray<I> {
        self.vec_array
    }

    #[inline]
    pub fn get_constructed(&self, index: usize) -> &[I] {
        if index + 1 >= self.vec_array.indices.len() {
            panic!("the entry for {index} is still under construction")
        }
        &self.vec_array[index]
    }
}