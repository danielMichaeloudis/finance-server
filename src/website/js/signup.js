let button = document.getElementById("signup-button");
let inputs = document.getElementsByClassName("login-signup-input");
let errorBox = document.getElementById("error");
let error = null;

for (input of inputs) {
    input.onchange = resetError;
    input.onclick = resetError;
}

function resetError() {
    errorBox.innerHTML = "";
    errorBox.style.display = "none";
}

button.onclick = () => {
    let password = document.getElementById("password").value;
    let confPassword = document.getElementById("conf-password").value;
    if (password != confPassword) {
        console.log("Passwords not equal");
        return;
    }
    fetch("/api/signup", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
            username: document.getElementById("username").value,
            email: document.getElementById("email").value,
            password: document.getElementById("password").value,
        }),
    }).then((res) => {
        if (!res.ok) {
            if (res.status == 500) {
                console.error("Database Error: " + res.error);
                return;
            }
            if (res.status == 409) {
                res.text().then((msg) => {
                    console.error("Duplicate field: ", msg);
                    errorBox.style.display = "inline";
                    if (msg.includes("username")) {
                        errorBox.innerHTML =
                            "User with this username already exists";
                    }
                    if (msg.includes("email")) {
                        errorBox.innerHTML =
                            "User with this email address already exists";
                    }
                });
                return;
            }
            console.error("Error: " + res.error);
        }
        res.json().then((res) => {
            const d = new Date();
            d.setTime(d.getTime() + 3 * 24 * 60 * 60 * 1000); // 3 days
            let expires = "expires=" + d.toUTCString();
            document.cookie = "token=" + res + ";" + expires + ";path=/";
            document.location.href = "/home";
        });
    });
};
