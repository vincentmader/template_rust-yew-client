use crate::services::{
    dom::{self, text_field_is_empty},
    hashing::generate_hashed_password,
};
use rs_web_api_models::api_message::{
    ApiError, ApiMessage, LoginError, PasswordResetError, RegistrationError,
};
use std::collections::HashMap;
use stylist::css;
use web_sys::{wasm_bindgen::JsCast, Document, Element, HtmlInputElement};
use yew::prelude::*;

const BACKEND_URL: &str = "http://localhost:8000/api";

pub enum DisplayMode {
    Login,
    Registration,
    ForgotPassword,
}

#[function_component]
pub fn LoginScreen() -> Html {
    let display_mode = use_state(|| DisplayMode::Login);

    let class = css!(
        "
            --pad: 0.5em;
            --background-color: gray;
            --border-color: var(--background-color);
            --border: 1px solid var(--border-color);
            --border-radius: 5px;

            width: 100%;
            form {
                width: min(100%, max(30%, 20em));
                margin: auto;
            }
            p.title {
                --title-pad: 7vh;
                --font-scale-factor: 2.5;
                margin: 0px;
                padding-top: var(--title-pad);
                padding-bottom: var(--title-pad);
                text-align: center;
                font-size: calc(var(--font-scale-factor) * 1em);
                margin-top: calc(var(--pad) / var(--font-scale-factor));
            }

            input[type=text], input[type=password], input[type=button] {
                outline: 1px solid var(--border-color);
                border-radius: var(--border-radius);
                border: none;
                height: 2.5em;
            }

            input[type=text], input[type=password] {
                width: calc(100% - 3*var(--pad));
                padding-left: calc(1.5*var(--pad));
                padding-right: calc(1.5*var(--pad));
            }

            input[type=button] {
                cursor: pointer;
                background-color: white;
            }
            input[type=button].submit {
                background-color: var(--background-color);
                color: white;
            }

            .buttons {
                width: 100%;
            }
            .left {
                width: calc(50% - var(--pad));
                margin-right: var(--pad);
            }
            .right {
                width: calc(50% - var(--pad));
                margin-left: var(--pad);
            }

            input, label {
                font-size: 1em;
                margin-top: var(--pad);
                margin-bottom: var(--pad);
                width: calc(100%);
            }

            #info_text {
                text-align: center;
            }
            p.success {
                color: green;
            }
            p.error {
                color: red;
            }
            input[type=text].error, input[type=password].error {
                outline: 2px solid red;
            }
        "
    );

    let window = dom::window();
    let document = dom::document(window);
    let form = match *display_mode {
        DisplayMode::Login => {
            let forgot_password = {
                let display_mode = display_mode.clone();
                let document = document.clone();
                move |_| {
                    clear_info_text(&document);
                    display_mode.set(DisplayMode::ForgotPassword);
                }
            };
            let switch_to_registration = {
                let display_mode = display_mode.clone();
                let document = document.clone();
                move |_| {
                    clear_info_text(&document);
                    display_mode.set(DisplayMode::Registration);
                }
            };
            let submit = {
                move |_| {
                    let window = dom::window();
                    let document = dom::document(window);

                    let user_info = dom::read_value_from_text_field(&document, "user_info");
                    let pass_word = dom::read_value_from_text_field(&document, "pass_word");

                    clear_info_text(&document);
                    let is_valid = assure_validity_of_fields(DisplayMode::Login, &document);

                    if is_valid {
                        wasm_bindgen_futures::spawn_local(async move {
                            let client = reqwest::Client::new();
                            let url = format!("{}/login", BACKEND_URL);

                            let pass_hash = generate_hashed_password(&pass_word);
                            let body = HashMap::from([
                                ("user_info", &user_info),
                                ("pass_hash", &pass_hash), //
                            ]);
                            let result = client.post(url).json(&body).send().await.unwrap();

                            // let headers = result.headers();
                            // let cookies = result.cookies();
                            // let status = result.status();

                            let info_text = document.get_element_by_id("info_text").unwrap();
                            let api_msg: ApiMessage = result.json().await.unwrap();
                            let api_msg_str: String = api_msg.clone().into();
                            match &api_msg {
                                ApiMessage::Ok(msg) => {
                                    set_info_text_ok(&info_text, &api_msg_str);
                                }
                                ApiMessage::Err(msg) => {
                                    set_info_text_error(&info_text, &api_msg_str);
                                    if let ApiError::LoginError(msg) = msg {
                                        let f = mark_textfield_validity;
                                        match msg {
                                            LoginError::InvalidLoginCredentials => {
                                                f(&document, "user_info", false);
                                                f(&document, "pass_word", false);
                                            }
                                            LoginError::EmptyUserInfo => {
                                                f(&document, "pass_word", false);
                                            }
                                            LoginError::EmptyPassWord => {
                                                f(&document, "user_info", false);
                                            }
                                        }
                                    }
                                }
                            };
                            let msg: String = api_msg.into();
                            gloo::console::log!(msg);
                        });
                    }
                }
            };

            html! {
                <form>
                    <p class="title">{ "Login" }</p>
                    <input type="text" name="user_info" id="user_info" placeholder="Username or E-Mail" /> <br/>
                    <input type="password" name="pass_word" id="pass_word" placeholder="Password" /> <br/>
                    <div class="buttons">
                        <input type="button" class="left" value="Forgot Password?" onclick={ forgot_password } />
                        <input type="button" class="right submit" value="Submit" onclick={ submit } />
                    </div>
                    <input type="button" value="Register new Account" onclick={ switch_to_registration } />
                    <p id="info_text"></p>
                </form>
            }
        }
        DisplayMode::Registration => {
            let switch_to_login = {
                let display_mode = display_mode.clone();
                let document = document.clone();
                move |_| {
                    clear_info_text(&document);
                    display_mode.set(DisplayMode::Login);
                }
            };

            let submit = {
                move |_| {
                    let window = dom::window();
                    let document = dom::document(window);

                    let user_name = dom::read_value_from_text_field(&document, "user_name");
                    let mail_addr = dom::read_value_from_text_field(&document, "mail_addr");
                    let pass_word = dom::read_value_from_text_field(&document, "pass_word");
                    let pwconfirm = dom::read_value_from_text_field(&document, "pwconfirm");

                    clear_info_text(&document);
                    let is_valid = assure_validity_of_fields(DisplayMode::Registration, &document);
                    // TODO Send registration form data to server.

                    if is_valid {
                        wasm_bindgen_futures::spawn_local(async move {
                            let client = reqwest::Client::new();
                            let url = format!("{}/register", BACKEND_URL);

                            let pass_hash = generate_hashed_password(&pass_word);
                            println!("{}", pass_hash);
                            let body = HashMap::from([
                                ("user_name", &user_name),
                                ("mail_addr", &mail_addr),
                                ("pass_hash", &pass_hash),
                            ]);
                            let result = client.post(url).json(&body).send().await.unwrap();

                            // let headers = result.headers();
                            // let cookies = result.cookies();
                            // let status = result.status();

                            let info_text = document.get_element_by_id("info_text").unwrap();
                            let api_msg: ApiMessage = result.json().await.unwrap();
                            let api_msg_str: String = api_msg.clone().into();
                            match &api_msg {
                                ApiMessage::Ok(msg) => {
                                    set_info_text_ok(&info_text, &api_msg_str);
                                }
                                ApiMessage::Err(msg) => {
                                    set_info_text_error(&info_text, &api_msg_str);
                                    if let ApiError::RegistrationError(msg) = msg {
                                        let f = mark_textfield_validity;
                                        match msg {
                                            RegistrationError::EmptyUserName
                                            | RegistrationError::UserNameExistsAlready
                                            | RegistrationError::InvalidUserNameFormat => {
                                                f(&document, "user_name", false)
                                            }
                                            RegistrationError::EmptyMailAddress
                                            | RegistrationError::MailAddressExistsAlready
                                            | RegistrationError::InvalidMailAddressFormat => {
                                                f(&document, "mail_addr", false)
                                            }
                                            RegistrationError::EmptyPassWord
                                            | RegistrationError::InvalidPasswordFormat => {
                                                f(&document, "pass_word", false);
                                                f(&document, "pwconfirm", false);
                                            }
                                            RegistrationError::EmptyPassWordConfirm
                                            | RegistrationError::InvalidPasswordConfirmation => {
                                                f(&document, "pwconfirm", false)
                                            }
                                        }
                                    }
                                }
                            };
                            let msg: String = api_msg.into();
                            gloo::console::log!(msg);
                        });
                    }
                }
            };

            html! {
                <form>
                    <p class="title">{ "Registration" }</p>
                    <input type="text" name="user_name" id="user_name" placeholder="Username" /> <br/>
                    <input type="text" name="mail_addr" id="mail_addr" placeholder="E-Mail" /> <br/>
                    <input type="password" name="pass_word" id="pass_word" placeholder="Password" /> <br/>
                    <input type="password" name="pwconfirm"
                        id="pwconfirm" placeholder="Confirm Password" /> <br/>
                    <div class="buttons">
                        <input type="button" class="left" value="Back to Login" onclick={ switch_to_login } />
                        <input type="button" class="right submit" value="Submit" onclick={ submit } />
                    </div>
                    <p id="info_text"></p>
                </form>
            }
        }
        DisplayMode::ForgotPassword => {
            let switch_to_login = {
                let display_mode = display_mode.clone();
                let document = document.clone();
                move |_| {
                    clear_info_text(&document);
                    display_mode.set(DisplayMode::Login);
                }
            };
            let submit = {
                move |_| {
                    let window = dom::window();
                    let document = dom::document(window);

                    let user_info = dom::read_value_from_text_field(&document, "user_info");

                    clear_info_text(&document);
                    let is_valid =
                        assure_validity_of_fields(DisplayMode::ForgotPassword, &document);

                    if is_valid {
                        wasm_bindgen_futures::spawn_local(async move {
                            let client = reqwest::Client::new();
                            let url = format!("{}/reset_password", BACKEND_URL);
                            let body = HashMap::from([("user_info", &user_info)]);
                            let result = client.post(url).json(&body).send().await.unwrap();

                            // let headers = result.headers();
                            // let cookies = result.cookies();
                            // let status = result.status();

                            let info_text = document.get_element_by_id("info_text").unwrap();
                            let api_msg: ApiMessage = result.json().await.unwrap();
                            let api_msg_str: String = api_msg.clone().into();
                            match &api_msg {
                                ApiMessage::Ok(_msg) => {
                                    set_info_text_ok(&info_text, &api_msg_str);
                                }
                                ApiMessage::Err(msg) => {
                                    set_info_text_error(&info_text, &api_msg_str);
                                    if let ApiError::PasswordResetError(
                                        PasswordResetError::AccountDoesNotExist,
                                    ) = msg
                                    {
                                        mark_textfield_validity(&document, "user_info", false);
                                    }
                                }
                            };
                            let msg: String = api_msg.into();
                            gloo::console::log!(msg);
                        });
                    }
                }
            };

            html! {
                <form>
                    <p class="title">{ "Password Reset" }</p>
                    <input type="text" name="user_info" id="user_info" placeholder="Username or E-Mail" /> <br/>
                    <div class="buttons">
                        <input type="button" class="left" value="Back to Login" onclick={ switch_to_login } />
                        <input type="button" class="right submit" value="Submit" onclick={ submit } />
                    </div>
                    <p id="info_text"></p>
                </form>
            }
        }
    };

    html! {
        <div { class }>
            { form }
        </div>
    }
}

fn assure_validity_of_fields(display_mode: DisplayMode, document: &Document) -> bool {
    let info_text = document.get_element_by_id("info_text").unwrap();

    let mut is_valid = true;
    match display_mode {
        DisplayMode::Login => {
            mark_textfield_validity(document, "user_info", true);
            mark_textfield_validity(document, "pass_word", true);

            let user_info_text_field = document.get_element_by_id("user_info").unwrap();
            let pass_word_text_field = document.get_element_by_id("pass_word").unwrap();

            let user_info = user_info_text_field
                .clone()
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value();
            let pass_word = pass_word_text_field
                .clone()
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value();

            if text_field_is_empty(&user_info_text_field) {
                let msg = LoginError::EmptyUserInfo;
                let msg: String = ApiMessage::Err(ApiError::LoginError(msg)).into();
                mark_textfield_validity(document, "user_info", false);
                set_info_text_error(&info_text, &msg);
                is_valid = false;
            } else if text_field_is_empty(&pass_word_text_field) {
                let msg = LoginError::EmptyPassWord;
                let msg: String = ApiMessage::Err(ApiError::LoginError(msg)).into();
                mark_textfield_validity(document, "pass_word", false);
                set_info_text_error(&info_text, &msg);
                is_valid = false;
            } else if !(rs_web_api_models::validation::is_valid_user_name(&user_info)
                || rs_web_api_models::validation::is_valid_mail_addr(&user_info))
                || !rs_web_api_models::validation::is_valid_pass_word(&pass_word)
            {
                let msg = LoginError::InvalidLoginCredentials;
                let msg: String = ApiMessage::Err(ApiError::LoginError(msg)).into();
                mark_textfield_validity(document, "user_info", false);
                mark_textfield_validity(document, "pass_word", false);
                set_info_text_error(&info_text, &msg);
                is_valid = false;
            }
        }
        DisplayMode::Registration => {
            mark_textfield_validity(document, "user_name", true);
            mark_textfield_validity(document, "mail_addr", true);
            mark_textfield_validity(document, "pass_word", true);
            mark_textfield_validity(document, "pwconfirm", true);

            let user_name_text_field = document.get_element_by_id("user_name").unwrap();
            let mail_addr_text_field = document.get_element_by_id("mail_addr").unwrap();
            let pass_word_text_field = document.get_element_by_id("pass_word").unwrap();
            let pwconfirm_text_field = document.get_element_by_id("pwconfirm").unwrap();

            let user_name_is_valid = assure_validity_of_user_name(document);
            let mail_addr_is_valid = assure_validity_of_mail_addr(document);
            let pass_word_is_valid = assure_validity_of_pass_word(document);
            let pwconfirm_is_valid = pass_word_matches_confirm(document);

            if text_field_is_empty(&user_name_text_field) {
                let msg = RegistrationError::EmptyUserName;
                let msg: String = ApiMessage::Err(ApiError::RegistrationError(msg)).into();
                mark_textfield_validity(document, "user_name", false);
                set_info_text_error(&info_text, &msg);
                is_valid = false;
            } else if !user_name_is_valid {
                let msg = RegistrationError::InvalidUserNameFormat;
                let msg: String = ApiMessage::Err(ApiError::RegistrationError(msg)).into();
                mark_textfield_validity(document, "user_name", false);
                set_info_text_error(&info_text, &msg);
                is_valid = false;
            } else if text_field_is_empty(&mail_addr_text_field) {
                let msg = RegistrationError::EmptyMailAddress;
                let msg: String = ApiMessage::Err(ApiError::RegistrationError(msg)).into();
                mark_textfield_validity(document, "mail_addr", false);
                set_info_text_error(&info_text, &msg);
                is_valid = false;
            } else if !mail_addr_is_valid {
                let msg = RegistrationError::InvalidMailAddressFormat;
                let msg: String = ApiMessage::Err(ApiError::RegistrationError(msg)).into();
                mark_textfield_validity(document, "mail_addr", false);
                set_info_text_error(&info_text, &msg);
                is_valid = false;
            } else if text_field_is_empty(&pass_word_text_field) {
                let msg = RegistrationError::EmptyPassWord;
                let msg: String = ApiMessage::Err(ApiError::RegistrationError(msg)).into();
                mark_textfield_validity(document, "pass_word", false);
                set_info_text_error(&info_text, &msg);
                is_valid = false;
            } else if !pass_word_is_valid {
                let msg = RegistrationError::InvalidPasswordFormat;
                let msg: String = ApiMessage::Err(ApiError::RegistrationError(msg)).into();
                mark_textfield_validity(document, "pass_word", false);
                set_info_text_error(&info_text, &msg);
                is_valid = false;
            } else if text_field_is_empty(&pwconfirm_text_field) {
                let msg = RegistrationError::EmptyPassWordConfirm;
                let msg: String = ApiMessage::Err(ApiError::RegistrationError(msg)).into();
                mark_textfield_validity(document, "pwconfirm", false);
                set_info_text_error(&info_text, &msg);
                is_valid = false;
            } else if !pwconfirm_is_valid {
                let msg = RegistrationError::InvalidPasswordConfirmation;
                let msg: String = ApiMessage::Err(ApiError::RegistrationError(msg)).into();
                mark_textfield_validity(document, "pwconfirm", false);
                set_info_text_error(&info_text, &msg);
                is_valid = false;
            }
        }
        DisplayMode::ForgotPassword => {
            mark_textfield_validity(document, "user_info", true);

            let user_info_is_valid = assure_validity_of_user_info(document);
            let user_info_text_field = document.get_element_by_id("user_info").unwrap();
            if text_field_is_empty(&user_info_text_field) {
                let msg = PasswordResetError::EmptyUserInfo;
                let msg: String = ApiMessage::Err(ApiError::PasswordResetError(msg)).into();
                mark_textfield_validity(document, "user_info", false);
                set_info_text_error(&info_text, &msg);
                is_valid = false;
            } else if !user_info_is_valid {
                let msg = PasswordResetError::AccountDoesNotExist;
                let msg: String = ApiMessage::Err(ApiError::PasswordResetError(msg)).into();
                mark_textfield_validity(document, "user_info", false);
                set_info_text_error(&info_text, &msg);
                is_valid = false;
            }
        }
    };
    is_valid
}

fn assure_validity_of_user_name(document: &Document) -> bool {
    let user_name_text_field = document.get_element_by_id("user_name").unwrap();
    let user_name = user_name_text_field
        .clone()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value();
    rs_web_api_models::validation::is_valid_user_name(&user_name)
}
fn assure_validity_of_user_info(document: &Document) -> bool {
    let user_info_text_field = document.get_element_by_id("user_info").unwrap();
    let user_info = user_info_text_field
        .clone()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value();
    rs_web_api_models::validation::is_valid_user_name(&user_info)
        || rs_web_api_models::validation::is_valid_mail_addr(&user_info)
}
fn assure_validity_of_mail_addr(document: &Document) -> bool {
    let mail_addr_text_field = document.get_element_by_id("mail_addr").unwrap();
    let mail_addr = mail_addr_text_field
        .clone()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value();
    rs_web_api_models::validation::is_valid_mail_addr(&mail_addr)
}
fn assure_validity_of_pass_word(document: &Document) -> bool {
    let pass_word_text_field = document.get_element_by_id("pass_word").unwrap();
    let pass_word = pass_word_text_field
        .clone()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value();
    rs_web_api_models::validation::is_valid_pass_word(&pass_word)
}
fn pass_word_matches_confirm(document: &Document) -> bool {
    let pass_word_text_field = document.get_element_by_id("pass_word").unwrap();
    let pwconfirm_text_field = document.get_element_by_id("pwconfirm").unwrap();
    let pass_word = pass_word_text_field
        .clone()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value();
    let pwconfirm = pwconfirm_text_field
        .clone()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value();
    pass_word == pwconfirm
}
fn clear_info_text(document: &Document) {
    let info_text = document.get_element_by_id("info_text").unwrap();
    info_text.set_inner_html("");
}
fn set_info_text_ok(info_text: &Element, text: &str) {
    info_text.class_list().remove_1("error").unwrap();
    info_text.set_class_name("success");
    info_text.set_inner_html(text);
}
fn set_info_text_error(info_text: &Element, text: &str) {
    info_text.class_list().remove_1("success").unwrap();
    info_text.set_class_name("error");
    info_text.set_inner_html(text);
}
fn mark_textfield_validity(document: &Document, element_id: &str, is_valid: bool) {
    let text_field = document.get_element_by_id(element_id).unwrap();
    match is_valid {
        true => text_field.class_list().remove_1("error").unwrap(),
        false => text_field.set_class_name("error"),
    };
}
