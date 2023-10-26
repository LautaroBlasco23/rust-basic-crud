# Rust Basic CRUD

This API was developed as a practical project with the objective of getting confidence with the Rust language and their uses on the backend development.

In this API you can save your favorite books reviews.

# To run the API:

You have to build the docker image contained in the Dockerfile:

```
docker build -t rust-crud-api .
```

And then, run the container exposing the port 8080:


```
docker run -p 8080:8080 rust-crud-api
```

### Technical information:

In this proejct i will use Actix-web to simplify the development of the API.

This API uses the memory of the running app as a database. every time you restart the server, all the saved data will disappear.

you can interact with the API using the following URL: http://0.0.0.0:8080/crudapi/

### Endpoints:

* /books/ -> POST: create a new book review.
* /books/ -> GET: Get saved reviews.
* /books/get/{:id}/ -> GET: Get saved review.
* /books/modify/{:id}/ -> PUT: Modify saved review.
* /books/delete/{:id}/ -> DELETE: Delete saved review.
* /books/stars/{number_starts} -> GET: Get only the reviews with the indicated ammount of stars.

### Book review entity:

**Review**

This is what you get when you send GET requests to the API.

* id
* book_name
* book_author
* review_comment
* stars
* created_at

If you want to create a new book review, your POST request have to contain the following parameters.

**ReviewCreation**

* book_name
* book_author
* review_comment
* stars

This are the different options you can choose on **stars**

* One
* Two
* Three
* Four
* Five
