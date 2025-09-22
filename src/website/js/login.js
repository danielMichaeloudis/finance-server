let button = document.getElementById("login-button");
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
    fetch("/api/login", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
            username: document.getElementById("username").value,
            password: document.getElementById("password").value,
        }),
    }).then((res) => {
        if (!res.ok) {
            if (res.status == 500) {
                console.error("Database Error: " + res.error);
                return;
            }
            if (res.status == 401) {
                console.error("Incorrect Username Or Password");
                errorBox.innerHTML = "Invalid Username Or Password";
                errorBox.style.display = "inline";
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
