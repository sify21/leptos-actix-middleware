use cookie::SameSite;
use leptos::*;
use leptos_router::*;
use leptos_use::{use_cookie_with_options, utils::FromToStringCodec, UseCookieOptions};

use crate::app::AppData;

#[component]
pub fn CookiePage1() -> impl IntoView {
    let navigate = use_navigate();
    let AppData { set_cookie2, .. } = expect_context::<AppData>();
    let validate = create_server_action::<Validate>();
    let validate_result = validate.value();
    view! {
        <h1>This is CookiePage1.</h1>
        <p>
            "Click 'Validate' button will set a cookie named 'cookie2', using a WriteSignal retrieved from global context, and then redirect to page /cookie2."
        </p>
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
                    None => Some("No Validate result yet!".to_string()),
                }
            }}

        </p>
    }
}

#[component]
pub fn LocalCookiePage1() -> impl IntoView {
    let navigate = use_navigate();
    let (_, set_cookie2) = use_cookie_with_options::<String, FromToStringCodec>(
        "cookie2",
        UseCookieOptions::default()
            .max_age(3600_000 * 24)
            .path("/")
            .same_site::<SameSite>(Some(SameSite::Strict)),
    );
    let validate = create_server_action::<Validate>();
    let validate_result = validate.value();
    view! {
        <h1>This is LocalCookiePage1</h1>
        <p>
            "Click 'Validate' button will set a cookie named 'cookie2', using a WriteSignal created exactly in this page, and then redirect to page /cookie2."
        </p>
        <A href="/cookie2">"Go to CookiePage2"</A>
        <ActionForm action=validate>
            <input type="text" name="first" placeholder="first"/>
            <input type="text" name="second" placeholder="second"/>
            <input type="submit" value="Validate"/>
        </ActionForm>
        <p>
            {move || match validate_result.get() {
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
                Some(Err(ServerFnError::ServerError(e))) => Some(format!("validate error: {}", e)),
                Some(Err(e)) => Some(format!("unknown error: {}", e)),
                None => Some("No Validate result yet!".to_string()),
            }}

        </p>
    }
}

#[component]
pub fn CookiePage2() -> impl IntoView {
    view! {
        <h1>This is CookiePage2</h1>
        <p>
            <A href="/cookie1">"Go to CookiePage1"</A>
        </p>
        <p>
            <A href="/local-cookie1">"Go to LocalCookiePage1"</A>
        </p>
    }
}

#[server]
pub async fn validate(first: String, second: String) -> Result<String, ServerFnError> {
    use chrono::Utc;

    if first.eq(&second) {
        Ok(format!("Same - {}", Utc::now().to_rfc3339()))
    } else {
        Err(ServerFnError::new(format!(
            "Different - {}",
            Utc::now().to_rfc3339()
        )))
    }
}
