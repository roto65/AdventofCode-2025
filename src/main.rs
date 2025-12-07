use crate::solver::shared;

mod solver;
 
fn main() { 

    shared::timer(|| {
        solver::day06::task_a();
    });

    shared::timer(|| {
        solver::day06::task_b();
    });

}
