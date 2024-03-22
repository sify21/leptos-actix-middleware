use cookie::SameSite;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::{use_cookie_with_options, utils::FromToStringCodec, UseCookieOptions};
use server_fn::codec::GetUrl;

pub mod cookie_page;

#[derive(Debug, Clone)]
pub struct AppData {
    pub cookie1: Signal<Option<String>>,
    pub set_cookie1: WriteSignal<Option<String>>,
    pub cookie2: Signal<Option<String>>,
    pub set_cookie2: WriteSignal<Option<String>>,
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let (cookie1, set_cookie1) = use_cookie_with_options::<String, FromToStringCodec>(
        "cookie1",
        UseCookieOptions::default()
            .max_age(3600_000 * 24)
            .path("/")
            .same_site::<SameSite>(Some(SameSite::Strict)),
    );
    create_effect(move |_| {
        log::info!("Effects: cookie1 is changed to {:?}", cookie1.get());
    });
    let (cookie2, set_cookie2) = use_cookie_with_options::<String, FromToStringCodec>(
        "cookie2",
        UseCookieOptions::default()
            .max_age(3600_000 * 24)
            .path("/")
            .same_site::<SameSite>(Some(SameSite::Strict)),
    );
    create_effect(move |_| {
        log::info!("Effects: cookie2 is changed to {:?}", cookie2.get());
    });
    provide_context(AppData {
        cookie1,
        set_cookie1,
        cookie2,
        set_cookie2,
    });
    match cookie1.get_untracked() {
        None => {
            let value = if cfg!(feature = "ssr") {
                "set-by-server"
            } else {
                "set-by-browser"
            };
            log::info!("cookie1 not set, setting to {}", value);
            set_cookie1.set(Some(value.to_string()));
        }
        Some(s) => {
            log::info!("cookie1 is set, current value {}", s);
        }
    };

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-actix-middleware.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/cookie1" view=cookie_page::CookiePage1/>
                    <Route path="/local-cookie1" view=cookie_page::LocalCookiePage1/>
                    <Route path="/cookie2" view=cookie_page::CookiePage2/>
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
