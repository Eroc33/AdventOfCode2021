fn main() -> Result<(), advent_of_utils::Error> {
    let numbers = advent_of_utils::input_lines_as::<u32>()?;
    let mut increases = 0;
    for window in numbers.windows(2) {
        if window[1] > window[0] {
            increases += 1;
        }
    }
    println!("{}", increases);
    Ok(())
}
