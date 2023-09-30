# Rust API with Docker
This project is a Rust API that uses Docker to run the App and the Database.

## Requirements

Install Docker following the official guide ~> https://docs.docker.com/engine/install/

## Getting Started
To get started with this project, you will need:

- Clone this repository:
  ```
  git clone https://github.com/hmanzoni/rust-crud-api.git
  ```
- Launch the containers using the command:
  ```
  docker-compose build
  docker-compose up -d
  ```

## Database

The app uses a PostgreSQL database to store user data. The database is configured in the docker-compose.yml file.

## Usage

The API is exposed on port 8080. 
You can access it using a command-line tool like `curl` or a tool like: 

- App ~> [Postman](https://www.postman.com/)
- VS Code extension ~> [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client)

In the file `test.http`, you can find all the requests available to run the queries using the *REST Client* extension.

### API Calls with `curl`

- Get a list of all users
  ```
  curl http://localhost:8080/users
  ```
- Get a specific user
  ```
  curl http://localhost:8080/users/1
  ```
- Create a new user
  ```
  curl -X POST http://localhost:8080/users -d '{"name": "John Doe", "email": "john.doe@test.com"}' --header "Content-Type: application/json"
  ```
- Update a user
  ```
  curl -X PUT http://localhost:8080/users/1 -d '{"name": "Jane Doe", "email": "jane.doe@test.com"}' --header "Content-Type: application/json"
  ```
- Delete a user
  ```
  curl -X DELETE http://localhost:8080/users/1
  ```

## Contributing

Contributions to this project are welcome. Please fork the repository and create a pull request for your changes.

## License

This project is licensed under the MIT License.