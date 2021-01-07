pub mod token;

// pub struct Request {
  
// }

// pub fn decode() {

// }

// pub enum TOKENS {
//   WRITE D TO C
//   READ D FROM C WHERE Q LIMIT L
//   UPDATE D IN C WHERE Q LIMIT L
//   DELETE D IN C WHERE Q LIMIT L

//   VERB (REF) LOCATOR (REF) QUERY


//   JOIN C AND C AT Q
//   (C) JOIN (C) AT Q
// }

// // WRITE username = 'myraduser' TO users WHERE uuid === '1203724981'

// // READ uuid FROM users WHERE accountAge > 3000 AND userAge < 50
// // WRITE username = 'deleted' TO users WHERE uuid # ['...', '...', '...', '...']

pub fn test() {
  let testtoken = "test";
  let tok = token::Token::from_str(&testtoken, 0, 0);

  println!("The token is {}", tok.term());
}