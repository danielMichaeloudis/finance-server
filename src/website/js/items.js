document.getElementById("add-item").onclick = () => {
    const itemsDiv = document.getElementById("items");

    const row = document.createElement("div");
    row.className = "add-item-row";

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
    removeBtn.addEventListener("click", () => {
        itemsDiv.removeChild(row);
    });

    row.appendChild(removeBtn);
    itemsDiv.appendChild(row);
};
