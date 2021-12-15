use rocket::*;
use rocket::info;
use crate::form::Form;
#[derive(FromForm, Debug)]
/// Book is a structure which contains information of book
///
/// #field
///
/// title:- title of the book of type String.
///
/// author:- author of the book of type String.
///
/// rating:- rating of the book of type i32
struct Book {
    title: String,
    author: String,
    star: i32
}
/// new_book is a handler which is used to handling /book route
///
/// #Arguments
///
/// book_form- it is From<Book> type
///
/// #Return
///
/// Return String type
#[post("/book", data = "<book_form>")]
fn new_book(book_form: Form<Book>) -> String {
    let book: Book = book_form.into_inner();
    let mut dummy_db: Vec<Book> = Vec::new();
    let book_1: Book = Book {
        title: String::from("The End Point"),
        author: String::from("Alex carry"),
        star: 5
    };
    dummy_db.push(book_1);
    dummy_db.push(book);
    info!("Create and Update data on server successfully");
    format!("Book added successfully: {:?}", dummy_db)
}
/// rocket is a function which creates a hello route,mounts the route at the /api,and launches the application
///
/// #Arguments
///
/// No Arguments
///
/// #Return
///
/// Return Rocket<Build>
#[launch]
pub fn rocket() -> Rocket<Build>{
    rocket::build()
        .mount("/api", routes![new_book])
}
