use thiserror::Error;

fn main() {
    println!("foo");
}

enum Tensor {
    /// Internal recursive representation of the elements of a tensor.
    InTensor(Vec<Tensor>),
    /// Base representation of a Numeric Tensor 
    Number(f64)
}

impl Tensor {
    /// Initialize a Singleton Tensor using a single floating point number.
    /// This represents the type of tensor which is essentially a single number, or a __0-dimensional list__ 
    pub fn from_num(num: f64) -> Result<Tensor, TensorError> {
        Ok(Tensor::Number(num))
    }

    /// A little more complicated: this function creates an n+1-dimensional tensor using a Vector of floating point numbers.
    pub fn from_vec(vec: Vec<Tensor>) -> Result<Tensor, TensorError> {

        Ok(Tensor::InTensor(vec))
    }

    pub fn from_vec_num(vec: Vec<f64>) -> Result<Tensor, TensorError> {
        // for
    }
} 

#[derive(Error, Debug)]
pub enum TensorError{

    #[error("The Operation {0} is not implemented yet!")]
    OperationNotImplemented(String),

    #[error("The Operation {0} is not supported!")]
    OperationNotSupported(String),

}