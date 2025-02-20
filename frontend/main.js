// Websocket
const url = "wss://127.0.0.1:3000";
const ws = new WebSocket(url);

ws.addEventListener("open", (e) => {
    console.log("Connected to server");
    ws.send("Client connected");
});

// Interaksi dengan kamera
const tombolMulai = document.getElementById("mulai");

tombolMulai.addEventListener("click", startCamera);

async function startCamera() {
    if (!navigator.mediaDevices) {
        showError(
            "Kamera tidak tersedia, mungkin karena koneksi Anda tidak aman (HTTPS), atau browser Anda tidak mendukung.", null);
        return;
    }
    try {
        for (;;) {
            const stream = await navigator.mediaDevices.getUserMedia({ video: true });
            const videoElement = document.getElementById("video");
            videoElement.srcObject = stream;
        }
    } catch (err) {
        showError("Gagal mengakses kamera", err);
    }
}

function showError(msg, e) {
    // TODO: Tampilkan error ke pengguna dengan popup
    console.error(`${msg} ${e ? ': ' + e : ''}`);
}
