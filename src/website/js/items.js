function updateCost() {
    const prices = document.querySelectorAll(
        "#transaction-items .add-item-row .transaction-item-price"
    );
    let totalCost = 0;
    prices.forEach((price) => {
        totalCost += parseFloat(price.value);
    });
    const transactionCost = document.getElementById("transaction-cost");
    transactionCost.value = totalCost;
    if (prices.length > 0) {
        transactionCost.readOnly = true;
    } else {
        transactionCost.readOnly = false;
    }
}

document.getElementById("add-item").onclick = addItem;

function addItem() {
    fetch("/components/item-row", {
        method: "POST",
        body: JSON.stringify(null),
        headers: {
            authorization: "Bearer " + getCookie("token"),
            "Content-Type": "application/json",
        },
    })
        .then((res) => res.text())
        .then((html) => {
            addHTML(html, null, document.getElementById("transaction-items"));
        });
}
document
    .getElementById("transaction-items")
    .addEventListener("keydown", (event) => {
        if (event.key === "Enter" && event.target.closest(".add-item-row")) {
            event.preventDefault();
            addItem();
        }
    });
