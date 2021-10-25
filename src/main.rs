mod simplex;
use simplex::tableau::Tableau;

fn main() {
    let A = vec![vec![1f64,1f64,0f64], vec![2f64,1f64,1f64], vec![1f64,0f64,1f64], vec![0f64,2f64,-1f64], vec![1f64,0f64,0f64], vec![0f64,1f64,0f64], vec![0f64,0f64,1f64]]; 
    let b = vec![50f64,30f64,20f64];
    let c = vec![4f64,-1f64,0f64,2f64,-1000f64,-1000f64,-1000f64];
    let mut tableau = Tableau::new(&A,&b,&c,String::from("standard"));
    tableau.set_debug(true);
    tableau.solve();
}

