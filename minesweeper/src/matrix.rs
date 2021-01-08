use std::slice::{Iter, IterMut};

pub struct Matrix<T> {
  #[allow(dead_code)]
  rows: usize,
  cols: usize,
  capacity: usize,
  elements: Vec<T>,
}

impl<T> Matrix<T> {
  pub fn new(rows: usize, cols: usize) -> Self {
    let capacity = rows * cols;

    Self {
      rows,
      cols,
      capacity,
      elements: Vec::with_capacity(capacity),
    }
  }

  pub fn capacity(&self) -> usize {
    self.capacity
  }

  #[allow(dead_code)]
  pub fn get_element(&self, idx: usize) -> Option<&T> {
    self.elements.get(idx)
  }

  pub fn get_element_mut(&mut self, idx: usize) -> Option<&mut T> {
    self.elements.get_mut(idx)
  }

  pub fn idx_above(&self, idx: usize) -> Option<usize> {
    if idx < self.cols {
      return None;
    }

    Some(idx - self.cols)
  }

  pub fn idx_below(&self, idx: usize) -> Option<usize> {
    if idx >= self.capacity - self.cols {
      return None;
    }

    Some(idx + self.cols)
  }

  pub fn idx_left(&self, idx: usize) -> Option<usize> {
    if idx % self.cols == 0 {
      return None;
    }

    Some(idx - 1)
  }

  pub fn idx_right(&self, idx: usize) -> Option<usize> {
    if idx % self.cols == (self.cols - 1) {
      return None;
    }

    Some(idx + 1)
  }

  pub fn adj_idxs(&self, idx: usize) -> Vec<usize> {
    // @TODO: optimize
    let adj = [
      self.idx_above(idx),
      self.idx_below(idx),
      self.idx_left(idx),
      self.idx_right(idx),
      self.idx_above(idx).and_then(|i| self.idx_left(i)),
      self.idx_above(idx).and_then(|i| self.idx_right(i)),
      self.idx_below(idx).and_then(|i| self.idx_left(i)),
      self.idx_below(idx).and_then(|i| self.idx_right(i)),
    ];

    adj
      .iter()
      .filter(|i| i.is_some())
      .map(|i| i.unwrap())
      .collect()
  }

  pub fn iter(&self) -> Iter<T> {
    self.elements.iter()
  }

  pub fn iter_mut(&mut self) -> IterMut<T> {
    self.elements.iter_mut()
  }
  pub fn push(&mut self, value: T) {
    self.elements.push(value);
  }
}

#[test]
fn test_indexing() {
  let mut matrix: Matrix<String> = Matrix::new(2, 2);

  for i in 0..matrix.capacity() {
    matrix.push(format!("element {:?}", i));
  }

  assert_eq!(matrix.idx_left(0), None);
  assert_eq!(matrix.idx_left(1), Some(0));
  assert_eq!(matrix.idx_left(2), None);
  assert_eq!(matrix.idx_left(3), Some(2));

  assert_eq!(matrix.idx_right(0), Some(1));
  assert_eq!(matrix.idx_right(1), None);
  assert_eq!(matrix.idx_right(2), Some(3));
  assert_eq!(matrix.idx_right(3), None);

  assert_eq!(matrix.idx_below(0), Some(2));
  assert_eq!(matrix.idx_below(1), Some(3));
  assert_eq!(matrix.idx_below(2), None);
  assert_eq!(matrix.idx_below(3), None);

  assert_eq!(matrix.idx_above(0), None);
  assert_eq!(matrix.idx_above(1), None);
  assert_eq!(matrix.idx_above(2), Some(0));
  assert_eq!(matrix.idx_above(3), Some(1));

  assert_eq!(matrix.adj_idxs(0), vec![2, 1, 3]);
}
