# How To Run
Create an .env file in the root folder, and initialize PORT with app port and MONGODB_HOST with MongoDB's host, e.g. `localhost`

# Current State
1. As of now, it starts a server and connects to a mongodb instance. It persists the database connection, rather than establishing connection when required, which is not only inefficient but resource intensive at high traffic.
2. POST request working at http://127.0.0.1:8080/postdata. Pass {"name":""} as the raw body.


Created with ♥ using Ubuntu 20.04.1 on Windows 10 via WSL2.