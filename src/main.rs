use std::vec;

// use Zehan::tools::array_tools::help;
use Zehan::tools::array_tools;

fn main() {

    let v1: Vec<i64> = vec![1, 2, 3];
    let v2: Vec<i64> = vec![1, 2, 3];

    let arr1: array_tools::Num_arr = array_tools::Num_arr{
        len: 5,
        vec: v1
    };

    let arr1: array_tools::Num_arr = array_tools::Num_arr{
        len: 5,
        vec: v2
    };

}
