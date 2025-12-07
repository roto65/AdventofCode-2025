use crate::solver::shared;

mod solver;
 
fn main() { 

    shared::timer(|| {
        solver::day07::task_a();
    });

    shared::timer(|| {
        solver::day07::task_b();
    });

}
