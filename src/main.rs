use crate::solver::shared;

mod solver;
 
fn main() { 

    shared::timer(|| {
        solver::day04::task_a();
    });

    shared::timer(|| {
        solver::day04::task_a_2();
    });
    
    shared::timer(|| {
        solver::day04::task_b();
    });

}
