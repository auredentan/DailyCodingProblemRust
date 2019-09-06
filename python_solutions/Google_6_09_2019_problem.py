"""
This problem was asked by Google.

In linear algebra, a Toeplitz matrix is one in which the elements on any given diagonal from top left to bottom right are identical.

Here is an example:

1 2 3 4 8
5 1 2 3 4
4 5 1 2 3
7 4 5 1 2
Write a program to determine whether a given input is a Toeplitz matrix.

"""
import numpy as np


def check_vector_is_identical(vector: list) -> bool:
    to_check = vector[0]
    for elem in vector[1:]:
        if to_check != elem:
            return False
    return True


def is_toeplitz_matrix(matrix: np.array) -> bool:
    n_cols = matrix.shape[1]

    for i in range(1, n_cols - 1):
        diag = np.diagonal(matrix, i)
        opposite = np.diagonal(matrix, -i + 1)
        if not check_vector_is_identical(diag):
            return False

        if not check_vector_is_identical(opposite):
            return False

    return True
