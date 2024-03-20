use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use server_fn::codec::GetUrl;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-actix-middleware.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let username = create_resource(|| {}, |_| get_username());

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=move |_| { username.refetch() }>"Reload"</button>
        <Suspense fallback=move || {
            view! { <p>Loading ...</p> }
        }>
            {move || {
                username
                    .get()
                    .map(|call_result| { call_result.map(|username| view! { <p>{username}</p> }) })
            }}

        </Suspense>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>"Not Found"</h1> }
}
#[server(input=GetUrl)]
pub async fn get_username() -> Result<String, ServerFnError> {
    use crate::ssr::middleware::JwtClaims;
    use leptos_actix::extract;
    use std::time::Duration;

    println!("==server_fn called==");
    let (jwt_claims,) = extract::<(JwtClaims,)>().await?;

    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("==server_fn end==\n");
    Ok(jwt_claims.username)
}
