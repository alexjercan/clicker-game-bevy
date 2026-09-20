import argparse
import json
import threading
import webbrowser
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import unquote, urlsplit

import numpy as np
from scipy.io import wavfile
from scipy.signal import butter, iirpeak, lfilter

RATE = 44100
OUTPUT = Path(__file__).parents[1] / "assets" / "audio"
RNG = np.random.default_rng(20260919)


def time(duration):
    return np.arange(int(RATE * duration)) / RATE


def fade(signal, attack, decay):
    length = len(signal)
    attack_samples = min(int(RATE * attack), length)
    envelope = np.exp(-np.arange(length) / (RATE * decay))
    if attack_samples:
        envelope[:attack_samples] *= np.linspace(0.0, 1.0, attack_samples)
    return signal * envelope


def tone(frequency, duration, decay, harmonics=()):
    t = time(duration)
    signal = np.sin(2.0 * np.pi * frequency * t)
    for multiplier, amount in harmonics:
        signal += amount * np.sin(2.0 * np.pi * frequency * multiplier * t)
    return fade(signal, 0.002, decay)


def filtered_noise(duration, cutoff, decay):
    samples = RNG.normal(0.0, 1.0, len(time(duration)))
    numerator, denominator = butter(2, cutoff / (RATE * 0.5), btype="low")
    return fade(lfilter(numerator, denominator, samples), 0.001, decay)


def whoosh(duration, low, high, attack, decay):
    t = time(duration)
    samples = RNG.normal(0.0, 1.0, len(t))
    numerator, denominator = butter(
        2, [low / (RATE * 0.5), high / (RATE * 0.5)], btype="bandpass"
    )
    envelope = (1.0 - np.exp(-t / attack)) * np.exp(-t / decay)
    return lfilter(numerator, denominator, samples) * envelope


def soft_whoosh(duration, low, high, attack, decay):
    length = len(time(duration))
    control_count = max(2, int(duration * 12000))
    controls = RNG.normal(0.0, 1.0, control_count)
    samples = np.interp(
        np.arange(length), np.linspace(0, length - 1, control_count), controls
    )
    numerator, denominator = butter(
        3, [low / (RATE * 0.5), high / (RATE * 0.5)], btype="bandpass"
    )
    t = time(duration)
    envelope = (1.0 - np.exp(-t / attack)) * np.exp(-t / decay)
    return lfilter(numerator, denominator, samples) * envelope


def grass_break(duration):
    t = time(duration)
    samples = RNG.normal(0.0, 1.0, len(t))
    numerator, denominator = butter(
        2, [380.0 / (RATE * 0.5), 4200.0 / (RATE * 0.5)], btype="bandpass"
    )
    texture = lfilter(numerator, denominator, samples)
    control_count = max(2, int(duration * 900))
    controls = np.abs(RNG.normal(0.0, 1.0, control_count))
    modulation = np.interp(
        np.arange(len(t)), np.linspace(0, len(t) - 1, control_count), controls
    )
    modulation = 0.35 + 0.65 * modulation / np.max(modulation)
    envelope = (1.0 - np.exp(-t / 0.0015)) * np.exp(-t / 0.048)
    return texture * modulation * envelope


def resonant_hit(duration, modes, transient_cutoff, transient_amount=0.16):
    length = len(time(duration))
    excitation = np.zeros(length)
    burst_length = min(int(RATE * 0.008), length)
    excitation[:burst_length] = RNG.normal(0.0, 1.0, burst_length) * np.exp(
        -np.arange(burst_length) / (RATE * 0.0015)
    )
    signal = np.zeros(length)
    for frequency, quality, amount in modes:
        numerator, denominator = iirpeak(frequency / (RATE * 0.5), quality)
        signal += amount * lfilter(numerator, denominator, excitation)
    signal += transient_amount * filtered_noise(duration, transient_cutoff, 0.012)
    return signal


def placement_sound():
    duration = 0.32
    pop = tone(82.0, duration, 0.032, ((2.0, 0.16),))
    air = whoosh(duration, 180.0, 1200.0, 0.045, 0.13)
    return 0.90 * pop + 0.16 * air


def sequence(notes, note_duration, gap):
    total = int(RATE * (note_duration + gap) * len(notes))
    signal = np.zeros(total)
    for index, frequency in enumerate(notes):
        note = tone(frequency, note_duration, note_duration * 0.55, ((2.0, 0.18),))
        start = int(index * RATE * (note_duration + gap))
        signal[start : start + len(note)] += note
    return signal


def write(name, signal, gain=0.8):
    OUTPUT.mkdir(parents=True, exist_ok=True)
    peak = np.max(np.abs(signal))
    normalized = signal if peak == 0.0 else signal / peak
    wavfile.write(OUTPUT / name, RATE, np.int16(normalized * gain * 32767))


def generate():
    write(
        "click_tree.wav",
        resonant_hit(0.15, ((95.0, 4.0, 1.0), (185.0, 3.5, 0.65), (310.0, 3.0, 0.28)), 900.0),
        0.62,
    )
    write(
        "click_stone.wav",
        resonant_hit(
            0.13,
            ((115.0, 3.2, 1.0), (215.0, 3.0, 0.78), (370.0, 2.7, 0.62), (610.0, 2.4, 0.16)),
            1200.0,
        ),
        0.52,
    )
    write("click_wheat.wav", grass_break(0.13), 0.26)
    write("select.wav", tone(185.0, 0.05, 0.016, ((2.0, 0.08),)), 0.16)
    write("place.wav", placement_sound(), 0.58)
    write("level_up.wav", sequence([523.25, 659.25, 880.00], 0.06, 0.008), 0.52)
    write(
        "click_empty.wav",
        resonant_hit(
            0.13,
            ((145.0, 1.7, 0.60), (255.0, 1.5, 0.32), (430.0, 1.4, 0.10)),
            850.0,
            0.05,
        ),
        0.24,
    )


def preview_page(files):
    sounds = json.dumps(files)
    return f"""<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Clicker SFX Preview</title>
<style>
:root {{ color-scheme: dark; font-family: Inter, ui-sans-serif, system-ui, sans-serif; }}
* {{ box-sizing: border-box; }}
body {{ margin: 0; min-height: 100vh; background: #111318; color: #f2f4f8; }}
main {{ width: min(960px, calc(100% - 32px)); margin: 0 auto; padding: 48px 0 72px; }}
h1 {{ margin: 0; font-size: clamp(2rem, 6vw, 4rem); letter-spacing: -0.05em; }}
header p {{ margin: 10px 0 32px; color: #9aa4b2; }}
#sounds {{ display: grid; gap: 14px; }}
.sound {{ display: grid; grid-template-columns: 180px 1fr 84px; align-items: center; gap: 18px; padding: 18px; border: 1px solid #2b313d; border-radius: 14px; background: #191d24; box-shadow: 0 12px 32px #0004; }}
.name {{ min-width: 0; }}
.name strong {{ display: block; overflow: hidden; text-overflow: ellipsis; }}
.name span, .duration {{ color: #8d98a8; font-size: 0.82rem; }}
.wave {{ width: 100%; height: 88px; border: 1px solid #323946; border-radius: 9px; background: #0d0f13; cursor: pointer; }}
button {{ height: 42px; border: 0; border-radius: 999px; background: #78dba9; color: #102019; font: inherit; font-weight: 750; cursor: pointer; }}
button:hover {{ background: #9ce9c2; }}
button:disabled {{ background: #3a414c; color: #8d98a8; cursor: wait; }}
@media (max-width: 720px) {{ .sound {{ grid-template-columns: 1fr 74px; }} .wave {{ grid-column: 1 / -1; grid-row: 2; }} }}
</style>
</head>
<body>
<main>
<header><h1>Clicker SFX</h1><p>Generated WAV previews. Click a waveform or Play to listen.</p></header>
<section id="sounds"></section>
</main>
<script>
const files = {sounds};
const list = document.querySelector('#sounds');
const audio = new AudioContext();
let active = null;
let activeButton = null;
function stop() {{
  if (active) active.stop();
  if (activeButton) activeButton.textContent = 'Play';
  active = null;
  activeButton = null;
}}
function draw(canvas, buffer) {{
  const ratio = window.devicePixelRatio || 1;
  canvas.width = Math.max(1, canvas.clientWidth * ratio);
  canvas.height = Math.max(1, canvas.clientHeight * ratio);
  const context = canvas.getContext('2d');
  const samples = buffer.getChannelData(0);
  const middle = canvas.height / 2;
  const stride = Math.max(1, Math.floor(samples.length / canvas.width));
  context.clearRect(0, 0, canvas.width, canvas.height);
  context.strokeStyle = '#252b35';
  context.beginPath();
  context.moveTo(0, middle);
  context.lineTo(canvas.width, middle);
  context.stroke();
  context.fillStyle = '#78dba9';
  for (let x = 0; x < canvas.width; x++) {{
    let minimum = 1;
    let maximum = -1;
    const start = x * stride;
    for (let i = start; i < Math.min(start + stride, samples.length); i++) {{
      minimum = Math.min(minimum, samples[i]);
      maximum = Math.max(maximum, samples[i]);
    }}
    const top = middle + minimum * middle * 0.86;
    const height = Math.max(1, (maximum - minimum) * middle * 0.86);
    context.fillRect(x, top, 1, height);
  }}
}}
function play(buffer, button) {{
  stop();
  audio.resume();
  const source = audio.createBufferSource();
  source.buffer = buffer;
  source.connect(audio.destination);
  source.onended = () => {{
    if (active === source) stop();
  }};
  active = source;
  activeButton = button;
  button.textContent = 'Stop';
  source.start();
}}
for (const file of files) {{
  const card = document.createElement('article');
  card.className = 'sound';
  const label = file.replace('.wav', '').replaceAll('_', ' ');
  card.innerHTML = `<div class="name"><strong>${{label}}</strong><span>${{file}}</span></div><canvas class="wave"></canvas><button disabled>Loading</button>`;
  list.append(card);
  const canvas = card.querySelector('canvas');
  const button = card.querySelector('button');
  fetch(`/audio/${{encodeURIComponent(file)}}`)
    .then(response => response.arrayBuffer())
    .then(data => audio.decodeAudioData(data))
    .then(buffer => {{
      card.querySelector('.name').insertAdjacentHTML('beforeend', `<span class="duration">${{Math.round(buffer.duration * 1000)}} ms</span>`);
      draw(canvas, buffer);
      button.disabled = false;
      button.textContent = 'Play';
      button.addEventListener('click', () => activeButton === button ? stop() : play(buffer, button));
      canvas.addEventListener('click', () => activeButton === button ? stop() : play(buffer, button));
      window.addEventListener('resize', () => draw(canvas, buffer));
    }});
}}
</script>
</body>
</html>""".encode()


class PreviewHandler(BaseHTTPRequestHandler):
    files = set()
    page = b""

    def do_GET(self):
        path = urlsplit(self.path).path
        if path == "/":
            self.send_response(200)
            self.send_header("Content-Type", "text/html; charset=utf-8")
            self.send_header("Content-Length", str(len(self.page)))
            self.end_headers()
            self.wfile.write(self.page)
            return
        if path.startswith("/audio/"):
            name = unquote(path.removeprefix("/audio/"))
            if name in self.files:
                payload = (OUTPUT / name).read_bytes()
                self.send_response(200)
                self.send_header("Content-Type", "audio/wav")
                self.send_header("Content-Length", str(len(payload)))
                self.send_header("Cache-Control", "no-store")
                self.end_headers()
                self.wfile.write(payload)
                return
        self.send_error(404)


def serve():
    files = sorted(path.name for path in OUTPUT.glob("*.wav"))
    PreviewHandler.files = set(files)
    PreviewHandler.page = preview_page(files)
    server = ThreadingHTTPServer(("127.0.0.1", 0), PreviewHandler)
    url = f"http://127.0.0.1:{server.server_port}/"
    print(f"SFX preview: {url}", flush=True)
    threading.Timer(0.2, lambda: webbrowser.open(url)).start()
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--serve", action="store_true")
    args = parser.parse_args()
    generate()
    if args.serve:
        serve()


if __name__ == "__main__":
    main()
