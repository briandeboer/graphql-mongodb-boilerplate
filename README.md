# Rust graphql service boilerplate

This provides a simple graphql boilerplate for services. It uses:
- [actix-web](https://actix.rs/)
- [juniper](https://github.com/graphql-rust/juniper)
- [mongodb-cursor-pagination](https://github.com/briandeboer/mongodb-cursor-pagination)


Just change/add to the models, services and the graphql_schema accorindgly.

### Usage

Seed some data with...
```
cargo run --bin seed
```

Run the server with...
```
cargo run
```

And then test at:
```
http://localhost:8080/graphiql
```

#### Sample query for pets
```
{
  allPets(limit:4){
    pageInfo{
      startCursor
      nextCursor
      hasPreviousPage
      hasNextPage
    }
    pets{
      name
      id
      age
      petType
      gender
      owner{
        id
        username
      }
    }
    totalCount
  }
}
```

#### Sample query for owners
```
{
  allOwners {
    pageInfo {
      startCursor
      nextCursor
    }
    owners {
      id
      firstName
      lastName
      pets {
        id
        name
      }
    }
  }
}
```

## Inspiration and some resources to help
- [Example using juniper and diesel(SQL)](https://dev.to/open-graphql/building-powerful-graphql-servers-with-rust-3gla)
- [Mongodb cursor pagination](https://github.com/briandeboer/mongodb-cursor-pagination)
- [Details about paging](https://engineering.mixmax.com/blog/api-paging-built-the-right-way/)
- [More information about mongodb](http://alex.amiran.it/post/2018-08-16-rust-graphql-webserver-with-warp-juniper-and-mongodb.html)
- [Another mongodb juniper example](https://github.com/shareeff/rust_graphql_mongodb)