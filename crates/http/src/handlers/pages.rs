pub(crate) async fn root() -> axum::response::Html<&'static str> {
    axum::response::Html(
        r#"
        <html>
            <head>
                <title>Lockpad</title>
            </head>
            <body>
                <h1>Hello, World!</h1>
            </body>

            <a href="/login">Login</a>
            <a href="/signup-screen">Signup</a>
        </html>
    "#,
    )
}

/// Sends a screen that asks the user to provide credentials.
pub(crate) async fn login_screen() -> axum::response::Html<&'static str> {
    axum::response::Html(
        r#"
        <h1>log in</h1>
        <form id="login-form">
            <input type="text" id="username" name="username" placeholder="username" />
            <input type="password" id="password" name="password" placeholder="password" />
            <input type="submit" value="Login" />
        </form>

        <script>
            const form = document.getElementById("login-form");
            form.onsubmit = function(event) {
                console.log("submitting form");
                event.preventDefault();
                const data = new FormData(form);
                const username = data.get("username");
                const password = data.get("password");

                // Perform a POST request to /authorize
                // If the request is successful, the s
                //
                fetch("/authorize", {
                    method: "POST",
                    body: JSON.stringify({ username, password }),
                    headers: {
                        "Content-Type": "application/json",
                    },
                })
                .then(response => response.json())
                .then(data => {
                    console.log("Success:", data);
                    // navigate to the main page
                    // TODO: this value should be populated from the server
                    window.location.href = "/";
                })
                .catch((error) => {
                    console.error("Error:", error);
                });
            }
        </script>

        <style>
            form {
                display: flex;
                flex-direction: column;
                align-items: center;
            }

            input {
                margin: 0.5rem;
            }

            input[type="submit"] {
                width: 100px;
            }

            input[type="text"], input[type="password"] {
                width: 200px;
            }

            input[type="text"]:focus, input[type="password"]:focus {
                outline: none;
            }

            h1 {
                text-align: center;
            }
        </style>
    "#,
    )
}

/// Sends a screen that asks the user to provide credentials.
/// These credentials will be used to create a new account.
/// This closely follows the login screen.
pub(crate) async fn signup_screen() -> axum::response::Html<&'static str> {
    // Keep this really simple for now.
    // Later this could be its own dedicated page, but for now it's just a simple form.

    axum::response::Html(
        r#"
        <h1>sign up</h1>
        <form id="signup-form">
            <input type="text" id="username" name="username" placeholder="username" />
            <input type="password" id="password" name="password" placeholder="password" />
            <input type="submit" value="Sign up" />
        </form>

        <script>
            const form = document.getElementById("signup-form");
            form.onsubmit = function(event) {
                console.log("submitting form");
                event.preventDefault();
                const data = new FormData(form);
                const username = data.get("username");
                const password = data.get("password");

                // Perform a POST request to /signup
                // If the request is successful, the s
                //
                fetch("/signup", {
                    method: "POST",
                    body: JSON.stringify({ username, password }),
                    headers: {
                        "Content-Type": "application/json",
                    },
                })
                .then(response => response.json())
                .then(data => {
                    console.log("Success:", data);
                    // navigate to the main page
                    window.location.href = "/";
                })
                .catch((error) => {
                    console.error("Error:", error);
                });

            }
        </script>

        <style>
            form {
                display: flex;
                flex-direction: column;
                align-items: center;
            }

            input {
                margin: 0.5rem;
            }

            input[type="submit"] {
                width: 100px;
            }

            input[type="text"], input[type="password"] {
                width: 200px;
            }

            input[type="text"]:focus, input[type="password"]:focus {
                outline: none;
            }

            h1 {
                text-align: center;
            }
        </style>
    "#,
    )
}
