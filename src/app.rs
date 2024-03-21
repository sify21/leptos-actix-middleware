use cookie::SameSite;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::{use_cookie_with_options, utils::FromToStringCodec, UseCookieOptions};
use server_fn::codec::GetUrl;

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
                    <Route path="/cookie1" view=CookiePage1/>
                    <Route path="/cookie2" view=CookiePage2/>
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

#[component]
fn CookiePage1() -> impl IntoView {
    let navigate = use_navigate();
    let AppData { set_cookie2, .. } = expect_context::<AppData>();
    let validate = create_server_action::<Validate>();
    let validate_result = validate.value();
    view! {
        <h1>This is CookiePage1</h1>
        <A href="/cookie2">"Go to CookiePage2"</A>
        <ActionForm action=validate>
            <input type="text" name="first" placeholder="first"/>
            <input type="text" name="second" placeholder="second"/>
            <input type="submit" value="Validate"/>
        </ActionForm>
        <p>
            {move || {
                match validate_result.get() {
                    Some(Ok(s)) => {
                        set_cookie2.set(Some(s));
                        navigate(
                            "/cookie2",
                            NavigateOptions {
                                resolve: false,
                                replace: true,
                                ..Default::default()
                            },
                        );
                        None
                    }
                    Some(Err(ServerFnError::ServerError(e))) => {
                        Some(format!("validate error: {}", e))
                    }
                    Some(Err(e)) => Some(format!("unknown error: {}", e)),
                    None => Some("No value yet!".to_string()),
                }
            }}

        </p>
    }
}
#[component]
fn CookiePage2() -> impl IntoView {
    view! {
        <h1>This is CookiePage2</h1>
        <A href="/cookie1">"Go to CookiePage1"</A>
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
#[server]
pub async fn validate(first: String, second: String) -> Result<String, ServerFnError> {
    if first.eq(&second) {
        Ok("Same".to_string())
    } else {
        Err(ServerFnError::new("Different"))
    }
}
