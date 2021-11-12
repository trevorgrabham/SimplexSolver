mod simplex;
use simplex::tableau::Tableau;

fn main() {
    let A = vec![vec![0.25f64,0.5f64,0f64], vec![-8f64,-0.5f64,0f64], vec![-1f64,-0.5f64,1f64], vec![9f64,3f64,0f64], vec![1f64,0f64,0f64], vec![0f64,1f64,0f64], vec![0f64,0f64,1f64]]; 
    let b = vec![0f64,0f64,1f64];
    let c = vec![0.75f64,-20f64,0.5f64,-6f64,0f64,0f64,0f64];
    let mut tableau = Tableau::new(&A,&b,&c,String::from("standard"),String::from("bland"),String::from("twophase"));
    tableau.set_debug(true);
    tableau.solve();
    tableau.find_b_inverse();
}

