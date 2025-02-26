use std::cell::Cell;

/// A vector that keeps track of largest and smallest
/// `f64` value to allow access in O(1) time. This
/// data structure is used to mitigate O(n) sort when
/// extreme values are of particular interest.
#[allow(dead_code)]
pub struct MinMaxList {
    // current index
    curr_idx: Cell<usize>,

    // min and max value index
    min_idx: Cell<Option<usize>>,
    max_idx: Cell<Option<usize>>,

    // list of values
    pub list: Vec<Cell<Option<f64>>>,
}


#[allow(dead_code)]
impl MinMaxList {

    pub fn new(size: usize) -> Self {
        return MinMaxList { curr_idx: Cell::new(0), min_idx: Cell::new(None),
                            max_idx: Cell::new(None), list: vec![Cell::new(None); size] };
    }


    /// Updates 'min_idx'.
    fn update_min(&self, value: f64) {
        if let Some(idx) = self.min_idx.get() {
            // if new value is less than old value, replace
            // min idx with current index
            if self.list[idx].get().unwrap() > value {
                self.min_idx.set(Some(self.curr_idx.get()));
            }
        }

        // if 'None', replace with 'Some(curr_idx)'
        else {
            self.min_idx.set(Some(self.curr_idx.get()));
        }
    }


    /// Updates 'max_idx'.
    fn update_max(&self, value: f64) {
        if let Some(idx) = self.max_idx.get() {
            // if new value is greater than old value, replace
            // max idx with current index
            if self.list[idx].get().unwrap() < value {
                self.max_idx.set(Some(self.curr_idx.get()));
            }
        }

        // if 'None', replace with 'Some(curr_idx)'
        else {
            self.max_idx.set(Some(self.curr_idx.get()));
        }
    }



    pub fn append(&self, value: f64) {

        // if max capacity is reached, simply ignore call
        if self.curr_idx.get() < self.list.len() {
            self.update_min(value);
            self.update_max(value);

            // put 'value' into vector and update index
            self.list[self.curr_idx.get()].set(Some(value));
            self.curr_idx.set(self.curr_idx.get() + 1);
        }
    }


    /// Return min value.
    pub fn find_min(&self) -> Option<f64> {

        if let Some(idx) = self.min_idx.get() {
            return self.list[idx].get();
        }

        return None;
    }


    /// Return max value.
    pub fn find_max(&self) -> Option<f64> {

        if let Some(idx) = self.max_idx.get() {
            return self.list[idx].get();
        }

        return None;
    }
}

