mod q1;

fn run_q1() -> Result<(), String> {
    let ans = q1::q1::solve().ok_or("Could not read file for q1")?;
    println!("answer : {ans}");
    Ok(())
}

fn main() -> Result<(), String> {
    run_q1()
}
