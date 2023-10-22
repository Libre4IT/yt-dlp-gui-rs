const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;


async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  // greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value});
  greetMsgEl.textContent = await invoke("greet", { url: greetInputEl[0].value, musiconly: greetInputEl[1].value});
}

// set checkbox audio only value to 1 if checked, else set value to 0
const el = document.getElementById("greet-input-audio-only");
el.addEventListener(
  "click",
  () => {
    el.value = "y";
  },
  el.value = "n",
); 

/// Trigger rust code
window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelectorAll('[id*=greet-input]');
  greetMsgEl = document.querySelector("#greet-msg");
 
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});

