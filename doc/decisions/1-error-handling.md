
Each endpoint has a response type or a specific api error type for that endpoint, lets call it `ExampleAPIError` from now on.

My functions have signatures such as this:
`fn example_api() -> Result<Response<ExampleResult>>`

The result comes from the generated error module using error_chain and the response is a generic struct that wraps a body as follows
```
pub struct Response<T> {
    pub body: T,
}
```

I would like to return the potential error deserialized from the `example_api()` function since we already have that information
ahead of time. I feel like I have 3 possible solutions:
1. Make the api error part of the error_chain error in a big `APIErrors` enum for all the type 
of API errors and document that this `example_api()` can return errors of that type.
The disadvantage here is that I feel like I am losing some information since the error
seems important enough that it should maybe be encoded in function signature.

2. Make the Response an enum that holds either a body or an APIError. This is a little 
analogous to the Result type but the disadvantage is that we are duplicating the error handling 
since a user of that function would have to handle errors twice and since my `Response` type isn't a 
result type won't benefit from using the `?` operator.

3. Similar to the above, make the Response an enum but map the error to the error case of the Result. 
This is quite ugly and will also confuse users since even though the Response type is an enum, 
it should essentially never actually be the error case. 
