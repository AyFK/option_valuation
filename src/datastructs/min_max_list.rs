
/// A vector that keeps track of largest and smallest
/// `f64` value to allow access in O(1) time. This
/// data structure is used to mitigate O(n) sort when
/// extreme values are of particular interest.
#[allow(dead_code)]
pub struct MinMaxList {
    // current index
    curr_idx: usize,

    // min and max value index
    min_idx: Option<usize>,
    max_idx: Option<usize>,

    // list of values
    list: Vec<Option<f64>>,
}


#[allow(dead_code)]
impl MinMaxList {

    pub fn new(size: usize) -> Self {
        return MinMaxList { curr_idx: 0, min_idx: None,
                            max_idx: None, list: vec![None; size] };
    }


    /// Updates 'min_idx'.
    fn update_min(&mut self, value: f64) {
        if let Some(idx) = self.min_idx {
            // if new value is less than old value, replace
            // min idx with current index
            if self.list[idx].unwrap() > value {
                self.min_idx = Some(self.curr_idx);
            }
        }

        // if 'None', replace with 'Some(curr_idx)'
        else {
            self.min_idx = Some(self.curr_idx);
        }
    }


    /// Updates 'max_idx'.
    fn update_max(&mut self, value: f64) {
        if let Some(idx) = self.max_idx {
            // if new value is greater than old value, replace
            // max idx with current index
            if self.list[idx].unwrap() < value {
                self.max_idx = Some(self.curr_idx);
            }
        }

        // if 'None', replace with 'Some(curr_idx)'
        else {
            self.max_idx = Some(self.curr_idx);
        }
    }



    pub fn append(&mut self, value: f64) {

        // if max capacity is reached, simply ignore call
        if self.curr_idx < self.list.len() {
            self.update_min(value);
            self.update_max(value);

            // put 'value' into vector and update index
            self.list[self.curr_idx] = Some(value);
            self.curr_idx += 1;
        }
    }


    /// Return min value.
    pub fn find_min(&self) -> Option<f64> {

        if let Some(idx) = self.min_idx {
            return self.list[idx];
        }

        return None;
    }


    /// Return max value.
    pub fn find_max(&self) -> Option<f64> {

        if let Some(idx) = self.max_idx {
            return self.list[idx];
        }

        return None;
    }
}
