const total = document.getElementById("total");
const incomming = document.getElementById("incomming");
const outgoing = document.getElementById("outgoing");

const token = getCookie("token");
const total_spent_request = new Request("/api/total_in_out", {
    method: "GET",
    headers: { AUTHORIZATION: "Bearer " + token },
});
fetch(total_spent_request).then((res) => {
    res.json().then((val) => {
        total.innerHTML = "£" + val.total.toFixed(2);
        incomming.innerHTML = "£" + val.incomming.toFixed(2);
        outgoing.innerHTML = "£" + val.outgoing.toFixed(2);
    });
});

const menuBtn = document.getElementById("menu-btn");
const menuCtnr = document.getElementById("menu-container");
const menuDrop = document.getElementById("menu-dropdown");

menuBtn.onmouseover = (event) => {
    menuDrop.classList.add("open");
};

menuCtnr.onmouseleave = (event) => {
    menuDrop.classList.remove("open");
};

const logoutBtn = document.getElementById("logout");

logoutBtn.onclick = () => {
    deleteCookie("token");
    location.reload();
};

const exportBtn = document.getElementById("export");

exportBtn.onclick = () => {
    fetch("/api/export", {
        method: "GET",
        headers: { AUTHORIZATION: "Bearer " + token },
    })
        .then((res) => res.blob())
        .then((blob) => {
            const url = window.URL.createObjectURL(blob);
            const a = document.createElement("a");
            a.href = url;
            a.download = "Transactions.json";
            document.body.appendChild(a);
            a.click();
            a.remove();
            window.URL.revokeObjectURL(url);
        });
};
