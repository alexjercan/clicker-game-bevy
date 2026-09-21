(function () {
    "use strict";

    function showFallback() {
        const container = document.querySelector(".game-container");
        if (!container) {
            return;
        }

        container.innerHTML =
            '<div class="webgpu-fallback">' +
            "<h1>WebGPU required</h1>" +
            "<p>Clicker runs in the browser on <strong>WebGPU</strong>, which this browser does not have working.</p>" +
            "<p>Try a recent <strong>Chrome</strong> or <strong>Edge</strong>, <strong>Safari</strong> on macOS or iOS, or <strong>Firefox</strong> on Windows. Firefox on Linux does not ship WebGPU yet.</p>" +
            "</div>";
    }

    if (!navigator.gpu) {
        showFallback();
        return;
    }

    try {
        navigator.gpu.requestAdapter().then(function (adapter) {
            if (!adapter) {
                showFallback();
            }
        }, showFallback);
    } catch (_error) {
        showFallback();
    }
})();
