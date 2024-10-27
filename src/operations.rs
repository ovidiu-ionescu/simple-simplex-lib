use std::fmt::{self, Display};

// add equality
#[derive(Debug, PartialEq)]
pub struct Matrix {
  pub rows: usize,
  pub cols: usize,
  pub data: Vec<f64>,
}

impl Display for Matrix {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "Matrix {}x{}:", self.rows, self.cols)?;
    for i in 0..self.rows {
      for j in 0..self.cols {
        write!(f, "{:.2}\t", self.data[i * self.cols + j])?;
      }
      writeln!(f)?;
    }
    Ok(())
  }
}

impl Matrix {
  pub fn new(rows: usize, cols: usize) -> Self {
    Matrix { rows, cols, data: vec![0.0; rows * cols] }
  }

  pub fn get(&self, row: usize, col: usize) -> f64 {
    self.data[row * self.cols + col]
  }

  pub fn set(&mut self, i: usize, j: usize, val: f64) {
    self.data[i * self.cols + j] = val;
  }

  pub fn add_line(&mut self, line: Vec<f64>) -> &mut Self {
    self.rows += 1;
    self.data.extend(line);
    self
  }

  fn find_most_negative_in_bottom_row(&self) -> Option<(usize, f64)> {
    let start_last_row = (self.rows - 1) * self.cols;
    let mut found = None;
    for (i, &x) in self.data.iter().enumerate().skip(start_last_row) {
      if x < 0.0 {
        found = match found {
          Some((_, val)) if x < val => Some((i - start_last_row, x)),
          None => Some((i - start_last_row, x)),
          _ => found,
        };
      }
    }
    found
  }

  fn find_pivot(&self) -> Option<(usize, usize)> {
    let (col, _) = self.find_most_negative_in_bottom_row()?;
    let mut min_ratio = None;
    let mut pivot = None;
    for i in 0..self.rows - 1 {
      let a = self.get(i, col);
      let b = self.get(i, self.cols - 1);
      // pivot must be positive
      if a > 0.0 {
        let ratio = b / a;
        match min_ratio {
          Some(val) if ratio < val => {
            min_ratio = Some(ratio);
            pivot = Some((i, col));
          }
          None => {
            min_ratio = Some(ratio);
            pivot = Some((i, col));
          }
          _ => (),
        }
      }
    }
    println!("pivot {:?}", pivot);
    pivot
  }

  fn pivot(&mut self, pivot: (usize, usize)) {
    let (i, j) = pivot;
    let pivot_val = self.get(i, j);
    for k in 0..self.cols {
      self.set(i, k, self.get(i, k) / pivot_val);
    }
    for k in 0..self.rows {
      if k != i {
        let ratio = self.get(k, j);
        for l in 0..self.cols {
          self.set(k, l, self.get(k, l) - ratio * self.get(i, l));
        }
      }
    }
    println!("{self}");
  }

  pub fn solve(&mut self) {
    loop {
      let pivot = self.find_pivot();
      match pivot {
        Some(p) => self.pivot(p),
        None => break,
      }
    }
  }

  pub fn get_solution(&self) -> Vec<f64> {
    let mut solution = vec![0.0; self.cols - 1];
    println!("{}", self.rows);
    // the cleared columns get the solution from the last column
    // the other columns get 0
    for col in 0..self.cols - 1 {
      // the column should contain only one 1, the rest should be 0
      let mut num_zeroes = 0;
      let mut num_ones = 0;
      let mut val = 0.0;
      for row in 0..self.rows {
        if self.get(row, col) == 0.0 {
          num_zeroes += 1;
        } else {
          num_ones += 1;
          val = self.get(row, self.cols - 1);
        }
      }
      if num_zeroes == self.rows - 1 && num_ones == 1 {
        solution[col] = val;
      } else {
        solution[col] = 0.0;
      }
    }
    
    solution
  }

  pub fn check_if_we_have_a_solution(&self, num_vars: usize) -> bool {
    let mut has_solution = true;
    for i in 0..self.rows - 1 {
      let mut count = 0;
      for j in 0..num_vars - 1 {
        if self.get(i, j) == 1.0 {
          count += 1;
        }
      }
      if count > 1 {
        has_solution = false;
        break;
      }
    }
    has_solution
  }

}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_matrix_new() {
    let m = Matrix::new(2, 3);
    assert_eq!(m.rows, 2);
    assert_eq!(m.cols, 3);
    assert_eq!(m.data, vec![0.0; 6]);

    println!("{}", m);
  }

  #[test]
  fn test_elementary_operations() {
    let mut m = Matrix::new(0, 6);
    m.add_line(vec![1.0, 1.0, 1.0, 0.0, 0.0, 12.0]);
    m.add_line(vec![2.0, 1.0, 0.0, 1.0, 0.0, 16.0]);
    m.add_line(vec![-40.0, -30.0, 0.0, 0.0, 1.0, 0.0]);
    println!("{}", m);
    let n = m.find_most_negative_in_bottom_row();
    assert_eq!(n, Some((0, -40.0)));

    let p = m.find_pivot();
    assert_eq!(p, Some((1, 0)));

    m.pivot((1, 0));
    println!("{m}");
  }

  #[test]
  fn test_solve() {
    let mut m = Matrix::new(0, 6);
    m.add_line(vec![1.0, 1.0, 1.0, 0.0, 0.0, 12.0]);
    m.add_line(vec![2.0, 1.0, 0.0, 1.0, 0.0, 16.0]);
    m.add_line(vec![-40.0, -30.0, 0.0, 0.0, 1.0, 0.0]);
    println!("{}", m);
    m.solve();
    println!("{}", m);
    let solution = m.get_solution();
    assert_eq!(solution[0..2], vec![4.0, 8.0]);
    assert_eq!(m.check_if_we_have_a_solution(2), true);
  }

  #[test]
  fn test_negative_coefficient() {
    let mut m = Matrix::new(0, 8);
    m.add_line(vec![7.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 6.0]);
    m.add_line(vec![1.0, 2.0, 0.0, 0.0, 1.0, 0.0, 0.0, 20.0]);
    m.add_line(vec![0.0, 3.0, 4.0, 0.0, 0.0, 1.0, 0.0, 30.0]);
    m.add_line(vec![-1.0, -2.0, -3.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
    println!("{}", m);
    m.solve();
    println!("{}", m);
    let solution = m.get_solution();
    println!("{:?}", solution);
    //assert_eq!(m.check_if_we_have_a_solution(3), true);
  }
}
