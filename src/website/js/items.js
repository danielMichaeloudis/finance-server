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
    const itemsDiv = document.getElementById("transaction-items");

    const row = document.createElement("form");
    row.onsubmit = (event) => {
        console.log("submit");
        event.preventDefault();
        addItem();
    };
    const sbmtBtn = document.createElement("input");
    sbmtBtn.type = "submit";
    sbmtBtn.style.display = "none";
    row.appendChild(sbmtBtn);
    row.className = "add-item-row";

    const itemName = document.createElement("input");
    itemName.type = "text";
    itemName.classList.add("styled-input");
    itemName.classList.add("item-input");
    itemName.classList.add("transaction-item-name");
    itemName.placeholder = "Item Name *";
    row.appendChild(itemName);

    const itemPrice = document.createElement("input");
    itemPrice.type = "number";
    itemPrice.classList.add("styled-input");
    itemPrice.classList.add("item-input");
    itemPrice.classList.add("transaction-item-price");
    itemPrice.step = 0.01;
    itemPrice.placeholder = "Item Price *";
    itemPrice.onchange = updateCost;

    row.appendChild(itemPrice);

    const itemBoughtFor = document.createElement("input");
    itemBoughtFor.type = "text";
    itemBoughtFor.classList.add("styled-input");
    itemBoughtFor.classList.add("item-input");
    itemBoughtFor.classList.add("transaction-item-bought-for");
    itemBoughtFor.placeholder = "Bought For *";
    row.appendChild(itemBoughtFor);

    const removeBtn = document.createElement("button");
    removeBtn.innerHTML =
        '<svg style="margin: 0;" width="15px" height="15px" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" stroke="#848484"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <path d="M6 6L18 18M18 6L6 18" stroke="#848484" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path> </g></svg>';
    removeBtn.classList.add("remove-item-btn");
    removeBtn.style.margin = "0"; //Otherwise overrided by transaction *
    removeBtn.onmouseenter = () => {
        removeBtn.style.backgroundColor = "#ffffff10";
    };
    removeBtn.onmouseleave = () => {
        removeBtn.style.backgroundColor = "#ffffff00";
    };
    removeBtn.type = "button";
    removeBtn.onclick = () => {
        itemsDiv.removeChild(row);
        updateCost();
    };

    row.appendChild(removeBtn);
    itemsDiv.appendChild(row);
}
