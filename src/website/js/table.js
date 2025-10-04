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

addContainer.onmouseleave = (event) => {
    event.currentTarget.classList.remove("open");
};

const addTransactionBtn = document.getElementById("add-transaction-btn");

addTransactionBtn.onclick = () => {
    fetch("/components/add_single_transaction")
        .then((res) => res.text())
        .then((html) => {
            addHTML(html, "transaction");
            //default date picker to now
            document.getElementById("transaction-date").valueAsDate =
                new Date();
            document.getElementById("close-transaction").onclick = () => {
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

function editTransaction(event) {
    const row = event.currentTarget.closest(".transaction-row");
    let transaction = {};
    transaction["uuid"] = row.getElementsByClassName("uuid")[0].innerText;
    transaction["vendor"] = row.getElementsByClassName("vendor")[0].innerText;
    transaction["buyer"] = row.getElementsByClassName("buyer")[0].innerText;
    transaction["cost"] = row.getElementsByClassName("cost")[0].innerText;
    transaction["tags"] = row.getElementsByClassName("tags")[0].innerText;
    transaction["date"] = null; //row.getElementsByClassName("date")[0].innerText;
    transaction["items"] = []; //row.getElementsByClassName("items")[0].innerText;
    fetch("/components/edit_transaction", {
        method: "POST",
        body: JSON.stringify(transaction),
        headers: {
            authorization: "Bearer " + getCookie("token"),
            "Content-Type": "application/json",
        },
    })
        .then((res) => res.text())
        .then((html) => {
            addHTML(html, "transaction");
            document.getElementById("close-transaction").onclick = () => {
                var els = document.getElementsByClassName(
                    "added-by-transaction"
                );

                while (els[0]) {
                    els[0].parentNode.removeChild(els[0]);
                }
            };
            document.getElementById("edit-transaction").classList.add("shown");
        });
}
