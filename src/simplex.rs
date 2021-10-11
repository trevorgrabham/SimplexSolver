pub mod fraction; 

pub fn solve(A: Vec<Vec<f64>>, b: Vec<f64>, c: Vec<f64>) -> (Vec<Vec<f64>>, Vec<f64>, Vec<f64>, f64) {
    assert_eq!(A.len(), c.len(), "A and c matrices are not compatable. c is 1x{} and A is {}x{}", c.len(), A[0].len(), A.len());
    assert_eq!(A[0].len(), b.len(), "A and b matrices are not compatable. A is {}x{} and b is {}x1", A[0].len(), A.len(), b.len());

    let m: usize = A[0].len();
    let n: usize = A.len();

    // scan to find the rows corresponding to I
    let mut B: Vec<usize> = vec![n+1;m];
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

    for i in 0..m {
        assert_ne!(B[i], n+1, "We could not find a column in A corresponding to the {}th row of an identity matrix", i);
    }

    let mut cB = vec![0f64;m];
    for i in 0..m {
        cB[i] = c[B[i]];
    }

    let mut reduced_cost = vec![0f64;n];
    let mut obj: f64 = 0f64;

    if cB != vec![0f64;m] {
        for i in 0..n {
            reduced_cost[i] = A[i]
                .clone()
                .into_iter()
                .zip(cB.clone().into_iter())
                .map(|(x,y)| x*y)
                .sum();
            reduced_cost[i] -= c[i];
            obj = b
            .clone()
            .into_iter()
            .zip(cB.clone().into_iter())
            .map(|(x,y)| x*y)
            .sum();
        }
    } else {
        for i in 0..n {
            reduced_cost[i] = -c[i];
        }
    }

    println!("{:?},{:?},{:?},{:?}", A, b, reduced_cost, obj);

    let mut first = A;
    let mut second = b;
    let mut third = reduced_cost;
    let mut fourth = obj;
    loop {
        let next = iterate(first.clone(), second.clone(), third.clone(), fourth);
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

fn iterate(A: Vec<Vec<f64>>, b: Vec<f64>, reduced_cost: Vec<f64>, obj: f64) -> Option<(Vec<Vec<f64>>, Vec<f64>, Vec<f64>, f64)> {
    let m = A[0].len();
    let n = A.len();


    let mut first_negative_reduced_cost: usize = n;
    for i in 0..n {
        if reduced_cost[i] < 0f64 {
            first_negative_reduced_cost = i;
            break;
        }
    }
    println!("Reduced cost index: {:?}", first_negative_reduced_cost);
    if first_negative_reduced_cost == n {
        println!("Optimal solution found!");
        return None;
    }

    let mut first_minimum_ratio = n;
    let mut minimum_ratio = f64::INFINITY;
    for i in 0..m {
        if A[first_negative_reduced_cost][i] > 0f64 && b[i]/A[first_negative_reduced_cost][i] < minimum_ratio {
            minimum_ratio = b[i]/A[first_negative_reduced_cost][i];
            first_minimum_ratio = i;
        }
    }
    println!("Minimum ratio: {:?}\tIndex: {:?}", minimum_ratio, first_minimum_ratio);
    if minimum_ratio == f64::INFINITY {
        println!("Linear program is unbounded. The resulting objective function value may or may not be optimal.");
        return None;
    }

    let mut new_A = A.clone();
    let mut new_b = vec![0f64;m];
    let mut new_reduced_cost = vec![0f64;n];
    let new_obj;
    for col in 0..n {
        for row in 0..m {
            if row == first_minimum_ratio {
                new_A[col][row] = A[col][row]/A[first_negative_reduced_cost][row];
            } else {
                new_A[col][row] = A[col][row] - (A[first_negative_reduced_cost][row] * A[col][first_minimum_ratio] / A[first_negative_reduced_cost][first_minimum_ratio]);
            }
        }
    }
    for i in 0..m {
        if i == first_minimum_ratio {
            new_b[i] = b[i]/A[first_negative_reduced_cost][i];
        } else {
            new_b[i] = b[i] - (b[first_minimum_ratio] * A[first_negative_reduced_cost][i] / A[first_negative_reduced_cost][first_minimum_ratio]);
        }
    }
    for i in 0..n {
        new_reduced_cost[i] = reduced_cost[i] - (reduced_cost[first_negative_reduced_cost] * A[i][first_minimum_ratio] / A[first_negative_reduced_cost][first_minimum_ratio]);
    }
    new_obj = obj - (b[first_minimum_ratio] * reduced_cost[first_negative_reduced_cost] / A[first_negative_reduced_cost][first_minimum_ratio]);

    println!("{:?},{:?},{:?},{:?}", new_A, new_b, new_reduced_cost, new_obj);
    Some((new_A, new_b, new_reduced_cost, new_obj))
}