const token = getCookie("token");

const menuBtn = document.getElementById("menu-btn");
const menuCtnr = document.getElementById("menu-container");
const menuDrop = document.getElementById("menu-dropdown");

menuBtn.onmouseover = () => {
    menuDrop.classList.add("open");
};

menuCtnr.onmouseleave = () => {
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
