use crate::solver::shared;

mod solver;

fn main() { 

    shared::timer(|| {
        solver::day02::task_a();
    });
    
    shared::timer(|| {
        solver::day01::task_b();
    });

}
