/// Implementation of two phase minimisation simplex algorithm
/// Currently it starts from the tableau and solves the problem
///
use std::fmt::{self, Display};


#[derive(Debug, PartialEq)]
enum Phase {
  One,
  Two,
}

// add equality
#[derive(Debug, PartialEq)]
pub struct Matrix {
  phase: Phase,
  artificials: usize,
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
  pub fn new(rows: usize, cols: usize, artificials: usize) -> Self {
    Matrix { phase: Phase::One, rows, cols, data: vec![0.0; rows * cols], artificials }
  }

  pub fn get(&self, row: usize, col: usize) -> f64 {
    self.data[row * self.cols + col]
  }

  pub fn set(&mut self, i: usize, j: usize, val: f64) {
    self.data[i * self.cols + j] = val;
  }

  pub fn add_line(&mut self, line: Vec<f64>) -> &mut Self {
    if self.cols != line.len() {
      panic!("Invalid number of columns");
    }
    self.rows += 1;
    self.data.extend(line);
    self
  }

  pub fn phase_two(&mut self) {
    self.phase = Phase::Two;
  }

  fn find_most_negative_in_bottom_row(&self) -> Option<(usize, f64)> {
    let start_last_row = (self.rows - 1) * self.cols;
    let mut found = None;
    //for (i, &x) in self.data.iter().enumerate().skip(start_last_row) {
    let last_row = &self.data[start_last_row..self.data.len() - 1];
    println!("{:?}", last_row);
    for (i, &x) in last_row.iter().enumerate() {
      if x < 0.0 {
        found = match found {
          Some((_, val)) if x < val => Some((i, x)),
          None => Some((i, x)),
          _ => found,
        };
      }
    }
    found
  }

  fn find_most_positive_in_bottom_row(&self) -> Option<(usize, f64)> {
    let start_last_row = match self.phase {
      Phase::One => (self.rows - 1) * self.cols,
      Phase::Two => (self.rows - 2) * self.cols,
    };
    let mut found = None;
    let limit = match self.phase {
      Phase::One => 1,
      Phase::Two => self.artificials + 1 + self.cols,
    };
    let last_row = &self.data[start_last_row..self.data.len() - limit];
    println!("{:?}", last_row);
    for (i, &x) in last_row.iter().enumerate() {
      if x > 0.0 {
        found = match found {
          Some((_, val)) if x > val => Some((i, x)),
          None => Some((i, x)),
          _ => found,
        };
      }
    }
    found
  }

  fn find_pivot(&self) -> Option<(usize, usize)> {
    let (col, _) = self.find_most_positive_in_bottom_row()?;
    let mut min_ratio = None;
    let mut pivot = None;
    let limit = match self.phase {
      Phase::One => 1,
      Phase::Two => 0,
    };
    for row in 0..self.rows - limit {
      let a = self.get(row, col);
      let b = self.get(row, self.cols - 1);
      // pivot must be positive
      if a > 0.0 && b >= 0.0 {
        let ratio = b / a;
        match min_ratio {
          Some(val) if ratio < val => {
            min_ratio = Some(ratio);
            pivot = Some((row, col));
          }
          None => {
            min_ratio = Some(ratio);
            pivot = Some((row, col));
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

  // FIXME: implement both phases check
  // current one is not enough even for the second phase
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
    let m = Matrix::new(2, 3, 0);
    assert_eq!(m.rows, 2);
    assert_eq!(m.cols, 3);
    assert_eq!(m.data, vec![0.0; 6]);

    println!("{}", m);
  }

  #[test]
  fn test_artificial_variables_stage_1() {
    let mut m = Matrix::new(0, 8, 2);
    m.add_line(vec![1.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0, 1.0]);
    m.add_line(vec![2.0, -1.0, 0.0, -1.0, 0.0, 0.0, 1.0, 1.0]);
    m.add_line(vec![0.0, 3.0, 0.0, 0.0, 1.0, 0.0, 0.0, 2.0]);
    m.add_line(vec![6.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    m.add_line(vec![-3.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, -2.0]);

    println!("{m}");
    m.solve();
    println!("{m}");
  }

  #[test]
  fn test_four_intervals() {
    let mut m = Matrix::new(0, 11, 2);
    m.add_line(vec![1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.5]);
    m.add_line(vec![0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0]);
    m.add_line(vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0]);
    m.add_line(vec![1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 3.0]);
    // the overload constraints
    m.add_line(vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0, 0.0, 1.0]);
    m.add_line(vec![1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0, 2.0]);
    // the objective function (price)
    m.add_line(vec![1.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    // the intermediate objective function
    m.add_line(vec![-2.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, -3.0]);

    println!("{m}");
    m.solve();
    println!("{m}");
  }

  // Tableau for the following minimization problem:
  // maximize p = x + 2y subject to the constraints
  // x <= 1.5
  // y <= 1
  //
  // x >= 1
  // x + y >= 2
  fn tableau_without_max_capacity() -> Matrix{
    let mut m = Matrix::new(0, 9, 2);
    // constraints on max load
    m.add_line(vec![1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.5]);
    m.add_line(vec![0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0]);
    // constraints with artificial variables
    m.add_line(vec![1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0, 0.0, 1.0]);
    m.add_line(vec![1.0, 1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0, 2.0]);
    // objective function
    m.add_line(vec![-1.0, -2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    // Intermediate objective function
    m.add_line(vec![2.0, 1.0, 0.0, 0.0, -1.0, -1.0, 0.0, 0.0, 3.0]);
    m
  }

  #[test]
  fn test_without_max_capacity_individual_steps() {
    let mut m = tableau_without_max_capacity();
    let pivot = m.find_pivot();
    assert_eq!(pivot, Some((2, 0)));
    m.pivot(pivot.unwrap());
    
    let pivot = m.find_pivot();
    assert_eq!(pivot, Some((1, 1)));
    m.pivot(pivot.unwrap());

    let pivot = m.find_pivot();
    assert_eq!(pivot, Some((3, 4)));
    m.pivot(pivot.unwrap());

    // check intermediate objective function is zero
    m.phase_two();
    let pivot = m.find_pivot();
    assert_eq!(pivot, Some((0, 3)));
    m.pivot(pivot.unwrap());

    let solution = m.get_solution();
    assert_eq!(vec![1.5, 0.5], solution[0..2]);
  }

  #[test]
  fn test_without_max_capacity() {
    let mut m = tableau_without_max_capacity();
    println!("{m}");
    m.solve();
    println!("{m}");

    println!("Go to phase 2");
    m.phase_two();
    m.solve();
    println!("{m}");
    let solution = m.get_solution();
    assert_eq!(vec![1.5, 0.5], solution[0..2]);
  }

}
