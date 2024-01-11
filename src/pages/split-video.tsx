import {useState} from "react";
import {cn} from "@/lib/utils.ts";
import {Textarea} from "@/components/ui/textarea.tsx";
import {Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow} from "@/components/ui/table.tsx";
import {Button} from "@/components/ui/button.tsx";
import {X} from "lucide-react";
import { open } from "@tauri-apps/api/dialog"
import {invoke} from "@tauri-apps/api/tauri";
import toast from "react-hot-toast";

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
                    placeholder="Type your message here."
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