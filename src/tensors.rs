use std::{clone, default};

pub enum NodeVals {
    Tensor(TensorNode),
    Value(MathVec)
}

pub struct TensorNode {
    val: Box<NodeVals>,
    next_ptr: Box<TensorNode>
}

pub enum VecOrient {
    Column,
    Row
}

impl Clone for VecOrient {
    fn clone(&self) -> Self {
        match self {
            Self::Column => Self::Column,
            Self::Row => Self::Row,
        }
    }
}

impl Copy for VecOrient{}

pub struct MathVec {
    orientation: VecOrient,
    len: usize,
    pub vals: Vec<f64>
}

impl Default for MathVec {
    fn default() -> Self {
        Self { 
            orientation: VecOrient::Column, 
            len: 0, 
            vals: vec![]
        }
    }
}

impl Clone for MathVec {
    // col by default
    fn clone(&self) -> Self {
        Self { 
            orientation: self.orientation.clone(), 
            len: self.len.clone(), 
            vals: self.vals.clone()
        }
    }
}

impl MathVec {

    // initialization functions
    // col unless otherwise stated
    pub fn new(v: Vec<f64>) -> Self {
        Self { 
            orientation: VecOrient::Column, 
            len: v.len(), 
            vals: v
        }
    }

    // from rust vectors
    pub fn col_vector(v: Vec<f64>) -> Self {
        Self::new(v)
    }

    pub fn row_vector(v: Vec<f64>) -> Self {
        Self { 
            orientation: VecOrient::Row, 
            len: v.len(), 
            vals: v
        }
    }

    pub fn norm(self) -> f64{
        // iterator?
        let v = self.vals;
        let mut tot_sq: f64 = 0.0;
        for i in v {
            tot_sq += i*i;
        };

        tot_sq.sqrt()
    } 

    // returns 'direction vector1' i.e 
    pub fn dirn(&self) -> Self {
        let norm = self.clone().norm();

        let mut unit_vec_vals: Vec<f64> = Vec::new();

        for component_mag in self.clone().vals {
            unit_vec_vals.push(component_mag/norm);
        };

        Self {
            orientation: self.orientation,
            len: self.len,
            vals: unit_vec_vals
        }
    }
}

pub fn zeros(size: usize) -> MathVec {
    let vals: Vec<f64> = vec![0.0;size];
    MathVec::new(vals)
}

