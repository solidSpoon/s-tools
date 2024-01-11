import {cn} from "@/lib/utils.ts";
import {Separator} from "@/components/ui/separator.tsx";

const TitleLayout = ({
                         title,
                         description,
                         children
                     }: {
    title: string,
    description: string,
    children: React.ReactNode,
}) => {
    return (<div className="w-full h-full flex flex-col">
        <div className={cn("p-2 pb-4")}>
            <h1 className={cn("text-2xl")}>{title}</h1>
            <p className={cn("text-base text-gray-500")}
            >{description}</p>
        </div>
        <Separator/>
        <div className={cn("flex-1 h-0 p-2")}>
            {children}
        </div>
    </div>);
}

export default TitleLayout;