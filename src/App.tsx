import {useState} from "react";

import {invoke} from "@tauri-apps/api/tauri";
import "./App.css";
import {cn} from "@/lib/utils.ts";
import { Button } from "./components/ui/button";

function App() {
    const [greetMsg, setGreetMsg] = useState("");
    const [name, _setName] = useState("");

    async function greet() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        setGreetMsg(await invoke("greet", {name}));
    }

    return (
        <div className={cn('w-full h-screen grid place-content-center')}>
            <Button
                onClick={greet}
                variant="outline">Button</Button>
            <p>{greetMsg}</p>
        </div>
    );
}

export default App;
