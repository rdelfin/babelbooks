CREATE TABLE book (
  isbn VARCHAR NOT NULL PRIMARY KEY,
  title VARCHAR NOT NULL,
  author VARCHAR NOT NULL
);

CREATE TABLE owned_books (
    isbn VARCHAR NOT NULL,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (isbn) REFERENCES book(isbn),
    FOREIGN KEY (user_id) REFERENCES user(id),
    PRIMARY KEY (isbn, user_id)
);

CREATE TABLE user (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    username VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL
);

CREATE TABLE user_sessions (
    session_id VARCHAR NOT NULL  PRIMARY KEY,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user(id)
);
