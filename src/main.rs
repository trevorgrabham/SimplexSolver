mod simplex;
use simplex::simplex::solve;

fn main() {
    let res = solve(vec![vec![2f64,6f64,3f64], vec![-3f64,-5f64,1f64], vec![-5f64,-3f64,2f64], vec![6f64,2f64,4f64], vec![1f64,0f64,0f64], vec![0f64,1f64,0f64], vec![0f64,0f64,1f64]], 
        vec![0f64,0f64,1f64], 
        vec![1f64,-1f64,1f64,-1f64,0f64,0f64,0f64]);
}

