// items是一个usize类型的vec切片
fn get_nth(items: &Vec<usize>, nth: usize) -> Option<usize> {
    if nth < items.len() {
        Some(items[nth])
    } else {
        None
    }
}

fn double(val: usize) -> usize {
    val * val
}

fn main() {
    let items = vec![1, 2, 3, 4];
    println!("len = {}", items.len());
    let d = get_nth(&items, 3).map(double);
    println!("d = {:?}", d); // d = Some(16)
}
