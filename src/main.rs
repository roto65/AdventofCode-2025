use crate::solver::shared;

mod solver;

fn main() { 

    shared::timer(|| {
        solver::day1::task_a();
    });
    
    shared::timer(|| {
        solver::day1::task_b();
    });

}
