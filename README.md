# sudoku-rs
Can generate every complete sudoku boards there is, because the generator uses a tree structure that tries every combination.
It can also solve any sudoku board using brute force.


# Example
  4   |       | 9 6 5 
      |   5   | 4     
  8 6 |       | 1     
- - - + - - - + - - -
1     |     9 | 2     
      |   6   |       
    2 | 3     |   4   
- - - + - - - + - - -
      |   7 2 | 5   8 
  3   |       |   1   
6     |       |       


# Example Solution
3 4 1 | 7 2 8 | 9 6 5 
2 7 9 | 1 5 6 | 4 8 3 
5 8 6 | 9 4 3 | 1 2 7 
- - - + - - - + - - -
1 5 3 | 4 8 9 | 2 7 6 
4 9 8 | 2 6 7 | 3 5 1 
7 6 2 | 3 1 5 | 8 4 9 
- - - + - - - + - - -
9 1 4 | 6 7 2 | 5 3 8 
8 3 7 | 5 9 4 | 6 1 2 
6 2 5 | 8 3 1 | 7 9 4 

