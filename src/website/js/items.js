document.getElementById("add-item").onclick = () => {
    const itemsDiv = document.getElementById("items");

    const row = document.createElement("div");
    row.className = "item-row";

    const itemName = document.createElement("input");
    itemName.type = "text";
    itemName.classList.add("styled-input");
    itemName.classList.add("item-input");
    itemName.classList.add("item-name");
    itemName.placeholder = "Item Name *";
    row.appendChild(itemName);

    const itemPrice = document.createElement("input");
    itemPrice.type = "number";
    itemPrice.classList.add("styled-input");
    itemPrice.classList.add("item-input");
    itemPrice.classList.add("item-price");
    itemPrice.placeholder = "Item Price *";
    row.appendChild(itemPrice);

    const itemBoughtFor = document.createElement("input");
    itemBoughtFor.type = "text";
    itemBoughtFor.classList.add("styled-input");
    itemBoughtFor.classList.add("item-input");
    itemBoughtFor.classList.add("item-bought-for");
    itemBoughtFor.placeholder = "Bought For *";
    row.appendChild(itemBoughtFor);

    const removeBtn = document.createElement("button");
    removeBtn.textContent = "X";
    removeBtn.style.margin = "0.25rem";
    removeBtn.style.backgroundColor = "rgba(0 0 0 / 0)";
    removeBtn.style.borderStyle = "none";
    removeBtn.style.color = "#fff";
    removeBtn.addEventListener("click", () => {
        itemsDiv.removeChild(row);
    });

    row.appendChild(removeBtn);
    itemsDiv.appendChild(row);
};
