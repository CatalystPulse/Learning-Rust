use std::io;
fn main(){
  loop{
   println!("0.Convert Fahrenheit to Celsius\n1.Covert Celsius to Fahrenheit\nPlease input your number(If it was nethier 0 nor 1,it would break.)\n");
   let mut choose = String::new();
   io::stdin()
      .read_line(&mut choose)
      .expect("Failed to read line");
   let choose =choose.trim().parse::<i32>().unwrap();
  if choose==0{
   c2f();
  }
  else if choose ==1{
   f2c();
  }
  else {break;}
}
}
fn f2c(){
   println!("F=");
   let c;
   let mut f  = String::new();
   io::stdin()
      .read_line(&mut f)
      .expect("Failed to read line");
   let f =f.trim().parse::<i32>().unwrap();
   c = (f-32)*5/9;
   println!("C={c}");
   }
fn c2f(){
   println!("C=");
   let  f;
   let mut c = String::new();
   io::stdin()
      .read_line(&mut c)
      .expect("Failed to read line");
      let c =c.trim().parse::<i32>().unwrap();
   f= c*9/5+32;
   println!("F={f}");
   }