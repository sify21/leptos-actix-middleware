Showing that actix middlewares won't run if server functions aren't called from the client side. 

Middlewares won't run in:
- Async Rendering
- In-Order Streaming
- Out-Of-Order Streaming (includes Partially-blocked out-of-order streaming)

Middlewares will run in:
- Synchronous Rendering


## 结论

在所有non-sync mode下，auth-protection(the middleware) 应该被作用于 view/html endpoint，而不是单独配在api endpoint（像leptos-axum的#[middleware]单独放server function上）。

因为在non-sync mode下，view（#[component]）调用server function是纯后端函数调用，返回给前台的是个html stream(async是调完返回整个html)。所以前台唯一与后台交互的就是这个请求html的接口。所以auth-protection应该放在这个html接口上（就像有些网站，访问某些页面会跳到一个中间页提示：你必须登录才能访问此页面，然后跳到登录页）。

只有sync-mode，才跟原来前后端分离的模式类似(所有访问server function都是浏览器发起个请求)。

https://discord.com/channels/1031524867910148188/1219644641062686771

Pasting here in case the link is dead

> @gbj: I think you might be misunderstanding either what I’m saying or how server functions run — you have added middleware for the separate server function endpoint, which is only called when a server function is called from the client. When a server function is called during server rendering, that endpoint is not called, the function just runs. So the middleware needs to be added on the routes being added to the router by leptos_routes. 
>
> This has been what I’m trying to say: nothing is adding that middleware to the leptos_routes

> @me: I dont understand, do you mean add middlewares to the routes that returns html? Not the /api/ routes?

> @gbj: Yes, isn’t that what the question is about? The fact that the middleware is not applied during HTML rendering?

> @me: Oh, I thought middlewares are only applied to server funtion endpoints. I only have backend api developing experience, so htmls/views used to be just static files for me that don't need auth-protection. But how do you make exceptions for pages that don't need authentication, for example the login page? 
> 
> Oh, I can use a whitelist in the auth middleware

> @gbj: I guess it depends on the kind of backend work you do… dynamically rendering HTML in response to a request has been the default for the web for a long time (since PHP was created) but if you’re mostly doing APIs I can see why that would not be obvious!
>
>(If you haven’t read the chapter in the book about how the SSR process works it might be worth it!)

> @me: So for axum, if a #middleware is put on a server funtion, it not only applys to /api endpoints, but also applys to any html/view endpoint that uses that server function, am I right?

> @gbj: No, that’s actually not the case and is an oversight/hard problem/good reason not to use middleware per server function. The further people push the mental model from “a server function is a function I call on the server” to “a server function is an API endpoint” the more it breaks. 
> 
> Authorization like that, if it needs to happen during HTML rendering, is probably best done either inside the server function, or in the component that calls it

> @me: I see, thanks for the clarification!


