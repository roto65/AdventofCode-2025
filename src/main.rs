use crate::solver::shared;

mod solver;
 
fn main() { 

    shared::timer(|| {
        solver::day11::task_a();
    });

    shared::timer(|| {
        solver::day11::task_b();
    });

}
