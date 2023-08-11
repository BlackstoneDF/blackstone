# Source:
```ts
event Join(e) {}
```
# Ast
```yml
span: 0..15 
file:
  event_decl:
    signature:
      decl_token:
        span: 0..4
      iden:
        string: Join
        span: 6..9
      params:
        string: e
        span: 10..12
    body:
      span: 14..15
      body: null
```
```yml
span: range
file: 
  event_decl:
    signature: null
    body:
      span: range
      body: block

```