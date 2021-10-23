mod simplex;
use simplex::solver::solve;

fn main() {
    let res = solve(vec![vec![1f64,1f64,0f64], vec![2f64,1f64,1f64], vec![1f64,0f64,1f64], vec![0f64,2f64,-1f64], vec![1f64,0f64,0f64], vec![0f64,1f64,0f64], vec![0f64,0f64,1f64]], 
        vec![50f64,30f64,20f64], 
        vec![4f64,-1f64,0f64,2f64,-1000f64,-1000f64,-1000f64]);
}

