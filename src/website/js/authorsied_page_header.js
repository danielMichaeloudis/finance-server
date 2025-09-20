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
