function updateCount() {
    let text = document.getElementById("body");
    let display = document.getElementById("count");
    display.textContent = text.value.length;
}