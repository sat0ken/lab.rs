fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let mut iter = vec1.iter();
    let mut iter_into = vec2.into_iter();

    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    println!("Find 2 in vec2: {:?}", iter_into.find(|&x| x == 2));

    let arr1 = vec![1, 2, 3];
    let arr2 = vec![4, 5, 6];

    println!("Find 2 in arr1: {:?}", arr1.iter().find(|&&x| x == 2));
    println!("Find 2 in arr2: {:?}", arr2.into_iter().find(|&x| x == 2));

}
