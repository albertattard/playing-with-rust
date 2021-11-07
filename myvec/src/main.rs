use myvec::MyVec;

fn main() {
    let mut vec = MyVec::<usize>::empty();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);

    println!("Vector: {:?}", vec);
}
