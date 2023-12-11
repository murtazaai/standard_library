#[allow(dead_code)]
pub struct AveragedCollection {
    pub(crate) list: Vec<i32>,
    pub(crate) average: f64,
}

impl AveragedCollection {
    #[allow(dead_code)]
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
    }

    #[allow(dead_code)]
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    #[allow(dead_code)]
    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

