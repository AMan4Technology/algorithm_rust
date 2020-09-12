struct MaxQueue {
    max: usize,
    data: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MaxQueue {
    fn new() -> Self {
        MaxQueue { max: 0, data: vec![] }
    }

    fn max_value(&self) -> i32 {
        if self.data.len() == 0 { return -1; }
        self.data[self.max]
    }

    fn push_back(&mut self, value: i32) {
        self.data.push(value);
        if self.data[self.max] <= value { self.max = self.data.len() - 1; }
    }

    fn pop_front(&mut self) -> i32 {
        if self.data.len() == 0 { return -1; }
        let result = self.data.remove(0);

        if self.max == 0 {
            for i in 1..self.data.len() {
                if self.data[i] >= self.data[self.max] {
                    self.max = i;
                }
            }
        } else { self.max -= 1; }

        result
    }
}

fn test() {
    let mut obj = MaxQueue::new();
    let ret_1: i32 = obj.max_value();
    obj.push_back(1);
    let ret_3: i32 = obj.pop_front();
}
