use crate::solver::shared;

mod solver;
 
fn main() { 

    shared::timer(|| {
        solver::day10::task_a();
    });

    shared::timer(|| {
        solver::day10::task_b();
    });

}
