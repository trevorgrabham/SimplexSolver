use super::fraction::Fraction;

#[derive(Debug, Clone, PartialEq)]
enum SolveMessage {
    Optimal,
    Unbounded,
    Phase1Complete,
    Infeasible,
    None,
}

#[derive(Debug, Clone, PartialEq)]
enum VariableSelectType {
    Standard,
    Bland,
}

#[derive(Debug, Clone, PartialEq)]
enum BigMSolveType {
    TwoPhase,
    Detatched,
}

#[derive(Debug, Clone, PartialEq)]
enum SolveType {
    Standard,
    Revised,
    Dual,
}


#[derive(Debug, Clone, PartialEq)]
pub struct Tableau {
    n: usize,
    m: usize,

    solve_type: SolveType,
    variable_select_type: VariableSelectType,
    big_M_solve_type: BigMSolveType,
    debug: bool,
    big_M: bool,

    A: Vec<Vec<Fraction>>,
    b: Vec<Fraction>,
    c: Vec<Fraction>,
    reduced_cost: Vec<Fraction>,
    obj: Fraction,

    basis_indecies: Vec<usize>,
    basis_cost_vector: Vec<Fraction>,
    two_phase_cost_vector: Vec<Fraction>,
    original_basis_indecies: Option<Vec<usize>>,

    b_inverse: Vec<Vec<Fraction>>,
    solution: Vec<Fraction>,
    
    entering_variable_index: usize,
    leaving_variable_index: usize,

    solved: bool, 
    additional_info: SolveMessage,
}

impl Tableau {
    pub fn new(A: &[&[f64]], b: &[f64], c: &[f64], variable_select_type: String, solve_type: String, big_M_solve_type: String) -> Tableau {
        // Checks to make sure that the dimensions of our matrices are valid.
        // Will not need this anymore once I get the input from a website. 
        assert_eq!(A.len(), c.len(), "A and c matrices are not compatable. c is 1x{} and A is {}x{}", c.len(), A[0].len(), A.len());
        assert_eq!(A[0].len(), b.len(), "A and b matrices are not compatable. A is {}x{} and b is {}x1", A[0].len(), A.len(), b.len());
        if solve_type.as_str() != "dual" {
            for i in 0..b.len() {
                match i%10 {
                    0 => {assert!(b[i] >= 0f64, "{}st entry in b is negative. Linear program is not in starndard form", i+1);},
                    1 => {assert!(b[i] >= 0f64, "{}nd entry in b is negative. Linear program is not in starndard form", i+1);},
                    _ => {assert!(b[i] >= 0f64, "{}th entry in b is negative. Linear program is not in starndard form", i+1);},
                }
            }
        }

        // re-work the input Strings into enum types. Potentially panics if an unknown String is passed in.
        let solve_type_enum: SolveType;
        let big_M_solve_type_enum: BigMSolveType;
        let variable_select_enum: VariableSelectType;
        match solve_type.as_str(){
            "standard" => {solve_type_enum = SolveType::Standard;},
            "revised" => {solve_type_enum = SolveType::Revised;},
            "dual" => {solve_type_enum = SolveType::Dual;},
            _ => {panic!("Not a recognized solve type.")},
        }
        match variable_select_type.as_str(){
            "standard" => {variable_select_enum = VariableSelectType::Standard;},
            "bland" => {variable_select_enum = VariableSelectType::Bland;},
            _ => {panic!("Not a recognized variable select type.")},
        }
        match big_M_solve_type.as_str(){
            "detached" => {big_M_solve_type_enum = BigMSolveType::Detatched;},
            "twophase" => {big_M_solve_type_enum = BigMSolveType::TwoPhase;},
            _ => {panic!("Not a recognized big M solve type.")},
        }

        // build the starting Tableau
        let mut t = Tableau {
            m: A[0].len(),
            n: A.len(),
            solve_type: solve_type_enum,
            variable_select_type: variable_select_enum,
            big_M_solve_type: big_M_solve_type_enum,
            debug: true,
            big_M: false,
            A: Vec::with_capacity(A.len()),
            b: Vec::with_capacity(A[0].len()),
            c: Vec::with_capacity(A.len()),
            reduced_cost: Vec::with_capacity(A.len()),
            obj: Fraction::from(0),
            basis_indecies: vec![A.len()+1 as usize;A[0].len()],
            basis_cost_vector: Vec::with_capacity(A.len()),
            two_phase_cost_vector: Vec::with_capacity(A.len()),
            original_basis_indecies: None,
            b_inverse: Vec::with_capacity(A[0].len()),
            solution: Vec::with_capacity(A.len()),
            solved: false,
            additional_info: SolveMessage::None,
            entering_variable_index: A.len(),
            leaving_variable_index: A[0].len(),
        };

        // fill in the tableau's tables with Fraction values
        for i in 0..t.n {
            t.A.push(Vec::with_capacity(t.m));
            if c[i] == f64::MAX {
                t.big_M = true;
                t.c.push(Fraction::from(i64::MAX));
            } else if c[i] == -f64::MAX {
                t.big_M = true;
                t.c.push(Fraction::from(-i64::MAX));
            } else {
                t.c.push(Fraction::from(c[i]));
            }
            for j in 0..t.m {
                t.A[i].push(Fraction::from(A[i][j]));
            }
        }
        for i in 0..t.m {
            t.b.push(Fraction::from(b[i]));
            t.b_inverse.push(Vec::with_capacity(t.m));
        }

        if t.debug {
            if t.big_M {
                println!("Working with Big M.");
            }
        }

        match t.solve_type {
            SolveType::Dual => {
                t.setup_dual_tableau();
            },
            _ => {
                // scan to find the rows corresponding to I
                t.find_basis_indecies();

                // Computing the cost vector corresponding to our basis matrix B
                t.compute_basis_cost_vector();

                // set up our reduced_cost vector and our objective value.
                // these are updated using our cost basis vector, and doing matrix multiplaction.
                t.compute_reduced_cost();
            },
        }

        t
    }

    pub fn solve(&mut self) {
        if self.big_M {
            println!("Working with Big M");
            match self.big_M_solve_type {
                BigMSolveType::Detatched => {

                },
                BigMSolveType::TwoPhase => {
                    while! self.solved {
                        self.print_table();
                        println!("Starting Phase 1:");
                        self.compute_entering_variable();
                        if self.solved {
                            if self.additional_info == SolveMessage::Infeasible {
                                self.print_solution();
                                break;
                            } else {
                                self.big_M = false;
                                self.solved = false;

                                // if artificial variables are in the basis, remove them
                                let mut new_m = self.m;
                                for i in (0..self.m).rev() {
                                    // check to see if the cost of any of our basis variables is equal to +-M, if it is we mark it as a leaving variable
                                    if self.c[self.basis_indecies[i]].abs() == Fraction::from(i64::MAX) {
                                        self.leaving_variable_index = i;
                                        self.entering_variable_index = self.n;
                                        // look for any non-zero pivot to replace our degenerate artificial variable
                                        for j in 0..self.n {
                                            // we don't want to pivot if the entering variable is the same as the leaving variable
                                            if j == self.basis_indecies[self.leaving_variable_index] {
                                                continue;
                                            }
                                            if self.A[j][self.leaving_variable_index] != Fraction::from(0) {
                                                self.entering_variable_index = j;
                                                break;
                                            }
                                        }
                                        if self.entering_variable_index != self.n {
                                            self.update();
                                        } else {
                                            // if we reached here than the only non-zero entry was the artificial variable, so we can remove the redundant constraint
                                            for col in 0..self.n {
                                                self.A[col].remove(self.leaving_variable_index);
                                            }
                                            self.b.remove(self.leaving_variable_index);
                                            new_m -= 1;
                                        }
                                    }
                                }
                                self.m = new_m;

                                let mut new_n = self.n;
                                for col in (0..self.n).rev() {
                                    if self.c[col].abs() == Fraction::from(i64::MAX) {
                                        match self.original_basis_indecies.clone() {
                                            Some(obi) => {
                                                if obi.contains(&col) {
                                                    self.original_basis_indecies = None;
                                                }
                                            },
                                            None => (),
                                        }
                                        self.A.remove(col);
                                        self.c.remove(col);
                                        new_n -= 1;
                                    }
                                }
                                self.n = new_n;
                                
                                // We use the current basis to compute the new basis_cost_vector and then calculate the new reduced cost.
                                // We can now solve the new tableau from here as normal
                                self.basis_cost_vector.drain(..);
                                self.reduced_cost.drain(..);
                                self.compute_basis_cost_vector();
                                self.compute_reduced_cost();
                                println!("Starting Phase 2:");
                                self.solve();
                                return;
                            }
                        }
                        self.compute_leaving_variable();
                        if self.solved {
                            self.print_solution();
                            return;
                        }
                        self.update();
                    }
                },
            }
        } else {
            match self.solve_type {
                SolveType::Standard => {
                    while !self.solved {
                        self.print_table();
                        self.compute_entering_variable();
                        if self.solved {
                            self.print_solution();
                            return;
                        }
                        self.compute_leaving_variable();
                        if self.solved {
                            self.print_solution();
                            return;
                        }
                        self.update();
                    }
                },
                SolveType::Revised => {
                },
                SolveType::Dual => {
                    while !self.solved {
                        self.print_table();
                        self.compute_leaving_variable();
                        if self.solved {
                            self.print_solution();
                            return;
                        }
                        self.compute_entering_variable();
                        if self.solved {
                            self.print_solution();
                            return;
                        }
                        self.update();
                    }
                },
            }
        }
    }


    pub fn set_debug(&mut self, input: bool) {
        self.debug = input;
    }

    pub fn find_b_inverse(&mut self) {
        match self.original_basis_indecies.clone() {
            Some(obi) => {
                for i in 0..self.m {
                    for j in 0..self.m {
                        self.b_inverse[i].push(self.A[obi[i]][j].clone());
                    }
                }
                if self.debug {
                    for i in 0..self.m {
                        print!("[\t");
                        for j in 0..self.m {
                            print!("{}\t", self.b_inverse[j][i]);
                        }
                        println!("]");
                    }
                }
            },
            None => {
                println!("Cannot find basis inverse as it has been dropped during calculations.");
            },
        }
    }

    fn find_basis_indecies(&mut self) {
        // create a row of the identity matrix to match against
        let mut I = vec![Fraction::from(0);self.m];
        I[0] = Fraction::from(1);

        // loop through each column of A and see if that column matches any columns of the identity matrix.
        // If it does match a column of the identity matrix, then the corresponding index is kept as a current basis_index and an original_basis_index
        let mut obi = vec![self.m+1;self.m];
        for A_col in 0..self.n {
            for I_col in 0..self.m {
                if I == self.A[A_col] {
                    self.basis_indecies[I_col] = A_col;
                    obi[I_col] = A_col;
                }
                I.rotate_right(1);
            }
        }
        self.original_basis_indecies = Some(obi);

        // check that we got all the corresponding columns
        for I_col in 0..self.m {
            match I_col%10 {
                0 => {assert_ne!(self.basis_indecies[I_col], self.n+1, "We could not find a column in A corresponding to the {}st column of an identity matrix", I_col+1);},
                1 => {assert_ne!(self.basis_indecies[I_col], self.n+1, "We could not find a column in A corresponding to the {}nd column of an identity matrix", I_col+1);},
                _ => {assert_ne!(self.basis_indecies[I_col], self.n+1, "We could not find a column in A corresponding to the {}th column of an identity matrix", I_col+1);},
            }
        }

        if self.debug {
            print!("basis indecies: [");
            for i in 0..self.m-1 {
                print!("{} ", self.basis_indecies[i]);
            }
            println!("{}]", self.basis_indecies[self.m-1]);
        }
    }

    fn compute_basis_cost_vector(&mut self) {
        // special case: first pass thru with Two Phase Big M method
        if !self.solved && self.big_M && self.big_M_solve_type == BigMSolveType::TwoPhase {
            
            // step 1: set up the cost vector for Phase 1
            for i in 0..self.n {
                if self.c[i].abs() == Fraction::from(i64::MAX) {
                    self.two_phase_cost_vector.push(Fraction::from(-1));
                } else {
                    self.two_phase_cost_vector.push(Fraction::from(0));
                }
            }
            // step 2: use the two_phase_cost_vector to set up the basis_cost_vector
            for i in 0..self.m {
                self.basis_cost_vector.push(self.two_phase_cost_vector[self.basis_indecies[i]].clone());
            }

        } else {
            for i in 0..self.m {
                self.basis_cost_vector.push(self.c[self.basis_indecies[i]].clone());
            }
        }

        if self.debug {
            print!("Cost basis: [");
            for i in 0..self.m-1 {
                print!("{} ", self.basis_cost_vector[i]);
            }
            println!("{}]", self.basis_cost_vector[self.m-1]);
        }
    }

    fn compute_reduced_cost(&mut self) {

        // check to see if the basis_cost_vector is zero.
        // If it is, we can skip a lot of this step
        let mut is_zero = true;
        for i in 0..self.m {
            if self.basis_cost_vector[i] != Fraction::from(0) {
                is_zero = false;
            }
        }

        if !is_zero {
            // add the matrix product of the cost basis and each column of A, and then subtract the original cost vector corresponding to the column we are working with
            for col in 0..self.n {
                let mut sum = Fraction::from(0);
                for row in 0..self.m {
                    sum = sum + Fraction::from(self.A[col][row].clone()) * self.basis_cost_vector[row].clone();
                }
                // special case: For Phase 1 of the TwoPhase Big M method, we use the two_phase_cost_vector instead of the original cost_vector
                    // if Phase 1 is complete already, then we need to compute the reduced cost with the actual cost_vector, so it can be treated like any other tableau
                if self.big_M && self.big_M_solve_type == BigMSolveType::TwoPhase && self.additional_info != SolveMessage::Phase1Complete {
                    self.reduced_cost.push(Fraction::from(sum)-self.two_phase_cost_vector[col].clone());
                } else {
                    self.reduced_cost.push(Fraction::from(sum)-self.c[col].clone());
                }
            }

            // set up the objective function value. There are no special cases for this
            self.obj = Fraction::from(0);
            for i in 0..self.m {
                self.obj = self.obj.clone() + self.basis_cost_vector[i].clone() * self.b[i].clone();
            }

        } else {
            // if the basis_cost_vector is zero then we are able to just use the negative values of the cost_vector as the reduced cost
            for col in 0..self.n {
                //we should never end up here, as if we are introducing artificial variables then they should be used in the starting basis, but just in case we check
                if self.big_M && self.big_M_solve_type == BigMSolveType::TwoPhase && self.additional_info != SolveMessage::Phase1Complete {
                    self.reduced_cost.push(-self.two_phase_cost_vector[col].clone());
                } else {
                    self.reduced_cost.push(-self.c[col].clone());
                }
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

    // will set solved=true if the linear program is optimal
    fn compute_entering_variable(&mut self) {
        match self.solve_type {
            SolveType::Dual => {
                let mut max_ratio = Fraction::from(-i64::MAX);
                for col in 0..self.n {
                    if self.A[col][self.leaving_variable_index] < Fraction::from(0) && self.reduced_cost[col].clone()/self.A[col][self.leaving_variable_index].clone() > max_ratio {
                        max_ratio = self.reduced_cost[col].clone()/self.A[col][self.leaving_variable_index].clone();
                        self.entering_variable_index = col;
                    }
                }
                
                if max_ratio == Fraction::from(-i64::MAX) {
                    self.solved = true;
                    self.additional_info = SolveMessage::Infeasible;
                }
                
                if self.debug {
                    if self.debug && !self.solved {
                        println!("Entering variable index: {:?}", self.entering_variable_index+1);
                    }
                }
            },
            _ => {
                match self.variable_select_type {
                    VariableSelectType::Bland => {

                        // find the first negative reduced cost and return
                        for col in 0..self.n {
                            if self.reduced_cost[col] < Fraction::from(0i64) {
                                self.entering_variable_index = col;
                                if self.debug {
                                    println!("Entering variable index: {:?}", self.entering_variable_index+1);
                                }
                                return;
                            }
                        }
                    },
                    VariableSelectType::Standard => {

                        // find the most negative reduced cost
                        let mut most_negative_value = Fraction::from(0);
                        self.entering_variable_index = self.n;
                        for col in 0..self.n {
                            if self.reduced_cost[col] < most_negative_value {
                                self.entering_variable_index = col;
                                most_negative_value = Fraction::from(self.reduced_cost[col].clone());
                            }
                        }
                        if self.entering_variable_index != self.n {
                            if self.debug {
                                println!("Entering variable index: {:?}", self.entering_variable_index+1);
                            }
                            return;
                        }
                    },
                }
    
                // if we could not find a negative reduced cost, then our solution is optimal
                self.solved = true;
                if self.big_M && self.big_M_solve_type == BigMSolveType::TwoPhase && self.obj != Fraction::from(0) {
                    self.additional_info = SolveMessage::Infeasible;
                } else {
                    self.additional_info = SolveMessage::Optimal;
                }
            },
        }
    }

    // will set solved=true if the linear program is unbounded
    fn compute_leaving_variable(&mut self) {
        match self.solve_type {
            SolveType::Dual => {
                let mut min = self.b[0].clone();
                self.leaving_variable_index = 0;
                for row in 1..self.m {
                    if self.b[row] < min {
                        self.leaving_variable_index = row;
                        min = self.b[row].clone();
                   }
                }

                if min >= Fraction::from(0) {
                    self.solved = true;
                    self.additional_info = SolveMessage::Optimal;
                }

                if self.debug {
                    match self.solved {
                        false => {println!("Leaving index: {:?}", self.leaving_variable_index+1);},
                        true => {println!("Linear program is optimal.");},
                    }
                }
            },
            _ => {
                // find the minimum_ratio
                self.leaving_variable_index = self.m;
                let mut minimum_ratio = Fraction::from(i64::MAX);
                for row in 0..self.m {
                    if self.A[self.entering_variable_index][row] <= Fraction::from(0) {
                        // if the entry in A[entering_variable_index] isn't positive, we don't consider it
                        continue;
                    }else if self.b[row].clone()/self.A[self.entering_variable_index][row].clone() < minimum_ratio {
                        // if the current row has a smaller ratio, then we update the minimum ratio.
                        // We use stricly less than, and we check the rows in ascending order, so that in the case of a tie, we take the first ratio we found
                        minimum_ratio = self.b[row].clone()/self.A[self.entering_variable_index][row].clone();
                        self.leaving_variable_index = row;
                    }
                }

                if self.leaving_variable_index == self.m {
                    self.solved = true;
                    self.additional_info = SolveMessage::Unbounded;
                }

                if self.debug {
                    match self.solved {
                        false => {println!("Minimum ratio: {}\tLeaving index: {:?}", minimum_ratio, self.leaving_variable_index+1);},
                        true => {println!("Linear program is unbounded.");},
                    }
                }
            },
        }
    }

    fn update(&mut self) {
        assert!(!self.solved, "Trying to update a tableau that is {:?}", self.additional_info);

        // we use temp variables to store our intermediate values
        let mut temp_A = Vec::with_capacity(self.n);
        let mut temp_b = Vec::with_capacity(self.m);
        let mut temp_r = Vec::with_capacity(self.n);
        let temp_o: Fraction;
        for _ in 0..self.n {
            temp_A.push(Vec::with_capacity(self.m));
        }

        // update our A matrix
        for col in 0..self.n {
            for row in 0..self.m {
                if row == self.leaving_variable_index {
                    // special case: the row corresponding to the leaving variable
                    temp_A[col].push(self.A[col][self.leaving_variable_index].clone() / self.A[self.entering_variable_index][self.leaving_variable_index].clone());
                } else {
                    temp_A[col].push(self.A[col][row].clone() - (self.A[self.entering_variable_index][row].clone() * self.A[col][self.leaving_variable_index].clone() / self.A[self.entering_variable_index][self.leaving_variable_index].clone()));
                }
            }
        }
        // update our b vector
        for row in 0..self.m {
            if row == self.leaving_variable_index {
                // special case: the row corresponding to the leaving variable
                temp_b.push(self.b[self.leaving_variable_index].clone()/self.A[self.entering_variable_index][self.leaving_variable_index].clone());
            } else {
                temp_b.push(self.b[row].clone() - (self.b[self.leaving_variable_index].clone() * self.A[self.entering_variable_index][row].clone() / self.A[self.entering_variable_index][self.leaving_variable_index].clone()));
            }
        }
        // update our reduced cost vector
        for col in 0..self.n {
            temp_r.push(self.reduced_cost[col].clone() - (self.reduced_cost[self.entering_variable_index].clone() * self.A[col][self.leaving_variable_index].clone() / self.A[self.entering_variable_index][self.leaving_variable_index].clone()));
        }
        // update our objective value function
        temp_o = self.obj.clone() - (self.b[self.leaving_variable_index].clone() * self.reduced_cost[self.entering_variable_index].clone() / self.A[self.entering_variable_index][self.leaving_variable_index].clone());

        // update our basis_indecies with the entering variable in place of the leaving variable
        self.basis_indecies[self.leaving_variable_index] = self.entering_variable_index;
        
        self.A = temp_A;
        self.b = temp_b;
        self.reduced_cost = temp_r;
        self.obj = temp_o;
    }

    fn retrieve_solution(&mut self) {
        if self.debug {
            print!("basis: [");
            for i in 0..self.m-1 {
                print!("{} ", self.basis_indecies[i]);
            }
            println!("{}]", self.basis_indecies[self.m-1]);
        }

        let mut in_index = false;
        for i in 0..self.n {

            // see if the variable is part of the basis
            for j in 0..self.m {
                if self.basis_indecies[j] == i {
                    self.solution.push(self.b[j].clone());
                    in_index = true;
                    break;
                }
            }
            if !in_index {
                self.solution.push(Fraction::from(0));
            }
            in_index = false;
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
            if self.reduced_cost[i] == Fraction::from(i64::MAX) {
                print!("M\t");
            } else if self.reduced_cost[i] == Fraction::from(-i64::MAX) {
                print!("-M\t");
            } else {
                print!("{}\t", self.reduced_cost[i]);
            }
        }
        print!("|\t{}\t", self.obj);
        println!("]\n");
    }

    fn print_solution(&mut self) {
        match self.additional_info {
            SolveMessage::Optimal => {
                self.print_table();
                self.retrieve_solution();
                println!("Optimal solution was found.");
                print!("Solution: (");
                for i in 0..self.n-1 {
                    print!("{}, ", self.solution[i]);
                }
                println!("{})", self.solution[self.n-1]);
                println!("Optimal objective function value: {}", self.obj);
            }
            SolveMessage::Unbounded => {
                self.print_table();
                self.retrieve_solution();
                println!("The linear program is unbounded. The solution may or may not be optimal.");
                print!("Solution: (");
                for i in 0..self.n-1 {
                    print!("{}, ", self.solution[i]);
                }
                println!("{})", self.solution[self.n-1]);
                println!("Objective function value: {}", self.obj);
            }
            SolveMessage::Infeasible => {
                self.print_table();
                println!("The linear program is infeasible.");
            }
            _ => {println!("An error seems to have occured.");}
        }
    }

    fn setup_dual_tableau(&mut self) {
        // keep track of which columns of I we have seen, because we don't need to do any pivots on these columns
        let mut seen = vec![false; self.m];
        let mut I = vec![Fraction::from(0);self.m];
        I[0] = Fraction::from(1);
        for col in 0..self.n {
            for i in 0..self.m {
                if I == self.A[col] {
                    seen[i] = true;
                    self.basis_indecies[i] = col;
                    I.rotate_right(1);
                    continue;
                }
                // multiply I by -1, if we have a column corresponding to -I, we can multiply the whole row by -1 to get a column of I
                I[i] = I[i].clone() * Fraction::from(-1);
                if I == self.A[col] {
                    seen[i] = true;
                    self.basis_indecies[i] = col;
                    // multiply the whole row by -1
                    for c in 0..self.n {
                        self.A[c][i] = self.A[c][i].clone() * Fraction::from(-1);
                    }
                    self.b[i] = self.b[i].clone() * Fraction::from(-1);
                }
                I[i] = I[i].clone() * Fraction::from(-1);
                I.rotate_right(1);
            }
        }

        if self.debug {
            print!("Seen: [");
            for i in 0..self.m-1 {
                if seen[i] {
                    print!("true, ");
                } else {
                    print!("false, ");
                }
            }
            if seen[self.m-1] {
                println!("true]");
            } else {
                println!("false]");
            }
        }

        // calculate our reduced cost row, using a cost basis of zeros.
        for _ in 0..self.n {
            self.basis_cost_vector.push(Fraction::from(0));
        }
        self.compute_reduced_cost();

        // for each row that corresponds to a column of I that was not found, pivot on any non-zero entry. If the entire row is zero, and b is not zero then the LP is infeasible
        let mut non_zero_entry_found = false;
        for row in (0..self.m).rev() {
            if seen[row] {
                continue;
            }
            // look for the first non-zero entry and pivot
            for col in 0..self.n {
                if self.A[col][row] != Fraction::from(0) {
                    non_zero_entry_found = true;
                    self.leaving_variable_index = row;
                    self.entering_variable_index = col;
                    self.update();
                    if self.debug {
                        self.print_table();
                    }
                    break;
                }
            }
            // if we didn't find a non-zero entry, the problem is either infeasible, or the constraint is redundant
            if !non_zero_entry_found {
                if self.b[row] != Fraction::from(0) {
                    self.solved = true;
                    self.additional_info = SolveMessage::Infeasible;
                    return;
                } else {
                    // redundant constraint dropped
                    for c in 0..self.n {
                        self.A[c].remove(row);
                    }
                    self.m -= 1;
                }
            }
            non_zero_entry_found = false;
        }

        // we should now have all of our rows corresponding to I
        if self.debug {
            self.print_table();
        }
        if self.debug {
            print!("Basis indecies: [");
            for i in 0..self.m-1 {
                print!("{}, ", self.basis_indecies[i]);
            }
            println!("{}]", self.basis_indecies[self.m-1]);
        }
        if self.debug {
            print!("Basis cost vector: [");
            for i in 0..self.m-1 {
                print!("{}, ", self.basis_cost_vector[i]);
            }
            println!("{}]", self.basis_cost_vector[self.m-1]);
        }

        // if we don't have negative reduced costs, we are done
        let mut negative_reduced_cost = false;
        for col in 0..self.n {
            if self.reduced_cost[col] < Fraction::from(0) {
                negative_reduced_cost = true;
                break;
            }
        }
        if !negative_reduced_cost {
            return;
        }

        // Add an artificial constraint
        for col in 0..self.n {
            if self.basis_indecies.contains(&col) {
                self.A[col].push(Fraction::from(0));
            } else {
                self.A[col].push(Fraction::from(1));
            }
        }
        self.b.push(Fraction::from(1000));
        self.m += 1;

        // Add an artificial variable for the artificial constraint
        self.A.push(vec![Fraction::from(0);self.m]);
        self.A[self.n][self.m-1] = Fraction::from(1);
        self.reduced_cost.push(Fraction::from(0));
        self.c.push(Fraction::from(0));
        self.n += 1;
        
        if self.debug {
            self.print_table();
        }

        // perform a pivot on the new constraint, selecting the most negative reduced cost
        let mut most_negative_value = Fraction::from(0);
        self.entering_variable_index = self.n;
        for col in 0..self.n {
            if self.reduced_cost[col] < most_negative_value {
                if self.debug {
                    println!("New most negative reduced cost is {} at index {}", self.reduced_cost[col], col);
                }
                self.entering_variable_index = col;
                most_negative_value = Fraction::from(self.reduced_cost[col].clone());
            }
        }

        self.original_basis_indecies = None;
        self.leaving_variable_index = self.m-1;
        self.basis_indecies.push(self.m-1);
        self.update();
    }

}
