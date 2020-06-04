fn pas(rows: u64) {
    for i in 0..rows  {
        let mut  s = 1;
        for _j in 1..2 * (rows - 1 - i) + 1 {
            print!(" ");
        
     }

      for l in 0..i + 1 {
        print!("{:2} ", s);
        s = s * (i - l) / (l + 1);
    }
       println!();
   }
}
fn main(){ 
    pas(10);
} 