pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct State {
    pub counter: usize,
}

#[no_mangle]
pub fn step(state: &mut State) {
    state.counter += 1;
    println!("doing stuff in iteration {}", state.counter);
}
