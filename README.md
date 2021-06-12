## Overview
Calculate percentile

## Run
```shell script
git clone git@github.com:linrium/quantile-api.git
cd quantile-api
RUST_LOG=info cargo run --release
```

## Unit test
Pass 100% unit test
`cargo test`

## Coverage test
![image](https://user-images.githubusercontent.com/14315537/121771668-da221180-cb9a-11eb-9a0a-a737163b559e.png)
Following this instruction https://github.com/mozilla/grcov#how-to-get-grcov

## Load test
(Without caching)
![image](https://user-images.githubusercontent.com/14315537/121772318-e6a86900-cb9e-11eb-87ba-db2bb635ea39.png)
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
curl --location --request POST 'https://compute-quantile.onrender.com/append' \
--data-raw '{
    "poolId": 1,
    "poolValues": [1,2,3]
}'
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
curl --location --request POST 'https://compute-quantile.onrender.com/query' \
--data-raw '{
    "poolId": 1,
    "percentile": 50
}'
```
