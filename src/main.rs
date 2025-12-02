use crate::solver::shared;

mod solver;

fn main() { 

    shared::timer(|| {
        solver::day2::task_a();
    });
    
    shared::timer(|| {
        solver::day2::task_b();
    });

}
