import "./App.css";
import {cn} from "@/lib/utils.ts";
import {createBrowserRouter, createRoutesFromElements, Route, RouterProvider} from "react-router-dom";
import Layout from "@/components/layout/layout.tsx";
import {Toaster} from "react-hot-toast";
import SplitVideo from "@/pages/split-video.tsx";

const router = createBrowserRouter(
    createRoutesFromElements(
        <Route path="/" element={<Layout />}>
            <Route path="split-video" element={<SplitVideo />}/>
            {/*<Route*/}
            {/*    path="dashboard"*/}
            {/*    element={<div />}*/}
            {/*    loader={({ request }) =>*/}
            {/*        fetch("/api/dashboard.json", {*/}
            {/*            signal: request.signal,*/}
            {/*        })*/}
            {/*    }*/}
            {/*/>*/}
        </Route>
    )
);

function App() {
    // const [greetMsg, setGreetMsg] = useState("");
    // const [name, _setName] = useState("");
    //
    // async function greet() {
    //     // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    //     setGreetMsg(await invoke("greet", {name}));
    // }

    return (
        <div className={cn('w-full h-screen')}>
            <RouterProvider router={router}/>
            <Toaster />
        </div>
    );
}

export default App;
