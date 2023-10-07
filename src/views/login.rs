use leptos::*;
use leptos_router::ActionForm;

use crate::{
    functions::{LoginResult, ResendVerificationMail},
    hooks::use_auth,
};

#[component]
pub fn LoginView() -> impl IntoView {
    let auth = use_auth();

    let result = move || {
        let result = auth.login.value();

        let msg: Option<LoginResult> = match result.get() {
            Some(return_value) => match return_value {
                Ok(result) => Some(result),
                Err(_) => Some(LoginResult::InternalServerError),
            },
            None => None,
        };
        msg
    };

    let message = move || {
        if let Some(result) = result() {
            result.to_string()
        } else {
            "".to_string()
        }
    };

    let is_ok = move || matches!(result(), Some(LoginResult::Ok));

    let need_to_verify_email = move || matches!(result(), Some(LoginResult::VerifyEmail));

    let (username, set_username) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());

    let resend_verification_email = move |_| {
        auth.resend_verification_email.dispatch(ResendVerificationMail {
            username: username(),
        });
    };

    view! {
        <Transition
            fallback=move || ()>
            {move || {
                let condition = move || {
                    let user = auth.user.get();
                    !matches!(user, Some(Ok(_)))
                };
                view!{
                    <Show
                        when=condition
                        fallback=|| view! { <section>"Logged in"</section>}>
                        <section class="login-view">
                            <ActionForm action=auth.login>
                                <Show
                                    when=move || result().is_some()
                                    fallback=|| view! { <span></span> }
                                >
                                    <div
                                        class="result"
                                        class:error=move || !is_ok()
                                        class:success=is_ok
                                    >
                                        {message()}
                                    </div>
                                    <Show
                                        when=need_to_verify_email
                                        fallback=|| view! {<></>}>
                                        <a
                                            href="#"
                                            on:click=resend_verification_email
                                            >"Resend Email"</a>
                                    </Show>
                                </Show>
                                <h1>"Login"</h1>
                                <label>
                                    <span>"Username"</span>
                                    <input
                                        type="text"
                                        name="username"
                                        prop:value=username
                                        on:input=move |ev| {
                                            set_username(event_target_value(&ev));
                                        }
                                        required/>
                                </label>
                                <label>
                                    <span>"Password"</span>
                                    <input
                                        type="password"
                                        name="password"
                                        prop:value=password
                                        on:input=move |ev| {
                                            set_password(event_target_value(&ev));
                                        }
                                        required/>
                                </label>
                                <button type="submit" class="primary">"Login"</button>
                            </ActionForm>
                        </section>
                </Show>
                }
            }}
        </Transition>
    }
}
