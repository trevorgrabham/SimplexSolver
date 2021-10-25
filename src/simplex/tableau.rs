use super::fraction::Fraction;

#[derive(Debug, Clone, PartialEq)]
enum SolveMessage {
    Optimal,
    Unbounded,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tableau {
    n: usize,
    m: usize,

    solve_type: String,
    debug: bool,

    A: Vec<Vec<Fraction>>,
    b: Vec<Fraction>,
    c: Vec<Fraction>,
    reduced_cost: Vec<Fraction>,
    obj: Fraction,

    basis_index: Vec<usize>,
    cost_basis: Vec<Fraction>,
    original_basis_index: Vec<usize>,

    basis_inverse: Vec<Vec<Fraction>>,
    solution: Vec<Fraction>,
    
    col_index: usize,
    row_index: usize,

    solved: bool, 
    additional_info: SolveMessage,
}

impl Tableau {
    pub fn new(A: &Vec<Vec<f64>>, b: &Vec<f64>, c: &Vec<f64>, solve_type: String) -> Tableau {
        // Checks to make sure that the dimensions of our matrices are valid.
        // Will not need this anymore once I get the input from a website. 
        assert_eq!(A.len(), c.len(), "A and c matrices are not compatable. c is 1x{} and A is {}x{}", c.len(), A[0].len(), A.len());
        assert_eq!(A[0].len(), b.len(), "A and b matrices are not compatable. A is {}x{} and b is {}x1", A[0].len(), A.len(), b.len());

        let mut t = Tableau {
            m: A[0].len(),
            n: A.len(),
            solve_type: solve_type,
            debug: false,
            A: Vec::with_capacity(A.len()),
            b: Vec::with_capacity(A[0].len()),
            c: Vec::with_capacity(A.len()),
            reduced_cost: Vec::with_capacity(A.len()),
            obj: Fraction::from(0),
            basis_index: vec![A.len()+1 as usize;A[0].len()],
            cost_basis: Vec::with_capacity(A.len()),
            original_basis_index: vec![A.len()+1 as usize;A[0].len()],
            basis_inverse: Vec::with_capacity(A[0].len()),
            solution: Vec::with_capacity(A.len()),
            solved: false,
            additional_info: SolveMessage::None,
            col_index: A.len(),
            row_index: A[0].len(),
        };

        for i in 0..t.n {
            t.A.push(Vec::with_capacity(t.m));
            t.c.push(Fraction::from(c[i]));
            for j in 0..t.m {
                t.A[i].push(Fraction::from(A[i][j]));
                t.b.push(Fraction::from(b[j]));
                t.basis_inverse.push(Vec::with_capacity(t.m));
            }
        }
        if t.debug {
            print!("Cost vector: [");
            for i in 0..t.n-1 {
                print!("{} ", t.c[i]);
            }
            println!("{}]", t.c[t.n-1]);
        }

        // scan to find the rows corresponding to I
        t.find_basis();

        // Computing the cost vector corresponding to our basis matrix B
        t.get_cost_basis();

        // set up our reduced_cost vector and our objective value.
        // these are updated using our cost basis vector, and doing matrix multiplaction.
        t.compute_reduced_cost();

        t
    }

    pub fn solve(&mut self) {
        match self.solve_type.as_str() {
            "bland" => {
                while !self.solved {
                    self.print_table();
                    self.bland_reduced_cost();
                    if self.solved {
                        self.print_solution();
                        return;
                    }
                    self.find_min_ratio();
                    if self.solved {
                        self.print_solution();
                        return;
                    }
                    self.update();
                }
            }
            "standard" => {
                let mut count = 0;
                while !self.solved {
                    count += 1;
                    self.print_table();
                    self.standard_reduced_cost();
                    if self.solved {
                        self.print_solution();
                        return;
                    }
                    self.find_min_ratio();
                    if self.solved {
                        self.print_solution();
                        return;
                    }
                    self.update();
                    if count > 20 {
                        println!("Cycle detected. Terminating solution. Please try again with \"bland\".");
                        break;
                    }
                }
            }
            _ => {println!("An error has occurred.")}
        }
    }

    pub fn set_debug(&mut self, input: bool) {
        self.debug = input;
    }

    pub fn find_basis_inverse(&mut self) {
        for i in 0..self.m {
            for j in 0..self.m {
                self.basis_inverse[i].push(self.A[self.original_basis_index[i]][j].clone());
            }
        }
        if self.debug {
            for i in 0..self.m {
                print!("[\t");
                for j in 0..self.m {
                    print!("{}\t", self.basis_inverse[j][i]);
                }
                println!("]");
            }
        }
    }

    fn find_basis(&mut self) {
        let mut I = vec![Fraction::from(0);self.m];
        I[0] = Fraction::from(1);

        for i in 0..self.n {
            for j in 0..self.m {
                if I == self.A[i] {
                    self.basis_index[j] = i;
                    self.original_basis_index[j] = i;
                }
                I.rotate_right(1);
            }
        }
        // check that we got all the corresponding columns
        for i in 0..self.m {
            assert_ne!(self.basis_index[i], self.n+1, "We could not find a column in A corresponding to the {}th row of an identity matrix", i);
        }
        if self.debug {
            print!("basis indecies: [");
            for i in 0..self.m-1 {
                print!("{} ", self.basis_index[i]);
            }
            println!("{}]", self.basis_index[self.m-1]);
        }
    }

    fn get_cost_basis(&mut self) {
        for i in 0..self.m {
            self.cost_basis.push(Fraction::from(self.c[self.basis_index[i]].clone()));
        }
        if self.debug {
            print!("Cost basis: [");
            for i in 0..self.m-1 {
                print!("{} ", self.cost_basis[i]);
            }
            println!("{}]", self.cost_basis[self.m-1]);
        }
    }

    fn compute_reduced_cost(&mut self) {
        let mut is_zero = true;
        for i in 0..self.m {
            if self.cost_basis[i] != Fraction::from(0) {
                is_zero = false;
            }
        }
        if !is_zero {
            if self.debug {
                println!("Cost basis is non-zero!");
            }
            for i in 0..self.n {
                let mut sum = Fraction::from(0);
                for j in 0..self.m {
                    sum = sum + Fraction::from(self.A[i][j].clone()) * self.cost_basis[j].clone();
                }
                self.reduced_cost.push(Fraction::from(sum)-self.c[i].clone());
            }
            self.obj = Fraction::from(0);
            for i in 0..self.m {
                self.obj = self.obj.clone() + self.cost_basis[i].clone() * self.b[i].clone();
            }
        } else {
            if self.debug {
                println!("Cost basis is zero!");
                print!("Cost vector: [");
                for i in 0..self.n-1 {
                    print!("{} ", self.c[i]);
                }
                println!("{}]", self.c[self.n-1]);
            }
            for i in 0..self.n {
                self.reduced_cost.push(-self.c[i].clone());
            }
        }
        if self.debug {
            print!("Reduced cost vector: [");
            for i in 0..self.n-1 {
                print!("{} ", self.reduced_cost[i]);
            }
            println!("{}]", self.reduced_cost[self.n-1]);
        }
    }

    fn bland_reduced_cost(&mut self) {
        for i in 0..self.n {
            if self.reduced_cost[i] < Fraction::from(0i64) {
                self.col_index = i;
                return;
            }
        }
        self.solved = true;
        self.additional_info = SolveMessage::Optimal;
    }

    fn find_min_ratio(&mut self) {
        self.row_index = self.m;
        let mut minimum_ratio = Fraction::from(i64::MAX);
        for i in 0..self.m {
            if self.A[self.col_index][i] <= Fraction::from(0) {
                continue;
            }else if self.b[i].clone()/self.A[self.col_index][i].clone() < minimum_ratio {
                minimum_ratio = self.b[i].clone()/self.A[self.col_index][i].clone();
                self.row_index = i;
            }
        }
        // update our basis_index with the entering variable in place of the leaving variable
        self.basis_index[self.row_index] = self.col_index;
        if self.row_index == self.m {
            self.solved = true;
            self.additional_info = SolveMessage::Unbounded;
        }
        if self.debug {
            println!("Minimum ratio: {}\tIndex: {:?}", minimum_ratio, self.row_index+1);
        }
    }

    fn update(&mut self) {
        let mut temp_A = Vec::with_capacity(self.n);
        let mut temp_b = Vec::with_capacity(self.m);
        let mut temp_r = Vec::with_capacity(self.n);
        let temp_o: Fraction;

        for _ in 0..self.n {
            temp_A.push(Vec::with_capacity(self.m));
        }

        // update our A matrix
        for c in 0..self.n {
            for r in 0..self.m {
                if r == self.row_index {
                    temp_A[c].push(self.A[c][self.row_index].clone() / self.A[self.col_index][self.row_index].clone());
                } else {
                    temp_A[c].push(self.A[c][r].clone() - (self.A[self.col_index][r].clone() * self.A[c][self.row_index].clone() / self.A[self.col_index][self.row_index].clone()));
                }
            }
        }
        // update our b vector
        for r in 0..self.m {
            if r == self.row_index {
                temp_b.push(self.b[self.row_index].clone()/self.A[self.col_index][self.row_index].clone());
            } else {
                temp_b.push(self.b[r].clone() - (self.b[self.row_index].clone() * self.A[self.col_index][r].clone() / self.A[self.col_index][self.row_index].clone()));
            }
        }
        // update our reduced cost vector
        for c in 0..self.n {
            temp_r.push(self.reduced_cost[c].clone() - (self.reduced_cost[self.col_index].clone() * self.A[c][self.row_index].clone() / self.A[self.col_index][self.row_index].clone()));
        }
        // update our objective value function
        temp_o = self.obj.clone() - (self.b[self.row_index].clone() * self.reduced_cost[self.col_index].clone() / self.A[self.col_index][self.row_index].clone());
        
        //quick check 
        for i in 0..self.m {
            assert!(temp_b[i] >= Fraction::from(0));
        }

        self.A = temp_A;
        self.b = temp_b;
        self.reduced_cost = temp_r;
        self.obj = temp_o;
    }

    fn retrieve_solution(&mut self) {
        if self.debug {
            print!("basis: [");
            for i in 0..self.m-1 {
                print!("{} ", self.basis_index[i]);
            }
            println!("{}]", self.basis_index[self.m-1]);
        }
        let mut row_index = self.m;
        for i in 0..self.n {
            for j in 0..self.m {
                if self.basis_index[j] == i {
                    row_index = j;
                }
            }
            if row_index != self.m {
                self.solution.push(self.b[row_index].clone());
            } else {
                self.solution.push(Fraction::from(0));
            }
            row_index = self.m;
        }
    }

    fn print_table(&self) {
        for i in 0..self.m {
            print!("[\t");
            for j in 0..self.n {
                print!("{}\t", self.A[j][i]);
            }
            print!("|\t{}\t", self.b[i]);
            println!("]");
        }
        for _ in 0..(self.n+3) {
            print!("________");
        } 
        print!("\n[\t");
        for i in 0..self.n {
            print!("{}\t", self.reduced_cost[i]);
        }
        print!("|\t{}\t", self.obj);
        println!("]\n");
    }

    fn print_solution(&mut self) {
        self.print_table();
        self.retrieve_solution();
        match self.additional_info {
            SolveMessage::Optimal => {
                println!("Optimal solution was found.");
                print!("Solution: (");
                for i in 0..self.n-1 {
                    print!("{}, ", self.solution[i]);
                }
                println!("{})", self.solution[self.n-1]);
                println!("Optimal objective function value: {}", self.obj);
            }
            SolveMessage::Unbounded => {
                println!("The linear program is unbounded. The solution may or may not be optimal.");
                print!("Solution: (");
                for i in 0..self.n-1 {
                    print!("{}, ", self.solution[i]);
                }
                println!("{})", self.solution[self.n-1]);
                println!("Objective function value: {}", self.obj);
            }
            _ => {println!("An error seems to have occured.");}
        }
    }

    fn standard_reduced_cost(&mut self) {
        let mut most_negative_value = Fraction::from(0);
        self.col_index = self.n;
        for i in 0..self.n {
            if self.reduced_cost[i] < most_negative_value {
                self.col_index = i;
                most_negative_value = Fraction::from(self.reduced_cost[i].clone());
            }
        }
        if self.col_index == self.n {
            self.solved = true;
            self.additional_info = SolveMessage::Optimal;
        }
        if self.debug {
            println!("Reduced cost index: {:?}", self.col_index+1);
        }
    }
}

