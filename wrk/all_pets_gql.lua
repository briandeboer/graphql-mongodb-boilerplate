wrk.method = "POST"
wrk.body = '{"query":"query allPets { allPets(limit:25){ items{ name id petType age gender } } }","variables":null,"operationName":"allPets"}'
wrk.headers["Content-Type"] = "application/json"