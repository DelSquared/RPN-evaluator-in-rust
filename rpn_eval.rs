use std::collections::HashMap;

fn main() {

  let line = "2 2 + 1 -";
  let ops = "+*-/%";
  let mut bin_num_ops: HashMap<String, fn(f32,f32)->f32> = HashMap::new();
  bin_num_ops.insert("+".to_string(), |x, y| x + y);
  bin_num_ops.insert("-".to_string(), |x, y| x - y);
  bin_num_ops.insert("*".to_string(), |x, y| x * y);
  bin_num_ops.insert("/".to_string(), |x, y| x / y);
  bin_num_ops.insert("%".to_string(), |x, y| x % y);
  let bin_num_ops = bin_num_ops;
  let mut stack: Vec<f32> = Vec::new();
  println!("{:?}",stack);

  let line_parse = line.split(' ').collect::<Vec<&str>>();


  for t in line_parse.iter(){
    println!("tkn = {:?}",t);
    if ops.contains(t){
      let place_holder = bin_num_ops[&t.to_string()](stack[0], stack[1]);
      stack = Vec::new();
      stack.push(place_holder);
      println!("{:?}",stack);

    } else{
      stack.push(t.to_string().parse::<f32>().unwrap());
      println!("{:?}",stack);
    }

  }
  println!("result = {:?} (quick maths)",stack);

}
