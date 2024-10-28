# Two Phase Minimization Simplex

The most basic implementation of the two phase minimization Simplex algorithm.

This implementation is not optimized for performance, but for simplicity and readability.

It starts from the tableau of the problem and iterates (pivots) over the tableau until the solution is found.\
Building the tableau is not part of this implementation.

## Description

The Simplex algorithm is a method for solving linear programming problems.\
The single phase minimisation Simplex requires all inequalities to be of the form >=\
Two phase Simplex is used to solve problems with inequalities of both >= and <=.

In the first phase the algorithm finds a feasible solution, and in the second phase it finds the optimal solution.
Finding the feasible solution is done by minimizing the sum of the artificial variables.

