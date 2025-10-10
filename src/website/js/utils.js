fetch("/version").then((res) => {
    res.text().then((version) => {
        console.log(version);
    });
});

function getCookie(cname) {
    let name = cname + "=";
    let decodedCookie = decodeURIComponent(document.cookie);
    let ca = decodedCookie.split(";");
    for (let i = 0; i < ca.length; i++) {
        let c = ca[i];
        while (c.charAt(0) == " ") {
            c = c.substring(1);
        }
        if (c.indexOf(name) == 0) {
            return c.substring(name.length, c.length);
        }
    }
    return "";
}

function deleteCookie(cname) {
    document.cookie =
        cname + "=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
}

function addHTML(html, idToRemove = null, addTo = document.body) {
    if (idToRemove != null) {
        var els = document.getElementsByClassName("added-by-" + idToRemove);

        while (els[0]) {
            els[0].parentNode.removeChild(els[0]);
        }
    }

    const temp = document.createElement("div");
    temp.innerHTML = html;

    const styleTags = temp.querySelectorAll("style");
    const scriptTags = temp.querySelectorAll("script");

    styleTags.forEach((style) => {
        const newStyle = document.createElement("style");
        newStyle.textContent = style.textContent;

        if (idToRemove != null) {
            newStyle.classList.add("added-by-" + idToRemove);
        }

        document.head.appendChild(newStyle);
    });

    scriptTags.forEach((oldScript) => {
        const newScript = document.createElement("script");
        if (oldScript.src) {
            newScript.src = oldScript.src;
            newScript.defer = oldScript.defer;
        } else {
            newScript.textContent = oldScript.textContent;
        }

        if (idToRemove != null) {
            newScript.classList.add("added-by-" + idToRemove);
        }
        document.body.appendChild(newScript);
    });

    styleTags.forEach((el) => el.remove());
    scriptTags.forEach((el) => el.remove());

    if (idToRemove != null) {
        temp.firstElementChild.classList.add("added-by-" + idToRemove);
    }
    return addTo.appendChild(temp.firstElementChild);
}
