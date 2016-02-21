use std::io::*;
fn main(){
    let I=stdin();
    for b in I.lock().lines().skip(1).map(|b|b.unwrap()){
        let mut p:Vec<_>=b.trim().split(' ').map(|b|b.as_bytes().to_vec()).collect();
        println!("{}",if!(p[0]==p[1])&&{p[0].sort();p[1].sort();p[0]==p[1]}{"Yes"}else{"No."})
    }
}
