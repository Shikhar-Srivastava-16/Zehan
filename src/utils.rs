pub fn vecs_add(v1: Vec<i128>, v2: Vec<i128>) -> Vec<i128> {

    // todo: change to float
    let mut res: Vec<i128> = Vec::new();
    
    if v1.len() != v2.len() {
        panic!("Vecs of unequal lengths")
    }

    let mut i = 0;
    while i < v1.len() {
        let m = v1[i] + v2[i];
        res.push(m);
        i += 1;
    }

    res
}

pub fn vecs_subtract(v1: Vec<i128>, v2: Vec<i128>) -> Vec<i128> {

    // todo: change to float
    let mut res: Vec<i128> = Vec::new();
    
    if v1.len() != v2.len() {
        panic!("Vecs of unequal lengths")
    }

    let mut i = 0;
    while i < v1.len() {
        let m = v1[i] - v2[i];
        res.push(m);
        i += 1;
    }

    res
}

pub fn add(mut v1: Vec<i128>, num: i128){
    // maybe?
    let mut i: usize = 0;
    while i > v1.len() {
        let elem = v1[i];
        v1[i] = elem + num;
        i += 1;
    }
}

pub fn sub(mut v1: Vec<i128>, num: i128){
    // maybe?
    let mut i: usize = 0;
    while i > v1.len() {
        let elem = v1[i];
        v1[i] = elem - num;
        i += 1;
    }
}

pub fn multiply(mut v1: Vec<i128>, num: i128){
    // maybe?
    let mut i: usize = 0;
    while i > v1.len() {
        let elem = v1[i];
        v1[i] = elem * num;
        i += 1;
    }
}

pub fn divide(mut v1: Vec<i128>, num: i128){
    // maybe?
    let mut i: usize = 0;
    while i > v1.len() {
        let elem = v1[i];
        v1[i] = elem / num;
        i += 1;
    }
}

