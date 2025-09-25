document.getElementById("submit-add-single").onclick = () => {
    const vendor = document.getElementById("vendor");
    const buyer = document.getElementById("buyer");
    const cost = document.getElementById("cost");
    const tags = document.getElementById("tags");
    const dat = document.getElementById("date");

    let transaction = {};
    transaction["vendor"] = vendor.value;
    transaction["buyer"] = buyer.value;
    transaction["cost"] = parseFloat(cost.value);
    transaction["tags"] = tags.value.split(",").map((t) => {
        return t.trim();
    });
    transaction["date"] = date.value;

    const rows = document.querySelectorAll("#items .add-item-row");
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
            document.getElementById("transaction-form").reset();
            const itemsDiv = document.getElementById("items");
            itemsDiv.innerHTML = "";
            location.reload();
        },
        (res) => {
            console.log("Failed to add transaction: ", res);
        }
    );
};
