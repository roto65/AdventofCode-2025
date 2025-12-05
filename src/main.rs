use crate::solver::shared;

mod solver;
 
fn main() { 

    shared::timer(|| {
        solver::day05::task_a();
    });

    shared::timer(|| {
        solver::day05::task_b();
    });

}
