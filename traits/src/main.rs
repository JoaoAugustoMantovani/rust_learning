fn main(){
    let stem = String::from("Tall");
    let leaves = String::from("Autumn");
    let root = String::from("Deep");
    let tree = Tree {stem, leaves, root};
    tree.print_stem();
    tree.print_leaves();
    tree.print_root();
    println!("Is a plant? {}\nBears fruits? {}", tree.is_plant(), tree.bears_fruit());
}

struct Tree{
    stem: String,
    leaves: String,
    root: String
}

impl Tree{
    fn print_stem(&self){
        println!("Stem: {}", self.stem);
    }
    fn print_leaves(&self){
        println!("Leaves: {}", self.leaves);
    }
    fn print_root(&self){
        println!("Root: {}", self.root);
    }
}

impl Plant for Tree{
    fn bears_fruit(&self) -> bool{
        true
    }
}

trait Plant {
    fn bears_fruit(&self) -> bool;
    fn is_plant(&self) -> bool{
        true
    }
}