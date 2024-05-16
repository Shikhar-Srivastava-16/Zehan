use std::vec;

pub fn help() {
    println!("hello world")
}
pub struct Num_arr {
    pub len: i32,
    pub vec: Vec<i64>
}



impl Num_arr {
    fn init_arr() -> Num_arr{
        
        let returner = Num_arr {
            len: 0,
            vec: Vec::new()
        };

        returner
    }

    fn add_array(self, arr_add: Num_arr){
        // Num_arr

        let mut index: usize = 0;
        loop {
            if index == Vec::len(&self.vec) {
                break;
            } else {
                // add index of arr_add to index of self
                index+=1;
            }
        }        

        // res_arr
    }
}
