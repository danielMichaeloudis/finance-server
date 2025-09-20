document.getElementById("submit-add-single").onclick = () => {
    let transaction = {};
    transaction["vendor"] = document.getElementById("add-single-vendor").value;
    transaction["buyer"] = document.getElementById("add-single-buyer").value;
    transaction["cost"] = parseFloat(
        document.getElementById("add-single-cost").value
    );
    transaction["tags"] = document
        .getElementById("add-single-tags")
        .value.split(",")
        .map((t) => {
            return t.trim();
        });

    const rows = document.querySelectorAll("#items .item-row");
    const items = [];

    rows.forEach((row) => {
        const itemName = row.querySelector(".item-name").value;
        const itemPrice = row.querySelector(".item-price").value;
        const itemBoughtFor = row.querySelector(".item-bought-for").value;
        items.push({
            name: itemName,
            price: parseFloat(itemPrice),
            bought_for: itemBoughtFor,
        });
    });

    transaction["items"] = items;

    fetch("/api/transactions_many", {
        method: "POST",
        body: JSON.stringify([transaction]),
        headers: {
            authorization: "Bearer " + getCookie("token"),
            "Content-Type": "application/json",
        },
    }).then(
        (res) => {
            location.reload();
        },
        (res) => {
            console.log("Failed to add transaction: ", res);
        }
    );
};
console.log("HTML inserted");
console.log(document.getElementById("add-single-transaction"));
