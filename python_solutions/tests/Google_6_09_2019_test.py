import unittest
import numpy as np

from python_solutions.Google_6_09_2019_problem import is_toeplitz_matrix


class Google_6_09_2019_test(unittest.TestCase):
    def test_example_1(self):
        example = np.array(
            [
                [1, 2, 3, 4, 8],
                [5, 1, 2, 3, 4],
                [4, 5, 1, 2, 3],
                [7, 4, 5, 1, 2],
            ]
        )
        is_it = is_toeplitz_matrix(example)
        self.assertTrue(is_it)

    def test_example_fail(self):
        example = np.array(
            [
                [1, 3, 3, 4, 8],
                [5, 1, 2, 3, 4],
                [4, 5, 1, 2, 3],
                [7, 4, 5, 1, 2],
            ]
        )
        is_it = is_toeplitz_matrix(example)
        self.assertFalse(is_it)
