fn main() {
  let x = 5;
  println!("The value of x is: {x}");
  let x = x + 1;
  {
    let x = x * 2;
    println!("Inside this block, x value is {x}");
  }
  println!("Now the value of x is: {x}");

  const THREE_HOURS_IN_SECONDS: i32 = 60 * 60 * 3;
  println!("{THREE_HOURS_IN_SECONDS}");
}
