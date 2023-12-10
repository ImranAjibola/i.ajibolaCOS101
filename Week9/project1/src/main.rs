use std::io::Write;
use std::fs::OpenOptions;

fn main() {
    let mut file = std::fs::File::create("project1.txt").expect("Failed");
    let lager = "\nLAGER \n33 Export \nDesperados \nGoldberg \nGulder \nHeineken \nStar";
    let stout = "\n\nSTOUT \nLegend \nTurbo \nKing \nWilliams";
    let non_alcoholic = "\n\nNON-ALCOHOLIC \nMaltina \nAmstel malta \nMalta Gold \nFayrouz";

    file.write_all(lager.as_bytes()).expect("Failed");
    file.write_all(stout.as_bytes()).expect("Failed");
    file.write_all(non_alcoholic.as_bytes()).expect("Failed");
    println!("done");
}
