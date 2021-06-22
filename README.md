## Overview
Calculate percentile

## Run
```shell script
git clone git@github.com:linrium/quantile-api.git
cd quantile-api
RUST_LOG=info cargo run --release
```

## Changelogs
### Update version 0.1.1
- Change f64 to f32 to improve memory usage
- Refactor common package (to errors and utils packages)
- Sort value in query service (remove sort in append service)
- Replace Mutex<Hashmap> to Dashmap (benchmark [here](https://github.com/xacrimon/conc-map-bench))
- Update load test with Dashmap

## Unit test
Pass 100% unit test
`cargo test`

## Coverage test
![image](https://user-images.githubusercontent.com/14315537/121771668-da221180-cb9a-11eb-9a0a-a737163b559e.png)
Following this instruction https://github.com/mozilla/grcov#how-to-get-grcov

## Load test
(Without caching)
![image](https://user-images.githubusercontent.com/14315537/121772318-e6a86900-cb9e-11eb-87ba-db2bb635ea39.png)

(With caching)
![image](https://user-images.githubusercontent.com/14315537/121773474-65ed6b00-cba6-11eb-9d77-93fa6614f5a4.png)

(With Dashmap)
![image](https://user-images.githubusercontent.com/14315537/122865028-df781c80-d34f-11eb-8524-3010a29dc0e4.png)

Following this instruction https://github.com/alexfernandez/loadtest
Example:
```shell script
loadtest -n 100000 -c 100 -m POST -T 'application/json' --data '{"poolId": 1, "percentile": 50}' http://localhost:3000/query
```

## API
### Postman online document
https://documenter.getpostman.com/view/2939491/TzeTJV3d

### Environments
- HOST: localhost:3000 or (deployed version https://compute-quantile.onrender.com)

### POST /append
#### Endpoint: {{HOST}}/append
#### Body:
```typescript
interface Body {
  poolId: number // required
  poolValues: number[] // required
}
```
#### Response:
```typescript
interface Response {
  status: "inserted" | "appended"
}
```
#### Example:
```shell script
curl --location --request POST "localhost:3000/append" \
--header "Content-Type: application/json" \
--data-raw "{
    \"poolId\": 1,
    \"poolValues\": [
    0,
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10
]
}"
```

### POST /query
#### Endpoint: {{HOST}}/query
#### Body:
```typescript
interface Body {
  poolId: number // required
  percentile: number // required, gte 0, lte 100
}
```
#### Response success:
```typescript
interface Response {
  quantile: number,
  count: number
}
```
#### Response error:
```typescript
interface Response {
  code: number,
  message: string
}
```
#### Example:
```shell script
curl --location --request POST "https://compute-quantile.onrender.com/query" \
--header "Content-Type: application/json" \
--data-raw "{
    \"poolId\": 1,
    \"percentile\": 50.6
}"
```
