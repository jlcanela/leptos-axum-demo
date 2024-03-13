use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_oidc::{Auth, AuthParameters};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let auth_parameters = AuthParameters {
        auth_endpoint: "https://ENDPOINT/auth/realms/REALM/protocol/openid-connect/auth".to_string(),
        token_endpoint: "https://ENDPOINT/auth/realms/REALM/protocol/openid-connect/token".to_string(),
        logout_endpoint: "https://ENDPOINT/auth/realms/REALM/protocol/openid-connect/logout".to_string(),
        client_id: "CLIENT_ID".to_string(),
        redirect_uri: "http://localhost:3000/profile".to_string(),
        post_logout_redirect_uri: "http://localhost:3000/bye".to_string(),
        scope: Some("openid".to_string()),
    };

    let auth = Auth::init(auth_parameters);

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-axum-demo.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
