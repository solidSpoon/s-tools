import {cn} from "@/lib/utils.ts";
import LinkButton from "@/components/link-button.tsx";
import {Outlet} from "react-router-dom";
import {Separator} from "@/components/ui/separator.tsx";

const Layout = () => {
    return (
        <div className={cn('w-full h-full flex')}>
            <div className="space-y-1 w-64 p-2">
                <LinkButton to={'/'}>Home</LinkButton>
                <LinkButton to={'/split-video'}>split-video</LinkButton>
            </div>
            <Separator orientation="vertical" className=""/>
            <div className="w-0 flex-1 h-full">
                <Outlet/>
            </div>
        </div>
    )
}

export default Layout;