import {useState} from "react";

import {invoke} from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
    const [greetMsg, setGreetMsg] = useState("");
    const [name, _setName] = useState("");

    async function greet() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        setGreetMsg(await invoke("greet", {name}));
    }

    return (
        <div className="h-full w-full bg-amber-300">
            <button
                onClick={greet}
            >Greet
            </button>
            <p>{greetMsg}</p>
        </div>
    );
}

export default App;
