## Overview
Calculate percentile

## Run
```shell script
git clone git@github.com:linrium/quantile-api.git
cd quantile-api
RUST_LOG=info cargo run --release
```

## Unit test
`cargo test`

## API
### Postman online document
https://documenter.getpostman.com/view/2939491/TzeTJV3d

### environments
- HOST: localhost:3000

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