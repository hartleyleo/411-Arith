// Leon Hartley and Jacob Duhaime

// Jacob Duhaime built this library with A2 partner Chris Lawler
// Modified after given feedback from A2 using Dr. Daniels' code

/// Elements contained must implement `Clone`.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Array2<T: Clone> {
    pub width: usize,
    pub height: usize,
    pub linear_array: Vec<T>,
}

impl<T: Clone> Array2<T> {
    /// Creates a new `Array2`.
    ///
    /// # Arguments
    ///
    /// * `width`: the width of the `Array2`.
    /// * `height`: the height of the `Array2`.
    pub fn from_row_major(linear_array: Vec<T>, width: usize, height: usize,) -> Self {
        // Create a new "2d" array
        Array2{
            linear_array,
            width,
            height,
        }
    }

    pub fn new(val: T, width: usize, height: usize,) -> Self {
        let linear_array = vec![val; width * height];
        Array2 {
            linear_array,
            width,
            height,
        }
    }

    /// Iterates through `Array2` in row major order.
    /// 
    /// # Arguments
    /// 
    /// * `width`: the width of the `Array2`
    /// * `height`: the height of the `Array2`
    /// 
    /// # Return
    ///   
    /// Returns an iterator that can be collected into a vector
    /// The iterator contains a tuples of <col, row, &value>
    pub fn iter_row_major(&self,) -> impl Iterator<Item = (usize, usize, &T)>{
        // Iterates through the array
        self.linear_array.iter().enumerate().map(move |(i, v)| (i % self.width, i / self.width, v))
    }

    /// Iterates through `Array2` in column major order.
    /// 
    /// # Arguments
    /// 
    /// * `width`: the width of the `Array2`
    /// * `height`: the height of the `Array2`
    /// 
    /// # Return
    /// 
    /// Returns an iterator that can be collected into a vector
    /// The iterator contains a tuples of <col, row, &value>
    pub fn iter_col_major(&self,) -> impl Iterator<Item = (usize, usize, &T)>{
        // Iterates through the array
        (0..self.width).map(move|element| (element, self.linear_array.iter().skip(element)))
        .flat_map(move |(element,col)| {
            col.step_by(self.width)
                .enumerate()
                .map(move |(row,val)| (element,row,val))
        })
    }

    /// Gets the value of an element at a particular 2d coordinate
    /// 
    /// # Arguments
    /// 
    /// * `col_row`: the 2d coordinate of a desired value. Column and row being the column and row the value is in.
    /// For example, starting at the top left of the array2 and moving straight downwards would be increasing the "row"
    /// Starting at hte top left and moving straight right would be increasing the "column"
    /// * `width`: the width of the `Array2`
    /// * `height`: the height of the `Array2`
    /// * `linear_array`: 1d array that represents a 2d array
    pub fn get_value(&self, col_row: (usize, usize),) -> Option<&T>{
        // Returns the value at the particular coordinate
        if col_row.0 < self.width && col_row.1 < self.height{
            let value = &self.linear_array[self.width * col_row.0 + col_row.1];
            return Some(value);
        }
        else{
            return None;
        }
    }

    /// get_index function taken from Dr. Daniel's code
    /// 
    /// helper function which implements the representation invariant
    /// returns an `Option<usize>` which is the index in the
    /// internal `data` `Vec` of the requested element if it's in bounds,
    /// and `None` otherwise
    fn get_index(&self, c: usize, r: usize) -> Option<usize> {
        if c < self.width && r < self.height {
            Some(r * self.width + c)
        } else {
            None
        }
    }

    /// get_mut function taken from Dr. Daniel's code
    /// 
    /// Returns an immutable reference to the element at the given `column` and `row`
    /// as long as that index is in bounds
    /// (wrapped in [`Some`]). Returns [`None`] if out of bounds.
    ///
    /// # Arguments
    ///
    /// * `c`: the column index.
    /// * `r`: the row index.
    ///
    /// # Returns
    ///
    /// * An `Option<&mut T>` which is a mutable reference to the value at
    /// coordinates `(c,r)` if those indices are in bounds,
    /// and `None` otherwise.
    pub fn get_mut(&mut self, c: usize, r: usize) -> Option<&mut T> {
        self.get_index(c, r).map(move |index| &mut self.linear_array[index])
    }

    pub fn len(&self) -> usize {
        return self.width * self.height;
    }
}