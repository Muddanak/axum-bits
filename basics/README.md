# Basic
A demonstration of the basic implmentations for Axum to get up and going

## Modules
The modules are described as follows:

|Modules|Description|
:---:|---
[Start](https://github.com/Muddanak/axum-bits/blob/master/basics/start/src/main.rs) | A simple Axum server setup to get going right away.<p>It has a single route handler for a GET request to the default or "/" URL.<p> It has a fallback 404 just in case a wrong address is added.  The index request returns a &'static str.<p>URL Route:<br>/ (GET)
[Responders](https://github.com/Muddanak/axum-bits/blob/master/basics/responders/src/main.rs) | A demonstration of the various responders in Axum.<p>It has three routes, with the default ("/") route handling a GET and POST request, one route for an HTML response, and one route for a JSON response. There is a 404 fallback handler just in case.<p>URL Routes:<p>/ (GET and POST)<br>/html (GET)<br>/json (GET)
[Extractors](https://github.com/Muddanak/axum-bits/blob/master/basics/extractors/src/main.rs) | A demonstration of the various extractors in Axum. It is an extension of the Responders module.<p>It has four additional routes to handle extracting data from a URL request.  The methods cover a Query, Path, Form, and JSON.  The Form and JSON requests are handled via POST.<p>URL Routes:<p>/studentquery (GET)<br>/studentpath (GET)<br>/studentform (POST)<br>/studentjson (POST)
[State](https://github.com/Muddanak/axum-bits/blob/master/basics/state/src/main.rs) | A demonstration of passing a state for the server to use.  It is a struct that is constructed before the server is called, then the server is called with the state struct provided.  The state is immutable in this demonstration.<p>URL routes: /state
[Htmlfile](https://github.com/Muddanak/axum-bits/blob/master/basics/htmlfile/src/main.rs) | A demonstration of passing a static HTML file.  It is one method of doing so, as there are other ways to go about it.<p>URL Route:<br>/ (GET)
