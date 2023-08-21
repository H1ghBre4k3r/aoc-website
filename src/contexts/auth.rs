use cfg_if::cfg_if;

use leptos::*;

use crate::functions::{Login, Logout};

cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::functions::use_identity;
}
}

#[derive(Clone)]
pub struct AuthContext {
    pub login: Action<Login, Result<(), ServerFnError>>,
    pub logout: Action<Logout, Result<(), ServerFnError>>,
    pub user: Resource<(usize, usize), Result<String, ServerFnError>>,
}

impl AuthContext {
    fn new(cx: Scope) -> Self {
        let login = create_server_action::<Login>(cx);
        let logout = create_server_action::<Logout>(cx);

        let user = create_resource(
            cx,
            move || (login.version().get(), logout.version().get()),
            move |_| get_user_id(cx),
        );

        AuthContext {
            login,
            logout,
            user,
        }
    }
}

#[server(GetUserId, "/api")]
async fn get_user_id(cx: Scope) -> Result<String, ServerFnError> {
    let Some(req) = use_context::<actix_web::HttpRequest>(cx) else {
        log!("some err");
        return Err(ServerFnError::MissingArg(
            "Failed to get the Request".to_string(),
        ));
    };

    let identity = use_identity(&req)?;

    let id = identity
        .id()
        .map_err(|_| ServerFnError::ServerError("User Not Found!".to_string()))?;

    Ok(id)
}

#[component]
pub fn AuthContextProvider(cx: Scope, children: Children) -> impl IntoView {
    provide_context(cx, AuthContext::new(cx));

    children(cx)
}

pub fn use_auth(cx: Scope) -> AuthContext {
    use_context::<AuthContext>(cx).expect("no valid AuthContext given!")
}