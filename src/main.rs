use crate::solver::shared;

mod solver;
 
fn main() { 

    shared::timer(|| {
        solver::day08::task_a();
    });

    shared::timer(|| {
        solver::day08::task_b();
    });

}
