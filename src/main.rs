use crate::solver::shared;

mod solver;
 
fn main() { 

    shared::timer(|| {
        solver::day09::task_a();
    });

    shared::timer(|| {
        solver::day09::task_b();
    });

}
