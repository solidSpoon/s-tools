import {useState} from "react";
import {cn} from "@/lib/utils.ts";
import {Textarea} from "@/components/ui/textarea.tsx";
import {Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow} from "@/components/ui/table.tsx";
import {Button} from "@/components/ui/button.tsx";
import {X} from "lucide-react";
import { open } from "@tauri-apps/api/dialog"
import {invoke} from "@tauri-apps/api/tauri";
import toast from "react-hot-toast";

const PLACEHOLDER = `Edit your timestamp and title here.

eg:
00:00:00 Intro
00:01:55 How JWT security works
00:07:26 Create a new spring boot 3.0 project
00:09:28 Add Data source
00:12:28 Connect to the database
00:17:12 Create user class
00:20:05 Transform the User to an entity
00:25:22 Extend the user to UserDeatils object
00:33:32 Create the user repository
00:35:50 Create the JWT authentication filter
00:40:58 Checking the JWT token
00:44:32 Create the JWT service
00:47:56 Add the JWT dependencies`


const SplitVideo = () => {
    const [files, setFiles] = useState<string[]>([])
    const [text, setText] = useState<string>("")
    const [loading, setLoading] = useState<boolean>(false)
    const select = async () => {
        const file: string | string[] | null = await open()
        if (file) {
            setFiles((f) => [...f, file].flat())
        }
    }

    const handleSplit = async () => {
        setLoading(true)
        await toast.promise(
            invoke<string>("split_file", { fileNames: files, timeStamp: text })
                .catch((e) => {
                    toast.error(e)
                    setLoading(false)
                }),
            {
                loading: "Splitting...",
                success: "Split Successfully",
                error: "Split Failed"
            },
        )
        setLoading(false)
    }


    return (
        <div
            style={{
                gridTemplateColumns: "45% 55%",
            }}
            className={cn("w-full h-full grid grid-cols-2")}
        >
            <div className={cn("p-2")}>
                <Textarea
                    value={text}
                    onChange={(e) => {
                        setText(e.target.value)
                    }}
                    className={cn("w-full h-full resize-none")}
                    placeholder={PLACEHOLDER}
                />
            </div>
            <div className={cn("p-2 flex flex-col")}>
                <Table className={cn("h-0 flex-1")}>
                    <TableCaption>A list of your recent invoices.</TableCaption>
                    <TableHeader>
                        <TableRow>
                            <TableHead>Name</TableHead>
                            <TableHead className="w-[100px] text-center">Type</TableHead>
                            <TableHead className="w-[150px] text-center">Action</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {files.map((file) => (
                            <TableRow key={file}>
                                <TableCell className="font-medium">{file}</TableCell>
                                <TableCell className={"text-center"}>Video</TableCell>
                                <TableCell className={"text-center"}>
                                    <Button
                                        onClick={() => {
                                            setFiles((f) => f.filter((x) => x !== file))
                                        }}
                                        variant="outline"
                                        size="icon"
                                    >
                                        <X className="h-4 w-4" />
                                    </Button>
                                </TableCell>
                            </TableRow>
                        ))}
                    </TableBody>
                </Table>
                <div className={cn("mt-auto flex justify-end gap-2")}>
                    <Button
                        disabled={loading}
                        onClick={() => {
                            select().catch((e) => {
                                console.log(e)
                            })
                        }}
                        variant="secondary"
                    >
                        Select File
                    </Button>
                    <Button
                        onClick={() => {
                            handleSplit().catch((e) => {
                                console.log(e)
                            })
                        }}
                        disabled={files.length === 0 || loading}
                    >
                        Split By Timestamp
                    </Button>
                </div>
            </div>
        </div>
    )
}

export default SplitVideo