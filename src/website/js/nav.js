const nav_home = document.getElementById("home-btn");
const nav_table = document.getElementById("table-btn");

if (window.location.pathname.endsWith("/table")) {
    nav_home.style.borderBottomStyle = "none";
    nav_table.style.borderBottomStyle = "solid";
} else {
    nav_home.style.borderBottomStyle = "solid";
    nav_table.style.borderBottomStyle = "none";
}
