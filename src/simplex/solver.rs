use super::fraction::Fraction;

const DEBUG: bool = true;


pub fn solve(A: Vec<Vec<f64>>, b: Vec<f64>, c: Vec<f64>) -> (Vec<Vec<Fraction>>, Vec<Fraction>, Vec<Fraction>, Fraction) {
    // Checks to make sure that the dimensions of our matrices are valid.
    // Will not need this anymore once I get the input from a website. 
    assert_eq!(A.len(), c.len(), "A and c matrices are not compatable. c is 1x{} and A is {}x{}", c.len(), A[0].len(), A.len());
    assert_eq!(A[0].len(), b.len(), "A and b matrices are not compatable. A is {}x{} and b is {}x1", A[0].len(), A.len(), b.len());

    let m: usize = A[0].len();
    let n: usize = A.len();

    // scan to find the rows corresponding to I
    let B: Vec<usize> = setup::find_basis(&A);

    for i in 0..m {
        assert_ne!(B[i], n+1, "We could not find a column in A corresponding to the {}th row of an identity matrix", i);
    }
    // at this point, we have found our basis, B. 
    // The indecies corresponding to the identity matrix are stored in order

    // Computing the cost vector corresponding to our basis matrix B
    let cB = setup::get_cost_basis(&c, &B);

    // set up our reduced_cost vector and our objective value.
    // these are updated using our cost basis vector, and doing matrix multiplaction.
    let (reduced_cost, obj) = setup::compute_reduced_cost(&cB, &A, &b, &c);

    // set up our variables to be sent off to the iterative stage. 
    // made clones so that we can hand over ownership to the iterate function.
    let mut first = Vec::with_capacity(n);
    for i in 0..n {
        first.push(Vec::with_capacity(m));
        for j in 0..m {
            first[i].push(Fraction::from(A[i][j]));
        }
    }
    let mut second = Vec::with_capacity(m);
    for i in 0..m {
        second.push(Fraction::from(b[i]));
    }
    let mut third = reduced_cost;
    let mut fourth = obj;
    // We iterate until the function returns None.
    // Iterate will print a line to let us know what caused the iterations to terminate.
    // We give it clones of our variables so that we have the previous version, before it terminates (if we didn't clone it would get consumed by the function call)
    loop {
        let next = iterate(first.clone(), second.clone(), third.clone(), fourth.clone());
        match next {
            Some((a,b,reduced,o)) => {
                first = a;
                second = b;
                third = reduced;
                fourth = o;
            }
            None => {
                return (first, second, third, fourth);
            }
        }
    }

}

fn iterate(A: Vec<Vec<Fraction>>, b: Vec<Fraction>, reduced_cost: Vec<Fraction>, obj: Fraction) -> Option<(Vec<Vec<Fraction>>, Vec<Fraction>, Vec<Fraction>, Fraction)> {
    // do not need to do bounds checking here, as it is already done in the function that calls this one
    let m = A[0].len();
    let n = A.len();

    if DEBUG {
        for i in 0..m {
            print!("[\t");
            for j in 0..n {
                print!("{}\t", A[j][i]);
            }
            print!("|\t{}\t", b[i]);
            println!("]");
        }
        for i in 0..(n+3) {
            print!("________");
        } 
        print!("\n[\t");
        for i in 0..n {
            print!("{}\t", reduced_cost[i]);
        }
        print!("|\t{}\t", obj);
        println!("]");
    }
    // First we find our variable with a negative reduced cost
    // We do this using Bland's rule, so that there will not be cycling
    let reduced_cost_index = solvers::bland_reduced_cost(&reduced_cost);

    if reduced_cost_index == n {
        println!("Optimal solution found!");
        return None;
    }
    if DEBUG {
        println!("Reduced cost index: {:?}", reduced_cost_index+1);
    }

    // We now have to find our minimum ratio after selecting our entering variable
    let (minimum_ratio, minimum_ratio_index) = solvers::find_min_ratio(&A, &b, &reduced_cost_index);

    if DEBUG {
        println!("Minimum ratio: {}\tIndex: {:?}", minimum_ratio, minimum_ratio_index);
    }
    if minimum_ratio == Fraction::from(i64::MAX) {
        println!("Linear program is unbounded. The resulting objective function value may or may not be optimal.");
        return None;
    }


    let mut new_A = Vec::with_capacity(n);
    for _ in 0..n {
        new_A.push(Vec::with_capacity(m));
    }
    let mut new_b = Vec::with_capacity(m);
    let mut new_reduced_cost = Vec::with_capacity(n);
    let new_obj: Fraction;
    // update our A matrix
    for col in 0..n {
        for row in 0..m {
            if row == minimum_ratio_index {
                new_A[col].push(A[col][row].clone()/A[reduced_cost_index][row].clone());
            } else {
                new_A[col].push(A[col][row].clone() - (A[reduced_cost_index][row].clone() * A[col][minimum_ratio_index].clone() / A[reduced_cost_index][minimum_ratio_index].clone()));
            }
        }
    }
    // update our b matrix
    for i in 0..m {
        if i == minimum_ratio_index {
            new_b.push(b[i].clone()/A[reduced_cost_index][i].clone());
        } else {
            new_b.push(b[i].clone() - (b[minimum_ratio_index].clone() * A[reduced_cost_index][i].clone() / A[reduced_cost_index][minimum_ratio_index].clone()));
        }
    }
    // update our reduced cost matrix
    for i in 0..n {
        new_reduced_cost.push(reduced_cost[i].clone() - (reduced_cost[reduced_cost_index].clone() * A[i][minimum_ratio_index].clone() / A[reduced_cost_index][minimum_ratio_index].clone()));
    }
    // update our objective value function
    new_obj = obj - (b[minimum_ratio_index].clone() * reduced_cost[reduced_cost_index].clone() / A[reduced_cost_index][minimum_ratio_index].clone());

    Some((new_A, new_b, new_reduced_cost, new_obj))
}

mod setup {
    use super::Fraction;
    // scans the columns of A, finding the identity matrix columns within. 
    // B[i] = n+1, when it is unable to find the i+1th column of the Identity matrix
    pub fn find_basis(A: &Vec<Vec<f64>>) -> Vec<usize> {
        let m = A[0].len();
        let n = A.len();
        let mut B: Vec<usize> = vec![n+1 as usize;m];
        let mut I = vec![0f64;m];
        I[0] = 1f64;

        for i in 0..n {
            for j in 0..m {
                if I == A[i] {
                    B[j] = i;
                }
                I.rotate_right(1);
            }
        }

        B
    }

    pub fn get_cost_basis(c: &Vec<f64>, B: &Vec<usize>) -> Vec<Fraction> {
        let m = B.len();
        let mut cB = Vec::with_capacity(m);
        for i in 0..m {
            cB.push(Fraction::from(c[B[i]]));
        }
        cB
    }

    pub fn compute_reduced_cost(cB: &Vec<Fraction>, A: &Vec<Vec<f64>>, b: &Vec<f64>, c: &Vec<f64>) -> (Vec<Fraction>, Fraction) {
        let n = A.len();
        let m = A[0].len();
        let mut reduced_cost = Vec::with_capacity(n);
        let mut obj: Fraction = Fraction::from(0);
        if *cB != vec![Fraction::from(0i64);m] {
            for i in 0..n {
                let mut sum = Fraction::from(0);
                for j in 0..m {
                    sum = sum + Fraction::from(A[i][j]) * cB[j].clone();
                }
                reduced_cost.push(sum-Fraction::from(c[i]));
            }
            for i in 0..m {
                obj = obj + cB[i].clone() * Fraction::from(b[i]);
            }
        } else {
            for i in 0..n {
                reduced_cost.push(Fraction::from(-c[i]));
            }
        }
        (reduced_cost, obj)
    }
}

mod solvers {
    use super::Fraction;
    pub fn bland_reduced_cost(reduced_cost: &Vec<Fraction>) -> usize {
        let n = reduced_cost.len();
        for i in 0..n {
            if reduced_cost[i] < Fraction::from(0i64) {
                return i;
            }
        }
        n
    }

    pub fn find_min_ratio(A: &Vec<Vec<Fraction>>, b: &Vec<Fraction>, reduced_cost_index: &usize) -> (Fraction, usize) {
        let n = A.len();
        let m = A[0].len();

        let mut minimum_ratio_index = n;
        let mut minimum_ratio = Fraction::from(i64::MAX);
        for i in 0..m {
            if A[*reduced_cost_index][i] <= Fraction::from(0) {
                continue;
            }else if b[i].clone()/A[*reduced_cost_index][i].clone() < minimum_ratio {
                minimum_ratio = b[i].clone()/A[*reduced_cost_index][i].clone();
                minimum_ratio_index = i;
            }
        }
        (minimum_ratio, minimum_ratio_index)
    }

}