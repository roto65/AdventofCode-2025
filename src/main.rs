use crate::solver::shared;

mod solver;
 
fn main() { 

    shared::timer(|| {
        solver::day03::task_a();
    });
    
    shared::timer(|| {
        solver::day03::task_b();
    });

}
