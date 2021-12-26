use std::collections::HashMap;

fn sum(stack: &Vec<f32>) -> f32{
  stack.iter().sum()
}

fn prod(stack: &Vec<f32>) -> f32{
  stack.iter().product()
}

fn main() {
  let line = "2 2 + 1 -";
  let ops = "+*-/%$£";
  let mut bin_num_ops: HashMap<String, fn(f32,f32)->f32> = HashMap::new();
  bin_num_ops.insert("+".to_string(), |x, y| x + y);
  bin_num_ops.insert("-".to_string(), |x, y| x - y);
  bin_num_ops.insert("*".to_string(), |x, y| x * y);
  bin_num_ops.insert("/".to_string(), |x, y| x / y);
  bin_num_ops.insert("%".to_string(), |x, y| x % y);
  let bin_num_ops = bin_num_ops;
  let mut stac_num_ops: HashMap<String, fn(&Vec<f32>)->f32> = HashMap::new();
  stac_num_ops.insert("$".to_string(), |stac| sum(stac));
  stac_num_ops.insert("£".to_string(), |stac| prod(stac));
  let stac_num_ops = stac_num_ops;
  let mut stack: Vec<f32> = Vec::new();
  println!("{:?}",stack);

  let line_parse = line.split(' ').collect::<Vec<&str>>();
  let mut place_holder: f32;

  for t in line_parse.iter(){

    println!("tkn = {:?}",t);
    if ops.contains(t){
      if bin_num_ops.contains_key(&t.to_string()){
          place_holder = bin_num_ops[&t.to_string()](stack[0], stack[1]);
      } else {
          place_holder = stac_num_ops[&t.to_string()](&stack);
      }
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
