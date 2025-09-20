const transactionRows = document.getElementsByClassName("transaction-row");

for (row of transactionRows) {
    row.onclick = (event) => {
        const container = event.currentTarget.closest(".transaction-container");
        const dropdown = container.querySelector(".dropdown");

        const isOpen = container.classList.toggle("open");

        if (isOpen) {
            dropdown.style.maxHeight = dropdown.scrollHeight + "px";
            dropdown.style.opacity = "1";
        } else {
            dropdown.style.maxHeight = "0";
            dropdown.style.opacity = "0";
        }
    };
}

const addContainer = document.getElementById("add-container");
const addBtn = document.getElementById("add-btn");

addBtn.onmouseover = (event) => {
    event.currentTarget.parentElement.classList.add("open");
};
addBtn.onclick = (event) => {
    event.currentTarget.parentElement.classList.toggle("open");
};

addContainer.onmouseleave = (event) => {
    event.currentTarget.classList.remove("open");
};

const addTransactionBtn = document.getElementById("add-transaction-btn");

addTransactionBtn.onclick = () => {
    fetch("/components/add_single_transaction")
        .then((res) => res.text())
        .then((html) => {
            addHTML(html, "add-transaction-btn");
            document.getElementById("close-add-single-transaction").onclick =
                () => {
                    var els = document.getElementsByClassName(
                        "added-by-add-transaction-btn"
                    );

                    while (els[0]) {
                        els[0].parentNode.removeChild(els[0]);
                    }
                };
            document
                .getElementById("add-single-transaction")
                .classList.add("shown");
        });
};
