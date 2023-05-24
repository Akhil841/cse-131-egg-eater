use std::env;
#[link(name = "our_code")]
extern "C" {
    // The \x01 here is an undocumented feature of LLVM that ensures
    // it does not add an underscore in front of the name.
    // Courtesy of Max New (https://maxsnew.com/teaching/eecs-483-fa22/hw_adder_assignment.html)
    #[link_name = "\x01our_code_starts_here"]
    fn our_code_starts_here(input : i64, memory : *mut i64) -> i64;
}

#[no_mangle]
#[export_name = "\x01snek_error"]
pub fn snek_error(errcode : i64) {
  match errcode {
    3 => eprintln!("Bad arguments to native function or operator"),
    4 => eprintln!("Called lookup on non-pair object"),
    5 => eprintln!("Called lookup with non-number as index"),
    6 => eprintln!("lookup: index out of bounds"),
    _ => eprintln!("An error occurred {}", errcode)
  }
  std::process::exit(1);
}

// let's change snek_str to print ... when it visits a cyclic value
fn snek_str(val : i64, seen : &mut Vec<i64>) -> String {
  if val == 7 { "true".to_string() }
  else if val == 3 { "false".to_string() }
  else if val % 2 == 0 { format!("{}", val >> 1) }
  else if val == 1 { "nil".to_string() }
  else if val & 1 == 1 {
    if seen.contains(&val)  { return "(tuple <cyclic>)".to_string() }
    seen.push(val);
    let mut result = "(tuple ".to_string();
    let addr = (val - 1) as *const i64;
    let ct = unsafe { *addr };
    for i in 0..(ct + 1) {
      result += &snek_str(unsafe {*addr.offset((1 + i) as isize)}, seen);
      if i != ct { result += " "; }
    }
    result += ")";
    seen.pop();
    return result;
  }
  else {
    format!("Unknown value: {}", val)
  }
}

#[no_mangle]
#[export_name = "\x01snek_print"]
fn snek_print(val : i64) -> i64 {
  let mut seen = Vec::<i64>::new();
  println!("{}", snek_str(val, &mut seen));
  return val;
}
fn parse_arg(v : &Vec<String>) -> i64 {
  if v.len() < 2 { return 3 }
  let s = &v[1];
  if s == "true" { 7 }
  else if s == "false" { 3 }
  else if s == "nil" { 1 }
  else { s.parse::<i64>().unwrap() << 1 }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = parse_arg(&args);

    let mut memory = Vec::<i64>::with_capacity(1000000);
    let buffer :*mut i64 = memory.as_mut_ptr();
    
    /*
    //chatgpt
    let address_string = format!("{:p}", buffer as *const () as *const ());
    eprintln!("{}", address_string);
    */
    let i : i64 = unsafe { our_code_starts_here(input, buffer) };
    snek_print(i);
}


