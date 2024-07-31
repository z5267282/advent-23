mod q1;

#[allow(unused)]
fn run_q1() -> Result<(), String> {
    let ans = q1::q1::solve().ok_or("Could not read file for q1")?;
    println!("answer : {ans}");
    Ok(())
}

fn main() -> Result<(), String> {
    q1::q1::part_2();
    Ok(())
}
