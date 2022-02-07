mod entity;

fn main() {
    use entity::Collection;

    struct Book {
        bookid: usize,
        name: String,
    }

    impl Collection for Book {
        type Id = usize;
        fn id(&self) -> usize {
            self.bookid
        }
    }

    let book = Book {
        bookid: 12345,
        name: String::from("Pride"),
    };

    book.store();
    println!("bookid {} name {}", book.id(), book.name);
}
