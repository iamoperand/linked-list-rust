use list::List;

fn main() {
    let mut list: List<i32> = List::new();
    list.add_at_index(0, 0);
    list.remove_at_index(0);

    list.print();
}
