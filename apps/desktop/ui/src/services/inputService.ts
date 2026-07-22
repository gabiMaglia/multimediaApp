import { listen } from "@tauri-apps/api/event"

export function startInputListener() {

    listen("input:event", (event) => {
        console.log("INPUT", event.payload)
    })

}