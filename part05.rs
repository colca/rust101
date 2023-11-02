#[derive(Clone)]
pub struct BigInt {
     pub data: Vec<u64>,
     pub num: u64
 }

 impl BigInt {
     pub fn new(x:u64) ->  Self {
         if x == 0 {
             BigInt {
                 data: vec![],
                 num:0
             }
         } else {
             BigInt {data : vec![x],
             num:0}
         }
     }

     pub fn test_invariant(&self) -> bool {
         if self.data.len() == 0 {
             true
         } else {
             self.data[self.data.len() - 1] != 0
         }
     }
     pub fn from_vec(mut v: Vec<u64>) -> u64 {
         let mut num = 0;
         for ele in v.iter().rev() {
             num *= 10;
             num += ele;
         }
         println!("number from vector is {}", num);
         num
     }
 }

 pub fn main() {
     let v = vec![0, 1, 2, 7, 1];
     for ele in v.clone() {
         println!("vector ele is {}", ele);
     }

     let b1 = BigInt::from_vec(v.clone());
     let b2 = BigInt::from_vec(v);
 }