# utilitybot miscellaneous backends

This worker does random stuff for [utilitybot](https://github.com/anand2312/utilitybot)
I just wanted to try Workers KV, and Workers with Rust, and also avoid having to set up a database for the bot rn

# API Documentation
All API requests need the API-KEY to be passed as a header.

## GET `/rolenames`
### Returns:
Status code 200
#### JSON
```
{"rolenames": String[]}
```

## POST `/rolenames`
### JSON Body:
```
rolename: String
added_by: Discord ID (String)
```
### Returns:
Status code 200 (no body)

## DELETE `/rolenames`
### JSON Body:
```
rolename: String (the name to be deleted)
```
### Returns:
Status code 200 (no body)
