use stylist::css;
use yew::prelude::*;

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
        "
    );

    let form = match *display_mode {
        DisplayMode::Login => {
            let forgot_password = {
                let display_mode = display_mode.clone();
                move |_| display_mode.set(DisplayMode::ForgotPassword)
            };
            let switch_to_registration = {
                let display_mode = display_mode.clone();
                move |_| display_mode.set(DisplayMode::Registration)
            };
            let submit = {
                move |_| {
                    // TODO Send login form data to server.
                }
            };

            html! {
                <form>
                    <p class="title">{ "Login" }</p>
                    <input type="text" name="user_info" placeholder="Username or E-Mail" /> <br/>
                    <input type="password" name="pass_word" placeholder="Password" /> <br/>
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
                move |_| display_mode.set(DisplayMode::Login)
            };
            let submit = {
                move |_| {
                    // TODO Send registration form data to server.
                }
            };

            html! {
                <form>
                    <p class="title">{ "Registration" }</p>
                    <input type="text" name="user_name" placeholder="Username" /> <br/>
                    <input type="text" name="mail_addr" placeholder="E-Mail" /> <br/>
                    <input type="password" name="pass_word" placeholder="Password" /> <br/>
                    <input type="password" name="pass_word_confirm" placeholder="Confirm Password" /> <br/>
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
                move |_| display_mode.set(DisplayMode::Login)
            };
            let submit = {
                move |_| {
                    // TODO Request new password.
                }
            };

            html! {
                <form>
                    <p class="title">{ "Password Reset" }</p>
                    <input type="text" name="user_info" placeholder="Username or E-Mail" /> <br/>
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
