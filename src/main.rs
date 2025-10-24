mod color;
use color::Color;
mod pallet;
use pallet::Pallet;

fn main() {
    let cor: Color = Pallet.random();

    println!("The random color was: {}", cor.to_rgb());
    println!(
        "Its reverse color is: {}",
        Pallet.reverse(cor.clone()).to_rgb()
    );
    println!(
        "Her random magic color is: {}",
        Pallet.random_magic(cor)
    );

    let a = Pallet.random();
    let b = Pallet.random();
    println!("Joining colors, color A {} and color B {}", a.to_rgb(), b.to_rgb());
    println!("Resulted in the color: {}", Pallet.join(a, b).to_rgb());
}
